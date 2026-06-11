use std::fs;
use std::path::Path;

/// Reads the SKILL.md for `name` from `.github/skills/<name>/SKILL.md`.
fn skill_md(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../.github/skills")
        .join(name)
        .join("SKILL.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

// @verifies REQ-022 v2 b34eb010
#[test]
fn to_smart_prd_upserts_records_in_docs_prds_and_is_idempotent() {
    let skill = skill_md("to-smart-prd");

    assert!(
        skill.contains("weft new"),
        "to-smart-prd must allocate new records via `weft new`"
    );
    assert!(
        skill.contains("docs/prds"),
        "to-smart-prd must write records under docs/prds/"
    );
    assert!(
        skill.contains("idempotent"),
        "to-smart-prd must document that re-running with an unchanged session changes nothing"
    );
    assert!(
        skill.contains("@implements REQ-022 v2 b34eb010"),
        "to-smart-prd must carry the REQ-022 @implements trace annotation"
    );
}

// @verifies REQ-023 v2 4fa1a337
#[test]
fn to_smart_prd_bumps_only_when_normative_text_changed() {
    let skill = skill_md("to-smart-prd");

    assert!(
        skill.contains("weft bump"),
        "to-smart-prd must bump a record via `weft bump` when its text changed"
    );
    assert!(
        skill.contains("no action") || skill.contains("No action"),
        "to-smart-prd must document making no change when normative text matches"
    );
    assert!(
        skill.contains("@implements REQ-023 v2 4fa1a337"),
        "to-smart-prd must carry the REQ-023 @implements trace annotation"
    );
}

// @verifies REQ-024 v2 00a713df
#[test]
fn to_smart_prd_deprecates_requirements_absent_from_session_intent() {
    let skill = skill_md("to-smart-prd");

    assert!(
        skill.contains("weft deprecate"),
        "to-smart-prd must deprecate removed requirements via `weft deprecate`"
    );
    assert!(
        skill.contains("ask") && skill.contains("confirm"),
        "to-smart-prd must ask the user to confirm before deprecating an ambiguous requirement"
    );
    assert!(
        skill.contains("@implements REQ-024 v2 00a713df"),
        "to-smart-prd must carry the REQ-024 @implements trace annotation"
    );
}

// @verifies REQ-025 v2 6ff51574
#[test]
fn to_smart_issues_plans_slices_from_not_yet_traced_requirements() {
    let skill = skill_md("to-smart-issues");

    assert!(
        skill.contains("weft check"),
        "to-smart-issues must discover not-yet-Traced requirements via `weft check`"
    );
    assert!(
        skill.contains("Orphaned") && skill.contains("Incomplete"),
        "to-smart-issues must cover Orphaned and Incomplete requirements"
    );
    assert!(
        skill.contains("No implementation gaps"),
        "to-smart-issues must create no issues when nothing is Orphaned or Incomplete"
    );
    assert!(
        skill.contains("@implements REQ-025 v2 6ff51574"),
        "to-smart-issues must carry the REQ-025 @implements trace annotation"
    );
}

// @verifies REQ-026 v2 f2ba6521
#[test]
fn to_smart_issues_embeds_req_id_version_and_hash_per_requirement() {
    let skill = skill_md("to-smart-issues");

    assert!(
        skill.contains("REQ-NNN | vN | <hash>") || skill.contains("REQ-NNN vN <hash>"),
        "to-smart-issues issue body must embed REQ-NNN vN <hash> for each requirement"
    );
    assert!(
        skill.contains("@implements REQ-NNN") && skill.contains("@verifies REQ-NNN"),
        "to-smart-issues issue body must show copy-paste-correct @implements/@verifies annotations"
    );
    assert!(
        skill.contains("@implements REQ-026 v2 f2ba6521"),
        "to-smart-issues must carry the REQ-026 @implements trace annotation"
    );
}

// @verifies REQ-027 v2 af09e7de
#[test]
fn to_smart_issues_distinguishes_rework_from_implement_slices() {
    let skill = skill_md("to-smart-issues");

    assert!(
        skill.contains("`Stale`") && skill.contains("`rework`"),
        "to-smart-issues must label a Stale requirement's slice as rework"
    );
    assert!(
        skill.contains("`implement`"),
        "to-smart-issues must label an Orphaned/Incomplete requirement's slice as implement"
    );
    assert!(
        skill.contains("@implements REQ-027 v2 af09e7de"),
        "to-smart-issues must carry the REQ-027 @implements trace annotation"
    );
}

// @verifies REQ-028 v2 46490789
#[test]
fn to_smart_issues_slices_span_whole_requirements_to_traced() {
    let skill = skill_md("to-smart-issues");

    assert!(
        skill.contains("Never sub-divide a requirement"),
        "to-smart-issues must never split a requirement across slices"
    );
    assert!(
        skill.contains("`weft check` exits 0 and reports `Traced`"),
        "to-smart-issues must define a slice as done when weft check reports Traced"
    );
    assert!(
        skill.contains("@implements REQ-028 v2 46490789"),
        "to-smart-issues must carry the REQ-028 @implements trace annotation"
    );
}
