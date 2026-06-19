---
name: weft
description: >
  Load when working with requirements, weft commands, trace annotations, or the
  .scratch issue tracker. Covers the full weft CLI reference, annotation format,
  Trace State workflow, .scratch conventions, and docs hierarchy.
version: "1.0"
---

<!-- @implements REQ-050 v2 b7e04277 -->
<!-- @implements REQ-051 v2 16a337fc -->

# Weft Skill

`weft` is a requirements-traceability CLI. It verifies an unbroken chain from a
requirement (PRD → design → code → test) and reports when any link is missing or
stale.

> **Invoke as:** `target/debug/weft <COMMAND>` (or just `weft` if on PATH).

---

## Command Reference

### `weft verify`
Validate requirement records (format + hash integrity). Checks that each
`docs/prds/*.toml` file is well-formed and its `hash` field matches the
computed Content Hash of its normative region (`statement` + `acceptance`).

### `weft check [--summary] [--json]`
Report each active requirement's Trace State; exits non-zero on any drift.

- `--summary` — print a rollup count per Trace State instead of per-requirement detail
- `--json` — emit a JSON array with `id`, `state`, `missing_links`, `stale_links`, `drifted_files`

### `weft new`
Allocate the next REQ_ID and write a skeleton requirement record under
`docs/prds/`.

### `weft list`
List all requirements by id and description.

### `weft get <REQ_ID> <FIELD>`
Print a single field of a requirement record (e.g. `weft get REQ-042 statement`).

### `weft bump <REQ_ID>`
Increment a requirement's version and recompute its Content Hash. Use after
editing the normative region (`statement` or `acceptance`) of a requirement.

### `weft deprecate <REQ_ID>`
Mark a requirement `status = "deprecated"`. Preserved, never deleted. Excluded
from `weft check` and `weft gate` going forward.

### `weft render`
Generate a human-readable Markdown view of `docs/prds/`. Read-only; never
modifies TOML records.

### `weft init`
Scaffold `docs/prds/`, `docs/decisions/`, `.weftignore`, and install agent
skills into `.claude/skills/`. Safe to run on an already-initialized project
(idempotent).

### `weft seal [REQ_ID]`
Record the current SHA-256 (File Hash) of every annotated file into
`docs/prds/weft.lock`. Run after confirming that changed files still satisfy
their requirements. Targeted form: `weft seal REQ-042`.

### `weft trace <REQ_ID>`
Print each Trace Link found for a requirement, with its kind and `file:line`
location.

### `weft annotate --kind <KIND> <REQ_ID>`
Print the exact Trace Link line for a requirement using its current version and
hash. Kinds: `addresses`, `implements`, `verifies`.

```
$ weft annotate --kind implements REQ-042
// @implements REQ-042 v3 a1b2c3d4
```

### `weft test [REQ_ID]`
Run the configured Test Command for each active requirement (or a single
REQ_ID) and record pass/fail into `docs/prds/weft.run.toml`.

### `weft gate`
Exit zero only when every active requirement is Verified. The autonomous
agent's single loop-termination check. Distinct from `weft check` (which gates
on drift, not test results).

### `weft next`
Emit the single highest-priority not-yet-Verified requirement with an explicit
action verb (`implement | rework | reseal | fix-tests | run-tests`). Exits
zero (with "no next work item") when all requirements are Verified. The Work
Driver for the autonomous agent loop.

---

## Trace Annotation Format

Every Trace Link is pinned to `REQ_ID version hash` frozen at link time.

### Design (ADR frontmatter — TOML `+++` block)
```toml
+++
addresses = [
    "REQ-042 v3 a1b2c3d4",
    "REQ-043 v1 e5f6a7b8",
]
+++
```

### Code (inline comment)
```
// @implements REQ-042 v3 a1b2c3d4
```

Place immediately before the function, struct, impl block, or in a file-level
comment. Markdown files use HTML comments:
```html
<!-- @implements REQ-042 v3 a1b2c3d4 -->
```

### Test (inline comment, before the test function)
```
// @verifies REQ-042 v3 a1b2c3d4
#[test]
fn my_test() { ... }
```

### Getting the correct annotation line
Always use `weft annotate` rather than copying by hand:
```
$ weft annotate --kind implements REQ-042
// @implements REQ-042 v3 a1b2c3d4
```

---

## Trace States

| State | Meaning |
|-------|---------|
| **Orphaned** | No Trace Links exist |
| **Incomplete** | At least one link is missing (needs @addresses, @implements, or @verifies) |
| **Stale** | A link pins an old Content Hash (requirement changed since link was written) |
| **Drifted** | All links are current, but ≥1 annotated file changed since last `weft seal` |
| **Traced** | Complete, all links current, all file hashes match `weft.lock` |
| **Verified** | Traced + most recent Verification Run passed |

A requirement is **Complete** only when all three links exist.

---

## .scratch/ Issue Tracker Conventions

Issues live as local Markdown files under `.scratch/`.

```
.scratch/
└── <feature-slug>/
    ├── PRD.md
    └── issues/
        ├── 01-<slug>.md
        └── 02-<slug>.md
```

- One feature per directory.
- Issues numbered from `01`, two digits.
- `Status:` line near the top records triage state: `needs-triage | needs-info | ready-for-agent | ready-for-human | wontfix | done`.
- Comments and conversation history append under a `## Comments` heading.

---

## Documentation Hierarchy

| Path | Purpose |
|------|---------|
| `docs/prds/` | Machine-first requirement records (TOML). Source of truth. |
| `docs/prds/weft.lock` | File Hashes for sealed annotated files. |
| `docs/prds/weft.run.toml` | Last Verification Run results per requirement. |
| `docs/adr/` | Architectural Decision Records (format: `NNNN-slug.md`). |
| `docs/decisions/` | Lightweight design notes (not full ADRs). |
| `CONTEXT.md` | Domain glossary — preferred terms and what to avoid. |

---

## Workflow: Advancing a Requirement to Traced

1. **`weft next`** — find the highest-priority unfinished requirement.
2. **Design** — write/update an ADR in `docs/adr/` with `@addresses` in the frontmatter (`weft annotate --kind addresses REQ-NNN`).
3. **Implement** — write the code; add `// @implements REQ-NNN vN hash` before the key function/module.
4. **Test** — write the test; add `// @verifies REQ-NNN vN hash` before it.
5. **`weft check`** — confirm Traced (or diagnose remaining gaps).
6. **`weft seal REQ-NNN`** — freeze File Hashes once satisfied.
7. **`weft test REQ-NNN`** (optional) — record Verification Run; advances to Verified.
