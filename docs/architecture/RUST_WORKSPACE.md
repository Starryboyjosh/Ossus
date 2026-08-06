# Rust workspace architecture

## Workspace shape

Start with a small workspace. Add crates only when domain boundaries are proven.

```text
ossus/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── crates/
│   ├── ossus-cli/
│   ├── ossus-core/
│   ├── ossus-registry/
│   ├── ossus-resolver/
│   ├── ossus-policy/
│   ├── ossus-adapter-claude/
│   └── ossus-eval/
├── specs/
├── catalog/
├── evaluations/
├── tests/
└── docs/
```

Later Researcher crates:

```text
crates/
├── ossus-researcher/
├── ossus-source-github/
├── ossus-source-reddit/
└── ossus-review-bundle/
```

Do not scaffold later crates in WAVE 01 unless an interface is already required.

## Crate responsibilities

### `ossus-core`

Pure domain types and shared errors: capability IDs, resource IDs, source locks, compatibility, runtime requirements, risk tiers, project profile, selection plan, explanation records and lockfile structures.

It performs no filesystem mutation and no network access.

### `ossus-registry`

- parse trusted canonical manifests;
- schema and semantic validation;
- build and query local index;
- manage catalog source metadata;
- verify source state and hashes;
- expose read-only search APIs.

### `ossus-resolver`

- map task text to capabilities;
- combine project signals;
- filter candidates;
- compute deterministic minimal coverage;
- calculate confidence;
- generate explanations.

### `ossus-policy`

- load policy;
- evaluate risk, source, runtime and surface;
- return structured allow, deny or confirm decisions;
- never call a model.

### `ossus-adapter-claude`

- validate Claude Code environment;
- create host-facing skill materialization;
- avoid exposing the whole content store;
- generate safe frontmatter;
- support rollback.

### `ossus-eval`

- load frozen golden cases;
- run project scanner and Resolver;
- calculate metrics;
- produce JSON and human-readable reports.

### `ossus-cli`

- command parsing;
- user-facing formatting;
- exit codes;
- orchestration of library crates;
- no domain logic that cannot be tested separately.

## Dependency direction

```text
ossus-cli ─┬─> ossus-registry
           ├─> ossus-resolver
           ├─> ossus-policy
           ├─> ossus-adapter-claude
           ├─> ossus-eval
           └─> ossus-core

ossus-eval ─────> { ossus-core, ossus-registry, ossus-resolver }
ossus-resolver ─> { ossus-core, ossus-policy, ossus-registry }
ossus-registry ─> ossus-core
ossus-policy ───> ossus-core
adapters ───────> ossus-core
```

`ossus-resolver -> ossus-policy` is deliberate and load-bearing: it is what makes "policy denial has priority over score" a structural property rather than a CLI convention.

Crates must not form cycles. This is the complete edge set; an edge not listed here is a boundary violation, not an omission.

### Added in WAVE-007 (ADR-018)

```text
ossus-cli ─> ossus-activation ─> { ossus-core, ossus-policy, ossus-registry }
ossus-adapter-claude ──(trait impl consumed by)──> ossus-activation
```

`ossus-activation` owns the host-neutral activation transaction. It never depends on an adapter crate; the adapter supplies a host materialization trait implementation that `ossus-activation` consumes. The crate is created in WAVE-007, not WAVE-001.

## Recommended dependency categories

Exact versions are selected and locked during WAVE 01 after audit.

- CLI: `clap` derive API.
- Serialization: `serde`, `serde_json`, `toml`.
- Schema: `schemars`, `jsonschema`.
- Index: `rusqlite` with a bundled SQLite build and verified FTS5.
- Errors: `thiserror`; `anyhow` only at binary boundaries.
- Logging: `tracing`, `tracing-subscriber`.
- Hashing: `sha2`.
- Paths: `directories`.
- Glob matching: `globset` or equivalent bounded matcher.
- Walking: `ignore` or a controlled walker honoring exclusions.
- Testing: `assert_cmd`, `predicates`, `insta`, `proptest`, `tempfile`.
- Packaging later: evaluate `cargo-dist` or an equivalent maintained release tool.

Do not add a YAML crate to the trusted core at all. Per ADR-006 and ADR-017, trusted taxonomy, configuration, manifests and evaluation data are TOML or JSON. YAML appears only behind a quarantine boundary for external import and host adapter output, and no such parser exists in V0.

### Lint policy in test targets

`[workspace.lints.clippy]` denies `unwrap_used` and `expect_used`, and `cargo clippy --all-targets` covers test targets. Do not relax the workspace lint to make tests compile. Instead place a narrow, commented allow at the top of the specific test module:

```rust
#[cfg(test)]
#[allow(clippy::unwrap_used, reason = "fixture loading in tests")]
mod tests { /* ... */ }
```

Any relaxation is then local, visible in review, and never touches workspace policy.

## Toolchain policy

- Rust edition 2024.
- `rust-toolchain.toml` pins a reviewed stable toolchain for releases.
- CI tests the pinned toolchain and current stable.
- MSRV is declared after WAVE 01, then changed only by ADR.
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --workspace` are required.
- `unsafe` is denied by default in first-party crates. Exceptions require a security ADR and targeted review.
- Workspace dependencies are centralized.
- `Cargo.lock` is committed.

## Performance targets

At 1,000 synthetic manifests:

- local resolve p95 under 500 ms on the reference machine;
- cold index open under 150 ms target;
- bounded project scan with configurable file and byte limits;
- no full resource-body reads during normal resolve;
- no network access.

## Cross-platform requirements

Supported release targets initially:

- Linux x86_64;
- macOS arm64 and x86_64 if CI capacity permits;
- Windows x86_64.

Path operations must use Rust path APIs, not string concatenation.

Symlink behavior must be capability-detected; adapters need copy fallback where symlinks are unavailable.
