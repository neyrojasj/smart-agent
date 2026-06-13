---
name: weft-audit
description: >
  Independently audit every Traced requirement by spawning one Haiku
  subagent per requirement to check whether its acceptance criteria are
  actually satisfied by the linked code/tests, and whether the
  implementation's logic makes sense given the requirement's intent.
  Findings are appended to .scratch/weft-audit/results.md for the
  orchestrator to review. Use when the user asks to "audit requirements",
  "double-check the implementation against the spec", "verify acceptance
  criteria are really met", or "sanity-check weft's Traced requirements".
version: "1.0"
---

# weft-audit Skill

## Purpose

`weft check` only confirms that the *structural* Trace Links exist and their
hashes match — it cannot tell whether the linked code actually satisfies a
requirement's acceptance criteria, or whether the implementation's logic is
sound. This skill closes that gap: for each `Traced` requirement, a Haiku
subagent independently reads the requirement and its linked implementation +
test, then reports a PASS/CONCERN/FAIL verdict per acceptance criterion plus
an overall logic-sanity verdict.

This skill never edits requirement records or code — it only reports
findings to a results file for a human or the orchestrating session to act on.

## Prerequisites

- `weft` CLI must be on the `PATH`. If absent, tell the user to run
  `cargo build --release` first.
- `weft verify` should be clean. If unsure, run `/weft-status` first.

## Workflow

### Step 1 — Select requirements to audit

```
weft check
```

Collect every `REQ-NNN: Traced` line. Only `Traced` requirements have a
complete implementation + test to audit — skip everything else.

If the user named specific `REQ_ID`s or a `--feat`, narrow to those (cross-
check against `weft list --feat FEAT-X` / `weft check` output). If none of
the requested requirements are `Traced`, report that and stop — point at
`/weft-status` or `/to-smart-issues` instead.

If there are no `Traced` requirements at all, report:
```
Nothing to audit yet — no requirements are Traced. Run /weft-status to see
what's outstanding.
```
and stop.

### Step 2 — Gather requirement detail + locate artifacts

For each `REQ-NNN` selected:

```
weft get REQ-NNN --field statement
weft get REQ-NNN --field acceptance
weft get REQ-NNN --field version
weft get REQ-NNN --field hash
```

Then locate every Trace Link annotation for it (do this search yourself —
do not delegate it):

```
grep -rn "REQ-NNN" --include='*.rs' --include='*.md' -- weft weft-core docs/adr
```

From the matches, separate by annotation kind (`@addresses`, `@implements`,
`@verifies`) and record each as `file:line`. These are the *exact* locations
you will hand to the subagent — never let the subagent search broadly.

If `Traced` but no `@implements`/`@verifies` annotation is found by this
grep, note it as a `CONCERN` for Step 4 ("Traced but annotation not found by
grep — investigate manually") and still include the requirement in the audit
using whatever locations *were* found.

### Step 3 — Spawn one Haiku subagent per requirement

Send the `Agent` calls for a batch of requirements in a single message (up
to ~4 in parallel) so they run concurrently. For each requirement, call
`Agent` with `model: "haiku"`, `subagent_type: "general-purpose"`, and a
SELF-CONTAINED prompt built from this template:

```
GOAL: Independently verify requirement REQ-NNN (v<version>, hash <hash>).

REQUIREMENT STATEMENT:
<statement text, verbatim>

ACCEPTANCE CRITERIA:
<acceptance list, verbatim, one per line>

SCOPE — read exactly these locations (and a small amount of surrounding
context in each file, e.g. the enclosing function/test — do not search
elsewhere or broaden scope):
- Design: <file:line or "none found">
- Code:   <file:line or "none found">
- Test:   <file:line or "none found">

TASK:
1. For each acceptance criterion above, decide PASS / CONCERN / FAIL: does
   the code at the Code location actually implement it, and does the test
   at the Test location actually exercise it? Cite the specific file:line
   that justifies your verdict.
2. Separately, judge LOGIC SANITY: does the implementation's logic make
   sense given the requirement statement's intent — are there obvious bugs,
   missed edge cases, or contradictions between what the code does and what
   the statement says it should do? PASS / CONCERN / FAIL with 1-2 sentences
   of reasoning.

OUTPUT FORMAT — respond with exactly this markdown, nothing else:

## REQ-NNN (vN, <hash>)
- Acceptance 1: PASS|CONCERN|FAIL — <one-line evidence with file:line>
- Acceptance 2: PASS|CONCERN|FAIL — <one-line evidence with file:line>
  (one line per acceptance criterion, in order)
- Logic sanity: PASS|CONCERN|FAIL — <1-2 sentence reasoning>
- Notes: <anything else worth flagging, or "none">

CONTEXT: You are READ-ONLY. Do not edit any file, run any command that
changes repository state (no git add/commit/checkout/etc.), and do not
create or modify files. Only read and report.

EXHAUSTIVENESS: Check every acceptance criterion listed above, then stop —
do not expand scope to other requirements or files.
```

Fill in `<file:line or "none found">` from Step 2's grep results — one line
per Trace Link kind found (a requirement may have multiple `@implements` /
`@verifies` lines; list all of them).

### Step 4 — Append results to the audit file

Create `.scratch/weft-audit/` if it doesn't exist (the rest of `.scratch/`
follows `docs/agents/issue-tracker.md`'s convention, but this audit log is a
flat append-only file, not a per-feature PRD).

Append (never overwrite) to `.scratch/weft-audit/results.md`:

```markdown
# weft-audit run — <ISO 8601 timestamp>

<each subagent's returned markdown block, in REQ_ID order>

---
```

If `.scratch/weft-audit/results.md` doesn't exist yet, create it with this
content as the first entry (no extra header needed — the per-run `#` heading
is sufficient).

### Step 5 — Summarize for the orchestrator

Print a compact table:

```
🔎 weft-audit — N requirement(s) reviewed

| REQ_ID  | Acceptance        | Logic sanity |
|---------|-------------------|--------------|
| REQ-036 | 2/2 PASS          | PASS         |
| REQ-037 | 1/2 PASS, 1 CONCERN | PASS       |

⚠️  CONCERN: REQ-037 — see .scratch/weft-audit/results.md for detail
```

If everything is `PASS`/`PASS`, end with:
```
✅ All audited requirements pass acceptance + logic sanity checks.
Full report appended to .scratch/weft-audit/results.md
```

If any `CONCERN`/`FAIL` exists, end with a recommendation to open the results
file, review the cited evidence, and decide whether it's a real gap (treat as
`Stale`/rework via `/to-smart-issues`) or a false positive (note it in the
requirement's `notes` field — does not require a `weft bump`).

## Rules

1. **One Haiku subagent per requirement** — keeps each review isolated;
   one subagent's confusion can't muddy another's report.
2. **Always hand subagents exact `file:line` targets** — never let them
   grep/search broadly. This mirrors the project's research-delegation
   convention (give a competent but unfamiliar colleague a precise brief).
3. **Subagents are strictly read-only** — no edits, no state-changing
   commands, no commits. This skill's only output is the appended report.
4. **Append-only results file** — never overwrite or truncate prior runs;
   each run gets its own timestamped `#` section.
5. **This skill never edits requirement records or code.** Acting on
   findings (bumping, reworking, updating notes) is a separate, explicit
   follow-up the user decides on after reviewing the report.

## Error Handling

| Situation | Action |
|-----------|--------|
| `weft` not found | Tell user to run `cargo build --release` first |
| No `Traced` requirements | Report nothing to audit, suggest `/weft-status` |
| Requested `REQ_ID` not `Traced` | Skip it, note why, suggest `/to-smart-issues` |
| `Traced` but no annotation found by grep | Include with a `CONCERN` note; do not fail the whole run |
| `.scratch/weft-audit/` missing | Create it |
