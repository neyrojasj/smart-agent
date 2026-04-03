---
name: planning
description: Create implementation plans, architectural options, and phased execution strategy. All plans are written to disk as markdown files.
version: "2.0"
---

# Planning Skill

## Identity

- **Name**: planning
- **Version**: 2.0
- **Description**: Creates implementation plans, architectural decisions, and strategic approaches for complex tasks. All plans — regardless of size — are persisted to disk as `.md` files in `.github/copilot/plans/`.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Keywords: plan, design, architect, approach | "Create a plan for user auth" |
| "how should I..." | "How should I structure the API?" |
| "design...for" | "Design a solution for caching" |
| Large scope tasks | "Add a complete payment system" |

---

## Capabilities

What this skill can do:

- ✅ Analyze request complexity (small/medium/big changes)
- ✅ Ask clarifying questions before planning
- ✅ Create detailed implementation plans with phases
- ✅ Identify files and components affected
- ✅ Assess risks and propose mitigations
- ✅ Provide rollback strategies
- ✅ Make architectural decisions (ADRs)

---

## Dependencies

- `context.md` - For session context and project identity
- `.github/copilot/docs/` - For existing architecture understanding
- `.github/copilot/standards/` - For coding standards to plan against
- `coding.skill` - Chains to for implementation

---

## Workflow

### Step 1: Load Project Context

```
Read from context.md:
- Project type and stack
- Existing decisions
- User preferences
```

### Step 2: Analyze Change Size

**SMALL** (<100 lines)
- Brief PLAN-XXX.md with summary, scope, and tasks
- Still written to disk (all plans go to disk)

**MEDIUM** (100-500 lines)
- Standard PLAN-XXX.md with phases and risks
- 2-3 phases max

**BIG** (>500 lines)
- Full PLAN-XXX.md document
- Multiple phases with milestones
- Risk assessment + rollback strategy

ALL sizes → Written to `.github/copilot/plans/PLAN-XXX.md`

### Step 3: Ask Clarifying Questions (MANDATORY)

Before creating ANY plan, ensure you understand:

```markdown
🤔 **Before I create a plan, I have some questions:**

1. **[Scope]**: [Clarify boundaries]
2. **[Behavior]**: [Clarify expected outcomes]
3. **[Constraints]**: [Identify limitations]
4. **[Integration]**: [Understand dependencies]

Please answer these so I can create an accurate plan.
```

**Question categories to consider:**

| Category | Example Questions |
|----------|-------------------|
| **Scope** | "Should this include X? What about Y?" |
| **Behavior** | "What should happen when Z occurs?" |
| **Constraints** | "Are there performance/security requirements?" |
| **Integration** | "How should this interact with existing feature A?" |
| **Edge Cases** | "What if the user does X? What about empty input?" |
| **Priority** | "Which aspects are must-have vs nice-to-have?" |

### Step 4: Present Multiple Solutions (When Applicable)

When multiple valid approaches exist:

```markdown
🔀 **Multiple Solutions Available**

---

**Option A: [Name]** ⭐ *Recommended*
- **Approach**: [Description]
- **Pros**: [Benefits]
- **Cons**: [Drawbacks]
- **Effort**: [Low/Medium/High]

---

**Option B: [Name]**
- **Approach**: [Description]
- **Pros**: [Benefits]
- **Cons**: [Drawbacks]
- **Effort**: [Low/Medium/High]

---

**My Recommendation**: Option [X] because [reasoning].

Which approach would you prefer?
```

### Step 5: Create Plan Document

**ALL plans are written to disk** in `.github/copilot/plans/PLAN-XXX.md`. No inline-only plans.

For SMALL changes, use a simplified version (summary, scope, tasks, testing, learning checklist).
For MEDIUM and BIG changes, use the full template:

```markdown
# PLAN-XXX: [Title]

## Status: pending_review

## Summary

[2-3 sentence description]

## Background

[Context and why this change is needed]

## Scope

### In Scope
- [Item 1]
- [Item 2]

### Out of Scope
- [Item 1]

## Implementation Phases

### Phase 1: [Name]
**Estimated changes**: ~XX lines

Files affected:
- `path/to/file.ts` - [create/modify/delete] - [what changes]

Tasks:
1. [Task 1]
2. [Task 2]

### Phase 2: [Name]
[Same structure]

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| [Risk] | Low/Med/High | Low/Med/High | [How to mitigate] |

## Rollback Strategy

[How to undo if something goes wrong]

## Testing Requirements

- [ ] Unit tests for [component]
- [ ] Integration tests for [flow]

## Documentation Updates

- [ ] Update `architecture.md` if structure changes
- [ ] Update `api.md` if endpoints change

## Post-Execution Learning Checklist

> Reviewed by the evaluator (learning mode) after this plan completes.

- [ ] **Skills used**: [list skills invoked] — review for missing context or workflow gaps
- [ ] **Docs referenced**: [list docs read] — check if still accurate after changes
- [ ] **Context relied on**: [list context.md sections used] — verify/update if changed
- [ ] **Discoveries**: [note any new patterns, conventions, or architecture insights]
- [ ] **Skill updates needed**: [flag if any skill lacked workflow steps for this task]

---
*Created: [DATE] | Status: pending_review*
```

### Step 5.5: Generate Knowledge File

Create `.github/copilot/plans/KNOWLEDGE-XXX.md` (same XXX as the plan) immediately after the plan. This file is the **execution context cheat sheet** — everything an AI agent needs to implement the plan without re-discovering context.

```markdown
# KNOWLEDGE-XXX: [Plan Title] — Execution Context

> **Plan**: PLAN-XXX.md
> **Generated**: [date]
>
> 📖 Re-read this file whenever you lose context mid-execution.

---

## Project Context Snapshot

- **Project**: [name from context.md]
- **Stack**: [language + framework]
- **Relevant standards**: [list which standards/*.md apply]

## Architecture Context

[Summarize the relevant parts of architecture.md — only what matters for THIS plan]

- Module boundaries relevant to this change
- Data flow through affected components
- Integration points the change touches

## Key Files & Their Roles

| File | Purpose | How This Plan Affects It |
|------|---------|--------------------------|
| `path/to/file` | [what it does] | [create/modify/delete — what changes] |

## Code Patterns to Follow

[Extract from standards and existing code — the specific patterns the agent must follow]

- Error handling pattern: [specific to this project/language]
- Naming convention: [relevant examples from codebase]
- Import/module pattern: [how this project organizes imports]

## Existing Code Snippets

[Include actual code snippets from the codebase that the agent will need to reference or extend — function signatures, type definitions, interfaces, config structures]

\`\`\`[language]
// Example: Current interface the agent needs to extend
[relevant code snippet]
\`\`\`

## Constraints & Gotchas

- [Things that are easy to get wrong in this codebase]
- [Non-obvious dependencies or side effects]
- [Environment or config requirements]

## Dependencies Between Phases

[If multi-phase plan: what each phase produces that later phases need]

- Phase 1 creates: [artifact] → used by Phase 2 in [file]
- Phase 2 creates: [artifact] → used by Phase 3 in [file]

---
*Auto-generated by planning skill. Edit if context is missing or wrong.*
```

**Rules for KNOWLEDGE-XXX.md:**
- Include REAL code snippets from the codebase, not placeholders
- Include REAL file paths discovered during planning
- Keep it focused — only context relevant to THIS plan
- If the plan is SMALL, the knowledge file can be brief (Project Context + Key Files + Code Patterns)
- For MEDIUM/BIG plans, include all sections

### Step 5b: Revise Plan from Plan-Reviewer Feedback

When the plan-reviewer (curate mode) returns a REVISE verdict, the planning skill receives structured feedback and revises the plan:

1. Receive evaluator feedback (issues table + suggestions)
2. Read the current PLAN-XXX.md
3. Address EACH issue from the evaluator's feedback:
   - Completeness gaps → Add missing sections/detail
   - Feasibility issues → Reorder phases or adjust scope
   - Risk gaps → Add to risk assessment
   - Scope creep → Trim to what was requested
4. Update PLAN-XXX.md in place (same file, same ID)
5. Return to evaluator for re-critique

**Rules**: Preserve plan ID across revisions. Address ALL issues. Don't add scope beyond what evaluator requested. Fix issues without bloating.

### Step 6: Update State

Update `.github/copilot/plans/state.yaml`:

```yaml
plans:
  PLAN-XXX:
    title: "[Title]"
    status: pending_review
    created: "[DATE]"
    updated: "[DATE]"
    knowledge: "KNOWLEDGE-XXX.md"
```

### Step 7: Request Approval

```markdown
📋 **Plan Ready for Review**

I've created **PLAN-XXX: [Title]**

**Summary:** [Brief summary]

**Phases:** [Number of phases]
**Files affected:** [Count]
**Estimated effort:** [Small/Medium/Large]

📄 **Plan:** `.github/copilot/plans/PLAN-XXX.md`
📖 **Knowledge:** `.github/copilot/plans/KNOWLEDGE-XXX.md`

Reply with: ✅ approve | ❌ reject | 📝 revise [feedback]
```

---

## Plan States

| State | Description |
|-------|-------------|
| `draft` | Being created |
| `pending_review` | Ready for approval |
| `approved` | Ready to implement |
| `in_progress` | Being implemented |
| `completed` | Successfully done |
| `archived` | Done and archived |
| `rejected` | Not proceeding |

---

## Step 8: Capture Decisions (After Completion)

When a plan reaches `completed` status, extract key architectural or design decisions into `docs/decisions/`:

When a plan reaches `completed` status, check:

1. Did the plan choose between multiple approaches? → DEC
2. Did the plan introduce a new pattern or convention? → DEC
3. Did the plan add/replace a dependency? → DEC
4. Did the plan change the project structure? → DEC

If ANY are true → Create `docs/decisions/DEC-XXX.md`
If NONE → Skip (not every plan produces a decision)

For each captured decision:
1. Create `docs/decisions/DEC-XXX.md` using the decision template
2. Update `docs/decisions/index.yaml` with new entry
3. Update `context.md` Key Decisions table if it affects project identity

This ensures plan rationale survives in git after the `plans/` directory is cleaned up.

---

## Output Format

Return to orchestrator:

```yaml
status: success | needs_input | error
result: 
  plan_id: "PLAN-XXX"
  plan_status: "pending_review"
  files_affected: [list]
  estimated_size: "small|medium|big"
context_updates:
  active_task: "Planning: [description]"
  pending_tasks:
    - "Implement PLAN-XXX (waiting approval)"
next_skill: null  # or "coding" if auto-approved
user_message: "[Message to show user]"
```

---

## Never Do

- ❌ Create a plan without asking clarifying questions first
- ❌ Choose an approach autonomously when multiple valid options exist
- ❌ Implement anything without explicit approval
- ❌ Skip the size analysis
- ❌ Keep a plan only in the conversation — ALL plans go to disk as `.md` files
- ❌ Create a plan without the Post-Execution Learning Checklist section
- ❌ Forget to update state.yaml
- ❌ Complete a plan without checking if decisions should be captured in `docs/decisions/`
- ❌ Ignore evaluator feedback during revision — address every issue

---

## Standards Integration

Before planning, check if `.github/copilot/standards/` exists and read:
- `general.md` - Universal standards
- `[language].md` - Language-specific patterns

Plans should align with these standards.
