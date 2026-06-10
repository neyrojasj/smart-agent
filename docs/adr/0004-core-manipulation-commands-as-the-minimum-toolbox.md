+++
addresses = [
    "REQ-007 v2 1d00916a",
    "REQ-008 v2 3da61ff3",
    "REQ-009 v2 c30912ae",
    "REQ-010 v2 59117494",
    "REQ-011 v2 5429311c",
]
+++

# 0004 — Core manipulation commands as the minimum toolbox

`weft new`, `get`, `list`, `bump`, and `verify` form the minimum viable
toolbox for working with requirement records. They are the exact set the
`to-smart-prd` skill depends on: a session needs to allocate an id (`new`),
read any field of an existing record (`get`), enumerate active requirements
(`list`), and atomically advance a record's version and hash together
(`bump`) — with `verify` as the integrity check run after each write.

## Considered Options

- **Five focused commands (chosen).** Each command does one thing on one
  record (or the whole tree, for `list`/`verify`), keeping the CLI surface
  small and each command's contract easy to state and test.
- **A single `weft edit` command with subflags.** Would reduce the number of
  subcommands, but conflates allocation, mutation, and read-only queries
  behind one verb, making the CLI harder to script against.

## Consequences

- `weft bump` increments `version` and recomputes `hash` as a single
  operation (ADR 0002) — there is no way to bump one without the other.
- `weft new` allocates ids by scanning `docs/prds/` for the current maximum
  and writing the new record with `create_new` (atomic file creation),
  retrying with the next id on collision — so concurrent invocations never
  produce two records with the same id.
- `weft list` only reports `status = "active"` requirements; deprecated
  records (ADR 0001) are excluded from the working list but remain on disk.
