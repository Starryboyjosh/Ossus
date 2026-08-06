# Gate S1 — Trusted contracts: human closure package

**Gate:** S1 — Trusted contracts (`docs/security/SECURITY_GATES.md`)  
**Required before:** WAVE-003 Registry indexing  
**Prepared by:** Opus 5 (`claude-opus-5`), 2026-08-05  
**Approver:** pending named human  
**Status:** **READY FOR HUMAN DECISION — NOT CLOSED**

Model review is evidence, not certification. This document prepares the required record but does not close S1. WAVE-003 remains blocked until a named human completes the closure record below.

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
| 5 | Medium findings have a correction or accepted-risk owner | **Human decision required for DIV-6.** The taxonomy capability `status` and separate deprecations list duplicate a fact without a governed authority rule. Runtime validation currently uses capability status. Ratify the current behavior for V0 or assign a corrective ADR owner before closing. |
| 6 | Required attack tests pass | **Met, subject to the final command transcript in the WAVE report.** The fixture corpus, parser-budget tests, F-12 tests and Opus 5 regressions are included in workspace verification. |
| 7 | Threat model and trust boundaries updated | **Met.** Existing trust-boundary documents define canonical state and hostile parsed input; WAVE-002 enforces those boundaries without creating a new network or activation path. |
| 8 | No security threshold weakened without ADR | **Met.** Review corrections tightened behavior. No schema, budget, test, or threshold was weakened to obtain a pass. |
| 9 | No secrets appear in reports or fixtures | **Met by review and staged-tree inspection; recheck before closure if the tree changes.** Fixtures use synthetic identities and hashes. |
| 10 | Residual risks understandable | **Met.** DIV-6 and the standing single-model-review risk are stated below. |
| 11 | Next WAVE does not depend on an unresolved control | **Conditional.** All executable S1 controls are present. WAVE-003 remains blocked until the human dispositions DIV-6 and closes this gate. |
| 12 | Closure decision and approver recorded | **Open.** Complete the record below. |

## Contributions and review attribution

- T1 trusted YAML-to-TOML conversion: delegated implementation; Opus 5 verified counts, mirrors, trusted-path exclusion and absence of a YAML parser.
- T2 Gate S0 schema corrections F-03/F-08/F-09/F-10: delegated implementation; Opus 5 independently reviewed contracts and tests.
- T3 core IDs, enums, versions and stable reason codes: delegated implementation; Opus 5 independently reviewed and tested.
- T4 canonical loading, budgets, taxonomy and diagnostics: delegated implementation with Opus 5 final ownership. Opus 5 corrected six defects and added falsified regression tests.
- T5 fixture corpus: completed by Opus 5. A Codex attempt made no edits because the bridge lacked `OMNIROUTE_API_KEY`.

## Residual risks and required human disposition

### DIV-6 — duplicated deprecation authority

`capabilities-v1.toml` carries capability `status`, while `deprecations-v1.toml` separately lists deprecations and policy. The governed documents do not say which file is authoritative if they disagree. WAVE-002 deliberately did not invent a rule or weaken an existing test. The human approver must choose one:

1. **Ratify current V0 behavior:** capability `status` is authoritative for validation; the deprecations document is governed metadata until a later ADR defines enforcement.
2. **Require correction before closure:** assign an ADR and implementation owner, then keep S1 open until the authority rule and regression are added.

### Standing cross-model risk

Opus 5 is the final security reviewer. No independent cross-model security review completed because the optional Codex bridge was unavailable. This is correlated-review risk; human closure is the compensating control.

## Closure record — human must complete

> **Decision:** PENDING (`CLOSED` or `REJECTED`)  
> **Approver (named human):** PENDING  
> **Date:** PENDING  
> **DIV-6 disposition:** PENDING  
> **Accepted risks and rationale:** PENDING  
> **Authorization to begin WAVE-003:** PENDING
