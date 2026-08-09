# WAVE-007 — Activation security boundary

## Status

**Planned.** No completion report or current-WAVE evidence indicates implementation.

## Technical summary

WAVE-007 will create `ossus-activation` and own host-neutral activation transactions: staged hash verification, path and symlink validation, journalled recoverable swaps, ownership tracking, managed deactivation, rollback, locking, and audit events.

## Practical plain-language summary

Ossus will safely expose only the selected resources to a host, while preserving existing user files and recovering safely if activation fails.

## Expected evidence

- Host-neutral activation API, security report, threat-model delta, contribution attribution, and Gate S3 closure record.
- Tests for traversal, absolute paths, external symlinks, hash mismatch, TOCTOU, crash rollback, modified and unmanaged files, concurrent activation, in-flight sync/resolve, and idempotent stale-journal recovery.

## Dependencies and gates

Depends on WAVE-006 and closed Gate S2. It closes Gate S3 through Agent Review Authority before dependent host work proceeds.

## Remaining work

Satisfy the activation design precondition, implement the eighth crate and transaction model, pass all attack and recovery tests, and close Gate S3. Claude-specific formatting and remote synchronization are not included.
