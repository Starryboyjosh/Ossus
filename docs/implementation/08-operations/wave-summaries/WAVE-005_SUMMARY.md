# WAVE-005 — Deterministic Resolver core

## Status

**Planned.** No completion report or current-WAVE evidence indicates implementation.

## Technical summary

WAVE-005 will normalize tasks, expand governed aliases, retrieve Registry candidates, apply policy and compatibility exclusions before scoring, calculate deterministic minimal coverage, and produce confidence and explanation records.

## Practical plain-language summary

Ossus will choose the smallest safe, compatible set of resources for a task and explain both selections and exclusions.

## Expected evidence

- Selection-plan contract, versioned algorithm, `resolve`/`explain` output, and reason codes.
- Tests for coverage, redundancy, policy denial before scoring, host mismatch, implicit R4 denial, low confidence, stable tie-breaking, active limits, and byte-equivalent output.
- An adversarial policy-monotonicity fixture proving project configuration cannot relax policy.
- Gate S2 closure record and Agent Review Authority decision.

## Dependencies and gates

Depends on WAVE-003 and WAVE-004. It closes Gate S2; WAVE-007 cannot begin until that gate is closed.

## Remaining work

Implement Resolver and scope-aware policy behavior, prove all gate criteria, and close Gate S2 through Agent Review Authority. Activation, remote models, embeddings, and synchronization are excluded.
