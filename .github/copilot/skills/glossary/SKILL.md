---
name: glossary
description: Manage project glossary — acronyms and terms used by DDD, TDD, and all skills to minimize context window usage.
version: "1.0"
---

# Glossary Skill

## Identity

- **Name**: glossary
- **Version**: 1.0
- **Mode**: Light
- **Output**: `.github/copilot/glossary.md`

---

## Purpose

Single source of short-form terms shared across all skills and agent responses.
Every skill reads the glossary before interpreting user requests.
Use glossary terms in ALL outputs — never spell out terms that exist in the glossary.

---

## Triggers

| Pattern | Example |
|---------|---------|
| add term / update glossary | "add BFF to the glossary" |
| what does X mean | "what does DDD mean here?" |
| glossary / terms / acronyms | "show me the glossary" |

---

## Built-in Terms (Bootstrap)

The following terms are always available even before `glossary.md` exists:

| Term | Meaning |
|------|---------|
| DDD | Design Driven Development — design-first workflow that produces AI-readable design docs with tagged interfaces |
| TDD | Test Driven Development — write tests before implementation |
| PLAN | Implementation plan file (`PLAN-XXX.md`) |
| KF | Knowledge File (`KNOWLEDGE-XXX.md`) — execution context cheat sheet |
| QA | QA Checklist (`QA-XXX.md`) |
| DES | Design document (`DESIGN-XXX.md`) produced by DDD skill |
| TST | Test spec (`TEST-XXX.md`) produced by TDD skill |
| GLO | Glossary file (`.github/copilot/glossary.md`) |
| CTX | Context file (`.github/copilot/context.md`) |
| SES | Session file (`.github/copilot/session.md`) |
| STD | Standards directory (`.github/copilot/standards/`) |
| IFC | Interface — contract boundary between modules |
| MOD | Module — a large cohesive code unit (prefer fewer, larger modules) |
| ADR | Architectural Decision Record |

---

## Workflow

### Add / Update Terms

```
1. Read .github/copilot/glossary.md (if exists)
2. Add/update requested terms
3. Sort alphabetically
4. Write back to .github/copilot/glossary.md
5. Confirm: "GLO updated: [N] terms total"
```

### Show Glossary

```
1. Read .github/copilot/glossary.md
2. Render as table: Term | Meaning | Added
```

### Bootstrap (first run)

If `glossary.md` does not exist:
```
1. Create .github/copilot/glossary.md with built-in terms above
2. Prompt user: "GLO created. Add project-specific terms with: 'add [TERM] = [meaning] to glossary'"
```

---

## Glossary File Format

```markdown
# Project Glossary

> Last updated: YYYY-MM-DD

| Term | Meaning | Added |
|------|---------|-------|
| DDD  | Design Driven Development | bootstrap |
| TDD  | Test Driven Development | bootstrap |
| ...  | ...                     | ...       |
```

---

## Rules

- Terms are UPPERCASE acronyms or short identifiers
- Meanings are ONE sentence max
- All skills must read GLO before routing user requests
- Agent responses use GLO terms instead of spelling out full phrases
- Never remove bootstrap terms
