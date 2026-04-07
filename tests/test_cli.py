"""
Tests for the smartagent CLI commands.

Uses Click's test runner so no real network calls or git operations are made.
All external I/O (installer.install, installer.update, sync.save, sync.restore)
is patched out.
"""

import unittest
from unittest.mock import patch

from click.testing import CliRunner

from smartagent.cli import main
from smartagent import __version__


class TestCliHelp(unittest.TestCase):
    """Verify the top-level CLI and each command respond to --help."""

    def setUp(self):
        self.runner = CliRunner()

    def test_help_exits_zero(self):
        result = self.runner.invoke(main, ["--help"])
        self.assertEqual(result.exit_code, 0)
        self.assertIn("install", result.output.lower())

    def test_version(self):
        result = self.runner.invoke(main, ["--version"])
        self.assertEqual(result.exit_code, 0)
        self.assertIn(__version__, result.output)

    def test_init_help(self):
        result = self.runner.invoke(main, ["init", "--help"])
        self.assertEqual(result.exit_code, 0)
        self.assertIn("--no-standards", result.output)
        self.assertIn("--minimal", result.output)
        self.assertIn("--force", result.output)

    def test_sync_save_help(self):
        result = self.runner.invoke(main, ["save", "--help"])
        self.assertEqual(result.exit_code, 0)

    def test_sync_restore_help(self):
        result = self.runner.invoke(main, ["restore", "--help"])
        self.assertEqual(result.exit_code, 0)

    def test_update_help(self):
        result = self.runner.invoke(main, ["update", "--help"])
        self.assertEqual(result.exit_code, 0)
        self.assertIn("--dry-run", result.output)


class TestInitCommand(unittest.TestCase):
    """smart init calls installer.install with the right arguments."""

    def setUp(self):
        self.runner = CliRunner()

    @patch("smartagent.cli.installer.install")
    def test_init_defaults(self, mock_install):
        result = self.runner.invoke(main, ["init", "--target", "/tmp/proj"])
        self.assertEqual(result.exit_code, 0)
        mock_install.assert_called_once_with(
            target_dir="/tmp/proj",
            standards=True,
            minimal=False,
            force=False,
        )

    @patch("smartagent.cli.installer.install")
    def test_init_no_standards(self, mock_install):
        result = self.runner.invoke(main, ["init", "--no-standards", "--target", "/tmp/proj"])
        self.assertEqual(result.exit_code, 0)
        mock_install.assert_called_once_with(
            target_dir="/tmp/proj",
            standards=False,
            minimal=False,
            force=False,
        )

    @patch("smartagent.cli.installer.install")
    def test_init_minimal(self, mock_install):
        result = self.runner.invoke(main, ["init", "--minimal", "--target", "/tmp/proj"])
        self.assertEqual(result.exit_code, 0)
        mock_install.assert_called_once_with(
            target_dir="/tmp/proj",
            standards=True,
            minimal=True,
            force=False,
        )

    @patch("smartagent.cli.installer.install")
    def test_init_force(self, mock_install):
        result = self.runner.invoke(main, ["init", "--force", "--target", "/tmp/proj"])
        self.assertEqual(result.exit_code, 0)
        mock_install.assert_called_once_with(
            target_dir="/tmp/proj",
            standards=True,
            minimal=False,
            force=True,
        )


class TestUpdateCommand(unittest.TestCase):
    """smart update calls installer.update with the right arguments."""

    def setUp(self):
        self.runner = CliRunner()

    @patch("smartagent.cli.installer.update")
    def test_update_defaults(self, mock_update):
        result = self.runner.invoke(main, ["update", "--target", "/tmp/proj"])
        self.assertEqual(result.exit_code, 0)
        self.assertIn("Updating", result.output)
        mock_update.assert_called_once_with(target_dir="/tmp/proj", dry_run=False)

    @patch("smartagent.cli.installer.update")
    def test_update_dry_run(self, mock_update):
        result = self.runner.invoke(main, ["update", "--dry-run", "--target", "/tmp/proj"])
        self.assertEqual(result.exit_code, 0)
        self.assertIn("Dry run", result.output)
        mock_update.assert_called_once_with(target_dir="/tmp/proj", dry_run=True)


class TestSyncCommands(unittest.TestCase):
    """smart save/restore call sync.save / sync.restore."""

    def setUp(self):
        self.runner = CliRunner()

    @patch("smartagent.cli.sync.save")
    def test_sync_save(self, mock_save):
        result = self.runner.invoke(main, ["save", "--target", "/tmp/proj"])
        self.assertEqual(result.exit_code, 0)
        mock_save.assert_called_once_with(project_root="/tmp/proj", force=False)

    @patch("smartagent.cli.sync.save")
    def test_sync_save_force(self, mock_save):
        result = self.runner.invoke(main, ["save", "--target", "/tmp/proj", "--force"])
        self.assertEqual(result.exit_code, 0)
        mock_save.assert_called_once_with(project_root="/tmp/proj", force=True)

    @patch("smartagent.cli.sync.save")
    def test_sync_save_force_short(self, mock_save):
        result = self.runner.invoke(main, ["save", "--target", "/tmp/proj", "-f"])
        self.assertEqual(result.exit_code, 0)
        mock_save.assert_called_once_with(project_root="/tmp/proj", force=True)

    @patch("smartagent.cli.sync.restore")
    def test_sync_restore(self, mock_restore):
        result = self.runner.invoke(main, ["restore", "--target", "/tmp/proj"])
        self.assertEqual(result.exit_code, 0)
        mock_restore.assert_called_once_with(project_root="/tmp/proj", force=False)

    @patch("smartagent.cli.sync.restore")
    def test_sync_restore_force(self, mock_restore):
        result = self.runner.invoke(main, ["restore", "--target", "/tmp/proj", "--force"])
        self.assertEqual(result.exit_code, 0)
        mock_restore.assert_called_once_with(project_root="/tmp/proj", force=True)

    @patch("smartagent.cli.sync.restore")
    def test_sync_restore_force_short(self, mock_restore):
        result = self.runner.invoke(main, ["restore", "--target", "/tmp/proj", "-f"])
        self.assertEqual(result.exit_code, 0)
        mock_restore.assert_called_once_with(project_root="/tmp/proj", force=True)


if __name__ == "__main__":
    unittest.main()
