# Getting started

Ossus is currently implementing WAVE-003. It can validate canonical resource
manifests and build a disposable local search index from approved manifest
metadata. It does not yet install, resolve, activate, synchronize, or discover
resources.

## Verify the repository

```bash
./scripts/verify.sh
cargo run -p ossus -- status
```

Read `AGENTS.md` and `docs/implementation/CURRENT_WAVE.md` before assigning
repository work to an agent. WAVE-004 and later functionality remains out of
scope while WAVE-003 is open.

## Validate manifests

```bash
cargo run -p ossus -- validate catalog/examples/canonical-manifest.example.toml
cargo run -p ossus -- --format json validate catalog/examples/canonical-manifest.example.toml
```

Validation checks the bounded canonical schema and governed taxonomy. A valid
file is not automatically approved or indexed.

## Build and inspect a local Registry index

```bash
cargo run -p ossus -- registry reindex \
  --manifest-root catalog/official/manifests \
  --index .ossus/registry.sqlite3

cargo run -p ossus -- registry status --index .ossus/registry.sqlite3
```

Reindexing reads canonical metadata, excludes invalid manifests, detects
conflicts, and replaces the disposable SQLite index deterministically. It does
not approve, download, install, resolve, or activate resource bodies. A missing,
corrupt, or incompatible index is reported as requiring reindexing.

## Search and show metadata

```bash
cargo run -p ossus -- search "responsive design" --index .ossus/registry.sqlite3
cargo run -p ossus -- search \
  --capability frontend.accessibility \
  --surface codex-cli \
  --risk-max R1 \
  --index .ossus/registry.sqlite3
cargo run -p ossus -- show <resource-id> --index .ossus/registry.sqlite3
```

Add `--format json` anywhere in an invocation for versioned machine-readable
output. Search operates only on approved canonical metadata stored in the local
index; it never reads upstream resource bodies.

The official WAVE-003 seed catalog remains unavailable until every real seed
has completed separated Curator Agent, Admission Review Agent, and Closure Agent
admission. Planned commands such as `scan`, `resolve`, `activate`, `sync`, and
Researcher discovery are intentionally not part of this workflow.
