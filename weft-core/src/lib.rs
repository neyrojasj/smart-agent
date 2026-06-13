//! weft-core: requirement record parsing, canonical hashing, and verification.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;

/// The lifecycle state of a requirement record.
// @implements REQ-006 v2 72cef08d
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    Deprecated,
}

/// A single requirement record, parsed from a `docs/prds/**/*.toml` file.
// @implements REQ-001 v2 f99f9f41
// @implements REQ-002 v2 1c857a61
// @implements REQ-005 v2 0659bb8e
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Requirement {
    pub id: String,
    pub version: u32,
    pub feat: Option<String>,
    pub hash: String,
    pub status: Status,
    pub statement: String,
    pub acceptance: Vec<String>,
    pub rationale: Option<String>,
    pub notes: Option<String>,
}

impl Requirement {
    /// Parses a requirement record from its TOML source.
    pub fn parse(toml_src: &str) -> Result<Requirement, toml::de::Error> {
        toml::from_str(toml_src)
    }
}

/// Computes the Content Hash for a requirement's normative region: the
/// `statement` (trimmed) plus each `acceptance` item (trimmed), joined with
/// `\n`, NFC-normalized, then SHA-256, truncated to the first 8 hex chars.
// @implements REQ-004 v2 7766a56e
pub fn canonical_hash(statement: &str, acceptance: &[String]) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(1 + acceptance.len());
    parts.push(statement.trim().to_string());
    parts.extend(acceptance.iter().map(|item| item.trim().to_string()));
    let canonical: String = parts.join("\n").nfc().collect();

    let digest = Sha256::digest(canonical.as_bytes());
    digest.iter().take(4).map(|b| format!("{b:02x}")).collect()
}

/// A single problem found while verifying a requirement record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyIssue {
    /// `id` does not match the `REQ-NNN` shape.
    InvalidIdFormat(String),
    /// `id` does not match the record's filename (without extension).
    IdFilenameMismatch { id: String, filename: String },
    /// `acceptance` has no entries.
    EmptyAcceptance,
    /// The stored `hash` no longer matches the hash derived from the
    /// normative region — the requirement was edited without bumping.
    HashMismatch {
        id: String,
        stored: String,
        derived: String,
    },
    /// The record contains a top-level User Story field (`as_a`, `i_want`,
    /// `so_that`, or `user_story`). User Stories are ephemeral and must
    /// never be persisted in `docs/prds/`.
    UserStoryRecord(String),
    /// `id` is [`EXAMPLE_REQ_ID`], the reserved id for illustrative
    /// annotation examples — it can never be a real requirement.
    ReservedExampleId,
}

impl fmt::Display for VerifyIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerifyIssue::InvalidIdFormat(id) => {
                write!(f, "'{id}' is not a valid requirement id (expected REQ-NNN)")
            }
            VerifyIssue::IdFilenameMismatch { id, filename } => {
                write!(f, "id '{id}' does not match filename '{filename}'")
            }
            VerifyIssue::EmptyAcceptance => write!(f, "acceptance must not be empty"),
            VerifyIssue::HashMismatch {
                id,
                stored,
                derived,
            } => write!(
                f,
                "stored hash '{stored}' does not match derived hash '{derived}' \
                 — the requirement was edited without bumping; run `weft bump {id}`"
            ),
            VerifyIssue::UserStoryRecord(field) => write!(
                f,
                "record contains '{field}', a User Story field — User Stories must never be \
                 persisted in docs/prds/; generate them ephemerally at implementation time"
            ),
            VerifyIssue::ReservedExampleId => write!(
                f,
                "'{EXAMPLE_REQ_ID}' is reserved for illustrative annotation examples and cannot \
                 be used as a real requirement id"
            ),
        }
    }
}

/// The reserved REQ_ID for illustrative annotation examples (ADR 0015). An
/// `@addresses`, `@implements`, or `@verifies` annotation citing this id is
/// dropped by [`scan_annotations`] — it contributes no Trace Link and is
/// never reported as a dangling annotation — so annotation syntax can be
/// documented inline, e.g. `@implements REQ-000 v3 a3f9b2c1`. `weft new`
/// never allocates this id (ids start at `REQ-001`), and [`verify_requirement`]
/// rejects a record that claims it.
// @implements REQ-047 v2 c4c2f006
pub const EXAMPLE_REQ_ID: &str = "REQ-000";

/// `id` must be `REQ-` followed by exactly three ASCII digits.
fn is_valid_id_format(id: &str) -> bool {
    match id.strip_prefix("REQ-") {
        Some(digits) => digits.len() == 3 && digits.chars().all(|c| c.is_ascii_digit()),
        None => false,
    }
}

/// Computes the next globally-unique `REQ-NNN` id given the ids of existing
/// requirement records (in any order, including malformed ones, which are
/// ignored).
pub fn next_req_id<'a>(existing_ids: impl Iterator<Item = &'a str>) -> String {
    let max = existing_ids
        .filter_map(|id| id.strip_prefix("REQ-"))
        .filter_map(|digits| digits.parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    format!("REQ-{:03}", max + 1)
}

/// Renders a skeleton requirement record for `id`, with placeholder
/// `statement`/`acceptance` and a `hash` that matches them, ready for the
/// author to fill in. If `feat` is given, it is included as the `feat` field.
pub fn skeleton_toml(id: &str, feat: Option<&str>) -> String {
    const STATEMENT: &str = "TODO: describe this requirement.";
    let acceptance = vec!["TODO: define an acceptance criterion.".to_string()];
    let hash = canonical_hash(STATEMENT, &acceptance);

    let mut out = String::new();
    out.push_str(&format!("id = \"{id}\"\n"));
    out.push_str("version = 1\n");
    if let Some(feat) = feat {
        out.push_str(&format!("feat = \"{feat}\"\n"));
    }
    out.push_str(&format!("hash = \"{hash}\"\n"));
    out.push_str("status = \"active\"\n");
    out.push_str(&format!("statement = \"{STATEMENT}\"\n"));
    out.push_str("acceptance = [\n");
    for item in &acceptance {
        out.push_str(&format!("    \"{item}\",\n"));
    }
    out.push_str("]\n");
    out
}

/// The new `version` and `hash` produced by [`bump`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bumped {
    pub version: u32,
    pub hash: String,
}

/// Bumps a requirement: increments `version` and recomputes `hash` from its
/// current normative region, as one operation — so a version bump and a hash
/// update can never happen independently.
// @implements REQ-003 v2 6e343519
pub fn bump(req: &Requirement) -> Bumped {
    Bumped {
        version: req.version + 1,
        hash: canonical_hash(&req.statement, &req.acceptance),
    }
}

/// The first line of a requirement's `statement`, trimmed — used as its
/// short description in listings.
pub fn description(statement: &str) -> &str {
    statement.lines().next().unwrap_or("").trim()
}

/// The chain stop a [`Annotation`] declares: design, code, or test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationKind {
    /// `@addresses` — a design decision addresses a requirement.
    Addresses,
    /// `@implements` — code implements a requirement.
    Implements,
    /// `@verifies` — a test verifies a requirement.
    Verifies,
}

impl fmt::Display for AnnotationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnnotationKind::Addresses => write!(f, "@addresses"),
            AnnotationKind::Implements => write!(f, "@implements"),
            AnnotationKind::Verifies => write!(f, "@verifies"),
        }
    }
}

impl AnnotationKind {
    /// The bare chain-stop name, without the `@` marker — used in gap detail
    /// reported by [`check_requirement`] (e.g. `"implements"`).
    // @implements REQ-036 v2 4cbbd466
    pub fn as_str(&self) -> &'static str {
        match self {
            AnnotationKind::Addresses => "addresses",
            AnnotationKind::Implements => "implements",
            AnnotationKind::Verifies => "verifies",
        }
    }
}

/// A single Trace Link found by scanning a file: a requirement id pinned to
/// the version and Content Hash that were current when the link was written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
    pub kind: AnnotationKind,
    pub req_id: String,
    pub version: u32,
    pub hash: String,
}

/// Scans `text` for Trace Links: `@addresses` entries in TOML frontmatter
/// (DEC/ADR docs), and inline `@implements`/`@verifies` markers, one per
/// line, in any comment syntax: `@implements REQ-000 v3 a3f9b2c1`. An
/// annotation citing [`EXAMPLE_REQ_ID`] is dropped — it is an illustrative
/// example, not a real Trace Link (ADR 0015).
// @implements REQ-019 v2 ed4d3199
// @implements REQ-047 v2 c4c2f006
pub fn scan_annotations(text: &str) -> Vec<Annotation> {
    let mut out = scan_addresses_frontmatter(text);
    for line in text.lines() {
        if let Some(idx) = line.find("@implements") {
            if let Some(annotation) = parse_inline_annotation(&line[idx..], AnnotationKind::Implements)
            {
                out.push(annotation);
            }
        } else if let Some(idx) = line.find("@verifies") {
            if let Some(annotation) = parse_inline_annotation(&line[idx..], AnnotationKind::Verifies)
            {
                out.push(annotation);
            }
        }
    }
    out.retain(|a| a.req_id != EXAMPLE_REQ_ID);
    out
}

/// Extracts `@addresses` Trace Links from a `+++`-delimited TOML frontmatter
/// block at the start of `text` (DEC/ADR docs). Returns an empty vec if
/// `text` has no frontmatter, the frontmatter is not valid TOML, or it has no
/// `addresses` array.
// @implements REQ-016 v2 84ac8548
fn scan_addresses_frontmatter(text: &str) -> Vec<Annotation> {
    let Some(rest) = text.strip_prefix("+++\n") else {
        return Vec::new();
    };
    let Some(end) = rest.find("\n+++") else {
        return Vec::new();
    };
    let Ok(frontmatter) = rest[..end].parse::<toml::Value>() else {
        return Vec::new();
    };
    let Some(addresses) = frontmatter.get("addresses").and_then(|v| v.as_array()) else {
        return Vec::new();
    };

    addresses
        .iter()
        .filter_map(|v| v.as_str())
        .filter_map(parse_addresses_entry)
        .collect()
}

/// Parses an `addresses` entry of the form `REQ-000 v3 a3f9b2c1` (no
/// `@addresses` marker — the field name itself is the marker).
fn parse_addresses_entry(s: &str) -> Option<Annotation> {
    let mut tokens = s.split_whitespace();
    let req_id = tokens.next()?.to_string();
    let version = tokens.next()?.strip_prefix('v')?.parse::<u32>().ok()?;
    let hash = tokens.next()?.to_string();
    Some(Annotation {
        kind: AnnotationKind::Addresses,
        req_id,
        version,
        hash,
    })
}

/// Parses `@implements REQ-000 v3 a3f9b2c1` (or `@verifies ...`) starting at
/// the marker itself.
// @implements REQ-017 v2 8af530a5
// @implements REQ-018 v2 e2253535
fn parse_inline_annotation(s: &str, kind: AnnotationKind) -> Option<Annotation> {
    let mut tokens = s.split_whitespace();
    tokens.next()?; // the @implements / @verifies marker itself
    let req_id = tokens.next()?.to_string();
    let version = tokens.next()?.strip_prefix('v')?.parse::<u32>().ok()?;
    let hash = tokens.next()?.to_string();
    Some(Annotation {
        kind,
        req_id,
        version,
        hash,
    })
}

/// Finds the 1-based line number of `annotation` within `text`, by locating
/// the line containing the same `req_id`/`version`/`hash` text that
/// [`scan_annotations`] matched. Returns `None` if no line contains it (e.g.
/// the text has changed since `annotation` was scanned).
// @implements REQ-041 v2 7194e93b
pub fn annotation_line(text: &str, annotation: &Annotation) -> Option<usize> {
    let needle = format!("{} v{} {}", annotation.req_id, annotation.version, annotation.hash);
    text.lines()
        .position(|line| line.contains(&needle))
        .map(|line| line + 1)
}

/// All `(path, annotation)` pairs from `file_annotations` whose `req_id` does
/// not match any id in `active_ids` — Trace Links pointing at a requirement
/// that is unknown or deprecated.
// @implements REQ-041 v2 7194e93b
pub fn dangling_annotations<'a>(
    file_annotations: &'a [(String, Vec<Annotation>)],
    active_ids: &[String],
) -> Vec<(&'a str, &'a Annotation)> {
    file_annotations
        .iter()
        .flat_map(|(path, annotations)| {
            annotations
                .iter()
                .filter(|a| !active_ids.iter().any(|id| id == &a.req_id))
                .map(move |a| (path.as_str(), a))
        })
        .collect()
}

/// The static verdict for a requirement: do its Trace Links exist
/// (completeness), do their frozen hashes match the requirement's current
/// Content Hash (freshness), and do its annotated files match their sealed
/// File Hashes in `weft.lock` (artifact integrity)?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceState {
    /// No Trace Links at all.
    Orphaned,
    /// At least one Trace Link is missing (design, code, or test).
    Incomplete,
    /// All three Trace Links are present, but at least one pins a hash that
    /// no longer matches the requirement's current Content Hash.
    Stale,
    /// All three Trace Links are present and current, but at least one
    /// annotated file's current SHA-256 differs from (or is absent from) its
    /// stored File Hash in `weft.lock`. Carries the names of the changed
    /// files.
    // @implements REQ-033 v2 04d42b48
    Drifted(Vec<String>),
    /// All three Trace Links are present and current, and every annotated
    /// file matches its sealed File Hash.
    Traced,
}

impl fmt::Display for TraceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraceState::Orphaned => write!(f, "Orphaned"),
            TraceState::Incomplete => write!(f, "Incomplete"),
            TraceState::Stale => write!(f, "Stale"),
            TraceState::Drifted(files) => write!(f, "Drifted ({})", files.join(", ")),
            TraceState::Traced => write!(f, "Traced"),
        }
    }
}

impl TraceState {
    /// The state's name alone, without `Drifted`'s file list — the `state`
    /// field of [`RequirementCheck`]'s JSON representation.
    // @implements REQ-037 v2 2371e246
    pub fn name(&self) -> &'static str {
        match self {
            TraceState::Orphaned => "Orphaned",
            TraceState::Incomplete => "Incomplete",
            TraceState::Stale => "Stale",
            TraceState::Drifted(_) => "Drifted",
            TraceState::Traced => "Traced",
        }
    }
}

/// A rollup count of [`TraceState`]s, one bucket per state — printed by
/// `weft check --summary` as an alternative to the per-requirement listing.
// @implements REQ-040 v2 1ead8691
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TraceSummary {
    pub orphaned: usize,
    pub incomplete: usize,
    pub stale: usize,
    pub drifted: usize,
    pub traced: usize,
}

impl TraceSummary {
    /// The total number of requirements summarized.
    pub fn total(&self) -> usize {
        self.orphaned + self.incomplete + self.stale + self.drifted + self.traced
    }
}

impl fmt::Display for TraceSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Orphaned: {}", self.orphaned)?;
        writeln!(f, "Incomplete: {}", self.incomplete)?;
        writeln!(f, "Stale: {}", self.stale)?;
        writeln!(f, "Drifted: {}", self.drifted)?;
        writeln!(f, "Traced: {}", self.traced)?;
        write!(f, "{}/{} Traced", self.traced, self.total())
    }
}

/// Buckets `states` into a [`TraceSummary`], one count per [`TraceState`]
/// variant.
// @implements REQ-040 v2 1ead8691
pub fn summarize_trace_states(states: &[TraceState]) -> TraceSummary {
    let mut summary = TraceSummary::default();
    for state in states {
        match state {
            TraceState::Orphaned => summary.orphaned += 1,
            TraceState::Incomplete => summary.incomplete += 1,
            TraceState::Stale => summary.stale += 1,
            TraceState::Drifted(_) => summary.drifted += 1,
            TraceState::Traced => summary.traced += 1,
        }
    }
    summary
}

/// Computes `req`'s [`TraceState`] from the Trace Links found by
/// [`scan_annotations`] across the project (annotations for other
/// requirements are ignored).
// @implements REQ-020 v2 9abea869
// @implements REQ-021 v2 58781e5c
pub fn trace_state(req: &Requirement, annotations: &[Annotation]) -> TraceState {
    let find = |kind: AnnotationKind| {
        annotations
            .iter()
            .find(|a| a.kind == kind && a.req_id == req.id)
    };

    let links = [
        find(AnnotationKind::Addresses),
        find(AnnotationKind::Implements),
        find(AnnotationKind::Verifies),
    ];

    let present: Vec<&Annotation> = links.into_iter().flatten().collect();

    if present.is_empty() {
        return TraceState::Orphaned;
    }
    if present.len() < 3 {
        return TraceState::Incomplete;
    }
    if present.iter().any(|a| a.hash != req.hash) {
        return TraceState::Stale;
    }
    TraceState::Traced
}

/// Refines [`trace_state`]'s verdict with artifact integrity: if the base
/// state is `Traced` but `drifted` (the annotated files whose current
/// SHA-256 no longer matches their stored File Hash) is non-empty, the
/// requirement is `Drifted` instead. `Stale` takes precedence over `Drifted`
/// — fix requirement drift first.
// @implements REQ-033 v2 04d42b48
pub fn trace_state_with_drift(
    req: &Requirement,
    annotations: &[Annotation],
    drifted: Vec<String>,
) -> TraceState {
    let state = trace_state(req, annotations);
    if state == TraceState::Traced && !drifted.is_empty() {
        TraceState::Drifted(drifted)
    } else {
        state
    }
}

/// A Trace Link that is present but pins a hash that no longer matches the
/// requirement's current Content Hash, with both hashes for comparison.
// @implements REQ-036 v2 4cbbd466
// @implements REQ-037 v2 2371e246
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StaleLink {
    pub kind: String,
    pub recorded_hash: String,
    pub current_hash: String,
}

/// The specific gap that prevents a requirement from being `Traced`: missing
/// Trace Link kinds (Incomplete), stale Trace Links with their recorded and
/// current hashes (Stale), or drifted file paths (Drifted). Empty for
/// `Orphaned` and `Traced`.
// @implements REQ-036 v2 4cbbd466
// @implements REQ-037 v2 2371e246
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct TraceGap {
    pub missing_links: Vec<String>,
    pub stale_links: Vec<StaleLink>,
    pub drifted_files: Vec<String>,
}

/// A requirement's [`TraceState`] plus the structured [`TraceGap`] explaining
/// why it is not `Traced`. The single underlying result shared by `weft
/// check`'s human-readable ([`fmt::Display`]) and `--json`
/// ([`serde::Serialize`]) renderers, so the two views can never diverge.
// @implements REQ-036 v2 4cbbd466
// @implements REQ-037 v2 2371e246
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementCheck {
    pub id: String,
    pub state: TraceState,
    pub gap: TraceGap,
}

impl fmt::Display for RequirementCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.state {
            TraceState::Incomplete => write!(
                f,
                "{}: Incomplete (missing {})",
                self.id,
                self.gap.missing_links.join(", ")
            ),
            TraceState::Stale => {
                let parts: Vec<String> = self
                    .gap
                    .stale_links
                    .iter()
                    .map(|link| {
                        format!(
                            "{} has {}, current {}",
                            link.kind, link.recorded_hash, link.current_hash
                        )
                    })
                    .collect();
                write!(f, "{}: Stale ({})", self.id, parts.join("; "))
            }
            other => write!(f, "{}: {other}", self.id),
        }
    }
}

impl Serialize for RequirementCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut out = serializer.serialize_struct("RequirementCheck", 5)?;
        out.serialize_field("id", &self.id)?;
        out.serialize_field("state", self.state.name())?;
        out.serialize_field("missing_links", &self.gap.missing_links)?;
        out.serialize_field("stale_links", &self.gap.stale_links)?;
        out.serialize_field("drifted_files", &self.gap.drifted_files)?;
        out.end()
    }
}

/// Computes `req`'s [`RequirementCheck`]: its [`TraceState`] (via
/// [`trace_state_with_drift`]) plus the [`TraceGap`] detail explaining the
/// gap — which Trace Link kinds are missing (Incomplete), which are stale
/// together with their recorded and current hashes (Stale), or which files
/// have drifted (Drifted).
// @implements REQ-036 v2 4cbbd466
// @implements REQ-037 v2 2371e246
pub fn check_requirement(
    req: &Requirement,
    annotations: &[Annotation],
    drifted: Vec<String>,
) -> RequirementCheck {
    let find = |kind: AnnotationKind| {
        annotations
            .iter()
            .find(|a| a.kind == kind && a.req_id == req.id)
    };

    let links = [
        (AnnotationKind::Addresses, find(AnnotationKind::Addresses)),
        (AnnotationKind::Implements, find(AnnotationKind::Implements)),
        (AnnotationKind::Verifies, find(AnnotationKind::Verifies)),
    ];

    let state = trace_state_with_drift(req, annotations, drifted);

    let gap = match &state {
        TraceState::Incomplete => TraceGap {
            missing_links: links
                .iter()
                .filter(|(_, a)| a.is_none())
                .map(|(kind, _)| kind.as_str().to_string())
                .collect(),
            ..TraceGap::default()
        },
        TraceState::Stale => TraceGap {
            stale_links: links
                .iter()
                .filter_map(|(kind, a)| {
                    let a = (*a)?;
                    if a.hash != req.hash {
                        Some(StaleLink {
                            kind: kind.as_str().to_string(),
                            recorded_hash: a.hash.clone(),
                            current_hash: req.hash.clone(),
                        })
                    } else {
                        None
                    }
                })
                .collect(),
            ..TraceGap::default()
        },
        TraceState::Drifted(files) => TraceGap {
            drifted_files: files.clone(),
            ..TraceGap::default()
        },
        _ => TraceGap::default(),
    };

    RequirementCheck {
        id: req.id.clone(),
        state,
        gap,
    }
}

/// The File Hash of `bytes`: its SHA-256 digest as a 64-character lowercase
/// hex string, stored in `weft.lock` at Seal time.
// @implements REQ-031 v2 6cdbe6cb
pub fn file_hash(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Parses `weft.lock`'s flat TOML body into a file path -> File Hash map.
/// Returns an empty map if `toml_src` is empty or malformed (e.g. the lock
/// file does not exist yet).
// @implements REQ-031 v2 6cdbe6cb
pub fn parse_lock(toml_src: &str) -> BTreeMap<String, String> {
    toml::from_str(toml_src).unwrap_or_default()
}

/// Renders a file path -> File Hash map as `weft.lock`'s flat TOML body,
/// sorted by path for a stable diff.
// @implements REQ-031 v2 6cdbe6cb
pub fn render_lock(entries: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    for (path, hash) in entries {
        out.push_str(&format!("\"{path}\" = \"{hash}\"\n"));
    }
    out
}

/// All distinct file paths in `file_annotations` carrying at least one Trace
/// Link for `req_id`, sorted.
// @implements REQ-032 v2 a2441bcc
// @implements REQ-033 v2 04d42b48
pub fn files_for_requirement(req_id: &str, file_annotations: &[(String, Vec<Annotation>)]) -> Vec<String> {
    let mut paths: Vec<String> = file_annotations
        .iter()
        .filter(|(_, annotations)| annotations.iter().any(|a| a.req_id == req_id))
        .map(|(path, _)| path.clone())
        .collect();
    paths.sort();
    paths.dedup();
    paths
}

/// All distinct file paths in `file_annotations` carrying at least one Trace
/// Link (for any requirement), sorted.
// @implements REQ-031 v2 6cdbe6cb
// @implements REQ-032 v2 a2441bcc
pub fn all_annotated_files(file_annotations: &[(String, Vec<Annotation>)]) -> Vec<String> {
    let mut paths: Vec<String> = file_annotations
        .iter()
        .filter(|(_, annotations)| !annotations.is_empty())
        .map(|(path, _)| path.clone())
        .collect();
    paths.sort();
    paths.dedup();
    paths
}

/// The subset of `paths` whose current File Hash (in `current_hashes`) is
/// missing from `lock` or differs from the stored File Hash, sorted.
// @implements REQ-033 v2 04d42b48
pub fn drifted_paths(
    paths: &[String],
    lock: &BTreeMap<String, String>,
    current_hashes: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut drifted: Vec<String> = paths
        .iter()
        .filter(|path| match (current_hashes.get(*path), lock.get(*path)) {
            (Some(current), Some(sealed)) => current != sealed,
            _ => true,
        })
        .cloned()
        .collect();
    drifted.sort();
    drifted
}

/// Renders a human-readable Markdown view of a set of requirements.
///
/// The output is non-authoritative — the TOML records under `docs/prds/` are
/// the source of truth. See ADR 0001.
pub fn render_markdown(requirements: &[Requirement]) -> String {
    let mut out = String::from("# Requirements\n");

    for req in requirements {
        out.push('\n');
        if let Some(feat) = &req.feat {
            out.push_str(&format!("## {} (v{}) [{}]\n\n", req.id, req.version, feat));
        } else {
            out.push_str(&format!("## {} (v{})\n\n", req.id, req.version));
        }
        out.push_str(&req.statement);
        out.push_str("\n\n**Acceptance:**\n\n");
        for item in &req.acceptance {
            out.push_str(&format!("- {item}\n"));
        }
    }

    out
}

/// Validates a requirement record's format and integrity.
///
/// `filename_id` is the record's id as derived from its filename (the
/// filename without extension), used to check `id` == filename.
pub fn verify_requirement(req: &Requirement, filename_id: &str) -> Vec<VerifyIssue> {
    let mut issues = Vec::new();

    if !is_valid_id_format(&req.id) {
        issues.push(VerifyIssue::InvalidIdFormat(req.id.clone()));
    }

    if req.id == EXAMPLE_REQ_ID {
        issues.push(VerifyIssue::ReservedExampleId);
    }

    if req.id != filename_id {
        issues.push(VerifyIssue::IdFilenameMismatch {
            id: req.id.clone(),
            filename: filename_id.to_string(),
        });
    }

    if req.acceptance.is_empty() {
        issues.push(VerifyIssue::EmptyAcceptance);
    }

    let derived = canonical_hash(&req.statement, &req.acceptance);
    if derived != req.hash {
        issues.push(VerifyIssue::HashMismatch {
            id: req.id.clone(),
            stored: req.hash.clone(),
            derived,
        });
    }

    issues
}

/// The `[test]` section of `weft.toml`: a default Test Command plus optional
/// per-FEAT or per-requirement overrides, keyed by FEAT label or REQ_ID.
// @implements REQ-042 v2 37857355
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct TestConfig {
    pub command: Option<String>,
    #[serde(default)]
    pub overrides: BTreeMap<String, String>,
}

/// The top-level shape of `weft.toml` — currently only its `[test]` section
/// is meaningful to weft.
#[derive(Debug, Default, Deserialize)]
struct WeftToml {
    test: Option<TestConfig>,
}

/// Parses the `[test]` section of `weft.toml`'s TOML source — the project's
/// configured Test Command. Returns `None` if `toml_src` has no `[test]`
/// section (or is not valid TOML): callers must then report that no Test
/// Command is configured rather than treating requirements as passed.
// @implements REQ-042 v2 37857355
pub fn parse_test_config(toml_src: &str) -> Option<TestConfig> {
    toml::from_str::<WeftToml>(toml_src).ok()?.test
}

/// Resolves the Test Command responsible for `req`: a per-requirement
/// override (`config.overrides[req.id]`) takes precedence over a per-FEAT
/// override (`config.overrides[req.feat]`), which takes precedence over the
/// default `config.command`. `None` if no override applies and no default
/// command is configured.
// @implements REQ-042 v2 37857355
pub fn resolve_test_command<'a>(config: &'a TestConfig, req: &Requirement) -> Option<&'a str> {
    if let Some(cmd) = config.overrides.get(&req.id) {
        return Some(cmd.as_str());
    }
    if let Some(feat) = &req.feat {
        if let Some(cmd) = config.overrides.get(feat) {
            return Some(cmd.as_str());
        }
    }
    config.command.as_deref()
}

/// The outcome of the Test Command responsible for a requirement at a
/// Verification Run, recorded in `docs/prds/weft.run.toml` by `weft test`.
// @implements REQ-043 v3 ceb81bfe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestResult {
    Passed,
    Failed,
    Unrun,
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestResult::Passed => write!(f, "passed"),
            TestResult::Failed => write!(f, "failed"),
            TestResult::Unrun => write!(f, "unrun"),
        }
    }
}

/// A single requirement's Run Lock entry: the [`TestResult`] of its last
/// Verification Run, pinned to the requirement's Content Hash
/// (`content_hash`) and the SHA-256 File Hashes of its annotated files
/// (`file_hashes`) at run time. A later change to either pinned value
/// invalidates the recorded pass.
// @implements REQ-043 v3 ceb81bfe
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunRecord {
    pub result: TestResult,
    pub content_hash: String,
    #[serde(default)]
    pub file_hashes: BTreeMap<String, String>,
}

/// Parses the Run Lock's TOML body into a REQ_ID -> [`RunRecord`] map.
/// Returns an empty map if `toml_src` is empty or malformed (e.g. the Run
/// Lock does not exist yet).
// @implements REQ-043 v3 ceb81bfe
pub fn parse_run_lock(toml_src: &str) -> BTreeMap<String, RunRecord> {
    toml::from_str(toml_src).unwrap_or_default()
}

/// Renders a REQ_ID -> [`RunRecord`] map as the Run Lock's TOML body, sorted
/// by REQ_ID for a stable diff.
// @implements REQ-043 v3 ceb81bfe
pub fn render_run_lock(entries: &BTreeMap<String, RunRecord>) -> String {
    toml::to_string_pretty(entries).unwrap_or_default()
}

/// Top-level TOML keys that mark a record as a User Story rather than a
/// Requirement. User Stories are ephemeral and must never be persisted.
const USER_STORY_FIELDS: &[&str] = &["as_a", "i_want", "so_that", "user_story"];

/// Checks `toml_src` for a top-level User Story field (see
/// [`USER_STORY_FIELDS`]). Requirement records carry only `id`, `version`,
/// `feat`, `hash`, `status`, `statement`, `acceptance`, `rationale`, and
/// `notes` — a User Story field at the top level means the file persists a
/// User Story, which `docs/prds/` must never contain.
// @implements REQ-030 v2 bf5f866e
pub fn verify_not_user_story(toml_src: &str) -> Option<VerifyIssue> {
    let table = toml_src.parse::<toml::Table>().ok()?;
    USER_STORY_FIELDS
        .iter()
        .find(|&&field| table.contains_key(field))
        .map(|&field| VerifyIssue::UserStoryRecord(field.to_string()))
}
