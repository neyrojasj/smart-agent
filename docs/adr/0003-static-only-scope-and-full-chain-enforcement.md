# 0003 — Static-only scope and full-chain enforcement over atomic requirements

The tool is a **static analyzer**: it parses requirement records, scans text for
trace annotations, compares hashes, and reports. It **never executes tests** —
keeping the linked tests green is the user's responsibility (CI, their own
discipline), explicitly out of scope. A requirement is `Traced` only when **all
three** links exist and are current: `@addresses` (design) ∧ `@implements`
(code) ∧ `@verifies` (test). Missing any → `Incomplete`.

## Why

- **Static-only** keeps the tool's job tight and language-agnostic: a `@verifies
  REQ-042 v3 a3f9b2` marker reads the same in any comment syntax. Running tests
  would force the tool to know every language's runner, build env, and flakes —
  enormous scope creep for a guarantee CI already provides. The tool proves a
  linked test *exists and is current*, not that it passes.
- **Full-chain enforcement** is what makes "implemented" mean something: a
  passing test alone could exist with no recorded design rationale and no
  locatable implementation.

## Atomic requirements

Requirements are atomic — `Traced` or not, with the hash covering the whole
normative region. There is no partial/per-acceptance-criterion tracing.
Consequently a **vertical slice spans one or more *whole* requirements** taken to
`Traced`; if a requirement is too big to finish in one verifiable slice, that is
the signal to **split the requirement**, never to sub-divide its trace.

## Consequences

- "Verifiable when a slice finishes" has a precise meaning: `check` goes green
  for the slice's requirements.
- Sub-requirement granularity was rejected because it would explode the
  hash/trace model (every acceptance bullet would need its own identity and
  hash).
