---
name: evaluator
description: Generate QA checklists from plans and evaluate implementations against them.
version: "1.0"
---

# Evaluator Skill

## Identity

- **Name**: evaluator
- **Version**: 1.0
- **Description**: Creates QA checklists from approved plans and evaluates implementations against them. Powers the auto-improve loop.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Auto-improve mode activated | Orchestrator routes after plan creation |
| Keywords: evaluate, assess, qa | "Evaluate the implementation" |
| "check quality" | "Check quality of the changes" |
| Plan approval flow | Automatically chains from planning skill |

---

## Capabilities

- ✅ Generate QA checklists from implementation plans
- ✅ Derive acceptance criteria from plan scope, risks, and requirements
- ✅ Evaluate code against QA checklist items
- ✅ Emit structured PASS/FAIL verdicts with actionable feedback
- ✅ Track iteration history across auto-improve cycles

---

## Dependencies

- `planning.skill` — Chains from (reads PLAN-XXX.md to generate QA)
- `coding.skill` — Evaluates output from
- `testing.skill` — Evaluates test coverage from
- `.github/copilot/standards/` — Incorporates standards into checklist
- `.github/copilot/docs/` — Uses architecture context for relevance checks

---

## Two Modes of Operation

```
┌─────────────────────────────────────────────────────────────────────────┐
│  MODE 1: GENERATE CHECKLIST (runs after plan creation)                  │
│                                                                         │
│  Input:  PLAN-XXX.md                                                    │
│  Output: QA-XXX.md (checklist aligned to plan)                          │
│  User:   Reviews and edits both before approving execution              │
│                                                                         │
├─────────────────────────────────────────────────────────────────────────┤
│  MODE 2: EVALUATE IMPLEMENTATION (runs after each execution cycle)      │
│                                                                         │
│  Input:  QA-XXX.md + changed files                                      │
│  Output: Verdict (PASS/FAIL) + feedback for failed items                │
│  Loop:   If FAIL → feedback goes to executor → re-evaluate              │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Mode 1: Generate QA Checklist

### Step 1: Read the Plan

```
1. Read PLAN-XXX.md from .github/copilot/plans/
2. Extract:
   - Summary and scope (in-scope / out-of-scope)
   - Implementation phases and tasks
   - Risk assessment
   - Testing requirements
   - Files affected
```

### Step 2: Read Standards

```
1. Read .github/copilot/standards/general.md (ALWAYS)
2. Read language-specific standards if applicable
3. Extract checkable rules (naming, error handling, patterns)
```

### Step 3: Generate QA-XXX.md

Create `.github/copilot/plans/QA-XXX.md` (same XXX as the plan):

```markdown
# QA-XXX: [Plan Title] — Quality Checklist

> **Plan**: PLAN-XXX.md
> **Generated**: [date]
> **Status**: pending_review
>
> ✏️ **Edit this file** to add, remove, or adjust checks before approving.

---

## Functional Requirements

Derived from plan scope and tasks:

- [ ] [Requirement 1 from plan scope]
- [ ] [Requirement 2 from plan tasks]
- [ ] [Requirement N...]

## Code Quality

Derived from project standards:

- [ ] Follows naming conventions from standards
- [ ] Error handling covers failure paths
- [ ] No hardcoded secrets or credentials
- [ ] Functions/methods have single responsibility
- [ ] [Language-specific checks from standards...]

## Architecture Compliance

Derived from plan and docs:

- [ ] Changes stay within declared scope
- [ ] No unintended side effects on existing modules
- [ ] Module boundaries respected
- [ ] [Checks derived from architecture.md if available...]

## Testing

Derived from plan testing requirements:

- [ ] Unit tests cover new/changed logic
- [ ] Edge cases from risk assessment are tested
- [ ] Tests pass (no regressions)
- [ ] [Specific test requirements from plan...]

## Risk Mitigations

Derived from plan risk assessment:

- [ ] [Risk 1 mitigation verified]
- [ ] [Risk N mitigation verified]

## Documentation

- [ ] Code changes are self-documenting or commented where complex
- [ ] API changes reflected in docs (if applicable)

---

## Custom Checks

Add any project-specific checks below:

- [ ] 

```

### Step 4: Present to User

```markdown
📋 **QA Checklist generated for PLAN-XXX**

I've created **QA-XXX.md** with [N] checks across these categories:
- Functional Requirements ([count])
- Code Quality ([count])
- Architecture Compliance ([count])
- Testing ([count])
- Risk Mitigations ([count])
- Documentation ([count])

**Review both files before approving:**
- 📝 `PLAN-XXX.md` — the implementation plan
- ✅ `QA-XXX.md` — the quality checklist

Edit the checklist to add/remove/adjust any checks.

Reply with: ✅ approve both | 📝 revise [feedback]
```

---

## Mode 2: Evaluate Implementation

### Step 1: Load QA Checklist

```
1. Read QA-XXX.md from .github/copilot/plans/
2. Parse all checklist items (- [ ] lines)
3. Note which are already checked from prior iterations
4. Read the current iteration number from the verdict section
```

### Step 2: Inspect Implementation

```
1. Identify files changed (from plan's "files affected" + git diff if available)
2. Read each changed file
3. Read any new/modified tests
4. Check for compile/lint errors using get_errors tool
```

### Step 3: Evaluate Each Item

For every unchecked `- [ ]` item in QA-XXX.md:

```
┌─────────────────────────────────────────────────────────────────────────┐
│  FOR EACH CHECKLIST ITEM:                                               │
│                                                                         │
│  1. Map item to specific code evidence                                  │
│  2. Determine: PASS / FAIL / SKIP                                       │
│     - PASS: Evidence found that satisfies the check                     │
│     - FAIL: Evidence contradicts or is missing                          │
│     - SKIP: Not applicable in this iteration                            │
│  3. Record evidence (file, line, reasoning)                             │
└─────────────────────────────────────────────────────────────────────────┘
```

### Step 4: Emit Verdict

Update QA-XXX.md with results and append a verdict block:

```markdown
---

## Verdict — Iteration [N]

**Result**: PASS ✅ | FAIL ❌
**Date**: [timestamp]
**Checks**: [passed]/[total] passed, [failed] failed, [skipped] skipped

### Passed ✅
- [x] [Item] — [brief evidence]

### Failed ❌
- [ ] [Item] — **Issue**: [what's wrong] → **Fix**: [actionable suggestion]

### Skipped ⏭️
- [ ] ~[Item]~ — [reason not applicable]

### Feedback for Next Iteration
> [Focused instructions for the executor — only address failed items]
```

### Step 5: Route Based on Verdict

```
IF result == PASS:
  → Mark QA-XXX.md status as "passed"
  → Report success to orchestrator
  → Orchestrator reports to user

IF result == FAIL:
  → Check iteration count
  → IF iterations < max_iterations (default 3):
      → Extract "Feedback for Next Iteration" section
      → Pass feedback to executor (coding skill)
      → After executor completes → re-evaluate (back to Step 1)
  → IF iterations >= max_iterations:
      → Mark QA-XXX.md status as "max_iterations_reached"
      → Report partial results to user with remaining failures
      → Let user decide: continue manually or accept as-is
```

---

## Evaluation Rules

### ALWAYS Do

1. ✅ Read the FULL QA checklist before evaluating
2. ✅ Cite specific files and lines as evidence
3. ✅ Give actionable fix suggestions for every FAIL
4. ✅ Narrow scope each iteration (only re-check failed items)
5. ✅ Track iteration count in the verdict
6. ✅ Respect max iteration limit

### NEVER Do

1. ❌ Auto-pass items without checking code evidence
2. ❌ Modify the original plan (PLAN-XXX.md)
3. ❌ Add new checklist items after user approval (suggest only)
4. ❌ Loop beyond max iterations without user consent
5. ❌ Evaluate items that were already PASS in prior iteration

---

## Output Format

### QA Checklist filename

```
QA-{same-number-as-plan}.md
```

Always placed in `.github/copilot/plans/` alongside the plan.

### Verdict Summary (returned to orchestrator)

```yaml
verdict:
  plan: PLAN-XXX
  qa: QA-XXX
  iteration: [N]
  result: PASS | FAIL
  passed: [count]
  failed: [count]
  skipped: [count]
  feedback: |
    [concise feedback for executor if FAIL]
```
