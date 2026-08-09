# WAVE-003 implementation report

## Metadata

- WAVE: WAVE-003 — Seed Registry and local search
- Implementer: root Codex agent
- Independent reviewers: seed Admission Review Agents A/B; FTS/hash Security Review Agent
- Closure Agent: distinct WAVE-003 security Closure Agent for ADR-021/CI and seed decisions; profiles 6 and 9 were accepted and admitted in the continuation batch
- Date: 2026-08-08
- Base commit: `ec9f1aa23aefa48f75a1db5396d232fd16bd02e0`
- Checkpoint commits: `4cc66c5` (mechanics), `0cb3987` (governance/evidence),
  `17d6e0d` (admitted manifests), `88b19b1` (post-push handoff), `3421b64`
  (checkpoint clarification), `603159f` (generated inventories), `aac6082`
  (final-admission sprint), and `eb140f3` (hosted evidence); the
  cross-platform inventory fix is staged for the next pushed checkpoint

## Objective completed

Partially. Registry/search/CLI implementation is complete locally. Two R0
standard-only resources are admitted; no additional candidate survived the
bounded final-admission review. Hosted release FTS5 evidence is now observed
for Ubuntu, macOS and Windows on CI run `31294757281` (workflow CI, run 14,
commit `aac60826b3f8c69a5a35c3cb3e3ab12270718a74`). The same workflow's
separate layout job failed. A true fresh-clone reproduction identified the
cause: `.gitattributes` materializes `scripts/verify.ps1` as CRLF on hosted
checkouts, while the inventory had hashed LF working-tree bytes. The inventory
generator is now staged to hash Git index/blob bytes, which are
platform-independent; a follow-up run is required before treating hosted CI as
green. The profile reconciliation keeps 20 governed profile decisions but sets
a provisional 16 admission-bearing target;
profiles 10, 17, 18, and 20 are intentionally unresolved. Profile 15 has an
accepted surface correction, but its independent review is blocked on a bounded
adapter, freshness and redaction evidence. Profiles 5, 7, 11 and 12 have
explicit amendment packets, not approvals.

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
- Bounded final-admission sprint: profile-amendment decision packets for 5, 7,
  11 and 12; targeted rejection/unresolved confirmations for profiles 1, 3, 4,
  14 and 16; and an independent block for the profile-15 substitution.

## Files changed

See the latest handoff and repository status. Principal new files are the
release FTS test, Git hash tool/tests/specification, project workflow
documents, seed source/admission/closure reports, the profile-amendment packet,
Curator proposal matrix, this report and the handoff.

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
| Release FTS5 | Local release test passes. Hosted CI run `31294757281`/14 passed the pinned release test on Ubuntu job `93197926838`, macOS job `93197926836`, and Windows job `93197926835`; the separate layout job failed and needs a follow-up run |
| Reconciled seed profile decisions | Met for this checkpoint; 20 governed dispositions and provisional target 16 |
| Admitted resources | 2 / 16 provisional admission-bearing slots; profiles 6 and 9 have Closure-approved manifests; no new candidate reached Closure in the final sprint |
| Provenance/licenses/hashes/review | Profiles 6 and 9 have immutable MIT source locks, hashes, independent review and distinct Closure evidence; profile 10 remains blocked, 17/18 intentionally unresolved, and profile-20 replacement conditional |
| Full repository verification | Met locally; `./scripts/verify.sh` passes. Hosted layout discrepancy remains under investigation |

## Hosted CI evidence

The pushed checkpoint `aac60826b3f8c69a5a35c3cb3e3ab12270718a74` triggered
workflow **CI**, run **14** (`31294757281`). The pinned matrix ran
`cargo +1.97.1 test -p ossus-registry --release --test release_fts5 --locked`
and completed successfully on all supported platforms:

| Platform | Job | Result |
|---|---:|---|
| Ubuntu | `93197926838` | PASS — release FTS5 |
| macOS | `93197926836` | PASS — release FTS5 |
| Windows | `93197926835` | PASS — release FTS5 |

The quality, advisory and cargo-deny jobs also passed. The independent
`Repository layout invariants` job (`93197926856`) failed with exit code 1.
A true fresh clone reproduces the failure as a stale `REPOSITORY_MANIFEST.json`
entry for `scripts/verify.ps1`: the hosted checkout has 366 CRLF bytes while
the committed inventory recorded 354 LF bytes. The staged generator correction
reads canonical Git index blobs instead of checkout-filtered working-tree bytes;
it leaves the inherited PowerShell file untouched and preserves deterministic
hashes across platforms. A follow-up push is being used to verify the fix.

## Known limitations and deferred work

Profile 16 remains a valid but unfilled profile; the original profile-20
candidate is rejected and its replacement needs build/provenance and host-adapter
corrections. Profile 15 has an accepted explicit surface substitution but no
admission. Profile 10 requires an immutable read-only enforcement adapter.
Profiles 6 and 9 are admitted only as Agent Skills standard resources and do
not satisfy aggregate cross-host diversity. Hosted FTS5 is observed on all
three supported platforms, but the layout-job discrepancy still blocks a fully
green hosted checkpoint. All deferred work remains within WAVE-003; WAVE-004 is
not authorized.

## Security and residual risk

The broad MCP source and unsafe dependency-audit candidates were rejected.
Conditional R3 resources require enforceable command, network, credential,
secret-redaction, origin and policy-precedence controls.

## Governance Lessons from Seed Admission

This temporary WAVE-003 section records evidence for the post-WAVE governance
update. It is deliberately about admission governance, not an implementation of
the future Researcher.

- The original target of 20 became a provisional admission-bearing target of
  16 only after every profile received a governed disposition. Four profiles
  (10, 17, 18 and 20) are intentionally unresolved and must not create quota
  pressure. The denominator is still provisional until architecture records a
  final decision.
- Profiles 5, 7, 11 and 12 exposed specification mistakes: browser validation,
  API implementation, TDD, and integration testing are operationally R3
  activities, not passive R0/R1 resources. Their amendment packets make the
  risk delta explicit instead of laundering execution as a low-risk skill.
- Profiles 6, 9 and 15 showed a surface mistake. A standard `SKILL.md` package
  does not prove Claude Code, Codex, or CLI compatibility. Profiles 6 and 9
  were admitted only after narrowing to the Agent Skills standard; profile 15
  received the same profile-level correction but remains blocked because its
  dependency-audit capability needs an enforced adapter, freshness design and
  secret redaction.
- Useful niches were rejected when provenance, references, licensing, scope or
  risk failed: raw-secret/policy handling, incomplete referenced trees,
  composite licensing, unpinned MCP build inputs, and unbounded evaluation are
  admission failures rather than invitations to lower policy.
- Rejections improved catalog quality by preventing a generative UI source from
  filling an R1 performance slot, a repository-writing source from filling an
  R0 unit-testing slot, and an install/plugin scanner from filling bounded R1
  supply-chain review. Profile substitution was more correct than relabeling
  any of those candidates.
- The permanent invariant is: **catalog growth is an outcome of successful
  review, never a goal that overrides review. Discovery volume must never create
  admission pressure.**

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
  slots; profile 15 admission remains blocked after independent review,
  profiles 10, 17, 18, and 20 are intentionally unresolved, and no staged
  draft carries approval claims
- Latest handoff: `HANDOFF_2026-08-08_WAVE-003_FINAL_ADMISSION_SPRINT.md`
