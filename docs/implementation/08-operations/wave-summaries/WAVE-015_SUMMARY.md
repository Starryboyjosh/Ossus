# WAVE-015 Summary — Optional local semantic fallback

## Status

Planned.

## Technical summary

Evaluates an optional local classifier or embedding fallback through an ADR. It is disabled by default, consumes only task text and trusted taxonomy metadata, records model/version/confidence, and degrades to low confidence when unavailable or invalid.

## Practical plain-language summary

This may improve unclear task matching without sending project data away or changing the reliable deterministic default behavior.

## Expected evidence/deliverables

- Optional feature or plugin, accuracy/performance comparison, and privacy/size documentation.
- Disabled-default, missing/corrupt-model, deterministic-mode, and non-regression golden tests.
- Evidence that the base binary works without a model and metrics justify adoption.

## Dependencies/gates

Depends on WAVE-014. The local runtime requires an ADR and Agent Review Authority review before adoption.

## Remaining work

Evaluate candidate local runtimes, write the ADR, implement only an approved approach, and demonstrate improved reviewed metrics without regressions.
