+++
addresses = ["REQ-040 v2 1ead8691", "REQ-041 v2 7194e93b"]
+++

# 0011 — `check --summary` rollup and dangling annotation detection

`weft check` gains two additions, both built on the same active-requirement
and project-wide annotation scans it already performs. A `--summary` flag
replaces the per-requirement `REQ-NNN: <State>` listing with a rollup count of
active requirements in each Trace State, plus a `<traced>/<total> Traced`
line. Independently, `weft check` now scans every Trace Link
(`@addresses`/`@implements`/`@verifies`) found in the project and reports any
whose `req_id` does not match an active requirement — a **dangling**
annotation — with its file path and line number. Dangling annotations cause
`weft check` to exit non-zero, same as any non-`Traced` requirement.

## Why

As the PRD grows past a few dozen requirements, the per-requirement listing
becomes too long for an AI agent to scan for the handful that need attention —
`--summary` gives a one-glance rollup (`34/35 Traced`) for "is this PRD
healthy overall?" before drilling into specifics.

Dangling annotations are the inverse failure mode from drift: a Trace Link
that once pointed at a real requirement but now points at nothing, because the
requirement was renamed, the annotation was copy-pasted with the wrong
REQ_ID, or the requirement was deprecated without updating its links. These
are silent today — `weft check` only walks forward from requirements to links,
never backward from links to requirements — so a dangling link can sit in the
codebase indefinitely, misleading anyone who reads it.

## Considered Options

**Rollup counts per `TraceState` variant, plus a `<traced>/<total> Traced`
summary line (chosen).** Reuses the `TraceState` enum already computed for the
per-requirement listing; no new scan. The `<traced>/<total> Traced` line gives
a single pass/fail-shaped number, while the per-state breakdown shows where
the remaining work is.

**Percentage-only summary (e.g. `97% Traced`).** Loses the absolute counts,
which matter more than the ratio when deciding how much work remains
(`34/35` vs `3400/3500` are very different amounts of work despite both being
~97%). Rejected.

**Dangling detection as a separate subcommand.** Rejected: it is the natural
inverse of the existing per-requirement scan in `check`, uses the same
project-wide annotation walk, and belongs in the same non-zero-on-problems
gate rather than a command an agent must remember to run separately.

**Dangling = unknown REQ_ID only (ignore deprecated).** A link to a
deprecated requirement is just as broken as a link to a REQ_ID that never
existed — both mean "this code/test/design no longer has a live requirement
behind it" and need the same remediation (update or remove the link).
Treating only unknown ids as dangling would miss the more common case
(a requirement was deprecated but its links were never cleaned up). Rejected.

## Consequences

- `weft check --summary` prints one line per `TraceState` variant
  (`Orphaned`/`Incomplete`/`Stale`/`Drifted`/`Traced`) followed by
  `<traced>/<total> Traced`, instead of the per-requirement listing. The
  non-zero-on-any-non-`Traced` exit code is unchanged.
- Dangling annotations are reported regardless of `--summary`, as
  `<path>:<line>: dangling <marker> <req_id>`.
- A project whose source tree contains example Trace Link syntax in prose
  (matching the `@implements REQ-NNN vN <hash>` shape but not a real Trace
  Link) will have those examples reported as dangling. This is the same
  category of false positive `.weftignore` (ADR 0010) exists to manage.
