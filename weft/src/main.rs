use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use weft_core::{
    all_annotated_files, bump, description, drifted_paths, file_hash, files_for_requirement,
    next_req_id, parse_lock, render_lock, render_markdown, scan_annotations, skeleton_toml,
    trace_state_with_drift, verify_not_user_story, verify_requirement, Annotation, Requirement,
    Status, TraceState,
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
    Check,
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
        Command::Check => check_cmd(),
        Command::Bump { req_id } => bump_cmd(&req_id),
        Command::Render => render_cmd(),
        Command::Init => init_cmd(),
        Command::Deprecate { req_id } => deprecate_cmd(&req_id),
        Command::Seal { req_id } => seal_cmd(req_id.as_deref()),
    }
}

/// Recursively collects `.toml` files under `root` (or returns `root` itself
/// if it is already a file).
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
        } else if path.extension().and_then(|e| e.to_str()) == Some("toml") {
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
fn check_cmd() -> ExitCode {
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

    let mut ok = true;
    for req in &requirements {
        let drifted = drifted_paths(
            &files_for_requirement(&req.id, &file_annotations),
            &lock,
            &current_hashes,
        );
        let state = trace_state_with_drift(req, &annotations, drifted);
        if state != TraceState::Traced {
            ok = false;
        }
        println!("{}: {state}", req.id);
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

// @implements REQ-013 v2 41174961
// @implements REQ-035 v2 1e646999
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

    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
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
