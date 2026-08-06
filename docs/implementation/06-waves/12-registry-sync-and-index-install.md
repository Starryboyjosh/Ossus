# WAVE-012 — Registry synchronization and selective install

**Phase:** Phase 4  
**Assigned role:** Implementation agent with security review  
**Depends on:** WAVE-011  
**Security WAVE:** no

## Objective

Safely synchronize Registry metadata and fetch only fixed selected content.

## In scope

- Configure official Registry sources.
- Enforce the **source transport allowlist** defined in `DATA_CONTRACTS.md` (F-15): `remote-index` and `vendored` accept `https://` only; `local-private` accepts an absolute local path resolved through platform path APIs. `file://`, `git://`, `http://`, `ssh://` and scp-style references are rejected at configuration time, not at fetch time. SSH is excluded specifically because agent forwarding would let a manifest borrow the user's credentials.
- Fetch and validate snapshots.
- Atomically swap indexes with rollback.
- Download immutable selected content.
- Use content-addressed cache.
- Report offline/freshness state.

## Out of scope

- Mutable branches.
- Automatic activation after sync.
- Candidate sources.

## Expected deliverables

- Registry add/sync/status.
- Selective install primitive.
- Freshness/revocation display.

## Required tests and evidence

- Interrupted sync.
- Bad digest.
- Unsupported schema.
- Rollback.
- Offline stale state.
- Same version with different hash.
- **F-15** — a rejection fixture per disallowed scheme (`file://`, `git://`, `http://`, `ssh://`, `git@host:org/repo`), each failing at configuration time with a specific reason code.
- **F-07** — `registry sync` running during an in-flight resolve, asserting the resolve completes against the snapshot it started with and that sync does not mutate it.

## Acceptance criteria

- Sync never changes active resources.
- Only fixed content installs.
- Failed sync preserves valid state.
- No transport outside the allowlist can be configured by any path.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt and obtain Opus 5 review because source trust and network behavior change.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
