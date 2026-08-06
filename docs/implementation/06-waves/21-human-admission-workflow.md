# WAVE-021 — Human review and admission workflow

**Phase:** Phase 5  
**Assigned role:** Opus 5 security owner; optional Luna Max implementation support  
**Depends on:** WAVE-019 and WAVE-020  
**Security WAVE:** yes

## Objective

Create an auditable path from evidence to human-authored canonical contribution.

## In scope

- Risk-tier checklists.
- Reviewer identity/approvals.
- Blank-first canonical manifest drafting with non-authoritative suggestions.
- Diff/evidence references.
- Separate staging contribution.
- Admission and rejection records.

## Out of scope

- Automatic approval.
- Direct Researcher write to main.
- Privileged candidate CI.

## Expected deliverables

- Review-bundle command.
- Admission package.
- Checklists.
- Gate S6 closure.

## Required tests and evidence

- Trigger stuffing.
- Permission understatement.
- Reviewer count.
- Stale evidence.
- Changed commit.
- Attempted main write.

## Acceptance criteria

- Canonical fields require reviewer action.
- Risk tier enforces reviewer count.
- Mutable/changed source cannot admit.
- Opus 5 security assessment and human closure complete.


## Mandatory security workflow

This is a security WAVE.

- Opus 5 owns the assigned security work and reviews the final implementation or audit evidence.
- Luna Max or another implementation agent may assist only through attributed, bounded tasks.
- The human closes the gate.
- There is no automatic replacement if Opus 5 is unavailable.
- Model review is evidence, not certification.


## Copy-ready implementation instruction

Use security prompts. Suggestions must never pre-approve Resolver-critical fields.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
