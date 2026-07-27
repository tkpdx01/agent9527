#!/usr/bin/env python3

from datetime import date
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


sys.path.insert(0, str(Path(__file__).resolve().parent))

from finalize_upstream_sync import next_release_version
from finalize_upstream_sync import replace_tracked_version
from finalize_upstream_sync import write_upstream_marker


class NextReleaseVersionTest(unittest.TestCase):
    def test_starts_new_sequence_on_a_new_day(self) -> None:
        self.assertEqual(
            next_release_version("20260727.4.0", date(2026, 7, 28)),
            "20260728.1.0",
        )

    def test_increments_sequence_on_the_same_day(self) -> None:
        self.assertEqual(
            next_release_version("20260727.4.0", date(2026, 7, 27)),
            "20260727.5.0",
        )

    def test_rejects_non_release_versions(self) -> None:
        with self.assertRaisesRegex(ValueError, "YYYYMMDD.N.0"):
            next_release_version("0.1.0", date(2026, 7, 27))


class ReplaceTrackedVersionTest(unittest.TestCase):
    def test_only_updates_tracked_text_files(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            repo_root = Path(temp_dir)
            subprocess.run(["git", "init", "-q"], cwd=repo_root, check=True)
            tracked = repo_root / "tracked.txt"
            tracked.write_text("version=20260727.1.0\n", encoding="utf-8")
            binary = repo_root / "binary.bin"
            binary.write_bytes(b"\0version=20260727.1.0")
            deleted = repo_root / "deleted.txt"
            deleted.write_text("version=20260727.1.0\n", encoding="utf-8")
            untracked = repo_root / "untracked.txt"
            untracked.write_text("version=20260727.1.0\n", encoding="utf-8")
            subprocess.run(
                ["git", "add", "tracked.txt", "binary.bin", "deleted.txt"],
                cwd=repo_root,
                check=True,
            )
            deleted.unlink()

            changed = replace_tracked_version(repo_root, "20260727.1.0", "20260727.2.0")

            self.assertEqual(changed, [tracked])
            self.assertEqual(tracked.read_text(), "version=20260727.2.0\n")
            self.assertEqual(binary.read_bytes(), b"\0version=20260727.1.0")
            self.assertFalse(deleted.exists())
            self.assertEqual(untracked.read_text(), "version=20260727.1.0\n")


class WriteUpstreamMarkerTest(unittest.TestCase):
    def test_writes_complete_marker(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            marker_path = Path(temp_dir) / "upstream.json"
            write_upstream_marker(
                marker_path,
                upstream_sha="a" * 40,
                version="20260727.2.0",
                synced_at="2026-07-27T12:00:00+08:00",
            )

            self.assertEqual(
                json.loads(marker_path.read_text()),
                {
                    "repository": "https://github.com/openai/codex",
                    "branch": "main",
                    "commit": "a" * 40,
                    "version": "20260727.2.0",
                    "syncedAt": "2026-07-27T12:00:00+08:00",
                },
            )


if __name__ == "__main__":
    unittest.main()
