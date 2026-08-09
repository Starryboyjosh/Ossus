# WAVE-017 — Researcher security design

**Phase:** Phase 5  
**Assigned role:** Opus 5 security owner; optional Luna Max implementation support  
**Depends on:** WAVE-016  
**Security WAVE:** yes

## Objective

Freeze Researcher boundaries, contracts and execution prohibitions before code.

## In scope

- Formal Researcher threat model.
- Quarantine locations and budgets.
- Evidence-bundle schema.
- Connector interface.
- Staging repository/CI model.
- Legal and privacy decision points.

## Out of scope

- Connector implementation.
- Candidate execution.
- Model approval.

## Expected deliverables

- Researcher security specification.
- Opus 5 security review and attributed support evidence.
- Distinct Closure Agent Gate S5 design decision.

## Required tests and evidence

- Threat scenarios for archives, links, hooks, submodules, huge repos and malicious manifests.
- CI token scenarios.
- Evidence/canonical boundary review.

## Acceptance criteria

- No Researcher code starts before closure.
- Every attacker input has a budget/destination.
- Researcher remains non-approving.


## Mandatory security workflow

This is a security WAVE.

- A capable independent Security Review Agent reviews the final design and evidence.
- Implementation agents may assist only through attributed, bounded tasks.
- A distinct Closure Agent closes the gate and has the final word.
- Human review is optional additional evidence.


## Copy-ready implementation instruction

Use security prompts. This is a design and test-design WAVE, not product implementation.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
