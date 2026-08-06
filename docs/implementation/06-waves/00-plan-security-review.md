# WAVE-000 — Opus 5 security and architecture review

**Phase:** Phase 0  
**Assigned role:** Opus 5 security and architecture reviewer  
**Optional support:** Luna Max for bounded evidence gathering and test preparation  
**Depends on:** None  
**Security WAVE:** yes

## Objective

Challenge and correct the implementation plan before product implementation.

## In scope

- Run the Opus 5 adversarial security and architecture plan review.
- Attribute any Luna Max or other implementation-agent assistance.
- Disposition every finding and edit the plan.
- Record human decisions and residual risks.

## Out of scope

- Writing product code.
- Creating the Git repository.
- Treating model review as certification.
- Treating implementation-agent output as Opus 5 evidence.

## Expected deliverables

- Opus 5 report.
- Finding disposition matrix.
- Updated plan and ADRs.
- Human Gate S0 closure.

## Required tests and evidence

- Every critical/high finding has a disposition.
- Trust boundaries and WAVE order agree.
- Model assignments and contribution attribution remain explicit.

## Acceptance criteria

- Gate S0 is closed by a human.
- No unresolved critical finding.
- All changes are traceable.

## Mandatory security workflow

This is a security WAVE.

- Opus 5 performs the assigned security and architecture review.
- Luna Max or another implementation agent may assist only with attributed, bounded evidence gathering or test preparation.
- The human closes the gate.
- There is no automatic replacement if Opus 5 is unavailable.
- Model review is evidence, not certification.

## Copy-ready implementation instruction

Use `07-prompts/PLAN_REVIEW_OPUS5.md`, then `FINDINGS_DISPOSITION.md` and the human closure checklist.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
