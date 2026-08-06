# WAVE-008 — Claude Code adapter

**Phase:** Phase 3  
**Assigned role:** Implementation agent with security review  
**Depends on:** WAVE-007  
**Security WAVE:** no

## Objective

Materialize selected skills through current Claude Code native mechanisms.

## In scope

- Detect Claude Code availability/version when possible.
- Support project scope under `.claude/skills/`.
- Generate or adapt `SKILL.md` without broadening permissions.
- Use safe symlinks or copy fallback.
- Disable implicit invocation when policy requires.
- Reuse activation transactions.

## Out of scope

- Plugin marketplace export.
- Agent SDK adapter.
- Codex adapter.
- Automatic host permission changes.

## Expected deliverables

- Claude activation/deactivation commands.
- Compatibility report.
- Host-version warnings.

## Required tests and evidence

- Clean project.
- Existing unmanaged same-name resource.
- No-symlink platform.
- Host-exclusive metadata.
- R3/R4 invocation policy.
- Rollback.
- Whole Registry exposure test.

## Acceptance criteria

- Only selected resources are visible.
- Unmanaged skills remain intact.
- Lockfile hashes match materialized content.
- No sandbox claim.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt. Reverify current official Claude Code skill behavior during implementation.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
