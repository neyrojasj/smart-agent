+++
addresses = ["REQ-042 v2 37857355", "REQ-043 v3 ceb81bfe", "REQ-044 v2 a74590fa", "REQ-045 v2 12e173a6", "REQ-046 v3 fb90b6b7"]
+++

# 0014 — Dynamic verification via a configured test command

*Status: Accepted. Amends ADR 0003 (does not supersede it).*

weft gains a narrow, opt-in dynamic capability. A `[test]` section in
`weft.toml` declares a default **Test Command**, with optional per-FEAT or
per-requirement overrides. `weft test` runs the command responsible for each
requirement and records pass/fail into a committed **Run Lock**
(`docs/prds/weft.run.toml`), the dynamic analogue of `weft.lock`, pinning each
result to the requirement's Content Hash and annotated-file hashes at run
time. **Verified** is a new Trace State strictly above `Traced`: Traced *and*
a recorded pass at the current hashes. Editing a file or bumping a requirement
invalidates the pinned run and drops it below Verified. `weft gate` is a
distinct command that exits zero only when every active requirement is
Verified — the agent's single, unfakeable loop-termination check, kept
separate from `weft check` so `check`'s existing exit contract is unchanged.
`weft next` emits the one highest-priority not-yet-Verified requirement,
regressions-first, with an explicit action verb (`implement` | `rework` |
`reseal` | `fix-tests` | `run-tests`) so a harness can drive the loop
deterministically: `next -> (implement | rework | reseal | fix-tests) -> seal
-> test -> gate`, repeating until `gate` is green.

## Context

ADR 0003 fixed weft as a static analyzer that never executes tests —
"keeping the linked tests green is the user's responsibility (CI, their own
discipline)." That rationale rested on two pillars: (1) language-agnosticism
— running tests "would force the tool to know every language's runner, build
env, and flakes"; and (2) redundancy — "a guarantee CI already provides." A
new use case stresses pillar 2: an AI agent driving a project from
requirement to implementation with no human watching CI. For that agent,
Trace State *is* the loop's control signal — but `Traced` means only that the
three trace links exist and are fresh, not that the linked test passes. ADR
0009 already recognised the danger of a false completeness signal ("a false
`Traced` sends the AI forward on broken ground") for file drift; the same
danger applies to a `Traced` requirement whose test is red or absent at
runtime. CI being "in the loop" is exactly the assumption that breaks for
autonomous operation.

## Why

- **Attribution is honest, not invented.** With only a default command, all
  requirements sharing it rise and fall on one exit code; `Verified` means
  "the Test Command responsible for this requirement passed," NOT "this
  requirement's specific test provably executed." weft cannot prove the
  latter without per-language integration, so it does not claim it.
  Per-requirement overrides buy sharper attribution where it matters — paid
  for in config, not in weft learning languages.
- **Opt-in.** With no `[test]` section, weft behaves exactly as before;
  `Verified` is simply unreachable and the static chain is unchanged.
  Existing users see no behavioural change.
- **A separate, committed Run Lock** keeps ADR 0009's clean "file path ->
  hash" model intact and makes a passing run a reviewable, hash-pinned claim
  rather than ambient machine state.

## Consequences

- weft is no longer purely static; this ADR amends ADR 0003's "never executes
  tests" for the configured-command case only. ADR 0003's static-analysis-of-
  text core and language-agnostic guarantee remain in force.
- A new committed artifact, `docs/prds/weft.run.toml`, must be committed
  alongside source like `weft.lock`. A stale Run Lock simply means affected
  requirements are not `Verified` — it is not, by itself, a build failure the
  way a missing `weft.lock` is.
- `Verified` is environment-sensitive: a result recorded in one environment is
  trusted by `check`/`gate` elsewhere only insofar as the team trusts the
  committed claim; CI re-running `weft test` produces a fresh, authoritative
  Run Lock.
- "Done" for the autonomous loop now has a precise dynamic meaning (`gate`
  green = all `Verified`), strictly stronger than the static "`check` green"
  of ADR 0003.
