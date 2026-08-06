# Activation security

## Required sequence

1. Load a previously generated selection plan.
2. Revalidate plan schema and policy hash.
3. Recheck Registry snapshot and revocations.
4. Verify every source commit and content hash.
5. Inventory selected files against adapter allowlist.
6. Reject path traversal and external symlinks.
7. Display risk, runtime and permission summary.
8. Stage host output in a transaction directory.
9. Validate staged host structure.
10. Atomically replace only Ossus-managed paths.
11. Write lockfile and ownership record.
12. Retain rollback metadata.

## Least exposure

The host receives only selected resources.

The global content store remains outside the project and host scan roots.

For Claude Code V0, project materialization targets controlled subdirectories under `.claude/skills/`.

## Ownership record

Ossus records each managed path and digest.

Deactivation deletes only paths whose ownership and current state match the record.

Modified paths require confirmation and are never silently deleted.

## Invocation policy

- R0/R1 may support implicit host invocation when policy allows.
- R2 requires user confirmation before activation.
- R3 requires explicit activation and runtime warning.
- R4 is explicit invocation only.
- R5 is denied.

Host-specific invocation controls are generated when supported.

## Rollback

Activation must be transactional.

On failure, leave prior active state intact, remove incomplete staging, emit a recovery report and return a stable exit code.

If atomic replacement is not supported, use a documented two-phase strategy and test crash recovery.

## Concurrency, atomicity and recovery

This section is a design precondition for WAVE-007. It must be satisfied by the design before implementation begins, not discovered during it.

### Transaction unit

The transaction unit is the **whole active set**, not the individual resource. A partially-applied set satisfies no policy check: the Resolver's set-cover output is only meaningful as a set, and activation limits and R4 rules are set-level properties.

Host discovery constrains how this is achieved. Claude Code requires `SKILL.md` at `.claude/skills/<name>/`, so there is no single Ossus-owned parent directory that can be renamed in one atomic operation. The multi-rename is therefore unavoidable and must be journalled.

### Journalled multi-rename

1. Write a transaction journal under `$OSSUS_HOME/transactions/<txn-id>/` before touching any host path. The journal records every intended rename, the prior state of each target path (absent, Ossus-managed with digest, or unmanaged) and the staging location.
2. Apply renames one at a time, appending a commit marker per completed rename.
3. On success, write the lockfile and ownership record, then remove the journal.
4. On failure or on a subsequent run finding a journal, replay the journal **backwards**. Rollback is idempotent: replaying a completed rollback is a no-op, so a crash during rollback is recoverable by repeating it.

`ossus doctor` reports stale transactions and offers replay. A stale journal never auto-applies forward.

### Locking

- An exclusive lock on the project `.ossus/` directory serializes activation within a project.
- An exclusive lock on `$OSSUS_HOME/transactions` serializes activation across projects that share a store.
- Locks record the holder's process identity and start time. A lock whose holder is demonstrably gone is stale; a stale lock may be broken only after journal replay, never by deletion alone.
- A second concurrent `ossus activate` fails with a stable lock reason code. It does not wait indefinitely and does not proceed.

### TOCTOU

Hash verification runs against the **staged bytes** that will be renamed into place, not against the store copy that was read earlier. Verifying the store and then copying reopens the window this control exists to close.

`registry sync` may not mutate a snapshot an in-flight resolve is reading. A resolve completes against the snapshot it started with.

### Shared store concurrency

`SOURCE_AND_INSTALLATION_MODEL.md` permits a shared custom store path "provided permissions and concurrency are handled". That clause is owned here: a shared store requires the `$OSSUS_HOME/transactions` lock above, content-addressed writes performed as write-to-temp-then-rename within the same filesystem, and no in-place mutation of an existing store object.

### Required evidence

- Crash injection between per-resource renames, asserting the prior active set is fully restored and no unmanaged path was touched.
- Two concurrent `ossus activate` runs in one project, asserting exactly one succeeds and the other returns the lock reason code.
- `registry sync` during an in-flight resolve, asserting the resolve completes against its snapshot.
- Stale-journal replay after a simulated kill, asserting idempotence when replayed twice.
