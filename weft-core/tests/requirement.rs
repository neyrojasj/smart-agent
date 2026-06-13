use weft_core::{
    canonical_hash, verify_not_user_story, verify_requirement, Requirement, Status, VerifyIssue,
};

const STATEMENT: &str = "The system must allow users to log in with email and password.";
const ACCEPTANCE: &[&str] = &[
    "Given valid credentials, the user is authenticated.",
    "Given invalid credentials, an error is shown.",
];

fn fixture_toml(hash: &str, id: &str) -> String {
    format!(
        r#"
id = "{id}"
version = 1
feat = "FEAT-Auth"
hash = "{hash}"
status = "active"
statement = "{STATEMENT}"

acceptance = [
    "{a0}",
    "{a1}",
]

rationale = "Login is the entry point for all authenticated features."
"#,
        a0 = ACCEPTANCE[0],
        a1 = ACCEPTANCE[1],
    )
}

fn current_hash() -> String {
    let acceptance: Vec<String> = ACCEPTANCE.iter().map(|s| s.to_string()).collect();
    canonical_hash(STATEMENT, &acceptance)
}

// @verifies REQ-001 v2 f99f9f41
// @verifies REQ-002 v2 1c857a61
// @verifies REQ-005 v2 0659bb8e
#[test]
fn parses_a_well_formed_requirement_record() {
    let hash = current_hash();
    let toml_src = fixture_toml(&hash, "REQ-001");

    let req = Requirement::parse(&toml_src).expect("should parse");

    assert_eq!(req.id, "REQ-001");
    assert_eq!(req.version, 1);
    assert_eq!(req.feat.as_deref(), Some("FEAT-Auth"));
    assert_eq!(req.hash, hash);
    assert_eq!(req.status, Status::Active);
    assert_eq!(req.statement, STATEMENT);
    assert_eq!(req.acceptance.len(), 2);
}

#[test]
fn well_formed_record_with_current_hash_passes_verification() {
    let hash = current_hash();
    let toml_src = fixture_toml(&hash, "REQ-001");
    let req = Requirement::parse(&toml_src).expect("should parse");

    let issues = verify_requirement(&req, "REQ-001");

    assert!(issues.is_empty(), "expected no issues, got {issues:?}");
}

#[test]
fn stale_stored_hash_fails_verification_with_bump_message() {
    let toml_src = fixture_toml("deadbeef", "REQ-001");
    let req = Requirement::parse(&toml_src).expect("should parse");

    let issues = verify_requirement(&req, "REQ-001");

    assert_eq!(issues.len(), 1);
    match &issues[0] {
        VerifyIssue::HashMismatch { stored, derived, .. } => {
            assert_eq!(stored, "deadbeef");
            assert_eq!(derived, &current_hash());
        }
        other => panic!("expected HashMismatch, got {other:?}"),
    }
    assert!(issues[0].to_string().contains("weft bump REQ-001"));
}

#[test]
fn id_not_matching_filename_fails_verification() {
    let hash = current_hash();
    let toml_src = fixture_toml(&hash, "REQ-001");
    let req = Requirement::parse(&toml_src).expect("should parse");

    let issues = verify_requirement(&req, "REQ-002");

    assert!(issues
        .iter()
        .any(|i| matches!(i, VerifyIssue::IdFilenameMismatch { .. })));
}

#[test]
fn malformed_id_fails_verification() {
    let hash = current_hash();
    let toml_src = fixture_toml(&hash, "REQ-1");
    let req = Requirement::parse(&toml_src).expect("should parse");

    let issues = verify_requirement(&req, "REQ-1");

    assert!(issues
        .iter()
        .any(|i| matches!(i, VerifyIssue::InvalidIdFormat(_))));
}

// @verifies REQ-006 v2 72cef08d
#[test]
fn deprecated_status_round_trips() {
    let hash = current_hash();
    let toml_src = format!(
        r#"
id = "REQ-001"
version = 1
hash = "{hash}"
status = "deprecated"
statement = "{STATEMENT}"
acceptance = [
    "{a0}",
    "{a1}",
]
"#,
        a0 = ACCEPTANCE[0],
        a1 = ACCEPTANCE[1],
    );

    let req = Requirement::parse(&toml_src).expect("should parse");

    assert_eq!(req.status, Status::Deprecated);
    assert!(
        verify_requirement(&req, "REQ-001").is_empty(),
        "a deprecated record with intact normative text should still verify cleanly"
    );
}

#[test]
fn empty_acceptance_fails_verification() {
    let toml_src = format!(
        r#"
id = "REQ-001"
version = 1
hash = "00000000"
status = "active"
statement = "{STATEMENT}"
acceptance = []
"#
    );
    let req = Requirement::parse(&toml_src).expect("should parse");

    let issues = verify_requirement(&req, "REQ-001");

    assert!(issues
        .iter()
        .any(|i| matches!(i, VerifyIssue::EmptyAcceptance)));
}

// @verifies REQ-030 v2 bf5f866e
#[test]
fn record_with_user_story_fields_is_rejected() {
    let hash = current_hash();
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
as_a = "developer"
i_want = "to log in with email and password"
so_that = "I can access my account"
"#,
        a0 = ACCEPTANCE[0],
        a1 = ACCEPTANCE[1],
    );

    let issue = verify_not_user_story(&toml_src);

    assert!(
        matches!(issue, Some(VerifyIssue::UserStoryRecord(_))),
        "expected a UserStoryRecord issue, got {issue:?}"
    );
}

// @verifies REQ-030 v2 bf5f866e
#[test]
fn record_without_user_story_fields_passes() {
    let hash = current_hash();
    let toml_src = fixture_toml(&hash, "REQ-001");

    assert!(
        verify_not_user_story(&toml_src).is_none(),
        "a normal requirement record must not be flagged as a User Story"
    );
}

// @verifies REQ-047 v2 c4c2f006
#[test]
fn req_000_is_rejected_as_the_reserved_example_id() {
    let hash = current_hash();
    let toml_src = fixture_toml(&hash, "REQ-000");
    let req = Requirement::parse(&toml_src).expect("should parse");

    let issues = verify_requirement(&req, "REQ-000");

    assert!(
        issues.iter().any(|i| matches!(i, VerifyIssue::ReservedExampleId)),
        "expected a ReservedExampleId issue, got {issues:?}"
    );
}
