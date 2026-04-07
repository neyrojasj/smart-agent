"""
Installer: downloads skills, agent, and templates from GitHub and copies them
into the target project's .github/ directory.
"""

import os
import urllib.request
from pathlib import Path

REPO_BASE = "https://raw.githubusercontent.com/neyrojasj/smart-agent/main"

SKILL_NAMES = [
    "planning",
    "plan-reviewer",
    "coding",
    "analysis",
    "documentation",
    "testing",
    "setup",
    "skill-generator",
    "evaluator",
    "ui-ux",
    "rust-web-app",
]

STANDARDS = [
    "general.md",
    "markdown.md",
    "rust.md",
    "nodejs.md",
    "python.md",
    "golang.md",
    "c.md",
    "cpp.md",
]

SHARED_FILES = [
    (".github/copilot-instructions.md", ".github/copilot-instructions.md"),
    (".github/agents/smart.agent.md", ".github/agents/smart.agent.md"),
    (".github/skills/index.yaml", ".github/skills/index.yaml"),
]

TEMPLATE_FILES = [
    (".github/copilot/context.md", ".github/copilot/context.md"),
    (".github/copilot/session.md", ".github/copilot/session.md"),
    (".github/copilot/instructions.md", ".github/copilot/instructions.md"),
    (".github/copilot/gitignore.txt", ".github/copilot/.gitignore"),
]


def _fetch(url: str) -> bytes:
    req = urllib.request.Request(
        url, headers={"User-Agent": "smartagent-cli/0.1"}
    )
    with urllib.request.urlopen(req) as resp:
        return resp.read()


def _write(dest: Path, data: bytes, force: bool) -> str:
    """Write data to dest. Returns 'created', 'skipped', or 'updated'."""
    if dest.exists() and not force:
        return "skipped"
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_bytes(data)
    return "updated" if dest.exists() else "created"


def install(
    target_dir: str = ".",
    standards: bool = True,
    minimal: bool = False,
    force: bool = False,
) -> None:
    root = Path(target_dir).resolve()

    pairs = list(SHARED_FILES)

    for skill in SKILL_NAMES:
        src = f".github/skills/{skill}/SKILL.md"
        pairs.append((src, src))

    if not minimal:
        pairs.extend(TEMPLATE_FILES)
        if standards:
            for s in STANDARDS:
                src = f".github/copilot/standards/{s}"
                pairs.append((src, src))

    for src_rel, dest_rel in pairs:
        url = f"{REPO_BASE}/{src_rel}"
        dest = root / dest_rel
        try:
            data = _fetch(url)
        except Exception as e:
            print(f"  ✗ {dest_rel} — download failed: {e}")
            continue

        # Personal files: never overwrite unless --force
        is_personal = "copilot/context" in src_rel or "copilot/session" in src_rel
        result = _write(dest, data, force=force and not is_personal)
        icon = "✓" if result != "skipped" else "·"
        print(f"  {icon} {dest_rel} [{result}]")

    # Always create these dirs
    for d in [".github/copilot/plans", ".github/copilot/docs", ".github/copilot/tmp"]:
        (root / d).mkdir(parents=True, exist_ok=True)

    print()
    print("✅  Smart Copilot installed. Run `@smart setup project` in Copilot Chat to initialize.")


def update(target_dir: str = ".", dry_run: bool = False) -> None:
    """Re-download only shared/skill files — never personal files."""
    root = Path(target_dir).resolve()

    pairs = list(SHARED_FILES)
    for skill in SKILL_NAMES:
        src = f".github/skills/{skill}/SKILL.md"
        pairs.append((src, src))

    for src_rel, dest_rel in pairs:
        dest = root / dest_rel
        url = f"{REPO_BASE}/{src_rel}"
        if dry_run:
            print(f"  · {dest_rel} [would update]")
            continue
        try:
            data = _fetch(url)
        except Exception as e:
            print(f"  ✗ {dest_rel} — {e}")
            continue
        _write(dest, data, force=True)
        print(f"  ✓ {dest_rel} [updated]")

    if not dry_run:
        print()
        print("✅  Skills and agent updated.")
