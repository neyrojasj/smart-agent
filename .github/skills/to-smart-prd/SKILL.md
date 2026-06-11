---
name: to-smart-prd
description: >
  Turn a grilling session into machine-first requirement records under
  docs/prds/. Upserts net-new records (weft new), bumps requirements whose
  normative text changed (weft bump), and deprecates requirements removed from
  the PRD's intent (weft deprecate). User Stories are ephemeral — never
  persisted.
version: "1.0"
---

# to-smart-prd Skill

## Identity

- **Name**: to-smart-prd
- **Version**: 1.0
- **Input**: The current session conversation (grilling output — requirements extracted from it)
- **Output**: TOML requirement records written/updated under `docs/prds/`; each passes `weft verify`

---

## Purpose

Convert a grilling session's stated requirements into durable, machine-first
requirement records. The PRD source of truth is the TOML files, never a prose
document or user story. Each run must be **idempotent** — re-running with the
same session changes nothing; re-running after an edit bumps the affected record.

**User Stories are ephemeral** — they are never written to `docs/prds/` and
never committed to the repository. The `Requirement` is the durable source;
stories are generated on demand from it at implementation time.

---

## Triggers

| Pattern | Example |
|---------|---------|
| `to-smart-prd` keyword | "run to-smart-prd" |
| chained from grilling session | after PRD grilling is finished |
| "upsert requirements" | "upsert requirements from this session" |
| "write requirement records" | "write the requirements we just discussed" |

---

## Prerequisites

`weft` CLI must be installed and on the `PATH`. Run `weft --help` to verify.
If absent, tell the user to build it first: `cargo build --release`.

---

## Workflow

<!-- @implements REQ-022 v2 b34eb010 -->
### Step 1 — Extract Requirements from Session

Read the current conversation and extract each distinct requirement. For each:

| Field | What to extract |
|-------|----------------|
| **statement** | A single declarative sentence: "The system must …" |
| **acceptance** | One criterion per bullet ("Given … then …") — at minimum one |
| **feat** | Optional group label (`FEAT-<Name>`), if discussed in the session |
| **rationale** | Optional — why the requirement exists |
| **notes** | Optional — commentary, examples |

Produce an internal working list. Do **not** generate or persist User Stories.

### Step 2 — Discover Existing Records

```
weft list
```

Capture the output: each line is `REQ-NNN: <description>`. Build a map of
existing IDs to their current first-line description.

For requirements grouped by feature:

```
weft list --feat FEAT-<Name>
```

### Step 3 — Upsert Each Session Requirement

For each extracted requirement, decide its action:

#### 3a — Net-new requirement (no match in existing list)

1. Allocate the next ID:
   ```
   weft new [--feat FEAT-<Name>]
   ```
   Capture the output line `REQ-NNN: <path>` to learn the assigned ID and file path.

2. Overwrite the skeleton file with the real content. Write a valid TOML record:
   ```toml
   id = "REQ-NNN"
   version = 1
   hash = "<placeholder — will be fixed by bump>"
   status = "active"
   feat = "FEAT-<Name>"           # only if a feat was identified
   statement = "<statement>"
   acceptance = [
       "<criterion 1>",
       "<criterion 2>",
   ]
   rationale = "<rationale>"      # only if provided
   notes = "<notes>"              # only if provided
   ```
   Set `hash` to any 8-char hex string for now (it will be fixed in the next step).

3. Recompute the hash and bump to v1 (a clean initial version):
   ```
   weft bump REQ-NNN
   ```
   The version will be 2 after this bump. That is correct: the skeleton was v1
   (placeholder content); the real content is v2.

   **Alternative when the skeleton's placeholder content was never meaningful:**
   After writing the real content, run `weft bump` once. The resulting version
   is v2; this is the canonical first real version.

<!-- @implements REQ-023 v2 4fa1a337 -->
#### 3b — Existing requirement, normative text unchanged

Compare the session's statement and acceptance against the current record with:

```
weft get REQ-NNN --field statement
weft get REQ-NNN --field acceptance
```

If they match (after trimming), **no action** — the record is already current.

#### 3c — Existing requirement, normative text changed

The session contains an evolved version of an existing requirement.

1. Edit the TOML file in place: update `statement` and/or `acceptance` with the
   new text. Leave `id`, `version`, `hash`, `feat`, `status` unchanged for now.

2. Bump to record the change honestly:
   ```
   weft bump REQ-NNN
   ```
   This increments `version` and recomputes `hash` from the new text as a single
   atomic operation.

<!-- @implements REQ-024 v2 00a713df -->
### Step 4 — Deprecate Removed Requirements

Any requirement that **was active** in `docs/prds/` but is **absent from the
session's intent** (not matched to any session requirement in Step 3) must be
deprecated — never deleted.

```
weft deprecate REQ-NNN
```

Use judgment: a requirement is "absent from the session's intent" only if the
grilling session explicitly removed or superseded it, or the session's scope
clearly no longer encompasses it. **When in doubt, do not deprecate** — ask the
user to confirm first.

### Step 5 — Validate All Touched Records

```
weft verify [docs/prds]
```

Every touched record must report `ok`. Fix any issues before proceeding.

### Step 6 — Report

Emit a concise summary:

```
✅ Created:    REQ-007 (v2) — The system must allow users to reset their password.
✅ Bumped:     REQ-003 (v1 → v3) — The system must enforce session timeouts.
⚠️  Deprecated: REQ-005 — The system must support OAuth login. [removed from PRD intent]
⏭  Unchanged:  REQ-001, REQ-002, REQ-004

weft verify: all ok
```

---

## Rules

1. **Never persist User Stories** — ephemeral expansions only, generated at
   implementation time.
2. **Never delete requirement files** — deprecate with `weft deprecate`, not `rm`.
3. **REQ_IDs are immutable** — never reassign a number to a different requirement.
4. **Normative text only in statement + acceptance** — `rationale` and `notes` are
   commentary; editing them does NOT require a bump.
5. **One canonical record per requirement** — if in doubt whether two session
   requirements map to the same record, keep them separate and let the author
   merge manually.
6. **Hash must always be consistent** — always run `weft bump` after editing
   normative text. Never leave a record whose stored hash diverges from its content.
7. **Verify before finishing** — `weft verify` must report `ok` for every touched
   record before the skill reports success.

---

## Error Handling

| Situation | Action |
|-----------|--------|
| `weft` not found | Tell user to `cargo build --release` first |
| `weft verify` reports hash mismatch | Re-run `weft bump REQ-NNN` on the affected record |
| Requirement in session has no clear statement | Ask user for clarification |
| Cannot determine if existing record matches session requirement | Ask user to confirm mapping |
| `weft deprecate` fails (file not found) | Report to user — do not silently skip |

---

## Glossary Notes

- **Requirement**: the durable unit; never the same as a User Story.
- **Normative Region**: `statement` + `acceptance` — the only part covered by the Content Hash.
- **Commentary Region**: `rationale` + `notes` — free to edit without bumping.
- **Deprecated**: `status = "deprecated"` — preserved, never deleted.
- **Vertical Slice**: not produced by this skill; see `to-smart-issues` for that.

See `CONTEXT.md` for the full glossary.
