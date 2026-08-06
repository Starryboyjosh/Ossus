# WAVE-015 — Optional local semantic fallback

**Phase:** Phase 4  
**Assigned role:** Implementation agent  
**Depends on:** WAVE-014  
**Security WAVE:** no

## Objective

Improve ambiguous task mapping without remote token cost or changing deterministic defaults.

## In scope

- Evaluate local classifier/embedding options via ADR.
- Keep disabled by default.
- Feed only task and trusted taxonomy metadata.
- Record model/version/confidence.
- Fail back to low confidence.

## Out of scope

- Remote LLM requirement.
- Resource-body reading.
- Automatic model download.

## Expected deliverables

- Optional feature or plugin.
- Accuracy/performance comparison.
- Privacy and size docs.

## Required tests and evidence

- Disabled default.
- Missing/corrupt model.
- Deterministic mode unchanged.
- Goldens do not regress.

## Acceptance criteria

- Base binary works without a model.
- No project/task data leaves the machine.
- Feature is adopted only if reviewed metrics improve.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt. An ADR must justify the local runtime.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
