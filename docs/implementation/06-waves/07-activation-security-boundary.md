# WAVE-007 — Activation security boundary

**Phase:** Phase 3  
**Assigned role:** Opus 5 security owner; optional Luna Max implementation support  
**Depends on:** WAVE-006, and on **Security Gate S2 being closed by a named human** (see `06-waves/05-resolver-core.md`)  
**Security WAVE:** yes  
**Closes:** Security Gate S3 — Activation boundary

## Objective

Implement integrity verification, transactions, ownership and safe materialization primitives.

## Design precondition

`docs/security/ACTIVATION_SECURITY.md`, section **Concurrency, atomicity and recovery**, is a precondition of this WAVE (finding F-07). It must be satisfied by the design before implementation begins, not discovered during it: the transaction unit is the whole active set, the multi-rename is journalled under `$OSSUS_HOME/transactions/<txn-id>/` and replays backwards, locks are held on the project `.ossus/` directory and on `$OSSUS_HOME/transactions`, a stale lock is never broken by deletion alone, and hash verification runs against the staged bytes rather than the store copy read earlier.

## In scope

- Create the `ossus-activation` crate defined by **ADR-018**. It owns the activation transaction. The dependency direction is `ossus-cli -> ossus-activation -> { ossus-core, ossus-policy, ossus-registry }`; `ossus-adapter-claude` supplies a trait implementation consumed by `ossus-activation` and never owns the transaction. This is the eighth crate; WAVE-001 correctly created seven.
- Create the content-addressed store interface.
- Verify hashes immediately before activation.
- Validate paths and symlinks.
- Implement staging and atomic/recoverable swaps.
- Record ownership.
- Implement managed deactivation and rollback.
- Emit structured audit events.

## Out of scope

- Claude-specific formatting.
- Remote sync.
- Candidate execution.

## Expected deliverables

- Host-neutral activation API.
- Attack tests.
- Threat-model delta.
- Opus 5 security report and implementation-support attribution.
- Human Gate S3 closure.

## Required tests and evidence

- Path traversal and absolute paths.
- External symlinks.
- Hash mismatch.
- TOCTOU scenarios where practical.
- Crash rollback.
- Modified managed paths.
- Unmanaged preservation.
- Concurrent activation lock.
- The four evidence items required by the **Concurrency, atomicity and recovery** section of `ACTIVATION_SECURITY.md` (F-07): crash injection between per-resource renames; two concurrent `ossus activate` runs in one project; `registry sync` during an in-flight resolve; stale-journal replay run twice for idempotence.

## Acceptance criteria

- No path escapes roots.
- Prior state survives failure.
- Deactivation never silently removes modified or unmanaged files.
- A partially-applied active set is never observable after a failed activation.
- Gate S3 closes.


## Mandatory security workflow

This is a security WAVE.

- Opus 5 owns the assigned security work and reviews the final implementation or audit evidence.
- Luna Max or another implementation agent may assist only through attributed, bounded tasks.
- The human closes the gate.
- There is no automatic replacement if Opus 5 is unavailable.
- Model review is evidence, not certification.


## Copy-ready implementation instruction

Use the Opus 5 security implementation and final analysis prompts, findings disposition, and human closure checklist.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
