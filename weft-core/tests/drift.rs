use std::collections::BTreeMap;

use weft_core::{
    all_annotated_files, canonical_hash, drifted_paths, file_hash, files_for_requirement,
    parse_lock, render_lock, trace_state_with_drift, Annotation, AnnotationKind, Requirement,
    Status, TraceState,
};

// @verifies REQ-031 v2 6cdbe6cb
#[test]
fn file_hash_is_a_64_char_lowercase_hex_sha256() {
    let hash = file_hash(b"");

    assert_eq!(
        hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(hash.len(), 64);
    assert!(hash.chars().all(|c: char| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
}

// @verifies REQ-031 v2 6cdbe6cb
#[test]
fn render_lock_then_parse_lock_round_trips() {
    let mut entries = BTreeMap::new();
    entries.insert("src/login.rs".to_string(), file_hash(b"fn login() {}"));
    entries.insert("tests/login.rs".to_string(), file_hash(b"fn test_login() {}"));

    let rendered = render_lock(&entries);
    let parsed = parse_lock(&rendered);

    assert_eq!(parsed, entries);
}

// @verifies REQ-032 v2 a2441bcc
#[test]
fn files_for_requirement_returns_only_files_annotated_for_that_req_id() {
    let file_annotations = vec![
        (
            "src/login.rs".to_string(),
            vec![Annotation {
                kind: AnnotationKind::Implements,
                req_id: "REQ-901".to_string(),
                version: 1,
                hash: "aaaaaaaa".to_string(),
            }],
        ),
        (
            "src/logout.rs".to_string(),
            vec![Annotation {
                kind: AnnotationKind::Implements,
                req_id: "REQ-902".to_string(),
                version: 1,
                hash: "bbbbbbbb".to_string(),
            }],
        ),
    ];

    assert_eq!(
        files_for_requirement("REQ-901", &file_annotations),
        vec!["src/login.rs".to_string()]
    );
}

// @verifies REQ-031 v2 6cdbe6cb
// @verifies REQ-032 v2 a2441bcc
#[test]
fn all_annotated_files_returns_sorted_deduplicated_paths_with_any_annotation() {
    let file_annotations = vec![
        (
            "src/logout.rs".to_string(),
            vec![Annotation {
                kind: AnnotationKind::Implements,
                req_id: "REQ-902".to_string(),
                version: 1,
                hash: "bbbbbbbb".to_string(),
            }],
        ),
        ("README.md".to_string(), vec![]),
        (
            "src/login.rs".to_string(),
            vec![Annotation {
                kind: AnnotationKind::Implements,
                req_id: "REQ-901".to_string(),
                version: 1,
                hash: "aaaaaaaa".to_string(),
            }],
        ),
    ];

    assert_eq!(
        all_annotated_files(&file_annotations),
        vec!["src/login.rs".to_string(), "src/logout.rs".to_string()]
    );
}

// @verifies REQ-033 v2 04d42b48
#[test]
fn drifted_paths_flags_changed_and_missing_entries() {
    let paths = vec![
        "src/login.rs".to_string(),
        "src/logout.rs".to_string(),
        "src/new.rs".to_string(),
    ];
    let mut lock = BTreeMap::new();
    lock.insert("src/login.rs".to_string(), "unchanged".to_string());
    lock.insert("src/logout.rs".to_string(), "old".to_string());
    // src/new.rs has no entry in weft.lock at all.

    let mut current_hashes = BTreeMap::new();
    current_hashes.insert("src/login.rs".to_string(), "unchanged".to_string());
    current_hashes.insert("src/logout.rs".to_string(), "new".to_string());
    current_hashes.insert("src/new.rs".to_string(), "new".to_string());

    assert_eq!(
        drifted_paths(&paths, &lock, &current_hashes),
        vec!["src/logout.rs".to_string(), "src/new.rs".to_string()]
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

// @verifies REQ-033 v2 04d42b48
#[test]
fn traced_with_no_drifted_files_stays_traced() {
    let hash = current_hash();
    let req = requirement("REQ-901", &hash);
    let annotations = traced_annotations(&hash);

    assert_eq!(
        trace_state_with_drift(&req, &annotations, Vec::new()),
        TraceState::Traced
    );
}

// @verifies REQ-033 v2 04d42b48
#[test]
fn traced_with_drifted_files_becomes_drifted() {
    let hash = current_hash();
    let req = requirement("REQ-901", &hash);
    let annotations = traced_annotations(&hash);

    assert_eq!(
        trace_state_with_drift(&req, &annotations, vec!["src/login.rs".to_string()]),
        TraceState::Drifted(vec!["src/login.rs".to_string()])
    );
}

// @verifies REQ-033 v2 04d42b48
#[test]
fn stale_takes_precedence_over_drifted() {
    let hash = current_hash();
    let req = requirement("REQ-901", &hash);
    let mut annotations = traced_annotations(&hash);
    // Pin a stale hash on the @implements link.
    annotations[1].hash = "deadbeef".to_string();

    assert_eq!(
        trace_state_with_drift(&req, &annotations, vec!["src/login.rs".to_string()]),
        TraceState::Stale
    );
}

// @verifies REQ-033 v2 04d42b48
#[test]
fn drifted_display_lists_changed_files() {
    let state = TraceState::Drifted(vec!["src/login.rs".to_string(), "tests/login.rs".to_string()]);

    assert_eq!(state.to_string(), "Drifted (src/login.rs, tests/login.rs)");
}
