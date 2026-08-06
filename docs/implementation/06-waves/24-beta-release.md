# WAVE-024 — Ossus beta release

**Phase:** Phase 6  
**Assigned role:** Release owner with architecture/security approval  
**Depends on:** WAVE-023  
**Security WAVE:** no

## Objective

Release Registry, Resolver and experimental Researcher beta with explicit limitations.

## In scope

- Package release artifacts.
- Publish incident/revocation procedure.
- Label Researcher experimental.
- Complete docs/examples.
- Document V0 migration.
- Define public Registry process.

## Out of scope

- Claims of guaranteed safety.
- R5 stable resources.
- Automatic approval.

## Expected deliverables

- Beta artifacts.
- Registry snapshot.
- Checksums/provenance.
- Security/limitations document.
- Maintainer runbook.

## Required tests and evidence

- Clean install.
- V0 upgrade.
- Resolver goldens.
- Researcher attack suite.
- Revocation drill.
- Artifact verification.

## Acceptance criteria

- All gates close.
- Release checklist passes.
- Residual risks publish.
- Human authorizes publication.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt and release checklist. Never publish automatically.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
