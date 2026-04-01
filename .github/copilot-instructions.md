````instructions
---
description: 'Central Copilot instructions for projects using Smart Agent (Skill-Based Architecture)'
applyTo: '**/*'
---

# Copilot Instructions

## Default Agent Mode — ALWAYS @smart-manager

When starting a new chat session or when no specific agent is selected, **automatically load the Smart Manager** (`@smart-manager`).

The Smart Manager is defined in `.github/agents/smart-manager.agent.md`. It is the **unified agent** that handles all requests:
- **Light mode**: analysis, docs, setup → direct skill execution
- **Full mode**: implementation → plan → curate → QA → approve → execute → evaluate → learn

All orchestrator logic, routing rules, skill definitions, plan curation, and post-execution learning live there. **Do NOT duplicate that logic here.**

> `@smart` still exists for backward compatibility but redirects to Smart Manager.

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
├── plans/               # 📋 Implementation plans (ALL plans go to disk)
└── prompts/             # 🎯 Setup prompts

.github/
├── agents/
│   ├── smart-manager.agent.md  # 🎯 Unified agent (routing + execution + learning)
│   └── smart.agent.md          # ↪️ Redirect to Smart Manager
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

