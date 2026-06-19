---
name: weft-status
description: >
  Run weft verify + weft check and present a human-readable summary of every
  requirement's Trace State, grouped by state with actionable next steps.
  Use when user asks "what's the weft status", "run weft check", "any
  requirements not traced?", "what needs implementing?", or after a commit to
  confirm nothing drifted.
version: "1.0"
---

# weft-status Skill

## Purpose

Give the user a single-glance health report of the requirements traceability
system: hash integrity first, then Trace State for every requirement, grouped
by urgency with a clear next-step for each problem group.

## Prerequisites

`weft` CLI must be on the `PATH`. If absent, tell the user to run
`cargo build --release` first.

## Workflow

### Step 1 — Verify hash integrity

```
weft verify
```

If any record reports a mismatch, stop and report:
```
❌ weft verify failed — hash mismatch on: REQ-NNN
   Fix with: weft bump REQ-NNN
   (Do not proceed until verify is clean.)
```

### Step 2 — Check Trace States

```
weft check
```

Parse every output line (`REQ-NNN: <State>`). Group requirements by state:

| State | Meaning |
|-------|---------|
| `Traced` | Complete, all links current, all sealed files match |
| `Drifted` | Complete + current links, but ≥1 annotated file changed since last seal |
| `Stale` | All links present but ≥1 link pins an old requirement hash |
| `Incomplete` | Some Trace Links present, but not all three |
| `Orphaned` | No Trace Links at all |

### Step 3 — Emit Summary

Print the summary in this format:

```
🔍 weft status

  ✅ Traced      <N>   — nothing to do
  🟠 Drifted     <N>   — review changed files, then: weft seal [REQ-NNN]
  🟡 Stale       <N>   — annotation hashes outdated; run /to-smart-issues (rework)
  🔴 Incomplete  <N>   — partial trace links; run /to-smart-issues
  🔴 Orphaned    <N>   — no trace links; run /to-smart-issues

  weft verify: all ok
```

After the summary table, list each non-Traced requirement with its state and
the first line of its statement:

```
Needs attention:
  🟠 Drifted    REQ-031  The system must maintain a flat TOML lock file…
                         Changed files: src/commands/seal.rs
  🟡 Stale      REQ-007  The system must emit a non-zero exit code…
  🔴 Orphaned   REQ-034  The system must…
```

If all requirements are `Traced`, replace the "Needs attention" block with:
```
✅ All requirements Traced — nothing to do.
```

### Step 4 — Suggest Next Steps

After the detail list, print one action line per non-Traced group present:

| State(s) present | Suggested action |
|------------------|-----------------|
| Orphaned / Incomplete | Run `/to-smart-issues` to generate implementation slices. |
| Stale | Run `/to-smart-issues` — existing annotations need updating (`rework` slices). |
| Drifted | Review the changed files, confirm they still satisfy the requirement, then run `weft seal` (or `weft seal REQ-NNN` to seal selectively). |
| Hash mismatch (verify failed) | Run `weft bump REQ-NNN` on each affected record, then re-run `/weft-status`. |

Only print suggestions for groups that are actually present.
