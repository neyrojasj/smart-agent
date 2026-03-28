---
name: planning
description: Create implementation plans, architectural options, and phased execution strategy.
version: "1.0"
---

# Planning Skill

## Identity

- **Name**: planning
- **Version**: 1.0
- **Description**: Creates implementation plans, architectural decisions, and strategic approaches for complex tasks.

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

```
┌─────────────────────────────────────────────────────────────────────────┐
│  AUTOMATIC SIZE DETECTION                                               │
│                                                                         │
│  📏 SMALL (<100 lines):                                                 │
│     → Quick inline plan, implement directly                             │
│                                                                         │
│  📐 MEDIUM (100-500 lines):                                             │
│     → Brief implementation plan                                         │
│     → 2-3 phases max                                                    │
│                                                                         │
│  📊 BIG (>500 lines):                                                   │
│     → Full PLAN-XXX.md document                                         │
│     → Multiple phases with milestones                                   │
│     → Risk assessment                                                   │
│     → Rollback strategy                                                 │
└─────────────────────────────────────────────────────────────────────────┘
```

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

For MEDIUM and BIG changes, create `.github/copilot/plans/PLAN-XXX.md`:

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

---
*Created: [DATE] | Status: pending_review*
```

### Step 6: Update State

Update `.github/copilot/plans/state.yaml`:

```yaml
plans:
  PLAN-XXX:
    title: "[Title]"
    status: pending_review
    created: "[DATE]"
    updated: "[DATE]"
```

### Step 7: Request Approval

```markdown
📋 **Plan Ready for Review**

I've created **PLAN-XXX: [Title]**

**Summary:** [Brief summary]

**Phases:** [Number of phases]
**Files affected:** [Count]
**Estimated effort:** [Small/Medium/Large]

📄 **Full plan:** `.github/copilot/plans/PLAN-XXX.md`

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
- ❌ Create PLAN files for small changes
- ❌ Forget to update state.yaml

---

## Standards Integration

Before planning, check if `.github/copilot/standards/` exists and read:
- `general.md` - Universal standards
- `[language].md` - Language-specific patterns

Plans should align with these standards.
