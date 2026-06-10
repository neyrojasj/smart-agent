//! weft-core: requirement record parsing, canonical hashing, and verification.

use std::fmt;

use serde::Deserialize;
use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;

/// The lifecycle state of a requirement record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    Deprecated,
}

/// A single requirement record, parsed from a `docs/prds/**/*.toml` file.
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
        }
    }
}

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
/// line, in any comment syntax: `@implements REQ-042 v3 a3f9b2c1`.
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
    out
}

/// Extracts `@addresses` Trace Links from a `+++`-delimited TOML frontmatter
/// block at the start of `text` (DEC/ADR docs). Returns an empty vec if
/// `text` has no frontmatter, the frontmatter is not valid TOML, or it has no
/// `addresses` array.
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

/// Parses an `addresses` entry of the form `REQ-042 v3 a3f9b2c1` (no
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

/// Parses `@implements REQ-042 v3 a3f9b2c1` (or `@verifies ...`) starting at
/// the marker itself.
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

/// The static verdict for a requirement: do its Trace Links exist
/// (completeness), and do their frozen hashes match the requirement's
/// current Content Hash (freshness)?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceState {
    /// No Trace Links at all.
    Orphaned,
    /// At least one Trace Link is missing (design, code, or test).
    Incomplete,
    /// All three Trace Links are present, but at least one pins a hash that
    /// no longer matches the requirement's current Content Hash.
    Stale,
    /// All three Trace Links are present and current.
    Traced,
}

impl fmt::Display for TraceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TraceState::Orphaned => "Orphaned",
            TraceState::Incomplete => "Incomplete",
            TraceState::Stale => "Stale",
            TraceState::Traced => "Traced",
        };
        write!(f, "{s}")
    }
}

/// Computes `req`'s [`TraceState`] from the Trace Links found by
/// [`scan_annotations`] across the project (annotations for other
/// requirements are ignored).
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

/// Renders a human-readable Markdown view of a set of requirements.
///
/// The output is non-authoritative — the TOML records under `docs/prds/` are
/// the source of truth. See ADR 0001.
// @implements REQ-020 v1 placeholder
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
