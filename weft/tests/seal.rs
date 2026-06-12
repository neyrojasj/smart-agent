use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use weft_core::{canonical_hash, file_hash, parse_lock};

const STATEMENT: &str = "The system must allow users to log in with email and password.";
const ACCEPTANCE: &[&str] = &[
    "Given valid credentials, the user is authenticated.",
    "Given invalid credentials, an error is shown.",
];

fn current_hash() -> String {
    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    canonical_hash(STATEMENT, &acceptance)
}

/// Sets up a temp project with a fully-traced REQ-901: a requirement record,
/// a design doc, a code file, and a test file, all pinned to the current
/// Content Hash.
fn project_with_traced_requirement(dir: &TempDir) -> (String, String, String) {
    let hash = current_hash();
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let toml_src = format!(
        r#"
id = "REQ-901"
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
    fs::write(prds_dir.join("REQ-901.toml"), toml_src).expect("write REQ-901");

    let decisions_dir = dir.path().join("docs/decisions");
    fs::create_dir_all(&decisions_dir).expect("create docs/decisions");
    let design = format!("+++\naddresses = [\"REQ-901 v1 {hash}\"]\n+++\n\n# Login design\n");
    fs::write(decisions_dir.join("0001-login.md"), &design).expect("write design doc");

    let src_dir = dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("create src");
    let code = format!("// @implements REQ-901 v1 {hash}\nfn login() {{}}\n");
    fs::write(src_dir.join("login.rs"), &code).expect("write code");

    let tests_dir = dir.path().join("tests");
    fs::create_dir_all(&tests_dir).expect("create tests");
    let test = format!("// @verifies REQ-901 v1 {hash}\nfn test_login() {{}}\n");
    fs::write(tests_dir.join("login.rs"), &test).expect("write test");

    (design, code, test)
}

// @verifies REQ-031 v2 6cdbe6cb
// @verifies REQ-032 v2 a2441bcc
#[test]
fn seal_writes_a_file_hash_for_every_annotated_file() {
    let dir = TempDir::new().expect("create temp dir");
    let (design, code, test) = project_with_traced_requirement(&dir);

    Command::cargo_bin("weft")
        .unwrap()
        .arg("seal")
        .current_dir(dir.path())
        .assert()
        .success();

    let lock_src = fs::read_to_string(dir.path().join("docs/prds/weft.lock"))
        .expect("expected docs/prds/weft.lock to be written");
    let lock = parse_lock(&lock_src);

    assert_eq!(
        lock.get("docs/decisions/0001-login.md"),
        Some(&file_hash(design.as_bytes()))
    );
    assert_eq!(lock.get("src/login.rs"), Some(&file_hash(code.as_bytes())));
    assert_eq!(
        lock.get("tests/login.rs"),
        Some(&file_hash(test.as_bytes()))
    );
}

// @verifies REQ-033 v2 04d42b48
#[test]
fn check_reports_traced_after_seal_with_no_changes() {
    let dir = TempDir::new().expect("create temp dir");
    project_with_traced_requirement(&dir);

    Command::cargo_bin("weft")
        .unwrap()
        .arg("seal")
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("weft")
        .unwrap()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("REQ-901: Traced"));
}

// @verifies REQ-033 v2 04d42b48
#[test]
fn check_reports_drifted_when_an_annotated_file_changes_after_seal() {
    let dir = TempDir::new().expect("create temp dir");
    project_with_traced_requirement(&dir);

    Command::cargo_bin("weft")
        .unwrap()
        .arg("seal")
        .current_dir(dir.path())
        .assert()
        .success();

    let hash = current_hash();
    let edited = format!("// @implements REQ-901 v1 {hash}\nfn login() {{ /* edited */ }}\n");
    fs::write(dir.path().join("src/login.rs"), edited).expect("edit code");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("REQ-901: Drifted (src/login.rs)"));
}

// @verifies REQ-033 v2 04d42b48
#[test]
fn check_reports_drifted_when_weft_lock_is_missing() {
    let dir = TempDir::new().expect("create temp dir");
    project_with_traced_requirement(&dir);
    // No `weft seal` has run yet: weft.lock does not exist.

    Command::cargo_bin("weft")
        .unwrap()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("REQ-901: Drifted ("));
}

// @verifies REQ-032 v2 a2441bcc
#[test]
fn targeted_seal_restricts_to_files_annotated_with_that_req_id() {
    let dir = TempDir::new().expect("create temp dir");
    project_with_traced_requirement(&dir);

    // A second, unrelated requirement with its own annotated file.
    let other_hash = canonical_hash(
        "The system must allow users to log out.",
        &["Given a logged-in user, logging out ends the session.".to_string()],
    );
    let other_toml = format!(
        r#"
id = "REQ-902"
version = 1
hash = "{other_hash}"
status = "active"
statement = "The system must allow users to log out."
acceptance = [
    "Given a logged-in user, logging out ends the session.",
]
"#
    );
    fs::write(dir.path().join("docs/prds/REQ-902.toml"), other_toml).expect("write REQ-902");
    let other_code = format!("// @implements REQ-902 v1 {other_hash}\nfn logout() {{}}\n");
    fs::write(dir.path().join("src/logout.rs"), &other_code).expect("write logout code");

    Command::cargo_bin("weft")
        .unwrap()
        .args(["seal", "REQ-901"])
        .current_dir(dir.path())
        .assert()
        .success();

    let lock_src = fs::read_to_string(dir.path().join("docs/prds/weft.lock"))
        .expect("expected docs/prds/weft.lock to be written");
    let lock = parse_lock(&lock_src);

    assert!(lock.contains_key("src/login.rs"));
    assert!(
        !lock.contains_key("src/logout.rs"),
        "targeted seal must not record File Hashes for files annotated with another REQ_ID"
    );
}

// @verifies REQ-031 v2 6cdbe6cb
#[test]
fn full_seal_removes_entries_for_files_with_no_remaining_annotations() {
    let dir = TempDir::new().expect("create temp dir");
    project_with_traced_requirement(&dir);

    Command::cargo_bin("weft")
        .unwrap()
        .arg("seal")
        .current_dir(dir.path())
        .assert()
        .success();

    // Remove the annotation from the code file entirely.
    fs::write(dir.path().join("src/login.rs"), "fn login() {}\n").expect("rewrite code");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("seal")
        .current_dir(dir.path())
        .assert()
        .success();

    let lock_src = fs::read_to_string(dir.path().join("docs/prds/weft.lock"))
        .expect("expected docs/prds/weft.lock to exist");
    let lock = parse_lock(&lock_src);

    assert!(
        !lock.contains_key("src/login.rs"),
        "expected src/login.rs to be pruned from weft.lock once unannotated"
    );
}
