# Architecture decision log

## ADR-001 — Project name: Ossus

**Status:** accepted

The product and CLI are named `Ossus`, inspired by Star Wars. Internal domains retain the names Researcher, Registry and Resolver.

## ADR-002 — Rust implementation

**Status:** accepted

Use a Rust 2024 Cargo workspace and a single primary binary named `ossus`.

The trusted core must not depend on a scripting runtime.

## ADR-003 — Resolver before Researcher

**Status:** accepted

The Resolver, Registry contracts and evaluation suite are built before Internet discovery.

The Researcher begins only after the Resolver is measured and useful.

## ADR-004 — Index-first Registry

**Status:** accepted

The Registry stores canonical metadata, immutable source references and hashes.

Content is fetched during installation. Vendoring is exceptional.

## ADR-005 — Canonical metadata is curator-owned

**Status:** accepted

External manifests never become trusted canonical manifests automatically.

## ADR-006 — TOML and JSON trusted formats

**Status:** accepted

- TOML: trusted human-authored configuration and canonical manifests.
- JSON: generated indexes, schemas, machine reports and lockfiles.
- YAML: external import and host adapter output only.

All parsing is bounded by size, depth, item count and string length.

## ADR-007 — SQLite local index

**Status:** accepted with validation spike

Use SQLite through Rust bindings for local catalog state and full-text search.

The implementation WAVE must prove FTS5 support in supported release builds. If that proof fails, stop and issue an ADR rather than silently replacing the index.

## ADR-008 — Deterministic Resolver

**Status:** accepted

No model is required for normal resolution.

The algorithm uses exact signals, aliases, full-text retrieval, policy filters and deterministic weighted set cover.

## ADR-009 — One adapter in V0

**Status:** accepted

Claude Code is the only fully implemented adapter in V0.

Codex is modeled in schemas and goldens, then implemented in a later phase.

## ADR-010 — No custom sandbox

**Status:** accepted

Ossus does not claim to provide process isolation.

When execution becomes necessary, use disposable existing runners or VMs with no secrets and explicit network policy.

## ADR-011 — No custom static-analysis engine

**Status:** accepted

Integrate mature tools such as Semgrep and ecosystem dependency scanners.

Ossus owns policy and evidence normalization, not language-analysis engines.

## ADR-012 — Candidate staging is separate

**Status:** accepted

External candidates never enter a privileged branch of the main Registry repository.

Use a separate staging repository or fork with no secrets and no privileged CI.

## ADR-013 — Risk-scaled human review

**Status:** accepted

Every approved entry has human authority, but required reviewers and tests scale from R0 to R4.

R5 is excluded from stable V0.

## ADR-014 — Security model assignments

**Status:** accepted for this implementation project

Opus 5 owns every security WAVE and reviews its final diff and evidence. Luna Max or another implementation agent may assist with attributed, bounded implementation and test work. A human closes every security gate.

**Revision, 2026-08-03:** The project owner removed the former mandatory security model because it requires an unavailable API plan. The owner accepted Opus 5 as the sole required security model. Loss of independent cross-model review remains an explicit residual risk and does not remove human gate closure.

Model names remain configuration, not permanent public protocol.

## ADR-015 — Host-native discovery is reused

**Status:** accepted

Ossus does not rebuild native skill loading.

Adapters materialize the resolved set into host-supported locations and controls.

## ADR-016 — Policy monotonicity for project-scoped configuration

**Status:** accepted at Security Gate S0, 2026-08-04

Project-scoped configuration may only restrict policy. It may never relax it.

A project directory is attacker-supplied the moment a repository is cloned, so `.ossus/config.toml` and `.ossus/policy.toml` are untrusted input, not user intent.

Security-relevant keys resolve by taking the strictest value across scopes:

- risk tiers and numeric limits resolve to the minimum;
- allowlists resolve to the intersection;
- denylists resolve to the union.

The following are **user/global scope only**. If present in a project-scoped file they are ignored, and the attempted relaxation is reported in `explain` output and recorded as an audit event:

- `require_hash_verification`
- `allow_modified_local`
- `allow_implicit_r4`
- `semantic_fallback`
- any upward change to a parser or traversal budget
- registry source registration

Registry priority does not confer policy authority. A project-scoped registry entry may lose a name contest to a lower-priority official entry when policy requires it, and shadowing must be attributed in output.

Rationale: without this rule a cloned repository silently obtains R4 implicit activation, R5 admission and disabled hash verification, with no user interaction beyond entering the directory. This was the single critical finding (F-01) of the WAVE-000 plan review.

## ADR-017 — Trusted format for taxonomy and evaluation data

**Status:** accepted at Security Gate S0, 2026-08-04

The trusted taxonomy and the frozen evaluation data are trusted human-authored inputs, so ADR-006 applies to them unchanged. They are converted out of YAML:

| File | Target format |
|---|---|
| `specs/taxonomy/capabilities-v1.yaml` | TOML |
| `specs/taxonomy/aliases-v1.yaml` | TOML |
| `specs/taxonomy/deprecations-v1.yaml` | TOML |
| `evaluations/goldens/goldens-v1.yaml` | TOML |
| `evaluations/seed-catalog-profiles.yaml` | TOML |
| `specs/config/model-roles.yaml` | TOML |

ADR-006 is **not** amended. No YAML parser enters the trusted computing base. YAML remains restricted to external import and host adapter output, where it is parsed only behind a quarantine boundary.

The conversion is owned by WAVE-002 and must preserve content exactly: 44 capability IDs and 50 golden cases, verified by count and by round-trip comparison against the YAML originals before the originals are removed.

Cost accepted: the 50-case golden file is less pleasant to hand-edit in TOML. This was judged smaller than the cost of admitting a parser class with a poor security history into the TCB, given that no code reads these files yet.

## ADR-018 — Activation transaction crate boundary

**Status:** accepted at Security Gate S0, 2026-08-04

An eighth crate, `ossus-activation`, owns the host-neutral activation transaction: the content-addressed store, hash verification, path and symlink validation, staging, atomic swap, ownership records, rollback and audit events.

Dependency direction:

```text
ossus-cli -> ossus-activation -> { ossus-core, ossus-policy, ossus-registry }
```

`ossus-adapter-claude` supplies a host materialization trait implementation that `ossus-activation` **consumes**. The adapter never owns the transaction, and `ossus-activation` never depends on an adapter crate.

The crate is created in **WAVE-007**, not WAVE-001. WAVE-001 scope is unchanged and still creates exactly seven crates.

Rationale: WAVE-007 requires a "host-neutral activation API" that no crate owned. `ossus-core` performs no filesystem mutation, `ossus-cli` may not hold domain logic, and the adapter is the Claude-specific boundary. Naming the owner before WAVE-007 begins prevents the product's most security-critical code from landing in whichever crate is convenient under time pressure.

## ADR-019 — MSRV and reviewed toolchain policy

**Status:** proposed

`rust-version` in the workspace manifest declares the minimum supported Rust version (MSRV) for published Ossus crates. It is chosen from the lowest stable toolchain that supports the current Rust edition, required language and library features, and the documented supported build targets. Raising the MSRV requires an ADR proposal, compatibility evidence from the supported targets, and human acceptance before the manifest changes.

The release and required CI toolchain is a separately reviewed, explicitly pinned stable version in `rust-toolchain.toml`. The pinned toolchain may be newer than the MSRV and provides reproducible formatting, lint and test evidence. CI also runs the same quality checks against floating `stable` as an advisory early-warning job; that job does not change the supported MSRV or release toolchain.

When the pinned toolchain is updated, the implementer must record the exact version, rerun the required format, Clippy, test and dependency checks, inspect the lockfile and review compiler or dependency changes. A pin update does not by itself raise the MSRV. The MSRV may be raised only through a later accepted ADR and a deliberate release-policy change.
