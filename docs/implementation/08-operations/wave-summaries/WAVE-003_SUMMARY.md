# WAVE-003 — Seed Registry and local search

## Status

**In progress.** Registry/index/search implementation is locally verified. All
20 seed profiles now have governed dispositions; the provisional
admission-bearing target is 16 and the official Registry contains 2 reviewed
resources (profiles 6 and 9). The bounded final sprint added no new admission:
profile 15 is independently blocked, profiles 5/7/11/12 have amendment
packets, and profile 16 remains valid but unfilled. Hosted Ubuntu/macOS/Windows
release FTS5 passes are recorded, but a separate layout-job discrepancy and
final Closure remain unfinished.

## Technical summary

The disposable local SQLite/FTS Registry index, deterministic rebuild, trusted-metadata search/show APIs, status and reindex commands are implemented. Release FTS5 and deterministic Git-tree hashing now have dedicated tests. Twenty profiles were reconciled, but no proposal becomes official without independent review and Closure. Profiles 6 and 9 are now admitted as standard-only static prompt packs; profile 10 needs an enforced read-only adapter; profile 15 has an accepted surface correction but its candidate is blocked after independent review; profiles 5, 7, 11 and 12 have Curator-only R3 amendment packets; profile 16 retains a valid but unfilled profile; and the original profile-20 candidate is rejected while its replacement is conditional.

## Practical plain-language summary

Ossus can now build and search a local catalog by capability and compatibility without reading or installing resource bodies. The catalog is not yet populated with twenty approved resources: weak or overly broad candidates were rejected instead of being counted.

## Delivered evidence

- Gate S1 closure and authorization: `../WAVE-002_GATE_S1_CLOSURE.md`.
- Current acceptance-state table: `../../CURRENT_WAVE.md`.
- Expected implementation evidence: deterministic rebuild and filter tests, FTS5 availability evidence, conflict and malformed-manifest tests, JSON output checks, provenance/license records, and a WAVE report.
- Interim seed evidence and closure-oriented re-review: `../WAVE-003_SEED_ADMISSION_AND_SOURCE_REPORT.md`.
- Profile reconciliation: `../WAVE-003_SEED_PROFILE_RECONCILIATION.md`.
- Profile amendment packets: `../WAVE-003_PROFILE_AMENDMENT_DECISIONS.md`.
- Official admitted manifests: `../../../../catalog/official/manifests/` (profiles 6 and 9).
- Curator-only exact field drafts (quarantined, not official): `research-evidence/wave003-staging/`.
- Interrupted-work handoff: `../HANDOFF_2026-08-08_WAVE-003.md`.
- Closure-push handoff: `../HANDOFF_2026-08-08_WAVE-003_CLOSURE_PUSH.md`.
- Current post-push handoff: `../HANDOFF_2026-08-08_WAVE-003_POST_PUSH.md`.
- Final-admission sprint handoff: `../HANDOFF_2026-08-08_WAVE-003_FINAL_ADMISSION_SPRINT.md`.

## Dependencies and gates

Depends on complete WAVE-002 and closed Gate S1. It does not close a gate.

## Remaining work

Obtain distinct Closure decisions only for candidates that first pass bounded
independent review, admit canonical manifests incrementally, record hosted
release FTS5 evidence (already passing on all three supported CI platforms),
resolve the separate hosted layout-job discrepancy, regenerate inventories with
the generator, rerun full verification, and obtain final WAVE Closure. Profile 10 needs an immutable
read-only adapter; profile 15 needs a dependency-only adapter, freshness and
redaction evidence; profile 16 needs a clean licensed, self-contained Claude
candidate; profile 20 needs a safe MCP replacement or an architecture
amendment. WAVE-004 remains unauthorized.
