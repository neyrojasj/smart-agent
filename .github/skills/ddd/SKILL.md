---
name: ddd
description: Design Driven Development — challenge the user with clarifying questions, then produce an AI-readable design document (DES) with tagged interfaces and MOD boundaries.
version: "1.0"
---

# DDD Skill

## Identity

- **Name**: ddd
- **Version**: 1.0
- **Mode**: Full (requires approval before doc is written)
- **Output**: `.github/copilot/docs/ddd/DES-NNN-<slug>.md`

---

## Purpose

Produce a DES: an AI-readable design document that:
- Defines MOD boundaries (fewer, larger modules over many small files)
- Defines IFCs between MODs (all cross-MOD communication goes through an IFC)
- Tags every section so coding and fix skills can locate implemented code fast
- Contains only shallow IFC definitions — NO implementation details

---

## Triggers

| Pattern | Example |
|---------|---------|
| design, DDD, architect | "design the auth system" |
| "how should X be structured" | "how should the queue module be structured?" |
| "define interfaces for" | "define interfaces for the payment MOD" |
| chained from: fix skill | fix detects design-level change needed |

---

## DES Document Tags

Every section in the DES uses `@tags` so coding/fix skills can grep for context without reading the entire document.

| Tag | Meaning |
|-----|---------|
| `@mod:<name>` | Declares a MOD (e.g., `@mod:auth`) |
| `@ifc:<name>` | Declares an IFC (e.g., `@ifc:AuthService`) |
| `@dep:<mod>` | Declares a dependency on another MOD |
| `@file:<path>` | Suggested file path for this MOD |
| `@impl:<path>` | Where this IFC is implemented (filled in by coding skill) |
| `@test:<path>` | Where tests for this IFC live (filled in by TDD skill) |
| `@status:draft\|approved\|implemented` | Lifecycle state |

---

## Design Principles (ENFORCE THESE)

1. **Fewer files is better** — prefer one large MOD over many small files
2. **IFC mandatory for cross-MOD calls** — no direct function calls across MOD boundaries
3. **Shallow IFCs only** — signatures + doc comment, no implementation
4. **Tags are mandatory** — every MOD and IFC must be tagged
5. **No implementation in DES** — DES describes WHAT, not HOW

---

## Workflow

### Step 1: Load GLO + CTX + Scan DES directory

```
1. Read .github/copilot/glossary.md
2. Read .github/copilot/context.md (project type, stack)
3. List files in .github/copilot/docs/ddd/ (grep titles/slugs)
4. If a DES matches the feature by slug, title, or MOD name → read it (UPDATE, do not create new)
5. If no match → create new DES (NNN = highest existing number + 1)
```

### Step 2: Challenge the User (MANDATORY)

Before ANY design is produced, ask adversarial clarifying questions.
Be direct. Push back on vague scope. Ask only what matters.

**Challenge Template:**

```markdown
🎯 **DDD: [Request Summary]**

Before I design this, I need clarity:

**Scope**
- [ ] What are the exact boundaries of this feature? What is explicitly OUT of scope?
- [ ] Which existing MODs does this interact with?

**Behavior**
- [ ] What are the 3 most critical operations this must support?
- [ ] What happens on failure / edge cases?

**Module structure**
- [ ] Should this be a new MOD or extend an existing one?
- [ ] List all data it reads/writes and from/to where.

**IFCs**
- [ ] Who calls this? (other MODs, external clients, both?)
- [ ] What must callers provide? What do they get back?

Answer these. I'll challenge any answer that is still vague.
```

**Adversarial Challenge Rules:**
- If an answer is vague ("just make it work"), push back with a specific counter-question
- If scope creep is detected ("and also add X"), flag it: "X is out of scope for this DES. Create a separate DDD for X."
- Stop asking when you have enough to define all MODs and IFCs unambiguously
- Max 2 rounds of questions

### Step 3: Draft DES Structure

After questions answered, draft the DES outline and confirm with user before writing the full file:

```markdown
📐 **DES Draft: [Name]**

MODs:
- `@mod:foo` → `@file:src/foo/mod.rs`
- `@mod:bar` → `@file:src/bar/mod.rs`

IFCs:
- `@ifc:FooService` (in @mod:foo) — [one-line description]
- `@ifc:BarHandler` (in @mod:bar) — [one-line description]

Cross-MOD deps:
- bar → foo via `@ifc:FooService`

Confirm? (y / adjust)
```

### Step 4: Write DES to Disk

After confirmation, write `DES-NNN-<slug>.md`:

Slug = kebab-case summary of the feature (e.g., `sync-cli-command`, `user-auth-jwt`).

```markdown
# DES-XXX: [Title]

> @status:draft | Created: YYYY-MM-DD | Stack: [stack]

## Overview

[2-3 sentences. What this design covers.]

---

## MODs

### @mod:foo
> @file:src/foo/mod.rs | @status:draft

[One paragraph: responsibility of this MOD]

**IFCs exposed:**

#### @ifc:FooService
> @dep:none | @impl:TBD | @test:TBD

\```[lang]
// IFC definition — signatures only, no bodies
interface FooService {
  doThing(input: InputType): OutputType
  query(filter: Filter): Result[]
}
\```

---

### @mod:bar
> @file:src/bar/mod.rs | @dep:foo | @status:draft

[One paragraph: responsibility of this MOD]

**IFCs exposed:**

#### @ifc:BarHandler
> @dep:foo | @impl:TBD | @test:TBD

\```[lang]
interface BarHandler {
  handle(event: Event): void
}
\```

---

## Cross-MOD Communication

| Caller | IFC | Provider |
|--------|-----|----------|
| bar | FooService | foo |

---

## Out of Scope

- [Item 1 explicitly excluded]
- [Item 2]

---

## Open Questions

- [Any unresolved design decisions]
```

### Step 5: Notify Downstream Skills

After writing DES:

```
✅ DES-XXX written → .github/copilot/docs/ddd/DES-NNN-<slug>.md

Next steps:
- Run TDD to write tests for the IFCs defined
- Run PLAN to create implementation plan using this DES
- Run coding to implement (will read DES tags)
```

---

## Naming

Format: `DES-NNN-<slug>.md`
- `NNN` — zero-padded sequence; scan the DDD directory to find the highest existing number, increment by 1
- `slug` — kebab-case feature name, 2-4 words, derived from the feature title

Examples: `DES-001-sync-cli-command.md`, `DES-002-user-auth-jwt.md`

To update an existing DES: grep the DDD directory for a matching slug → open the file → edit in place. Never create a duplicate.

---

## Rules

- NEVER write implementation code in a DES
- NEVER skip the challenge phase
- NEVER produce a DES that doesn't tag every MOD and IFC
- ALWAYS prefer fewer MODs (merge if responsibilities overlap >50%)
- ALWAYS define an IFC for every cross-MOD dependency
