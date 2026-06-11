+++
addresses = [
    "REQ-022 v2 b34eb010",
    "REQ-023 v2 4fa1a337",
    "REQ-024 v2 00a713df",
    "REQ-025 v2 6ff51574",
    "REQ-026 v2 f2ba6521",
    "REQ-027 v2 af09e7de",
    "REQ-028 v2 46490789",
]
+++

# 0007 — Agent skills bookend the toolchain as traceable documents

`to-smart-prd` and `to-smart-issues` (`.github/skills/`) are the human-facing
ends of the `weft` workflow: one turns a grilling session into requirement
records, the other turns `weft check`'s gaps into a vertical-slice
implementation plan. Both are Markdown prompt documents, not Rust code — but
ADR 0005 established that Trace Annotations are language-agnostic, so a
SKILL.md is just another file `weft check` can scan.

## Considered Options

- **Skills as Trace Annotation carriers (chosen).** Each SKILL.md gets inline
  `<!-- @implements REQ-NNN vN <hash> -->` markers at the workflow step that
  satisfies the requirement, exactly as a `.rs` file would. A companion Rust
  integration test (`weft/tests/skills.rs`) reads the real SKILL.md and
  asserts both the annotation and the documented behaviour are present,
  carrying `@verifies`. This keeps the skills inside the same trace chain as
  the CLI they drive — no special-casing for "documentation requirements".
- **Separate non-code tracking for skill requirements.** Rejected — it would
  introduce a second, weaker notion of "done" for REQ-022..028, contradicting
  ADR 0003's full-chain enforcement.

## Consequences

- `to-smart-prd`'s workflow steps for upserting (REQ-022), bumping only on
  normative-text change (REQ-023), and deprecating requirements absent from
  the session's intent (REQ-024) each carry an `@implements` marker.
- `to-smart-issues`'s workflow steps for discovering not-yet-Traced
  requirements (REQ-025), embedding `REQ-NNN vN <hash>` per requirement in
  generated issues (REQ-026), distinguishing `rework` (Stale) from
  `implement` (Orphaned/Incomplete) slices (REQ-027), and defining a finished
  slice as one where `weft check` reports `Traced` for every requirement in
  it (REQ-028) each carry an `@implements` marker.
- `weft/tests/skills.rs` reads `.github/skills/*/SKILL.md` directly (no
  fixtures) so the tests fail the moment the real skill document drifts from
  its acceptance criteria or loses an annotation.
- Per ADR 1, the requirement records under `docs/prds/FEAT-Skills/` remain the
  source of truth for REQ-022..028; this ADR and the SKILL.md/test annotations
  are downstream Trace Links pinned to their current hashes.
