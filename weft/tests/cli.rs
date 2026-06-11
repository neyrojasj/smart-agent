use std::fs;
use std::thread;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use weft_core::canonical_hash;

const STATEMENT: &str = "The system must allow users to log in with email and password.";
const ACCEPTANCE: &[&str] = &[
    "Given valid credentials, the user is authenticated.",
    "Given invalid credentials, an error is shown.",
];

/// Sets up a temp project with a single well-formed requirement record at
/// `docs/prds/REQ-001.toml`, with `hash` matching the current canonical hash.
fn project_with_well_formed_record() -> TempDir {
    let dir = TempDir::new().expect("create temp dir");
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    let hash = canonical_hash(STATEMENT, &acceptance);

    let toml_src = format!(
        r#"
id = "REQ-001"
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

#[test]
fn get_returns_statement_for_a_fixture_record() {
    let dir = project_with_well_formed_record();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["get", "REQ-001", "--field", "statement"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(STATEMENT));
}

#[test]
fn get_returns_version_for_a_fixture_record() {
    let dir = project_with_well_formed_record();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["get", "REQ-001", "--field", "version"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("1"));
}

#[test]
fn get_returns_hash_for_a_fixture_record() {
    let dir = project_with_well_formed_record();
    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    let hash = canonical_hash(STATEMENT, &acceptance);

    Command::cargo_bin("weft")
        .unwrap()
        .args(["get", "REQ-001", "--field", "hash"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(&hash));
}

#[test]
fn get_returns_acceptance_for_a_fixture_record() {
    let dir = project_with_well_formed_record();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["get", "REQ-001", "--field", "acceptance"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(
            predicate::str::contains(ACCEPTANCE[0]).and(predicate::str::contains(ACCEPTANCE[1])),
        );
}

// @verifies REQ-008 v2 3da61ff3
#[test]
fn get_fails_for_nonexistent_requirement() {
    let dir = project_with_well_formed_record();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["get", "REQ-999", "--field", "statement"])
        .current_dir(dir.path())
        .assert()
        .failure();
}

#[test]
fn verify_passes_for_a_well_formed_record() {
    let dir = project_with_well_formed_record();

    Command::cargo_bin("weft")
        .unwrap()
        .arg("verify")
        .current_dir(dir.path())
        .assert()
        .success();
}

// @verifies REQ-007 v2 1d00916a
#[test]
fn verify_fails_with_format_error_for_malformed_toml() {
    let dir = TempDir::new().expect("create temp dir");
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");
    fs::write(
        prds_dir.join("REQ-001.toml"),
        "id = \"REQ-001\nthis is not valid toml",
    )
    .expect("write malformed fixture");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("verify")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("REQ-001"));
}

/// Sets up a temp project whose only requirement record is `REQ-007.toml`
/// (well-formed, hash matches), to exercise `weft new`'s id allocation.
fn project_with_max_req_007() -> TempDir {
    let dir = TempDir::new().expect("create temp dir");
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    let hash = canonical_hash(STATEMENT, &acceptance);

    let toml_src = format!(
        r#"
id = "REQ-007"
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

    fs::write(prds_dir.join("REQ-007.toml"), toml_src).expect("write fixture");
    dir
}

#[test]
fn new_allocates_next_id_and_writes_a_valid_skeleton() {
    let dir = project_with_max_req_007();

    Command::cargo_bin("weft")
        .unwrap()
        .arg("new")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("REQ-008"));

    let skeleton_path = dir.path().join("docs/prds/REQ-008.toml");
    assert!(
        skeleton_path.exists(),
        "expected docs/prds/REQ-008.toml to be written"
    );

    Command::cargo_bin("weft")
        .unwrap()
        .arg("verify")
        .current_dir(dir.path())
        .assert()
        .success();
}

#[test]
fn new_allocates_req_001_in_an_empty_project() {
    let dir = TempDir::new().expect("create temp dir");
    fs::create_dir_all(dir.path().join("docs/prds")).expect("create docs/prds");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("new")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("REQ-001"));

    assert!(dir.path().join("docs/prds/REQ-001.toml").exists());
}

#[test]
fn new_with_feat_places_record_in_feature_folder_and_sets_feat_field() {
    let dir = project_with_max_req_007();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["new", "--feat", "FEAT-Auth"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("REQ-008"));

    let skeleton_path = dir.path().join("docs/prds/FEAT-Auth/REQ-008.toml");
    let src = fs::read_to_string(&skeleton_path)
        .expect("expected docs/prds/FEAT-Auth/REQ-008.toml to be written");
    assert!(src.contains("feat = \"FEAT-Auth\""));

    Command::cargo_bin("weft")
        .unwrap()
        .arg("verify")
        .current_dir(dir.path())
        .assert()
        .success();
}

// @verifies REQ-009 v2 c30912ae
#[test]
fn new_allocates_unique_ids_under_concurrent_invocation() {
    let dir = project_with_max_req_007();
    let path = dir.path().to_path_buf();

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let path = path.clone();
            thread::spawn(move || {
                Command::cargo_bin("weft")
                    .unwrap()
                    .arg("new")
                    .current_dir(&path)
                    .assert()
                    .success();
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("weft new thread panicked");
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(dir.path().join("docs/prds")).expect("read docs/prds") {
        let entry = entry.expect("dir entry");
        if entry.path().extension().and_then(|e| e.to_str()) == Some("toml") {
            files.push(entry.file_name().to_string_lossy().into_owned());
        }
    }

    let unique: std::collections::HashSet<_> = files.iter().cloned().collect();
    assert_eq!(
        files.len(),
        unique.len(),
        "expected unique ids, got: {files:?}"
    );
    // REQ-007 (pre-existing) plus 4 newly allocated records.
    assert_eq!(files.len(), 5, "expected 5 records, got: {files:?}");
}

/// Sets up a temp project with two requirement records: `REQ-001` (no
/// `feat`) and `REQ-002` (`feat = "FEAT-Auth"`).
fn project_with_two_requirements() -> TempDir {
    let dir = TempDir::new().expect("create temp dir");
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).expect("create docs/prds");

    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    let hash = canonical_hash(STATEMENT, &acceptance);

    let req_001 = format!(
        r#"
id = "REQ-001"
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
    fs::write(prds_dir.join("REQ-001.toml"), req_001).expect("write REQ-001");

    let other_statement = "The system must allow users to log out.";
    let other_acceptance =
        vec!["Given a logged-in user, logging out ends the session.".to_string()];
    let other_hash = canonical_hash(other_statement, &other_acceptance);

    let auth_dir = prds_dir.join("FEAT-Auth");
    fs::create_dir_all(&auth_dir).expect("create docs/prds/FEAT-Auth");
    let req_002 = format!(
        r#"
id = "REQ-002"
version = 1
feat = "FEAT-Auth"
hash = "{other_hash}"
status = "active"
statement = "{other_statement}"
acceptance = [
    "{a0}",
]
"#,
        a0 = other_acceptance[0],
    );
    fs::write(auth_dir.join("REQ-002.toml"), req_002).expect("write REQ-002");

    dir
}

#[test]
fn list_prints_id_and_description_for_every_requirement() {
    let dir = project_with_two_requirements();

    let output = Command::cargo_bin("weft")
        .unwrap()
        .arg("list")
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).unwrap();

    assert!(stdout.contains("REQ-001"), "expected REQ-001 in: {stdout}");
    assert!(
        stdout.contains(STATEMENT),
        "expected statement in: {stdout}"
    );
    assert!(stdout.contains("REQ-002"), "expected REQ-002 in: {stdout}");
    assert!(
        stdout.contains("The system must allow users to log out."),
        "expected logout description in: {stdout}"
    );
}

#[test]
fn list_filters_by_feat() {
    let dir = project_with_two_requirements();

    let output = Command::cargo_bin("weft")
        .unwrap()
        .args(["list", "--feat", "FEAT-Auth"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).unwrap();

    assert!(stdout.contains("REQ-002"), "expected REQ-002 in: {stdout}");
    assert!(
        !stdout.contains("REQ-001"),
        "did not expect REQ-001 in: {stdout}"
    );
}

// @verifies REQ-010 v2 59117494
#[test]
fn list_excludes_deprecated_requirements() {
    let dir = project_with_two_requirements();
    let prd_path = dir.path().join("docs/prds/REQ-001.toml");
    let deprecated = fs::read_to_string(&prd_path)
        .unwrap()
        .replace("status = \"active\"", "status = \"deprecated\"");
    fs::write(&prd_path, deprecated).unwrap();

    let output = Command::cargo_bin("weft")
        .unwrap()
        .arg("list")
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).unwrap();

    assert!(
        !stdout.contains("REQ-001"),
        "did not expect deprecated REQ-001 in: {stdout}"
    );
    assert!(stdout.contains("REQ-002"), "expected REQ-002 in: {stdout}");
}

#[test]
fn verify_fails_with_bump_message_for_a_stale_hash() {
    let dir = project_with_well_formed_record();
    let prd_path = dir.path().join("docs/prds/REQ-001.toml");
    let stale = fs::read_to_string(&prd_path)
        .unwrap()
        .replacen(&canonical_hash(STATEMENT, &ACCEPTANCE.iter().map(|s| s.to_string()).collect::<Vec<_>>()), "deadbeef", 1);
    fs::write(&prd_path, stale).unwrap();

    Command::cargo_bin("weft")
        .unwrap()
        .arg("verify")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("weft bump REQ-001"));
}

// @verifies REQ-011 v2 5429311c
#[test]
fn bump_recomputes_hash_and_increments_version_after_an_edit() {
    let dir = project_with_well_formed_record();
    let prd_path = dir.path().join("docs/prds/REQ-001.toml");
    let original_hash = canonical_hash(
        STATEMENT,
        &ACCEPTANCE.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
    );

    // Silently edit the statement without bumping: the stored hash is now stale.
    let edited = fs::read_to_string(&prd_path)
        .unwrap()
        .replace(STATEMENT, "The system must allow users to log in with email, password, and a one-time code.");
    fs::write(&prd_path, edited).unwrap();

    Command::cargo_bin("weft")
        .unwrap()
        .arg("verify")
        .current_dir(dir.path())
        .assert()
        .failure();

    Command::cargo_bin("weft")
        .unwrap()
        .args(["bump", "REQ-001"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("REQ-001"))
        .stdout(predicate::str::contains("v2"));

    Command::cargo_bin("weft")
        .unwrap()
        .arg("verify")
        .current_dir(dir.path())
        .assert()
        .success();

    let bumped = fs::read_to_string(&prd_path).unwrap();
    assert!(bumped.contains("version = 2"), "expected version = 2 in: {bumped}");
    assert!(
        !bumped.contains(&original_hash),
        "expected the original hash to be replaced in: {bumped}"
    );
}
