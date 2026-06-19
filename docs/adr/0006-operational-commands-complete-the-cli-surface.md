+++
addresses = [
    "REQ-012 v2 8afcf842",
    "REQ-013 v5 b7c46a27",
    "REQ-014 v2 d217a603",
    "REQ-015 v2 3d05542c",
    "REQ-048 v2 94fdea44",
    "REQ-049 v2 a5273df5",
    "REQ-050 v2 b7e04277",
    "REQ-051 v2 16a337fc",
]
+++

# 0006 — Operational commands complete the CLI surface

`weft render`, `init`, `check`, and `deprecate` round out the CLI alongside
the core manipulation toolbox (ADR 0004) and the Trace State engine (ADR
0005). Together they turn `weft` from a library of building blocks into a
tool a project can adopt and run in CI.

## Considered Options

- **Four focused commands (chosen).** `render` is a read-only projection of
  `docs/prds/` to Markdown (ADR 0001 — never a second source of truth);
  `init` scaffolds the directories a new project needs; `check` is the CI
  gate that reports `TraceState` (ADR 0005) per active requirement and exits
  non-zero on any drift; `deprecate` is the one mutation that removes a
  requirement from the working set without deleting its record (ADR 0001).
- **Folding `check` into `verify`.** Rejected — `verify` is a per-record
  format/hash integrity check (ADR 0002) with no notion of Trace Links;
  conflating the two would make `verify`'s exit code depend on annotations
  scattered across the whole tree instead of just the record at hand.

## Consequences

- `weft render` only reads `docs/prds/`; it never rewrites a TOML record, so
  it can be run freely without risk of clobbering the source of truth.
- `weft init` uses `create_dir_all`, which is a no-op on directories that
  already exist — running it again on an initialized project is safe and
  leaves existing records untouched.
- `weft check` excludes `status = "deprecated"` requirements from the drift
  gate entirely (ADR 0001): a deprecated requirement with no Trace Links does
  not fail CI.
- `weft deprecate REQ-NNN` rewrites only the `status` line in place, is
  idempotent, and fails for an unknown id — `weft list` and `weft check` then
  treat the record as inactive while `weft verify` continues to pass on it.
