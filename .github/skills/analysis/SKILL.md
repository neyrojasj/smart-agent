---
name: analysis
description: Analyze code, debug issues, run audits, and explain behavior.
version: "1.0"
---

# Analysis Skill

## Identity

- **Name**: analysis
- **Version**: 1.0
- **Description**: Analyzes code, performs audits, debugging, and provides explanations.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Keywords: analyze, audit, review, debug | "Analyze this function" |
| "explain..." | "Explain how auth works" |
| "why does..." | "Why does this fail?" |
| "what...doing" | "What is this code doing?" |
| "help me understand" | "Help me understand the flow" |

---

## Capabilities

What this skill can do:

- ✅ Explain code functionality
- ✅ Debug issues and errors
- ✅ Perform code audits
- ✅ Review code quality
- ✅ Identify security issues
- ✅ Find performance bottlenecks
- ✅ Trace data flow
- ✅ Analyze dependencies

---

## Dependencies

- `context.md` - For project context
- `.github/copilot/docs/` - For architecture understanding
- `.github/copilot/standards/` - For audit comparisons
- `planning.skill` - Chains to for fixes
- `coding.skill` - Chains to for implementation

---

## Workflow

### Step 1: Understand the Question

Categorize the analysis request:

| Type | Description | Approach |
|------|-------------|----------|
| **Explanation** | "What does X do?" | Trace and explain |
| **Debugging** | "Why is X failing?" | Investigate and diagnose |
| **Audit** | "Review X for issues" | Systematic check |
| **Understanding** | "How does X work?" | Architecture overview |

### Step 2: Gather Context

```
1. Read relevant files completely
2. Check .github/copilot/docs/ for architecture context
3. Look at related tests for expected behavior
4. Check git history if investigating bugs
```

### Step 3: Perform Analysis

#### For Code Explanation

```markdown
## Code Explanation: [File/Function]

### Overview
[High-level summary of what the code does]

### Step-by-Step Flow
1. [Step 1]: [What happens]
2. [Step 2]: [What happens]
3. [Step 3]: [What happens]

### Key Concepts
- **[Concept 1]**: [Explanation]
- **[Concept 2]**: [Explanation]

### Dependencies
- Uses: [what it depends on]
- Used by: [what depends on it]
```

#### For Debugging

```markdown
## Debug Analysis: [Issue]

### Error Summary
[What the error is]

### Root Cause
[Why it's happening]

### Investigation Trail
1. Checked [X] → Found [Y]
2. Traced [A] → Discovered [B]

### Solution
[How to fix it]

### Prevention
[How to prevent in future]
```

#### For Code Audit

```markdown
## Code Audit Report: [Scope]

### Summary
- **Files reviewed**: [count]
- **Issues found**: [count]
- **Severity**: [Critical/High/Medium/Low]

### Findings

#### 🔴 Critical Issues
| Issue | Location | Impact |
|-------|----------|--------|
| [issue] | `file:line` | [impact] |

#### 🟡 Warnings
| Issue | Location | Recommendation |
|-------|----------|----------------|
| [issue] | `file:line` | [fix] |

#### 🟢 Best Practices
[What's being done well]

### Standards Compliance
- ✅/❌ [Standard 1]
- ✅/❌ [Standard 2]

### Recommendations
1. [Priority 1 fix]
2. [Priority 2 fix]
```

### Step 4: Provide Actionable Output

Always end with actionable next steps:

```markdown
### Next Steps

**If you want to fix these issues:**
→ I can route to the **planning.skill** to create a fix plan

**If you need more details:**
→ Ask me to dive deeper into [specific area]

**If this looks good:**
→ No action needed
```

---

## Audit Checklist

When performing audits, check for:

### Security
- [ ] No hardcoded secrets/credentials
- [ ] Input validation present
- [ ] SQL injection protection
- [ ] XSS prevention
- [ ] CSRF protection (if web)
- [ ] Proper authentication checks

### Code Quality
- [ ] Consistent naming conventions
- [ ] No duplicate code (DRY)
- [ ] Functions have single responsibility
- [ ] Error handling is explicit
- [ ] No silent failures
- [ ] Comments where necessary

### Performance
- [ ] No N+1 queries
- [ ] Proper indexing considered
- [ ] No memory leaks risks
- [ ] Async operations handled correctly
- [ ] Caching considered

### Standards Compliance
- [ ] Read `.github/copilot/standards/` first
- [ ] Compare against language-specific rules
- [ ] Flag violations with severity

---

## Output Format

Return to orchestrator:

```yaml
status: success | needs_more_info | error
result:
  analysis_type: "explanation | debugging | audit | understanding"
  summary: "[Brief summary]"
  findings_count: [number]
  critical_issues: [number]
  suggested_fixes: [list]
context_updates:
  recent_actions:
    - "Analyzed [target] - Found [N] issues"
next_skill: null  # or "planning" if fixes needed
user_message: "[Analysis results]"
```

---

## Never Do

- ❌ Make changes directly (delegate to coding.skill)
- ❌ Skip reading the actual code
- ❌ Give vague answers without specifics
- ❌ Miss security issues in audits
- ❌ Ignore existing tests/documentation
- ❌ Provide solutions without explaining the problem

---

## Special Analysis Modes

### #terminalLastCommand
When user references terminal output:
```
1. Read the terminal error output
2. Parse error messages
3. Identify root cause
4. Suggest specific fix
```

### #selection
When user references selected code:
```
1. Analyze selected code in context
2. Consider surrounding code
3. Explain or debug as requested
```

### Performance Profiling
When analyzing performance:
```
1. Identify hot paths
2. Look for O(n²) operations
3. Check database query patterns
4. Analyze memory usage patterns
```
