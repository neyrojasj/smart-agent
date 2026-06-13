+++
addresses = ["REQ-047 v2 c4c2f006"]
+++

# 0015 — A reserved example REQ_ID for illustrative annotations

The annotation scanner (`scan_annotations`) matches Trace Link markers by a
naive line/text scan across every non-excluded file (REQ-019, ADR 0003). That
makes any file which *documents* the annotation syntax — doc-comments in
`lib.rs`, prose examples in ADRs — a hazard: its examples are mis-detected as
real Trace Links.

We reserve a single **example REQ_ID, `REQ-000`**. Any `@addresses`,
`@implements`, or `@verifies` annotation citing it is dropped at scan time:
it contributes no Trace Link and is never reported as a dangling annotation.
`REQ-000` is for illustration only and can never be a real requirement.

## Why

- **It actually bit us.** `lib.rs` and ADR 0002 used `REQ-042` in syntax
  examples. When `REQ-042` was later allocated as a real requirement, those
  examples became false `@implements` links that silently advanced its
  completeness. The interim workaround — re-pointing examples at a made-up
  `REQ-900` — only converted the false link into permanent
  "dangling `@implements REQ-900`" noise in every `weft check` (REQ-041).
- **Today's only escape is accidental.** Examples that write
  `@implements REQ-NNN vN <hash>` are safe purely because the version token
  `vN` fails `strip_prefix('v').parse::<u32>()` and the line is dropped. That
  is fragile, undocumented, and forces examples to use a fake version — so you
  cannot teach the real `v3 a3f9b2c1` shape without creating a link. A reserved
  id makes the escape **explicit and id-based**, decoupled from the
  version/hash tokens.
- **The sentinel is nearly free.** `next_req_id` is `max(existing)+1` with
  `unwrap_or(0)`, so allocation starts at `REQ-001` — `weft new` can *never*
  mint `REQ-000`. Reserving it costs zero real id-space and closes the exact
  REQ-042 failure mode: the sentinel can never later become real. `REQ-000` is
  also format-valid (`REQ-` + three digits), so example annotations stay
  copy-paste-realistic in shape.
- **One mechanism, both symptoms.** Filtering the sentinel at scan time means
  example annotations never become a Trace Link *and* never dangle. Migrating
  the repo's own examples from `REQ-900` to `REQ-000` therefore strictly
  *removes* `weft check` noise rather than adding a marker.

## Considered Options

**Reserved example id `REQ-000`, dropped at scan time (chosen).** Smallest
change consistent with the naive line scan: one `retain` in `scan_annotations`
on an id-equality check, plus a `verify` guard so a record can never claim the
reserved id. No new token vocabulary, no per-file state, no parsing. Cost:
examples must use `REQ-000` rather than an arbitrary realistic id — which is a
feature, since it keeps examples visibly non-real.

**Inline `weft:example` escape marker.** A line containing the token is
skipped. Also language-agnostic and parse-free, and strictly more general (it
can exempt a line citing any id, including a real-numbered one). Rejected *for
now*: it introduces new vocabulary the sentinel makes unnecessary for the
motivating cases, and every example line must carry the marker. Kept on the
shelf as a future complement if marking a real-numbered id as an example is
ever genuinely needed.

**Skip markers inside backtick code spans or `///` doc-comment contexts.**
Rejected: it violates the no-AST, language-agnostic constraint (ADR 0003 /
REQ-019). Real annotations *live* in `//`/`#`/`<!-- -->` comments, so "skip
comment contexts" is incoherent, and code-span detection needs Markdown
awareness.

**Per-file exclude via `.weftignore` (ADR 0010).** Already exists, but
unworkable here: `lib.rs` and ADR 0002 carry *both* real Trace Links and
examples in the same file, so excluding the whole file would drop real links.
`.weftignore` stays the right tool for wholly non-source files (`.scratch`,
`logs`); this is its complement for *mixed* files.

**A reserved id range (e.g. `REQ-000`–`REQ-009`) or the high placeholders
(`REQ-9xx`).** Rejected: a single id suffices. Reserving the `REQ-9xx`
placeholders used by test fixtures would also wrongly silence `REQ-999`, which
those tests cite *specifically to assert that dangling detection fires* — that
behavior must be preserved (REQ-041).

## Consequences

- `scan_annotations` drops every annotation whose `req_id` is `REQ-000`, across
  all three kinds (the inline `@implements`/`@verifies` paths and the
  `@addresses` frontmatter path).
- `weft verify` rejects a requirement record whose `id` is `REQ-000`, so the
  reserved id can never silently become a real, unscannable requirement.
- The repo's own syntax examples move from `REQ-900` to `REQ-000`; the
  long-standing `dangling REQ-900` noise in `weft check` disappears.
- `REQ-000` is the documented, idiomatic placeholder for *all* future
  annotation examples in code comments and design docs.
