# Ossus WAVE-003 final-admission sprint handoff — 2026-08-08

## Current state

`WAVE-003 — IN PROGRESS`

Registry mechanics are complete and locally verified. The original seed target
was 20; profile reconciliation produced a **provisional 16 admission-bearing
target** and four intentionally unresolved profiles (10, 17, 18 and 20). The
official Registry contains **2 / 16** resources:

- Profile 6 — `mohitagw.technical-spec-template`
- Profile 9 — `mohitagw.database-schema-design`

No additional candidate survived the bounded final-admission sprint. WAVE-004
remains unauthorized.

## Work completed in this sprint

- Performed bounded Luna Max review instead of another broad discovery sweep.
- Profile 15's accepted `standalone-cli` → `agent-skills-standard` substitution
  received independent review `admission-review-a/profile-15-dependency-audit-20260808`.
  The source tuple, MIT license and static tree passed, but the candidate is
  **BLOCKED — NOT READY FOR CLOSURE** because no dependency-only adapter,
  freshness protocol or mandatory secret/private-registry redaction evidence
  exists. No manifest was created.
- Created Curator-only amendment packets for profiles 5, 7, 11 and 12 in
  `WAVE-003_PROFILE_AMENDMENT_DECISIONS.md`. Each proposes an explicit R3
  contract for intrinsic browser, command, write or test-runner behavior. None
  changes taxonomy, approves a candidate, or enters the Registry.
- Confirmed profiles 1, 3, 4 and 14 remain valid profiles with rejected current
  candidates. Profile 16 remains valid and intentionally unfilled after one
  bounded replacement triage; the alternative failed capability, scope and
  R1-boundary checks. Profiles 8, 13, 17, 18 and 20 retain their prior governed
  rejection/unresolved decisions.
- Recorded the temporary `Governance Lessons from Seed Admission` section in
  `WAVE-003_REPORT.md`, including the no-quota/catalog-pressure invariant.

## Profile disposition summary

| Disposition | Profiles |
|---|---|
| Unchanged | 2 |
| Profile substitution proposed | 5, 7, 11, 12 |
| Profile substitution accepted | 6, 9, 15 |
| Candidate rejected; profile valid | 1, 3, 4, 8, 13, 14, 16, 19 |
| Intentionally unresolved | 10, 17, 18, 20 |

Profile substitution is not admission. Profiles 6 and 9 are the only official
entries. Profile 15 is accepted at the profile-design level but its candidate
is not ready for Closure.

## Verification baseline

Previously verified and preserved by this sprint:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features` — 111 tests
- `cargo test -p ossus --test cli_snapshots` — 24 tests
- `cargo test -p ossus-registry --release --test release_fts5 --locked` — 1 pass
- `python3 -W error::ResourceWarning scripts/test-hash-git-resource.py` — 2 pass
- `./scripts/verify.sh` — pass; cargo-deny emitted only existing unmatched
  allowed-license warnings
- Official schema validation, exact lookup, filters, FTS search, conflict and
  deterministic reindex checks — pass

Official Registry reindex remains:

```text
indexed: 2
excluded: 0
fts5: available
fingerprint: fnv1a64:dbada94391f09954
```

The deterministic SQLite rebuild hash remains
`453d695e646ffb50dea903abe98f7211d96be8c532f18b03b916027a56f28e36`.

## Hosted FTS5

The repository matrix targets Ubuntu, macOS and Windows with pinned Rust
`1.97.1` and the release FTS5 test. CI workflow run **14** (`31294757281`) for
commit `aac60826b3f8c69a5a35c3cb3e3ab12270718a74` reported:

| Platform | Job | Release FTS5 |
|---|---:|---|
| Ubuntu | `93197926838` | PASS |
| macOS | `93197926836` | PASS |
| Windows | `93197926835` | PASS |

Cargo deny, pinned format/Clippy, floating advisory checks and layout all
passed. The inventory correction fixed the checkout-EOL mismatch without
modifying the inherited PowerShell file. Hosted FTS5 and the complete CI
workflow are now green on run **16** (`31295109423`):

| Platform | Job | Release FTS5 |
|---|---:|---|
| Ubuntu | `93198830494` | PASS |
| macOS | `93198830507` | PASS |
| Windows | `93198830495` | PASS |

Layout job `93198830448`, pinned quality `93198830455`, advisory
`93198830478`, and cargo-deny `93198830480` also passed.

## Git state and next actions

Branch: `main`  
Remote: `origin git@github.com:Starryboyjosh/Ossus.git`  
Last pushed checkpoint before this hosted-evidence documentation update:
`c91d3a8cc9c1a1fc19fe1d9766efe25e7d97f965`.

The current documentation changes should be committed coherently, inventories
regenerated through `scripts/generate-repository-inventories.py`, and pushed
normally. Do not force-push or rewrite inherited history. The public Actions
API exposed run and job conclusions, while job logs remain admin-authenticated.

## Remaining blockers

1. Complete only those additional admissions whose exact source, risk, surface,
   runtime, license and role-separated evidence survives independent review and
   Closure.
2. Decide the final seed denominator through architecture authority; do not
   reduce 16 merely because candidates are difficult to find.
3. Resolve profile 15's adapter, freshness and redaction blockers or leave the
   accepted substitution unfilled.
4. Keep profiles 10, 16, 17, 18 and 20 fail-closed until their specific evidence
   blockers are repaired.
5. Keep hosted verification recorded at run 16; no CI blocker remains. Any
   future change must preserve the Git-index-byte inventory behavior.

The Closure Agent retains final authority under ADR-020. WAVE-004 must not be
started from this handoff.
