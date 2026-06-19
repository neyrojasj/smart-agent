---
name: to-smart-issues
description: >
  Consume the not-yet-Traced requirements reported by `weft check` and produce
  a vertical-slice implementation plan as local markdown files under `.scratch/`.
  Each issue spans one or more whole requirements taken end-to-end to the Traced
  state, embeds the exact REQ_ID + version + hash so @implements/@verifies
  annotations are copy-paste correct, and is tagged `implement`
  (Orphaned/Incomplete) or `rework` (Stale). A PRD.md tracking file ties all
  slices together. Fully-Traced requirements produce no work.
version: "1.1"
---

# to-smart-issues Skill

## Identity

- **Name**: to-smart-issues
- **Version**: 1.1
- **Input**: A project with `docs/prds/` requirement records and existing annotations
- **Output**: Local markdown files under `.scratch/<feature-slug>/` — one issue file per vertical slice plus a `PRD.md` tracking file. Each issue embeds REQ_ID+version+hash and is tagged `implement` or `rework`.

---

<!-- @implements REQ-028 v2 46490789 -->
## Purpose

Turn the gap reported by `weft check` into a concrete, ordered implementation
plan. Each gap is one or more *whole* requirements that must reach the `Traced`
state. A finished slice is verifiable by `weft check` going green for its
requirements — no softer "done" definition is accepted.

**Never sub-divide a requirement.** If a requirement is too large to finish in
one slice, the correct action is to split the requirement first (via
`to-smart-prd`), not to split the slice.

**Ephemeral User Stories** — do not persist User Stories to `docs/prds/` or to
the issue body as standalone artifacts. Generate them on demand for context only.

---

## Triggers

| Pattern | Example |
|---------|---------|
| `to-smart-issues` keyword | "run to-smart-issues" |
| "plan implementation" | "plan the implementation gaps" |
| "create implementation issues" | "turn unfinished requirements into issues" |
| "what needs implementing" | "what requirements are not yet traced?" |

---

## Prerequisites

- `weft` CLI must be installed and on the `PATH`. Run `weft --help` to verify.
  If absent, tell the user to build it first: `cargo build --release`.
- Requirement records must be up to date and pass `weft verify`.
- The `.scratch/` directory must exist at the project root (created by convention;
  create it if absent).

---

## Workflow

<!-- @implements REQ-025 v2 6ff51574 -->
### Step 1 — Discover Not-Traced Requirements

```
weft check
```

Parse the output. Each line is `REQ-NNN: <TraceState>`. Collect every line whose
state is **not** `Traced`:

| TraceState | Slice kind |
|------------|------------|
| `Orphaned` | `implement` |
| `Incomplete` | `implement` |
| `Stale` | `rework` |
| `Traced` | _skip — no work_ |

If all requirements are `Traced`, report success and stop:
```
✅ All requirements are Traced. No implementation gaps.
```

### Step 2 — Gather Requirement Details

For each not-Traced requirement ID, capture its three key fields:

```
weft get REQ-NNN --field statement
weft get REQ-NNN --field version
weft get REQ-NNN --field hash
```

Build an internal working list:

```
REQ-NNN | <TraceState> | v<version> | <hash> | <statement first line>
```

<!-- @implements REQ-027 v2 af09e7de -->
### Step 3 — Group Requirements into Vertical Slices

Apply the following grouping rules:

1. **One slice per FEAT** — requirements sharing the same `FEAT` label should
   be grouped together when they are all `implement` or all `rework`. Mixed
   states within a FEAT → separate slices.

2. **Cohesion over FEAT** — two requirements in different FEATs that share an
   obvious implementation seam (e.g., both require the same CLI command) may be
   grouped, but keep groups small enough to finish in one session.

3. **`rework` slices are always separate from `implement` slices** — a Stale
   requirement and an Orphaned/Incomplete requirement must never be in the same
   slice, because the work shapes are different.

4. **No sub-requirement splitting** — a requirement that appears too large must
   be split at the requirement level (via `to-smart-prd`) before this skill runs.

5. **Order: tracer-bullet first** — order slices thin-to-thick: the slice that
   demonstrates the thinnest end-to-end path goes first; foundational
   requirements before the requirements that depend on them.

Each group becomes one issue.

### Step 4 — Write Issue Files

Determine the feature slug from the dominant FEAT label (e.g. `FEAT-FileDrift` →
`feat-file-drift`). If requirements span multiple FEATs, use a short kebab-case
description of the overall goal.

Create the directory structure:
```
.scratch/<feature-slug>/
.scratch/<feature-slug>/issues/
```

For each slice, write one file at:
```
.scratch/<feature-slug>/issues/<NN>-<short-slug>.md
```

where `<NN>` is zero-padded slice number (`01`, `02`, …) and `<short-slug>` is a
3-5 word kebab-case description of the slice.

#### Issue file template

```markdown
<!-- @implements REQ-026 v2 f2ba6521 -->
# Slice N: <short description>

Status: ready-for-agent
Labels: slice, <implement|rework>

## Scope

<one paragraph describing what the slice achieves end-to-end>

## Requirements in this slice

<!-- Copy-paste these into your @addresses / @implements / @verifies annotations -->
| REQ_ID | Version | Hash | Statement |
|--------|---------|------|-----------|
| REQ-NNN | vN | <hash> | <first line of statement> |
| REQ-MMM | vM | <hash> | <first line of statement> |

## Trace annotations (copy-paste correct)

Design decision:
```
@addresses REQ-NNN vN <hash>
```

Code:
```
// @implements REQ-NNN vN <hash>
```

Test:
```
// @verifies REQ-NNN vN <hash>
```

## Verifiable when

`weft check` exits 0 and reports `Traced` for every requirement in this slice.

## Slice kind

<!-- implement: Orphaned/Incomplete requirements — net-new work -->
<!-- rework: Stale requirements — existing work to update -->
<implement | rework>
```

#### Status line

Set `Status: ready-for-agent` on every new issue file — the slice is already
scoped and verified by `weft check`, so no manual triage step is needed.

### Step 5 — Update PRD Tracking File

After all slice issue files are written, update `.scratch/<feature-slug>/PRD.md`
with the implementation plan.

**Check whether `PRD.md` already exists** (written by a prior `to-smart-prd` run):

- **If it exists:** locate the `## Slices` section and replace its body (from
  the line after `## Slices` up to the next `##` heading or end of file) with
  the slice task-list and updated `## Done when` block. Leave every other
  section (Problem Statement, Solution, Implementation Decisions, Testing
  Decisions, Out of Scope) completely untouched.

- **If it does not exist:** create the file using the full standalone template
  below.

#### Full PRD.md template (standalone — no prior to-smart-prd run)

```markdown
# <Feature name>: Implementation Tracking

Status: ready-for-agent
Labels: epic

## Goal

<one paragraph: what the completed implementation will deliver>

## Slices

Complete these in order (each slice's scope and trace annotations are in the linked file):

- [ ] [Slice 1: <title>](issues/01-<slug>.md)
- [ ] [Slice 2: <title>](issues/02-<slug>.md)
…

## Done when

`weft check` exits 0 for all requirements listed across the slices above.
```

#### Slices section replacement (existing PRD.md)

Replace only the body of the `## Slices` section with:

```markdown
Complete these in order (each slice's scope and trace annotations are in the linked file):

- [ ] [Slice 1: <title>](issues/01-<slug>.md)
- [ ] [Slice 2: <title>](issues/02-<slug>.md)
…
```

The task-list items must link to the exact filenames written in Step 4, in the
same order as the slices.

### Step 6 — Report

After writing all slice files and the PRD tracking file, emit a concise summary:

```
✅ Created 3 slice file(s) + PRD tracking file:

  .scratch/feat-weft/PRD.md                              [epic]
  .scratch/feat-weft/issues/01-verify-hash-integrity.md  [implement] Slice 1: weft verify + hash integrity (REQ-005, REQ-006)
  .scratch/feat-weft/issues/02-get-and-list.md           [implement] Slice 2: weft get + weft list (REQ-007, REQ-009, REQ-010)
  .scratch/feat-weft/issues/03-annotation-scan-update.md [rework]   Slice 3: annotation scanning update (REQ-012)

⏭  Skipped (already Traced): REQ-001, REQ-002, REQ-003, REQ-004
```

---

## Rules

1. **Never sub-divide a requirement** — if a requirement is too big, split it
   first at the PRD level, then re-run this skill.
2. **Always embed REQ_ID + version + hash** — the exact values from `weft get`
   so annotations written by the engineer are copy-paste correct.
3. **`rework` and `implement` never mix in one slice** — the work shapes are
   different; keep them separate.
4. **Traced requirements produce no files** — `weft check` reporting `Traced`
   means the requirement is done; do not create placeholder issues.
5. **Verify before writing files** — run `weft verify` first; refuse to
   write any files if any requirement record has a hash mismatch.
6. **Order matters** — tracer-bullet order: thinnest end-to-end slice first.
7. **Issue file titles start with `Slice N:`** — so files sort and relate visually.
8. **Compose with, never overwrite, an existing PRD.md** — if `to-smart-prd`
   already wrote the file, update only the `## Slices` section; all prose
   sections (Problem Statement, Solution, Implementation Decisions, Testing
   Decisions, Out of Scope) must be left untouched.
9. **Follow the local issue tracker convention** — files go under `.scratch/<feature-slug>/`
   as defined in `docs/agents/issue-tracker.md`. Every issue file gets `Status: ready-for-agent`.

---

## Error Handling

| Situation | Action |
|-----------|--------|
| `weft` not found | Tell user to `cargo build --release` first |
| `weft verify` reports hash mismatch | Run `weft bump REQ-NNN` on the affected record; re-verify before continuing |
| `weft check` exits 0 (all Traced) | Report success and stop; no files to write |
| Requirement too large to slice alone | Recommend splitting the requirement via `to-smart-prd` |
| Ambiguous FEAT grouping | Ask user to confirm grouping before writing files |
| `.scratch/` directory does not exist | Create it with `mkdir -p .scratch` before writing |

---

## Glossary Notes

- **Vertical Slice**: spans one or more whole requirements taken to `Traced`.
  Done is verifiable: `weft check` goes green for the slice's requirements.
- **Traced**: all three Trace Links present (design + code + test) and all
  current (frozen hashes match the requirement's current Content Hash).
- **Stale**: all links present but at least one pins an old hash → `rework`.
- **Orphaned/Incomplete**: missing one or more links → `implement`.
- **REQ_ID**: immutable identity of a requirement (e.g. `REQ-042`).
- **Content Hash**: 8 hex chars over the normalized normative region.
- **Normative Region**: `statement` + `acceptance` — what the hash covers.

See `CONTEXT.md` for the full glossary.
