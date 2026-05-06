"""
TST-001: sync() — unit tests for @ifc:SyncFunc
@ifc:SyncFunc | @mod:sync | @impl:TBD

RUN: python -m pytest tests/test_sync.py -v
"""

import unittest
from unittest.mock import MagicMock, patch

from smartagent.sync import sync, PERSONAL_PATHS


def _ok(stdout=""):
    r = MagicMock()
    r.stdout = stdout
    r.stderr = ""
    r.returncode = 0
    return r


class TestSyncDefault(unittest.TestCase):
    """sync() with default args (force=False, dry_run=False)."""

    # @case: clean working tree — no stash, fetch + checkout + reset
    @patch("smartagent.sync._branch_exists_remotely", return_value=True)
    @patch("smartagent.sync._is_dirty", return_value=False)
    @patch("smartagent.sync.get_username", return_value="alice")
    @patch("smartagent.sync._git")
    def test_clean_tree_fetches_and_applies(self, mock_git, *_):
        # ARRANGE
        mock_git.return_value = _ok()
        # ACT
        sync(project_root="/tmp/repo")
        # ASSERT
        cmds = [c.args[0] for c in mock_git.call_args_list]
        self.assertTrue(any(c[:2] == ["fetch", "origin"] for c in cmds), "fetch not called")
        self.assertFalse(any("stash" in c for c in cmds), "stash must not run on clean tree")

    # @case: dirty working tree — stash push before, stash pop after (ordered)
    @patch("smartagent.sync._branch_exists_remotely", return_value=True)
    @patch("smartagent.sync._is_dirty", return_value=True)
    @patch("smartagent.sync.get_username", return_value="alice")
    @patch("smartagent.sync._git")
    def test_dirty_tree_stash_push_and_pop(self, mock_git, *_):
        # ARRANGE
        mock_git.return_value = _ok()
        # ACT
        sync(project_root="/tmp/repo")
        # ASSERT
        cmds = [c.args[0] for c in mock_git.call_args_list]
        stash_cmds = [c for c in cmds if "stash" in c]
        self.assertEqual(len(stash_cmds), 2, "expected stash push + stash pop")
        self.assertIn("push", stash_cmds[0], "first stash op must be push")
        self.assertIn("pop", stash_cmds[1], "second stash op must be pop")

    # @case: all PERSONAL_PATHS are checked out from origin/copilot/<user>
    @patch("smartagent.sync._branch_exists_remotely", return_value=True)
    @patch("smartagent.sync._is_dirty", return_value=False)
    @patch("smartagent.sync.get_username", return_value="alice")
    @patch("smartagent.sync._git")
    def test_checkouts_all_personal_paths(self, mock_git, *_):
        # ARRANGE
        mock_git.return_value = _ok()
        # ACT
        sync(project_root="/tmp/repo")
        # ASSERT
        cmds = [c.args[0] for c in mock_git.call_args_list]
        checkout_cmds = [
            c for c in cmds
            if c[0] == "checkout" and any("origin/copilot/" in a for a in c)
        ]
        self.assertEqual(len(checkout_cmds), len(PERSONAL_PATHS))

    # @case: applied files are unstaged after checkout
    @patch("smartagent.sync._branch_exists_remotely", return_value=True)
    @patch("smartagent.sync._is_dirty", return_value=False)
    @patch("smartagent.sync.get_username", return_value="alice")
    @patch("smartagent.sync._git")
    def test_unstages_applied_files(self, mock_git, *_):
        # ARRANGE
        mock_git.return_value = _ok()
        # ACT
        sync(project_root="/tmp/repo")
        # ASSERT
        cmds = [c.args[0] for c in mock_git.call_args_list]
        reset_cmds = [c for c in cmds if c[0] == "reset" and "HEAD" in c]
        self.assertTrue(reset_cmds, "git reset HEAD not called")

    # @case: branch not found → exit(1)
    @patch("smartagent.sync._branch_exists_remotely", return_value=False)
    @patch("smartagent.sync.get_username", return_value="alice")
    def test_missing_branch_exits_1(self, *_):
        # ACT & ASSERT
        with self.assertRaises(SystemExit) as ctx:
            sync(project_root="/tmp/repo")
        self.assertEqual(ctx.exception.code, 1)

    # @case: branch not found → error message mentions "smart save"
    @patch("builtins.print")
    @patch("smartagent.sync._branch_exists_remotely", return_value=False)
    @patch("smartagent.sync.get_username", return_value="alice")
    def test_missing_branch_prints_save_hint(self, mock_username, mock_exists, mock_print):
        # ACT
        with self.assertRaises(SystemExit):
            sync(project_root="/tmp/repo")
        # ASSERT
        all_output = " ".join(str(a) for c in mock_print.call_args_list for a in c.args)
        self.assertIn("smart save", all_output)


class TestSyncForce(unittest.TestCase):
    """sync() with force=True."""

    # @case: --force with dirty tree → no stash operations
    @patch("smartagent.sync._branch_exists_remotely", return_value=True)
    @patch("smartagent.sync._is_dirty", return_value=True)
    @patch("smartagent.sync.get_username", return_value="alice")
    @patch("smartagent.sync._git")
    def test_force_skips_stash_on_dirty_tree(self, mock_git, *_):
        # ARRANGE
        mock_git.return_value = _ok()
        # ACT
        sync(project_root="/tmp/repo", force=True)
        # ASSERT
        cmds = [c.args[0] for c in mock_git.call_args_list]
        self.assertFalse(any("stash" in c for c in cmds), "stash must not run with --force")

    # @case: --force still applies all PERSONAL_PATHS
    @patch("smartagent.sync._branch_exists_remotely", return_value=True)
    @patch("smartagent.sync._is_dirty", return_value=True)
    @patch("smartagent.sync.get_username", return_value="alice")
    @patch("smartagent.sync._git")
    def test_force_applies_all_personal_paths(self, mock_git, *_):
        # ARRANGE
        mock_git.return_value = _ok()
        # ACT
        sync(project_root="/tmp/repo", force=True)
        # ASSERT
        cmds = [c.args[0] for c in mock_git.call_args_list]
        checkout_cmds = [
            c for c in cmds
            if c[0] == "checkout" and any("origin/copilot/" in a for a in c)
        ]
        self.assertEqual(len(checkout_cmds), len(PERSONAL_PATHS))

    # @case: --force does not bypass branch-not-found check
    @patch("smartagent.sync._branch_exists_remotely", return_value=False)
    @patch("smartagent.sync.get_username", return_value="alice")
    def test_force_missing_branch_exits_1(self, *_):
        # ACT & ASSERT
        with self.assertRaises(SystemExit) as ctx:
            sync(project_root="/tmp/repo", force=True)
        self.assertEqual(ctx.exception.code, 1)


class TestSyncDryRun(unittest.TestCase):
    """sync() with dry_run=True."""

    # @case: --dry-run → no checkout and no stash operations
    @patch("smartagent.sync._branch_exists_remotely", return_value=True)
    @patch("smartagent.sync.get_username", return_value="alice")
    @patch("smartagent.sync._git")
    def test_dry_run_no_checkout_or_stash(self, mock_git, *_):
        # ARRANGE
        mock_git.return_value = _ok()
        # ACT
        sync(project_root="/tmp/repo", dry_run=True)
        # ASSERT
        cmds = [c.args[0] for c in mock_git.call_args_list]
        self.assertFalse(any(c[0] == "checkout" for c in cmds), "checkout must not run in --dry-run")
        self.assertFalse(any("stash" in c for c in cmds), "stash must not run in --dry-run")

    # @case: --dry-run runs git diff for each PERSONAL_PATH
    @patch("smartagent.sync._branch_exists_remotely", return_value=True)
    @patch("smartagent.sync.get_username", return_value="alice")
    @patch("smartagent.sync._git")
    def test_dry_run_diffs_each_personal_path(self, mock_git, *_):
        # ARRANGE
        mock_git.return_value = _ok()
        # ACT
        sync(project_root="/tmp/repo", dry_run=True)
        # ASSERT
        cmds = [c.args[0] for c in mock_git.call_args_list]
        diff_cmds = [c for c in cmds if c[0] == "diff"]
        self.assertEqual(len(diff_cmds), len(PERSONAL_PATHS))

    # @case: --dry-run does not bypass branch-not-found check
    @patch("smartagent.sync._branch_exists_remotely", return_value=False)
    @patch("smartagent.sync.get_username", return_value="alice")
    def test_dry_run_missing_branch_exits_1(self, *_):
        # ACT & ASSERT
        with self.assertRaises(SystemExit) as ctx:
            sync(project_root="/tmp/repo", dry_run=True)
        self.assertEqual(ctx.exception.code, 1)


if __name__ == "__main__":
    unittest.main()
