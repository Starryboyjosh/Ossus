#!/usr/bin/env python3
"""Known-answer and integrity regression tests for hash-git-resource.py."""

from __future__ import annotations

import importlib.util
import os
import subprocess
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("hash-git-resource.py")
SPEC = importlib.util.spec_from_file_location("hash_git_resource", SCRIPT)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("cannot load hash-git-resource.py")
HASHER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(HASHER)

EXPECTED = "sha256:dc2b133ce393a81fce14dee43376ae9eec2425df85878ef4259d1537918cc734"


def git(repository: Path, *arguments: str, input_bytes: bytes | None = None) -> bytes:
    result = subprocess.run(
        ["git", "-C", str(repository), *arguments],
        input=input_bytes,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
        env={key: value for key, value in os.environ.items() if not key.startswith("GIT_")},
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr.decode("utf-8", "backslashreplace"))
    return result.stdout.strip()


def blob(repository: Path, content: bytes) -> str:
    return git(repository, "hash-object", "-w", "--stdin", input_bytes=content).decode()


def tree(repository: Path, entries: list[tuple[str, str, str]]) -> str:
    body = b"".join(
        f"{mode} {kind} {object_id}\t{name}\n".encode()
        for mode, kind, object_id, name in entries
    )
    return git(repository, "mktree", input_bytes=body).decode()


def fixture_repository(root: Path) -> tuple[Path, str]:
    repository = root / "repo"
    repository.mkdir()
    git(repository, "init", "--quiet")
    git(repository, "config", "user.name", "Ossus test")
    git(repository, "config", "user.email", "test@ossus.invalid")
    skill = blob(repository, b"hello\n")
    link = blob(repository, b"SKILL.md")
    tool = blob(repository, b"#!/bin/sh\n")
    demo = tree(
        repository,
        [
            ("100644", "blob", skill, "SKILL.md"),
            ("120000", "blob", link, "link"),
            ("100755", "blob", tool, "tool.sh"),
        ],
    )
    skills = tree(repository, [("040000", "tree", demo, "demo")])
    root_tree = tree(repository, [("040000", "tree", skills, "skills")])
    environment = {
        key: value for key, value in os.environ.items() if not key.startswith("GIT_")
    }
    environment.update(
        {
            "GIT_AUTHOR_NAME": "Ossus test",
            "GIT_AUTHOR_EMAIL": "test@ossus.invalid",
            "GIT_AUTHOR_DATE": "2000-01-01T00:00:00Z",
            "GIT_COMMITTER_NAME": "Ossus test",
            "GIT_COMMITTER_EMAIL": "test@ossus.invalid",
            "GIT_COMMITTER_DATE": "2000-01-01T00:00:00Z",
        }
    )
    result = subprocess.run(
        ["git", "-C", str(repository), "commit-tree", root_tree],
        input=b"hash vector\n",
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
        env=environment,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr.decode("utf-8", "backslashreplace"))
    return repository, result.stdout.strip().decode()


class HashGitResourceTests(unittest.TestCase):
    def test_known_answer_and_integrity_controls(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repository, commit = fixture_repository(Path(directory))
            self.assertEqual(HASHER.tree_hash(repository, commit, "skills/demo"), EXPECTED)
            self.assertEqual(HASHER.tree_hash(repository, commit, "skills/demo"), EXPECTED)

            git(repository, "branch", "a" * 64, commit)
            with self.assertRaises(HASHER.HashError):
                HASHER.tree_hash(repository, "a" * 64, "skills/demo")

            replacement_demo = tree(
                repository,
                [
                    ("100644", "blob", blob(repository, b"tampered\n"), "SKILL.md"),
                    ("120000", "blob", blob(repository, b"SKILL.md"), "link"),
                    ("100755", "blob", blob(repository, b"#!/bin/sh\n"), "tool.sh"),
                ],
            )
            replacement_skills = tree(
                repository, [("040000", "tree", replacement_demo, "demo")]
            )
            replacement_root = tree(
                repository, [("040000", "tree", replacement_skills, "skills")]
            )
            replacement = git(
                repository,
                "commit-tree",
                replacement_root,
                input_bytes=b"replacement\n",
            ).decode()
            git(repository, "replace", commit, replacement)
            self.assertEqual(HASHER.tree_hash(repository, commit, "skills/demo"), EXPECTED)

            prior = os.environ.get("GIT_DIR")
            os.environ["GIT_DIR"] = str(repository / "not-the-repository")
            try:
                self.assertEqual(HASHER.tree_hash(repository, commit, "skills/demo"), EXPECTED)
            finally:
                if prior is None:
                    os.environ.pop("GIT_DIR", None)
                else:
                    os.environ["GIT_DIR"] = prior

    def test_subpath_validation(self) -> None:
        for invalid in ["/absolute", "../escape", "a//b", "a/./b", "a\\b", "a\0b"]:
            with self.subTest(invalid=invalid), self.assertRaises(HASHER.HashError):
                HASHER.validate_subpath(invalid)


if __name__ == "__main__":
    unittest.main()
