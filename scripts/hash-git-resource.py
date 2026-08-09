#!/usr/bin/env python3
"""Compute an Ossus Git resource tree hash without checking out or executing it."""

from __future__ import annotations

import argparse
import hashlib
import os
import re
import subprocess
import sys
import tempfile
import threading
import time
from pathlib import Path, PurePosixPath

MAX_FILE_BYTES = 64 * 1024 * 1024
MAX_RESOURCE_BYTES = 512 * 1024 * 1024
MAX_LISTING_BYTES = 32 * 1024 * 1024
MAX_RESOURCE_FILES = 100_000
MAX_HASH_SECONDS = 300
COMMIT_RE = re.compile(r"(?:[0-9a-f]{40}|[0-9a-f]{64})\Z")
ALLOWED_MODES = {b"100644", b"100755", b"120000"}


class HashError(Exception):
    """A bounded, user-facing hashing failure."""


def git(
    repository: Path, arguments: list[str], max_output: int, deadline: float
) -> bytes:
    environment = {
        key: value for key, value in os.environ.items() if not key.startswith("GIT_")
    }
    environment["GIT_CONFIG_NOSYSTEM"] = "1"
    environment["GIT_NO_REPLACE_OBJECTS"] = "1"
    with tempfile.TemporaryFile() as stderr:
        process = subprocess.Popen(
            [
                "git",
                "--no-replace-objects",
                "--literal-pathspecs",
                "-C",
                str(repository.resolve()),
                *arguments,
            ],
            stdout=subprocess.PIPE,
            stderr=stderr,
            env=environment,
        )
        assert process.stdout is not None
        captured: list[bytes] = []

        def read_stdout() -> None:
            output = process.stdout.read(max_output + 1)
            captured.append(output)
            if len(output) > max_output:
                process.kill()

        reader = threading.Thread(target=read_stdout, daemon=True)
        reader.start()
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            process.kill()
            process.wait()
            process.stdout.close()
            raise HashError("resource hashing exceeded the 300 second deadline")
        try:
            return_code = process.wait(timeout=remaining)
        except subprocess.TimeoutExpired as error:
            process.kill()
            process.wait()
            process.stdout.close()
            raise HashError("resource hashing exceeded the 300 second deadline") from error
        reader.join(timeout=5)
        if reader.is_alive() or not captured:
            process.kill()
            process.stdout.close()
            raise HashError("unable to read bounded Git output")
        output = captured[0]
        process.stdout.close()
        if len(output) > max_output:
            raise HashError("Git output exceeds the hashing budget")
        stderr.seek(0)
        error_output = stderr.read(8192)
    if return_code != 0:
        detail = error_output.decode("utf-8", "backslashreplace").strip()
        raise HashError(detail or f"git {' '.join(arguments)} failed")
    return output


def validate_subpath(value: str) -> str:
    if value in {"", "."}:
        return "."
    if "\0" in value:
        raise HashError("subpath must not contain NUL")
    if any(component in {"", ".", ".."} for component in value.split("/")):
        raise HashError("subpath must use canonical relative POSIX components")
    path = PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts or "\\" in value:
        raise HashError("subpath must be a relative Git path without '..' or backslashes")
    normalized = str(path)
    if normalized in {"", "."}:
        return "."
    return normalized


def tree_hash(repository: Path, commit: str, subpath: str) -> str:
    deadline = time.monotonic() + MAX_HASH_SECONDS
    subpath = validate_subpath(subpath)
    if not COMMIT_RE.fullmatch(commit):
        raise HashError("commit must be 40 or 64 lowercase hexadecimal characters")
    if not repository.is_dir():
        raise HashError("repository path is not a directory")

    object_format = git(
        repository, ["rev-parse", "--show-object-format"], 32, deadline
    ).strip()
    expected_length = {b"sha1": 40, b"sha256": 64}.get(object_format)
    if expected_length is None or len(commit) != expected_length:
        raise HashError("commit length does not match the repository object format")
    resolved_commit = git(
        repository,
        ["rev-parse", "--verify", "--end-of-options", f"{commit}^{{commit}}"],
        128,
        deadline,
    ).strip()
    if resolved_commit != commit.encode("ascii"):
        raise HashError("commit must be the full object ID, not a ref name")
    arguments = ["ls-tree", "-r", "-z", "--full-tree", commit]
    if subpath != ".":
        arguments.extend(["--", subpath])
    listing = git(repository, arguments, MAX_LISTING_BYTES, deadline)
    records = [record for record in listing.split(b"\0") if record]
    if not records:
        raise HashError("selected resource tree contains no files")
    if len(records) > MAX_RESOURCE_FILES:
        raise HashError("resource contains more than 100,000 files")

    digest = hashlib.sha256()
    digest.update(b"ossus-git-tree-v1\0")
    total_bytes = 0
    previous_path: bytes | None = None

    for record in records:
        metadata, separator, path = record.partition(b"\t")
        fields = metadata.split(b" ")
        if not separator or len(fields) != 3:
            raise HashError("unexpected git ls-tree record")
        mode, object_type, object_id = fields
        if object_type != b"blob" or mode not in ALLOWED_MODES:
            raise HashError(
                "resource contains an unsupported Git entry (submodules are not hashable)"
            )
        if previous_path is not None and path <= previous_path:
            raise HashError("Git tree paths are not in strict byte order")
        previous_path = path

        size_text = git(
            repository,
            ["cat-file", "-s", object_id.decode("ascii")],
            1024,
            deadline,
        ).strip()
        try:
            size = int(size_text)
        except ValueError as error:
            raise HashError("invalid Git blob size") from error
        if size > MAX_FILE_BYTES:
            raise HashError("resource contains a file larger than 64 MiB")
        total_bytes += size
        if total_bytes > MAX_RESOURCE_BYTES:
            raise HashError("resource content exceeds the 512 MiB hashing budget")

        content = git(
            repository,
            ["cat-file", "blob", object_id.decode("ascii")],
            MAX_FILE_BYTES,
            deadline,
        )
        if len(content) != size:
            raise HashError("Git blob size changed while hashing")

        digest.update(b"entry\0")
        digest.update(mode)
        digest.update(b"\0")
        digest.update(str(len(path)).encode("ascii"))
        digest.update(b"\0")
        digest.update(path)
        digest.update(b"\0")
        digest.update(str(size).encode("ascii"))
        digest.update(b"\0")
        digest.update(content)
        digest.update(b"\0")

    return f"sha256:{digest.hexdigest()}"


def main() -> int:
    parser = argparse.ArgumentParser(
        description="compute the ossus-git-tree-v1 hash of an immutable Git subpath"
    )
    parser.add_argument("repository", type=Path)
    parser.add_argument("commit")
    parser.add_argument("subpath", nargs="?", default=".")
    arguments = parser.parse_args()
    try:
        print(
            tree_hash(
                arguments.repository,
                arguments.commit,
                validate_subpath(arguments.subpath),
            )
        )
    except (HashError, OSError) as error:
        print(f"hash-git-resource: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
