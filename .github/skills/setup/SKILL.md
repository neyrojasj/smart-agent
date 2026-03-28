---
name: setup
description: Initialize project context, scan codebase, and generate project documentation.
version: "1.0"
---

# Setup Skill

## Identity

- **Name**: setup
- **Version**: 1.0
- **Description**: Project initialization — scans codebase, creates context memory, and generates documentation.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Keywords: setup, initialize, configure | "Setup the project" |
| "scan...project" | "Scan the project structure" |
| First-time interaction | When context.md doesn't exist |

---

## Capabilities

What this skill can do:

- ✅ Scan entire project structure
- ✅ Identify languages and frameworks
- ✅ Generate initial documentation
- ✅ Create context.md with project identity
- ✅ Create session.md for session state
- ✅ Build documentation index

---

## Dependencies

- None (this is the initialization skill)
- Creates: `context.md`, `session.md`, `.github/copilot/docs/`
- Chains to: `skill-generator` (for custom skill creation)

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

Create `.github/copilot/context.md`:

```markdown
# Project Context

> Last updated: [TIMESTAMP]

## Project Identity

- **Name**: [detected project name]
- **Type**: [web-api/cli/library/monorepo/etc]
- **Stack**: [primary language + framework]
- **Stage**: development

## User Preferences

(to be learned)

## Project-Specific Rules

(analyzing...)

## Key Decisions

| Decision | Reason | Skill | Date |
|----------|--------|-------|------|
| (none)   | -      | -     | -    |

---
*Auto-updated by Smart Orchestrator*
```

Create `.github/copilot/session.md`:

```markdown
# Session State

> Last updated: [TIMESTAMP]
> Active skill: setup
> Current task: Project initialization

## Pending Tasks

- [ ] Generate documentation
- [ ] Build documentation index

## Recent Actions (last 20)

1. [TIMESTAMP] - Started project setup

## Skill Confidence Log

(no entries)

---
*Auto-updated by Smart Orchestrator. Overwritten each session.*
```

### Step 3: Generate Documentation

Create all documentation files in `.github/copilot/docs/`:

1. `overview.md` - From README or analysis
2. `architecture.md` - From directory structure
3. `tech-stack.md` - From package files
4. `api.md` - From routes/endpoints found
5. `testing.md` - From test files/configs
6. `development.md` - From scripts/configs
7. `conventions.md` - From existing patterns
8. `decisions/index.yaml` - Empty decision registry

### Step 4: Build Documentation Index

Create `.github/copilot/docs/index.yaml` with accurate summaries.

### Step 5: Report Summary

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

## Context Memory
- `.github/copilot/context.md` — initialized
- `.github/copilot/session.md` — initialized

## Next Steps
1. Review generated documentation
2. Run **skill-generator** to detect and create project-specific skills
3. Add project-specific preferences to context.md
4. Start using @smart for your tasks
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
  index_updated: true
context_updates:
  project_identity: { ... }
  stage: "initialized"
next_skill: skill-generator  # Chain to skill generation
user_message: "[Setup summary]"
```

---

## Never Do

- ❌ Skip scanning the entire project
- ❌ Overwrite existing documentation without confirmation
- ❌ Leave context.md with placeholder values
- ❌ Generate custom skills (delegate to skill-generator skill)
