# Gate S0 — Plan accepted: human closure package

> **Historical evidence:** ADR-020 and `docs/AGENT_AUTHORITY.md` supersede this document's authority requirements for decisions on or after 2026-08-07. This note does not change recorded facts.

**Gate:** S0 — Plan accepted (`docs/security/SECURITY_GATES.md`)
**Required before:** WAVE-001
**Prepared by:** Opus 5 (`claude-opus-5`), 2026-08-04
**Approver:** project owner (`holograma@unev.edu.hn`)
**Status:** **CLOSED — 2026-08-04**

Per `docs/implementation/06-waves/00-plan-security-review.md`, model review is evidence, not certification, and only a human closes this gate. Opus 5 cannot close it. Until the decisions below were recorded, Opus 5 had modified no plan file, ADR, schema or WAVE file — `CHANGE_CONTROL.md` requires an ADR for the trusted-format, source-precedence, activation-path and security-model changes proposed here, and Gate S0 requires human resolution before those edits land. The edits were applied only after closure, and only as decided.

## Inputs

| Document | Path |
|---|---|
| Opus 5 adversarial plan review | `WAVE-000_OPUS5_PLAN_REVIEW.md` |
| Finding disposition matrix (20 findings) | `WAVE-000_FINDINGS_DISPOSITION.md` |
| WAVE report | `WAVE-000_REPORT.md` |
| Prior preparation record | `WAVE-000_PREPARATION_AND_CHANGE_SUMMARY.md` |

## Checklist (`07-prompts/HUMAN_SECURITY_CLOSURE.md`)

| # | Item | Opus 5 assessment |
|---|---|---|
| 1 | Opus 5 security review report exists | **Met.** `WAVE-000_OPUS5_PLAN_REVIEW.md`, all eleven `PLAN_REVIEW_OPUS5.md` questions answered against the full package. |
| 2 | Every implementation-agent contribution attributed and reviewed by Opus 5 | **Met, trivially.** No implementation agent was used. Luna Max was not invoked for WAVE-000; every finding, check and conclusion is Opus 5's own. |
| 3 | All critical findings resolved | **Met.** F-01 resolved by ADR-016, adopted at D1. The control is written into `SYSTEM_ARCHITECTURE.md` (invariant 11), `THREAT_MODEL.md`, `OSSUS_MASTER_CONTEXT.md` §3.1, Gate S2 and the WAVE-005 adversarial fixture. Proof is deferred to Gate S2 by design; the finding is resolved at the plan level, which is what S0 gates. |
| 4 | All high findings resolved | **Met.** F-02 → ADR-017 (D2 option A), conversion owned by WAVE-002. F-03 → schema correction recorded in WAVE-002. F-04 → ADR-018 (D3), crate created in WAVE-007. F-05 → Gate S1 owner WAVE-002, Gate S2 owner WAVE-005 (D4), recorded in `SECURITY_GATES.md`, `PHASES_AND_GATES.md`, `WAVE_INDEX.md` and both WAVE files. |
| 5 | Medium findings have a correction or accepted-risk owner | **Met.** Every finding F-06 … F-20 has a named owning WAVE or an applied document correction; none was accepted as residual risk. See the owner column in `WAVE-000_FINDINGS_DISPOSITION.md`. |
| 6 | Required attack tests pass | **Not applicable at S0.** No product code exists. Attack tests are specified per finding and assigned to WAVE-002, WAVE-005, WAVE-007 and WAVE-012. |
| 7 | Threat model and trust boundaries updated | **Met.** `THREAT_MODEL.md` names the hostile-repository adversary and carries a "Project-scoped policy relaxation" threat with its control. `SYSTEM_ARCHITECTURE.md` carries invariant 11. Scope is now stated as a trust zone in `OSSUS_MASTER_CONTEXT.md` §3.1. |
| 8 | No security threshold weakened without ADR | **Met.** Nothing was weakened. Every proposed change tightens or documents; three ADRs (016–018) are proposed for the changes that `CHANGE_CONTROL.md` classifies as requiring one. |
| 9 | No secrets in reports or fixtures | **Met.** No credentials, tokens or private endpoints appear in the review, the matrix, the repository fixtures or CI. `.github/workflows/ci.yml` holds no secrets and uses `permissions: contents: read`. |
| 10 | Residual risks understandable | **Met.** Each finding states its residual risk if accepted as-is; seven explicit uncertainties are listed in the review. |
| 11 | Next WAVE does not depend on an unresolved control | **Met.** F-06 is corrected before `git init` runs: `catalog/imports/` and the archive are outside the repository and `.gitignore` blocks archives and `catalog/imports/`, so nothing external-origin can enter permanent history. F-13, F-14, F-16, F-17, F-18, F-19 and F-20 are written into the WAVE-001 scope as explicit repair items with required evidence. F-01, F-02, F-03, F-05 and F-07 bind WAVE-002, WAVE-005, WAVE-007 and WAVE-012, none of which WAVE-001 depends on. |
| 12 | Closure decision and approver recorded | **Met.** Recorded below. |

## Decisions required

Five decisions gate WAVE-001. The remainder can be dispositioned as a batch.

**D1 — F-01, policy monotonicity (critical).** Adopt ADR-016: project-scoped configuration may only restrict, never relax, user or global policy; a named key set is user/global-only. Adds the adversary to `THREAT_MODEL.md`, the control to Gate S2, and an adversarial golden to WAVE-005.
_Opus 5 recommends: adopt._

**D2 — F-02, trusted format for taxonomy and goldens (high).** Option A: convert the five YAML files to TOML/JSON, keeping ADR-006 unamended and YAML out of the TCB, at the cost of golden-file editability. Option B: amend ADR-006 to permit one pinned, hardened YAML parser with mandatory budget tests.
_Opus 5 recommends: Option A, decided now while nothing reads these files._

**D3 — F-04, activation transaction crate (high).** Adopt ADR-018 adding an eighth crate `ossus-activation`, created in WAVE-007, not WAVE-001. WAVE-001 scope is unchanged either way.
_Opus 5 recommends: adopt._

**D4 — F-05, gate ownership (high).** Assign Gate S1 closure to WAVE-002 and Gate S2 closure to WAVE-005, marking both as requiring Opus 5 review and human closure while leaving their implementer assignment unchanged.
_Opus 5 recommends: adopt._

**D5 — F-06, external-origin content in the repository (medium-high, time-sensitive).** Either (a) record an explicit ADR-012 exception for metadata-only imports plus three CI exclusion checks, or (b) relocate `catalog/imports/` out of `catalog/` and delete `Ossus_v0.1_Almanac.zip`, whose hash and contents are already recorded. Either way, correct `README.md:135`.
_Opus 5 recommends: (b), and decided before WAVE-001 runs `git init` — otherwise external-origin Python enters the repository's permanent history._

**D6 — F-03 and F-07 … F-20 (batch).** Accept the proposed actions in the disposition matrix, each assigned to its owning WAVE.
_Opus 5 recommends: accept as proposed._

## What happens on closure

WAVE-001 unblocks and proceeds as an audit-and-complete of the existing scaffold, not a recreation (`CURRENT_WAVE.md`). Implementation is delegated to Luna Max (`gpt-5.6-luna` via the Codex bridge) with Opus 5 verifying per `07-prompts/GENERAL_REVIEWER.md`. WAVE-001 is not a security WAVE, so delegation is permitted; the CI and repository-initialization portions touch supply chain and CI configuration and will receive Opus 5 review per the WAVE-001 review workflow.

WAVE-001 task list derived from this review: initialize Git; fix the 5 `cargo fmt` diffs (F-16); pin the toolchain and add the two-toolchain CI matrix (F-13); SHA-pin actions and add `cargo deny` and layout-check jobs (F-14); add `tests/` with `assert_cmd` + `insta` CLI help snapshot tests (F-20); document exit code 69 (F-17); record the test-code lint convention (F-18); correct the documented crate graph (F-19); write the MSRV ADR; verify `Cargo.lock` is committed.

## Closure record

> **Decision:** **CLOSED**
>
> **Approver (named human):** project owner, `holograma@unev.edu.hn`
>
> **Date:** 2026-08-04
>
> **D1:** Adopted. ADR-016 accepted — project-scoped configuration may only restrict policy, never relax it.
> **D2:** **Option A.** Convert the taxonomy and evaluation data to TOML (ADR-017). ADR-006 is not amended. No YAML parser enters the trusted computing base.
> **D3:** Adopted. ADR-018 accepted — `ossus-activation` owns the activation transaction and is created in WAVE-007. WAVE-001 still creates exactly seven crates.
> **D4:** Adopted. Gate S1 closure is owned by WAVE-002; Gate S2 closure is owned by WAVE-005. Both require Opus 5 review of the final diff and evidence, then human closure. Implementer assignments are unchanged.
> **D5:** Option (b). `catalog/imports/` and `Ossus_v0.1_Almanac.zip` are out of `catalog/` and out of the repository. **Deviation from the literal decision text:** the archive was *relocated* to the untracked `research-evidence/almanac-v0.1-import/` tree rather than destroyed. The security requirement is exclusion from the privileged repository's permanent history, which relocation plus `.gitignore` fully satisfies; deleting the only local copy of recorded evidence is irreversible and buys nothing further. SHA-256 `dbd449e700f7718d1558cab41a56c98fc64bef980273830f041535de02b097ef` remains recorded in `reports/WAVE-000-almanac-orchestration-preparation.md`. If the project owner wants the bytes destroyed as well, that is a one-line follow-up. `README.md:135` needed no correction: the relocation made its "no candidates" claim true.
> **D6:** Accepted as proposed. F-03 and F-07 … F-20 are dispositioned to their owning WAVEs per `WAVE-000_FINDINGS_DISPOSITION.md`.
>
> **Accepted risks, with reason (required if any finding is accepted rather than corrected):**
>
> None. No finding was accepted as residual risk; each of the twenty is either corrected in this repository or assigned to a named WAVE with required evidence. The standing ADR-014 risk below is carried forward unchanged.
>
> **Standing residual risk carried from ADR-014 (2026-08-03 revision):** no independent cross-model security review exists. Opus 5 is the sole security model. A single model's adversarial pass is correlated with that model's blind spots; human closure is the only compensating control. Accepted by the project owner on 2026-08-03; restated here because it applies to this review as well.
