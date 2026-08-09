# WAVE-014 Summary — Codex adapter

## Status

Planned.

## Technical summary

Materializes selected skills into supported Codex repository or approved user locations, deriving `SKILL.md` and optional host metadata solely from canonical policy. It reuses activation transactions and ownership while supporting disabled implicit invocation.

## Practical plain-language summary

This lets users activate only the skills they selected for Codex, without exposing the whole Registry or letting untrusted origin data control host behavior.

## Expected evidence/deliverables

- Codex activation command, adapter tests, and documentation.
- Repository/user scope, symlink, duplicate-name, disabled-invocation, host-dependency, and rollback tests.
- Evidence that elevated scopes are not defaults and only selected resources materialize.

## Dependencies/gates

Depends on WAVE-012 and WAVE-013. Codex paths and host metadata require Agent Review Authority review.

## Remaining work

Verify current Codex paths, implement policy-derived materialization and transaction reuse, add the adapter test matrix, and complete review evidence.
