# WAVE report — WAVE-000

> **Historical evidence:** ADR-020 and `docs/AGENT_AUTHORITY.md` supersede this document's authority requirements for decisions on or after 2026-08-07. This note does not change recorded facts.

## Metadata

- **WAVE:** WAVE-000 — Opus 5 security and architecture review
- **Implementer:** Opus 5, acting as `security_implementer` per `specs/config/model-roles.yaml`
- **Model:** `claude-opus-5`
- **Date:** 2026-08-04
- **Base commit:** none — the working tree is not a Git repository (`git rev-parse --is-inside-work-tree` → `fatal: not a git repository`)
- **Final working tree state:** three new documents under `docs/implementation/08-operations/`. No product code, no schema, no ADR, no WAVE file and no existing document was modified. No repository was created.
- **Implementation-agent contribution:** none. Luna Max was not invoked for this WAVE.

## Objective completed

Partially. The adversarial review is complete and every finding is dispositioned to the point where a human decision is the only remaining input. Gate S0 itself remains open, which is correct: `00-plan-security-review.md` states that a human closes the gate and that there is no automatic replacement, and Gate S0's acceptance criterion is human closure.

## Scope implemented

- Ran `07-prompts/PLAN_REVIEW_OPUS5.md` adversarially against the full package: `AGENTS.md`, `README.md`, all of `docs/product/`, `docs/architecture/`, `docs/security/`, `docs/roadmap/`, `docs/implementation/`, `specs/`, `.ossus/`, `evaluations/`, `catalog/`, the seven crates, and all build/CI configuration.
- Answered all eleven review questions with cited evidence.
- Produced 20 findings with severity, supporting and conflicting evidence, affected invariant, deterministic verification, proposed action and residual risk.
- Attributed contributions: none to attribute.
- Prepared the human Gate S0 closure package with six explicit decisions.

Not done, deliberately: **the plan was not edited.** `CHANGE_CONTROL.md` requires an ADR for the trusted-format (F-02), source-precedence (F-01), activation-path (F-04) and adapter-trust-claim changes proposed here, and Gate S0 requires human resolution before those edits land. Editing the plan first would invert the gate. Proposed edits are specified precisely enough to apply mechanically once decided.

## Files changed

| File | Change |
|---|---|
| `docs/implementation/08-operations/WAVE-000_OPUS5_PLAN_REVIEW.md` | new — review report, eleven answers, 20-finding index, uncertainties |
| `docs/implementation/08-operations/WAVE-000_FINDINGS_DISPOSITION.md` | new — nine-field disposition per finding |
| `docs/implementation/08-operations/WAVE-000_GATE_S0_CLOSURE.md` | new — human checklist, six decisions, closure record |
| `docs/implementation/08-operations/WAVE-000_REPORT.md` | new — this report |

## Architecture decisions

None taken. Three are **proposed** and await the human:

- **ADR-016 — Policy monotonicity for project-scoped configuration** (from F-01). Project-scoped configuration may only restrict, never relax, user or global policy; a named key set is user/global-only.
- **ADR-017 — Trusted format for taxonomy and evaluation data** (from F-02). Either convert the five YAML files to TOML/JSON, or amend ADR-006 to permit one pinned hardened YAML parser with mandatory budget tests.
- **ADR-018 — Activation transaction crate boundary** (from F-04). An eighth crate `ossus-activation`, created in WAVE-007, owns the host-neutral transaction; the adapter supplies a trait implementation and never owns the transaction.

## Tests and commands

| Command | Result |
|---|---|
| `git rev-parse --is-inside-work-tree` | FAIL — not a Git repository |
| `rustc --version` | `1.97.1 (8bab26f4f 2026-07-14)` |
| `cargo --version` | `1.97.1 (c980f4866 2026-06-30)` |
| `cargo fmt --all -- --check` | FAIL — 5 diff hunks (4 in `crates/ossus-cli/src/main.rs`, 1 in `crates/ossus-resolver/src/lib.rs:21`) |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --workspace --all-features` | PASS — 9 tests, 13 suites |
| `cargo run -q -p ossus -- status` | PASS |
| `scripts/verify.sh` | STOPPED at the fmt step |
| Doc mirror drift, 25 pairs | PASS — byte-identical |
| Capability count, `specs/taxonomy/capabilities-v1.yaml` | 44 — matches `README.md` |
| Golden count, `evaluations/goldens/goldens-v1.yaml` | 50 — matches `README.md` |
| `sha256sum Ossus_v0.1_Almanac.zip` | matches the recorded value |
| Rust references to `catalog/`, `specs/`, `evaluations/` | none |
| Schema enum and required-field extraction | complete; see review §4 and §6 |

No product tests were written; WAVE-000 forbids writing product code.

## Acceptance criteria

| Criterion | Status |
|---|---|
| Gate S0 is closed by a human | **Met** — closed 2026-08-04 by the project owner; see the closure record |
| No unresolved critical finding | **Met** — F-01 resolved by ADR-016, with proof assigned to Gate S2 |
| All changes are traceable | **Met** — four new files, nothing modified, every finding cites file and line |
| Every critical/high finding has a disposition | **Met** — proposed disposition and verification for all five |
| Trust boundaries and WAVE order agree | **Met** — F-05 corrected: Gate S1 owner WAVE-002, Gate S2 owner WAVE-005, recorded in four places |
| Model assignments and attribution explicit | **Met** — Opus 5 sole author, recorded in all four documents |

The first, second and fifth rows read **Open**, **Open** and **Not met** until closure; they are updated here because the criteria describe the WAVE's end state, not its state at the moment the review was written.

## Known limitations

The review evaluates a plan, not an implementation. Findings F-01, F-03, F-07, F-10, F-12 and F-15 describe controls that are absent from the specification; whether they would also be absent from the code is unknowable, because the code does not exist. I treated a missing stated invariant as a defect rather than assuming the implementer would supply it, which is the correct posture for a security plan review but does mean some findings may be resolved by careful implementation alone.

## Security impact

Positive and confined to documentation. Nothing was weakened; no threshold, schema, golden case or policy was altered. The review surfaced one critical and four high plan-level defects before any code depends on them, which is the entire purpose of the gate.

The most consequential result is F-01: the plan does not state that project-scoped configuration may only restrict policy. Because `.ossus/` is attacker-supplied the moment a repository is cloned, and because `.ossus/config.toml` and `.ossus/policy.toml` carry `risk_max`, `allow_implicit_r4`, `block`, `require_hash_verification` and registry precedence, a hostile repository could otherwise defeat the R4 rule, the R5 exclusion and hash verification simultaneously with no user interaction.

## Performance impact

None. No code was changed. The V0 targets — resolve p95 < 500 ms at 1,000 manifests, ≥ 80% context reduction, zero external model calls — are unmeasured and unchallenged by this review.

## Deferred work

| Item | Owner |
|---|---|
| Apply ADR-016/017/018 and the plan edits | WAVE-000 continuation, after human decision |
| Schema changes: license required, commit pattern, surfaces enum, `distribution.mode` invariant, origin negative fixtures | WAVE-002 |
| Policy-monotonicity adversarial golden | WAVE-005 and `EVALUATION_STRATEGY.md` Layer 4 |
| Activation concurrency, locking and set-atomicity design | WAVE-007, before implementation |
| Transport allowlist enforcement | WAVE-012 |
| Repository initialization, fmt fixes, toolchain pin, CI hardening, help snapshot tests | WAVE-001 |

## Residual risks

1. **No independent cross-model security review.** ADR-014's 2026-08-03 revision accepted this. A single model's adversarial pass is correlated with that model's blind spots; human closure is the only compensating control. Carried forward unchanged.
2. **F-01 severity rests on absent specification, not observed behaviour.** No configuration loader exists to test against.
3. **F-02's correct answer is a genuine trade-off** between golden-file editability and keeping a YAML parser out of the TCB. Neither option is risk-free.
4. **F-06 is time-sensitive.** Once WAVE-001 runs `git init`, the external-origin archive including two network-fetching Python scripts enters the repository's permanent history.
5. **FTS5 availability is unverified.** ADR-007's spike belongs to WAVE-003 and was not pre-empted.
6. **Seven explicit uncertainties** are listed in the review and are not resolved by it.

## Recommended reviewer focus

D1 (F-01) first — it is the only critical finding and the only one that changes the product's threat model. Then D5 (F-06), because it expires when `git init` runs. Then D2, D3, D4. D6 is a batch acceptance.

## Handoff summary

WAVE-000's agent-side work is complete. Four documents are in `docs/implementation/08-operations/`. Gate S0 was open and required a named human approver to record six decisions in `WAVE-000_GATE_S0_CLOSURE.md`.

**Gate S0 was closed on 2026-08-04** by the project owner, adopting D1, D3, D4 and D6 as recommended, choosing D2 option A (convert to TOML) and D5 option (b) (relocate out of the repository). The decided edits are applied; see the amendment below. `CURRENT_WAVE.md` is advanced to WAVE-001, which proceeds as an audit-and-complete of the existing scaffold, delegated to Luna Max with Opus 5 verifying per `07-prompts/GENERAL_REVIEWER.md`.

## Amendment — 2026-08-04, post-closure edits applied

Applied after closure, and only as decided. Nothing here weakens a schema, threshold, golden case or policy.

**New ADRs** in `docs/product/DECISION_LOG.md`: ADR-016 (policy monotonicity), ADR-017 (TOML for taxonomy and evaluation data; ADR-006 unamended), ADR-018 (`ossus-activation` crate, created in WAVE-007).

**F-06 / D5, executed before `git init`:** `catalog/imports/` and `Ossus_v0.1_Almanac.zip` moved to the untracked `research-evidence/almanac-v0.1-import/`; `.gitignore` now blocks `*.zip`, `*.tar`, `*.tar.gz`, `*.tgz` and `catalog/imports/`; `catalog/README.md` and `catalog/official/README.md` corrected. The archive was relocated, not destroyed — see the closure record for the reasoning. `README.md:135` needed no edit: the relocation made its claim true.

**F-01 / ADR-016:** `SYSTEM_ARCHITECTURE.md` invariant 11; `THREAT_MODEL.md` hostile-repository adversary plus the "Project-scoped policy relaxation" threat and control; `OSSUS_MASTER_CONTEXT.md` §3.1 scope-as-trust-zone; Gate S2 monotonicity criterion; WAVE-005 adversarial fixture with four numbered assertions; `EVALUATION_STRATEGY.md` Layer 4 mandatory case.

**F-02 / ADR-017:** conversion scope, equivalence requirement and the "no YAML crate" constraint written into WAVE-002; `OSSUS_MASTER_CONTEXT.md` §7 extended to the taxonomy and frozen evaluation data.

**F-04 / ADR-018:** crate creation, dependency direction and the adapter-supplies-a-trait rule written into WAVE-007 and `RUST_WORKSPACE.md`.

**F-05 / D4:** gate ownership recorded in `SECURITY_GATES.md`, `PHASES_AND_GATES.md`, `WAVE_INDEX.md` (new **Closes gate** column) and the WAVE-002/WAVE-005 files. `PHASES_AND_GATES.md` Phase 3 now reads "Gates S3–S4"; S2 closes in Phase 2.

**Carried findings** written into their owning WAVE files: F-03, F-08, F-09, F-10, F-11, F-12 → WAVE-002; F-09 conflict fixture → WAVE-003; F-13, F-14, F-16, F-17, F-18, F-19, F-20 → WAVE-001; F-07 → `ACTIVATION_SECURITY.md` and WAVE-007; F-15 → `DATA_CONTRACTS.md` and WAVE-012; F-11, F-15 → `DATA_CONTRACTS.md`; F-18, F-19 → `RUST_WORKSPACE.md`; F-14 partial-satisfaction note → WAVE-010.

**Mirror integrity:** the nine canonical documents edited above were re-synced to their `docs/implementation/` mirrors. All pairs verified byte-identical after the sync.
