# WAVE-013 — Private catalogs and source precedence

**Phase:** Phase 4  
**Assigned role:** Implementation agent  
**Depends on:** WAVE-012  
**Security WAVE:** no

## Objective

Support project, user and private overlays without silent impersonation.

## In scope

- Multiple sources.
- Priority and namespace rules.
- Explicit override records.
- Conflict explanations.
- Project-local canonical entries.

## Out of scope

- Remote authorization service.
- Automatic conflict merge.

## Expected deliverables

- Source-management CLI.
- Precedence resolver.
- Conflict audit output.

## Required tests and evidence

- Same official/private ID.
- Project override.
- Unknown publisher.
- Revoked lower source.
- Hash conflict.

## Acceptance criteria

- Every override is visible.
- No source silently impersonates another namespace.
- Lockfiles record source identity.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
