use weft_core::{
    annotation_line, canonical_hash, dangling_annotations, scan_annotations, summarize_trace_states,
    trace_state, Annotation, AnnotationKind, Requirement, Status, TraceState, TraceSummary,
};

// @verifies REQ-017 v2 8af530a5
// @verifies REQ-019 v2 ed4d3199
#[test]
fn finds_an_implements_annotation_in_a_code_comment() {
    let src = "// @implements REQ-901 v1 a3f9b2c1\nfn login() {}\n";

    let annotations = scan_annotations(src);

    assert_eq!(
        annotations,
        vec![Annotation {
            kind: AnnotationKind::Implements,
            req_id: "REQ-901".to_string(),
            version: 1,
            hash: "a3f9b2c1".to_string(),
        }]
    );
}

// @verifies REQ-018 v2 e2253535
// @verifies REQ-019 v2 ed4d3199
#[test]
fn finds_a_verifies_annotation_in_a_test_comment() {
    let src = "# @verifies REQ-902 v3 deadbeef\ndef test_login(): ...\n";

    let annotations = scan_annotations(src);

    assert_eq!(
        annotations,
        vec![Annotation {
            kind: AnnotationKind::Verifies,
            req_id: "REQ-902".to_string(),
            version: 3,
            hash: "deadbeef".to_string(),
        }]
    );
}

// @verifies REQ-019 v2 ed4d3199
#[test]
fn finds_annotations_regardless_of_comment_syntax() {
    let src = "<!-- @implements REQ-903 v1 cafef00d -->\n<div>login form</div>\n";

    let annotations = scan_annotations(src);

    assert_eq!(
        annotations,
        vec![Annotation {
            kind: AnnotationKind::Implements,
            req_id: "REQ-903".to_string(),
            version: 1,
            hash: "cafef00d".to_string(),
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

// @verifies REQ-020 v2 9abea869
#[test]
fn no_links_is_orphaned() {
    let req = requirement("REQ-001", &current_hash());

    assert_eq!(trace_state(&req, &[]), TraceState::Orphaned);
}

// @verifies REQ-020 v2 9abea869
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

// @verifies REQ-020 v2 9abea869
// @verifies REQ-021 v2 58781e5c
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

// @verifies REQ-020 v2 9abea869
// @verifies REQ-021 v2 58781e5c
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

// @verifies REQ-040 v2 1ead8691
#[test]
fn summarizes_trace_states_into_per_state_counts() {
    let states = vec![
        TraceState::Traced,
        TraceState::Traced,
        TraceState::Incomplete,
        TraceState::Orphaned,
    ];

    let summary = summarize_trace_states(&states);

    assert_eq!(
        summary,
        TraceSummary {
            orphaned: 1,
            incomplete: 1,
            stale: 0,
            drifted: 0,
            traced: 2,
        }
    );
    assert_eq!(summary.total(), 4);
    assert_eq!(
        summary.to_string(),
        "Orphaned: 1\nIncomplete: 1\nStale: 0\nDrifted: 0\nTraced: 2\n2/4 Traced"
    );
}

// @verifies REQ-041 v2 7194e93b
#[test]
fn finds_the_line_number_of_an_annotation() {
    let src = "fn login() {}\n// @implements REQ-901 v1 a3f9b2c1\nfn other() {}\n";
    let annotation = Annotation {
        kind: AnnotationKind::Implements,
        req_id: "REQ-901".to_string(),
        version: 1,
        hash: "a3f9b2c1".to_string(),
    };

    assert_eq!(annotation_line(src, &annotation), Some(2));
}

// @verifies REQ-041 v2 7194e93b
#[test]
fn annotation_line_is_none_when_not_found() {
    let src = "fn login() {}\n";
    let annotation = Annotation {
        kind: AnnotationKind::Implements,
        req_id: "REQ-901".to_string(),
        version: 1,
        hash: "a3f9b2c1".to_string(),
    };

    assert_eq!(annotation_line(src, &annotation), None);
}

// @verifies REQ-041 v2 7194e93b
#[test]
fn dangling_annotations_finds_links_to_unknown_or_deprecated_requirements() {
    let file_annotations = vec![(
        "src/login.rs".to_string(),
        vec![
            Annotation {
                kind: AnnotationKind::Implements,
                req_id: "REQ-001".to_string(),
                version: 1,
                hash: "a3f9b2c1".to_string(),
            },
            Annotation {
                kind: AnnotationKind::Implements,
                req_id: "REQ-099".to_string(),
                version: 1,
                hash: "deadbeef".to_string(),
            },
        ],
    )];
    let active_ids = vec!["REQ-001".to_string()];

    let dangling = dangling_annotations(&file_annotations, &active_ids);

    assert_eq!(dangling, vec![("src/login.rs", &file_annotations[0].1[1])]);
}

// @verifies REQ-016 v2 84ac8548
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

// @verifies REQ-047 v2 c4c2f006
#[test]
fn ignores_an_implements_annotation_citing_the_reserved_example_id() {
    let src = "// @implements REQ-000 v3 a3f9b2c1\nfn login() {}\n";

    let annotations = scan_annotations(src);

    assert_eq!(annotations, vec![]);
}

// @verifies REQ-047 v2 c4c2f006
#[test]
fn ignores_a_verifies_annotation_citing_the_reserved_example_id() {
    let src = "# @verifies REQ-000 v3 a3f9b2c1\ndef test_login(): ...\n";

    let annotations = scan_annotations(src);

    assert_eq!(annotations, vec![]);
}

// @verifies REQ-047 v2 c4c2f006
#[test]
fn ignores_an_addresses_annotation_citing_the_reserved_example_id_in_frontmatter() {
    let src = "+++\naddresses = [\"REQ-000 v3 a3f9b2c1\", \"REQ-002 v2 deadbeef\"]\n+++\n\n# Decision\n";

    let annotations = scan_annotations(src);

    assert_eq!(
        annotations,
        vec![Annotation {
            kind: AnnotationKind::Addresses,
            req_id: "REQ-002".to_string(),
            version: 2,
            hash: "deadbeef".to_string(),
        }]
    );
}

// @verifies REQ-047 v2 c4c2f006
#[test]
fn an_annotation_citing_a_non_reserved_unknown_id_is_still_dangling() {
    let file_annotations = vec![(
        "src/lib.rs".to_string(),
        scan_annotations("// @implements REQ-999 v1 deadbeef\nfn x() {}\n"),
    )];
    let active_ids = vec!["REQ-001".to_string()];

    let dangling = dangling_annotations(&file_annotations, &active_ids);

    assert_eq!(dangling.len(), 1);
    assert_eq!(dangling[0].1.req_id, "REQ-999");
}
