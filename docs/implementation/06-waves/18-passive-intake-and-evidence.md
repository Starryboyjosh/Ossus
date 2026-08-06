# WAVE-018 — Passive quarantine intake and evidence

**Phase:** Phase 5  
**Assigned role:** Opus 5 security owner; optional Luna Max implementation support  
**Depends on:** WAVE-017  
**Security WAVE:** yes

## Objective

Implement passive source intake, inventory, hashing and evidence bundles without execution.

## In scope

- Manual URL/local path intake.
- Immutable Git commit resolution.
- Quarantine filesystem.
- Bounded inventory.
- Safe archive behavior if included.
- Symlink/submodule policy.
- Evidence bundle generation.

## Out of scope

- GitHub search.
- Reddit.
- Scripts/tests.
- Canonical manifest writing.

## Expected deliverables

- Experimental ingest command.
- Quarantine/evidence APIs.
- Attack tests.
- Gate S5 implementation closure.

## Required tests and evidence

- Git hooks.
- Submodules.
- Symlink escape.
- Archive traversal.
- Large file/count.
- Unicode/reserved names.
- Interrupted cleanup.

## Acceptance criteria

- No candidate executes.
- No secrets are available.
- Evidence is untrusted by type.
- Opus 5 security assessment and human closure complete.


## Mandatory security workflow

This is a security WAVE.

- Opus 5 owns the assigned security work and reviews the final implementation or audit evidence.
- Luna Max or another implementation agent may assist only through attributed, bounded tasks.
- The human closes the gate.
- There is no automatic replacement if Opus 5 is unavailable.
- Model review is evidence, not certification.


## Copy-ready implementation instruction

Use security prompts. Exclude archives if safe extraction cannot be proven in scope.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
