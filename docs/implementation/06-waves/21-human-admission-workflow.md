# WAVE-021 — Agent review and admission workflow

**Phase:** Phase 5  
**Assigned role:** Implementer Agent, independent Security Review Agent, and distinct Closure Agent
**Depends on:** WAVE-019 and WAVE-020  
**Security WAVE:** yes

## Objective

Create an auditable path from evidence to agent-authored canonical contribution.

## In scope

- Risk-tier checklists.
- Curator Agent, Admission Review Agent, and Closure Agent identities, with conflict-of-interest attestations.
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
- Gate S6 Closure Agent decision.

## Required tests and evidence

- Trigger stuffing.
- Permission understatement.
- Reviewer count.
- Stale evidence.
- Changed commit.
- Attempted main write.

## Acceptance criteria

- Canonical fields require independent Admission Review Agent action.
- Risk tier enforces the required independent review depth.
- Mutable/changed source cannot admit.
- Independent Security Review Agent assessment and distinct Closure Agent decision complete.


## Mandatory security workflow

This is a security WAVE.

- An Implementer Agent prepares the work and evidence.
- An independent Security Review Agent reviews the final implementation and evidence.
- A distinct Closure Agent makes the final gate decision after review; no agent may review or close its own work.
- Human review is optional additional evidence. Missing or unverifiable evidence fails closed.


## Copy-ready implementation instruction

Use security prompts. Suggestions must never pre-approve Resolver-critical fields. Update the reader summary with technical and practical summaries, evidence, dependencies/gates, status, and remaining work.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
