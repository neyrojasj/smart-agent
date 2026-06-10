// @verifies REQ-019 v1 placeholder
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

fn project_with_active_requirement() -> TempDir {
    let dir = TempDir::new().expect("create temp dir");
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    let hash = canonical_hash(STATEMENT, &acceptance);

    let toml_src = format!(
        r#"id = "REQ-001"
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

// @verifies REQ-019 v1 placeholder
#[test]
fn deprecate_marks_requirement_as_deprecated() {
    let dir = project_with_active_requirement();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["deprecate", "REQ-001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("REQ-001"));

    let src = fs::read_to_string(dir.path().join("docs/prds/REQ-001.toml")).unwrap();
    assert!(
        src.contains("status = \"deprecated\""),
        "expected status = deprecated in:\n{src}"
    );
    assert!(
        !src.contains("status = \"active\""),
        "expected no status = active in:\n{src}"
    );
}

// @verifies REQ-019 v1 placeholder
#[test]
fn deprecate_is_idempotent_on_already_deprecated_record() {
    let dir = project_with_active_requirement();

    // First deprecation
    Command::cargo_bin("weft")
        .unwrap()
        .args(["deprecate", "REQ-001"])
        .current_dir(dir.path())
        .assert()
        .success();

    // Second deprecation on the same record — must not fail
    Command::cargo_bin("weft")
        .unwrap()
        .args(["deprecate", "REQ-001"])
        .current_dir(dir.path())
        .assert()
        .success();

    let src = fs::read_to_string(dir.path().join("docs/prds/REQ-001.toml")).unwrap();
    assert!(
        src.contains("status = \"deprecated\""),
        "still deprecated after second run:\n{src}"
    );
}

#[test]
fn deprecate_fails_for_nonexistent_requirement() {
    let dir = project_with_active_requirement();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["deprecate", "REQ-999"])
        .current_dir(dir.path())
        .assert()
        .failure();
}

// @verifies REQ-019 v1 placeholder
#[test]
fn deprecated_record_still_passes_verify() {
    let dir = project_with_active_requirement();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["deprecate", "REQ-001"])
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("weft")
        .unwrap()
        .arg("verify")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("REQ-001: ok"));
}
