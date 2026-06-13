# Requirements Traceability

The language of a Rust tool that verifies an unbroken trace from a requirement
(defined in the PRD, the source of truth) through design decisions and code to
passing tests — and reports when a requirement has changed but its
implementation has not kept up.

> **Running `weft`:** use the path `target/debug/weft` to invoke the command
> (e.g. `target/debug/weft check`).

## Language

**Requirement**:
A single, uniquely-identified statement of something the system must do,
authored in the PRD. The atomic unit of traceability.
_Avoid_: Spec, story, ticket, feature (a feature is a group of requirements).

**PRD**:
The source-of-truth set of all requirements. Stored machine-first as one
structured record per requirement (TOML, under `docs/prds/`); any Markdown PRD
is a generated, human-readable view, never the source. If code and the PRD disagree,
the PRD wins.
_Avoid_: Spec doc, design doc.

**Normative Region**:
The part of a requirement that defines its meaning — the `statement` plus the
ordered `acceptance` criteria. The only part covered by the Content Hash;
editing it marks downstream Trace Links Stale.
_Avoid_: Body, spec.

**Commentary Region**:
The non-normative parts of a requirement — rationale, notes, examples. Excluded
from the Content Hash; can be edited freely without triggering drift.
_Avoid_: Notes, description.

**REQ_ID**:
The stable identity of a requirement (e.g. `REQ-042`). Never changes for the
life of the requirement, even as its text evolves.
_Avoid_: Req number, ticket id.

**Version**:
The human-facing, citable label for a requirement's current revision (e.g.
`v3`). Bumped by the author when the requirement's meaning changes. It is a
label, not the enforcement mechanism — see Content Hash.
_Avoid_: Revision, rev.

**Content Hash**:
A tool-computed hash of a requirement's normalized text. The enforcement
mechanism that keeps Version honest: if the text changes, the hash changes, so
the tool can detect an edit even when the author forgot to bump the Version.
_Avoid_: Checksum, fingerprint, digest.

**User Story**:
An ephemeral, derived expansion of a requirement, generated on demand to guide
implementation. Never persisted to `docs/prds/` and never the source of truth —
the Requirement is. Regenerated from the current requirement each time, so it
cannot go stale.
_Avoid_: Requirement (a User Story is derived from one, not equal to it), spec.

**Deprecated**:
The lifecycle state of a requirement that has been removed from the PRD's
intent. It is marked `status = "deprecated"` rather than deleted, preserving its
REQ_ID and history. The tool never auto-deletes records.
_Avoid_: Removed, deleted, archived.

**Vertical Slice**:
A unit of implementation work spanning one or more *whole* requirements taken
end-to-end to the `Traced` state (design + code + test). Its "done" is
verifiable by definition: `check` goes green for its requirements. A slice never
sub-divides a requirement — if a requirement is too big to finish in one slice,
that is the signal to split the requirement, not the slice.
_Avoid_: Task, story, chunk.

**FEAT**:
An optional grouping label that buckets related requirements for organization
and roll-up reporting (e.g. `FEAT-Auth`). It is metadata on a requirement, not
part of the REQ_ID and not independently versioned or traceable. A requirement
belongs to at most one FEAT.
_Avoid_: Epic, module, component.

**Trace Link**:
A declaration that an artifact satisfies a requirement, pinned to the Version
and Content Hash frozen at link time. There is exactly one keyword per chain
stop: **`@addresses`** (design — a structured field in the DEC/ADR
frontmatter), **`@implements`** (code — inline annotation), **`@verifies`**
(test — inline annotation). The frozen hash is what makes Stale detection
possible, per stage.
_Avoid_: Reference, mapping (a Trace Link is never stored in a central map),
`@satisfies` (too generic — the chain stop must be distinguishable).

**Complete / Incomplete**:
A requirement is **Complete** only when all three Trace Links exist — an
`@addresses` (design), an `@implements` (code), and a `@verifies` (test).
Missing any one makes it **Incomplete**. Completeness is about link *presence*,
independent of freshness or test results.
_Avoid_: Done, finished.

**Trace State**:
The static verdict for a requirement, combining three axes the tool can check
without executing anything — completeness (are all three Trace Links present?),
freshness (does each link's frozen hash match the requirement's current Content
Hash?), and artifact integrity (do annotated files match their sealed File
Hashes in `weft.lock`?). Values: **Orphaned** (no links), **Incomplete** (a
link missing), **Stale** (a link pins an old hash), **Drifted** (Complete and
all links Current, but ≥1 annotated file has changed since last Seal),
**Traced** (Complete, all links Current, and all annotated files match their
sealed hashes), or **Verified** (Traced, and the most recent recorded
Verification Run passed at the current Content Hash and current File Hashes).
Test execution is opt-in via a configured Test Command; with none configured
the tool runs no tests and Verified is unreachable.
_Avoid_: Status, coverage, verified (the tool does not verify test results).

**Drifted**:
The Trace State reported when a requirement is Complete and all Trace Link
hashes are Current, but at least one annotated file has changed since it was
last Sealed. Signals that a human or AI must re-confirm the implementation
still satisfies the requirement before running `weft seal`.
_Avoid_: Stale (`Stale` means the *requirement* changed; `Drifted` means an
*artifact* changed).

**Seal**:
The act of recording the current File Hash of every annotated file into
`weft.lock`, performed after a human or AI confirms the current file contents
still satisfy their annotated requirements. Invoked via `weft seal` (all
requirements) or `weft seal REQ-NNN` (targeted).
_Avoid_: Bless, lock, approve.

**File Hash**:
The SHA-256 digest of an annotated file's full contents, stored in `weft.lock`
at Seal time. The enforcement mechanism for artifact-level drift detection:
when a file's current digest differs from its stored File Hash, the requirement
is reported Drifted.
_Avoid_: Checksum, fingerprint (use File Hash to distinguish from Content Hash,
which applies to requirement records only).

**Weft Lock**:
The committed artifact (`docs/prds/weft.lock`) that maps each annotated file
path to its File Hash at last Seal. Flat TOML, keyed by file path. Must be
committed alongside source changes; regenerated via `weft seal` after
reviewing artifact-level drift.
_Avoid_: Manifest, snapshot.

**Test Command**:
The project-declared command weft executes to determine whether a
requirement's tests pass, configured in a `[test]` section of `weft.toml` as
a default with optional per-FEAT or per-requirement overrides. weft runs it
as an opaque string and reads only its exit code, never parsing the
language's test framework — preserving language-agnosticism.
_Avoid_: Test runner (weft is not a runner), suite.

**Verification Run**:
The act of executing the Test Command and recording each requirement's
result, performed by `weft test`. The dynamic counterpart to Seal.
_Avoid_: Test run (too generic), CI run.

**Run Lock**:
The committed artifact (`docs/prds/weft.run.toml`) that records, per
requirement, the result of its last Verification Run (passed/failed) pinned
to the content hash and annotated-file hashes at run time. The dynamic
analogue of the Weft Lock; a recorded pass is invalidated automatically when
any pinned hash changes.
_Avoid_: Test cache, results file (it is committed and hash-pinned, not
ambient).

**Verified**:
The Trace State above Traced — a requirement that is Traced and whose most
recent recorded Verification Run passed at its current content hash and
current annotated-file hashes. Verified means the Test Command responsible
for the requirement passed, NOT that the requirement's specific test provably
executed, which weft cannot determine language-agnostically. Invalidated by
any edit that changes a pinned hash.
_Avoid_: Passing, green, Tested (Verified is hash-pinned and sits atop the
full static chain).

**Completion Gate**:
The project-level done check (`weft gate`) that exits zero only when every
active requirement is Verified — the autonomous agent's single termination
condition. Distinct from `weft check`, which is per-requirement and gates on
drift.
_Avoid_: Check (different exit contract).

**Work Driver**:
The `weft next` command that selects the single highest-priority
not-yet-Verified requirement (regressions-first) and emits it with an
explicit action verb, letting an agent advance the project one requirement at
a time.
_Avoid_: Queue, scheduler.
