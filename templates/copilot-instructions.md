# Copilot Instructions

## Default Agent Mode

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

## General Programming Standards

**If standards are installed**, always read and apply `.copilot/standards/general.md` which contains critical programming guidelines.

### 🚫 FORBIDDEN Practices (Always Enforce)

#### 1. No Default Values for Environment Variables
```
❌ FORBIDDEN: process.env.PORT || 3000
❌ FORBIDDEN: std::env::var("PORT").unwrap_or("3000")
✅ REQUIRED: Fail if env var is not defined
```
Missing configuration must cause startup/compile failure, not silent fallback.

#### 2. No Silent Error Swallowing
```
❌ FORBIDDEN: catch (e) { } // empty catch
❌ FORBIDDEN: if let Ok(v) = result { } // ignoring Err
✅ REQUIRED: Handle, propagate, or log with context
```

#### 3. No Catch-All Defaults in Pattern Matching
```
❌ FORBIDDEN: _ => "default" // when all cases are known
❌ FORBIDDEN: default: return "other" // hiding known cases
✅ REQUIRED: Exhaustively match all known variants
```
Adding a new enum variant must trigger a compile error, not silent default handling.

#### 4. No Unsafe Unwrapping Without Justification
```
❌ FORBIDDEN: .unwrap() without explanation
❌ FORBIDDEN: value! (force unwrap)
✅ REQUIRED: .expect("reason") or explicit error handling
```

### Core Principle: Fail Fast, Fail Loud

When generating or reviewing code, always ask:
> *"If something goes wrong here, will I know about it immediately?"*

If the answer is **no**, the code needs to be more explicit.
