# WAVE-003 — Seed Registry and local search

**Phase:** Phase 2  
**Assigned role:** Implementation agent  
**Depends on:** WAVE-002 and historical Security Gate S1 closure; ADR-020 governs any new decision
**Security WAVE:** no

## Objective

Build the index-first Registry and searchable manual seed catalog.

## In scope

- Reconcile the 20 seed coverage profiles against ecosystem evidence, then
  admit only resources that survive the Curator Agent → independent Admission
  Review Agent → distinct Closure Agent chain. Twenty is a coverage objective,
  not a quota; profile substitutions, intentionally unresolved decisions, and
  a smaller minimum useful catalog must be explicit and architecture-compliant.
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

- Reviewed manifests for the reconciled minimum useful catalog; no profile is
  counted until its canonical resource is admitted.
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
- Hosted release FTS5 evidence recorded separately for Ubuntu, the Arch Linux
  validation container, macOS and Windows. The Arch lane is an Arch userspace
  running on an Ubuntu-hosted runner; it is not evidence from a native Arch
  host and must not be reported as such.

## Acceptance criteria

- Every profile has a governed final disposition (admitted, approved
  substitution, intentionally unresolved, or removed by architecture
  decision), and every admitted resource has a complete authority chain.
- The set includes host-exclusive, cross-host and standalone-CLI examples.
- Search never reads resource bodies.
- Index is disposable.


## Review workflow

Use an Implementer Agent, independent Review Agent, and distinct Closure Agent. Require an independent Security Review Agent whenever the work changes a trust boundary, network source, host path, permission, update mechanism, or CI configuration; no self-review or self-closure.


## Copy-ready implementation instruction

Use the general implementer prompt. Independent agent admission is mandatory; do not fabricate entries, evidence, or approvals. Update the WAVE reader summary with technical and practical summaries, evidence, dependencies/gates, status, and remaining work.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
