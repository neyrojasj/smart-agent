import click

from smartagent import __version__
from smartagent import installer, sync


@click.group()
@click.version_option(__version__, prog_name="smart")
def main():
    """Smart Copilot CLI — install and manage GitHub Copilot skills."""
    pass


# ---------------------------------------------------------------------------
# smart init
# ---------------------------------------------------------------------------

@main.command()
@click.option(
    "--no-standards",
    is_flag=True,
    default=False,
    help="Skip installing language standards.",
)
@click.option(
    "--minimal",
    is_flag=True,
    default=False,
    help="Install only the agent and skills, no templates or standards.",
)
@click.option(
    "--force",
    is_flag=True,
    default=False,
    help="Overwrite existing skill/agent files.",
)
@click.option(
    "--target",
    default=".",
    show_default=True,
    help="Target project directory.",
)
def init(no_standards, minimal, force, target):
    """Install Smart Copilot into the current project."""
    click.echo("🚀  Installing Smart Copilot...\n")
    installer.install(
        target_dir=target,
        standards=not no_standards,
        minimal=minimal,
        force=force,
    )


# ---------------------------------------------------------------------------
# smart sync
# ---------------------------------------------------------------------------

@main.group()
def sync_group():
    """Sync personal Copilot state to/from a personal git branch."""
    pass


# register as 'sync' on the CLI
main.add_command(sync_group, name="sync")


@sync_group.command("save")
@click.option("--target", default=".", show_default=True, help="Project root.")
def sync_save(target):
    """Save personal state (context, plans, docs) to branch copilot/<user>."""
    sync.save(project_root=target)


@sync_group.command("restore")
@click.option("--target", default=".", show_default=True, help="Project root.")
def sync_restore(target):
    """Restore personal state from branch copilot/<user> onto working tree."""
    sync.restore(project_root=target)


# ---------------------------------------------------------------------------
# smart update
# ---------------------------------------------------------------------------

@main.command()
@click.option(
    "--dry-run",
    is_flag=True,
    default=False,
    help="Show what would be updated without making changes.",
)
@click.option(
    "--target",
    default=".",
    show_default=True,
    help="Target project directory.",
)
def update(dry_run, target):
    """Pull the latest skills and agent from upstream. Never touches personal files."""
    if dry_run:
        click.echo("🔍  Dry run — no files will be changed:\n")
    else:
        click.echo("🔄  Updating skills and agent...\n")
    installer.update(target_dir=target, dry_run=dry_run)
