# WAVE-003 implementation report

## Metadata

- WAVE: WAVE-003 — Seed Registry and local search
- Implementer: root Codex agent
- Independent reviewers: seed Admission Review Agents A/B; FTS/hash Security Review Agent
- Closure Agent: distinct WAVE-003 security Closure Agent for ADR-021/CI and seed decisions; profiles 6 and 9 were accepted and admitted in the continuation batch, followed by the separately closed profile-2 admission on 2026-08-16
- Date: 2026-08-08
- Base commit: `ec9f1aa23aefa48f75a1db5396d232fd16bd02e0`
- Checkpoint commits: `4cc66c5` (mechanics), `0cb3987` (governance/evidence),
  `17d6e0d` (admitted manifests), `88b19b1` (post-push handoff), `3421b64`
  (checkpoint clarification), `603159f` (generated inventories), `aac6082`
  (final-admission sprint), `eb140f3` (hosted evidence), and `c91d3a8`
  (cross-platform inventory fix), `59bbccd` (hosted closure documentation),
  and `eecd2f5` (hosted verification checkpoint); pushed to `origin/main`

## Objective completed

Partially. Registry/search/CLI implementation is complete locally. Three R0
standard-only resources are admitted; profile 2 was added in the targeted
2026-08-16 admission continuation. Hosted release FTS5 evidence is now observed
for Ubuntu, macOS and Windows on CI run `31294757281` (workflow CI, run 14,
commit `aac60826b3f8c69a5a35c3cb3e3ab12270718a74`). That run exposed a
checkout-filter mismatch in the separate layout job: `.gitattributes`
materialized `scripts/verify.ps1` as CRLF while the inventory hashed LF
working-tree bytes. The inventory generator now hashes Git index/blob bytes,
which are platform-independent. Follow-up CI run **16** (`31295109423`) passed
the complete workflow on commit `c91d3a8cc9c1a1fc19fe1d9766efe25e7d97f965`.
The profile reconciliation keeps 20 governed profile decisions but sets
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
  acceptance/materialization of profiles 6 and 9; the later targeted
  continuation accepted profile 2 as a standard-only skill.
- Cumulative technical and practical workflow documentation.
- Arch Linux userspace CI validation lane with independent review, distinct
  Closure acceptance and hosted evidence on CI run 19.
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
taxonomy or later-WAVE behavior was added. The Arch CI trust-boundary change
was independently reviewed by `/root/seed_admission_review_b` and accepted by
`/root/wave003_security_closure` under
`closure/wave003-arch-container-ci-20260809`; that decision is limited to the
exact workflow diff and does not close WAVE-003.

## Tests and commands

The final local baseline passed formatting, Clippy and 111 Rust workspace
tests. The CLI suite passed 24 tests; the release FTS5 target passed 1; the
Python hash suite passed 2. After Closure identified NUL handling, the focused
Rust regression passed 1 and the Python suite again passed 2. The three official
manifests pass schema validation and the disposable Registry reindexes to three
resources with no exclusions; the staged
profile-10 draft remains review-free and excluded. Inventories were regenerated
with the repository generator; `./scripts/verify.sh` passes. The hardened local
Arch Linux userspace reproduction (immutable image index, read-only checkout,
isolated target directory, Rust `1.97.1`) also passed the 111-test workspace
suite and the release FTS5 test.

## Acceptance criteria

| Criterion | Evidence/state |
|---|---|
| Disposable deterministic index | Implemented and tested |
| Search does not read bodies | Implemented and tested |
| Exact/capability/category/FTS and filters | Implemented and tested |
| CLI human and JSON output | Implemented; expanded suite passes |
| Release FTS5 | Local release test passes. Hosted CI run `31298472061`/19 passed the pinned release test on Ubuntu job `93207265221`, macOS job `93207265246`, Windows job `93207265192`, and Arch-container job `93207265220` |
| Reconciled seed profile decisions | Met for this checkpoint; 20 governed dispositions and provisional target 16 |
| Admitted resources | 3 / 16 provisional admission-bearing slots; profiles 2, 6 and 9 have Closure-approved manifests; profile 2 is standard-only |
| Provenance/licenses/hashes/review | Profiles 2, 6 and 9 have immutable MIT source locks, hashes, independent review and distinct Closure evidence; profile 10 remains blocked, 17/18 intentionally unresolved, and profile-20 replacement conditional |
| Full repository verification | Met locally; `./scripts/verify.sh` passes; hosted CI run 19 is fully green |

## Hosted CI evidence

The pushed checkpoint `c91d3a8cc9c1a1fc19fe1d9766efe25e7d97f965` triggered
workflow **CI**, run **16** (`31295109423`). The pinned matrix for that run ran
`cargo +1.97.1 test -p ossus-registry --release --test release_fts5 --locked`
and completed successfully on every platform declared in that run:

| Platform | Job | Result |
|---|---:|---|
| Ubuntu | `93198830494` | PASS — release FTS5 |
| macOS | `93198830507` | PASS — release FTS5 |
| Windows | `93198830495` | PASS — release FTS5 |

CI run **19** (`31298472061`) for commit
`65b79e1e21d96f406e099bfcd98b551c4f6198a7` passed every job. The Arch job
(`93207265220`) ran the workspace suite and release FTS5 test inside the
immutable Arch image index below on an Ubuntu-hosted Linux/amd64 runner. The
workflow logs retain the x86_64 assertion, package/toolchain output and both
locked test commands; unauthenticated API access in this environment exposes
job conclusions but not downloadable logs.

| Platform | Environment | Image / toolchain | Release FTS5 |
|---|---|---|---|
| Ubuntu | GitHub-hosted native runner; job [`93207265221`](https://github.com/Starryboyjosh/Ossus/actions/runs/31298472061/job/93207265221) | Rust `1.97.1`; bundled SQLite | PASS — run 19 |
| Arch Linux | Arch userspace in Ubuntu-hosted Docker container (`linux/amd64`); job [`93207265220`](https://github.com/Starryboyjosh/Ossus/actions/runs/31298472061/job/93207265220) | `archlinux:base-devel@sha256:c1829f370be8434135f43fb3acaef1256780804ac3b2d2eec90dfb1232e1ffdf`; resolved amd64 child `sha256:fae033b815a16f930325c2697e620362be4d2e5d739a301b10ad1fc9c8643a06`; Rust `1.97.1`; bundled SQLite | PASS — run 19; containerized userspace, not native Arch |
| macOS | GitHub-hosted native runner; job [`93207265246`](https://github.com/Starryboyjosh/Ossus/actions/runs/31298472061/job/93207265246) | Rust `1.97.1`; bundled SQLite | PASS — run 19 |
| Windows | GitHub-hosted native runner; job [`93207265192`](https://github.com/Starryboyjosh/Ossus/actions/runs/31298472061/job/93207265192) | Rust `1.97.1`; bundled SQLite | PASS — run 19 |

The complete run is recorded at
https://github.com/Starryboyjosh/Ossus/actions/runs/31298472061. The quality,
advisory, cargo-deny and layout jobs also passed; the inventory correction
continues to resolve the checkout-EOL mismatch without modifying the inherited
PowerShell file.

## Known limitations and deferred work

Profile 16 remains a valid but unfilled profile; the original profile-20
candidate is rejected and its replacement needs build/provenance and host-adapter
corrections. Profile 15 has an accepted explicit surface substitution but no
admission. Profile 10 requires an immutable read-only enforcement adapter.
Profiles 2, 6 and 9 are admitted only as Agent Skills standard resources and do
not satisfy aggregate cross-host diversity. Hosted Ubuntu/macOS/Windows and
Arch-container FTS5 plus the complete CI run 19 are green. Remaining blockers
are seed admissions and profile governance. All deferred work remains within
WAVE-003; WAVE-004 is not authorized.

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
- Profiles 2, 6, 9 and 15 showed a surface mistake. A standard `SKILL.md` package
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

Registry code is at a locally passing baseline. Admission and profile governance,
not core indexing or platform validation, are the closure blockers.

## Practical plain-language summary

The search engine works and three carefully bounded resources are now in the
official catalog. The other candidates remain out until their source, risk and
host claims survive review; the project is not filling a number by relabeling
resources.

## Closure decision

- Decision: blocked from acceptance; WAVE remains in progress
- Evidence revision: 2026-08-08 reconciliation, Closure acceptance for profiles 6/9, the 2026-08-16 profile-2 Closure, Arch CI Closure, and hosted CI run 19
- Independence attestation: implementer has not self-approved any resource;
  official Registry count is 3; the provisional target is 16 admission-bearing
  slots; profile 15 admission remains blocked after independent review,
  profiles 10, 17, 18, and 20 are intentionally unresolved, and no staged
  draft carries approval claims
- Latest handoff: `HANDOFF_2026-08-08_WAVE-003_ARCH_COVERAGE.md`

## Final coverage authority pass — 2026-08-09

The coverage audit separated profile correctness, candidate correctness and
admission. It confirms the following governed denominator:

| Classification | Profiles | Count |
|---|---|---:|
| `SEED_REQUIRED` | 1, 2, 3, 4, 8, 13, 14, 16, 19 | 9 |
| `SEED_REQUIRED_WITH_SUBSTITUTION` | 5, 6, 7, 9, 11, 12, 15 | 7 |
| `INTENTIONALLY_UNRESOLVED` | 10, 17, 18, 20 | 4 |
| `REDUNDANT_FOR_SEED` | none | 0 |
| `INVALID_ORIGINAL_PROFILE` | none | 0 |

The resulting **provisional planning denominator is 16**, while the original
governed WAVE completion obligation remains 20. The four intentionally unresolved profiles
are useful future coverage, not discarded profiles, and they do not create
quota pressure. The profile-level accepted substitutions for 6, 9 and 15 do
not admit candidates; profiles 2, 6 and 9 now have official manifests.

Independent Review Agent A narrowed the amendment recommendations: profile 5
is R2→R3, profile 7 is R1→R2, profile 11 is R0→R2, and profile 12 is R1→R3.
Closure record `closure/wave003-profile-amendments-5-7-11-12-20260809`
accepted those profile-only corrections; no candidate, host adapter, or
manifest was approved. Profile 15's candidate is **USEFUL_BUT_DEFERRED**
pending an immutable dependency-only adapter, advisory freshness, strict
redaction and source-scope closure.

The final authority document records the dimensions the seed Registry must
prove. At the 2026-08-09 coverage pass, the official catalog had only two R0
Agent Skills-standard prompt-packs. The subsequent profile-2 admission added
one R0 standard-only skill, but the catalog still has no admitted MCP server,
Claude/Codex/standalone-CLI/generic-MCP surface, R1/R2/R3 resource, cross-host
resource, or overlapping competitor. Synthetic fixtures
may cover negative, adversarial and no-candidate behavior, but cannot replace
real provenance and Closure evidence for exact-resource seed coverage.

Accordingly the final coverage decision remains:

```text
WAVE-003 — IN PROGRESS
WAVE-004 AUTHORIZATION RECOMMENDED: NO
```

The authoritative coverage record is
`WAVE-003_FINAL_COVERAGE_AUTHORITY.md`; the amendment review addendum is in
`WAVE-003_PROFILE_AMENDMENT_DECISIONS.md`. No Registry mechanics or hosted
infrastructure were changed by this pass.

The independent WAVE acceptance review is
`WAVE-003_INDEPENDENT_ACCEPTANCE_2026-08-16.md`; its verdict is `BLOCKED`
because the active 20-entry obligation and required resource/surface/risk
diversity remain unmet.

The latest documented green hosted matrix is CI run **22** on commit
`f6e58a0`: Ubuntu native FTS5 PASS, Arch userspace-container FTS5 PASS (not
native Arch), macOS native FTS5 PASS, and Windows native FTS5 PASS. The run
also passed the workspace, quality, advisory, cargo-deny and layout jobs.

## Targeted admission continuation — 2026-08-16

After the 2026-08-09 coverage authority pass, the Curator prepared a bounded
proposal for profile 2: `wshobson.responsive-design`. The immutable source
tuple and `ossus-git-tree-v1` hash were independently verified. The selected
subtree contains five regular Markdown files, totals 43,958 bytes, and is
MIT-licensed; no executable files or runtime requirements were observed.

Independent Admission Review Agent
`admission-review-codex/wave003-responsive-design-20260816-r1` returned
`READY_FOR_CLOSURE` with no critical, high, or medium findings. The distinct
Closure Agent `closure-agent/wave003-responsive-design-20260816-c1` returned
`ACCEPTED` for a narrowly bounded R0 `skill` with `instruction-only` runtime,
`source-only` distribution, project scope, and `agent-skills-standard` only.
The `frontend.accessibility` mapping is limited to responsive interaction
semantics and is not a WCAG or screen-reader audit.

The accepted canonical manifest is
`catalog/official/manifests/wshobson.responsive-design.toml`; the full Closure
record is `WAVE-003_RESPONSIVE_DESIGN_CLOSURE_2026-08-16.md`. The disposable
Registry reindexed successfully with three resources, no exclusions, FTS5
available, and fingerprint `fnv1a64:5061c5129b71b19a`. The manifest validated;
the capability search and exact show command returned the new entry.

This admission increases the official count from two to three. It does not
prove Claude Code, Codex, standalone-CLI, or aggregate cross-host coverage and
does not close WAVE-003. The active 20-entry obligation, remaining diversity
gaps, and WAVE-004 prohibition are unchanged.
