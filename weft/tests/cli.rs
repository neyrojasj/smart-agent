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

/// Sets up a temp project with a single well-formed requirement record at
/// `docs/prds/REQ-001.toml`, with `hash` matching the current canonical hash.
fn project_with_well_formed_record() -> TempDir {
    let dir = TempDir::new().expect("create temp dir");
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    let hash = canonical_hash(STATEMENT, &acceptance);

    let toml_src = format!(
        r#"
id = "REQ-001"
version = 1
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

    fs::write(prds_dir.join("REQ-001.toml"), toml_src).expect("write fixture");
    dir
}

#[test]
fn get_returns_statement_for_a_fixture_record() {
    let dir = project_with_well_formed_record();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["get", "REQ-001", "--field", "statement"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(STATEMENT));
}

#[test]
fn get_returns_hash_for_a_fixture_record() {
    let dir = project_with_well_formed_record();
    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    let hash = canonical_hash(STATEMENT, &acceptance);

    Command::cargo_bin("weft")
        .unwrap()
        .args(["get", "REQ-001", "--field", "hash"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(&hash));
}

#[test]
fn get_returns_acceptance_for_a_fixture_record() {
    let dir = project_with_well_formed_record();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["get", "REQ-001", "--field", "acceptance"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(
            predicate::str::contains(ACCEPTANCE[0]).and(predicate::str::contains(ACCEPTANCE[1])),
        );
}

#[test]
fn verify_passes_for_a_well_formed_record() {
    let dir = project_with_well_formed_record();

    Command::cargo_bin("weft")
        .unwrap()
        .arg("verify")
        .current_dir(dir.path())
        .assert()
        .success();
}

#[test]
fn verify_fails_with_bump_message_for_a_stale_hash() {
    let dir = project_with_well_formed_record();
    let prd_path = dir.path().join("docs/prds/REQ-001.toml");
    let stale = fs::read_to_string(&prd_path)
        .unwrap()
        .replacen(&canonical_hash(STATEMENT, &ACCEPTANCE.iter().map(|s| s.to_string()).collect::<Vec<_>>()), "deadbeef", 1);
    fs::write(&prd_path, stale).unwrap();

    Command::cargo_bin("weft")
        .unwrap()
        .arg("verify")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("weft bump REQ-001"));
}
