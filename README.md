# Ossus

> A local-first Registry and deterministic Resolver for agent skills, prompt packs, and MCP servers.

[![Status: WAVE-003 in progress](https://img.shields.io/badge/status-WAVE--003%20in%20progress-5c4ee5)](#project-status)
[![Rust 2024](https://img.shields.io/badge/Rust-2024-black?logo=rust)](Cargo.toml)
[![Security model](https://img.shields.io/badge/security-threat--modeled-167a4a)](docs/security/THREAT_MODEL.md)

Ossus is a command-line system for keeping a large library of reusable agent capabilities while exposing only the smallest compatible set required by a specific project and task.

Its central rule is:

> **Stored does not mean approved. Approved does not mean installed. Installed does not mean active.**

Ossus is not another folder of prompts. It is the control layer around that folder: provenance, compatibility, risk, deterministic selection, activation boundaries, and reproducibility.

## Project status

This repository is under active implementation. **WAVE-002 is complete**, Security Gate S1 is closed, and the current implementation WAVE is:

```text
WAVE-003 — Registry and local search (in progress)
```

Implemented foundations include:

- the Rust Cargo workspace and `ossus` bootstrap binary;
- typed shared IDs, versions, enums, and stable diagnostics;
- canonical TOML manifest loading with bounded parsing;
- governed taxonomy and semantic validation;
- executable negative fixtures and trusted lockfile hash contracts;
- security, governance, CI, and the WAVE-000 through WAVE-024 implementation plan;
- original Spanish architecture documents retained as historical records.

WAVE-003 is adding the local SQLite/FTS Registry and its search, show, status, and reindex commands. Until its acceptance criteria pass, the repository does **not** claim those commands are complete. Resolver, Researcher, installation, activation, and later-WAVE behavior remain unimplemented.

## The three domains

```text
External sources
       |
       v
+----------------+       +----------------+       +----------------+
|   Researcher   | ----> |    Registry    | ----> |    Resolver    |
| discover and   |       | trusted index  |       | select minimum |
| prepare evidence|      | and provenance |       | compatible set |
+----------------+       +----------------+       +----------------+
                                                          |
                                                          v
                                                Claude Code / Codex
```

### Researcher

Discovers external candidates and prepares evidence. It never approves, activates, or writes trusted canonical metadata directly.

### Registry

Stores curator-owned canonical manifests, immutable source references, hashes, compatibility, risk, and review state. The initial design is **index-first**: resource content is fetched selectively, not copied wholesale into the repository.

### Resolver

Scans the project, maps the task to a governed capability vocabulary, filters by host/runtime/policy, and computes a deterministic minimal set. External model calls are not required by default.

## Trust path

```text
external source
  -> quarantine
  -> candidate evidence
  -> agent-approved canonical manifest
  -> installed fixed content
  -> Resolver selection
  -> active host materialization
```

The approved Registry remains an instruction channel into an agent session. Approval reduces risk under documented conditions; it is not a universal safety guarantee.

## CLI shape

The production command is:

```bash
ossus
```

Planned command groups:

```text
ossus init
ossus config
ossus registry
ossus search
ossus show
ossus scan
ossus resolve
ossus explain
ossus activate
ossus deactivate
ossus lock
ossus doctor
ossus eval
ossus audit
ossus research
```

For the currently implemented bootstrap surface, use:

```bash
cargo run -p ossus -- --help
cargo run -p ossus -- status
cargo run -p ossus -- plan
```

Commands outside the current implemented WAVE return an explicit “planned but not implemented” error rather than pretending to work.

## Repository structure

```text
ossus/
├── crates/                     Rust workspace
│   ├── ossus-cli/              `ossus` binary and presentation boundary
│   ├── ossus-core/             Shared domain contracts
│   ├── ossus-registry/         Trusted Registry domain
│   ├── ossus-resolver/         Deterministic selection domain
│   ├── ossus-policy/           Risk and policy decisions
│   ├── ossus-adapter-claude/   Claude Code materialization boundary
│   └── ossus-eval/             Golden evaluation domain
├── catalog/                    Canonical Registry layout; no candidates
├── specs/                      Taxonomy, schemas, config, and policy
├── evaluations/                Frozen goldens and seed profiles
├── docs/                       Product, architecture, security, and plan
├── examples/                   Safe examples and fixture guidance
├── scripts/                    Local verification helpers
└── .github/                    CI, issue forms, and repository governance
```

A generated full tree is available at [`REPOSITORY_TREE.txt`](REPOSITORY_TREE.txt).

## Quick start for contributors

### Requirements

- Git
- Rust stable with Rust 2024 support
- `rustfmt`
- Clippy

### Verify the repository

Linux or macOS:

```bash
./scripts/verify.sh
```

Windows PowerShell:

```powershell
./scripts/verify.ps1
```

Or directly:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

### Read before implementing

1. [`AGENTS.md`](AGENTS.md)
2. [`docs/product/OSSUS_MASTER_CONTEXT.md`](docs/product/OSSUS_MASTER_CONTEXT.md)
3. [`docs/product/DECISION_LOG.md`](docs/product/DECISION_LOG.md)
4. [`docs/roadmap/PHASES_AND_GATES.md`](docs/roadmap/PHASES_AND_GATES.md)
5. [`docs/implementation/06-waves/WAVE_INDEX.md`](docs/implementation/06-waves/WAVE_INDEX.md)
6. The current WAVE file

The two Spanish master documents are preserved under [`docs/implementation/00-original-context/`](docs/implementation/00-original-context/) for intentional restructuring and historical review. They are not the active implementation contract.

## First useful vertical slice

```text
20 agent-curated canonical manifests
        -> local Registry index
        -> bounded project scan
        -> task-to-capability mapping
        -> policy and compatibility filters
        -> deterministic minimal selection
        -> explanation
        -> Claude Code materialization
        -> skills.lock.json
```

The Researcher comes later. The Resolver is implemented and measured first.

## Security governance

Security WAVEs require separated Implementer, Independent Security Review, and Closure Agents. The Closure Agent has final technical authority after independently verifying the review and evidence; configured model names do not confer authority:

```text
WAVE-000  WAVE-007  WAVE-010  WAVE-017
WAVE-018  WAVE-020  WAVE-021  WAVE-023
```

Human review may supplement the record but is not a required approval step. Model review is evidence, not certification. See:

- [`SECURITY.md`](SECURITY.md)
- [`docs/security/THREAT_MODEL.md`](docs/security/THREAT_MODEL.md)
- [`docs/security/SECURITY_GATES.md`](docs/security/SECURITY_GATES.md)
- [`docs/security/SECURITY_GOVERNANCE.md`](docs/security/SECURITY_GOVERNANCE.md)

## Adding a resource

The future end-to-end path is documented in [`docs/ADDING_A_RESOURCE.md`](docs/ADDING_A_RESOURCE.md):

```text
discover -> quarantine -> inspect -> risk tier -> agent canonicalization
         -> Registry approval -> install -> resolve -> activate -> lock
```

Canonical capabilities, triggers, permissions, compatibility, and risk are controlled through the separated Curator, Admission Review, and Closure Agent roles. They are not imported blindly from upstream authors.

## Compatibility model

Ossus does not use a single `compatible = true` field. It models:

- resource type;
- capability coverage;
- host surface;
- runtime requirements;
- portability;
- installation scope;
- source identity;
- risk tier;
- activation mode.

A standalone CLI can be usable from Claude Code with shell access while being unsuitable for an API-only Claude host. See [`docs/HOST_COMPATIBILITY.md`](docs/HOST_COMPATIBILITY.md).

## Contributing

Read [`CONTRIBUTING.md`](CONTRIBUTING.md) and [`GOVERNANCE.md`](GOVERNANCE.md). Changes to taxonomy meaning, trust boundaries, activation, source precedence, model defaults, or the deterministic algorithm require explicit change control and usually an ADR.

[`docs/AGENT_AUTHORITY.md`](docs/AGENT_AUTHORITY.md) is the normative policy for agent-final technical acceptance, Registry admission, risk disposition, security gates, release readiness, and the required per-WAVE technical and practical summaries.

Do not weaken goldens, policies, schemas, or security thresholds merely to make an implementation pass.

## License

Ossus is dual-licensed under either:

- Apache License, Version 2.0; or
- MIT License.

You may choose either license. See [`LICENSE-APACHE`](LICENSE-APACHE) and [`LICENSE-MIT`](LICENSE-MIT).
