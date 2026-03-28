````instructions
---
description: 'Central Copilot instructions for projects using Smart Agent (Skill-Based Architecture)'
applyTo: '**/*'
---

# Copilot Instructions

## Default Agent Mode — ALWAYS @smart

When starting a new chat session or when no specific agent is selected, **automatically load the Smart Agent** (`@smart`).

The Smart Agent is defined in `.github/agents/smart.agent.md`. All orchestrator logic, routing rules, skill definitions, and chaining behavior live there. **Do NOT duplicate that logic here.**

## First Step — Read Context

Before doing anything, read `.github/copilot/context.md`. This is the agent's memory (project identity, preferences, session state).

If `context.md` doesn't exist → Run the **Setup Project** handoff first.

## Project Layout

```
.github/copilot/
├── context.md           # 🧠 Project memory (READ FIRST)
├── session.md           # 📋 Session state (actions, tasks)
├── docs/                # 📖 Project documentation
├── standards/           # 🛡️ Coding standards (optional)
├── plans/               # 📋 Implementation plans
└── prompts/             # 🎯 Setup prompts

.github/
├── agents/smart.agent.md   # 🎯 Orchestrator (all routing logic)
└── skills/
    ├── index.yaml           # Skill registry
    └── <skill>/SKILL.md     # Individual skill files
```

## Standards (If Installed)

When `.github/copilot/standards/` exists, skills automatically apply:

| File | Applied By |
|------|------------|
| `general.md` | All skills |
| `[language].md` | coding, testing |
| `markdown.md` | documentation |

````

