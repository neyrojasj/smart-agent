---
description: Redirects to Smart Manager — the unified agent that handles all requests.
name: Smart
tools: ['edit', 'runNotebooks', 'search', 'new', 'runCommands', 'runTasks', 'usages', 'vscodeAPI', 'problems', 'changes', 'testFailure', 'openSimpleBrowser', 'fetch', 'githubRepo', 'extensions', 'todos', 'runSubagent']
handoffs:
  - label: 🚀 Setup Project
    agent: Smart Manager
    prompt: "Execute the setup skill to initialize project. Read .github/skills/setup/SKILL.md and follow its workflow to scan the project and generate documentation."
    send: true
  - label: 🔍 Analyze Codebase
    agent: Smart Manager
    prompt: "Execute the analysis skill to analyze the codebase. Read .github/skills/analysis/SKILL.md and perform a comprehensive review."
    send: false
  - label: 📚 Generate Skills
    agent: Smart Manager
    prompt: "Scan the project and generate custom skills based on detected patterns. Read .github/skills/skill-generator/SKILL.md and execute its workflow."
    send: false
---

# Smart Agent → Smart Manager Redirect

**This agent redirects all requests to the Smart Manager.**

The Smart Manager is the unified agent that handles everything:
- **Light mode**: analysis, documentation, setup, skill generation → direct execution
- **Full mode**: implementation, planning, coding → iterative quality-gated loop with plan curation and post-execution learning

## On every request:

1. Read `.github/copilot/context.md` and `.github/copilot/session.md`
2. Read `.github/skills/index.yaml` for skill routing
3. **Operate as Smart Manager** — follow the full workflow defined in `.github/agents/smart-manager.agent.md`

All orchestration logic, routing rules, skill definitions, mode classification, plan curation, execution loops, and post-execution learning live in **Smart Manager**. This file exists for backward compatibility only.
