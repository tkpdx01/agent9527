#!/usr/bin/env python3
"""Finalize an Agent9527 upstream sync with a release version and marker."""

import argparse
from datetime import date, datetime
import json
from pathlib import Path
import re
import subprocess
from zoneinfo import ZoneInfo


REPO_ROOT = Path(__file__).resolve().parent.parent
VERSION_PATH = REPO_ROOT / "VERSION"
UPSTREAM_MARKER_PATH = REPO_ROOT / ".github" / "upstream.json"
VERSION_PATTERN = re.compile(r"^(?P<day>\d{8})\.(?P<sequence>\d+)\.0$")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--upstream-sha",
        required=True,
        help="Full OpenAI Codex upstream commit SHA included by this sync.",
    )
    parser.add_argument(
        "--date",
        dest="release_date",
        type=date.fromisoformat,
        help="Release date override in YYYY-MM-DD format (defaults to Asia/Shanghai today).",
    )
    parser.add_argument(
        "--synced-at",
        help="ISO-8601 timestamp override for deterministic tests.",
    )
    parser.add_argument(
        "--github-output",
        type=Path,
        help="Optional GitHub Actions output file that receives version=<new version>.",
    )
    return parser.parse_args()


def next_release_version(current_version: str, release_date: date) -> str:
    match = VERSION_PATTERN.fullmatch(current_version)
    if match is None:
        raise ValueError(
            f"Agent9527 version must match YYYYMMDD.N.0, got {current_version!r}."
        )

    release_day = release_date.strftime("%Y%m%d")
    sequence = (
        int(match.group("sequence")) + 1 if match.group("day") == release_day else 1
    )
    return f"{release_day}.{sequence}.0"


def tracked_files(repo_root: Path) -> list[Path]:
    output = subprocess.check_output(
        ["git", "ls-files", "-z"], cwd=repo_root, text=False
    )
    return [
        repo_root / path.decode() for path in output.rstrip(b"\0").split(b"\0") if path
    ]


def replace_tracked_version(
    repo_root: Path, old_version: str, new_version: str
) -> list[Path]:
    old_bytes = old_version.encode()
    new_bytes = new_version.encode()
    changed_paths = []

    for path in tracked_files(repo_root):
        if path.is_symlink() or not path.is_file():
            continue
        data = path.read_bytes()
        if b"\0" in data or old_bytes not in data:
            continue
        path.write_bytes(data.replace(old_bytes, new_bytes))
        changed_paths.append(path)

    return changed_paths


def write_upstream_marker(
    marker_path: Path,
    *,
    upstream_sha: str,
    version: str,
    synced_at: str,
) -> None:
    marker = {
        "repository": "https://github.com/openai/codex",
        "branch": "main",
        "commit": upstream_sha,
        "version": version,
        "syncedAt": synced_at,
    }
    marker_path.parent.mkdir(parents=True, exist_ok=True)
    marker_path.write_text(json.dumps(marker, indent=2) + "\n", encoding="utf-8")


def verify_version(repo_root: Path, version: str) -> None:
    required_files = [
        repo_root / "VERSION",
        repo_root / "package.json",
        repo_root / "agent9527-cli" / "package.json",
        repo_root / "agent9527-rs" / "Cargo.toml",
        repo_root / "agent9527-rs" / "Cargo.lock",
    ]
    missing = [
        str(path.relative_to(repo_root))
        for path in required_files
        if not path.is_file()
    ]
    if missing:
        raise RuntimeError(f"Required version files are missing: {', '.join(missing)}")

    mismatched = [
        str(path.relative_to(repo_root))
        for path in required_files
        if version.encode() not in path.read_bytes()
    ]
    if mismatched:
        raise RuntimeError(
            f"New version {version} was not written to: {', '.join(mismatched)}"
        )


def main() -> int:
    args = parse_args()
    current_version = VERSION_PATH.read_text(encoding="utf-8").strip()
    release_date = args.release_date or datetime.now(ZoneInfo("Asia/Shanghai")).date()
    new_version = next_release_version(current_version, release_date)

    changed_paths = replace_tracked_version(REPO_ROOT, current_version, new_version)
    if VERSION_PATH not in changed_paths:
        raise RuntimeError(
            f"Current version {current_version} was not found in VERSION."
        )

    synced_at = args.synced_at or datetime.now(ZoneInfo("Asia/Shanghai")).isoformat()
    write_upstream_marker(
        UPSTREAM_MARKER_PATH,
        upstream_sha=args.upstream_sha,
        version=new_version,
        synced_at=synced_at,
    )
    verify_version(REPO_ROOT, new_version)

    if args.github_output is not None:
        with args.github_output.open("a", encoding="utf-8") as output:
            output.write(f"version={new_version}\n")

    print(new_version)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
