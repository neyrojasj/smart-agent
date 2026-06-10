use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use weft_core::{description, next_req_id, skeleton_toml, verify_requirement, Requirement};

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
}

#[derive(Clone, ValueEnum)]
enum Field {
    Statement,
    Acceptance,
    Hash,
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
        }
        return ExitCode::SUCCESS;
    }

    eprintln!("requirement '{req_id}' not found under docs/prds");
    ExitCode::FAILURE
}

fn new_cmd(feat: Option<&str>) -> ExitCode {
    let prds_root = Path::new("docs/prds");
    let mut files = Vec::new();
    find_toml_files(prds_root, &mut files);

    let existing_ids: Vec<String> = files
        .iter()
        .filter_map(|f| f.file_stem().and_then(|s| s.to_str()).map(String::from))
        .collect();
    let id = next_req_id(existing_ids.iter().map(String::as_str));

    let dir = match feat {
        Some(feat) => prds_root.join(feat),
        None => prds_root.to_path_buf(),
    };
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("{}: {e}", dir.display());
        return ExitCode::FAILURE;
    }

    let path = dir.join(format!("{id}.toml"));
    if let Err(e) = fs::write(&path, skeleton_toml(&id, feat)) {
        eprintln!("{}: {e}", path.display());
        return ExitCode::FAILURE;
    }

    println!("{id}: {}", path.display());
    ExitCode::SUCCESS
}

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

        if let Some(feat) = feat {
            if req.feat.as_deref() != Some(feat) {
                continue;
            }
        }

        println!("{}: {}", req.id, description(&req.statement));
    }

    ExitCode::SUCCESS
}
