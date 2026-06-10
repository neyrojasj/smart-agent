---
name: to-smart-issues
description: >
  Consume the not-yet-Traced requirements reported by `weft check` and produce
  a vertical-slice implementation plan as GitHub issues. Each issue spans one or
  more whole requirements taken end-to-end to the Traced state, embeds the exact
  REQ_ID + version + hash so @implements/@verifies annotations are copy-paste
  correct, and is tagged `implement` (Orphaned/Incomplete) or `rework` (Stale).
  Fully-Traced requirements produce no work.
version: "1.0"
---

# to-smart-issues Skill

## Identity

- **Name**: to-smart-issues
- **Version**: 1.0
- **Input**: A project with `docs/prds/` requirement records and existing annotations
- **Output**: GitHub issues, one per vertical slice; each issue embeds REQ_ID+version+hash and is tagged `implement` or `rework`

---

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
- `gh` CLI must be available and authenticated for issue creation.
  Run `gh auth status` to verify.
- Requirement records must be up to date and pass `weft verify`.

---

## Workflow

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

### Step 4 — Create Issues

For each slice, create a GitHub issue:

```
gh issue create \
  --title "Slice N: <short description>" \
  --label "slice" \
  --label "<implement|rework>" \
  --body "$(cat <<'EOF'
<issue body — see template below>
EOF
)"
```

#### Issue body template

```markdown
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

#### Labels to apply

| Condition | Labels |
|-----------|--------|
| All requirements Orphaned or Incomplete | `slice`, `implement` |
| Any requirement Stale | `slice`, `rework` |

Ensure the labels exist before running `gh issue create`:
```
gh label create slice --color "0075ca" --description "Vertical slice of work" 2>/dev/null || true
gh label create implement --color "e4e669" --description "Net-new implementation" 2>/dev/null || true
gh label create rework --color "d93f0b" --description "Update stale implementation" 2>/dev/null || true
```

### Step 5 — Report

After creating all issues, emit a concise summary:

```
✅ Created 3 issue(s):

  #42 [implement] Slice 1: weft verify + hash integrity (REQ-005, REQ-006)
  #43 [implement] Slice 2: weft get + weft list (REQ-007, REQ-009, REQ-010)
  #44 [rework]   Slice 3: annotation scanning update (REQ-012)

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
4. **Traced requirements produce no issues** — `weft check` reporting `Traced`
   means the requirement is done; do not create placeholder issues.
5. **Verify before creating issues** — run `weft verify` first; refuse to
   create issues if any requirement record has a hash mismatch.
6. **Order matters** — tracer-bullet order: thinnest end-to-end slice first.
7. **Issue titles start with `Slice N:`** — so issues sort and relate visually.

---

## Error Handling

| Situation | Action |
|-----------|--------|
| `weft` not found | Tell user to `cargo build --release` first |
| `gh` not found or not authenticated | Tell user to install and run `gh auth login` |
| `weft verify` reports hash mismatch | Run `weft bump REQ-NNN` on the affected record; re-verify before continuing |
| `weft check` exits 0 (all Traced) | Report success and stop; no issues to create |
| Requirement too large to slice alone | Recommend splitting the requirement via `to-smart-prd` |
| Ambiguous FEAT grouping | Ask user to confirm grouping before creating issues |

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
