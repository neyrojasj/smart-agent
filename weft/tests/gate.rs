// @verifies REQ-045 v2 12e173a6
use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use weft_core::canonical_hash;

const STATEMENT_901: &str = "The system must allow users to log in with email and password.";
const ACCEPTANCE_901: &[&str] = &[
    "Given valid credentials, the user is authenticated.",
    "Given invalid credentials, an error is shown.",
];

fn hash_901() -> String {
    let acceptance: Vec<String> = ACCEPTANCE_901.iter().map(|s| s.to_string()).collect();
    canonical_hash(STATEMENT_901, &acceptance)
}

fn write_requirement(dir: &TempDir, id: &str, status: &str, statement: &str, acceptance: &[&str], hash: &str) {
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let acceptance_lines: String = acceptance
        .iter()
        .map(|a| format!("    \"{a}\",\n"))
        .collect();

    let toml_src = format!(
        r#"id = "{id}"
version = 1
hash = "{hash}"
status = "{status}"
statement = "{statement}"
acceptance = [
{acceptance_lines}]
"#
    );
    fs::write(prds_dir.join(format!("{id}.toml")), toml_src).expect("write requirement");
}

fn write_design_doc(dir: &TempDir, req_id: &str, hash: &str) {
    let decisions_dir = dir.path().join("docs/decisions");
    fs::create_dir_all(&decisions_dir).expect("create docs/decisions");

    let doc = format!("+++\naddresses = [\"{req_id} v1 {hash}\"]\n+++\n\n# Design\n");
    fs::write(
        decisions_dir.join(format!("{req_id}-design.md")),
        doc,
    )
    .expect("write design doc");
}

fn write_code(dir: &TempDir, req_id: &str, hash: &str) {
    let src_dir = dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("create src");

    let code = format!("// @implements {req_id} v1 {hash}\nfn implementation() {{}}\n");
    fs::write(src_dir.join(format!("{req_id}.rs")), code).expect("write code");
}

fn write_test_file(dir: &TempDir, req_id: &str, hash: &str) {
    let tests_dir = dir.path().join("tests_fixture");
    fs::create_dir_all(&tests_dir).expect("create tests_fixture");

    let test = format!("// @verifies {req_id} v1 {hash}\nfn test_it() {{}}\n");
    fs::write(tests_dir.join(format!("{req_id}.rs")), test).expect("write test");
}

fn seal(dir: &TempDir) {
    Command::cargo_bin("weft")
        .unwrap()
        .arg("seal")
        .current_dir(dir.path())
        .assert()
        .success();
}

fn write_weft_toml(dir: &TempDir) {
    fs::write(
        dir.path().join("weft.toml"),
        "[test]\ncommand = \"true\"\n",
    )
    .expect("write weft.toml");
}

fn run_weft_test(dir: &TempDir) {
    Command::cargo_bin("weft")
        .unwrap()
        .arg("test")
        .current_dir(dir.path())
        .assert()
        .success();
}

fn setup_verified_req(dir: &TempDir, req_id: &str) {
    let hash = hash_901();
    write_requirement(dir, req_id, "active", STATEMENT_901, ACCEPTANCE_901, &hash);
    write_design_doc(dir, req_id, &hash);
    write_code(dir, req_id, &hash);
    write_test_file(dir, req_id, &hash);
    seal(dir);
    write_weft_toml(dir);
    run_weft_test(dir);
}

// @verifies REQ-045 v2 12e173a6
#[test]
fn all_verified_exits_zero() {
    let dir = TempDir::new().expect("create temp dir");
    setup_verified_req(&dir, "REQ-901");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("gate")
        .current_dir(dir.path())
        .assert()
        .success();
}

// @verifies REQ-045 v2 12e173a6
#[test]
fn non_verified_requirement_exits_non_zero_with_req_id_and_state_listed() {
    let hash = hash_901();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, "REQ-901", "active", STATEMENT_901, ACCEPTANCE_901, &hash);
    write_design_doc(&dir, "REQ-901", &hash);
    write_code(&dir, "REQ-901", &hash);
    write_test_file(&dir, "REQ-901", &hash);
    seal(&dir);
    // No weft test run: requirement is Traced, not Verified

    Command::cargo_bin("weft")
        .unwrap()
        .arg("gate")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("REQ-901"))
        .stdout(predicate::str::contains("Traced"));
}

// @verifies REQ-045 v2 12e173a6
#[test]
fn deprecated_requirement_is_excluded_gate_passes_when_active_are_verified() {
    let dir = TempDir::new().expect("create temp dir");
    // Active requirement: set up as Verified
    setup_verified_req(&dir, "REQ-901");

    // Deprecated requirement with no trace links — must be excluded from gate
    let prds_dir = dir.path().join("docs/prds");
    fs::write(
        prds_dir.join("REQ-902.toml"),
        r#"id = "REQ-902"
version = 1
hash = "deadbeef"
status = "deprecated"
statement = "The system must do something no longer needed."
acceptance = [
    "It used to work.",
]
"#,
    )
    .expect("write deprecated REQ-902");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("gate")
        .current_dir(dir.path())
        .assert()
        .success();
}
