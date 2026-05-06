---
name: documentation
description: Generate and update project documentation and keep docs synchronized with code.
version: "1.0"
---

# Documentation Skill

## Identity

- **Name**: documentation
- **Version**: 1.0
- **Description**: Generates and updates project documentation in `.github/copilot/docs/`.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Keywords: document, docs, readme | "Document this API" |
| "update...docs" | "Update the architecture docs" |
| "write...readme" | "Write a README for this" |
| "add...comments" | "Add JSDoc comments" |

---

## Capabilities

What this skill can do:

- ✅ Generate project documentation
- ✅ Update existing docs
- ✅ Create/update index.yaml
- ✅ Write API documentation
- ✅ Document architecture
- ✅ Add code comments
- ✅ Create ADRs (Architecture Decision Records)
- ✅ Sync docs with code changes

---

## Dependencies

- `context.md` - For project context
- `.github/copilot/docs/` - Where docs live
- `.github/copilot/standards/markdown.md` - For markdown formatting
- `analysis.skill` - May chain from for understanding

---

## Workflow

### Step 1: Load Markdown Standards

```
┌─────────────────────────────────────────────────────────────────────────┐
│  BEFORE WRITING ANY MARKDOWN                                            │
│                                                                         │
│  Read .github/copilot/standards/markdown.md if it exists                       │
│  Apply consistent formatting across all docs                            │
└─────────────────────────────────────────────────────────────────────────┘
```

### Step 2: Identify Documentation Target

| Target | Output Location |
|--------|-----------------|
| Project overview | `.github/copilot/docs/overview.md` |
| Architecture | `.github/copilot/docs/architecture.md` |
| Tech stack | `.github/copilot/docs/tech-stack.md` |
| API endpoints | `.github/copilot/docs/api.md` |
| Testing | `.github/copilot/docs/testing.md` |
| Development | `.github/copilot/docs/development.md` |
| Conventions | `.github/copilot/docs/conventions.md` |
| Decision | `.github/copilot/docs/decisions/DEC-XXX.md` |

### Step 3: Gather Information

```
1. Analyze codebase for current state
2. Read existing documentation
3. Check what's outdated
4. Identify gaps
```

### Step 4: Generate Documentation

#### For overview.md

```markdown
# [Project Name]

> [One-line description]

## Purpose

[2-3 sentences about what this project does]

## Quick Start

\`\`\`bash
[install command]
[run command]
\`\`\`

## Key Features

- Feature 1
- Feature 2

## Status

- **Version**: X.Y.Z
- **Stage**: [development/staging/production]

---
*Last updated: [DATE]*
```

#### For architecture.md

```markdown
# System Architecture

## Overview

[Architectural style and patterns]

## System Diagram

\`\`\`
[ASCII diagram]
\`\`\`

## Directory Structure

\`\`\`
[Tree structure]
\`\`\`

## Core Modules

| Module | Purpose | Key Files |
|--------|---------|-----------|
| [name] | [purpose] | [files] |

---
*Last updated: [DATE]*
```

#### For api.md

```markdown
# API Documentation

## Base URL

- **Development**: `http://localhost:[port]`

## Authentication

[Method description]

## Endpoints

### [Resource]

#### GET /api/[resource]
- **Description**: [what it does]
- **Response**: [format]

[Continue for all endpoints]

---
*Last updated: [DATE]*
```

### Step 5: Update Index

After any doc change, update `.github/copilot/docs/index.yaml`:

```yaml
documents:
  [doc_key]:
    file: "[filename].md"
    title: "[Title]"
    summary: "[Updated summary]"
    keywords: [updated, keywords]
    last_updated: "[DATE]"
```

### Step 6: No Duplication Rule

```
┌─────────────────────────────────────────────────────────────────────────┐
│  SINGLE SOURCE OF TRUTH                                                 │
│                                                                         │
│  Each piece of information lives in EXACTLY ONE place:                  │
│                                                                         │
│  • Project name/description → overview.md                               │
│  • Tech stack/deps → tech-stack.md                                      │
│  • Directory structure → architecture.md                                │
│  • API endpoints → api.md                                               │
│  • Test commands → testing.md                                           │
│  • Environment vars → development.md                                    │
│  • Decisions → decisions/DEC-XXX.md                                     │
│                                                                         │
│  NEVER duplicate information across files                               │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Decision Records (ADRs)

When documenting significant decisions:

`.github/copilot/docs/decisions/DEC-XXX.md`:

```markdown
# DEC-XXX: [Title]

## Status

[proposed | accepted | deprecated | superseded]

## Context

[Why this decision is needed]

## Decision

[What we decided]

## Consequences

### Positive
- [Benefit 1]

### Negative
- [Tradeoff 1]

## Alternatives Considered

1. **[Alternative 1]**: [Why rejected]
2. **[Alternative 2]**: [Why rejected]

---
*Created: [DATE] | Status: [status]*
```

Update `.github/copilot/docs/decisions/index.yaml`:

```yaml
decisions:
  DEC-XXX:
    title: "[Title]"
    status: "[status]"
    category: "[category]"
    created: "[DATE]"
```

---

## Output Format

Return to orchestrator:

```yaml
status: success | error
result:
  docs_created: [list]
  docs_updated: [list]
  index_updated: true|false
context_updates:
  recent_actions:
    - "Updated [doc] documentation"
next_skill: null
user_message: "[Summary of changes]"
```

---

## Documentation Sync Checklist

After code changes, check if docs need updates:

| Code Change | Doc to Update |
|-------------|---------------|
| New dependency | tech-stack.md |
| New API endpoint | api.md |
| Directory restructure | architecture.md |
| New script | development.md |
| New pattern | conventions.md |
| Major decision | decisions/DEC-XXX.md |

---

## Never Do

- ❌ Create duplicate information
- ❌ Leave index.yaml outdated
- ❌ Write docs without reading existing ones
- ❌ Ignore markdown standards
- ❌ Create docs outside `.github/copilot/docs/`
- ❌ Forget the last_updated timestamp
