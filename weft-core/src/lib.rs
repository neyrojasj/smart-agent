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
