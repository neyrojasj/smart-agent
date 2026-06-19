use std::fs;

use assert_cmd::Command;
use tempfile::TempDir;

// @verifies REQ-013 v4 c6954fdc
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

// @verifies REQ-035 v2 1e646999
#[test]
fn init_creates_default_weftignore() {
    let dir = TempDir::new().expect("create temp dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let weftignore = dir.path().join(".weftignore");
    let src = fs::read_to_string(&weftignore).expect("read .weftignore");
    assert!(
        src.contains(".scratch"),
        "expected .scratch entry, got:\n{src}"
    );
    assert!(src.contains("logs"), "expected logs entry, got:\n{src}");
}

// @verifies REQ-035 v2 1e646999
#[test]
fn init_does_not_overwrite_existing_weftignore() {
    let dir = TempDir::new().expect("create temp dir");
    let weftignore = dir.path().join(".weftignore");
    fs::write(&weftignore, "custom\n").expect("write fixture .weftignore");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let src = fs::read_to_string(&weftignore).unwrap();
    assert_eq!(
        src, "custom\n",
        "expected init to leave existing .weftignore untouched, got:\n{src}"
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

// @verifies REQ-013 v4 c6954fdc
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
