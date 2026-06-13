use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use weft_core::canonical_hash;

const STATEMENT: &str = "The system must allow users to log in with email and password.";
const ACCEPTANCE: &[&str] = &[
    "Given valid credentials, the user is authenticated.",
    "Given invalid credentials, an error is shown.",
];

fn current_hash() -> String {
    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    canonical_hash(STATEMENT, &acceptance)
}

fn write_requirement(dir: &TempDir, version: u32, hash: &str) {
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let toml_src = format!(
        r#"
id = "REQ-901"
version = {version}
hash = "{hash}"
status = "active"
statement = "{STATEMENT}"
acceptance = [
    "{a0}",
    "{a1}",
]
"#,
        a0 = ACCEPTANCE[0],
        a1 = ACCEPTANCE[1],
    );
    fs::write(prds_dir.join("REQ-901.toml"), toml_src).expect("write REQ-901");
}

// @verifies REQ-039 v3 2900b820
#[test]
fn annotate_implements_prints_the_implements_trace_link_line() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, 1, &hash);

    Command::cargo_bin("weft")
        .unwrap()
        .args(["annotate", "REQ-901", "--kind", "implements"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("@implements REQ-901 v1 {hash}")));
}

// @verifies REQ-039 v3 2900b820
#[test]
fn annotate_verifies_prints_the_verifies_trace_link_line() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, 1, &hash);

    Command::cargo_bin("weft")
        .unwrap()
        .args(["annotate", "REQ-901", "--kind", "verifies"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("@verifies REQ-901 v1 {hash}")));
}

// @verifies REQ-039 v3 2900b820
#[test]
fn annotate_addresses_prints_the_frontmatter_entry_shape() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, 1, &hash);

    Command::cargo_bin("weft")
        .unwrap()
        .args(["annotate", "REQ-901", "--kind", "addresses"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("\"REQ-901 v1 {hash}\"")));
}

// @verifies REQ-039 v3 2900b820
#[test]
fn annotate_reflects_version_and_hash_after_a_bump() {
    let dir = TempDir::new().expect("create temp dir");
    // A stale hash forces `weft bump` to produce a new version + hash.
    write_requirement(&dir, 1, "deadbeef");

    Command::cargo_bin("weft")
        .unwrap()
        .args(["bump", "REQ-901"])
        .current_dir(dir.path())
        .assert()
        .success();

    let hash = current_hash();
    Command::cargo_bin("weft")
        .unwrap()
        .args(["annotate", "REQ-901", "--kind", "implements"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("@implements REQ-901 v2 {hash}")));
}

// @verifies REQ-039 v3 2900b820
#[test]
fn annotate_fails_for_nonexistent_requirement() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, 1, &hash);

    Command::cargo_bin("weft")
        .unwrap()
        .args(["annotate", "REQ-999", "--kind", "implements"])
        .current_dir(dir.path())
        .assert()
        .failure();
}
