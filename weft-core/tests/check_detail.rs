use weft_core::{
    canonical_hash, check_requirement, Annotation, AnnotationKind, Requirement, StaleLink, Status,
    TraceState,
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

fn link(kind: AnnotationKind, req_id: &str, hash: &str) -> Annotation {
    Annotation {
        kind,
        req_id: req_id.to_string(),
        version: 1,
        hash: hash.to_string(),
    }
}

// @verifies REQ-036 v2 4cbbd466
#[test]
fn orphaned_requirement_has_no_gap_detail() {
    let req = requirement("REQ-001", &current_hash());

    let check = check_requirement(&req, &[], Vec::new());

    assert_eq!(check.state, TraceState::Orphaned);
    assert!(check.gap.missing_links.is_empty());
    assert!(check.gap.stale_links.is_empty());
    assert!(check.gap.drifted_files.is_empty());
    assert_eq!(check.to_string(), "REQ-001: Orphaned");
}

// @verifies REQ-036 v2 4cbbd466
#[test]
fn incomplete_requirement_reports_missing_link_kinds() {
    let hash = current_hash();
    let req = requirement("REQ-001", &hash);
    let annotations = vec![
        link(AnnotationKind::Addresses, "REQ-001", &hash),
        link(AnnotationKind::Implements, "REQ-001", &hash),
        // no @verifies link
    ];

    let check = check_requirement(&req, &annotations, Vec::new());

    assert_eq!(check.state, TraceState::Incomplete);
    assert_eq!(check.gap.missing_links, vec!["verifies".to_string()]);
    assert_eq!(check.to_string(), "REQ-001: Incomplete (missing verifies)");
}

// @verifies REQ-036 v2 4cbbd466
#[test]
fn incomplete_requirement_reports_multiple_missing_link_kinds_in_order() {
    let hash = current_hash();
    let req = requirement("REQ-001", &hash);
    let annotations = vec![link(AnnotationKind::Addresses, "REQ-001", &hash)];

    let check = check_requirement(&req, &annotations, Vec::new());

    assert_eq!(check.state, TraceState::Incomplete);
    assert_eq!(
        check.gap.missing_links,
        vec!["implements".to_string(), "verifies".to_string()]
    );
    assert_eq!(
        check.to_string(),
        "REQ-001: Incomplete (missing implements, verifies)"
    );
}

// @verifies REQ-036 v2 4cbbd466
#[test]
fn stale_requirement_reports_recorded_and_current_hash() {
    let hash = current_hash();
    let req = requirement("REQ-001", &hash);
    let annotations = vec![
        link(AnnotationKind::Addresses, "REQ-001", &hash),
        link(AnnotationKind::Implements, "REQ-001", "deadbeef"),
        link(AnnotationKind::Verifies, "REQ-001", &hash),
    ];

    let check = check_requirement(&req, &annotations, Vec::new());

    assert_eq!(check.state, TraceState::Stale);
    assert_eq!(
        check.gap.stale_links,
        vec![StaleLink {
            kind: "implements".to_string(),
            recorded_hash: "deadbeef".to_string(),
            current_hash: hash.clone(),
        }]
    );
    assert_eq!(
        check.to_string(),
        format!("REQ-001: Stale (implements has deadbeef, current {hash})")
    );
}

// @verifies REQ-036 v2 4cbbd466
#[test]
fn traced_requirement_has_no_gap_detail() {
    let hash = current_hash();
    let req = requirement("REQ-001", &hash);
    let annotations = vec![
        link(AnnotationKind::Addresses, "REQ-001", &hash),
        link(AnnotationKind::Implements, "REQ-001", &hash),
        link(AnnotationKind::Verifies, "REQ-001", &hash),
    ];

    let check = check_requirement(&req, &annotations, Vec::new());

    assert_eq!(check.state, TraceState::Traced);
    assert!(check.gap.missing_links.is_empty());
    assert!(check.gap.stale_links.is_empty());
    assert!(check.gap.drifted_files.is_empty());
    assert_eq!(check.to_string(), "REQ-001: Traced");
}

// @verifies REQ-036 v2 4cbbd466
#[test]
fn drifted_requirement_reports_drifted_files() {
    let hash = current_hash();
    let req = requirement("REQ-001", &hash);
    let annotations = vec![
        link(AnnotationKind::Addresses, "REQ-001", &hash),
        link(AnnotationKind::Implements, "REQ-001", &hash),
        link(AnnotationKind::Verifies, "REQ-001", &hash),
    ];

    let check = check_requirement(&req, &annotations, vec!["src/login.rs".to_string()]);

    assert_eq!(
        check.state,
        TraceState::Drifted(vec!["src/login.rs".to_string()])
    );
    assert_eq!(check.gap.drifted_files, vec!["src/login.rs".to_string()]);
    assert_eq!(check.to_string(), "REQ-001: Drifted (src/login.rs)");
}

// @verifies REQ-037 v2 2371e246
#[test]
fn requirement_check_serializes_to_stable_json_shape() {
    let hash = current_hash();
    let req = requirement("REQ-001", &hash);
    let annotations = vec![
        link(AnnotationKind::Addresses, "REQ-001", &hash),
        link(AnnotationKind::Implements, "REQ-001", "deadbeef"),
        link(AnnotationKind::Verifies, "REQ-001", &hash),
    ];

    let check = check_requirement(&req, &annotations, Vec::new());
    let json = serde_json::to_value(&check).expect("serialize");

    assert_eq!(
        json,
        serde_json::json!({
            "id": "REQ-001",
            "state": "Stale",
            "missing_links": [],
            "stale_links": [
                {"kind": "implements", "recorded_hash": "deadbeef", "current_hash": hash}
            ],
            "drifted_files": []
        })
    );
}

// @verifies REQ-037 v2 2371e246
#[test]
fn traced_requirement_check_serializes_with_empty_gap_arrays() {
    let hash = current_hash();
    let req = requirement("REQ-001", &hash);
    let annotations = vec![
        link(AnnotationKind::Addresses, "REQ-001", &hash),
        link(AnnotationKind::Implements, "REQ-001", &hash),
        link(AnnotationKind::Verifies, "REQ-001", &hash),
    ];

    let check = check_requirement(&req, &annotations, Vec::new());
    let json = serde_json::to_value(&check).expect("serialize");

    assert_eq!(
        json,
        serde_json::json!({
            "id": "REQ-001",
            "state": "Traced",
            "missing_links": [],
            "stale_links": [],
            "drifted_files": []
        })
    );
}
