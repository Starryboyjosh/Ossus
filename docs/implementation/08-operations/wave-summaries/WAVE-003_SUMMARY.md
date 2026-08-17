# WAVE-003 — Seed Registry and local search

## Status

**In progress.** Registry/index/search implementation is locally verified. All
20 seed profiles now have governed dispositions. The final coverage authority
records 9 `SEED_REQUIRED`, 7 `SEED_REQUIRED_WITH_SUBSTITUTION`, and 4
`INTENTIONALLY_UNRESOLVED` profiles; 16 is a provisional planning denominator,
while the active WAVE completion obligation remains 20 until architecture
changes it. The official Registry contains 3 reviewed resources (profiles 2,
6 and 9). Profile 2 is an R0 standard-only skill and does not prove Claude,
Codex, or CLI compatibility. Profile 15 is useful but
deferred, profiles 5/7/11/12 have Closure-accepted profile-only ceiling
corrections (R3/R2/R2/R3), and profile 16 remains valid but unfilled. Hosted
Ubuntu/macOS/Windows and Arch-container release FTS5 plus complete CI run 22
pass. Positive seed diversity and final admissions remain unfinished.

## Technical summary

The disposable local SQLite/FTS Registry index, deterministic rebuild, trusted-metadata search/show APIs, status and reindex commands are implemented. Release FTS5 and deterministic Git-tree hashing now have dedicated tests. Twenty profiles were reconciled, but no proposal becomes official without independent review and Closure. Profiles 2, 6 and 9 are now admitted as standard-only resources; profile 10 needs an enforced read-only adapter; profile 15 has an accepted surface correction but its candidate is useful and deferred after independent review; profiles 5, 7, 11 and 12 have bounded profile-only ceiling corrections (R3/R2/R2/R3) but no candidate admission; profile 16 retains a valid but unfilled profile; and the original profile-20 candidate is rejected while its replacement is conditional.

## Practical plain-language summary

Ossus can now build and search a local catalog by capability and compatibility without reading or installing resource bodies. The catalog is not yet populated with twenty approved resources: weak or overly broad candidates were rejected instead of being counted.

## Delivered evidence

- Gate S1 closure and authorization: `../WAVE-002_GATE_S1_CLOSURE.md`.
- Current acceptance-state table: `../../CURRENT_WAVE.md`.
- Expected implementation evidence: deterministic rebuild and filter tests, FTS5 availability evidence, conflict and malformed-manifest tests, JSON output checks, provenance/license records, and a WAVE report.
- Interim seed evidence and closure-oriented re-review: `../WAVE-003_SEED_ADMISSION_AND_SOURCE_REPORT.md`.
- Profile reconciliation: `../WAVE-003_SEED_PROFILE_RECONCILIATION.md`.
- Profile amendment packets and independent review: `../WAVE-003_PROFILE_AMENDMENT_DECISIONS.md`.
- Final coverage authority: `../WAVE-003_FINAL_COVERAGE_AUTHORITY.md`.
- Official admitted manifests: `../../../../catalog/official/manifests/` (profiles 2, 6 and 9).
- Targeted profile-2 admission packet: `../WAVE-003_TARGETED_ADMISSION_2026-08-16.md`.
- Profile-2 Closure record: `../WAVE-003_RESPONSIVE_DESIGN_CLOSURE_2026-08-16.md`.
- Independent WAVE acceptance review: `../WAVE-003_INDEPENDENT_ACCEPTANCE_2026-08-16.md`;
  verdict `BLOCKED` for the remaining 20-entry and diversity obligations.
- Curator-only exact field drafts (quarantined, not official): `research-evidence/wave003-staging/`.
- Interrupted-work handoff: `../HANDOFF_2026-08-08_WAVE-003.md`.
- Closure-push handoff: `../HANDOFF_2026-08-08_WAVE-003_CLOSURE_PUSH.md`.
- Current post-push handoff: `../HANDOFF_2026-08-08_WAVE-003_POST_PUSH.md`.
- Final-admission sprint handoff: `../HANDOFF_2026-08-08_WAVE-003_FINAL_ADMISSION_SPRINT.md`.
- Arch coverage handoff: `../HANDOFF_2026-08-08_WAVE-003_ARCH_COVERAGE.md`.
- Arch CI review: `../WAVE-003_ARCH_CI_REVIEW.md`.

## Dependencies and gates

Depends on complete WAVE-002 and closed Gate S1. It does not close a gate.

## Remaining work

Obtain distinct Closure decisions only for candidates that first pass bounded
independent review, admit canonical manifests incrementally, preserve the
Git-index-byte inventory fix, and rerun inventory/full verification after each
future materialization before obtaining final WAVE Closure. The targeted
profile-2 admission is complete and leaves the official Registry at three
resources.
Hosted Ubuntu, Arch
container, macOS and Windows release FTS5 evidence is recorded in CI run 22.
Profile 10 needs an immutable read-only adapter; profile 15 needs a
dependency-only adapter, freshness and redaction evidence; profile 16 needs a
clean licensed, self-contained Claude candidate; profile 20 needs a safe MCP
replacement or an architecture amendment; and the official Registry needs
real skill/MCP/surface/risk/overlap diversity. WAVE-004 remains unauthorized.
