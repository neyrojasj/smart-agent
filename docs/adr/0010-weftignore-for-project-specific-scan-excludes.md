+++
addresses = ["REQ-034 v2 b68c2987", "REQ-035 v2 1e646999"]
+++

# 0010 — `.weftignore` for project-specific scan excludes

`weft check` and `weft seal` walk the whole project tree looking for Trace
Link annotations, skipping a hardcoded `SCAN_EXCLUDES` list (`.git`, `target`,
`node_modules`). An optional `.weftignore` file at the project root adds
project-specific names (files or directories) to that exclude list. Each
non-empty, non-`#` line is a literal basename (a trailing `/` is stripped),
matched anywhere in the tree — the same matching style as `SCAN_EXCLUDES`,
just user-extensible. `weft init` writes a default `.weftignore` containing
`.scratch` and `logs`.

## Why

Files under `.scratch/` (issue drafts that embed example `@implements`/
`@verifies` annotations for copy-paste) and `logs/` (AI agent transcripts that
echo annotation-shaped text) were being scanned, producing false Trace Links.
Once `weft seal` ran, those files' hashes landed in `weft.lock`, and every
subsequent `weft check` reported the affected requirements as `Drifted`
because the log files keep changing. The tool needs a way to keep these
non-source files out of the scan entirely, and that exclude set is
project-specific (a different repo may stage its scratch work elsewhere).

## Considered Options

**Basename-literal `.weftignore`, additive to `SCAN_EXCLUDES` (chosen).**
Minimal: a handful of lines added to `find_scannable_files`, no new
dependency. Matches the existing `SCAN_EXCLUDES` mental model exactly — one
name, skipped anywhere in the tree — just made user-configurable via a file.
Sufficient for the motivating cases (`.scratch`, `logs`), which are simple
named directories, not patterns.

**Full `.gitignore` syntax via the `ignore` crate.** Supports globs,
negation, and nested ignore files. Rejected: pulls in a non-trivial dependency
and a much larger matching surface for a problem that's just "skip these
named directories." Conflicts with the project's "minimum toolbox" stance
(ADR-0004). A `.weftignore` file using `.gitignore`-style naming but
basename-only semantics also risks confusing users who expect glob/negation
support — worth remembering if this is ever revisited.

**Fully configurable exclude list (replaces `SCAN_EXCLUDES`).** Rejected:
`.git`, `target`, and `node_modules` should never be scanned regardless of
project config; making them overridable invites accidental misconfiguration
for no benefit.

## Consequences

- `.weftignore` is optional; a missing file means zero extra excludes (today's
  behavior, unchanged).
- Entries match by basename only — `logs` excludes any directory or file
  named `logs` at any depth, not just a top-level one. There is no glob or
  negation support, despite the `.gitignore`-inspired name.
- `weft init` seeds `.weftignore` with `.scratch` and `logs`, the two
  directories this toolchain itself creates that aren't source/test
  artifacts.
- Existing projects (including this one) need to add `.weftignore` manually
  once. After adding it, a full `weft seal` will prune the now-excluded
  `logs/ralph/*` entries from `weft.lock` (full seal rebuilds the lock from
  the current scan, pruning files with no remaining annotations).
