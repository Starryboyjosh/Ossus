# WAVE-008 — Claude Code adapter

## Status

**Planned.** No completion report or current-WAVE evidence indicates implementation.

## Technical summary

WAVE-008 will use the activation transaction to materialize selected skills through current Claude Code mechanisms, detect host availability where possible, support `.claude/skills/`, preserve permissions, and use safe symlink or copy behavior.

## Practical plain-language summary

Ossus will make only the chosen skills visible to Claude Code without overwriting a user’s existing skills or claiming that visibility filtering is a sandbox.

## Expected evidence

- Claude activation/deactivation commands, compatibility report, and host-version warnings.
- Tests for clean projects, unmanaged name conflicts, no-symlink platforms, host-exclusive metadata, R3/R4 invocation policy, rollback, and whole-Registry exposure.

## Dependencies and gates

Depends on complete WAVE-007 and closed Gate S3. It does not close a gate.

## Remaining work

Implement the adapter against current Claude Code behavior, complete the required tests, and ensure materialized hashes and ownership behavior remain consistent with WAVE-007.
