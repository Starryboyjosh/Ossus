# WAVE-004 — Bounded project scanner

**Phase:** Phase 2  
**Assigned role:** Implementation agent  
**Depends on:** WAVE-002  
**Security WAVE:** no

## Objective

Generate deterministic project profiles without broadly ingesting source code.

## In scope

- Detect languages, package managers, frameworks, databases, tests, CI and host signals.
- Honor ignore rules and explicit exclusions.
- Implement file, byte, depth and time budgets.
- Produce stable JSON and profile hash.
- Add common-stack and monorepo fixtures.

## Out of scope

- Task interpretation.
- Resource selection.
- Network calls.
- Secret scanning.

## Expected deliverables

- `ossus scan`.
- Detector registry and schema.
- Visible truncation explanations.
- Fixtures.

## Required tests and evidence

- Rust, Node, Python and mixed projects.
- Nested monorepos.
- Symlink loops.
- Permission denied.
- Huge-tree budgets.
- Secret-like filenames excluded by default.
- Deterministic hash.

## Acceptance criteria

- No link is followed outside the root.
- Limits appear in output.
- Identical fixtures produce identical normalized profiles.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt. Prefer metadata detectors over source-body inspection.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
