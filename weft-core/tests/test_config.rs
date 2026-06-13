use weft_core::{parse_test_config, resolve_test_command, Requirement, Status};

fn requirement(id: &str, feat: Option<&str>) -> Requirement {
    Requirement {
        id: id.to_string(),
        version: 1,
        feat: feat.map(String::from),
        hash: "deadbeef".to_string(),
        status: Status::Active,
        statement: "The system must allow users to log in.".to_string(),
        acceptance: vec!["Given valid credentials, the user is authenticated.".to_string()],
        rationale: None,
        notes: None,
    }
}

// @verifies REQ-042 v2 37857355
#[test]
fn parses_default_command_from_test_section() {
    let toml_src = r#"
[test]
command = "cargo test"
"#;

    let config = parse_test_config(toml_src).expect("expected a [test] section");

    assert_eq!(config.command, Some("cargo test".to_string()));
}

// @verifies REQ-042 v2 37857355
#[test]
fn missing_test_section_returns_none() {
    assert_eq!(parse_test_config(""), None);

    let toml_src = r#"
[other]
foo = "bar"
"#;
    assert_eq!(parse_test_config(toml_src), None);
}

// @verifies REQ-042 v2 37857355
#[test]
fn per_requirement_override_takes_precedence_over_feat_and_default() {
    let toml_src = r#"
[test]
command = "cargo test"

[test.overrides]
"FEAT-AutonomyLoop" = "cargo test -p weft"
"REQ-042" = "cargo test -p weft-core test_config"
"#;
    let config = parse_test_config(toml_src).expect("expected a [test] section");
    let req = requirement("REQ-042", Some("FEAT-AutonomyLoop"));

    assert_eq!(
        resolve_test_command(&config, &req),
        Some("cargo test -p weft-core test_config")
    );
}

// @verifies REQ-042 v2 37857355
#[test]
fn per_feat_override_takes_precedence_over_default() {
    let toml_src = r#"
[test]
command = "cargo test"

[test.overrides]
"FEAT-AutonomyLoop" = "cargo test -p weft"
"#;
    let config = parse_test_config(toml_src).expect("expected a [test] section");
    let req = requirement("REQ-043", Some("FEAT-AutonomyLoop"));

    assert_eq!(resolve_test_command(&config, &req), Some("cargo test -p weft"));
}

// @verifies REQ-042 v2 37857355
#[test]
fn default_command_used_when_no_override_applies() {
    let toml_src = r#"
[test]
command = "cargo test"
"#;
    let config = parse_test_config(toml_src).expect("expected a [test] section");
    let req = requirement("REQ-099", None);

    assert_eq!(resolve_test_command(&config, &req), Some("cargo test"));
}

// @verifies REQ-042 v2 37857355
#[test]
fn no_command_resolved_when_no_default_and_no_matching_override() {
    let toml_src = r#"
[test]

[test.overrides]
"FEAT-AutonomyLoop" = "cargo test -p weft"
"#;
    let config = parse_test_config(toml_src).expect("expected a [test] section");
    let req = requirement("REQ-099", None);

    assert_eq!(resolve_test_command(&config, &req), None);
}
