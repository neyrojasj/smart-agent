"""
Sync: persist and restore personal .github/copilot/ state to a per-user
git branch (copilot/<username>).
"""

import re
import subprocess
import sys
from pathlib import Path

PERSONAL_PATHS = [
    ".github/agents",
    ".github/copilot/skills",
    ".github/copilot-instructions.md",
    ".github/copilot/context.md",
    ".github/copilot/session.md",
    ".github/copilot/instructions.md",
    ".github/copilot/plans",
    ".github/copilot/docs",
]


def _git(args: list, cwd: str, check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(
        ["git"] + args,
        cwd=cwd,
        capture_output=True,
        text=True,
        check=check,
    )


def get_username(cwd: str) -> str:
    result = _git(["config", "user.email"], cwd=cwd)
    email = result.stdout.strip()
    if not email:
        print("✗ git user.email is not set. Run: git config user.email <your@email.com>")
        sys.exit(1)
    # Sanitize: keep alphanumeric and hyphens, replace everything else with hyphen
    sanitized = re.sub(r"[^a-zA-Z0-9-]", "-", email).strip("-")
    return sanitized


def branch_name(cwd: str) -> str:
    return f"copilot/{get_username(cwd)}"


def _is_dirty(cwd: str) -> bool:
    result = _git(["status", "--porcelain"], cwd=cwd)
    return bool(result.stdout.strip())


def _branch_exists_remotely(branch: str, cwd: str) -> bool:
    result = _git(["ls-remote", "--heads", "origin", branch], cwd=cwd, check=False)
    return bool(result.stdout.strip())


def save(project_root: str = ".", force: bool = False) -> None:
    cwd = str(Path(project_root).resolve())
    branch = branch_name(cwd)

    personal_files = [
        p for p in PERSONAL_PATHS
        if (Path(cwd) / p).exists()
    ]

    if not personal_files:
        print("· No personal files found to save.")
        return

    current = _git(["rev-parse", "--abbrev-ref", "HEAD"], cwd=cwd).stdout.strip()

    # Create or reset the personal branch from current HEAD (orphan-safe)
    remote_exists = _branch_exists_remotely(branch, cwd)

    if remote_exists:
        # Fetch the branch so we can reset it
        _git(["fetch", "origin", branch], cwd=cwd, check=False)

    # Switch to personal branch (create if missing)
    branch_local = _git(["branch", "--list", branch], cwd=cwd).stdout.strip()
    if branch_local:
        _git(["checkout", branch], cwd=cwd)
    else:
        _git(["checkout", "-b", branch], cwd=cwd)

    # Cherry-pick personal files from the working state
    # We copy them as-is — they were already on disk while on the main branch
    for p in personal_files:
        _git(["add", "--force", p], cwd=cwd, check=False)

    result = _git(
        ["commit", "--allow-empty", "-m", f"sync: save personal state [{current}]"],
        cwd=cwd,
        check=False,
    )
    if result.returncode != 0 and "nothing to commit" not in result.stdout + result.stderr:
        print(f"✗ Commit failed: {result.stderr.strip()}")
        _git(["checkout", current], cwd=cwd, check=False)
        sys.exit(1)

    push_flags = ["push", "origin", branch, "--force"] if force else ["push", "origin", branch, "--force-with-lease"]
    _git(push_flags, cwd=cwd, check=False)
    _git(["checkout", current], cwd=cwd)

    print(f"✅  Personal state saved to branch '{branch}' and pushed.")


def sync(project_root: str = ".", force: bool = False, dry_run: bool = False) -> None:
    """
    Sync all PERSONAL_PATHS from the user's copilot/<user> branch into the working tree.

    Args:
        project_root: Path to the git repository root.
        force:        If True, skip stash and directly overwrite local files from branch.
        dry_run:      If True, print what would be changed and exit without modifying anything.
    """
    cwd = str(Path(project_root).resolve())
    username = get_username(cwd)
    branch = f"copilot/{username}"

    if not _branch_exists_remotely(branch, cwd):
        print(f"✗ Branch '{branch}' not found on remote. Run 'smart save' first.")
        sys.exit(1)

    _git(["fetch", "origin", branch], cwd=cwd)

    if dry_run:
        for path in PERSONAL_PATHS:
            _git(["diff", "HEAD", f"origin/{branch}", "--", path], cwd=cwd, check=False)
        return

    needs_stash = not force and _is_dirty(cwd)

    if needs_stash:
        _git(
            ["stash", "push", "--include-untracked", "-m", "smart sync: pre-sync stash"],
            cwd=cwd,
        )

    applied = []
    for path in PERSONAL_PATHS:
        result = _git(["checkout", f"origin/{branch}", "--", path], cwd=cwd, check=False)
        if result.returncode == 0:
            applied.append(path)

    if applied:
        _git(["reset", "HEAD", "--"] + applied, cwd=cwd, check=False)

    if needs_stash:
        _git(["stash", "pop"], cwd=cwd, check=False)

    print(f"✅  Personal state synced from branch '{branch}'.")
