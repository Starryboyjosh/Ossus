# WAVE-012 Summary — Registry synchronization and selective install

## Status

Planned.

## Technical summary

Adds trusted Registry source configuration, validated snapshot fetch/swap with rollback, immutable selected-content download, content-addressed caching, and freshness/revocation reporting. Source transports are strictly limited to HTTPS for remote/vendored sources and absolute platform-resolved paths for local-private sources.

## Practical plain-language summary

Users can refresh catalog information and fetch only what they selected, without a failed sync corrupting the current usable state or changing active resources.

## Expected evidence/deliverables

- Registry add/sync/status commands, selective-install primitive, and freshness/revocation display.
- Interrupted-sync, bad-digest, schema, rollback, offline-stale, hash-conflict, transport-rejection, and concurrent sync/resolve tests.
- Proof that only immutable content installs and failed sync preserves valid state.

## Dependencies/gates

Depends on WAVE-011. Source trust and network behavior require Agent Review Authority review. Sync must never activate resources automatically.

## Remaining work

Implement the synchronization and cache primitives, enforce the allowlist at configuration time, add concurrency coverage, and collect the review evidence.
