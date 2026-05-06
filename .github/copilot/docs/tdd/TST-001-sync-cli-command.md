# TST-001: `smart sync` — Sync local files from personal copilot branch

> Linked DES: DES-001-sync-cli-command.md | @status:draft | Created: 2026-05-05

---

## Test Matrix

| IFC | Method | Case | Expected | Priority |
|-----|--------|------|----------|----------|
| SyncFunc | sync | clean tree, branch exists | fetch + checkout PERSONAL_PATHS + reset, no stash | high |
| SyncFunc | sync | dirty tree, branch exists | stash push → checkout → reset → stash pop (ordered) | high |
| SyncFunc | sync | dirty tree + force=True | no stash; checkout + reset only | high |
| SyncFunc | sync | force=True | applies all PERSONAL_PATHS from branch | high |
| SyncFunc | sync | dry_run=True | git diff per path; no checkout, no stash | high |
| SyncFunc | sync | dry_run=True | exactly len(PERSONAL_PATHS) diff calls | high |
| SyncFunc | sync | branch not found | sys.exit(1) | high |
| SyncFunc | sync | branch not found | output mentions "smart save" | high |
| SyncFunc | sync | branch not found + force=True | sys.exit(1) | high |
| SyncFunc | sync | branch not found + dry_run=True | sys.exit(1) | high |
| SyncCommand | smart sync | no flags | sync.sync(project_root=".", force=False, dry_run=False) | high |
| SyncCommand | smart sync --force | --force flag | sync.sync called with force=True | high |
| SyncCommand | smart sync -f | -f short flag | sync.sync called with force=True | high |
| SyncCommand | smart sync --dry-run | --dry-run flag | sync.sync called with dry_run=True | high |
| SyncCommand | smart sync --target | --target /tmp/proj | sync.sync called with project_root="/tmp/proj" | high |
| SyncCommand | smart sync --help | help text | exit 0; output contains --force and --dry-run | medium |
| SyncCommand | smart sync --help (cli help) | top-level help coverage | "sync" appears in top-level --help output | medium |

---

## Coverage Goals

- All `@ifc` methods: 100%
- Error paths: 100%
- Edge cases: best-effort

---

## Test Files

| File | MOD | IFC(s) covered |
|------|-----|----------------|
| `tests/test_sync.py` | sync | SyncFunc |
| `tests/test_cli.py` | cli | SyncCommand |
