# WAVE-011 — Ossus V0 release candidate

**Phase:** Phase 3  
**Assigned role:** Implementation agent and human release owner  
**Depends on:** WAVE-010  
**Security WAVE:** no

## Objective

Package and document the resolver-first release candidate.

## In scope

- Version and changelog.
- Clean-environment installation test.
- Approved target binaries.
- Seed Registry snapshot.
- Upgrade and rollback notes.
- Known limitations and security posture.

## Out of scope

- Researcher.
- Remote sync.
- Stable 1.0 guarantees.

## Expected deliverables

- V0 release candidate.
- Checksums.
- Seed snapshot.
- Release report.

## Required tests and evidence

- Install from artifact.
- CLI smoke.
- Golden suite.
- Activation fixture.
- Checksum verification.

## Acceptance criteria

- Phase 3 gates pass.
- Release checklist completes.
- Limitations explicitly state no sandbox and no Researcher.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt and release checklist. Do not publish without explicit human authorization.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
