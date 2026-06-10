use weft_core::{
    canonical_hash, scan_annotations, trace_state, Annotation, AnnotationKind, Requirement,
    Status, TraceState,
};

#[test]
fn finds_an_implements_annotation_in_a_code_comment() {
    let src = "// @implements REQ-001 v1 a3f9b2c1\nfn login() {}\n";

    let annotations = scan_annotations(src);

    assert_eq!(
        annotations,
        vec![Annotation {
            kind: AnnotationKind::Implements,
            req_id: "REQ-001".to_string(),
            version: 1,
            hash: "a3f9b2c1".to_string(),
        }]
    );
}

#[test]
fn finds_a_verifies_annotation_in_a_test_comment() {
    let src = "# @verifies REQ-002 v3 deadbeef\ndef test_login(): ...\n";

    let annotations = scan_annotations(src);

    assert_eq!(
        annotations,
        vec![Annotation {
            kind: AnnotationKind::Verifies,
            req_id: "REQ-002".to_string(),
            version: 3,
            hash: "deadbeef".to_string(),
        }]
    );
}

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

#[test]
fn no_links_is_orphaned() {
    let req = requirement("REQ-001", &current_hash());

    assert_eq!(trace_state(&req, &[]), TraceState::Orphaned);
}

#[test]
fn missing_a_link_is_incomplete() {
    let hash = current_hash();
    let req = requirement("REQ-001", &hash);
    let annotations = vec![
        Annotation {
            kind: AnnotationKind::Addresses,
            req_id: "REQ-001".to_string(),
            version: 1,
            hash: hash.clone(),
        },
        Annotation {
            kind: AnnotationKind::Implements,
            req_id: "REQ-001".to_string(),
            version: 1,
            hash: hash.clone(),
        },
        // no @verifies link
    ];

    assert_eq!(trace_state(&req, &annotations), TraceState::Incomplete);
}

#[test]
fn a_link_pinning_an_old_hash_is_stale() {
    let hash = current_hash();
    let req = requirement("REQ-001", &hash);
    let annotations = vec![
        Annotation {
            kind: AnnotationKind::Addresses,
            req_id: "REQ-001".to_string(),
            version: 1,
            hash: hash.clone(),
        },
        Annotation {
            kind: AnnotationKind::Implements,
            req_id: "REQ-001".to_string(),
            version: 1,
            hash: "deadbeef".to_string(), // stale: req's hash has since changed
        },
        Annotation {
            kind: AnnotationKind::Verifies,
            req_id: "REQ-001".to_string(),
            version: 1,
            hash: hash.clone(),
        },
    ];

    assert_eq!(trace_state(&req, &annotations), TraceState::Stale);
}

#[test]
fn all_links_present_and_current_is_traced() {
    let hash = current_hash();
    let req = requirement("REQ-001", &hash);
    let annotations = vec![
        Annotation {
            kind: AnnotationKind::Addresses,
            req_id: "REQ-001".to_string(),
            version: 1,
            hash: hash.clone(),
        },
        Annotation {
            kind: AnnotationKind::Implements,
            req_id: "REQ-001".to_string(),
            version: 1,
            hash: hash.clone(),
        },
        Annotation {
            kind: AnnotationKind::Verifies,
            req_id: "REQ-001".to_string(),
            version: 1,
            hash: hash.clone(),
        },
    ];

    assert_eq!(trace_state(&req, &annotations), TraceState::Traced);
}

#[test]
fn finds_addresses_annotations_in_toml_frontmatter() {
    let src = "+++\naddresses = [\"REQ-001 v1 a3f9b2c1\", \"REQ-002 v2 deadbeef\"]\n+++\n\n# Decision\n";

    let annotations = scan_annotations(src);

    assert_eq!(
        annotations,
        vec![
            Annotation {
                kind: AnnotationKind::Addresses,
                req_id: "REQ-001".to_string(),
                version: 1,
                hash: "a3f9b2c1".to_string(),
            },
            Annotation {
                kind: AnnotationKind::Addresses,
                req_id: "REQ-002".to_string(),
                version: 2,
                hash: "deadbeef".to_string(),
            },
        ]
    );
}
