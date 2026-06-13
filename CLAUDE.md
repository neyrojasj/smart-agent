## Agent skills

### Issue tracker

Issues are tracked as local markdown files under `.scratch/`. See `docs/agents/issue-tracker.md`.

### Triage labels

Triage uses the default five-label vocabulary with no overrides. See `docs/agents/triage-labels.md`.

### Domain docs

Domain docs are single-context (root `CONTEXT.md` + root `docs/adr/`). See `docs/agents/domain.md`.

### Architecture

New and touched code follows deep-modules + SOLID-via-traits. See `docs/agents/architecture.md`.

### Research delegation

If you are running as Sonnet or Opus, delegate file investigation — `Read`,
`Grep`, `Glob`, or `grep`/`find` via Bash — to a subagent running on Haiku
via the Agent tool (`model: "haiku"`, `subagent_type: "Explore"` for pure
search/navigation or `"general-purpose"` for broader research). This keeps
the main session's context window focused on synthesis and decisions rather
than raw search output.

Construct the subagent prompt with:

1. **GOAL** — the exact question to answer or fact to locate.
2. **SCOPE** — exact paths, directories, file globs, or grep patterns to
   search. Be specific; do not make the subagent guess where to look.
3. **OUTPUT FORMAT** — the precise shape of the answer needed back (e.g. a
   list of `file:line` matches with one line of context each, the current
   value of X, yes/no plus the deciding evidence).
4. **CONTEXT** — naming conventions, what to ignore, edge cases, and anything
   else needed to interpret results correctly without follow-up questions.
5. **EXHAUSTIVENESS** — be thorough within scope, then stop and report — do
   not broaden the search on its own.

**Research subagents are read-only.** They must not edit files, run commands
that change repository state, or create commits — their job is to report
findings back, not to act on them.

Small, single-file lookups where you already know the exact path don't need
delegation — reserve it for open-ended searches across the codebase.
