# WAVE-006 — Golden evaluation harness

## Status

**Planned.** No completion report or current-WAVE evidence indicates implementation.

## Technical summary

WAVE-006 will execute the 50-case evaluation set, add exact-resource expectations once seed IDs exist, measure selection quality and safety constraints, and benchmark a synthetic 1,000-manifest catalog.

## Practical plain-language summary

Ossus will have a repeatable scorecard showing whether its recommendations are useful, safe, fast, and smaller than loading the entire catalog.

## Expected evidence

- `ossus eval`, machine-readable and human reports, Layer 2 exact-resource data, and a baseline performance report.
- Metric, failure-reporting, constraint-accounting, cold/warm-run, and determinism tests.
- Evidence that V0 thresholds pass: zero constraint violations, zero implicit R4 activations, and at least 80% context reduction.

## Dependencies and gates

Depends on WAVE-005. It does not close a gate, but its evidence is a prerequisite for WAVE-007.

## Remaining work

Implement the harness and benchmark, populate exact-resource expectations from real seed entries, and meet the frozen thresholds without weakening them.
