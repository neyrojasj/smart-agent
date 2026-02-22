````instructions
---
description: 'Central Copilot instructions for projects using Smart Agent (Skill-Based Architecture)'
applyTo: '**/*'
---

# Copilot Instructions

## Default Agent Mode - **ALWAYS @smart**

When starting a new chat session or when no specific agent is selected, **automatically load the Smart Agent** (`@smart`).

The Smart Agent is a **skill-based orchestrator** that routes requests to specialized skills while maintaining unified context in `.copilot/context.md`.

## 🚨 CRITICAL: Always Read Context First

```
┌─────────────────────────────────────────────────────────────────────────┐
│  MANDATORY FIRST STEP - DO THIS BEFORE ANYTHING ELSE                    │
│                                                                         │
│  READ: .copilot/context.md                                              │
│                                                                         │
│  This is the agent's UNIFIED MEMORY containing:                         │
│  • Project identity (name, type, stack)                                 │
│  • Current session state                                                │
│  • Pending tasks and recent actions                                     │
│  • User preferences and decisions                                       │
│                                                                         │
│  IF context.md doesn't exist → Run "Setup Project" handoff first        │
└─────────────────────────────────────────────────────────────────────────┘
```

## Skill-Based Architecture

The Smart Agent delegates to specialized skills:

| Skill | Purpose | Requires Approval |
|-------|---------|-------------------|
| **planning** | Plans, architecture, strategy | ✅ Yes |
| **coding** | Code generation & modification | ✅ Yes |
| **analysis** | Code review, debugging, explanations | ❌ No |
| **documentation** | Docs generation & updates | ❌ No |
| **testing** | Test creation with mocking | ❌ No |
| **setup** | Project initialization & skill generation | ❌ No |

## Orchestrator Flow

```
User Request → Load Context → Match Skill(s) → Execute Skill → Update Context → Respond
```

### Routing Rules

1. Read `.github/skills/index.yaml` for available skills
2. Match request against skill triggers (keywords + patterns)
3. Route to highest confidence skill(s)
4. Chain skills when appropriate (e.g., coding → testing)

## Project Configuration

```
.copilot/
├── context.md           # 🧠 Unified memory (READ FIRST)
├── docs/                # 📖 Project documentation
├── standards/           # 🛡️ Coding standards
├── plans/               # 📋 Implementation plans
└── prompts/             # 🎯 Setup prompts

.github/
└── skills/
	├── index.yaml       # Skill registry & routing rules
	└── <skill>/SKILL.md # Individual skill files
```

## Required Behavior

### Always On Every Request:
1. **FIRST**: Read `.copilot/context.md`
2. **THEN**: Read `.github/skills/index.yaml`
3. Route to appropriate skill(s)
4. Execute skill workflow completely
5. Update context.md with results

### Never:
- Execute tasks directly (always delegate to skills)
- Skip reading context.md
- Bypass skill approval requirements
- Leave context.md outdated

## Skill Chaining

| Workflow | Chain |
|----------|-------|
| New Feature | planning → coding → testing → documentation |
| Bug Fix | analysis → coding → testing |
| Code Review | analysis → documentation |

## Standards (If Installed)

When `.copilot/standards/` exists, skills automatically read and apply standards:

| File | Applied By |
|------|------------|
| `general.md` | All skills (always) |
| `[language].md` | coding.skill, testing.skill |
| `markdown.md` | documentation.skill |

## Context Operators

- `#file:name` - Reference specific file
- `#codebase` - Search entire codebase  
- `@workspace` - Full workspace context
- `#selection` - Currently selected code

````

