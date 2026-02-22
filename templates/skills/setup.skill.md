---
name: setup
description: Initialize project context and generate project-specific skills from codebase patterns.
---

# Setup Skill

## Identity

- **Name**: setup
- **Version**: 1.0
- **Description**: Project initialization, configuration, and automatic skill generation based on project patterns.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Keywords: setup, initialize, configure | "Setup the project" |
| "scan...project" | "Scan the project structure" |
| "generate...skills" | "Generate skills for this project" |
| First-time interaction | When context.md doesn't exist |

---

## Capabilities

What this skill can do:

- ✅ Scan entire project structure
- ✅ Identify languages and frameworks
- ✅ Generate initial documentation
- ✅ Create context.md with project identity
- ✅ Build documentation index
- ✅ **Propose project-specific skills** based on capability gaps
- ✅ **Generate custom skills** based on project patterns and requests

---

## Dependencies

- None (this is the initialization skill)
- Creates: `context.md`, `.copilot/docs/`, skill files

---

## Workflow

### Step 1: Scan Project Structure

```
Analyze:
- Root directory structure
- All programming languages (by extension/content)
- Package files (package.json, Cargo.toml, pyproject.toml, go.mod)
- Configuration files (.env.example, configs)
- Build configuration (webpack, vite, tsconfig)
- CI/CD configuration (.github/workflows)
- Existing documentation
```

### Step 2: Initialize Context Memory

Create/update `.copilot/context.md`:

```markdown
# Agent Context Memory

> Last updated: [TIMESTAMP]
> Active skill: setup
> Current task: Project initialization

## Project Identity

- **Name**: [detected project name]
- **Type**: [web-api/cli/library/monorepo/etc]
- **Stack**: [primary language + framework]
- **Stage**: development

## Current Session

### Active Conversation
- **Started**: [timestamp]
- **Topic**: Project initialization
- **Decisions Made**: (awaiting scan results)

### Pending Tasks
- [x] Scan project structure
- [ ] Generate documentation
- [ ] Create skill registry
- [ ] Generate custom skills

### Recent Actions
1. [timestamp] - Started project setup

## Learned Context

### User Preferences
(to be learned)

### Project-Specific Rules
(analyzing...)

---
*Auto-updated by Smart Orchestrator*
```

### Step 3: Generate Documentation

Create all documentation files in `.copilot/docs/`:

1. `overview.md` - From README or analysis
2. `architecture.md` - From directory structure
3. `tech-stack.md` - From package files
4. `api.md` - From routes/endpoints found
5. `testing.md` - From test files/configs
6. `development.md` - From scripts/configs
7. `conventions.md` - From existing patterns
8. `decisions/index.yaml` - Empty decision registry

### Step 4: Build Documentation Index

Create `.copilot/docs/index.yaml` with accurate summaries.

### Step 5: Detect Custom Skill Opportunities

```
┌─────────────────────────────────────────────────────────────────────────┐
│  SKILL GENERATION - PATTERN DETECTION                                   │
│                                                                         │
│  Scan for patterns that warrant custom skills:                          │
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

### Step 5.1: Propose Skill Set (MANDATORY)

Before creating files, produce a proposal table:

```markdown
## Proposed Skill Set

| Skill | Source Evidence | Why It Matters | Action |
|-------|------------------|----------------|--------|
| `[name]/SKILL.md` | `[path(s)]` | [project-specific need] | create now / defer until requested |
```

Rules:
- Favor project-specific skills over generic catch-all skills.
- Mark low-confidence items as `defer until requested`.
- If no extra skills are needed, explicitly state why.

### Step 6: Generate Custom Skills

For each detected pattern marked `create now`, create a custom skill:

```markdown
---
name: [skill-name]
description: [What this skill does and when to use it in this project]
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
*Auto-generated by setup.skill based on project analysis*
```

### Step 7: Update Skill Registry

Add generated skills to `.github/skills/index.yaml`:

```yaml
skills:
  # ... existing skills ...
  
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

### Step 7.1: Register Deferred Skills as Gaps (NEW)

If proposal includes `defer until requested`, store this in `.copilot/docs/skills-opportunities.md` so the orchestrator can create those skills on demand.

Required note format:

```markdown
## Deferred Skill Gaps

- `[name]/SKILL.md`: [short reason], trigger when request mentions [keywords]
```

### Step 8: Report Summary

```markdown
✅ **Project Setup Complete**

## Project Identity
- **Name**: [name]
- **Type**: [type]
- **Stack**: [stack]

## Documentation Created
- overview.md
- architecture.md
- tech-stack.md
- api.md
- testing.md
- development.md
- conventions.md
- decisions/index.yaml

## Custom Skills Generated

| Skill | Detected From | Triggers |
|-------|---------------|----------|
| [skill] | [pattern] | [keywords] |

## Context Memory
Updated at: `.copilot/context.md`

## Next Steps
1. Review generated documentation
2. Add project-specific preferences to context.md
3. Start using @smart for your tasks
```

---

## Skill Generation Templates

### GraphQL Skill

When `.graphql` or `schema.graphql` detected:

```markdown
# GraphQL Skill

## Identity
- **Name**: graphql
- **Description**: Manages GraphQL schema, resolvers, and queries

## Triggers
| Pattern | Example |
|---------|---------|
| Keywords: graphql, schema, resolver, query, mutation | "Add a new GraphQL query" |

## Capabilities
- Generate GraphQL types
- Create resolvers
- Generate queries/mutations
- Update schema

## Workflow
1. Read existing schema
2. Understand data model
3. Generate type-safe code
4. Update schema.graphql
5. Create/update resolvers
```

### Database Skill

When `migrations/` or database config detected:

```markdown
# Database Skill

## Identity
- **Name**: database
- **Description**: Manages database migrations and models

## Triggers
| Pattern | Example |
|---------|---------|
| Keywords: migration, database, model, schema | "Create a new migration" |

## Capabilities
- Generate migrations
- Create models/entities
- Manage seed data
- Handle schema changes

## Workflow
1. Understand current schema
2. Generate migration file
3. Update models
4. Create seed data if needed
```

### DevOps Skill

When `.github/workflows/` or CI config detected:

```markdown
# DevOps Skill

## Identity
- **Name**: devops
- **Description**: Manages CI/CD workflows and deployment

## Triggers
| Pattern | Example |
|---------|---------|
| Keywords: ci, cd, deploy, pipeline, workflow | "Add a deployment workflow" |

## Capabilities
- Create CI/CD workflows
- Configure deployments
- Set up environments
- Manage secrets configuration

## Workflow
1. Read existing workflows
2. Understand deployment targets
3. Generate workflow files
4. Configure environments
```

---

## Output Format

Return to orchestrator:

```yaml
status: success | error
result:
  project:
    name: "[name]"
    type: "[type]"
    stack: "[stack]"
  docs_created: [list]
  skills_generated: [list]
  index_updated: true
context_updates:
  project_identity: { ... }
  stage: "initialized"
next_skill: documentation  # To sync docs
user_message: "[Setup summary]"
```

---

## Never Do

- ❌ Skip scanning the entire project
- ❌ Generate skills without detected patterns
- ❌ Overwrite existing documentation without confirmation
- ❌ Create duplicate skills
- ❌ Forget to update skill registry
- ❌ Leave context.md with placeholder values

---

## Rescan Workflow

When user requests "rescan" or "regenerate":

```
1. Read current context.md
2. Scan project for changes
3. Identify new patterns
4. Generate additional skills if needed
5. Update documentation for changes
6. Update skill registry
7. Report what changed
```

## On-Demand Missing Skill Workflow (NEW)

When a user requests a change and no existing skill is appropriate:

```
1. Read .copilot/docs/skills-opportunities.md and index.yaml
2. Determine whether a deferred skill already matches the request
3. If yes, generate that skill now and register it
4. If no, synthesize a new project-specific skill from request + project evidence
5. Add triggers for this request family (keywords + patterns)
6. Re-run routing and execute the newly created skill
```

Never proceed with direct implementation until a suitable skill exists.
