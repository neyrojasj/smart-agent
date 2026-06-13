use std::collections::BTreeMap;

use weft_core::{
    canonical_hash, check_requirement, verify_check, Annotation, AnnotationKind, Requirement,
    RunRecord, Status, TestResult, TraceState,
};

fn requirement(id: &str, hash: &str) -> Requirement {
    let statement = "The system must allow users to log in.".to_string();
    let acceptance = vec!["Given valid credentials, the user is authenticated.".to_string()];
    Requirement {
        id: id.to_string(),
        version: 1,
        feat: None,
        hash: hash.to_string(),
        status: Status::Active,
        statement,
        acceptance,
        rationale: None,
        notes: None,
    }
}

fn current_hash() -> String {
    canonical_hash(
        "The system must allow users to log in.",
        &["Given valid credentials, the user is authenticated.".to_string()],
    )
}

fn traced_annotations(hash: &str) -> Vec<Annotation> {
    vec![
        Annotation {
            kind: AnnotationKind::Addresses,
            req_id: "REQ-901".to_string(),
            version: 1,
            hash: hash.to_string(),
        },
        Annotation {
            kind: AnnotationKind::Implements,
            req_id: "REQ-901".to_string(),
            version: 1,
            hash: hash.to_string(),
        },
        Annotation {
            kind: AnnotationKind::Verifies,
            req_id: "REQ-901".to_string(),
            version: 1,
            hash: hash.to_string(),
        },
    ]
}

fn file_hashes() -> BTreeMap<String, String> {
    let mut hashes = BTreeMap::new();
    hashes.insert("src/login.rs".to_string(), "current-file-hash".to_string());
    hashes
}

// @verifies REQ-044 v2 a74590fa
#[test]
fn traced_with_recorded_pass_at_current_hashes_becomes_verified() {
    let hash = current_hash();
    let req = requirement("REQ-901", &hash);
    let annotations = traced_annotations(&hash);
    let check = check_requirement(&req, &annotations, Vec::new());
    assert_eq!(check.state, TraceState::Traced);

    let run_record = RunRecord {
        result: TestResult::Passed,
        content_hash: hash,
        file_hashes: file_hashes(),
    };

    let verified = verify_check(check, &req, Some(&run_record), &file_hashes());

    assert_eq!(verified.state, TraceState::Verified);
    assert_eq!(verified.to_string(), "REQ-901: Verified");
}

// @verifies REQ-044 v2 a74590fa
#[test]
fn traced_with_no_recorded_run_stays_traced() {
    let hash = current_hash();
    let req = requirement("REQ-901", &hash);
    let annotations = traced_annotations(&hash);
    let check = check_requirement(&req, &annotations, Vec::new());

    let verified = verify_check(check, &req, None, &file_hashes());

    assert_eq!(verified.state, TraceState::Traced);
}

// @verifies REQ-044 v2 a74590fa
#[test]
fn verified_drops_when_content_hash_changed_since_recorded_pass() {
    let hash = current_hash();
    let req = requirement("REQ-901", &hash);
    let annotations = traced_annotations(&hash);
    let check = check_requirement(&req, &annotations, Vec::new());
    assert_eq!(check.state, TraceState::Traced);

    // The recorded pass was pinned to an older content hash — the
    // requirement was edited (and re-trace-linked) since that run.
    let run_record = RunRecord {
        result: TestResult::Passed,
        content_hash: "stale-content-hash".to_string(),
        file_hashes: file_hashes(),
    };

    let verified = verify_check(check, &req, Some(&run_record), &file_hashes());

    assert_eq!(verified.state, TraceState::Traced);
}

// @verifies REQ-044 v2 a74590fa
#[test]
fn verified_drops_when_annotated_file_hashes_no_longer_match_recorded_run() {
    let hash = current_hash();
    let req = requirement("REQ-901", &hash);
    let annotations = traced_annotations(&hash);
    let check = check_requirement(&req, &annotations, Vec::new());
    assert_eq!(check.state, TraceState::Traced);

    let run_record = RunRecord {
        result: TestResult::Passed,
        content_hash: hash,
        file_hashes: file_hashes(),
    };

    // The annotated file was edited after the recorded run, so its current
    // hash no longer matches the one pinned in the Run Lock.
    let mut current = file_hashes();
    current.insert("src/login.rs".to_string(), "edited-file-hash".to_string());

    let verified = verify_check(check, &req, Some(&run_record), &current);

    assert_eq!(verified.state, TraceState::Traced);
}

// @verifies REQ-044 v2 a74590fa
#[test]
fn drifted_requirement_with_recorded_pass_stays_drifted() {
    let hash = current_hash();
    let req = requirement("REQ-901", &hash);
    let annotations = traced_annotations(&hash);
    let check = check_requirement(&req, &annotations, vec!["src/login.rs".to_string()]);
    assert_eq!(
        check.state,
        TraceState::Drifted(vec!["src/login.rs".to_string()])
    );

    let run_record = RunRecord {
        result: TestResult::Passed,
        content_hash: hash,
        file_hashes: file_hashes(),
    };

    let verified = verify_check(check, &req, Some(&run_record), &file_hashes());

    assert_eq!(
        verified.state,
        TraceState::Drifted(vec!["src/login.rs".to_string()])
    );
}
