use assert_cmd::Command;
use tempfile::TempDir;

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
