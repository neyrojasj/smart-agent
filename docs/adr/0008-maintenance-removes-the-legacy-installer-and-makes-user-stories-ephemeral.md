+++
addresses = [
    "REQ-029 v2 6f452ce5",
    "REQ-030 v2 bf5f866e",
]
+++

# 0008 — Maintenance removes the legacy installer and makes User Stories ephemeral

ADR 0001 established the pivot away from the Python `smart` installer and its
`save`/`sync` personal-branch feature, and from User Stories as a persisted
artifact. This ADR closes out that pivot: the legacy installer is gone, and
`weft verify` now actively rejects any record that persists a User Story
shape under `docs/prds/`.

## Considered Options

- **Verify-time rejection of User Story records (chosen).** `weft verify`
  parses each record's raw TOML and rejects any file whose top level
  contains a User Story field (`as_a`, `i_want`, `so_that`, `user_story`).
  Requirement records carry only `id`, `version`, `feat`, `hash`, `status`,
  `statement`, `acceptance`, `rationale`, and `notes` (ADR 1) — a User Story
  field at the top level means the file persists a User Story instead of, or
  alongside, a Requirement. This keeps the rule enforced by the same command
  authors already run before committing (ADR 1's `weft verify`), rather than
  relying on convention alone.
- **Convention only, documented in `to-smart-prd`.** Rejected on its own —
  `to-smart-prd` already documents "never persist User Stories" (ADR 7), but
  a documented convention with no enforcement can silently drift if a record
  is hand-edited or written by a different tool.
- **A separate `weft lint` command.** Rejected — would duplicate `verify`'s
  per-record file-walking for a single additional check; folding it into
  `verify` keeps one validation entry point (ADR 6's reasoning for not
  splitting `verify`/`check`).

## Consequences

- `weft verify` fails a record with `record contains '<field>', a User Story
  field — User Stories must never be persisted in docs/prds/; generate them
  ephemerally at implementation time` whenever `as_a`, `i_want`, `so_that`, or
  `user_story` appears at the record's top level.
- The `Command` enum in `weft/src/main.rs` carries no `save`/`sync`
  subcommand, by design — the legacy Python `smart` installer and its
  personal-branch save/sync feature (ADR 1) are not present anywhere in the
  repository.
- The repository contains no Python source files; the `smart` CLI's
  `pyproject.toml`, `smartagent/cli.py`, `installer.py`, and `sync.py` are
  gone entirely, leaving `weft` as the repository's single tool with a single
  purpose.
