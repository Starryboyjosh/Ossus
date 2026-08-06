# WAVE-001 — Rust workspace bootstrap

**Phase:** Phase 1  
**Assigned role:** Implementation agent  
**Depends on:** WAVE-000, and on **Security Gate S0 being closed** (closed 2026-08-04)  
**Security WAVE:** no — implementation may be delegated

## Objective

Create the Ossus Git repository, Rust workspace, quality baseline and empty command skeleton.

## Starting state

A workspace scaffold already exists on disk: `Cargo.toml`, the seven crates, `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml`, `deny.toml`, `Cargo.lock` and `.github/workflows/ci.yml`. This WAVE **audits and completes** that scaffold. Do not recreate or regenerate it, and do not restructure crates that already match `RUST_WORKSPACE.md`.

## In scope

- Initialize Git and the Cargo workspace.
- Create the seven initial crates defined in `RUST_WORKSPACE.md`. Exactly seven: `ossus-activation` is created in WAVE-007 under ADR-018, not here.
- Configure Rust 2024, a pinned stable toolchain, formatting, Clippy and tests.
- Implement `ossus --version`, help and placeholder command groups.
- Add least-privilege CI with no release credentials.
- Copy active ADRs and specifications selected for the product repository.

### Repair items carried from Gate S0

These are findings dispositioned at Gate S0 and assigned to this WAVE. Each is in scope and each needs evidence in the WAVE report.

- **F-16 — the repository is not a Git repository and `cargo fmt --all -- --check` fails.** Run `git init`, set `main` as the initial branch, and make the first commit only if the human requests it. Fix the five formatting diffs (four in `crates/ossus-cli/src/main.rs`, one at `crates/ossus-resolver/src/lib.rs:21`) by running the formatter, not by relaxing `rustfmt.toml`.
- **F-13 — `rust-toolchain.toml` pins the floating channel `stable`.** Replace `channel = "stable"` with an explicit version (for example `1.90.0`) so a toolchain release cannot change what CI verified. Add a second CI job that runs the workspace against floating `stable` as an early-warning signal; that job may be advisory, but the pinned job is required. Record the chosen version and the update procedure in the WAVE report. The declared `rust-version = "1.85"` MSRV stays as it is and is separately addressed by the MSRV ADR proposal.
- **F-14 — mutable action references and an unused `deny.toml`.** Pin every `uses:` in `.github/workflows/ci.yml` to a full commit SHA with the tag in a trailing comment (`actions/checkout@<sha> # v6.0.0`). Add a `cargo deny check` job so the existing `deny.toml` is actually enforced. Add a layout-check job asserting the Gate S0 decision D5 invariants: no `*.zip`, `*.tar`, `*.tar.gz` or `*.tgz` tracked anywhere in the repository, and no `catalog/imports/` directory. CI keeps `permissions: contents: read` and receives no release credentials.
- **F-17 — exit code 69 is emitted by the placeholder commands but is not in the stable contract.** Add `69 — command not implemented` to the exit-code table in `docs/architecture/CLI_CONTRACT.md` and its mirror `docs/implementation/02-architecture/CLI_CONTRACT.md`, marked as valid only while a command group is a placeholder. A command that ships must not return 69.
- **F-19 — no root `tests/` directory.** Create `tests/` for workspace-level integration tests, starting with the CLI surface tests below.
- **F-20 — no CLI help snapshot tests.** Add `assert_cmd` and `insta` snapshot tests covering `ossus --version`, `ossus --help` and the help output of every placeholder command group, so a later WAVE cannot silently change the documented CLI surface. Snapshots are committed.
- **F-18 — `unwrap_used` and `expect_used` are workspace-level `deny`.** Where a test needs them, use a scoped allow on the test module with a stated reason, as documented in `RUST_WORKSPACE.md`. Do not relax the workspace lint to make tests compile.

## Out of scope

- Registry behavior.
- Resolver scoring.
- Network access.
- Researcher crates.
- Release publishing.

## Expected deliverables

- Compiling workspace.
- CLI command skeleton.
- Shared error/output foundations.
- CI and contributor commands.
- MSRV ADR proposal.
- Root `tests/` directory with committed CLI snapshots.
- Updated exit-code table in `CLI_CONTRACT.md` and its mirror.

## Required tests and evidence

- Cargo format, Clippy and workspace tests.
- CLI help snapshot tests.
- CI permission review.
- `git status` output showing a real repository and a clean tree.
- `cargo fmt --all -- --check` exiting zero, with the diff it previously produced quoted in the report.
- `cargo deny check` output.
- The pinned toolchain version, and CI logs from both the pinned job and the floating-`stable` job.
- Evidence that the layout check fails when a `*.zip` or a `catalog/imports/` path is introduced.

## Acceptance criteria

- Workspace builds on Linux and one developer platform.
- No domain logic lives in CLI placeholders.
- First-party crates deny `unsafe`.
- Cargo.lock is committed.
- The workspace contains exactly seven crates.
- Every CI `uses:` reference is a commit SHA.
- No formatting, Clippy or `cargo deny` failure remains.
- Every stable exit code the binary can emit appears in `CLI_CONTRACT.md`.
- No canonical document under `docs/product/`, `docs/architecture/`, `docs/security/`, `specs/`, `evaluations/` or `docs/roadmap/` differs from its `docs/implementation/` mirror.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use `07-prompts/GENERAL_IMPLEMENTER.md`. Create the repository only in this WAVE and do not implement future behavior.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
