"""
Sync: persist and restore personal .github/copilot/ state to a per-user
git branch (copilot/<username>).
"""

import re
import subprocess
import sys
from pathlib import Path

PERSONAL_PATHS = [
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


def save(project_root: str = ".") -> None:
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

    _git(["push", "origin", branch, "--force-with-lease"], cwd=cwd, check=False)
    _git(["checkout", current], cwd=cwd)

    print(f"✅  Personal state saved to branch '{branch}' and pushed.")


def restore(project_root: str = ".") -> None:
    cwd = str(Path(project_root).resolve())
    branch = branch_name(cwd)

    if _is_dirty(cwd):
        print("⚠  Working tree has uncommitted changes. Commit or stash them first.")
        sys.exit(1)

    # Fetch the personal branch
    fetch = _git(["fetch", "origin", branch], cwd=cwd, check=False)
    if fetch.returncode != 0:
        print(f"✗ Branch '{branch}' not found on remote. Run `smart sync save` first.")
        sys.exit(1)

    # Checkout personal files onto the current branch
    for p in PERSONAL_PATHS:
        result = _git(
            ["checkout", f"origin/{branch}", "--", p],
            cwd=cwd,
            check=False,
        )
        icon = "✓" if result.returncode == 0 else "·"
        print(f"  {icon} {p}")

    print(f"\n✅  Personal state restored from branch '{branch}'.")
