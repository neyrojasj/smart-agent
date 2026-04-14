---
name: setup
description: Initialize project context, scan codebase, and generate project documentation.
version: "2.0"
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

### Step 0: Check Existing State (Re-run Guard)

Before scanning, check whether setup has already been run:

```
1. Does .github/copilot/context.md exist?
   YES → Read it. Note project identity already detected.
          Continue as UPDATE run — preserve user edits; only refresh stale sections.
          Sections to always refresh: Last updated timestamp, session.md state.
          Sections to preserve (user-owned): Project-Specific Rules, Key Decisions, User Preferences.
   NO  → Fresh setup — create everything from scratch.

2. Does .github/copilot/docs/ exist with content?
   YES → Read docs/index.yaml. Note which docs already exist.
          Skip regenerating docs that are current (only regenerate if project changed significantly).
   NO  → Generate all docs fresh.

3. Does .github/skills/index.yaml exist?
   YES → Read it. Pass skill list to skill-generator as "already exists" baseline.
   NO  → skill-generator will start fresh.
```

> On re-run: always update timestamps, context updates only if something genuinely changed.

### Step 1: Scan Project Structure

```
Analyze:
- Root directory structure (depth 2-3)
- All programming languages (by extension/content)
- Package files (package.json, Cargo.toml, pyproject.toml, go.mod)
- Configuration files (.env.example, configs, .editorconfig)
- Build configuration (webpack, vite, tsconfig, esbuild config)
- CI/CD configuration (.github/workflows)
- Existing documentation
```

**Read package manifests for exact versions** — do not approximate:
- `package.json` → dependencies{} and devDependencies{} with semver
- `pyproject.toml` → [project.dependencies] and [project.optional-dependencies]
- `go.mod` → require section
- `global.json` / `Directory.Packages.props` (.NET)
- `Cargo.toml` → [dependencies]

Record actual version constraints in tech-stack.md. "React ^18.0.0" is better than "React 18+".

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

Create or update `.github/copilot/context.md`.

> **SIZE CONSTRAINT**: context.md MUST stay under 80 lines. It is loaded on every request — brevity is critical.
> - Project identity, architecture summary, key rules → context.md
> - Detailed coding style, testing rules, debug guides, language specifics → instructions.md
> - If context.md would exceed 80 lines, move the overflow sections to instructions.md and add a reference in context.md.

```markdown
# Project Context

> Last updated: [TIMESTAMP]

## Project Identity

- **Name**: [detected project name]
- **Type**: [web-api/cli/library/monorepo/etc]
- **Stack**: [primary language + framework, e.g. TypeScript / Node.js 22]
- **Stage**: [development/beta/production]

## Architecture Summary

[1-3 sentence description of the main architecture pattern detected]

## User Preferences

(to be learned)

## Project-Specific Rules

- [Critical rule 1 — e.g. "Generated code in */generated/ is never hand-edited"]
- [Critical rule 2 — e.g. ".NET tests must not use InternalsVisibleTo"]
[Keep to ≤5 critical rules. Remaining rules go in instructions.md]

## Key Decisions

| Decision | Reason | Skill | Date |
|----------|--------|-------|------|
| (none)   | -      | -     | -    |

---
*Auto-updated by Smart Orchestrator*
```

> After writing context.md, count lines. If over 80: move "Project-Specific Rules" body to instructions.md and replace with: `See instructions.md — Coding & Testing Rules.`

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

#### Per-Document Quality Standards

| Document | Minimum Content Required |
|----------|-------------------------|
| `overview.md` | Repo layout tree, quickstart snippet, auth options (if any), key concepts |
| `architecture.md` | Architecture diagram (text art OK), layer table with responsibilities, source-file map |
| `tech-stack.md` | **Actual version numbers** from package manifests; per-SDK dependency table; build/test framework per language |
| `testing.md` | Test locations per SDK with run command; **≥2 code examples** of key test patterns (e.g. streaming, auth); E2E setup if present |
| `development.md` | Full setup steps; per-language build commands; code generation workflow if present |
| `conventions.md` | Cross-language naming table (if multi-SDK); per-language formatter + linter config file references |

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

Generate from what was discovered. Include **all applicable sections** below:

```markdown
# Project Instructions

> Auto-generated by Setup skill on [TIMESTAMP]. Edit freely — this file is yours.

## Project Goals

- [Derived from README/package description]
- [Include primary use case and target users if detectable]

## Architecture Constraints

- [Derived from directory structure and module boundaries]
- [Forbidden patterns observed in code or config]
- [Protocol/API constraints enforced by the project]

## Coding Rules

### [Primary Language]
- **Formatter**: [tool + config file]
- **Linter**: [tool + config file]
- [Framework-specific conventions]
- [Private/public conventions]

### [Secondary Language] (if applicable)
- [Same structure — one block per language]

## Testing Rules

- [Test framework per SDK/language + discovery command]
- [E2E test setup if detected]
- [Coverage expectations]
- [Key test patterns (e.g. must assert both delta AND final events, cleanup teardown rules)]

## Documentation Rules

- Keep docs in sync with behavior and API/config changes.
- Update examples/snippets in the same change as code updates.
- [Changelog auto-generated? Note it if detected]
- [Link validation if CI workflow detected]

## Development Workflow

1. [Install command]
2. [Format/lint command]
3. [Test command]
4. [Build or generate command if applicable]
[Include actual commands from package.json scripts, Makefile, justfile, etc.]

## Debugging & Troubleshooting

- **[Common issue 1]**: [Cause + fix — e.g. stale generated code]
- **[Common issue 2]**: [Cause + fix — e.g. protocol version mismatch]
[Derive from README, CONTRIBUTING.md, or docs/troubleshooting/ if present]

## Version & Release

- [Versioning strategy — semver + registry names]
- [How to trigger a release — tag, CI, manual]
- [Changelog policy — manual or auto-generated]
[Skip this section if no package publishing detected]

## Skills Available

This project has custom skills for specialized workflows:
[List each generated skill with a one-line description]
- **`[skill-name]/SKILL.md`**: [what it does]
[This section is filled in after skill-generator runs — update it then]

---

Last updated: [TIMESTAMP]
```

> Sections to ALWAYS include: Goals, Architecture Constraints, Coding Rules, Testing Rules, Documentation Rules, Development Workflow.
> Sections to include ONLY when evidence exists: Debugging, Version & Release, Skills Available.
> If the codebase is empty or brand-new: generate sensible defaults and leave `[TBD]` where no evidence exists.

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
- ❌ Overwrite existing documentation without reading it first (Step 0)
- ❌ Leave context.md with placeholder values
- ❌ Generate custom skills (delegate to skill-generator skill)
- ❌ Let context.md exceed 80 lines without moving overflow to instructions.md
- ❌ Use approximate version numbers in tech-stack.md — always read package manifests
- ❌ Generate testing.md without code examples when test files exist
- ❌ Omit the "Skills Available" section in instructions.md after skill-generator has run
