// @verifies REQ-046 v3 fb90b6b7
use std::collections::BTreeMap;
use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use weft_core::{canonical_hash, render_run_lock, RunRecord, TestResult};

const STATEMENT: &str = "The system must allow users to log in.";
const ACCEPTANCE: &[&str] = &["Given valid credentials, the user is authenticated."];

fn hash() -> String {
    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    canonical_hash(STATEMENT, &acceptance)
}

fn write_requirement(dir: &TempDir, id: &str) {
    let h = hash();
    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).unwrap();
    let toml = format!(
        "id = \"{id}\"\nversion = 1\nhash = \"{h}\"\nstatus = \"active\"\nstatement = \"{STATEMENT}\"\nacceptance = [\n    \"{a0}\",\n]\n",
        a0 = ACCEPTANCE[0]
    );
    fs::write(prds_dir.join(format!("{id}.toml")), toml).unwrap();
}

fn write_design_doc(dir: &TempDir, id: &str) {
    let h = hash();
    let decisions_dir = dir.path().join("docs/decisions");
    fs::create_dir_all(&decisions_dir).unwrap();
    let doc = format!("+++\naddresses = [\"{id} v1 {h}\"]\n+++\n\n# Design\n");
    fs::write(decisions_dir.join(format!("{id}-design.md")), doc).unwrap();
}

fn write_code(dir: &TempDir, id: &str) {
    let h = hash();
    let src_dir = dir.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();
    let code = format!("// @implements {id} v1 {h}\nfn impl_fn() {{}}\n");
    fs::write(src_dir.join(format!("{id}.rs")), code).unwrap();
}

fn write_test_file(dir: &TempDir, id: &str) {
    let h = hash();
    let tests_dir = dir.path().join("tests_fixture");
    fs::create_dir_all(&tests_dir).unwrap();
    let test = format!("// @verifies {id} v1 {h}\nfn test_it() {{}}\n");
    fs::write(tests_dir.join(format!("{id}.rs")), test).unwrap();
}

fn seal(dir: &TempDir) {
    Command::cargo_bin("weft").unwrap().arg("seal").current_dir(dir.path()).assert().success();
}

fn write_weft_toml(dir: &TempDir) {
    fs::write(dir.path().join("weft.toml"), "[test]\ncommand = \"true\"\n").unwrap();
}

fn run_weft_test(dir: &TempDir) {
    Command::cargo_bin("weft").unwrap().arg("test").current_dir(dir.path()).assert().success();
}

/// Sets up a fully-traced and verified requirement.
fn setup_verified(dir: &TempDir, id: &str) {
    write_requirement(dir, id);
    write_design_doc(dir, id);
    write_code(dir, id);
    write_test_file(dir, id);
    seal(dir);
    write_weft_toml(dir);
    run_weft_test(dir);
}

/// Sets up a fully-traced (sealed) requirement, but without a test run.
fn setup_traced(dir: &TempDir, id: &str) {
    write_requirement(dir, id);
    write_design_doc(dir, id);
    write_code(dir, id);
    write_test_file(dir, id);
    seal(dir);
}

/// Writes a run lock with a failed result for `id` at the current hash/file-hashes,
/// so `weft check` reports this requirement as Traced-with-failing-tests.
fn write_failing_run_lock(dir: &TempDir, id: &str) {
    let h = hash();
    let src_path = format!("src/{id}.rs");
    let src_bytes = fs::read(dir.path().join(&src_path)).unwrap();
    let src_file_hash = weft_core::file_hash(&src_bytes);

    let tests_path = format!("tests_fixture/{id}.rs");
    let tests_bytes = fs::read(dir.path().join(&tests_path)).unwrap();
    let tests_file_hash = weft_core::file_hash(&tests_bytes);

    let design_path = format!("docs/decisions/{id}-design.md");
    let design_bytes = fs::read(dir.path().join(&design_path)).unwrap();
    let design_file_hash = weft_core::file_hash(&design_bytes);

    let mut file_hashes = BTreeMap::new();
    file_hashes.insert(src_path, src_file_hash);
    file_hashes.insert(tests_path, tests_file_hash);
    file_hashes.insert(design_path, design_file_hash);

    let mut lock: BTreeMap<String, RunRecord> = BTreeMap::new();
    lock.insert(
        id.to_string(),
        RunRecord { result: TestResult::Failed, content_hash: h, file_hashes },
    );

    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).unwrap();
    fs::write(prds_dir.join("weft.run.toml"), render_run_lock(&lock)).unwrap();
}

// @verifies REQ-046 v3 fb90b6b7
#[test]
fn all_verified_exits_zero_with_no_next_work_message() {
    let dir = TempDir::new().unwrap();
    setup_verified(&dir, "REQ-901");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("next")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("no next work item"));
}

// @verifies REQ-046 v3 fb90b6b7
#[test]
fn orphaned_requirement_yields_implement_action() {
    let dir = TempDir::new().unwrap();
    write_requirement(&dir, "REQ-901");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("next")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("REQ-901"))
        .stdout(predicate::str::contains("implement"));
}

// @verifies REQ-046 v3 fb90b6b7
#[test]
fn incomplete_requirement_yields_implement_action_with_missing_annotations() {
    let dir = TempDir::new().unwrap();
    write_requirement(&dir, "REQ-901");
    write_design_doc(&dir, "REQ-901");
    // Missing: implements, verifies

    let h = hash();
    Command::cargo_bin("weft")
        .unwrap()
        .arg("next")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("implement"))
        .stdout(predicate::str::contains(format!("@implements REQ-901 v1 {h}")))
        .stdout(predicate::str::contains(format!("@verifies REQ-901 v1 {h}")));
}

/// Writes a requirement whose stored hash deliberately differs from the
/// annotation hash in the fixture files. `annotation_hash` is the hash
/// the code/test/design files were written with; `stored_hash` is the
/// NEW hash stored in the requirement record — so every annotation is Stale.
fn write_requirement_with_new_hash(dir: &TempDir, id: &str, annotation_hash: &str) {
    // Write a second TOML with a different statement so canonical_hash
    // produces a different value, making the links Stale.
    const STATEMENT_V2: &str = "The system must allow users to log in securely.";
    const ACCEPTANCE_V2: &[&str] = &["Given valid credentials, access is granted."];

    let new_hash = {
        let acceptance: Vec<String> = ACCEPTANCE_V2.iter().map(|s| s.to_string()).collect();
        canonical_hash(STATEMENT_V2, &acceptance)
    };
    assert_ne!(&new_hash, annotation_hash, "new_hash must differ from annotation_hash");

    let prds_dir = dir.path().join("docs/prds");
    fs::create_dir_all(&prds_dir).unwrap();
    let toml = format!(
        "id = \"{id}\"\nversion = 2\nhash = \"{new_hash}\"\nstatus = \"active\"\nstatement = \"{STATEMENT_V2}\"\nacceptance = [\n    \"{a0}\",\n]\n",
        a0 = ACCEPTANCE_V2[0]
    );
    fs::write(prds_dir.join(format!("{id}.toml")), toml).unwrap();
}

// @verifies REQ-046 v3 fb90b6b7
#[test]
fn stale_requirement_yields_rework_action_with_stale_annotations() {
    let dir = TempDir::new().unwrap();
    let h = hash();
    // Write trace links with hash H1, then overwrite the requirement with
    // a different statement so the stored hash becomes H2 ≠ H1 → Stale.
    write_requirement(&dir, "REQ-901");
    write_design_doc(&dir, "REQ-901");
    write_code(&dir, "REQ-901");
    write_test_file(&dir, "REQ-901");
    seal(&dir);

    write_requirement_with_new_hash(&dir, "REQ-901", &h);

    Command::cargo_bin("weft")
        .unwrap()
        .arg("next")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("rework"))
        .stdout(predicate::str::contains("@implements"))
        .stdout(predicate::str::contains("@verifies"));
}

// @verifies REQ-046 v3 fb90b6b7
#[test]
fn drifted_requirement_yields_reseal_action_without_annotations() {
    let dir = TempDir::new().unwrap();
    setup_traced(&dir, "REQ-901");
    // Modify a source file to cause drift
    let src_path = dir.path().join("src/REQ-901.rs");
    let mut content = fs::read_to_string(&src_path).unwrap();
    content.push_str("// drift\n");
    fs::write(&src_path, content).unwrap();

    let h = hash();
    Command::cargo_bin("weft")
        .unwrap()
        .arg("next")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("reseal"))
        // annotations must NOT be present for reseal
        .stdout(predicate::str::contains(format!("@implements REQ-901 v1 {h}")).not())
        .stdout(predicate::str::contains(format!("@verifies REQ-901 v1 {h}")).not());
}

// @verifies REQ-046 v3 fb90b6b7
#[test]
fn traced_with_failing_tests_yields_fix_tests_action_without_annotations() {
    let dir = TempDir::new().unwrap();
    setup_traced(&dir, "REQ-901");
    write_failing_run_lock(&dir, "REQ-901");

    let h = hash();
    Command::cargo_bin("weft")
        .unwrap()
        .arg("next")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("fix-tests"))
        .stdout(predicate::str::contains(format!("@implements REQ-901 v1 {h}")).not());
}

// @verifies REQ-046 v3 fb90b6b7
#[test]
fn traced_without_run_yields_run_tests_action_without_annotations() {
    let dir = TempDir::new().unwrap();
    setup_traced(&dir, "REQ-901");
    // No run lock written → "no recorded test run"

    let h = hash();
    Command::cargo_bin("weft")
        .unwrap()
        .arg("next")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("run-tests"))
        .stdout(predicate::str::contains(format!("@implements REQ-901 v1 {h}")).not());
}

// @verifies REQ-046 v3 fb90b6b7
#[test]
fn failing_tests_selected_over_drifted_by_priority() {
    // REQ-901 is Drifted, REQ-902 is Traced-with-failing-tests.
    // `next` must select REQ-902 (priority 1) over REQ-901 (priority 2).
    let dir = TempDir::new().unwrap();

    // REQ-901: Drifted
    setup_traced(&dir, "REQ-901");
    let src_path = dir.path().join("src/REQ-901.rs");
    let mut content = fs::read_to_string(&src_path).unwrap();
    content.push_str("// drift\n");
    fs::write(&src_path, content).unwrap();

    // REQ-902: Traced with failing tests
    setup_traced(&dir, "REQ-902");
    write_failing_run_lock(&dir, "REQ-902");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("next")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("REQ-902"))
        .stdout(predicate::str::contains("fix-tests"));
}

// @verifies REQ-046 v3 fb90b6b7
#[test]
fn implement_for_orphaned_includes_all_three_annotation_strings() {
    let dir = TempDir::new().unwrap();
    write_requirement(&dir, "REQ-901");
    // No trace links at all (Orphaned)

    let h = hash();
    Command::cargo_bin("weft")
        .unwrap()
        .arg("next")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains(format!("REQ-901 v1 {h}"))) // addresses annotation
        .stdout(predicate::str::contains(format!("@implements REQ-901 v1 {h}")))
        .stdout(predicate::str::contains(format!("@verifies REQ-901 v1 {h}")));
}

// @verifies REQ-046 v3 fb90b6b7
#[test]
fn json_flag_emits_structured_object() {
    let dir = TempDir::new().unwrap();
    write_requirement(&dir, "REQ-901");

    let output = Command::cargo_bin("weft")
        .unwrap()
        .args(["next", "--json"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).unwrap();
    let value: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
    assert_eq!(value["id"], "REQ-901");
    assert_eq!(value["action"], "implement");
    assert_eq!(value["state"], "Orphaned");
}

// @verifies REQ-046 v3 fb90b6b7
#[test]
fn json_flag_no_next_work_emits_no_work_object_and_exits_zero() {
    let dir = TempDir::new().unwrap();
    setup_verified(&dir, "REQ-901");

    let output = Command::cargo_bin("weft")
        .unwrap()
        .args(["next", "--json"])
        .current_dir(dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).unwrap();
    let value: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
    assert_eq!(value["status"], "no_next_work_item");
}
