---
name: coding
description: Generate and modify code following project standards and approval flow.
---

# Coding Skill

## Identity

- **Name**: coding
- **Version**: 1.0
- **Description**: Generates and modifies code following project standards, with mandatory approval for all changes.

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

---

## Dependencies

- `context.md` - For project context
- `.copilot/standards/` - MANDATORY for code generation
- `.copilot/docs/` - For architecture understanding
- `planning.skill` - For approved plans
- `testing.skill` - Chains to for test creation

---

## Workflow

### Step 1: Load Standards (MANDATORY)

```
┌─────────────────────────────────────────────────────────────────────────┐
│  BEFORE WRITING ANY CODE                                                │
│                                                                         │
│  1. Check if .copilot/standards/ exists                                 │
│  2. Read .copilot/standards/general.md (ALWAYS)                         │
│  3. Read language-specific standard (e.g., nodejs.md, python.md)        │
│  4. Apply ALL rules to generated code                                   │
│                                                                         │
│  IF standards exist but not read → CODE QUALITY VIOLATION               │
└─────────────────────────────────────────────────────────────────────────┘
```

### Step 2: Check for Approved Plan

If implementing a plan:
```
1. Read .copilot/plans/state.yaml
2. Find plan with status: approved
3. Read full plan document
4. Follow phases in order
```

If no plan (small change):
```
1. Estimate change size
2. If >100 lines → Redirect to planning.skill first
3. If <100 lines → Proceed with quick implementation
```

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
- [ ] Naming conventions followed
- [ ] Error handling implemented (no defaults for runtime data)
- [ ] No silent error swallowing
- [ ] Functions are focused (single responsibility)
- [ ] Comments only where necessary
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
