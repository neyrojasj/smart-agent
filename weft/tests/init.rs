use std::fs;

use assert_cmd::Command;
use tempfile::TempDir;

// @verifies REQ-013 v2 41174961
#[test]
fn init_creates_docs_prds_directory() {
    let dir = TempDir::new().expect("create temp dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        dir.path().join("docs/prds").is_dir(),
        "expected docs/prds/ to be created"
    );
}

#[test]
fn init_creates_docs_decisions_directory() {
    let dir = TempDir::new().expect("create temp dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        dir.path().join("docs/decisions").is_dir(),
        "expected docs/decisions/ to be created"
    );
}

#[test]
fn init_is_idempotent_on_existing_project() {
    let dir = TempDir::new().expect("create temp dir");

    // Run init twice — should not fail the second time
    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();
}

// @verifies REQ-013 v2 41174961
#[test]
fn init_second_run_does_not_overwrite_existing_prd_record() {
    let dir = TempDir::new().expect("create temp dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let req_path = dir.path().join("docs/prds/REQ-001.toml");
    fs::write(&req_path, "id = \"REQ-001\"\n").expect("write fixture record");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let src = fs::read_to_string(&req_path).unwrap();
    assert_eq!(
        src, "id = \"REQ-001\"\n",
        "expected init to leave existing records untouched, got:\n{src}"
    );
}
