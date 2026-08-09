# Ossus WAVE-003 final coverage authority handoff — 2026-08-09

## State

`WAVE-003 — IN PROGRESS`

WAVE-004 remains unauthorized. Registry mechanics, local search, release FTS5,
and hosted platform validation are complete and green. The official Registry
contains **2** resources:

- Profile 6 — `mohitagw.technical-spec-template`
- Profile 9 — `mohitagw.database-schema-design`

Registry fingerprint: `fnv1a64:dbada94391f09954`.

## Coverage authority

The coverage pass keeps **20 original governed profiles**. It recommends a
provisional planning set of 16 current admission-bearing responsibilities:

- `SEED_REQUIRED`: 1, 2, 3, 4, 8, 13, 14, 16, 19 (9)
- `SEED_REQUIRED_WITH_SUBSTITUTION`: 5, 6, 7, 9, 11, 12, 15 (7)
- `INTENTIONALLY_UNRESOLVED`: 10, 17, 18, 20 (4)
- `REDUNDANT_FOR_SEED`: none
- `INVALID_ORIGINAL_PROFILE`: none

The 16 figure is **provisional planning evidence only**. The Closure Agent
accepted it as a planning denominator, but did not authorize replacing the
active WAVE-003 completion obligation of 20 real admitted seeds. That change
would require an explicit architecture decision/ADR. No profile was removed
because a candidate was hard to find.

The final authority record is
`WAVE-003_FINAL_COVERAGE_AUTHORITY.md`. It explains the required capability,
resource-type, surface, risk and overlap dimensions, the 17 golden capabilities
outside the 20 profiles, and the separation between real catalog entries and
synthetic negative/adversarial fixtures.

## Amendment and admission decisions

Independent Review Agent A found that a blanket R3 amendment was not justified.
Distinct Closure record `closure/wave003-profile-amendments-5-7-11-12-20260809`
accepted profile-only ceilings:

| Profile | Accepted ceiling | Candidate status |
|---:|---|---|
| 5 visual/browser validation | R2 → R3 | blocked; no admission |
| 7 backend API | R1 → R2 | blocked; no admission |
| 11 unit testing | R0 → R2 | blocked; no admission |
| 12 integration testing | R1 → R3 | blocked; no admission |

The amendment review and Closure decision do not approve a resource, adapter,
host activation, or Registry manifest. Profile 15's accepted surface
substitution is also only a profile decision; its candidate is
**USEFUL_BUT_DEFERRED** pending a dependency-only adapter, advisory freshness,
mandatory redaction, and source-scope closure. Profile 16 remains valid and
seed-required but unfilled. Profiles 10, 17, 18 and 20 remain intentionally
unresolved; do not reopen them without new coverage evidence.

## Actual remaining gaps

The current catalog has only R0 standard prompt-packs. It has no admitted
skill, MCP server, Claude/Codex/standalone-CLI/generic-MCP surface, R1/R2/R3
resource, cross-host resource, or overlapping competitor. It covers only three
of the 24 profile capabilities. These are positive seed gaps; synthetic
fixtures cannot replace the required real provenance and Closure evidence.

The frozen 50 goldens require 41 capabilities, 17 of which are not represented
by a seed profile. This is evaluation-scope debt to reconcile before Layer 2
exact-resource goldens; it is not permission to stuff unrelated capabilities
into current manifests or weaken policy. Negative/risk/overlap/no-candidate
behavior can use synthetic fixtures.

## Verification and hosted evidence

Local baseline remains green:

- 111 workspace tests;
- 24 CLI tests;
- 1 release FTS5 test;
- 2 Git hashing tests;
- formatting, Clippy (`-D warnings`), inventories, layout, and
  `./scripts/verify.sh`.

Hosted CI run **22**, commit `f6e58a0`, passed the release FTS5 and workspace
checks on:

- Ubuntu native runner — PASS;
- Arch Linux userspace container on Ubuntu runner — PASS (not native Arch);
- macOS native runner — PASS;
- Windows native runner — PASS.

## Agent usage

This continuation used Luna Max workers for coverage, amendment, blocker and
evidence analysis. No Sol Medium call was made in this continuation. A previous
continuation used one advisory Sol Medium call for the profile-15/profile-16
reconciliation; it did not approve a candidate or WAVE. The exact historical
record remains in `WAVE-003_REPORT.md` and `WAVE-003_SEED_CLOSURE_2026-08-08.md`.

## Next actions

1. Do not start WAVE-004.
2. Use targeted research only for genuine positive gaps, starting with clean
   candidates that can provide real skill/surface/risk/category diversity.
3. For each candidate, preserve Curator → independent Admission Review →
   Closure, then create and validate a canonical manifest incrementally.
4. Do not claim WAVE-003 complete until the active 20-entry obligation is
   either satisfied or changed by an explicit architecture decision, and the
   official Registry has sufficient real diversity for the next stage.
5. Keep hosted CI evidence and the Arch-container limitation visible in every
   subsequent report.
