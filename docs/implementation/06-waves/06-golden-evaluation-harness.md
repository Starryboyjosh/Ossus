# WAVE-006 — Golden evaluation harness

**Phase:** Phase 2  
**Assigned role:** Implementation agent and independent reviewer  
**Depends on:** WAVE-005  
**Security WAVE:** no

## Objective

Measure capability mapping, exact resource selection, safety constraints and performance.

## In scope

- Implement the 50-case loader.
- Add exact-resource Layer 2 after real IDs exist.
- Implement aggregate and per-case metrics.
- Add synthetic 1,000-manifest benchmark data.
- Add adversarial Resolver fixtures.
- Tune only through justified implementation changes.

## Out of scope

- Activation tests.
- Researcher evaluation.
- Threshold reduction.

## Expected deliverables

- `ossus eval`.
- Layer 2 data.
- JSON/human reports.
- Baseline performance report.

## Required tests and evidence

- Metric formula unit tests.
- Failure reporting.
- Constraint accounting.
- Cold/warm runs.
- Repeated determinism.

## Acceptance criteria

- V0 thresholds pass.
- Zero constraint violations.
- Zero implicit R4 activations.
- At least 80% context reduction.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt. Treat a failing golden as evidence, not an invitation to rewrite expectations.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
