+++
addresses = []
+++

# 0009 — File-hash drift detection via weft.lock and Seal

Each annotated file (design doc, implementation, test) is hashed at Seal time
and the digest stored in `docs/prds/weft.lock`. When `weft check` finds a file
whose current SHA-256 differs from its stored **File Hash**, it reports the
requirement as **Drifted** — a new Trace State distinct from Stale.

## Why

The existing Content Hash mechanism detects drift in one direction: requirement
changes that annotations haven't acknowledged. The inverse was unguarded: a
file carrying `@implements REQ-042` could be silently refactored or emptied
while `weft check` continued to report `Traced`. In an AI-assisted development
workflow this is dangerous — the AI treats Trace State as a completeness signal
and acts on it. A false `Traced` sends the AI forward on broken ground.

## Considered Options

**Whole-file SHA-256 in a flat lock file (chosen).** Language-agnostic and
simple. Unrelated edits (formatting, imports) produce false alarms, but the
remediation cost is low — run `weft seal` after reviewing — and false negatives
(missed real drift) are strictly worse than false positives.

**Marked region hashing.** Hashes only the delimited block around the
annotation. Less noise, but requires new annotation delimiters and discipline
to maintain boundaries. Complexity cost exceeds noise reduction at current
scale; can be introduced later if noise proves unbearable.

**File hash embedded in the requirement TOML.** Couples requirement definitions
to filesystem paths. Every rename or refactor breaks the TOML record. Rejected.

**Reuse `Stale` state.** `Stale` and `Drifted` have different causes and
different remediations (`weft bump` vs `weft seal`). Conflating them makes the
output ambiguous and the remediation path unclear. Rejected.

## Consequences

- `docs/prds/weft.lock` is a flat TOML file, keyed by file path:
  `"src/login.rs" = "<sha256>"`. One entry per annotated file, regardless of
  how many requirements that file covers.
- `weft check` names the drifted files in its output:
  `REQ-042: Drifted (src/login.rs, tests/auth_test.rs)`.
- `Drifted` is only reported when all Trace Links are already Current. A
  requirement that is `Stale` reports `Stale` — fix requirement drift first.
- `weft seal` (all) and `weft seal REQ-NNN` (targeted) are the only commands
  that write `weft.lock`. Targeted seal updates the File Hash for every file
  annotated with that REQ_ID.
- First-time setup: teams run `weft seal` once after the feature ships. Until
  then, all requirements with annotated files report `Drifted`.
- `weft.lock` must be committed. A missing or stale lock file in CI is a build
  failure.
