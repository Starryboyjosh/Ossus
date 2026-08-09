# WAVE-003 implementation report

## Metadata

- WAVE: WAVE-003 — Seed Registry and local search
- Implementer: root Codex agent
- Independent reviewers: seed Admission Review Agents A/B; FTS/hash Security Review Agent
- Closure Agent: distinct WAVE-003 security Closure Agent for ADR-021/CI and seed decisions; profiles 6 and 9 were accepted and admitted in the continuation batch
- Date: 2026-08-08
- Base commit: `ec9f1aa23aefa48f75a1db5396d232fd16bd02e0`
- Checkpoint commits: `4cc66c5` (mechanics), `0cb3987` (governance/evidence),
  `17d6e0d` (admitted manifests and final handoff); pushed to `origin/main`

## Objective completed

Partially. Registry/search/CLI implementation is complete locally. Two R0
standard-only resources are now admitted; hosted platform FTS5 evidence is not
yet observable because the GitHub Actions/API endpoint is private to the
authenticated project account. The profile reconciliation keeps 20 governed
profile decisions but sets a provisional 16 admission-bearing target; profiles
10, 17, 18, and 20 are intentionally unresolved. Profile 15 has an accepted
surface correction but no admission, and the remaining candidates are rejected
or conditional.

## Scope implemented

- Local SQLite Registry and FTS5 index.
- Deterministic rebuild and malformed/conflicting manifest handling.
- Search and exact lookup with all WAVE-003 filters.
- Human/JSON CLI commands and expanded conformance tests.
- Release FTS5 CI target.
- Deterministic immutable Git source hashing and canonical subpath validation.
- Parallel real-resource research and independent admission/security review.
- Closure-oriented exact-manifest re-review for profiles 6, 9 and 10, an
  independent review of a replacement profile-20 MCP candidate, and Closure
  acceptance/materialization of profiles 6 and 9.
- Cumulative technical and practical workflow documentation.

## Files changed

See `HANDOFF_2026-08-08_WAVE-003.md` and the working-tree status. Principal new
files are the release FTS test, Git hash tool/tests/specification, project
workflow documents, seed source/admission/closure reports, Curator proposal
matrix, this report and the handoff.

## Architecture decisions made

ADR-020 governs agent-final authority. ADR-021 defines
`ossus-git-tree-v1`; the distinct Security Closure Agent accepted it after the
independent reviewer accepted the corrected NUL handling. The single permitted
Sol Medium advisory reconciled profiles 15/16 but did not approve either. No
taxonomy or later-WAVE behavior was added.

## Tests and commands

The final local baseline passed formatting, Clippy and 111 Rust workspace
tests. The CLI suite passed 24 tests; the release FTS5 target passed 1; the
Python hash suite passed 2. After Closure identified NUL handling, the focused
Rust regression passed 1 and the Python suite again passed 2. The two official
manifests pass schema validation and reindex to two resources; the staged
profile-10 draft remains review-free and excluded. Inventories were regenerated
with the repository generator; `./scripts/verify.sh` passes.

## Acceptance criteria

| Criterion | Evidence/state |
|---|---|
| Disposable deterministic index | Implemented and tested |
| Search does not read bodies | Implemented and tested |
| Exact/capability/category/FTS and filters | Implemented and tested |
| CLI human and JSON output | Implemented; expanded suite passes |
| Release FTS5 | Local release test passes; hosted matrix was triggered by the authorized push but is not observable without authenticated project access |
| Reconciled seed profile decisions | Met for this checkpoint; 20 governed dispositions and provisional target 16 |
| Admitted resources | 2 / 16 provisional admission-bearing slots; profiles 6 and 9 have Closure-approved manifests |
| Provenance/licenses/hashes/review | Profiles 6 and 9 have immutable MIT source locks, hashes, independent review and distinct Closure evidence; profile 10 remains blocked, 17/18 intentionally unresolved, and profile-20 replacement conditional |
| Full repository verification | Met locally; `./scripts/verify.sh` passes |

## Known limitations and deferred work

Profile 16 remains a valid but unfilled profile; the original profile-20
candidate is rejected and its replacement needs build/provenance and host-adapter
corrections. Profile 15 has an accepted explicit surface substitution but no
admission. Profile 10 requires an immutable read-only enforcement adapter.
Profiles 6 and 9 are admitted only as Agent Skills standard resources and do
not satisfy aggregate cross-host diversity. Hosted CI was triggered by the
authorized push but its results are not visible without authenticated project
access. All deferred work remains within WAVE-003; WAVE-004 is not authorized.

## Security and residual risk

The broad MCP source and unsafe dependency-audit candidates were rejected.
Conditional R3 resources require enforceable command, network, credential,
secret-redaction, origin and policy-precedence controls.

## Technical reader summary

Registry code is at a locally passing baseline. Admission and platform evidence,
not core indexing behavior, are the closure blockers.

## Practical plain-language summary

The search engine works and two carefully bounded resources are now in the
official catalog. The other candidates remain out until their source, risk and
host claims survive review; the project is not filling a number by relabeling
resources.

## Closure decision

- Decision: blocked from acceptance; WAVE remains in progress
- Evidence revision: 2026-08-08 reconciliation, Closure acceptance for profiles 6/9, and authorized push
- Independence attestation: implementer has not self-approved any resource;
  official Registry count is 2; the provisional target is 16 admission-bearing
  slots; profile 15 admission remains blocked,
  profiles 10, 17, 18, and 20 are intentionally unresolved, and no staged
  draft carries approval claims
- Latest handoff: `HANDOFF_2026-08-08_WAVE-003_POST_PUSH.md`
