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
- ✅ Select and generate relevant documentation from catalog
- ✅ Create context.md with project identity
- ✅ Create session.md for session state
- ✅ Build documentation index

---

## Dependencies

- None (this is the initialization skill)
- Creates: `context.md`, `session.md`, `.github/copilot/docs/` (selective)
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

### Step 1.1: Build Capability Map

From the scan, produce a capability map before generating any docs or skills:

| Project Part | Evidence (files/dirs) | Capability | Candidate Skill | Confidence |
|--------------|----------------------|------------|-----------------|------------|
| [part] | [path examples] | [capability] | `[name]/SKILL.md` | high/medium/low |

Rules:
- `high` confidence + clear evidence → propose skill creation to user
- `medium` / `low` → record as deferred gap in `docs/skills-opportunities.md`
- This map drives both documentation scope and skill-generator proposals

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

### Step 3: Select and Generate Documentation from Catalog

Do NOT create all docs blindly. Use this catalog to decide which docs to generate based on scan evidence:

#### Document Catalog

| Document | Create When | Evidence Required |
|----------|-------------|-------------------|
| `overview.md` | **Always** | README or package description exists |
| `architecture.md` | >5 source dirs OR explicit layers (routes/, handlers/, models/) | Directory structure scan |
| `tech-stack.md` | Multiple languages or frameworks detected | Package files, config files |
| `api.md` | Routes, endpoints, OpenAPI/Swagger files detected | `routes/`, `controllers/`, `*.openapi.*`, `swagger.*` |
| `testing.md` | Test files or test config exists | `tests/`, `__tests__/`, `*.test.*`, `*.spec.*`, jest/pytest/cargo-test config |
| `development.md` | Docker, Makefile, dev scripts detected | `Dockerfile`, `docker-compose.*`, `Makefile`, `scripts/` |
| `conventions.md` | Linter/formatter configs OR strong patterns found | `.eslintrc`, `.prettierrc`, `rustfmt.toml`, `.editorconfig` |
| `decisions/index.yaml` | **Defer** — created on first architectural decision, not upfront | (none at setup time) |

#### Selection Process

```
1. For each document in the catalog:
   a. Check if evidence exists from the scan
   b. If YES → mark for creation
   c. If NO → skip (do not create empty templates)
2. Log which docs were created and which were skipped (and why)
```

#### Generation Rules

- Generate docs with **real content** derived from the scan — not placeholder templates
- If a doc would be mostly empty, skip it
- `overview.md` is the only mandatory doc (always created)
- For each generated doc, fill it with actual project data (file paths, detected patterns, real commands)

### Step 4: Build Documentation Index

Create `.github/copilot/docs/index.yaml` listing **only the docs that were actually created**.

```yaml
version: 1
last_updated: "[TIMESTAMP]"

project:
  name: "[detected]"
  type: "[detected]"
  primary_language: "[detected]"
  framework: "[detected]"
  stage: development

documents:
  # Only list docs that were created — omit skipped docs
  overview:
    file: "overview.md"
    summary: "[real summary]"
    keywords: [...]
  # ... (only created docs)

skipped:
  # Record what was NOT created and why
  - doc: "api.md"
    reason: "No routes or endpoints detected"
  # ...
```

> The index must reflect reality. Do not list docs that don't exist.

### Step 5: Populate Project Instructions

Write `.github/copilot/instructions.md` with real content derived from the scan.

This file is **user-owned** — it is separate from `.github/copilot-instructions.md` (which governs agent behavior). This file captures **project-specific rules** that the project owner wants enforced on every task.

Generate from what was discovered:

```markdown
# Project Instructions

> Auto-generated by Setup skill on [TIMESTAMP]. Edit freely — this file is yours.

## Project Goals

- [Derived from README/package description]

## Architecture Constraints

- [Derived from directory structure and module boundaries]
- [Derived from forbidden patterns observed in code or config]

## Coding Rules

- [Language/framework-specific rules derived from existing code style]
- [Error handling and logging patterns found in codebase]

## Testing Rules

- [Test levels found (unit/integration/e2e)]
- [Naming conventions observed in test files]

## Documentation Rules

- Keep docs in sync with behavior and API/config changes.
- Update examples/snippets in the same change as code updates.

---

Last updated: [TIMESTAMP]
```

> If the codebase is empty or a brand-new project, generate sensible defaults based on the detected stack and leave `[TBD]` markers where no evidence exists.

### Step 6: Report Summary

```markdown
✅ **Project Setup Complete**

## Project Identity
- **Name**: [name]
- **Type**: [type]
- **Stack**: [stack]

## Documentation Created
[List only docs that were actually generated]
- overview.md
- [other docs based on evidence]

## Documentation Skipped
[List skipped docs with reason]
- api.md — no routes/endpoints detected
- [other skipped docs]

## Context Memory
- `.github/copilot/context.md` — initialized
- `.github/copilot/session.md` — initialized
- `.github/copilot/instructions.md` — populated with project-specific rules (user-editable)

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
