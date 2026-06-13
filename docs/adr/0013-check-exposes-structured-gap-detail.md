+++
addresses = ["REQ-036 v2 4cbbd466", "REQ-037 v2 2371e246"]
+++

# 0013 — `check` exposes structured gap detail (human + `--json`)

`trace_state`/`trace_state_with_drift` already determine, via the
`present`/`links` arrays, exactly which Trace Link kinds are missing
(Incomplete) and which are present-but-stale (Stale, with both the recorded
and current hash). A new `check_requirement` function computes a
`RequirementCheck { id, state, gap }` that carries this detail as a
`TraceGap { missing_links, stale_links, drifted_files }`. `weft check`'s
default human-readable output renders `RequirementCheck` via `Display`,
printing the gap inline next to the state word; a new `weft check --json`
flag renders the same `RequirementCheck` values via `Serialize` as a JSON
array, one object per active requirement. Both renderers consume the same
`Vec<RequirementCheck>`, so the human and JSON views can never diverge.

## Why

Today `weft check` collapses all of this detail to a single state word
(`Incomplete`, `Stale`, ...), so an AI agent re-driving requirement-driven
development must re-scan the repository to discover what is actually missing
or stale. The classification logic already computes this detail internally;
it was simply being discarded before printing. Surfacing it — and giving
agents and CI tooling a stable JSON schema instead of parsed English sentences
— removes that re-scan and the brittleness of text parsing.

## Considered Options

**One `RequirementCheck` struct shared by both renderers (chosen).** `Display`
formats it as the human-readable line (`REQ-NNN: Incomplete (missing
verifies)`, `REQ-NNN: Stale (implements has <recorded>, current <current>)`);
`Serialize` formats it as `{id, state, missing_links, stale_links,
drifted_files}`. A single computation feeds both, so the two views are
guaranteed consistent by construction.

**Two separate code paths — one that prints text, one that builds JSON.**
Rejected: duplicating the missing/stale-link computation risks the two views
drifting apart as `TraceState`'s classification logic evolves.

**`state` as the full `TraceState` enum in JSON (including `Drifted`'s file
list).** Rejected: `Drifted(Vec<String>)` would serialize awkwardly and
duplicate `drifted_files`. `RequirementCheck::Serialize` instead emits
`state` as the bare state name (`"Drifted"`) via `TraceState::name()`, with
the file list only in `drifted_files`.

## Consequences

- `weft check`'s per-requirement line now includes gap detail for
  `Incomplete` (`missing <kind>[, <kind>]`) and `Stale` (`<kind> has
  <recorded_hash>, current <current_hash>`, joined with `; ` for multiple
  stale links). `Orphaned`, `Traced`, and `Drifted` are unchanged.
- `weft check --json` emits a single JSON array, one object per active
  requirement ordered by id, with `missing_links`/`stale_links`/
  `drifted_files` all empty for `Orphaned` and `Traced`.
- Exit code semantics (REQ-014: non-zero if any requirement is not `Traced`)
  are unchanged for both modes. In `--json` mode, dangling-annotation lines
  are suppressed from stdout (to keep stdout a single JSON array) but still
  contribute to the non-zero exit code.
