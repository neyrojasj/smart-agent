# DES-001: `smart sync` — Sync local files from personal copilot branch

> @status:draft | Created: 2026-05-05 | Stack: Python, Click, Git

## Overview

Adds a `smart sync` CLI command that merges files saved on the user's personal `copilot/<user>` branch back into the working tree. The command stashes local changes before applying branch files (unless `--force`), then pops the stash so the user resolves any conflicts. A `--dry-run` flag shows what would change without modifying anything.

---

## MODs

### @mod:sync
> @file:smartagent/sync.py | @status:draft

Responsible for all git-based personal-state operations: save, restore, and now sync. The new `sync()` function fetches the personal branch, optionally stashes the working tree, applies all `PERSONAL_PATHS` files from the branch, and restores the stash. It never deletes any branch.

**IFCs exposed:**

#### @ifc:SyncFunc
> @dep:none | @impl:TBD | @test:tests/test_sync.py

```python
# IFC definition — signatures only, no bodies

def sync(project_root: str = ".", force: bool = False, dry_run: bool = False) -> None:
    """
    Sync all PERSONAL_PATHS from the user's copilot/<user> branch into the working tree.

    Args:
        project_root: Path to the git repository root.
        force:        If True, skip stash and directly overwrite local files from branch.
                      If False (default), stash local changes first, apply branch files,
                      then pop stash — leaving merge conflicts for the user to resolve.
        dry_run:      If True, print what would be changed and exit without modifying anything.

    Exits with non-zero status if:
        - The personal branch does not exist on the remote.
        - Git operations fail unexpectedly.
    """
    ...
```

---

### @mod:cli
> @file:smartagent/cli.py | @dep:sync | @status:draft

Responsible for CLI command definitions. Adds the `sync` click command that wires user flags to `@ifc:SyncFunc`.

**IFCs exposed:**

#### @ifc:SyncCommand
> @dep:sync | @impl:TBD | @test:tests/test_cli.py::TestSyncSyncCommand

```python
# IFC definition — signatures only, no bodies

@main.command()
@click.option("--force", "-f", is_flag=True, default=False,
              help="Overwrite local files from branch, skipping stash.")
@click.option("--dry-run", is_flag=True, default=False,
              help="Show what would be synced without making changes.")
@click.option("--target", default=".", show_default=True, help="Project root.")
def sync(force: bool, dry_run: bool, target: str) -> None:
    """Sync all personal files from copilot/<user> branch into the working tree."""
    ...
```

---

## Cross-MOD Communication

| Caller | IFC | Provider |
|--------|-----|----------|
| cli | SyncCommand → SyncFunc | sync |

---

## Behavior Details

### Default (no `--force`)
1. Verify `origin/copilot/<user>` exists — if not, print error and exit.
2. `git fetch origin copilot/<user>`
3. `git stash push --include-untracked -m "smart sync: pre-sync stash"`
4. For each path in `PERSONAL_PATHS`: `git checkout origin/copilot/<user> -- <path>` (skip silently if path missing on branch)
5. Unstage all applied files: `git reset HEAD -- <paths>`
6. `git stash pop` — git surfaces conflict markers; user resolves manually.
7. Print summary of applied paths.

### `--force`
1. Verify `origin/copilot/<user>` exists — if not, print error and exit.
2. `git fetch origin copilot/<user>`
3. For each path in `PERSONAL_PATHS`: `git checkout origin/copilot/<user> -- <path>`
4. Unstage all applied files.
5. Print summary. No stash involved — local changes to those paths are overwritten.

### `--dry-run`
1. Verify `origin/copilot/<user>` exists — if not, print error and exit.
2. `git fetch origin copilot/<user>`
3. For each path in `PERSONAL_PATHS`: run `git diff HEAD origin/copilot/<user> -- <path>` and report whether the file differs or is absent.
4. Print report. Exit without modifying anything.

### Branch-not-found
- Print: `✗ Branch 'copilot/<user>' not found on remote. Run 'smart save' first.`
- Exit with code 1.

---

## Out of Scope

- Auto-resolving merge conflicts
- Syncing to or from any branch other than `copilot/<user>`
- Deleting any branch at any point
- Interactively selecting which paths to sync

---

## Open Questions

- None — all decisions confirmed by user.
