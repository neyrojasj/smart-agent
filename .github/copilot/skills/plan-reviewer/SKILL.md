---
name: plan-reviewer
description: Review plans via adversarial critique and generate QA checklists. Pre-execution quality gate.
version: "1.0"
---

# Plan Reviewer Skill

## Identity

- **Name**: plan-reviewer
- **Version**: 1.0
- **Description**: Reviews plans via adversarial critique (curate mode) and generates QA checklists (QA mode). Runs before execution as a pre-execution quality gate. Smart orchestrates the critique→revise loop.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Plan created by planning skill | Orchestrator routes after plan creation |
| Keywords: review plan, critique plan | "Review the plan" |
| QA generation needed | Orchestrator routes after plan curation passes |

---

## Capabilities

- ✅ Critique plans for completeness, feasibility, risk coverage, and scope discipline
- ✅ Emit structured PASS/REVISE verdicts with actionable feedback
- ✅ Generate QA checklists from curated plans
- ✅ Derive acceptance criteria from plan scope, risks, and requirements
- ✅ Incorporate project standards into QA checklists

---

## Dependencies

- `planning.skill` — Chains from (reads PLAN-XXX.md to critique and generate QA)
- `.github/copilot/standards/` — Incorporates standards into QA checklist
- `.github/copilot/docs/` — Uses architecture context for relevance checks

---

## Two Modes of Operation

**CURATE MODE** (runs after plan creation, before user sees it)
- Input: PLAN-XXX.md (draft)
- Output: PASS → plan ready for QA generation | REVISE → feedback to planner
- Loop: Smart orchestrates critique ↔ revise (max 2 rounds)

**QA MODE** (runs after plan curation passes)
- Input: PLAN-XXX.md (curated)
- Output: QA-XXX.md (checklist aligned to plan)
- User reviews and edits both before approving execution

---

## Curate Mode: Adversarial Plan Review

### Purpose

Before the user sees a plan, the plan-reviewer critiques it to ensure quality. The planning skill revises based on feedback. Smart orchestrates this loop up to `max_curation_rounds` (default: 2) times.

### Step 1: Read the Draft Plan

```
1. Read PLAN-XXX.md from .github/copilot/plans/
2. Read KNOWLEDGE-XXX.md from .github/copilot/plans/
3. Read .github/copilot/docs/ for architecture context
4. Read .github/copilot/context.md for project identity and decisions
```

### Step 2: Critique the Plan

Evaluate against this checklist:

**Completeness**
- [ ] Summary clearly describes what and why
- [ ] Scope has explicit in-scope AND out-of-scope
- [ ] All phases have files affected and tasks
- [ ] Risk assessment covers likely failure modes
- [ ] Rollback strategy is actionable
- [ ] Testing requirements are specific (not generic)
- [ ] Post-execution learning checklist is present
- [ ] KNOWLEDGE-XXX.md exists and contains real code snippets (not placeholders)
- [ ] KNOWLEDGE-XXX.md key files table matches plan's files affected
- [ ] KNOWLEDGE-XXX.md code patterns match project standards

**Feasibility**
- [ ] Phases ordered correctly (dependencies respected)
- [ ] Estimated changes realistic per phase
- [ ] No phase depends on files/APIs that don't exist yet (unless created in a prior phase)

**Risk Coverage**
- [ ] Breaking changes identified
- [ ] Integration points with existing code addressed
- [ ] Edge cases relevant to the change noted

**Scope Discipline**
- [ ] Plan doesn't exceed what was requested
- [ ] No unnecessary refactoring bundled in
- [ ] Each phase has a clear deliverable

### Step 3: Emit Verdict

```markdown
## Plan Curation — Round [N]

**Result**: PASS ✅ | REVISE 📝
**Plan**: PLAN-XXX.md

### Strong Points
- [What the plan does well]

### Issues Found (if REVISE)
| # | Category | Issue | Suggested Fix |
|---|----------|-------|---------------|
| 1 | [completeness/feasibility/risk/scope] | [what's wrong] | [how to fix] |

### Feedback for Planner
> [Focused instructions for the planning skill — only address issues found]
```

### Step 4: Route

```
IF result == PASS:
  → Plan is ready for QA generation (QA Mode)
  → Then both plan + QA are presented to user for approval

IF result == REVISE AND round < max_curation_rounds (2):
  → Send feedback to planning skill for revision
  → Planning skill updates PLAN-XXX.md
  → Re-evaluate (back to Step 1)

IF result == REVISE AND round >= max_curation_rounds:
  → Accept plan as-is with a note about remaining concerns
  → Proceed to QA generation (QA Mode)
  → User will see the concerns in the QA checklist
```

### Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `max_curation_rounds` | 2 | Max critique→revise cycles before accepting |

---

## QA Mode: Generate QA Checklist

### Step 1: Read the Plan and Knowledge File

```
1. Read PLAN-XXX.md from .github/copilot/plans/
2. Read KNOWLEDGE-XXX.md from .github/copilot/plans/
3. Extract:
   - Summary and scope (in-scope / out-of-scope)
   - Implementation phases and tasks
   - Risk assessment
   - Testing requirements
   - Files affected
   - Code patterns and constraints from knowledge file
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

**Review all files before approving:**
- 📝 `PLAN-XXX.md` — the implementation plan
- 📖 `KNOWLEDGE-XXX.md` — the execution context cheat sheet
- ✅ `QA-XXX.md` — the quality checklist

Edit the checklist to add/remove/adjust any checks.

Reply with: ✅ approve both | 📝 revise [feedback]
```

---

## Output Format

### Curate Mode (returned to orchestrator)

```yaml
status: success
result:
  verdict: PASS | REVISE
  round: [N]
  issues_count: [N]
next_skill: "planning" (if REVISE) | "plan-reviewer:qa" (if PASS)
```

### QA Mode (returned to orchestrator)

```yaml
status: success
result:
  qa_file: "QA-XXX.md"
  checks_count: [N]
next_skill: null  # User approval required
```

---

## Rules

### ALWAYS Do

1. ✅ Read the FULL plan before critiquing
2. ✅ Read architecture docs and context for informed review
3. ✅ Cite specific plan sections when reporting issues
4. ✅ Give actionable fix suggestions for every REVISE issue
5. ✅ Generate QA checks that are specific and testable (not generic)
6. ✅ Align QA checks to project standards when available

### NEVER Do

1. ❌ Auto-pass a plan without checking all checklist categories
2. ❌ Modify the plan directly (only provide feedback for the planner)
3. ❌ Generate vague QA checks ("code is good") — be specific
4. ❌ Skip the curation checklist categories
5. ❌ Add implementation scope beyond what the plan describes

---

## Standards Integration

Before generating QA checklist, check if `.github/copilot/standards/` exists and read:
- `general.md` — Universal standards
- `[language].md` — Language-specific patterns

QA checks should incorporate these standards.
