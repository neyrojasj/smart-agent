---
description: 'Central Copilot instructions for all projects using Copilot Agents'
applyTo: '**/*'
---

# Copilot Instructions

## Default Agent Mode - **ALWAYS @planning**

When starting a new chat session or when no specific agent is selected, **automatically load the Planning Agent** (`@planning`).

The Planning Agent ensures all code changes are planned, tracked, and explicitly approved before implementation.

## Agent Loading Priority

1. If user explicitly selects an agent → Use that agent
2. If no agent selected → Load `@planning` agent automatically
3. Always check `.github/agents/` for available custom agents

## Available Agents

### @planning (Default)
- **Purpose**: Plan, track, and implement code changes with user approval
- **Location**: `.github/agents/planning.agent.md`
- **When to use**: Any task that involves code modifications

## Required Behavior

### Always On First Interaction:
1. Check if `.copilot/project_summary.md` exists
   - If NOT: Analyze project structure and create it
2. Read `.copilot/instructions.md` for project-specific rules
3. Check `.copilot/plans/state.yaml` for pending plans

### Never:
- Implement code changes without explicit user approval
- Skip the planning phase for significant changes
- Ignore the state tracking in `.copilot/plans/`

## Project Configuration

- **Plans Location**: `.copilot/plans/`
- **State File**: `.copilot/plans/state.yaml`
- **Standards**: `.copilot/standards/` (if installed)
- **Instructions**: `.copilot/instructions.md`

## Workflow Summary

```
User Request → Create Plan → User Review → User Approval → Implementation
```

Every plan must go through `pending_review` state before any code is written.

---

## Standards (If Installed)

When `.copilot/standards/` folder exists, **always read and apply** the relevant standards files before generating or reviewing code:

| File | When to Apply |
|------|---------------|
| `general.md` | **Always** - Core principles for all languages |
| `rust.md` | When working with Rust code |
| `nodejs.md` | When working with Node.js/TypeScript code |

The standards contain critical guidelines that must be followed. Read them on first interaction with a project.
