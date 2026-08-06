# WAVE-014 — Codex adapter

**Phase:** Phase 4  
**Assigned role:** Implementation agent with security review  
**Depends on:** WAVE-012 and WAVE-013  
**Security WAVE:** no

## Objective

Materialize selected skills into current Codex-native repository or user locations.

## In scope

- Detect Codex surface/version.
- Support repository `.agents/skills` and approved user scope.
- Generate `SKILL.md` and optional host metadata only from canonical policy.
- Support disabling implicit invocation.
- Reuse transactions and ownership.

## Out of scope

- Plugin submission.
- Hosted ChatGPT upload.
- Admin/system scope by default.

## Expected deliverables

- Codex activation command.
- Adapter tests and docs.

## Required tests and evidence

- Repository/user scope.
- Symlinks.
- Duplicate names.
- Implicit invocation disabled.
- Host dependency declarations.
- Rollback.

## Acceptance criteria

- Only selected resources materialize.
- Elevated scopes are not default.
- Origin metadata cannot control host fields.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt. Reverify current official Codex skill paths and request Opus 5 review of paths and metadata.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
