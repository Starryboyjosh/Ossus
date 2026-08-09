# Gate S1 — Trusted contracts: human closure package

> **Historical evidence:** ADR-020 and `docs/AGENT_AUTHORITY.md` supersede this document's authority requirements for decisions on or after 2026-08-07. This note does not change recorded facts.

**Gate:** S1 — Trusted contracts (`docs/security/SECURITY_GATES.md`)
**Required before:** WAVE-003 Registry indexing
**Prepared by:** Opus 5 (`claude-opus-5`), 2026-08-05
**Approver:** Starryboyjosh
**Status:** **CLOSED — WAVE-003 AUTHORIZED**

Model review is evidence, not certification. The named project owner reviewed this evidence, accepted the stated residual risks, ratified the V0 authority rule, and closed S1 on 2026-08-06.

## Inputs

| Evidence | Path |
|---|---|
| WAVE definition and Gate S1 requirements | `docs/implementation/06-waves/02-spec-and-taxonomy.md` |
| T4 implementation report and Opus 5 reviewer addendum | `WAVE-002_T4_REPORT.md` |
| Executable fixture inventory and expected reason codes | `crates/ossus-registry/tests/fixtures/INDEX.toml` |
| Fixture regression harness | `crates/ossus-registry/tests/wave_t5_fixtures.rs` |
| Opus 5 review regressions | `crates/ossus-registry/tests/wave_t4_review.rs` |
| Trusted lockfile schema | `specs/schemas/skills-lock.schema.json` |
| Final command evidence | `WAVE-002_T4_REPORT.md` final verification addendum |

## Gate S1 requirements

| Requirement | Assessment |
|---|---|
| Schemas reject unknown major versions | **Met.** Canonical manifests use a fixed 1.0.0 schema contract; typed `Version` validation rejects unsupported majors. The executable corpus includes `schema-version-unknown-major.toml` with a stable reason code. |
| Parser budgets tested | **Met.** Tests cover manifest bytes, invalid UTF-8, UTF-8 string bytes, list items, restricted and boundary nesting, brackets inside strings/comments, and hostile 10,000-level input. Parsing and structural walks are bounded. |
| Canonical/origin separation explicit | **Met.** F-12 fixtures reject `origin`, `author_capabilities`, `upstream_triggers`, and `stars`; regression evidence proves they cannot deserialize into canonical state. Origin evidence remains outside the canonical manifest contract. |
| Taxonomy and policy hashes supported | **Met at the trusted lockfile-contract boundary.** `taxonomy_hash` and `policy_hash` are required and constrained to lowercase `sha256:<64 hex>` values in the source schema and mirror. The canonical example carries both. Layout regression checks reject missing, uppercase, wrong-length, and wrong-prefix values. Hash calculation and lockfile generation belong to later producer WAVEs and were not implemented early. |

## Human security closure checklist

| # | Item | Opus 5 assessment |
|---|---|---|
| 1 | Opus 5 security implementation or review report exists | **Met.** `WAVE-002_T4_REPORT.md` contains the independent reviewer addendum and final verification evidence. |
| 2 | Every implementation-agent contribution attributed and reviewed by Opus 5 | **Met.** T1–T3 and T4 delegated implementation are recorded in task/report history and independently inspected and re-executed by Opus 5. T5 was completed directly by Opus 5 after the Codex attempt failed before edits because `OMNIROUTE_API_KEY` was unavailable. |
| 3 | All critical findings resolved | **Met.** No critical finding survived final review. |
| 4 | All high findings resolved | **Met.** No high finding survived final review. |
| 5 | Medium findings have a correction or accepted-risk owner | **Met through named-human disposition.** Starryboyjosh ratified capability `status` as the V0 validation authority; the separate deprecations document remains governed, non-enforcing metadata pending a later ADR. |
| 6 | Required attack tests pass | **Met, subject to the final command transcript in the WAVE report.** The fixture corpus, parser-budget tests, F-12 tests and Opus 5 regressions are included in workspace verification. |
| 7 | Threat model and trust boundaries updated | **Met.** Existing trust-boundary documents define canonical state and hostile parsed input; WAVE-002 enforces those boundaries without creating a new network or activation path. |
| 8 | No security threshold weakened without ADR | **Met.** Review corrections tightened behavior. No schema, budget, test, or threshold was weakened to obtain a pass. |
| 9 | No secrets appear in reports or fixtures | **Met by review and staged-tree inspection; recheck before closure if the tree changes.** Fixtures use synthetic identities and hashes. |
| 10 | Residual risks understandable | **Met.** DIV-6 and the standing single-model-review risk are stated below. |
| 11 | Next WAVE does not depend on an unresolved control | **Met.** All executable S1 controls are present, and the named-human DIV-6 disposition closes the remaining authority question for V0. |
| 12 | Closure decision and approver recorded | **Met.** The closure record below names the decision, approver, date, disposition, accepted risk, and WAVE-003 authorization. |

## Contributions and review attribution

- T1 trusted YAML-to-TOML conversion: delegated implementation; Opus 5 verified counts, mirrors, trusted-path exclusion and absence of a YAML parser.
- T2 Gate S0 schema corrections F-03/F-08/F-09/F-10: delegated implementation; Opus 5 independently reviewed contracts and tests.
- T3 core IDs, enums, versions and stable reason codes: delegated implementation; Opus 5 independently reviewed and tested.
- T4 canonical loading, budgets, taxonomy and diagnostics: delegated implementation with Opus 5 final ownership. Opus 5 corrected six defects and added falsified regression tests.
- T5 fixture corpus: completed by Opus 5. A Codex attempt made no edits because the bridge lacked `OMNIROUTE_API_KEY`.

## Residual risks and required human disposition

### DIV-6 — duplicated deprecation authority

`capabilities-v1.toml` carries capability `status`, while `deprecations-v1.toml` separately lists deprecations and policy. WAVE-002 deliberately did not invent a rule or weaken an existing test. Starryboyjosh ratified the current V0 behavior: capability `status` is authoritative for validation, while the deprecations document remains governed metadata until a later ADR defines enforcement.

### Standing cross-model risk

Opus 5 is the final security reviewer. No independent cross-model security review completed because the optional Codex bridge was unavailable. This is correlated-review risk; human closure is the compensating control.

## Closure record

> **Decision:** CLOSED
>
> **Approver (named human):** Starryboyjosh
>
> **Date:** 2026-08-06
>
> **DIV-6 disposition:** Ratify current V0 behavior. `capability.status` is authoritative for validation. `deprecations-v1.toml` remains governed, non-enforcing metadata until a later ADR defines enforcement.
>
> **Accepted risks and rationale:** The correlated-review risk is accepted because Opus 5 implemented or independently reviewed the final behavior, all executable S1 controls pass, and named-human closure provides the required compensating control. The duplicated deprecation representation is accepted under the explicit V0 authority rule above.
>
> **Authorization to begin WAVE-003:** AUTHORIZED
