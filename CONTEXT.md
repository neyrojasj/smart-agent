# Requirements Traceability

The language of a Rust tool that verifies an unbroken trace from a requirement
(defined in the PRD, the source of truth) through design decisions and code to
passing tests — and reports when a requirement has changed but its
implementation has not kept up.

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
all links Current, but ≥1 annotated file has changed since last Seal), or
**Traced** (Complete, all links Current, and all annotated files match their
sealed hashes). The tool never executes tests; keeping the linked tests green
is the user's responsibility, outside the tool's scope.
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
