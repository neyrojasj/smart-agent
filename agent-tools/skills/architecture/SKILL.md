---
name: architecture
description: >
  Load when writing, reviewing, or refactoring code. Covers the deep-modules
  principle and SOLID-via-traits conventions used in weft-core and weft.
version: "1.0"
---

<!-- @implements REQ-051 v2 16a337fc -->

# Architecture: deep modules, SOLID via traits

How code in `weft-core` and `weft` should be structured as it grows beyond a
single `lib.rs` / `main.rs`.

## Deep modules

From *A Philosophy of Software Design* (Ousterhout): a good module has a
**small interface and a deep implementation** — it does a lot, but exposes
little.

- Before adding a public item, ask: does the caller need this, or is it an
  implementation detail leaking out?
- Prefer one method that does the right thing over several methods the caller
  must sequence correctly.
- A module whose public surface is as big as its implementation (a "shallow"
  module) is a sign the boundary is in the wrong place — fold it into its
  caller or merge it with a neighbor.

## SOLID, in Rust terms

- **Single Responsibility** — a module corresponds to one concept from
  `CONTEXT.md` (e.g. Seal, Trace Link, Requirement). If a module's
  responsibility needs "and" to describe, split it.
- **Open/Closed** — extend behavior by adding new implementations of a trait,
  not by adding `match` arms / flags to existing code.
- **Liskov** — any implementor of a trait must be substitutable without the
  caller special-casing it. If a `match` on the concrete type creeps back in
  at call sites, the trait abstraction is wrong.
- **Interface Segregation** — keep traits small and focused on one capability.
  Split a trait rather than adding a method only some implementors support
  (no `unimplemented!()` methods).
- **Dependency Inversion** — modules that orchestrate (e.g. `weft`'s CLI
  commands) depend on traits defined by the modules they consume, not on
  concrete types. Concrete implementations live behind the trait, swappable
  for tests.

## Module shape

- A module's public API is a **trait** (the "interface") plus the minimal set
  of types needed to call it. Everything else — structs, helpers, internal
  state — is `pub(crate)` or private.
- Tests for a module exercise it through its trait, not its internals.
- When a new capability is needed, prefer adding a method to an existing deep
  module's trait (if it fits the responsibility) over creating a new shallow
  module.

## When code already in the repo conflicts with this

Don't mass-refactor unprompted. Apply this shape to new modules and to code
you're already touching for another reason; flag larger violations as a
suggestion rather than doing them inline.
