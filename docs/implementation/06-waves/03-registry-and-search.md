# WAVE-003 — Seed Registry and local search

**Phase:** Phase 2  
**Assigned role:** Implementation agent  
**Depends on:** WAVE-002, and on **Security Gate S1 being closed by a named human**  
**Security WAVE:** no

## Objective

Build the index-first Registry and searchable manual seed catalog.

## In scope

- Select and human-curate 20 real seed entries matching the profiles.
- Create canonical manifests and provenance/license records.
- Implement SQLite schema, migrations and rebuild.
- Implement exact, capability, category and FTS search.
- Implement surface, source, runtime and risk filters.

## Out of scope

- Remote synchronization.
- Installing bodies.
- Resolver.
- Researcher discovery.

## Expected deliverables

- 20 reviewed manifests.
- Rebuildable local index.
- `ossus search`, `show`, `registry status` and `reindex`.
- Seed license/source report.

## Required tests and evidence

- Index rebuild equivalence.
- FTS5 availability in release builds.
- Combined filters.
- Namespace and same-version hash conflicts.
- **F-09** — a conflict fixture where two entries reference the same commit in different letter cases. Because WAVE-002 normalizes `source.commit` to lowercase at ingest, these must collide as one commit rather than index as two distinct sources.
- Malformed manifest exclusion.
- JSON output schema.

## Acceptance criteria

- Every profile is filled or an approved substitution is documented.
- The set includes host-exclusive, cross-host and standalone-CLI examples.
- Search never reads resource bodies.
- Index is disposable.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt. Human source curation is mandatory; do not fabricate approvals.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
