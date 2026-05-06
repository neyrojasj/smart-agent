---
name: skill-generator
description: Detect project patterns and generate project-specific skills on demand.
version: "2.0"
---

# Skill Generator Skill

## Identity

- **Name**: skill-generator
- **Version**: 1.0
- **Description**: Detects project patterns, generates custom skills, and manages the skill gap registry.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Keywords: generate skills, create skill | "Generate skills for this project" |
| "scan...skills" | "Scan for skill opportunities" |
| Missing skill detected by orchestrator | Automatic when no skill covers a change request |
| Keywords: rescan, regenerate skills | "Rescan the project for new skills" |

---

## Capabilities

What this skill can do:

- ✅ Detect patterns that warrant custom skills
- ✅ Propose skill set with evidence
- ✅ Generate custom skill files (`.github/copilot/skills/[name]/SKILL.md`)
- ✅ Register skills in `.github/copilot/skills/index.yaml`
- ✅ Track deferred gaps in `.github/copilot/docs/skills-opportunities.md`
- ✅ On-demand skill creation when orchestrator detects a gap
- ✅ Rescan project for new patterns after changes

---

## Dependencies

- `context.md` - For project identity and stack
- `.github/copilot/docs/` - For architecture understanding
- `.github/copilot/skills/index.yaml` - Current skill registry

---

## Workflow

### Step 1: Load Context

```
1. Read .github/copilot/context.md for project identity
2. Read .github/copilot/skills/index.yaml for existing skills
3. Read .github/copilot/docs/skills-opportunities.md if it exists (deferred gaps)
```

### Step 1.5: Classify Existing Skills

Before detecting new patterns, map existing skills to the patterns they cover:

```
For each skill in index.yaml:
  - Note its detected_from / trigger keywords
  - Mark the pattern as "ALREADY COVERED"

Result: a checklist of covered vs. uncovered patterns
```

This prevents duplicating skills that already exist. The goal is to find **gaps**, not recreate what's present.

> If `.github/copilot/skills/index.yaml` does not exist → treat all patterns as uncovered.

### Step 2: Detect Skill Opportunities

```
┌─────────────────────────────────────────────────────────────────────────┐
│  PATTERN DETECTION                                                      │
│                                                                         │
│  Pattern                    → Suggested Skill                           │
│  ─────────────────────────────────────────────────────────────────────  │
│  GraphQL schema files       → graphql/SKILL.md                          │
│  Database migrations        → database/SKILL.md                         │
│  CI/CD workflows            → devops/SKILL.md                           │
│  i18n/l10n files            → localization/SKILL.md                     │
│  Security configs           → security/SKILL.md                         │
│  Performance tests          → performance/SKILL.md                      │
│  Kubernetes manifests       → kubernetes/SKILL.md                       │
│  Terraform/IaC files        → infrastructure/SKILL.md                   │
│  Mobile app (React Native)  → mobile/SKILL.md                           │
│  Microservices structure    → microservices/SKILL.md                    │
└─────────────────────────────────────────────────────────────────────────┘
```

### Step 3: Propose Skill Set (MANDATORY)

Before creating files, produce a proposal table. **Only include skills NOT already in index.yaml**:

```markdown
## Skill Generation Proposal

### New Skills to Create

| Skill | Source Evidence | Why It Matters | Action |
|-------|-----------------|----------------|--------|
| `[name]/SKILL.md` | `[path(s)]` | [project-specific need] | create now / defer until requested |

### Existing Skills Verified (No Changes Needed)

| Skill | Registered Name | Status |
|-------|-----------------|--------|
| `sdk-parity/SKILL.md` | SDK Parity | ✅ active — covers pattern X |
| `codegen/SKILL.md` | Code Generation | ✅ active — covers pattern Y |
```

Rules:
- Favor project-specific skills over generic catch-all skills.
- Mark low-confidence items as `defer until requested`.
- **If no new patterns are found**: explicitly state "All detected patterns already covered by existing skills" and list them. Do NOT invent new skills.
- Do NOT create near-duplicates of existing skills — merge or extend instead.
- Do NOT re-register a skill that is already in index.yaml.

### Step 4: Generate Custom Skills

For each pattern marked `create now`, create `.github/copilot/skills/[name]/SKILL.md`:

```markdown
---
name: [skill-name]
description: [What this skill does and when to use it in this project]
version: "1.0"
---

# [Pattern Name] Skill

## Identity

- **Name**: [pattern_name]
- **Version**: 1.0
- **Description**: [Auto-generated for this project]

## Triggers

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| [detected keywords] | "[example]" |

## Capabilities

- [Capability based on pattern]

## Workflow

[Pattern-specific workflow]

## Project-Specific Context

[Include specific paths, conventions, etc. found in this project]

---
*Auto-generated by skill-generator based on project analysis*
```

### Step 5: Update Skill Registry

Add generated skills to `.github/copilot/skills/index.yaml`:

```yaml
skills:
  [new_skill]:
    file: "[name]/SKILL.md"
    name: "[Name]"
    description: "[Description]"
    priority: [next priority number]
    triggers:
      keywords: [detected keywords]
      patterns: [detected patterns]
    requires_approval: [true/false based on impact]
    auto_generated: true
    detected_from: "[what pattern triggered this]"
```

**Skill Priority Ranges**:
- 1-9: Core skills (reserved)
- 10-19: Technology skills
- 20-29: Framework skills
- 30-39: Domain skills
- 40+: Custom/project-specific

### Step 6: Register Deferred Gaps

Store deferred items in `.github/copilot/docs/skills-opportunities.md`:

```markdown
## Deferred Skill Gaps

- `[name]/SKILL.md`: [short reason], trigger when request mentions [keywords]
```

### Step 7: Report Summary

```markdown
✅ **Skill Generation Complete**

## Skills Created

| Skill | Detected From | Triggers |
|-------|---------------|----------|
| [skill] | [pattern] | [keywords] |

## Skills Verified (Already Exist)

| Skill | Registered Name | Notes |
|-------|-----------------|-------|
| [skill] | [name in index.yaml] | Current, no update needed |

## Deferred Gaps

| Skill | Reason | Trigger Keywords |
|-------|--------|------------------|
| [skill] | [reason] | [keywords] |

## Registry Updated
`.github/copilot/skills/index.yaml` — [N] skills added, [M] verified existing, [K] deferred
```

> **No new skills case**: If all patterns detected are already covered: output only "Skills Verified" + "Deferred Gaps" sections. Do not output an empty "Skills Created" section.

---

## On-Demand Missing Skill Workflow

When invoked because a live request exposed missing capability:

```
1. Read .github/copilot/docs/skills-opportunities.md and index.yaml
2. Determine whether a deferred skill already matches the request
3. If yes → generate that skill now and register it
4. If no → synthesize a new project-specific skill from request + project evidence
5. Add triggers for this request family (keywords + patterns)
6. Register in index.yaml
7. Append to skills-opportunities.md under "Generated On Demand":
   - `[name]/SKILL.md`: generated for "[request summary]", subtype: [capability], evidence: [paths]
8. Signal orchestrator to re-run skill matching
```

Rules:
- Use narrow skill names that map to the subtype (e.g., `test-contract/SKILL.md`, not `advanced-testing/SKILL.md`)
- Never proceed with direct implementation until a suitable skill exists

---

## Rescan Workflow

When user requests "rescan" or "regenerate skills":

```
1. Read current context.md and index.yaml
2. Run Step 1.5 (Classify Existing Skills) — catalog all currently covered patterns
3. Scan project for NEW patterns not covered by any existing skill
4. Follow Steps 3-7 for new patterns only — never re-create existing skills
5. Report: N new skills created, M existing skills verified, K new deferred gaps
```

> Rescan does NOT touch existing skills — it only adds. To update an existing skill, use the on-demand workflow or point the coding skill at the specific SKILL.md.

---

## Output Format

Return to orchestrator:

```yaml
status: success | error
result:
  skills_created: [list]              # newly created SKILL.md files
  skills_verified_existing: [list]    # patterns already covered, no action taken
  skills_deferred: [list]             # patterns found but not yet created
  registry_updated: true
context_updates:
  recent_actions:
    - "Generated [N] new skills, verified [M] existing, deferred [K]"
next_skill: null
user_message: "[Generation summary]"
```

---

## Never Do

- ❌ Generate skills without detected patterns or evidence
- ❌ Create duplicate or near-duplicate skills
- ❌ Skip the proposal step — always propose before creating
- ❌ Forget to update skill registry
- ❌ Create speculative skills without concrete project evidence
- ❌ Re-create a skill that already exists in index.yaml (use RescanWorkflow or on-demand update instead)
- ❌ Skip Step 1.5 — always classify existing skills before proposing new ones
- ❌ Output an empty "Skills Created" section when no new skills were needed — report "verified existing" instead
