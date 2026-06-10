# 0001 — Pivot to a Rust traceability engine with machine-first requirement records

The repository pivots from a Python skills-installer into a Rust tool whose
purpose is end-to-end requirements traceability: an unbroken trace from a
requirement, through a design decision and code, to a test. The **source of
truth is a set of machine-first TOML records** — one requirement per file under
`docs/prds/`, foldered per PRD/feature — not a prose PRD on the issue tracker.
Any Markdown PRD is a generated, non-authoritative view.

## Considered Options

- **Machine-first TOML records (chosen).** A requirement is a structured record
  (`id`, `version`, `feat`, `hash`, `statement`, `acceptance`, commentary). The
  tool can retrieve any field in O(1) (`get REQ-042 --field statement`), compute
  hashes deterministically, and gate CI. Cost: the PRD stops being a document
  humans author directly in prose; they author records (or generate them from a
  grilling session via `to-smart-prd`).
- **Narrative Markdown PRD as source.** Readable, but merge-conflict-prone,
  ambiguous to parse, and impossible to hash deterministically.
- **PRD on the issue tracker (the old `to-prd` model).** Not version-controlled
  alongside code, not parseable, not a durable long-run artifact.

## Consequences

- One requirement per file gives clean per-requirement git history — superseded
  versions live in git, not as retained queryable entities.
- The old Python `smart` installer and its `save`/`sync` personal-branch feature
  are dropped; they are unrelated to traceability.
