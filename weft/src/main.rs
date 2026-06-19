use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

include!(concat!(env!("OUT_DIR"), "/skills_registry.rs"));

use clap::{Parser, Subcommand, ValueEnum};
use weft_core::{
    all_annotated_files, annotation_line, bump, check_requirement, dangling_annotations,
    description, drifted_paths, file_hash, files_for_requirement, next_req_id, parse_lock,
    parse_run_lock, parse_test_config, render_lock, render_markdown, render_run_lock,
    resolve_test_command, scan_annotations, skeleton_toml, summarize_trace_states,
    verify_check, verify_not_user_story, verify_requirement, Annotation, Requirement, RunRecord,
    Status, TestResult, TraceState,
};

#[derive(Parser)]
#[command(name = "weft")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

// @implements REQ-029 v2 6f452ce5
/// The CLI surface. By design there is no `save`/`sync` subcommand: the
/// legacy Python `smart` installer and its personal-branch save/sync
/// feature were removed (ADR 0008) — `weft` has a single purpose,
/// requirements traceability.
#[derive(Subcommand)]
enum Command {
    /// Validate requirement records (format + hash integrity).
    Verify {
        /// File or directory to verify (default: docs/prds).
        path: Option<PathBuf>,
    },
    /// Print a single field of a requirement record.
    Get {
        /// The requirement's REQ_ID, e.g. REQ-001.
        req_id: String,
        #[arg(long, value_enum)]
        field: Field,
    },
    /// Allocate the next REQ_ID and write a skeleton requirement record.
    New {
        /// Group the new requirement under this FEAT label.
        #[arg(long)]
        feat: Option<String>,
    },
    /// List requirements by id and description.
    List {
        /// Only show requirements with this FEAT label.
        #[arg(long)]
        feat: Option<String>,
    },
    /// Report each requirement's Trace State; exit non-zero on any drift.
    Check {
        /// Print a rollup count of active requirements per Trace State
        /// instead of the per-requirement listing.
        #[arg(long)]
        summary: bool,
        /// Emit a JSON array, one object per active requirement, with id,
        /// state, and gap detail (missing_links, stale_links, drifted_files).
        #[arg(long)]
        json: bool,
    },
    /// Increment a requirement's version and recompute its hash, together.
    Bump {
        /// The requirement's REQ_ID, e.g. REQ-001.
        req_id: String,
    },
    /// Generate a human-readable Markdown view of the PRD.
    Render,
    /// Scaffold docs/prds/, the design-decision docs dir, and weft skill stubs.
    Init,
    /// Mark a requirement as deprecated (preserved, not deleted).
    Deprecate {
        /// The requirement's REQ_ID, e.g. REQ-001.
        req_id: String,
    },
    /// Record the current SHA-256 of annotated files into docs/prds/weft.lock.
    Seal {
        /// Restrict sealing to files annotated with this REQ_ID, e.g. REQ-001.
        req_id: Option<String>,
    },
    /// Print each Trace Link found for a requirement, with its kind and
    /// file:line location.
    Trace {
        /// The requirement's REQ_ID, e.g. REQ-001.
        req_id: String,
    },
    /// Run the configured Test Command for each active requirement (or a
    /// single REQ_ID) and record pass/fail/unrun into docs/prds/weft.run.toml.
    Test {
        /// Restrict the run to this REQ_ID, updating only its recorded
        /// result.
        req_id: Option<String>,
    },
    /// Print the exact Trace Link line for a requirement, using its current
    /// version and hash.
    Annotate {
        /// The requirement's REQ_ID, e.g. REQ-001.
        req_id: String,
        #[arg(long, value_enum)]
        kind: AnnotateKind,
    },
    /// Exit zero only when every active requirement is Verified; otherwise
    /// list each non-Verified requirement with its Trace State and exit
    /// non-zero. The autonomous agent's single loop-termination check.
    Gate,
    /// Emit the single highest-priority not-yet-Verified requirement with an
    /// explicit action verb (implement|rework|reseal|fix-tests|run-tests).
    /// Exits zero (with "no next work item") when all active requirements are
    /// Verified. The Work Driver for the autonomous agent loop (ADR 0014).
    Next {
        /// Emit the payload as a single JSON object instead of human-readable text.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Clone, ValueEnum)]
enum AnnotateKind {
    Addresses,
    Implements,
    Verifies,
}

#[derive(Clone, ValueEnum)]
enum Field {
    Statement,
    Acceptance,
    Hash,
    Version,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Verify { path } => {
            verify_cmd(&path.unwrap_or_else(|| PathBuf::from("docs/prds")))
        }
        Command::Get { req_id, field } => get_cmd(&req_id, &field),
        Command::New { feat } => new_cmd(feat.as_deref()),
        Command::List { feat } => list_cmd(feat.as_deref()),
        Command::Check { summary, json } => check_cmd(summary, json),
        Command::Bump { req_id } => bump_cmd(&req_id),
        Command::Render => render_cmd(),
        Command::Init => init_cmd(),
        Command::Deprecate { req_id } => deprecate_cmd(&req_id),
        Command::Seal { req_id } => seal_cmd(req_id.as_deref()),
        Command::Test { req_id } => test_cmd(req_id.as_deref()),
        Command::Trace { req_id } => trace_cmd(&req_id),
        Command::Annotate { req_id, kind } => annotate_cmd(&req_id, &kind),
        Command::Gate => gate_cmd(),
        Command::Next { json } => next_cmd(json),
    }
}

/// Recursively collects `.toml` files under `root` (or returns `root` itself
/// if it is already a file), skipping the Run Lock (`weft.run.toml`) — a
/// committed artifact, not a requirement record.
fn find_toml_files(root: &Path, out: &mut Vec<PathBuf>) {
    if root.is_file() {
        out.push(root.to_path_buf());
        return;
    }
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            find_toml_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("toml")
            && path.file_name().and_then(|n| n.to_str()) != Some("weft.run.toml")
        {
            out.push(path);
        }
    }
}

fn load_requirement(path: &Path) -> Result<Requirement, String> {
    let src = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Requirement::parse(&src).map_err(|e| format!("{}: {e}", path.display()))
}

// @implements REQ-007 v2 1d00916a
fn verify_cmd(path: &Path) -> ExitCode {
    let mut files = Vec::new();
    find_toml_files(path, &mut files);
    files.sort();

    let mut ok = true;
    for file in &files {
        let id = file.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
        let src = match fs::read_to_string(file) {
            Ok(src) => src,
            Err(e) => {
                eprintln!("{}: {e}", file.display());
                ok = false;
                continue;
            }
        };
        let req = match Requirement::parse(&src) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("{}: {e}", file.display());
                ok = false;
                continue;
            }
        };

        let mut issues = verify_requirement(&req, id);
        issues.extend(verify_not_user_story(&src));
        if issues.is_empty() {
            println!("{id}: ok");
        } else {
            ok = false;
            for issue in issues {
                println!("{id}: {issue}");
            }
        }
    }

    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

// @implements REQ-008 v2 3da61ff3
fn get_cmd(req_id: &str, field: &Field) -> ExitCode {
    let mut files = Vec::new();
    find_toml_files(Path::new("docs/prds"), &mut files);

    for file in &files {
        if file.file_stem().and_then(|s| s.to_str()) != Some(req_id) {
            continue;
        }

        let req = match load_requirement(file) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        };

        match field {
            Field::Statement => println!("{}", req.statement),
            Field::Acceptance => {
                for item in &req.acceptance {
                    println!("{item}");
                }
            }
            Field::Hash => println!("{}", req.hash),
            Field::Version => println!("{}", req.version),
        }
        return ExitCode::SUCCESS;
    }

    eprintln!("requirement '{req_id}' not found under docs/prds");
    ExitCode::FAILURE
}

// @implements REQ-009 v2 c30912ae
fn new_cmd(feat: Option<&str>) -> ExitCode {
    let prds_root = Path::new("docs/prds");

    let dir = match feat {
        Some(feat) => prds_root.join(feat),
        None => prds_root.to_path_buf(),
    };
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("{}: {e}", dir.display());
        return ExitCode::FAILURE;
    }

    let mut files = Vec::new();
    find_toml_files(prds_root, &mut files);
    let existing_ids: Vec<String> = files
        .iter()
        .filter_map(|f| f.file_stem().and_then(|s| s.to_str()).map(String::from))
        .collect();
    let mut id = next_req_id(existing_ids.iter().map(String::as_str));

    // Allocating an id and creating its file is not atomic across two
    // `weft new` invocations, so retry with the next id whenever the chosen
    // file already exists — `create_new` makes each attempt itself atomic.
    loop {
        let path = dir.join(format!("{id}.toml"));
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(skeleton_toml(&id, feat).as_bytes()) {
                    eprintln!("{}: {e}", path.display());
                    return ExitCode::FAILURE;
                }
                println!("{id}: {}", path.display());
                return ExitCode::SUCCESS;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                id = next_req_id(std::iter::once(id.as_str()));
            }
            Err(e) => {
                eprintln!("{}: {e}", path.display());
                return ExitCode::FAILURE;
            }
        }
    }
}

/// Directories never scanned for Trace Links: VCS metadata and build output.
const SCAN_EXCLUDES: &[&str] = &[".git", "target", "node_modules"];

/// The path to the optional, project-specific scan-exclude file (ADR 0010).
const WEFTIGNORE_PATH: &str = ".weftignore";

/// Reads project-specific scan excludes from `.weftignore` at `root`, one
/// basename per line. Blank lines and lines starting with `#` are skipped,
/// and a trailing `/` is stripped. Returns an empty list if the file doesn't
/// exist.
// @implements REQ-034 v2 b68c2987
fn load_weftignore(root: &Path) -> Vec<String> {
    let Ok(src) = fs::read_to_string(root.join(WEFTIGNORE_PATH)) else {
        return Vec::new();
    };
    src.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.trim_end_matches('/').to_string())
        .collect()
}

/// Recursively collects every file under `root`, skipping [`SCAN_EXCLUDES`]
/// directories and any name listed in `extra_excludes` (from `.weftignore`).
fn find_scannable_files(root: &Path, extra_excludes: &[String], out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
                SCAN_EXCLUDES.contains(&n) || extra_excludes.iter().any(|e| e == n)
            }) {
                continue;
            }
            find_scannable_files(&path, extra_excludes, out);
        } else {
            out.push(path);
        }
    }
}

/// The path to the Weft Lock: the committed record of each annotated file's
/// File Hash at last Seal.
const LOCK_PATH: &str = "docs/prds/weft.lock";

/// The path to the Run Lock: the committed record of each requirement's last
/// Verification Run, pinned to its Content Hash and annotated-file hashes.
const RUN_LOCK_PATH: &str = "docs/prds/weft.run.toml";

/// The path to the project's configuration: a `[test]` section declares the
/// Test Command (ADR 0014).
const WEFT_TOML_PATH: &str = "weft.toml";

/// Strips a leading `./` from `path` so paths read consistently whether they
/// were collected by walking `.` or referenced directly, matching the
/// `weft.lock` path format (e.g. `src/login.rs`, not `./src/login.rs`).
fn normalize_path(path: &Path) -> String {
    path.strip_prefix("./")
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

/// Scans every file under `root` (skipping [`SCAN_EXCLUDES`] directories and
/// any `.weftignore` entries) for Trace Links, returning each file's
/// normalized path paired with the annotations found in it (possibly empty).
fn scan_file_annotations(root: &Path) -> Vec<(String, Vec<Annotation>)> {
    let extra_excludes = load_weftignore(root);
    let mut scan_files = Vec::new();
    find_scannable_files(root, &extra_excludes, &mut scan_files);

    scan_files
        .into_iter()
        .filter_map(|file| {
            let src = fs::read_to_string(&file).ok()?;
            let annotations = scan_annotations(&src);
            Some((normalize_path(&file), annotations))
        })
        .collect()
}

// @implements REQ-014 v2 d217a603
// @implements REQ-033 v2 04d42b48
// @implements REQ-036 v2 4cbbd466
// @implements REQ-037 v2 2371e246
// @implements REQ-040 v2 1ead8691
// @implements REQ-041 v2 7194e93b
// @implements REQ-044 v2 a74590fa
fn check_cmd(summary: bool, json: bool) -> ExitCode {
    let mut req_files = Vec::new();
    find_toml_files(Path::new("docs/prds"), &mut req_files);
    req_files.sort();

    let mut requirements = Vec::new();
    for file in &req_files {
        match load_requirement(file) {
            Ok(req) => requirements.push(req),
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        }
    }
    requirements.retain(|req| req.status == Status::Active);
    requirements.sort_by(|a, b| a.id.cmp(&b.id));

    let file_annotations = scan_file_annotations(Path::new("."));
    let annotations: Vec<Annotation> = file_annotations
        .iter()
        .flat_map(|(_, annotations)| annotations.iter().cloned())
        .collect();

    let lock = fs::read_to_string(LOCK_PATH)
        .map(|src| parse_lock(&src))
        .unwrap_or_default();

    let current_hashes: std::collections::BTreeMap<String, String> =
        all_annotated_files(&file_annotations)
            .into_iter()
            .filter_map(|path| {
                let bytes = fs::read(&path).ok()?;
                Some((path, file_hash(&bytes)))
            })
            .collect();

    let run_lock = fs::read_to_string(RUN_LOCK_PATH)
        .map(|src| parse_run_lock(&src))
        .unwrap_or_default();

    let mut ok = true;
    let mut states = Vec::new();
    let mut checks = Vec::new();
    for req in &requirements {
        let req_files = files_for_requirement(&req.id, &file_annotations);
        let drifted = drifted_paths(&req_files, &lock, &current_hashes);
        let req_file_hashes: BTreeMap<String, String> = req_files
            .iter()
            .filter_map(|path| current_hashes.get(path).map(|hash| (path.clone(), hash.clone())))
            .collect();

        let check = check_requirement(req, &annotations, drifted);
        let check = verify_check(check, req, run_lock.get(&req.id), &req_file_hashes);
        if !matches!(check.state, TraceState::Traced | TraceState::Verified) {
            ok = false;
        }
        if !summary && !json {
            println!("{check}");
        }
        states.push(check.state.clone());
        checks.push(check);
    }

    if json {
        println!("{}", serde_json::to_string(&checks).expect("serialize check results"));
    } else if summary {
        println!("{}", summarize_trace_states(&states));
    }

    let active_ids: Vec<String> = requirements.iter().map(|req| req.id.clone()).collect();
    for (path, annotation) in dangling_annotations(&file_annotations, &active_ids) {
        ok = false;
        if json {
            continue;
        }
        let line = fs::read_to_string(path)
            .ok()
            .and_then(|src| annotation_line(&src, annotation));
        match line {
            Some(line) => println!("{path}:{line}: dangling {} {}", annotation.kind, annotation.req_id),
            None => println!("{path}: dangling {} {}", annotation.kind, annotation.req_id),
        }
    }

    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Rewrites a requirement record's `version` and `hash` lines in place,
/// leaving everything else (formatting, commentary) untouched.
fn rewrite_bumped(src: &str, version: u32, hash: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    for line in src.lines() {
        if line.trim_start().starts_with("version =") {
            out.push(format!("version = {version}"));
        } else if line.trim_start().starts_with("hash =") {
            out.push(format!("hash = \"{hash}\""));
        } else {
            out.push(line.to_string());
        }
    }
    let mut result = out.join("\n");
    result.push('\n');
    result
}

// @implements REQ-011 v2 5429311c
fn bump_cmd(req_id: &str) -> ExitCode {
    let mut files = Vec::new();
    find_toml_files(Path::new("docs/prds"), &mut files);

    for file in &files {
        if file.file_stem().and_then(|s| s.to_str()) != Some(req_id) {
            continue;
        }

        let src = match fs::read_to_string(file) {
            Ok(src) => src,
            Err(e) => {
                eprintln!("{}: {e}", file.display());
                return ExitCode::FAILURE;
            }
        };
        let req = match Requirement::parse(&src) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("{}: {e}", file.display());
                return ExitCode::FAILURE;
            }
        };

        let bumped = bump(&req);
        let rewritten = rewrite_bumped(&src, bumped.version, &bumped.hash);
        if let Err(e) = fs::write(file, rewritten) {
            eprintln!("{}: {e}", file.display());
            return ExitCode::FAILURE;
        }

        println!(
            "{req_id}: v{} -> v{} ({})",
            req.version, bumped.version, bumped.hash
        );
        return ExitCode::SUCCESS;
    }

    eprintln!("requirement '{req_id}' not found under docs/prds");
    ExitCode::FAILURE
}

// @implements REQ-010 v2 59117494
fn list_cmd(feat: Option<&str>) -> ExitCode {
    let mut files = Vec::new();
    find_toml_files(Path::new("docs/prds"), &mut files);
    files.sort();

    for file in &files {
        let req = match load_requirement(file) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        };

        if req.status != Status::Active {
            continue;
        }

        if let Some(feat) = feat {
            if req.feat.as_deref() != Some(feat) {
                continue;
            }
        }

        println!("{}: {}", req.id, description(&req.statement));
    }

    ExitCode::SUCCESS
}

// @implements REQ-012 v2 8afcf842
fn render_cmd() -> ExitCode {
    let mut files = Vec::new();
    find_toml_files(Path::new("docs/prds"), &mut files);
    files.sort();

    let mut requirements = Vec::new();
    for file in &files {
        match load_requirement(file) {
            Ok(req) => requirements.push(req),
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        }
    }

    print!("{}", render_markdown(&requirements));
    ExitCode::SUCCESS
}

/// Parses the YAML frontmatter of a SKILL.md file and extracts the `name` and
/// `description` fields. Handles both inline (`description: text`) and folded-
/// block (`description: >\n  line1\n  line2`) forms by joining continuation
/// lines with a single space.
fn parse_skill_frontmatter(content: &str) -> Option<(String, String)> {
    let trimmed = content.trim_start_matches('\n');
    if !trimmed.starts_with("---") {
        return None;
    }
    let after_open = trimmed["---".len()..].trim_start_matches('\n');
    let close = after_open.find("\n---")?;
    let frontmatter = &after_open[..close];

    let mut name: Option<String> = None;
    let mut description: Option<String> = None;
    let mut in_desc_block = false;
    let mut desc_lines: Vec<&str> = Vec::new();

    for line in frontmatter.lines() {
        if in_desc_block {
            if line.starts_with("  ") || line.starts_with('\t') {
                desc_lines.push(line.trim());
                continue;
            }
            in_desc_block = false;
        }
        if let Some(val) = line.strip_prefix("name:") {
            name = Some(val.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("description:") {
            let rest = rest.trim();
            if rest == ">" {
                in_desc_block = true;
                desc_lines.clear();
            } else {
                description = Some(rest.to_string());
            }
        }
    }

    if description.is_none() && !desc_lines.is_empty() {
        description = Some(desc_lines.join(" "));
    }

    match (name, description) {
        (Some(n), Some(d)) => Some((n, d)),
        _ => None,
    }
}

/// Builds the CONTEXT.md content from the list of installed skills.
fn build_context_md(skills: &[(String, String)]) -> String {
    let mut out = String::from(
        "# Project Context\n\
         \n\
         ## Requirements\n\
         \n\
         This project uses **weft** for requirements traceability. Load the `weft` skill\n\
         for any requirements-related activity (weft CLI, trace annotations, Trace State\n\
         workflow, `.scratch/` issue tracker conventions).\n\
         \n\
         Run `target/debug/weft` (or the installed `weft` binary) to invoke commands.\n\
         \n\
         ## Issue Tracker\n\
         \n\
         Issues and implementation plans live under `.scratch/`, one subdirectory per\n\
         feature (e.g. `.scratch/feat-auth/`). Load the `issue-tracker` skill when\n\
         creating, reading, or planning work in `.scratch/`.\n\
         \n\
         ## Architecture Decisions\n\
         \n\
         - `docs/adr/` — Architecture Decision Records\n\
         - `docs/decisions/` — Design decisions\n\
         \n\
         ## Installed Skills\n\
         \n\
         | Skill | Trigger |\n\
         |-------|---------|\n",
    );

    for (name, description) in skills {
        out.push_str(&format!("| `{name}` | {description} |\n"));
    }

    out.push('\n');
    out
}

// @implements REQ-049 v2 a5273df5
fn write_context_md_if_absent() -> Result<(), String> {
    let context_path = Path::new("CONTEXT.md");
    if context_path.exists() {
        return Ok(());
    }

    let mut skills: Vec<(String, String)> = EMBEDDED_SKILLS
        .iter()
        .filter(|(rel_path, _)| rel_path.ends_with("/SKILL.md") || *rel_path == "SKILL.md")
        .filter_map(|(_, content)| parse_skill_frontmatter(content))
        .collect();
    skills.sort_by(|(a, _), (b, _)| a.cmp(b));
    skills.dedup_by(|(a, _), (b, _)| a == b);

    let content = build_context_md(&skills);
    fs::write(context_path, content).map_err(|e| format!("CONTEXT.md: {e}"))?;
    println!("created CONTEXT.md");
    Ok(())
}

// @implements REQ-013 v5 b7c46a27
// @implements REQ-035 v2 1e646999
// @implements REQ-048 v2 94fdea44
fn init_cmd() -> ExitCode {
    let dirs = ["docs/prds", "docs/decisions"];
    let mut ok = true;

    for dir in &dirs {
        let path = Path::new(dir);
        if let Err(e) = fs::create_dir_all(path) {
            eprintln!("{}: {e}", path.display());
            ok = false;
        } else {
            println!("created {}", path.display());
        }
    }

    let weftignore = Path::new(WEFTIGNORE_PATH);
    if !weftignore.exists() {
        match fs::write(weftignore, ".scratch\nlogs\n") {
            Ok(()) => println!("created {}", weftignore.display()),
            Err(e) => {
                eprintln!("{}: {e}", weftignore.display());
                ok = false;
            }
        }
    }

    if !ok {
        return ExitCode::FAILURE;
    }

    let provider = detect_or_prompt_provider();
    let skills_dest = match provider.as_str() {
        "claude" => PathBuf::from(".claude/skills"),
        "copilot" => PathBuf::from(".github/copilot"),
        _ => {
            println!("note: AI provider not selected, skipping skill installation");
            return ExitCode::SUCCESS;
        }
    };

    if let Err(e) = fs::create_dir_all(&skills_dest) {
        eprintln!("{}: {e}", skills_dest.display());
        return ExitCode::FAILURE;
    }

    if let Err(e) = install_embedded_skills(&skills_dest) {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    if let Err(e) = write_context_md_if_absent() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn detect_or_prompt_provider() -> String {
    if Path::new(".claude").is_dir() {
        println!("detected Claude Code project (.claude/ found)");
        return "claude".to_string();
    }
    if Path::new(".github/copilot-instructions.md").exists()
        || Path::new(".github/copilot").is_dir()
    {
        println!("detected Copilot project (.github/copilot found)");
        return "copilot".to_string();
    }

    print!("AI provider not detected. Choose [claude/copilot]: ");
    let _ = io::stdout().flush();
    let stdin = io::stdin();
    let answer = stdin.lock().lines().next().unwrap_or(Ok(String::new())).unwrap_or_default();
    match answer.trim().to_lowercase().as_str() {
        "claude" => "claude".to_string(),
        "copilot" => "copilot".to_string(),
        other => {
            eprintln!("unknown provider: {other:?}, expected claude or copilot");
            String::new()
        }
    }
}

// @implements REQ-048 v2 94fdea44
fn install_embedded_skills(dest: &Path) -> Result<(), String> {
    // Group entries by top-level skill directory name
    let mut skill_names: Vec<&str> = EMBEDDED_SKILLS
        .iter()
        .filter_map(|(rel_path, _)| rel_path.split('/').next())
        .collect();
    skill_names.sort_unstable();
    skill_names.dedup();

    for skill_name in skill_names {
        let skill_dest = dest.join(skill_name);
        if skill_dest.exists() {
            println!("skipped {} (already exists)", skill_dest.display());
            continue;
        }
        for &(rel_path, content) in EMBEDDED_SKILLS {
            if !rel_path.starts_with(skill_name) {
                continue;
            }
            let file_dest = dest.join(rel_path);
            if let Some(parent) = file_dest.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("{}: {e}", parent.display()))?;
            }
            fs::write(&file_dest, content)
                .map_err(|e| format!("{}: {e}", file_dest.display()))?;
        }
        println!("installed {}", skill_dest.display());
    }
    Ok(())
}

/// Rewrites a requirement record's `status` line to `"deprecated"`, leaving
/// everything else untouched.  Idempotent: safe to call on a record that is
/// already deprecated.
fn rewrite_deprecated(src: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    for line in src.lines() {
        if line.trim_start().starts_with("status =") {
            out.push("status = \"deprecated\"".to_string());
        } else {
            out.push(line.to_string());
        }
    }
    let mut result = out.join("\n");
    result.push('\n');
    result
}

// @implements REQ-031 v2 6cdbe6cb
// @implements REQ-032 v2 a2441bcc
fn seal_cmd(req_id: Option<&str>) -> ExitCode {
    let file_annotations = scan_file_annotations(Path::new("."));

    let existing_lock = fs::read_to_string(LOCK_PATH)
        .map(|src| parse_lock(&src))
        .unwrap_or_default();

    let targets = match req_id {
        Some(req_id) => files_for_requirement(req_id, &file_annotations),
        None => all_annotated_files(&file_annotations),
    };

    let mut lock = match req_id {
        // Targeted seal updates entries for the targeted files only, leaving
        // every other entry untouched.
        Some(_) => existing_lock,
        // A full seal rebuilds the lock from scratch, pruning entries for
        // files that no longer carry any Trace Link.
        None => std::collections::BTreeMap::new(),
    };

    for path in &targets {
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("{path}: {e}");
                return ExitCode::FAILURE;
            }
        };
        lock.insert(path.clone(), file_hash(&bytes));
    }

    if let Some(parent) = Path::new(LOCK_PATH).parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("{}: {e}", parent.display());
            return ExitCode::FAILURE;
        }
    }
    if let Err(e) = fs::write(LOCK_PATH, render_lock(&lock)) {
        eprintln!("{LOCK_PATH}: {e}");
        return ExitCode::FAILURE;
    }

    println!("sealed {} file(s) into {LOCK_PATH}", targets.len());
    ExitCode::SUCCESS
}

/// Runs `cmd` as an opaque shell command via `sh -c`, returning whether it
/// exited successfully. weft reads only the exit code — never the command's
/// output or test-framework internals — to preserve language-agnosticism
/// (ADR 0014).
fn run_test_command(cmd: &str) -> bool {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

// @implements REQ-042 v2 37857355
// @implements REQ-043 v3 ceb81bfe
fn test_cmd(req_id: Option<&str>) -> ExitCode {
    let config = fs::read_to_string(WEFT_TOML_PATH)
        .ok()
        .and_then(|src| parse_test_config(&src));

    let Some(config) = config else {
        eprintln!("no test command configured: add a [test] section to weft.toml");
        return ExitCode::FAILURE;
    };

    let mut req_files = Vec::new();
    find_toml_files(Path::new("docs/prds"), &mut req_files);
    req_files.sort();

    let mut requirements = Vec::new();
    for file in &req_files {
        match load_requirement(file) {
            Ok(req) => requirements.push(req),
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        }
    }
    requirements.retain(|req| req.status == Status::Active);
    requirements.sort_by(|a, b| a.id.cmp(&b.id));

    if let Some(req_id) = req_id {
        if !requirements.iter().any(|req| req.id == req_id) {
            eprintln!("requirement '{req_id}' not found under docs/prds");
            return ExitCode::FAILURE;
        }
        requirements.retain(|req| req.id == req_id);
    }

    let file_annotations = scan_file_annotations(Path::new("."));

    let existing_lock = fs::read_to_string(RUN_LOCK_PATH)
        .map(|src| parse_run_lock(&src))
        .unwrap_or_default();

    // A targeted run preserves every other requirement's recorded result; a
    // full run rebuilds the Run Lock from scratch, like a full `weft seal`.
    let mut lock = match req_id {
        Some(_) => existing_lock,
        None => BTreeMap::new(),
    };

    let mut ok = true;
    let mut command_results: HashMap<String, bool> = HashMap::new();

    for req in &requirements {
        let result = match resolve_test_command(&config, req) {
            None => TestResult::Unrun,
            Some(cmd) => {
                let passed = *command_results
                    .entry(cmd.to_string())
                    .or_insert_with(|| run_test_command(cmd));
                if passed {
                    TestResult::Passed
                } else {
                    TestResult::Failed
                }
            }
        };

        if result != TestResult::Passed {
            ok = false;
        }
        println!("{}: {result}", req.id);

        let file_hashes: BTreeMap<String, String> =
            files_for_requirement(&req.id, &file_annotations)
                .into_iter()
                .filter_map(|path| {
                    let bytes = fs::read(&path).ok()?;
                    Some((path, file_hash(&bytes)))
                })
                .collect();

        lock.insert(
            req.id.clone(),
            RunRecord {
                result,
                content_hash: req.hash.clone(),
                file_hashes,
            },
        );
    }

    if let Some(parent) = Path::new(RUN_LOCK_PATH).parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("{}: {e}", parent.display());
            return ExitCode::FAILURE;
        }
    }
    if let Err(e) = fs::write(RUN_LOCK_PATH, render_run_lock(&lock)) {
        eprintln!("{RUN_LOCK_PATH}: {e}");
        return ExitCode::FAILURE;
    }

    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

// @implements REQ-038 v2 82357796
fn trace_cmd(req_id: &str) -> ExitCode {
    let mut files = Vec::new();
    find_toml_files(Path::new("docs/prds"), &mut files);

    let found = files
        .iter()
        .any(|file| file.file_stem().and_then(|s| s.to_str()) == Some(req_id));
    if !found {
        eprintln!("requirement '{req_id}' not found under docs/prds");
        return ExitCode::FAILURE;
    }

    let file_annotations = scan_file_annotations(Path::new("."));
    let mut links: Vec<(String, String, usize)> = Vec::new();
    for (path, annotations) in &file_annotations {
        for annotation in annotations {
            if annotation.req_id != req_id {
                continue;
            }
            if let Some(line) =
                annotation_line(&fs::read_to_string(path).unwrap_or_default(), annotation)
            {
                links.push((annotation.kind.to_string(), path.clone(), line));
            }
        }
    }

    if links.is_empty() {
        println!("Orphaned");
    } else {
        for (kind, path, line) in links {
            println!("{kind} {path}:{line}");
        }
    }

    ExitCode::SUCCESS
}

// @implements REQ-039 v3 2900b820
fn annotate_cmd(req_id: &str, kind: &AnnotateKind) -> ExitCode {
    let mut files = Vec::new();
    find_toml_files(Path::new("docs/prds"), &mut files);

    for file in &files {
        if file.file_stem().and_then(|s| s.to_str()) != Some(req_id) {
            continue;
        }

        let req = match load_requirement(file) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        };

        match kind {
            AnnotateKind::Addresses => println!("\"{req_id} v{} {}\"", req.version, req.hash),
            AnnotateKind::Implements => println!("@implements {req_id} v{} {}", req.version, req.hash),
            AnnotateKind::Verifies => println!("@verifies {req_id} v{} {}", req.version, req.hash),
        }
        return ExitCode::SUCCESS;
    }

    eprintln!("requirement '{req_id}' not found under docs/prds");
    ExitCode::FAILURE
}

// @implements REQ-045 v2 12e173a6
fn gate_cmd() -> ExitCode {
    let mut req_files = Vec::new();
    find_toml_files(Path::new("docs/prds"), &mut req_files);
    req_files.sort();

    let mut requirements = Vec::new();
    for file in &req_files {
        match load_requirement(file) {
            Ok(req) => requirements.push(req),
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        }
    }
    requirements.retain(|req| req.status == Status::Active);
    requirements.sort_by(|a, b| a.id.cmp(&b.id));

    let file_annotations = scan_file_annotations(Path::new("."));
    let annotations: Vec<Annotation> = file_annotations
        .iter()
        .flat_map(|(_, annotations)| annotations.iter().cloned())
        .collect();

    let lock = fs::read_to_string(LOCK_PATH)
        .map(|src| parse_lock(&src))
        .unwrap_or_default();

    let current_hashes: BTreeMap<String, String> =
        all_annotated_files(&file_annotations)
            .into_iter()
            .filter_map(|path| {
                let bytes = fs::read(&path).ok()?;
                Some((path, file_hash(&bytes)))
            })
            .collect();

    let run_lock = fs::read_to_string(RUN_LOCK_PATH)
        .map(|src| parse_run_lock(&src))
        .unwrap_or_default();

    let mut all_verified = true;
    for req in &requirements {
        let req_files = files_for_requirement(&req.id, &file_annotations);
        let drifted = drifted_paths(&req_files, &lock, &current_hashes);
        let req_file_hashes: BTreeMap<String, String> = req_files
            .iter()
            .filter_map(|path| current_hashes.get(path).map(|hash| (path.clone(), hash.clone())))
            .collect();

        let check = check_requirement(req, &annotations, drifted);
        let check = verify_check(check, req, run_lock.get(&req.id), &req_file_hashes);

        if check.state != TraceState::Verified {
            all_verified = false;
            println!("{}: {}", req.id, check.state);
        }
    }

    if all_verified {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// The action verb an agent must perform to advance the selected requirement,
/// derived from its blocking condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActionVerb {
    Implement,
    Rework,
    Reseal,
    FixTests,
    RunTests,
}

impl ActionVerb {
    fn as_str(self) -> &'static str {
        match self {
            ActionVerb::Implement => "implement",
            ActionVerb::Rework => "rework",
            ActionVerb::Reseal => "reseal",
            ActionVerb::FixTests => "fix-tests",
            ActionVerb::RunTests => "run-tests",
        }
    }
}

/// Priority score for a non-Verified requirement (lower = higher priority).
/// Regressions-first, stable total order as defined in ADR 0014.
fn next_priority(state: &TraceState, run_record: Option<&RunRecord>) -> u8 {
    match state {
        TraceState::Traced
            if matches!(run_record, Some(r) if r.result == weft_core::TestResult::Failed) =>
        {
            1 // Traced-with-failing-tests
        }
        TraceState::Drifted(_) => 2,
        TraceState::Stale => 3,
        TraceState::Incomplete => 4,
        TraceState::Orphaned => 5,
        TraceState::Traced => 6, // without a recorded test run
        TraceState::Verified => 7,
    }
}

fn action_verb(state: &TraceState, run_record: Option<&RunRecord>) -> ActionVerb {
    match state {
        TraceState::Traced
            if matches!(run_record, Some(r) if r.result == weft_core::TestResult::Failed) =>
        {
            ActionVerb::FixTests
        }
        TraceState::Drifted(_) => ActionVerb::Reseal,
        TraceState::Stale => ActionVerb::Rework,
        TraceState::Incomplete | TraceState::Orphaned => ActionVerb::Implement,
        TraceState::Traced => ActionVerb::RunTests,
        TraceState::Verified => unreachable!("Verified requirements are not selected by next"),
    }
}

/// Returns the annotation strings for each relevant link kind for `implement`
/// or `rework` actions. Keys are "addresses", "implements", "verifies".
fn annotation_strings(
    req: &weft_core::Requirement,
    action: ActionVerb,
    gap: &weft_core::TraceGap,
) -> BTreeMap<String, String> {
    if !matches!(action, ActionVerb::Implement | ActionVerb::Rework) {
        return BTreeMap::new();
    }
    let kinds: Vec<&str> = match action {
        ActionVerb::Implement => {
            if gap.missing_links.is_empty() {
                // Orphaned: all three links are missing
                vec!["addresses", "implements", "verifies"]
            } else {
                gap.missing_links.iter().map(String::as_str).collect()
            }
        }
        ActionVerb::Rework => gap.stale_links.iter().map(|l| l.kind.as_str()).collect(),
        _ => unreachable!(),
    };
    let id = &req.id;
    let v = req.version;
    let h = &req.hash;
    kinds
        .into_iter()
        .map(|kind| {
            let annotation = match kind {
                "addresses" => format!("\"{id} v{v} {h}\""),
                "implements" => format!("@implements {id} v{v} {h}"),
                "verifies" => format!("@verifies {id} v{v} {h}"),
                _ => format!("@{kind} {id} v{v} {h}"),
            };
            (kind.to_string(), annotation)
        })
        .collect()
}

// @implements REQ-046 v3 fb90b6b7
fn next_cmd(json: bool) -> ExitCode {
    let mut req_files = Vec::new();
    find_toml_files(Path::new("docs/prds"), &mut req_files);
    req_files.sort();

    let mut requirements = Vec::new();
    for file in &req_files {
        match load_requirement(file) {
            Ok(req) => requirements.push(req),
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        }
    }
    requirements.retain(|req| req.status == weft_core::Status::Active);
    requirements.sort_by(|a, b| a.id.cmp(&b.id));

    let file_annotations = scan_file_annotations(Path::new("."));
    let annotations: Vec<weft_core::Annotation> = file_annotations
        .iter()
        .flat_map(|(_, a)| a.iter().cloned())
        .collect();

    let lock = fs::read_to_string(LOCK_PATH)
        .map(|src| parse_lock(&src))
        .unwrap_or_default();

    let current_hashes: BTreeMap<String, String> =
        all_annotated_files(&file_annotations)
            .into_iter()
            .filter_map(|path| {
                let bytes = fs::read(&path).ok()?;
                Some((path, file_hash(&bytes)))
            })
            .collect();

    let run_lock = fs::read_to_string(RUN_LOCK_PATH)
        .map(|src| parse_run_lock(&src))
        .unwrap_or_default();

    // Build (priority, check, req) for all non-Verified requirements.
    let mut candidates: Vec<(u8, weft_core::RequirementCheck, weft_core::Requirement)> = Vec::new();
    for req in &requirements {
        let req_files = files_for_requirement(&req.id, &file_annotations);
        let drifted = drifted_paths(&req_files, &lock, &current_hashes);
        let req_file_hashes: BTreeMap<String, String> = req_files
            .iter()
            .filter_map(|path| current_hashes.get(path).map(|h| (path.clone(), h.clone())))
            .collect();

        let check = check_requirement(req, &annotations, drifted);
        let run_record = run_lock.get(&req.id);
        let check = verify_check(check, req, run_record, &req_file_hashes);

        if check.state == weft_core::TraceState::Verified {
            continue;
        }

        let priority = next_priority(&check.state, run_record);
        candidates.push((priority, check, req.clone()));
    }

    if candidates.is_empty() {
        if json {
            println!("{{\"status\":\"no_next_work_item\"}}");
        } else {
            println!("no next work item");
        }
        return ExitCode::SUCCESS;
    }

    // Select the highest-priority candidate (lowest priority score), breaking
    // ties by REQ_ID for a stable, reproducible selection.
    candidates.sort_by(|(pa, ca, _), (pb, cb, _)| pa.cmp(pb).then(ca.id.cmp(&cb.id)));
    let (_, check, req) = &candidates[0];

    let run_record = run_lock.get(&req.id);
    let action = action_verb(&check.state, run_record);
    let annotations_map = annotation_strings(req, action, &check.gap);

    if json {
        let mut obj = serde_json::Map::new();
        obj.insert("id".into(), serde_json::Value::String(req.id.clone()));
        obj.insert("action".into(), serde_json::Value::String(action.as_str().into()));
        obj.insert("state".into(), serde_json::Value::String(check.state.name().into()));
        obj.insert("statement".into(), serde_json::Value::String(req.statement.clone()));
        obj.insert("missing_links".into(), serde_json::Value::Array(
            check.gap.missing_links.iter().map(|s| serde_json::Value::String(s.clone())).collect(),
        ));
        obj.insert("stale_links".into(), serde_json::json!(check.gap.stale_links));
        obj.insert("drifted_files".into(), serde_json::Value::Array(
            check.gap.drifted_files.iter().map(|s| serde_json::Value::String(s.clone())).collect(),
        ));
        if !annotations_map.is_empty() {
            let ann_obj: serde_json::Map<String, serde_json::Value> = annotations_map
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            obj.insert("annotations".into(), serde_json::Value::Object(ann_obj));
        }
        println!("{}", serde_json::Value::Object(obj));
    } else {
        println!("{}: {}", req.id, action.as_str());
        println!("state: {}", check.state.name());
        println!("statement: {}", description(&req.statement));
        if !check.gap.missing_links.is_empty() {
            println!("missing: {}", check.gap.missing_links.join(", "));
        }
        if !check.gap.stale_links.is_empty() {
            let parts: Vec<String> = check.gap.stale_links.iter()
                .map(|l| format!("{} (recorded: {}, current: {})", l.kind, l.recorded_hash, l.current_hash))
                .collect();
            println!("stale: {}", parts.join("; "));
        }
        if !check.gap.drifted_files.is_empty() {
            println!("drifted: {}", check.gap.drifted_files.join(", "));
        }
        if action == ActionVerb::FixTests {
            println!("gap: tests failed");
        } else if action == ActionVerb::RunTests {
            println!("gap: no recorded test run");
        }
        for (kind, annotation) in &annotations_map {
            println!("annotation {kind}: {annotation}");
        }
    }

    ExitCode::FAILURE
}

// @implements REQ-015 v2 3d05542c
fn deprecate_cmd(req_id: &str) -> ExitCode {
    let mut files = Vec::new();
    find_toml_files(Path::new("docs/prds"), &mut files);

    for file in &files {
        if file.file_stem().and_then(|s| s.to_str()) != Some(req_id) {
            continue;
        }

        let src = match fs::read_to_string(file) {
            Ok(src) => src,
            Err(e) => {
                eprintln!("{}: {e}", file.display());
                return ExitCode::FAILURE;
            }
        };

        let rewritten = rewrite_deprecated(&src);
        if let Err(e) = fs::write(file, rewritten) {
            eprintln!("{}: {e}", file.display());
            return ExitCode::FAILURE;
        }

        println!("{req_id}: deprecated");
        return ExitCode::SUCCESS;
    }

    eprintln!("requirement '{req_id}' not found under docs/prds");
    ExitCode::FAILURE
}
