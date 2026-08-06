# WAVE-019 — GitHub discovery connector

**Phase:** Phase 5  
**Assigned role:** Implementation agent with mandatory Opus 5 review  
**Depends on:** WAVE-018  
**Security WAVE:** no

## Objective

Discover GitHub candidate references and feed them into passive intake.

## In scope

- Design minimum-scope GitHub App/token access.
- Search by categories and ecosystem signals.
- Implement rate-limit cache.
- Collect repository metadata evidence.
- Resolve immutable commits.
- Deduplicate forks/copies.

## Out of scope

- Running repository code.
- Automatic admission.
- Using popularity as security.

## Expected deliverables

- GitHub discovery command.
- Cache/evidence records.
- Rate-limit behavior.

## Required tests and evidence

- Pagination.
- Rate limiting.
- Deleted/private repo.
- Fork/duplicate.
- Branch mutation.
- Malformed response.

## Acceptance criteria

- Discovery returns references only.
- Intake always uses immutable commits.
- Credentials are minimum scope and redacted.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt, then mandatory Opus 5 security review.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
