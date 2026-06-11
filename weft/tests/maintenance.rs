use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use weft_core::canonical_hash;

/// Recursively collects every file under `root`, skipping VCS metadata and
/// build output (mirrors `weft check`'s `SCAN_EXCLUDES`).
fn collect_files(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if matches!(
                path.file_name().and_then(|n| n.to_str()),
                Some(".git") | Some("target") | Some("node_modules")
            ) {
                continue;
            }
            collect_files(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// The repository root, two levels up from this crate's manifest dir
/// (`weft/` -> repo root).
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .expect("repo root")
}

// @verifies REQ-029 v2 6f452ce5
#[test]
fn repository_has_no_legacy_python_installer() {
    let mut files = Vec::new();
    collect_files(&repo_root(), &mut files);

    let py_files: Vec<&PathBuf> = files
        .iter()
        .filter(|f| f.extension().and_then(|e| e.to_str()) == Some("py"))
        .collect();

    assert!(
        py_files.is_empty(),
        "expected no legacy Python installer files, found: {py_files:?}"
    );
}

// @verifies REQ-029 v2 6f452ce5
#[test]
fn cli_provides_no_save_or_sync_subcommands() {
    let output = Command::cargo_bin("weft")
        .unwrap()
        .arg("--help")
        .output()
        .expect("run weft --help");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    let subcommands: Vec<&str> = stdout
        .lines()
        .skip_while(|line| *line != "Commands:")
        .skip(1)
        .take_while(|line| line.starts_with("  "))
        .filter_map(|line| line.trim_start().split_whitespace().next())
        .collect();

    assert!(
        !subcommands.is_empty(),
        "expected to find at least one subcommand in: {stdout}"
    );
    assert!(
        !subcommands.contains(&"save") && !subcommands.contains(&"sync"),
        "expected no save/sync subcommands, found: {subcommands:?}"
    );
}

// @verifies REQ-030 v2 bf5f866e
#[test]
fn verify_rejects_a_persisted_user_story_record() {
    const STATEMENT: &str = "The system must allow users to log in with email and password.";
    const ACCEPTANCE: &[&str] = &["Given valid credentials, the user is authenticated."];
    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    let hash = canonical_hash(STATEMENT, &acceptance);

    let dir = TempDir::new().expect("create temp dir");
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let toml_src = format!(
        r#"
id = "REQ-001"
version = 1
hash = "{hash}"
status = "active"
statement = "{STATEMENT}"
acceptance = [
    "{a0}",
]
as_a = "developer"
i_want = "to log in with email and password"
so_that = "I can access my account"
"#,
        a0 = ACCEPTANCE[0],
    );
    fs::write(prds_dir.join("REQ-001.toml"), toml_src).expect("write fixture");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("verify")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("User Story"));
}
