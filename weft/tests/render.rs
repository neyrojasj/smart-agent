use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use weft_core::canonical_hash;

const STATEMENT_A: &str = "The system must allow users to log in with email and password.";
const ACCEPTANCE_A: &[&str] = &[
    "Given valid credentials, the user is authenticated.",
    "Given invalid credentials, an error is shown.",
];

const STATEMENT_B: &str = "The system must allow users to log out.";
const ACCEPTANCE_B: &[&str] = &["Given a logged-in user, logging out ends the session."];

fn project_with_two_requirements() -> TempDir {
    let dir = TempDir::new().expect("create temp dir");

    let acc_a: Vec<String> = ACCEPTANCE_A.iter().map(|s| s.to_string()).collect();
    let hash_a = canonical_hash(STATEMENT_A, &acc_a);

    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let req_001 = format!(
        r#"id = "REQ-001"
version = 1
hash = "{hash_a}"
status = "active"
statement = "{STATEMENT_A}"
acceptance = [
    "{a0}",
    "{a1}",
]
"#,
        a0 = ACCEPTANCE_A[0],
        a1 = ACCEPTANCE_A[1],
    );
    fs::write(prds_dir.join("REQ-001.toml"), req_001).expect("write REQ-001");

    let acc_b: Vec<String> = ACCEPTANCE_B.iter().map(|s| s.to_string()).collect();
    let hash_b = canonical_hash(STATEMENT_B, &acc_b);

    let auth_dir = prds_dir.join("FEAT-Auth");
    fs::create_dir_all(&auth_dir).expect("create FEAT-Auth");

    let req_002 = format!(
        r#"id = "REQ-002"
version = 2
feat = "FEAT-Auth"
hash = "{hash_b}"
status = "active"
statement = "{STATEMENT_B}"
acceptance = [
    "{a0}",
]
"#,
        a0 = ACCEPTANCE_B[0],
    );
    fs::write(auth_dir.join("REQ-002.toml"), req_002).expect("write REQ-002");

    dir
}

#[test]
fn render_emits_markdown_with_id_version_and_statement() {
    let dir = project_with_two_requirements();

    let output = Command::cargo_bin("weft")
        .unwrap()
        .arg("render")
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();

    assert!(stdout.contains("REQ-001"), "expected REQ-001 in: {stdout}");
    assert!(stdout.contains("REQ-002"), "expected REQ-002 in: {stdout}");
    assert!(
        stdout.contains(STATEMENT_A),
        "expected login statement in: {stdout}"
    );
    assert!(
        stdout.contains(STATEMENT_B),
        "expected logout statement in: {stdout}"
    );
    // version numbers present
    assert!(stdout.contains("v1"), "expected v1 in: {stdout}");
    assert!(stdout.contains("v2"), "expected v2 in: {stdout}");
}

#[test]
fn render_includes_acceptance_criteria() {
    let dir = project_with_two_requirements();

    Command::cargo_bin("weft")
        .unwrap()
        .arg("render")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(ACCEPTANCE_A[0]))
        .stdout(predicate::str::contains(ACCEPTANCE_A[1]))
        .stdout(predicate::str::contains(ACCEPTANCE_B[0]));
}

#[test]
fn render_output_is_valid_markdown_headers() {
    let dir = project_with_two_requirements();

    let output = Command::cargo_bin("weft")
        .unwrap()
        .arg("render")
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    // Each requirement should appear as a markdown heading (## or similar)
    assert!(
        stdout.contains("## REQ-001") || stdout.contains("# REQ-001"),
        "expected a markdown heading for REQ-001 in: {stdout}"
    );
    assert!(
        stdout.contains("## REQ-002") || stdout.contains("# REQ-002"),
        "expected a markdown heading for REQ-002 in: {stdout}"
    );
}
