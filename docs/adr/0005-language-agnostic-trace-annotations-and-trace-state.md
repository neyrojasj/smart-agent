+++
addresses = [
    "REQ-016 v2 84ac8548",
    "REQ-017 v2 8af530a5",
    "REQ-018 v2 e2253535",
    "REQ-019 v2 ed4d3199",
    "REQ-020 v2 9abea869",
    "REQ-021 v2 58781e5c",
]
+++

# 0005 — Language-agnostic trace annotations and the Trace State engine

A Trace Link (ADR 0001, ADR 0002) is recorded as one of three Trace
Annotations, one per chain stop:

- **`@addresses`** — a structured array entry in a design decision's `+++`
  TOML frontmatter (a DEC/ADR doc), of the form `"REQ-NNN vN <hash>"`.
- **`@implements`** — an inline marker in source code, of the form
  `@implements REQ-NNN vN <hash>`.
- **`@verifies`** — an inline marker in a test file, of the form
  `@verifies REQ-NNN vN <hash>`.

`scan_annotations` finds every Trace Annotation in a file by line/marker
matching: it looks for the literal substrings `@implements` and `@verifies`
on each line and tokenizes from that point, and separately parses the
`addresses` array out of a leading `+++` frontmatter block. No per-language
parser is involved — the same scan works whether the marker sits in a Rust
`//` comment, a Python `#` comment, an HTML `<!-- -->` comment, or any other
text-based syntax.

`trace_state` then combines the annotations found for a requirement with that
requirement's current `hash` to produce a [`TraceState`]: `Orphaned` (no
links), `Incomplete` (1-2 of the three links present), `Stale` (all three
present but at least one pins a hash that no longer matches the requirement's
current hash), or `Traced` (all three present and all pin the current hash).

## Considered Options

- **Line/marker scanning, language-agnostic (chosen).** A single scanner
  handles every language a Trace Annotation might appear in, per ADR 0003 —
  introducing a new language requires no new parser.
- **Per-language AST parsing.** Would let annotations live in structured
  metadata (e.g. doc-comment attributes), but requires a parser per language
  and breaks the "any text-based language" guarantee.

## Consequences

- A Trace Annotation is just a line of text containing a recognizable marker
  followed by `REQ-NNN vN <hash>` tokens — easy to write by hand and easy to
  copy-paste from a requirement record's table.
- `Traced` requires full-chain presence *and* freshness (ADR 0003): a
  requirement with all three links present but one stale annotation is
  `Stale`, not `Traced`, until the annotation is updated to the requirement's
  current hash.
