# WAVE-023 — Researcher end-to-end security audit

**Phase:** Phase 5  
**Assigned role:** Opus 5 security auditor; optional Luna Max evidence support  
**Depends on:** WAVE-022  
**Security WAVE:** yes

## Objective

Attack the complete Researcher path before beta.

## In scope

- End-to-end adversarial fixtures.
- Quarantine escapes.
- CI/token review.
- Source substitution.
- Prompt injection.
- Evidence poisoning.
- Admission bypass.
- Privacy and cleanup.

## Out of scope

- New product features.
- Schedule-driven severity reduction.

## Expected deliverables

- Opus 5 security audit.
- Attributed implementation-support evidence and final assessment.
- Correction WAVEs if needed.
- Distinct Closure Agent Gate S7 decision.

## Required tests and evidence

- All Researcher attack fixtures.
- Cross-platform paths.
- Concurrent jobs.
- Interrupted cleanup.
- Compromised upstream update.
- Malicious scanner output.

## Acceptance criteria

- No unresolved critical/high finding.
- Researcher cannot approve or activate.
- Candidate content cannot reach privileged CI.
- The distinct Closure Agent decision is recorded.


## Mandatory security workflow

This is a security WAVE.

- A capable independent Security Review Agent owns the final security audit and evidence review.
- Implementation agents may assist only through attributed, bounded tasks.
- A distinct Closure Agent closes the gate and has the final word.
- Human review is optional additional evidence.


## Copy-ready implementation instruction

Use the security prompts as an audit. Corrections must be explicit, tested and re-reviewed.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
