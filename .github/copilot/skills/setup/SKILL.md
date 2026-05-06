---
name: setup
description: Initialize project context, scan codebase, generate project documentation, and bootstrap the glossary and DDD/TDD directory structure.
version: "3.0"
---

# Setup Skill

## Identity

- **Name**: setup
- **Version**: 3.0
- **Description**: Project initialization — scans codebase, bootstraps GLO, creates context memory, generates documentation. After setup the project is ready for the DDD → TDD → CODE workflow.

---

## Triggers

| Pattern | Example |
|---------|---------|
| setup, initialize, configure | "Setup the project" |
| "scan project" | "Scan the project structure" |
| First-time interaction | When `context.md` doesn't exist |

---

## Workflow

### Step 0: Re-run Guard

```
1. context.md exists? → UPDATE run (preserve user edits, refresh timestamps only)
                    NO → Fresh setup
2. docs/index.yaml exists? → Skip docs that haven't changed
3. glossary.md exists? → Skip bootstrap
```

### Step 1: Bootstrap GLO

If `.github/copilot/glossary.md` does not exist → create it with canonical terms from `glossary/SKILL.md`.
Do NOT overwrite an existing glossary (user may have added terms).

### Step 2: Scan Project

Read (depth 2-3):
- Directory structure, languages, package manifests
- Config files, CI/CD, existing docs
- **Use exact version numbers from package manifests** — never approximate

Build a capability map: `project area → evidence → candidate skill → confidence (high/medium/low)`.

### Step 3: Create Memory Files

**`context.md`** (≤80 lines — brevity is critical, loaded on every request):

```markdown
# Project Context
> Last updated: [TIMESTAMP]

## Project Identity
- **Name**: [name]
- **Type**: [web-api/cli/library/monorepo]
- **Stack**: [language + framework]
- **Stage**: [development/beta/production]

## Architecture Summary
[1-3 sentences]

## Available Skills
- GLO: .github/copilot/glossary.md
- DDD: .github/copilot/skills/ddd/SKILL.md
- TDD: .github/copilot/skills/tdd/SKILL.md
- coding: .github/copilot/skills/coding/SKILL.md
- fix: .github/copilot/skills/fix/SKILL.md

## User Preferences
(to be learned)

## Key Decisions
| Decision | Reason | Skill | Date |
|----------|--------|-------|------|
| (none)   | -      | -     | -    |
```

> If context.md would exceed 80 lines → move overflow to `instructions.md` and add a reference.

**`session.md`** — standard template (active skill: setup, pending tasks: generate docs).

**`instructions.md`** — project-specific rules derived from scan: goals, architecture constraints, coding rules (formatter/linter), testing rules (framework + run command), dev workflow (install/test/build commands). Only include sections with real evidence.

### Step 4: Generate Documentation

Only create docs with **real content** from the scan — no empty templates.

| Doc | Create when |
|-----|-------------|
| `overview.md` | Always — repo layout + quickstart |
| `architecture.md` | >5 source dirs or explicit layers |
| `tech-stack.md` | Multiple languages/frameworks detected |
| `api.md` | Routes, endpoints, or OpenAPI files detected |
| `testing.md` | Test files or test config exists |
| `development.md` | Docker, Makefile, or dev scripts detected |
| `conventions.md` | Linter/formatter configs detected |

Create `docs/index.yaml` listing only docs that were actually created.

### Step 5: Create DDD / TDD / Plans Directories

```
.github/copilot/docs/ddd/   ← DES files (DDD skill)
.github/copilot/docs/tdd/   ← TST files (TDD skill)
.github/copilot/plans/      ← PLAN + KF + QA files (planning skill)
```

No placeholder files inside — filled by downstream skills.

### Step 6: Report

```
✅ Project setup complete
- Memory: context.md, session.md, glossary.md, instructions.md
- Docs created: [list] | Skipped: [list + reason]
- Directories: docs/ddd/, docs/tdd/, plans/ — ready

Next:
1. Run skill-generator to create project-specific skills
2. New feature → @smart design [feature]  (DDD → TDD → CODE)
3. Bug → @smart fix [issue]
```

---

## Rules

- ❌ Never overwrite existing context.md without reading it first
- ❌ Never create docs with placeholder content — skip if no evidence
- ❌ Never let context.md exceed 80 lines
- ❌ Never generate custom skills — delegate to skill-generator
- ❌ Never use approximate version numbers — read package manifests

