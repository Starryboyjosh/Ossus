# WAVE-021 — Review and admission workflow

## Status

Planned. No repository evidence shows implementation or closure.

## Technical summary

Create an auditable admission path from evidence to an agent-authorized canonical contribution: risk-tier checklists, reviewer identity and approvals, blank-first manifest drafting with non-authoritative suggestions, diff/evidence references, isolated staging contributions, and admission/rejection records. Researcher cannot write main or pre-approve Resolver-critical fields.

## Practical plain-language summary

Make acceptance of a candidate a deliberate, traceable review decision rather than an automatic outcome of collected evidence.

## Expected evidence/deliverables

- Review-bundle command, admission package, checklists, and Gate S6 evidence.
- Tests for trigger stuffing, understated permissions, insufficient reviewers, stale evidence, changed commits, and attempted main writes.
- Agent Review Authority security assessment.

## Dependencies/gates

Depends on WAVEs 019 and 020. It closes Gate S6 under Agent Review Authority and is required before WAVE-022.

## Remaining work

Implement the auditable workflow and staging boundary, enforce review counts and immutable source checks, run adversarial tests, and obtain Gate S6 assessment.
