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

fn write_deprecated_requirement(dir: &TempDir, hash: &str) {
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let toml_src = format!(
        r#"
id = "REQ-901"
version = 1
hash = "{hash}"
status = "deprecated"
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

fn write_design_doc(dir: &TempDir, hash: &str) {
    let decisions_dir = dir.path().join("docs/decisions");
    fs::create_dir_all(&decisions_dir).expect("create docs/decisions");

    let doc = format!("+++\naddresses = [\"REQ-901 v1 {hash}\"]\n+++\n\n# Login design\n");
    fs::write(decisions_dir.join("0001-login.md"), doc).expect("write design doc");
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

fn current_hash() -> String {
    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    canonical_hash(STATEMENT, &acceptance)
}

// @verifies REQ-014 v2 d217a603
// @verifies REQ-033 v2 04d42b48
#[test]
fn fully_traced_requirement_passes_check() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);
    write_design_doc(&dir, &hash);
    write_code(&dir, &hash);
    write_test(&dir, &hash);

    // Seal first: until `weft seal` has run, annotated files have no entry
    // in weft.lock and are reported Drifted (ADR 0009 first-time setup).
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

#[test]
fn missing_verifies_link_is_incomplete() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);
    write_design_doc(&dir, &hash);
    write_code(&dir, &hash);
    // no test file: @verifies is missing

    Command::cargo_bin("weft")
        .unwrap()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("REQ-901: Incomplete"));
}

#[test]
fn old_hash_in_implements_link_is_stale() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);
    write_design_doc(&dir, &hash);
    write_code(&dir, "deadbeef"); // pins a hash that no longer matches the requirement
    write_test(&dir, &hash);

    Command::cargo_bin("weft")
        .unwrap()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("REQ-901: Stale"));
}

// @verifies REQ-014 v2 d217a603
#[test]
fn requirement_with_no_links_is_orphaned() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);
    // no design doc, no code, no test: no links at all

    Command::cargo_bin("weft")
        .unwrap()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("REQ-901: Orphaned"));
}

// @verifies REQ-034 v2 b68c2987
#[test]
fn weftignore_excludes_listed_directory_from_scan() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);
    write_design_doc(&dir, &hash);

    // Trace Links for code + test live only inside a directory listed in
    // `.weftignore`, so they must not be picked up by the scan.
    fs::write(dir.path().join(".weftignore"), "ignored\n").expect("write .weftignore");
    let ignored_dir = dir.path().join("ignored");
    fs::create_dir_all(&ignored_dir).expect("create ignored dir");
    let code = format!(
        "// @implements REQ-901 v1 {hash}\n// @verifies REQ-901 v1 {hash}\nfn login() {{}}\n"
    );
    fs::write(ignored_dir.join("login.rs"), code).expect("write code");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("REQ-901: Incomplete"));
}

// @verifies REQ-040 v2 1ead8691
#[test]
fn summary_flag_prints_trace_state_rollup_instead_of_per_requirement_listing() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);
    write_design_doc(&dir, &hash);
    write_code(&dir, &hash);
    write_test(&dir, &hash);

    Command::cargo_bin("weft")
        .unwrap()
        .arg("seal")
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["check", "--summary"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("1/1 Traced"))
        .stdout(predicate::str::contains("REQ-901:").not());
}

// @verifies REQ-040 v2 1ead8691
#[test]
fn summary_flag_still_fails_on_incomplete_requirements() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);
    write_design_doc(&dir, &hash);
    write_code(&dir, &hash);
    // no test file: @verifies is missing

    Command::cargo_bin("weft")
        .unwrap()
        .args(["check", "--summary"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("Incomplete: 1"))
        .stdout(predicate::str::contains("0/1 Traced"));
}

// @verifies REQ-041 v2 7194e93b
#[test]
fn dangling_annotation_to_unknown_requirement_is_reported_with_file_and_line() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_requirement(&dir, &hash);
    write_design_doc(&dir, &hash);
    write_code(&dir, &hash);
    write_test(&dir, &hash);

    // An annotation pointing at a requirement that does not exist anywhere
    // under docs/prds.
    let src_dir = dir.path().join("src");
    fs::write(
        src_dir.join("orphan.rs"),
        "fn other() {}\n// @implements REQ-999 v1 deadbeef\n",
    )
    .expect("write orphan code");

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
        .failure()
        .stdout(predicate::str::contains("src/orphan.rs:2"))
        .stdout(predicate::str::contains("REQ-999"));
}

// @verifies REQ-041 v2 7194e93b
#[test]
fn dangling_annotation_to_deprecated_requirement_is_reported() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_deprecated_requirement(&dir, &hash);
    write_code(&dir, &hash); // @implements REQ-901, but REQ-901 is deprecated

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
        .failure()
        .stdout(predicate::str::contains("src/login.rs:1"))
        .stdout(predicate::str::contains("REQ-901"));
}

#[test]
fn deprecated_requirement_with_no_links_does_not_fail_check() {
    let hash = current_hash();
    let dir = TempDir::new().expect("create temp dir");
    write_deprecated_requirement(&dir, &hash);
    // no design doc, no code, no test, and status = "deprecated":
    // excluded from the drift gate entirely.

    Command::cargo_bin("weft")
        .unwrap()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("REQ-901").not());
}
