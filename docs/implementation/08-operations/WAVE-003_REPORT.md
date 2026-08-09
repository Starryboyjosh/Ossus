# WAVE-003 implementation report

## Metadata

- WAVE: WAVE-003 — Seed Registry and local search
- Implementer: root Codex agent
- Independent reviewers: seed Admission Review Agents A/B; FTS/hash Security Review Agent
- Closure Agent: distinct WAVE-003 security Closure Agent for ADR-021/CI and seed decisions; no new admission acceptance was issued in the continuation batch
- Date: 2026-08-08
- Base commit: `ec9f1aa23aefa48f75a1db5396d232fd16bd02e0`
- Final working tree state: dirty, preserved, uncommitted and unpushed

## Objective completed

Partially. Registry/search/CLI implementation is complete locally. The WAVE is
not complete because no resource is admitted and hosted platform FTS5 evidence
is absent. The profile reconciliation keeps 20 governed profile decisions but
sets a provisional 16 admission-bearing target; profiles 10, 17, 18, and 20
are intentionally unresolved. Profile 15 has an accepted surface correction
but no admission, profiles 6/9 have proposed standard-only substitutions, and
the current candidates for the remaining profiles are rejected or conditional.

## Scope implemented

- Local SQLite Registry and FTS5 index.
- Deterministic rebuild and malformed/conflicting manifest handling.
- Search and exact lookup with all WAVE-003 filters.
- Human/JSON CLI commands and expanded conformance tests.
- Release FTS5 CI target.
- Deterministic immutable Git source hashing and canonical subpath validation.
- Parallel real-resource research and independent admission/security review.
- Closure-oriented exact-manifest re-review for profiles 6, 9 and 10, plus an
  independent review of a replacement profile-20 MCP candidate.
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
Rust regression passed 1 and the Python suite again passed 2. The three staged
manifest drafts passed schema validation when review fields were supplied;
they intentionally omit review fields now because admission review was
conditional and no new Closure acceptance was issued. Inventories were
regenerated with the repository generator; `./scripts/verify.sh` passes.

## Acceptance criteria

| Criterion | Evidence/state |
|---|---|
| Disposable deterministic index | Implemented and tested |
| Search does not read bodies | Implemented and tested |
| Exact/capability/category/FTS and filters | Implemented and tested |
| CLI human and JSON output | Implemented; expanded suite passes |
| Release FTS5 | Local release test passes; hosted matrix pending |
| Twenty admitted resources | Not met; official count remains zero |
| Provenance/licenses/hashes/review | Interim evidence and Curator proposals recorded; exact re-review blocks 2/3/6/9/10 on surfaces/adapter controls, 17/18 remain intentionally unresolved pending complete evidence, and profile-20 replacement remains conditional; no official manifest |
| Full repository verification | Met locally; `./scripts/verify.sh` passes |

## Known limitations and deferred work

Profile 16 remains unresolved; the original profile-20 candidate is rejected
and its replacement needs build/provenance and host-adapter corrections. Profile
15 has only a conditional explicit surface substitution. Profiles 6 and 9 may
be viable only after Closure records standard-only profile substitutions; profile
10 requires an immutable read-only enforcement adapter. Official manifests must
not be added until Closure. Hosted CI requires a future authorized push. All
deferred work remains within WAVE-003; WAVE-004 is not authorized.

## Security and residual risk

The broad MCP source and unsafe dependency-audit candidates were rejected.
Conditional R3 resources require enforceable command, network, credential,
secret-redaction, origin and policy-precedence controls.

## Technical reader summary

Registry code is at a locally passing baseline. Admission and platform evidence,
not core indexing behavior, are the closure blockers.

## Practical plain-language summary

The search engine works, but the official catalog is still empty because the
project refused to label questionable resources as approved merely to reach
twenty.

## Closure decision

- Decision: blocked from acceptance; WAVE remains in progress
- Evidence revision: 2026-08-08 closure-oriented continuation and interrupted-work handoff
- Independence attestation: implementer has not self-approved any resource;
  official Registry count remains 0; the provisional target is 16 admission-
  bearing slots; profile 15 admission remains blocked,
  profiles 10, 17, 18, and 20 are intentionally unresolved, and no staged
  draft carries approval claims
- Latest handoff: `HANDOFF_2026-08-08_WAVE-003_CLOSURE_PUSH.md`
