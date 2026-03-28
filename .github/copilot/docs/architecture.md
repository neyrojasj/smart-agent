<!-- TEMPLATE: Replace this file with real project information. Run the Setup skill (@smart setup project) to auto-generate. -->

# System Architecture

## Overview

[Brief description of the architectural style - e.g., "Layered architecture", "Microservices", "Monolith", etc.]

**Key Patterns:**
- [Pattern 1 - e.g., Repository Pattern]
- [Pattern 2 - e.g., Dependency Injection]

## System Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        [System Name]                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│   [Draw your system components and their relationships]      │
│                                                              │
│   Example:                                                   │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐             │
│   │  Client  │───▶│   API    │───▶│ Database │             │
│   └──────────┘    └──────────┘    └──────────┘             │
│                         │                                    │
│                         ▼                                    │
│                   ┌──────────┐                              │
│                   │  Cache   │                              │
│                   └──────────┘                              │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
[project-root]/
├── src/                    # Source code
│   ├── [layer1]/          # [Purpose - e.g., "API routes and controllers"]
│   ├── [layer2]/          # [Purpose - e.g., "Business logic services"]
│   ├── [layer3]/          # [Purpose - e.g., "Data access layer"]
│   └── [shared]/          # [Purpose - e.g., "Shared utilities"]
├── tests/                  # Test files
│   ├── unit/              # Unit tests
│   └── integration/       # Integration tests
├── config/                 # Configuration files
├── scripts/               # Build and utility scripts
└── docs/                  # Documentation
```

## Core Modules

| Module | Purpose | Key Files |
|--------|---------|-----------|
| [Module 1] | [What it does] | `src/[path]` |
| [Module 2] | [What it does] | `src/[path]` |
| [Module 3] | [What it does] | `src/[path]` |

## Data Flow

```
1. [Entry Point] ──▶ 2. [Processing Layer] ──▶ 3. [Data Layer] ──▶ 4. [Response]

Example:
1. HTTP Request arrives at Controller
2. Controller calls Service with validated input
3. Service executes business logic, calls Repository
4. Repository queries Database
5. Response flows back through layers
```

### Request Lifecycle

1. **Entry**: [Where requests enter - e.g., "Express router"]
2. **Validation**: [How input is validated]
3. **Processing**: [Where business logic lives]
4. **Persistence**: [How data is stored/retrieved]
5. **Response**: [How responses are formatted]

## Integration Points

| External System | Type | Purpose | Config |
|----------------|------|---------|--------|
| [System 1] | [API/DB/Queue] | [Why integrated] | `[config location]` |
| [System 2] | [API/DB/Queue] | [Why integrated] | `[config location]` |

## Boundaries

### What This System Does
- [Responsibility 1]
- [Responsibility 2]

### What This System Does NOT Do
- [Out of scope 1]
- [Out of scope 2]

---
*Last updated: [DATE]*
