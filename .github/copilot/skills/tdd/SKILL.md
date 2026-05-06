---
name: tdd
description: Test Driven Development — write tests before implementation, driven by DES IFCs and MOD boundaries.
version: "1.0"
---

# TDD Skill

## Identity

- **Name**: tdd
- **Version**: 1.0
- **Mode**: Full (tests written before any implementation)
- **Input**: DES (`DES-NNN-<slug>.md`) — required
- **Output**: `TST-NNN-<slug>.md` + actual test files

---

## Purpose

Write tests BEFORE implementation. All tests are derived from IFCs defined in the DES.
Test files are created empty of implementation (stubs/mocks only) so coding skill has a clear contract to satisfy.

---

## Triggers

| Pattern | Example |
|---------|---------|
| TDD, write tests, test first | "TDD for DESIGN-001" |
| "tests for [MOD/IFC]" | "tests for FooService" |
| chained from: DDD skill | after DES is written |
| chained from: fix skill | after DES update |

---

## TDD Principles

1. **Tests precede implementation** — test files exist before any implementation file
2. **Tests come from IFCs** — every `@ifc` in DES gets test cases
3. **Tests define the contract** — implementation must make tests pass, not vice versa
4. **Minimal stubs** — use interface stubs/mocks, not real implementations
5. **One test file per MOD** — mirrors the fewer-files MOD philosophy

---

## Workflow

### Step 1: Load Context

```
1. Read .github/copilot/glossary.md (terms)
2. Read .github/copilot/context.md (stack, test framework)
3. Find and read target DES:
   - If user specified DES slug/ID → grep `.github/copilot/docs/ddd/` for matching filename, open it
   - If not specified → list files in the DDD directory and ask which one
4. Read .github/copilot/docs/testing.md (test conventions) if exists
5. Scan existing test files for framework/pattern reference
```

### Step 2: Extract IFCs from DES

Parse the DES for all `@ifc` tags. For each IFC, extract:
- Method signatures
- Input/output types
- Dependencies (`@dep`)
- MOD it belongs to

Build a table:

| IFC | MOD | Methods | Test priority |
|-----|-----|---------|---------------|
| FooService | foo | doThing, query | high |
| BarHandler | bar | handle | high |

### Step 3: Plan Test Cases

For each IFC method, define test cases BEFORE writing code:

```markdown
**@ifc:FooService.doThing**
- ✅ happy path: valid input → expected output
- ❌ error: null input → throws InputError
- ❌ error: invalid type → throws ValidationError
- 🔄 edge: empty input → returns default
```

Ask user: "Any missing cases? Any IFCs to skip?" (one round only)

### Step 4: Write TST to Disk

Write `TST-NNN-<slug>.md` (slug matches the linked DES) as the test specification:

```markdown
# TST-XXX: [Title]

> Linked DES: DES-NNN-<slug>.md | @status:draft | Created: YYYY-MM-DD

## Test Matrix

| IFC | Method | Case | Expected | Priority |
|-----|--------|------|----------|----------|
| FooService | doThing | valid input | OutputType | high |
| FooService | doThing | null input | throws InputError | high |
| BarHandler | handle | valid event | void, side effect triggered | high |

## Coverage Goals

- All `@ifc` methods: 100%
- Error paths: 100%
- Edge cases: best-effort

## Test Files

| File | MOD | IFC(s) covered |
|------|-----|----------------|
| `tests/foo_test.[ext]` | foo | FooService |
| `tests/bar_test.[ext]` | bar | BarHandler |
```

### Step 5: Write Test Files

Create actual test files with:
- All test cases from the matrix as named test functions
- Stub/mock implementations for all dependencies
- Assertions that will FAIL until implementation exists (red phase)
- `@impl:TBD` comment — coding skill will fill this in

**Template per test file:**

```
// TST-XXX: [IFC] tests
// @ifc:[name] | @mod:[name] | @impl:TBD
//
// RUN: [test command]

[imports / mock setup]

describe("[IFC]", () => {
  // @case: valid input
  test("doThing returns OutputType for valid input", () => {
    // ARRANGE
    const svc = new MockFooService()
    // ACT
    const result = svc.doThing(validInput)
    // ASSERT
    expect(result).toMatchObject(expectedOutput)
  })

  // @case: null input
  test("doThing throws InputError for null", () => {
    expect(() => svc.doThing(null)).toThrow(InputError)
  })
})
```

### Step 6: Update DES Tags

After writing test files, update the DES to fill in `@test` tags:

```
@ifc:FooService → @test:tests/foo_test.[ext]
```

### Step 7: Notify

```
✅ TST-NNN-<slug>.md written → .github/copilot/docs/tdd/TST-NNN-<slug>.md
✅ Test files created: [list]
✅ DES updated: @test tags filled

All tests are RED (failing) — implementation needed.
Next: Run PLAN → coding to implement.
```

---

## Naming

Format: `TST-NNN-<slug>.md` — slug and NNN always match the linked DES.

Example: `DES-001-sync-cli-command.md` → `TST-001-sync-cli-command.md`

To update an existing TST: find it by slug/NNN, edit in place. Never duplicate.

---

## Rules

- NEVER write implementation code in test files (mocks/stubs only)
- NEVER skip tests for any `@ifc` in the DES
- Tests MUST fail before implementation (red phase verification)
- One test file per MOD (mirrors DES MOD structure)
- Always update DES `@test` tags after creating test files
