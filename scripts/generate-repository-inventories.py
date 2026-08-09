#!/usr/bin/env python3
"""Generate or check Ossus tracked-file inventories deterministically."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
ROOT_MANIFEST = "REPOSITORY_MANIFEST.json"
ROOT_TREE = "REPOSITORY_TREE.txt"
PACKAGE_ROOT = "docs/implementation"
PACKAGE_MANIFEST = f"{PACKAGE_ROOT}/PACKAGE_MANIFEST.json"
GENERATED_PATHS = {ROOT_MANIFEST, ROOT_TREE, PACKAGE_MANIFEST}


def tracked_paths() -> list[str]:
    result = subprocess.run(
        ["git", "ls-files", "-z"],
        cwd=ROOT,
        check=True,
        capture_output=True,
    )
    return sorted(
        path.decode("utf-8")
        for path in result.stdout.split(b"\0")
        if path
    )


def tracked_blob_bytes(path: str) -> bytes:
    """Read the canonical Git index blob, independent of checkout EOL filters.

    The repository declares platform-specific checkout filters for PowerShell
    files. Hashing working-tree bytes therefore made the generated inventory
    disagree between a Linux checkout and a Windows checkout (and between a
    clean clone and a developer tree). The index/blob representation is the
    source-of-truth byte sequence used by the tracked-file inventory.
    """
    result = subprocess.run(
        ["git", "cat-file", "blob", f":{path}"],
        cwd=ROOT,
        check=True,
        capture_output=True,
    )
    return result.stdout


def file_record(path: str, *, relative_to: str | None = None) -> dict[str, Any]:
    data = tracked_blob_bytes(path)
    display_path = path
    if relative_to is not None:
        display_path = str(Path(path).relative_to(relative_to))
    return {
        "path": display_path,
        "bytes": len(data),
        "sha256": hashlib.sha256(data).hexdigest(),
    }


def root_manifest(paths: list[str]) -> bytes:
    indexed = [path for path in paths if path not in GENERATED_PATHS]
    document = {
        "name": "ossus",
        "kind": "implementation-repository",
        "current_wave": "WAVE-003",
        "generation": {
            "source": "git tracked files",
            "self_excluded": sorted(GENERATED_PATHS),
        },
        "file_count_excluding_generated_inventories": len(indexed),
        "files": [file_record(path) for path in indexed],
    }
    return (json.dumps(document, indent=2, ensure_ascii=False) + "\n").encode()


def package_manifest(paths: list[str]) -> bytes:
    prefix = f"{PACKAGE_ROOT}/"
    indexed = [
        path
        for path in paths
        if path.startswith(prefix) and path != PACKAGE_MANIFEST
    ]
    document = {
        "package": "ossus-implementation-plan",
        "version": "1.0.0-active",
        "active_language": "English",
        "historical_language": "Spanish originals preserved",
        "generation": {
            "source": f"git tracked files under {PACKAGE_ROOT}/",
            "self_excluded": "PACKAGE_MANIFEST.json",
        },
        "file_count_excluding_manifest": len(indexed),
        "files": [file_record(path, relative_to=PACKAGE_ROOT) for path in indexed],
    }
    return (json.dumps(document, indent=2, ensure_ascii=False) + "\n").encode()


def tree_text(paths: list[str]) -> bytes:
    tree: dict[str, Any] = {}
    for path in paths:
        node = tree
        parts = Path(path).parts
        for index, part in enumerate(parts):
            if index == len(parts) - 1:
                node[part] = None
            else:
                child = node.setdefault(part, {})
                if child is None:
                    raise ValueError(f"tracked path is both file and directory: {path}")
                node = child

    lines = ["ossus/"]

    def render(node: dict[str, Any], prefix: str) -> None:
        entries = sorted(node.items(), key=lambda item: (item[1] is None, item[0]))
        for index, (name, child) in enumerate(entries):
            last = index == len(entries) - 1
            branch = "└── " if last else "├── "
            suffix = "/" if child is not None else ""
            lines.append(f"{prefix}{branch}{name}{suffix}")
            if child is not None:
                render(child, prefix + ("    " if last else "│   "))

    render(tree, "")
    return ("\n".join(lines) + "\n").encode()


def expected_inventories() -> dict[str, bytes]:
    paths = tracked_paths()
    missing = [path for path in paths if not (ROOT / path).is_file()]
    if missing:
        raise ValueError(f"tracked paths are not regular files: {', '.join(missing)}")
    return {
        ROOT_MANIFEST: root_manifest(paths),
        ROOT_TREE: tree_text(paths),
        PACKAGE_MANIFEST: package_manifest(paths),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail if a generated inventory differs instead of rewriting it",
    )
    arguments = parser.parse_args()

    stale: list[str] = []
    for path, expected in expected_inventories().items():
        destination = ROOT / path
        if arguments.check:
            if not destination.is_file() or destination.read_bytes() != expected:
                stale.append(path)
        else:
            destination.write_bytes(expected)
            print(f"generated: {path}")

    if stale:
        for path in stale:
            print(f"stale generated inventory: {path}", file=sys.stderr)
        print(
            "run 'python3 scripts/generate-repository-inventories.py' and stage the results",
            file=sys.stderr,
        )
        return 1
    if arguments.check:
        print("repository inventories: ok")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, subprocess.CalledProcessError, UnicodeDecodeError, ValueError) as error:
        print(f"inventory generation failed: {error}", file=sys.stderr)
        raise SystemExit(1) from error
