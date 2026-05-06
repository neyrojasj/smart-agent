---
name: coding
description: Generate and modify code following project standards, DES MOD/IFC structure, and TDD test contracts.
version: "2.0"
---

# Coding Skill

## Identity

- **Name**: coding
- **Version**: 2.0
- **Description**: Implements code to make TDD tests GREEN, following DES MOD/IFC structure and project standards.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Keywords: implement, code, create, add, fix | "Implement the login API" |
| "add...feature" | "Add a caching layer" |
| "fix...bug" | "Fix the null pointer error" |
| "refactor..." | "Refactor the user service" |

---

## Capabilities

What this skill can do:

- ✅ Generate new code files
- ✅ Modify existing code
- ✅ Apply coding standards automatically
- ✅ Handle multi-file changes
- ✅ Refactor code structures
- ✅ Fix bugs with proper error handling
- ✅ Document API and behavior changes

---

## Dependencies

- `glossary.md` — Term resolution (read first)
- `context.md` — Project identity
- `.github/copilot/docs/ddd/DESIGN-XXX.md` — **MOD structure and IFC signatures** (primary source of truth)
- `.github/copilot/docs/tdd/TEST-XXX.md` — **Test matrix** (pass all tests = done)
- `.github/copilot/standards/` — MANDATORY coding standards
- `plans/PLAN-XXX.md` + `KNOWLEDGE-XXX.md` — Approved plan + context cheat sheet
- `testing.skill` — Chains to for generic test creation (when no TST exists)

---

## Workflow

### Step 1: Load Context (in order)

```
1. Read .github/copilot/glossary.md
2. Read .github/copilot/standards/general.md (MANDATORY)
3. Read language-specific standard if exists
4. Read KF (KNOWLEDGE-XXX.md) — primary context cheat sheet
5. If DES exists → read DESIGN-XXX.md → extract @mod and @ifc tags
6. If TST exists → read TEST-XXX.md → extract test matrix (these are your acceptance criteria)
```

**DES-driven implementation rules (when DES exists):**
- One file per `@mod` — do NOT split a MOD into multiple files
- IFC signatures must match the DES exactly — no ad-hoc variations
- After implementing, fill in `@impl:<path>` tag in the DES
- Cross-MOD calls MUST go through the defined IFC — never bypass it

**TDD contract (when TST exists):**
- Goal is GREEN tests, not just compiling code
- Run tests after each MOD implementation
- Do NOT modify test files — if a test seems wrong, flag it and ask
- After all tests GREEN, update TST `@status:implemented`

### Step 2: Check for Approved Plan

```
1. Read .github/copilot/plans/state.yaml → find status: approved
2. Read PLAN-XXX.md (phases, tasks)
3. Re-read KNOWLEDGE-XXX.md whenever context is lost
4. Follow phases in order; re-read KF between phases
```

If no plan (small change, <100 lines):
- Proceed directly; re-read DES/TST if they exist

### Step 3: Understand Current Code

Before modifying:
```
1. Read target files completely
2. Understand existing patterns
3. Identify integration points
4. Note existing tests
```

### Step 4: Generate Code

Apply these quality rules:

#### Naming Conventions
```
Files:      kebab-case (user-service.ts)
Classes:    PascalCase (UserService)
Functions:  camelCase (getUserById)
Constants:  SCREAMING_SNAKE (MAX_RETRIES)
```

#### Error Handling
```
❌ NEVER: Use default values for runtime data
❌ NEVER: Silent error swallowing
❌ NEVER: Bare catch blocks

✅ ALWAYS: Explicit error handling
✅ ALWAYS: Meaningful error messages
✅ ALWAYS: Proper error propagation
```

#### Code Structure
```
✅ Single responsibility per function
✅ Maximum 3 levels of nesting
✅ Comments for complex logic only
✅ Meaningful variable names
```

#### Documentation Good Practices
```
✅ Update user-facing docs when behavior changes
✅ Add/update public API docs (docstrings, JSDoc, rustdoc, godoc, etc.)
✅ Explain WHY for non-obvious decisions
✅ Keep examples aligned with the implemented code
❌ Do not leave stale docs after refactors or renames
```

### Step 5: Present Changes for Approval

```markdown
📝 **Code Changes Ready for Review**

**Files to create:**
- `path/to/new-file.ts` - [purpose]

**Files to modify:**
- `path/to/existing.ts` - [what changes]

**Code Preview:**

\`\`\`[language]
// Key changes shown here
[code snippet]
\`\`\`

**Standards Applied:**
- ✅ [standard 1]
- ✅ [standard 2]

Reply with: ✅ approve | ❌ reject | 📝 revise [feedback]
```

### Step 6: Implement (After Approval Only)

```
1. Create/modify files as approved
2. Maintain consistent formatting
3. Update imports/exports
4. Preserve existing functionality
```

### Step 7: Chain to Testing

After implementation:
```
→ Automatically chain to testing.skill to create tests
```

---

## Code Quality Checklist

Before submitting code:

- [ ] Standards file read for this language
- [ ] If standards missing, repository search completed and user asked to create standards
- [ ] Naming conventions followed
- [ ] Error handling implemented (no defaults for runtime data)
- [ ] No silent error swallowing
- [ ] Functions are focused (single responsibility)
- [ ] Comments only where necessary
- [ ] Public API docs/docstrings updated when signatures or behavior changed
- [ ] User-facing documentation updated for behavior/config/API changes
- [ ] Imports organized
- [ ] No hardcoded secrets/credentials

---

## Output Format

Return to orchestrator:

```yaml
status: success | needs_approval | error
result:
  files_created: [list]
  files_modified: [list]
  lines_changed: [count]
  standards_applied: [list]
context_updates:
  recent_actions:
    - "Created/Modified [files] (pending approval)"
next_skill: testing  # Chain to testing after implementation
user_message: "[What was done or needs approval]"
```

---

## Forbidden Default Values

```
┌─────────────────────────────────────────────────────────────────────────┐
│  CRITICAL: NO DEFAULT VALUES FOR RUNTIME DATA                           │
│                                                                         │
│  ❌ DON'T:                                                              │
│  • .unwrap_or("default")                                                │
│  • || "fallback"                                                        │
│  • ?? "default_value"                                                   │
│  • env.get("VAR").unwrap_or("default")                                  │
│                                                                         │
│  ✅ DO:                                                                 │
│  • Return error if required data missing                                │
│  • Skip operation gracefully with logging                               │
│  • Fail fast with meaningful error message                              │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Never Do

- ❌ Generate code without reading standards first
- ❌ Implement without explicit approval
- ❌ Use default values for user IDs, sessions, auth context
- ❌ Skip error handling
- ❌ Create duplicate code
- ❌ Ignore existing patterns in codebase
- ❌ Commit secrets or credentials
- ❌ Delete files without confirmation

---

## Language-Specific Notes

### Node.js/TypeScript
- Use TypeScript strict mode
- Prefer async/await over callbacks
- Use ESLint + Prettier formatting

### Python
- Follow PEP 8
- Use type hints
- Prefer f-strings

### Rust
- Use `thiserror` for errors
- Avoid `.unwrap()` without `.expect()`
- Run clippy before completion

### Go
- Follow Effective Go
- Handle all errors explicitly
- Use gofmt
