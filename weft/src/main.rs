use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use weft_core::{
    bump, description, next_req_id, render_markdown, scan_annotations, skeleton_toml, trace_state,
    verify_requirement, Annotation, Requirement, Status, TraceState,
};

#[derive(Parser)]
#[command(name = "weft")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

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
        let req = match load_requirement(file) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("{e}");
                ok = false;
                continue;
            }
        };

        let issues = verify_requirement(&req, id);
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
// @implements REQ-026 v1 placeholder
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

/// Recursively collects every file under `root`, skipping [`SCAN_EXCLUDES`]
/// directories.
fn find_scannable_files(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| SCAN_EXCLUDES.contains(&n))
            {
                continue;
            }
            find_scannable_files(&path, out);
        } else {
            out.push(path);
        }
    }
}

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

    let mut scan_files = Vec::new();
    find_scannable_files(Path::new("."), &mut scan_files);

    let mut annotations: Vec<Annotation> = Vec::new();
    for file in &scan_files {
        if let Ok(src) = fs::read_to_string(file) {
            annotations.extend(scan_annotations(&src));
        }
    }

    let mut ok = true;
    for req in &requirements {
        let state = trace_state(req, &annotations);
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

// @implements REQ-020 v1 placeholder
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

// @implements REQ-021 v1 placeholder
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

// @implements REQ-019 v1 placeholder
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
