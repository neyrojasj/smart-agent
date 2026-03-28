# Setup Project - Initialize Agent Memory

Execute this prompt to fully initialize or update the Smart Agent's memory (documentation) for this project.

## ⚠️ CRITICAL: Memory Location

**ALL documentation MUST be created in `.github/copilot/docs/`**

This folder is the agent's persistent memory - the single source of truth for project understanding.

## Instructions

Perform ALL of the following steps in order. Do not skip any steps.

---

## Step 1: Analyze Project Structure

Scan the entire project and identify:
- Root directory structure and key folders
- All programming languages used (by file extension and content)
- Package/dependency files (package.json, Cargo.toml, pyproject.toml, go.mod, etc.)
- Configuration files (.env.example, config files, etc.)
- Build/bundler configuration (webpack, vite, tsconfig, etc.)
- CI/CD configuration (.github/workflows, .gitlab-ci.yml, etc.)
- Existing documentation (README.md, docs/, etc.)
- Distinct project domains/capabilities (auth, billing, messaging, reporting, data pipeline, etc.)

### 1.1 Build a Capability Map (NEW)

From the scan, map project parts to capabilities and candidate skills:

| Project Part | Evidence (files/dirs) | Capability | Candidate Skill | Confidence |
|--------------|------------------------|------------|-----------------|------------|
| [part] | [path examples] | [capability] | `[name]/SKILL.md` | [high/med/low] |

This map is required and will drive both setup-time skill proposals and future on-demand skill creation.

---

## Step 2: Create/Update Memory Files in .github/copilot/docs/

### 2.1 Overview (.github/copilot/docs/overview.md)

```markdown
# [Project Name]

> [One-line description]

## Purpose

[2-3 sentences about what this project does and why it exists]

## Quick Start

\`\`\`bash
# Install dependencies
[install command]

# Start development
[dev command]

# Run tests
[test command]
\`\`\`

## Key Features

- Feature 1
- Feature 2
- Feature 3

## Status

- **Version**: X.Y.Z
- **Stage**: [development/staging/production]
- **License**: [license]

---
*Last updated: [DATE] | Initial setup*
```

### 2.2 Architecture (.github/copilot/docs/architecture.md)

```markdown
# System Architecture

## Overview

[Brief description of the architectural style - e.g., "Layered architecture", "Microservices", "Monolith", etc.]

**Key Patterns:**
- [Pattern 1 - e.g., Repository Pattern]
- [Pattern 2 - e.g., Dependency Injection]

## System Diagram

\`\`\`
[ASCII diagram of system components and their relationships]
\`\`\`

## Directory Structure

\`\`\`
[project-root]/
├── src/                    # Source code
│   ├── [layer1]/          # [Purpose]
│   ├── [layer2]/          # [Purpose]
│   └── [shared]/          # [Purpose]
├── tests/                  # Test files
├── config/                 # Configuration files
└── docs/                  # Documentation
\`\`\`

## Core Modules

| Module | Purpose | Key Files |
|--------|---------|-----------|
| [Module 1] | [What it does] | `src/[path]` |
| [Module 2] | [What it does] | `src/[path]` |

## Data Flow

1. [Entry Point] → 2. [Processing Layer] → 3. [Data Layer] → 4. [Response]

---
*Last updated: [DATE]*
```

### 2.3 Tech Stack (.github/copilot/docs/tech-stack.md)

```markdown
# Technology Stack

## Runtime

| Component | Version | Purpose |
|-----------|---------|---------|
| [Runtime] | [version] | [Runtime environment] |

## Languages

| Language | Usage | File Extensions |
|----------|-------|-----------------|
| [Language] | [Primary/Secondary] | [extensions] |

## Frameworks & Libraries

### Core
| Name | Version | Purpose |
|------|---------|---------|
| [Framework] | [version] | [purpose] |

### Development
| Name | Version | Purpose |
|------|---------|---------|
| [Tool] | [version] | [purpose] |

## Database & Storage

| Type | Technology | Purpose |
|------|------------|---------|
| [Type] | [tech] | [purpose] |

---
*Last updated: [DATE]*
```

### 2.4 API Documentation (.github/copilot/docs/api.md)

```markdown
# API Documentation

## Overview

[Brief description of the API - REST, GraphQL, RPC, etc.]

## Base URL

- **Development**: `http://localhost:[port]`
- **Production**: `[production URL]`

## Authentication

[Authentication method description]

## Endpoints

### [Resource Name]

#### GET /api/[resource]
- **Description**: [What it does]
- **Response**: `200 OK` - List of [resources]

#### POST /api/[resource]
- **Description**: [What it does]
- **Body**: [request body structure]
- **Response**: `201 Created`

[Continue for all endpoints...]

---
*Last updated: [DATE]*
```

### 2.5 Testing (.github/copilot/docs/testing.md)

```markdown
# Testing Strategy

## Framework

- **Primary**: [Jest/Vitest/etc]
- **E2E**: [Playwright/Cypress/none]
- **Coverage Tool**: [tool]

## Commands

| Command | Purpose |
|---------|---------|
| `[test command]` | Run all tests |
| `[watch command]` | Watch mode |
| `[coverage command]` | With coverage |

## Structure

\`\`\`
tests/
├── unit/           # Unit tests
├── integration/    # Integration tests
└── fixtures/       # Test data
\`\`\`

## Coverage

- **Target**: [X%]
- **Current**: [Y%]

---
*Last updated: [DATE]*
```

### 2.6 Development Guide (.github/copilot/docs/development.md)

```markdown
# Development Guide

## Prerequisites

- [Requirement 1]
- [Requirement 2]

## Setup

\`\`\`bash
# Clone and install
git clone [repo]
cd [project]
[install command]

# Configure environment
cp .env.example .env
# Edit .env with your values
\`\`\`

## Environment Variables

| Variable | Required | Description | Example |
|----------|----------|-------------|---------|
| [VAR] | [Yes/No] | [description] | [example] |

## Scripts

| Command | Description |
|---------|-------------|
| `[script]` | [what it does] |

---
*Last updated: [DATE]*
```

### 2.7 Conventions (.github/copilot/docs/conventions.md)

```markdown
# Code Conventions

## Style Guide

- **Language Config**: [tsconfig.json / etc]
- **Linter**: [ESLint/etc config]
- **Formatter**: [Prettier/etc config]

## Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Files | kebab-case | `user-service.ts` |
| Classes | PascalCase | `UserService` |
| Functions | camelCase | `getUserById` |
| Constants | SCREAMING_SNAKE | `MAX_RETRIES` |

## Patterns Used

| Pattern | Where | Example |
|---------|-------|---------|
| [Pattern] | [context] | [example] |

## Git Conventions

- **Branch naming**: `feature/`, `fix/`, `chore/`
- **Commit format**: [Conventional Commits / etc]

---
*Last updated: [DATE]*
```

### 2.8 Skill Opportunities (.github/copilot/docs/skills-opportunities.md) (NEW)

```markdown
# Skill Opportunities

## Capability Map

| Capability | Evidence | Existing Skill | Gap | Suggested Skill |
|------------|----------|----------------|-----|-----------------|
| [capability] | `[path]` | [yes/no + name] | [yes/no] | `[name]/SKILL.md` |

## Proposed Project-Specific Skills

| Skill | Why Needed | Primary Triggers | Priority |
|-------|------------|------------------|----------|
| `[name]/SKILL.md` | [reason] | [keywords/patterns] | [high/med/low] |

## Notes

- Skills listed as gaps should be generated now or on first matching change request.
- Proposals should be specific to this project, not generic boilerplate.

---
*Last updated: [DATE]*
```

---

## Step 3: Build Memory Index (.github/copilot/docs/index.yaml)

**CRITICAL**: This is the agent's navigation map. Update with REAL data from your analysis.

```yaml
# Smart Agent - Memory Index
# ALWAYS read this file first!

version: 1
last_updated: "[CURRENT_TIMESTAMP]"

project:
  name: "[PROJECT_NAME]"
  type: "[web-api/cli/library/monorepo/etc]"
  primary_language: "[language]"
  framework: "[main framework]"
  stage: "[development/staging/production]"

documents:
  overview:
    file: "overview.md"
    title: "Project Overview"
    summary: "[2-3 sentence summary of project purpose]"
    keywords: [purpose, quick-start, getting-started, about, features]
    last_updated: "[DATE]"
    
  architecture:
    file: "architecture.md"
    title: "System Architecture"
    summary: "[Architecture style and key patterns]"
    keywords: [layers, structure, modules, data-flow, directories, components]
    sections: [System Diagram, Directory Structure, Core Modules, Data Flow]
    last_updated: "[DATE]"
    
  tech-stack:
    file: "tech-stack.md"
    title: "Technology Stack"
    summary: "[Key technologies: language, framework, database]"
    keywords: [dependencies, frameworks, libraries, versions, runtime, database]
    dependencies_count: [NUMBER]
    last_updated: "[DATE]"
    
  api:
    file: "api.md"
    title: "API Documentation"
    summary: "[API type and scope]"
    keywords: [endpoints, routes, rest, http, requests]
    has_api: [true/false]
    endpoints_count: [NUMBER]
    last_updated: "[DATE]"
    
  testing:
    file: "testing.md"
    title: "Testing Strategy"
    summary: "[Test framework and coverage info]"
    keywords: [tests, coverage, unit, integration, commands]
    coverage: "[X%]"
    last_updated: "[DATE]"
    
  development:
    file: "development.md"
    title: "Development Guide"
    summary: "[Key setup info]"
    keywords: [setup, install, scripts, env, commands, run, build]
    scripts_count: [NUMBER]
    last_updated: "[DATE]"
    
  conventions:
    file: "conventions.md"
    title: "Code Conventions"
    summary: "[Key conventions]"
    keywords: [style, naming, patterns, linting, formatting, git]
    last_updated: "[DATE]"

  skill-opportunities:
    file: "skills-opportunities.md"
    title: "Skill Opportunities"
    summary: "Capability map and project-specific skill proposals"
    keywords: [skills, capabilities, routing, gaps, proposal]
    gaps_count: [NUMBER]
    proposed_skills_count: [NUMBER]
    last_updated: "[DATE]"

decisions:
  count: 0
  recent: []

cross_references: {}

quick_commands:
  dev: "[dev command]"
  build: "[build command]"
  test: "[test command]"
  lint: "[lint command]"
```

---

## Step 4: Initialize Supporting Files

### 4.1 Create decisions index if not exists

`.github/copilot/docs/decisions/index.yaml`:
```yaml
version: 1
last_updated: "[TIMESTAMP]"
next_id: 1
decisions: {}
by_category:
  architecture: []
  api: []
  security: []
  testing: []
  infrastructure: []
  dependencies: []
  patterns: []
  other: []
by_status:
  proposed: []
  accepted: []
  deprecated: []
  superseded: []
summary:
  total: 0
  proposed: 0
  accepted: 0
  deprecated: 0
  superseded: 0
```

### 4.2 Create plans state if not exists

`.github/copilot/plans/state.yaml`:
```yaml
version: 1
last_updated: "[TIMESTAMP]"
plans: {}
summary:
  draft: 0
  pending_review: 0
  approved: 0
  in_progress: 0
  completed: 0
  archived: 0
  rejected: 0
```

---

## Step 5: Report Summary

After completing all steps, provide a summary:

```
✅ **Agent Memory Initialized**

**Project**: [name]
**Tech Stack**: [primary language] + [framework]

**Memory Files Created/Updated in .github/copilot/docs/:**
- index.yaml (Memory Index)
- overview.md
- architecture.md
- tech-stack.md
- api.md
- testing.md
- development.md
- conventions.md
- skills-opportunities.md
- decisions/index.yaml

**Key Findings**:
- [Notable patterns or conventions found]
- [Potential issues or gaps]
- [Proposed project-specific skills and rationale]
- [Recommendations]

The Smart Agent now has full context for this project.
Use the index.yaml to quickly navigate documentation.
```

---

## Important Notes

1. **Use real data** - Do not use placeholder text. Analyze the actual codebase.
2. **Memory location** - ALL documentation MUST be in `.github/copilot/docs/`
3. **No duplication** - Each piece of information lives in ONE place only
4. **Update index** - The index.yaml must reflect all documentation accurately
5. **Be thorough** - Scan all relevant files, not just root level
