# 0002 — Drift detection via a content hash frozen into trace annotations

Each requirement carries a `hash` over its **normative region** (`statement` +
ordered `acceptance`). Every trace annotation freezes the hash it was written
against — `@implements REQ-042 v3 a3f9b2`. Drift is then mechanical: the tool
compares the frozen hash to the requirement's current hash. The integer
`version` is only a human-facing label; the **hash is the enforcement
mechanism**.

## Why

The headline feature — "when a requirement changes, tell us if it was already
implemented" — is only trustworthy if the tool can detect a content change
*itself*. Two comparisons fall out of the stored hash:

1. **Version honesty:** stored `hash` vs. freshly-derived hash of the normative
   text → catches an edit where the author changed the text but forgot to
   `bump`.
2. **Link freshness:** an annotation's frozen hash vs. the record's current hash
   → marks that link `Stale`.

## Considered Options

- **Hash frozen in the annotation (chosen).** Catches even silent same-version
  edits. Cost: the hash string lives in the codebase and must be updated when a
  requirement legitimately changes — intentional friction, since that update
  *is* the signal.
- **Author-bumped version only.** Simpler, but a forgotten bump goes undetected;
  traceability you can't trust is worse than none.
- **Content hash only, no human version.** Honest but unreadable — humans and
  PRDs need a stable, citable label.

## Consequences

- The hash is short (8 hex of SHA-256 over an NFC-normalized canonical
  serialization) so it sits comfortably inside a code comment.
- Commentary (`rationale`, `notes`) is outside the hash and can be edited freely
  without triggering drift.
