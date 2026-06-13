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

fn write_requirement(dir: &TempDir, hash: &str) {
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
}

fn write_code(dir: &TempDir, hash: &str) {
    let src_dir = dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("create src");

    let code = format!("// @implements REQ-901 v1 {hash}\nfn login() {{}}\n");
    fs::write(src_dir.join("login.rs"), code).expect("write code");
}

fn write_test(dir: &TempDir, hash: &str) {
    let tests_dir = dir.path().join("tests");
    fs::create_dir_all(&tests_dir).expect("create tests");

    let test = format!("// @verifies REQ-901 v1 {hash}\nfn test_login() {{}}\n");
    fs::write(tests_dir.join("login.rs"), test).expect("write test");
}

fn write_design_doc(dir: &TempDir, hash: &str) {
    let decisions_dir = dir.path().join("docs/decisions");
    fs::create_dir_all(&decisions_dir).expect("create docs/decisions");

    let doc = format!("+++\naddresses = [\"REQ-901 v1 {hash}\"]\n+++\n\n# Login design\n");
    fs::write(decisions_dir.join("0001-login.md"), doc).expect("write design doc");
}

// @verifies REQ-038 v2 82357796
#[test]
fn trace_reports_implements_link_with_file_and_line() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);
    write_code(&dir, &hash);

    Command::cargo_bin("weft")
        .unwrap()
        .args(["trace", "REQ-901"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("@implements"))
        .stdout(predicate::str::contains("src/login.rs:1"));
}

// @verifies REQ-038 v2 82357796
#[test]
fn trace_reports_orphaned_for_requirement_with_no_links() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);
    // no design doc, no code, no test: no links at all

    Command::cargo_bin("weft")
        .unwrap()
        .args(["trace", "REQ-901"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Orphaned"));
}

// @verifies REQ-038 v2 82357796
#[test]
fn trace_fails_for_nonexistent_requirement() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);

    Command::cargo_bin("weft")
        .unwrap()
        .args(["trace", "REQ-999"])
        .current_dir(dir.path())
        .assert()
        .failure();
}

// @verifies REQ-038 v2 82357796
#[test]
fn trace_reports_each_link_kind_on_its_own_line() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);
    write_design_doc(&dir, &hash);
    write_code(&dir, &hash);
    write_test(&dir, &hash);

    Command::cargo_bin("weft")
        .unwrap()
        .args(["trace", "REQ-901"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("@addresses"))
        .stdout(predicate::str::contains("docs/decisions/0001-login.md:2"))
        .stdout(predicate::str::contains("@implements"))
        .stdout(predicate::str::contains("src/login.rs:1"))
        .stdout(predicate::str::contains("@verifies"))
        .stdout(predicate::str::contains("tests/login.rs:1"));
}
