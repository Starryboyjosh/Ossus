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
- Independent Security Review Agent assessment and distinct Closure Agent decision complete.


## Mandatory security workflow

This is a security WAVE.

- A capable independent Security Review Agent reviews the final implementation and evidence.
- Implementation agents may assist only through attributed, bounded tasks.
- A distinct Closure Agent closes the gate and has the final word.
- Human review is optional additional evidence.


## Copy-ready implementation instruction

Use security prompts. Exclude archives if safe extraction cannot be proven in scope.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
