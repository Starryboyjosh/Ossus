# Phases and gates

Ossus is implemented as a sequence of measurable vertical increments. A later phase may not consume a security control that has not passed its gate.

## Phase 0 — Plan and security baseline

**WAVE:** 000

Outputs:

- Opus 5 adversarial security and architecture plan review;
- attributed implementation-agent evidence where used;
- explicit finding dispositions;
- human architecture and security decision.

Gate S0 — owner WAVE-000, **closed 2026-08-04**:

- no unresolved critical architecture finding;
- high findings resolved or represented by a blocking WAVE;
- threat model, trust boundaries, assignments and ordering agree.

Closure record: `docs/implementation/08-operations/WAVE-000_GATE_S0_CLOSURE.md`. The critical finding F-01 is resolved by ADR-016 and proven at Gate S2; the high findings F-02, F-03, F-04 and F-05 are represented by ADR-017, WAVE-002, ADR-018/WAVE-007 and the gate ownership recorded below.

## Phase 1 — Contracts and Rust foundation

**WAVEs:** 001–002

Outputs:

- Rust 2024 Cargo workspace;
- stable CLI skeleton;
- canonical manifest and lock schemas;
- capability taxonomy V1;
- parser budgets and version policy.

Gate S1 — owner **WAVE-002**, closed by a named human before WAVE-003 begins:

- trusted formats validate deterministically;
- unknown major versions fail closed;
- canonical and origin metadata cannot be confused;
- all 44 capabilities load and validate.

## Phase 2 — Resolver MVP

**WAVEs:** 003–006

Outputs:

- 20 human-curated seed Registry entries;
- SQLite search index;
- bounded project scanner;
- deterministic Resolver;
- 50 capability goldens and exact-resource expectations;
- quality and performance reports.

Gate S2 — owner **WAVE-005**, closed by a named human before WAVE-007 begins. The criteria are in `docs/security/SECURITY_GATES.md`; policy monotonicity (ADR-016) is proven here, not in Phase 3.

Gate R0:

- capability micro-F1 at least 0.90;
- required-resource recall at least 0.90;
- zero constraint violations;
- zero implicit R4 selection;
- local resolve p95 below 500 ms at 1,000 manifests;
- at least 80% metadata/context reduction versus loading the full catalog;
- zero external model calls by default.

## Phase 3 — Safe vertical slice

**WAVEs:** 007–011

Outputs:

- host-neutral activation transaction;
- integrity, ownership and rollback controls;
- Claude Code adapter;
- complete CLI workflow;
- dependency and CI hardening;
- V0 release candidate.

Gates S3–S4 (S2 closes in Phase 2 and is a precondition of WAVE-007):

- path traversal and symlink escape tests pass;
- failed activation preserves prior state;
- unmanaged files are preserved;
- dependency, license and CI reviews close;
- release artifacts are verifiable.

## Phase 4 — Distribution and multi-source use

**WAVEs:** 012–016

Outputs:

- safe Registry synchronization;
- selective immutable installation;
- private and project catalogs;
- Codex adapter;
- optional local semantic fallback;
- cross-platform distribution.

Gate D0:

- synchronization cannot mutate active content;
- every override is visible and attributable;
- Codex activation uses native current paths and controls;
- optional semantics remain local, opt-in and non-authoritative;
- supported release targets pass clean-install smoke tests.

## Phase 5 — Researcher

**WAVEs:** 017–023

Outputs:

- formal Researcher security design;
- passive quarantine intake;
- GitHub discovery;
- external analysis integrations;
- human admission workflow;
- Reddit community evidence;
- end-to-end Researcher audit.

Gates S5–S7:

- no candidate content executes by default;
- Researcher has no trusted Registry write path;
- candidate CI has no secrets or write token;
- immutable source and evidence are required;
- canonical fields require human authority;
- community evidence cannot approve or reduce risk;
- no unresolved critical or high audit finding.

## Phase 6 — Public beta

**WAVE:** 024

Outputs:

- beta artifacts;
- public Registry process;
- incident and revocation runbook;
- upgrade path;
- published limitations and residual risks.

Gate B0:

- every preceding gate is closed;
- release drill and revocation drill pass;
- human release owner authorizes publication.
