<!-- ⚠️ REQUIRED: This file drives the Smart Orchestrator. Run the Setup skill (@smart setup project) to auto-generate. -->

# Project Context

> Last updated: 2026-03-31

## Project Identity

- **Name**: Smart Copilot (planning-copilot)
- **Type**: AI agent framework / VS Code Copilot skill-based architecture
- **Stack**: Markdown (agent/skill definitions), YAML (config/registry), Shell (install scripts)
- **Stage**: development

## User Preferences

- All plans must be written to disk as `.md` files (no inline-only plans)
- Post-execution learning updates skills, docs, context — never standards
- User approves all learning proposals before they are applied

## Project-Specific Rules

- Smart Manager is the unified agent; Smart Agent is a backward-compat redirect
- Standards files are human-curated only — never auto-updated by the agent

## Key Decisions

| Decision | Reason | Skill | Date |
|----------|--------|-------|------|
| Unified Smart Manager (merged Smart + Smart Manager) | Eliminate routing-only agent, make all flows iterative | planning | 2026-03-31 |
| Adversarial plan curation (plan-reviewer) | Improve plan quality before user sees it | planning | 2026-03-31 |
| Post-execution learning (evaluator learning mode) | Auto-improve skills/docs/context after every plan | planning | 2026-03-31 |
| All plans to disk | Filesystem is the AI working memory, not conversation | planning | 2026-03-31 |
| Separated plan-reviewer from evaluator | Pre-execution review (plan-reviewer) vs post-execution evaluation (evaluator) for focused skills | planning | 2026-03-31 |

---

*Auto-updated by Smart Orchestrator*
