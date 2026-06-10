use weft_core::{bump, canonical_hash, Requirement, Status};

fn requirement(version: u32, hash: &str, statement: &str, acceptance: &[&str]) -> Requirement {
    Requirement {
        id: "REQ-001".to_string(),
        version,
        feat: None,
        hash: hash.to_string(),
        status: Status::Active,
        statement: statement.to_string(),
        acceptance: acceptance.iter().map(|s| s.to_string()).collect(),
        rationale: None,
        notes: None,
    }
}

// @verifies REQ-003 v2 6e343519
#[test]
fn bump_increments_version_and_recomputes_hash_from_current_text() {
    let statement = "The system must allow users to log in.";
    let acceptance = ["Given valid credentials, the user is authenticated."];
    // stored hash is stale (doesn't match the current statement/acceptance)
    let req = requirement(1, "deadbeef", statement, &acceptance);

    let bumped = bump(&req);

    assert_eq!(bumped.version, 2);
    assert_eq!(
        bumped.hash,
        canonical_hash(
            statement,
            &acceptance.iter().map(|s| s.to_string()).collect::<Vec<_>>()
        )
    );
    assert_ne!(bumped.hash, "deadbeef");
}
