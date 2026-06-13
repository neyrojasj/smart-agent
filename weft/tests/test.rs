use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use weft_core::{canonical_hash, parse_run_lock, RunRecord, TestResult};

const STATEMENT_901: &str = "The system must allow users to log in with email and password.";
const ACCEPTANCE_901: &[&str] = &[
    "Given valid credentials, the user is authenticated.",
    "Given invalid credentials, an error is shown.",
];

const STATEMENT_902: &str = "The system must allow users to log out.";
const ACCEPTANCE_902: &[&str] = &["Given a logged-in user, logging out ends the session."];

fn hash_901() -> String {
    let acceptance: Vec<String> = ACCEPTANCE_901.iter().map(|s| s.to_string()).collect();
    canonical_hash(STATEMENT_901, &acceptance)
}

fn hash_902() -> String {
    let acceptance: Vec<String> = ACCEPTANCE_902.iter().map(|s| s.to_string()).collect();
    canonical_hash(STATEMENT_902, &acceptance)
}

fn write_requirement(dir: &TempDir, id: &str, feat: Option<&str>, statement: &str, acceptance: &[&str], hash: &str) {
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let acceptance_lines: String = acceptance
        .iter()
        .map(|a| format!("    \"{a}\",\n"))
        .collect();

    let feat_line = match feat {
        Some(feat) => format!("feat = \"{feat}\"\n"),
        None => String::new(),
    };

    let toml_src = format!(
        r#"id = "{id}"
version = 1
{feat_line}hash = "{hash}"
status = "active"
statement = "{statement}"
acceptance = [
{acceptance_lines}]
"#
    );
    fs::write(prds_dir.join(format!("{id}.toml")), toml_src).expect("write requirement");
}

/// Sets up a temp project with two active requirements, REQ-901 and REQ-902,
/// neither carrying any Trace Links (irrelevant to `weft test`, which only
/// needs the requirement records and the configured Test Command).
fn project_with_two_requirements(dir: &TempDir) {
    write_requirement(dir, "REQ-901", None, STATEMENT_901, ACCEPTANCE_901, &hash_901());
    write_requirement(dir, "REQ-902", None, STATEMENT_902, ACCEPTANCE_902, &hash_902());
}

fn write_weft_toml(dir: &TempDir, contents: &str) {
    fs::write(dir.path().join("weft.toml"), contents).expect("write weft.toml");
}

fn run_lock(dir: &TempDir) -> std::collections::BTreeMap<String, RunRecord> {
    let src = fs::read_to_string(dir.path().join("docs/prds/weft.run.toml"))
        .expect("expected docs/prds/weft.run.toml to be written");
    parse_run_lock(&src)
}

// @verifies REQ-043 v3 ceb81bfe
#[test]
fn default_command_passing_records_every_requirement_as_passed() {
    let dir = TempDir::new().expect("create temp dir");
    project_with_two_requirements(&dir);
    write_weft_toml(
        &dir,
        r#"
[test]
command = "true"
"#,
    );

    Command::cargo_bin("weft")
        .unwrap()
        .arg("test")
        .current_dir(dir.path())
        .assert()
        .success();

    let lock = run_lock(&dir);

    let req_901 = lock.get("REQ-901").expect("expected REQ-901 entry");
    assert_eq!(req_901.result, TestResult::Passed);
    assert_eq!(req_901.content_hash, hash_901());

    let req_902 = lock.get("REQ-902").expect("expected REQ-902 entry");
    assert_eq!(req_902.result, TestResult::Passed);
    assert_eq!(req_902.content_hash, hash_902());
}

// @verifies REQ-043 v3 ceb81bfe
#[test]
fn default_command_failing_records_every_requirement_as_failed_and_exits_non_zero() {
    let dir = TempDir::new().expect("create temp dir");
    project_with_two_requirements(&dir);
    write_weft_toml(
        &dir,
        r#"
[test]
command = "false"
"#,
    );

    Command::cargo_bin("weft")
        .unwrap()
        .arg("test")
        .current_dir(dir.path())
        .assert()
        .failure();

    let lock = run_lock(&dir);

    assert_eq!(lock.get("REQ-901").unwrap().result, TestResult::Failed);
    assert_eq!(lock.get("REQ-902").unwrap().result, TestResult::Failed);
}

// @verifies REQ-043 v3 ceb81bfe
#[test]
fn per_requirement_override_reflects_its_own_command_independently() {
    let dir = TempDir::new().expect("create temp dir");
    project_with_two_requirements(&dir);
    write_weft_toml(
        &dir,
        r#"
[test]
command = "true"

[test.overrides]
"REQ-902" = "false"
"#,
    );

    Command::cargo_bin("weft")
        .unwrap()
        .arg("test")
        .current_dir(dir.path())
        .assert()
        .failure();

    let lock = run_lock(&dir);

    assert_eq!(lock.get("REQ-901").unwrap().result, TestResult::Passed);
    assert_eq!(lock.get("REQ-902").unwrap().result, TestResult::Failed);
}

// @verifies REQ-043 v3 ceb81bfe
#[test]
fn targeted_run_updates_only_that_requirements_record() {
    let dir = TempDir::new().expect("create temp dir");
    project_with_two_requirements(&dir);
    write_weft_toml(
        &dir,
        r#"
[test]
command = "true"
"#,
    );

    // Full run: both requirements pass.
    Command::cargo_bin("weft")
        .unwrap()
        .arg("test")
        .current_dir(dir.path())
        .assert()
        .success();

    // Switch the default command to "false" and re-run only REQ-901.
    write_weft_toml(
        &dir,
        r#"
[test]
command = "false"
"#,
    );

    Command::cargo_bin("weft")
        .unwrap()
        .args(["test", "REQ-901"])
        .current_dir(dir.path())
        .assert()
        .failure();

    let lock = run_lock(&dir);

    assert_eq!(lock.get("REQ-901").unwrap().result, TestResult::Failed);
    assert_eq!(
        lock.get("REQ-902").unwrap().result,
        TestResult::Passed,
        "untargeted REQ-902 must keep its previously recorded result"
    );
}

// @verifies REQ-042 v2 37857355
// @verifies REQ-043 v3 ceb81bfe
#[test]
fn no_weft_toml_exits_non_zero_reporting_no_test_command_configured() {
    let dir = TempDir::new().expect("create temp dir");
    project_with_two_requirements(&dir);
    // No weft.toml at all.

    Command::cargo_bin("weft")
        .unwrap()
        .arg("test")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("no test command configured"));

    assert!(
        !dir.path().join("docs/prds/weft.run.toml").exists(),
        "no Run Lock should be written when no Test Command is configured"
    );
}

// @verifies REQ-043 v3 ceb81bfe
#[test]
fn unknown_req_id_fails_with_not_found_message() {
    let dir = TempDir::new().expect("create temp dir");
    project_with_two_requirements(&dir);
    write_weft_toml(
        &dir,
        r#"
[test]
command = "true"
"#,
    );

    Command::cargo_bin("weft")
        .unwrap()
        .args(["test", "REQ-999"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("REQ-999"));
}
