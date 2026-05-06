---
name: fix
description: Fix issues by reading DDD/TDD/GLO, updating DES and TST to reflect the change, then triggering the planning skill to create an implementation plan.
version: "1.0"
---

# Fix Skill

## Identity

- **Name**: fix
- **Version**: 1.0
- **Mode**: Full
- **Inputs**: GLO + DES (if exists) + TST (if exists) + broken code/test/error
- **Outputs**: Updated DES + Updated TST + new PLAN-XXX.md via planning skill

---

## Purpose

Fixes are not ad-hoc patches. Every fix goes through DES → TST → PLAN → coding.
This ensures the design stays coherent and tests stay accurate after every change.

---

## Triggers

| Pattern | Example |
|---------|---------|
| fix, bug, broken, error, failing | "fix the auth login bug" |
| "tests are failing" | "tests failing in FooService" |
| "update X to do Y" | "update BarHandler to handle null events" |
| "regression in" | "regression in queue MOD" |

---

## Fix vs New Feature

| Situation | Route |
|-----------|-------|
| Bug in existing IFC behavior | Fix skill |
| New IFC or new MOD needed | DDD skill first, then fix/plan |
| Breaking test that was never green | TDD skill (test was wrong) |
| Change to existing IFC signature | Fix skill (updates DES + TST + PLAN) |

---

## Workflow

### Step 1: Load All Context

```
1. Read .github/copilot/glossary.md
2. Read .github/copilot/context.md
3. Find and read the relevant DES:
   - Search .github/copilot/docs/ddd/ for DESIGN-XXX.md files
   - If multiple exist, grep for @mod or @ifc tags matching the broken area
4. Find and read the relevant TST:
   - Search .github/copilot/docs/tdd/ for TEST-XXX.md files
   - Match to the DES by number or topic
5. Read the failing code, error message, or test output
```

### Step 2: Classify the Fix

Determine what kind of change is needed:

| Class | Description | DES update? | TST update? |
|-------|-------------|-------------|-------------|
| **impl-fix** | Implementation wrong, IFC correct | No | No |
| **ifc-fix** | IFC signature needs to change | Yes | Yes |
| **scope-fix** | Behavior was never designed — gap in DES | Yes | Yes |
| **test-fix** | Test was incorrect (never reflected real contract) | No | Yes |

Ask user to confirm class if ambiguous.

### Step 3: Update DES (if ifc-fix or scope-fix)

```
1. Read current DESIGN-XXX.md
2. Update affected @ifc signatures
3. Update @status of affected MODs to draft
4. If new IFC added: assign @ifc tag, update cross-MOD table
5. Write updated DESIGN-XXX.md
6. Note all changes made
```

### Step 4: Update TST (if ifc-fix, scope-fix, or test-fix)

```
1. Read current TEST-XXX.md
2. Update test matrix rows affected by the DES change
3. Add new test cases for new behavior
4. Remove test cases that no longer apply (log them as removed)
5. Write updated TEST-XXX.md
6. Update actual test files to match new matrix
   - New/changed cases → update test file (still RED until impl)
   - Removed cases → delete from test file
```

### Step 5: Create Fix Plan via Planning Skill

Pass to planning skill:
- The classification (impl-fix / ifc-fix / scope-fix / test-fix)
- The updated DES and TST (or unchanged if impl-fix)
- The specific issue to fix

Planning skill will:
1. Read KF context
2. Create a narrowly scoped PLAN (small or medium)
3. Create KF with DES/TST summary
4. Route to plan-reviewer → user approval → coding

### Step 6: Report

```markdown
🔧 **Fix classified: [class]**

DES updated: [yes/no] — [summary of changes]
TST updated: [yes/no] — [summary of changes]

→ PLAN created: .github/copilot/plans/PLAN-XXX.md
→ Awaiting approval to implement
```

---

## Rules

- NEVER patch code without reading DES and TST first
- NEVER change IFC signatures without updating DES
- NEVER change IFC behavior without updating TST
- impl-fix: only coding changes — DES and TST stay untouched
- All fixes go through planning skill (even small impl-fixes get a PLAN)
- Do not widen scope — fix only what is broken
