+++
addresses = ["REQ-038 v2 82357796", "REQ-039 v3 2900b820"]
+++

# 0012 — `trace` and `annotate` reverse-lookup commands

Two new subcommands close the loop from requirement back to artifact, both
built on the existing annotation scanner (`scan_file_annotations`) and the
`Requirement` lookup already used by `weft get`/`weft bump`.

`weft trace REQ-NNN` prints, for each Trace Link found for that requirement,
its kind (`@addresses`/`@implements`/`@verifies`) and the file path + line
number where the annotation occurs, one per line. If the requirement has no
links it prints `Orphaned` and lists nothing. An unknown REQ_ID exits non-zero
with an error, matching `weft get`'s error handling.

`weft annotate REQ-NNN --kind <addresses|implements|verifies>` prints the
exact, ready-to-paste Trace Link line for that kind, using the requirement's
*current* version and hash — so the output stays correct across `weft bump`.
For `implements`/`verifies` this is `@implements REQ-NNN vN <hash>` (no
comment syntax, since that is language-dependent — ADR 0005); for `addresses`
it is the quoted `"REQ-NNN vN <hash>"` entry shape used in design-decision
frontmatter arrays. An unknown REQ_ID also exits non-zero with an error.

## Why

Today an agent that wants to view the implementation or test for a given
requirement must grep the repository for the REQ_ID by hand, repeating work
the annotation scanner already does internally. Hand-transcribing REQ_ID,
version, and hash into Trace Link annotations is also the most common source
of `Stale` results — a single mis-copied hash digit silently breaks
traceability.

## Considered Options

**Two focused subcommands, `trace` (read) and `annotate` (generate),
reusing the existing scanner and requirement lookup (chosen).** Each has a
single, deep responsibility and composes with existing plumbing
(`scan_file_annotations`, `annotation_line`, `find_toml_files` +
`load_requirement`).

**A single `weft links REQ-NNN` command covering both directions.** Rejected:
"where are this requirement's links" and "give me the line to paste" are
different operations with different outputs (a report vs. a single
ready-to-paste line); splitting them keeps each command's output predictable
for scripting.

## Consequences

- `weft trace REQ-NNN` output is `<kind> <path>:<line>` per link, or
  `Orphaned` if none exist.
- `weft annotate REQ-NNN --kind <kind>` output is a single line, exactly as it
  should appear in source (for `implements`/`verifies`) or in an `addresses`
  frontmatter array (for `addresses`).
- Both commands share `weft get`'s "requirement not found under docs/prds"
  error message and non-zero exit code for unknown REQ_IDs.
