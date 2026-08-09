# Canonical Git resource hash

`source.tree_hash` for a Git-backed resource uses the
`ossus-git-tree-v1` byte framing below. The reference implementation is
`scripts/hash-git-resource.py`.

## Inputs

- A repository using Git SHA-1 or SHA-256 object format.
- A lowercase full commit object ID of exactly the repository's object-ID
  length. Ref names and replacement objects are not accepted.
- An optional canonical repository-root-relative POSIX subpath. Absence means
  the repository root. Empty strings, `.`, `..`, empty components, absolute
  paths, backslashes, and embedded NUL (`0x00`) bytes are not canonical
  manifest values. NUL is forbidden because it is a framing delimiter and
  cannot occur in a Git path.

The subpath selects entries but is not stripped: every framed path remains the
full repository-root-relative raw Git path. Entries are ordered by strict
lexicographic comparison of those raw path bytes.

## Wire framing

All quoted literals are ASCII bytes. `NUL` is one `0x00` byte. Decimal lengths
have no sign or leading zeroes, except the number zero itself.

```text
"ossus-git-tree-v1" NUL

for each selected blob:
  "entry" NUL
  git-mode-ascii NUL
  decimal-path-byte-length NUL
  raw-repository-relative-path-bytes NUL
  decimal-blob-byte-length NUL
  raw-blob-bytes NUL
```

Permitted modes are `100644`, `100755`, and `120000`. A `120000` symbolic link
is hashed as its raw target blob and is never followed. Gitlinks/submodules and
all other entry types are rejected. A hashable symlink is not automatically
safe to materialize; activation must still reject external or escaping links.

The SHA-256 digest of the entire framing is serialized as lowercase
`sha256:<64 hex>`. The prefix identifies the digest algorithm; the manifest and
ADR-021 identify the `ossus-git-tree-v1` framing version.

## Bounds and environment

The reference implementation hashes without checkout or candidate execution.
It removes inherited `GIT_*` routing/configuration, disables replacement
objects and pathspec magic, and enforces: 32 MiB maximum listing, 100,000 files,
64 MiB per blob, 512 MiB aggregate blob bytes, and a 300-second global deadline.

## Known-answer vector

The selected `skills/demo` tree contains these repository-root-relative blobs:

| Mode | Path | Blob bytes |
|---|---|---|
| `100644` | `skills/demo/SKILL.md` | `hello\n` |
| `120000` | `skills/demo/link` | `SKILL.md` |
| `100755` | `skills/demo/tool.sh` | `#!/bin/sh\n` |

Expected result:

```text
sha256:dc2b133ce393a81fce14dee43376ae9eec2425df85878ef4259d1537918cc734
```

`scripts/test-hash-git-resource.py` constructs this tree directly as Git
objects and verifies the vector plus ref-name, replacement-object, environment,
path, mode, and content protections.
