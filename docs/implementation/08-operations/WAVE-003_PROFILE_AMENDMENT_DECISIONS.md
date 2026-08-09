# WAVE-003 profile-amendment decision packets

**Date:** 2026-08-08  
**Authority:** ADR-020; the original packets are Curator evidence. The
independent review addendum below records a bounded review recommendation; it
is not resource admission. A separate Closure Agent decision is still required
for any amendment to become operative.

These packets separate profile design from candidate admission. A proposed
amendment changes what a seed profile honestly asks for; it does not approve
the candidate that motivated the amendment. Each amendment still requires an
independent review and a distinct Closure Agent decision before it can affect
the official Registry.

## Profile 5 — visual/browser validation

```text
profile: seed.visual-browser-validation
original requirement: skill; testing.visual and testing.end-to-end; Claude Code;
  maximum R2
reason original requirement failed: the investigated webapp-testing resource
  uses Playwright/MCP or a local Node/Playwright runner, navigates URLs, clicks
  and submits controls, captures screenshots, and inspects browser/network
  logs. Driver, browser and network behavior are R3.
real ecosystem evidence: github/awesome-copilot webapp-testing at the pinned
  immutable source; the source is useful but not admission-ready.
proposed requirement: retain type, capabilities and Claude surface; amend the
  maximum risk to R3. Require local-origin defaults, explicit URL/egress
  allowlists, no cookies/storage/credentials, no destructive actions, no
  automatic dependency installation, a disposable runner, and bounded output
  paths.
security delta: intentionally expands authority from R2 to R3 and makes
  browser/network controls mandatory; no implicit activation is permitted.
surface delta: Claude remains required; no Codex or other surface is inferred.
risk delta: R2 -> R3, with a fresh security review required.
candidate availability: conditional candidate exists; no approved resource.
golden/evaluation impact: retain GOLD-007 and GOLD-026; add deny-under-R2 and
  permit-only-under-R3 policy cases with origin, credential and egress checks.
recommended decision: propose the R3 amendment; keep the candidate blocked.
required authority: Curator amendment, independent security review, distinct
  Closure Agent decision.
```

## Profile 7 — backend API implementation

```text
profile: seed.backend-api
original requirement: skill; backend.api-implementation; Claude Code and Codex;
  maximum R1
reason original requirement failed: the investigated ASP.NET resource writes
  code, invokes dotnet workflows, builds/tests, and consults current web
  material. That is R3 rather than bounded R1 guidance.
real ecosystem evidence: openai/skills aspnet-core at the pinned immutable
  source; it is API-relevant but broader than the seed contract and lacks
  direct Claude evidence.
proposed requirement: retain type, capability and both required surfaces;
  amend the maximum risk to R2 and restrict the scope to API work. Exclude
  deployment, production databases, credentials, and auth/authorization
  changes unless separately authorized.
security delta: makes scoped project writes explicit; adds path allowlists,
  expected-diff/rollback and confirmation while denying shell, network,
  deployment, production-database, credential and auth changes by default.
surface delta: both surfaces remain requirements; no Claude compatibility is
  inferred from the source format.
risk delta: R1 -> R2.
candidate availability: conditional candidate exists; scope and Claude evidence
  remain incomplete, so it is not admission-ready.
golden/evaluation impact: retain GOLD-011 and GOLD-017 and add R1-deny/R2-
  allow cases proving API-only scope cannot select deployment, auth or database
  mutation behavior.
recommended decision: propose the R3 amendment; keep the candidate blocked.
required authority: Curator amendment, independent review of the adapter and
  policy tests, distinct Closure Agent decision.
```

## Profile 11 — unit testing

```text
profile: seed.unit-testing
original requirement: skill; testing.unit; Agent Skills standard, Claude Code
  and Codex; maximum R0
reason original requirement failed: the investigated Superpowers TDD resource
  writes tests, runs repository commands, and deletes/restarts production code
  before tests. Shell, execution and mutation are intrinsic.
real ecosystem evidence: obra/superpowers test-driven-development at the pinned
  MIT source; the selected tree is locally closed but remains unreviewed at R3.
proposed requirement: retain type, capability and required surfaces; amend the
  maximum risk to R2. Limit writes to test files or explicitly approved paths, deny
  blind deletion, require expected-diff/rollback checks and a disposable
  no-secret runner, and deny dependency installation/network by default.
security delta: converts test-file mutation into explicit R2 authority with
  confirmation, scope and rollback controls while denying shell, network,
  dependency installation and production-code deletion by default.
surface delta: all three surfaces remain requirements; none is inferred until
  host evidence exists.
risk delta: R0 -> R2.
candidate availability: one plausible candidate exists; it has no independent
  admission review or Closure decision.
golden/evaluation impact: retain GOLD-024; add R2 path/rollback cases and a
  denial case proving the resource cannot pass the R0 contract.
recommended decision: propose the R3 amendment; do not admit the candidate yet.
required authority: Curator amendment, independent review and activation
  evidence for the bounded runner, distinct Closure Agent decision.
```

## Profile 12 — integration testing

```text
profile: seed.integration-testing
original requirement: skill; testing.integration; Claude Code and Codex;
  maximum R1
reason original requirement failed: the investigated addyosmani resource writes
  project code, runs shell/tests, uses browser/Chrome DevTools MCP, and has
  references outside the selected immutable tree. Genuine integration testing
  is executable and mutating, not R1 passive review.
real ecosystem evidence: addyosmani test-driven-development at the pinned MIT
  source; it is reviewable but not admissible under the current profile.
proposed requirement: preserve skill/category/capability. Prefer an explicit
  R3 executable-test amendment with a disposable no-secret runner, scoped
  workspace writes, bounded commands/time/actions, local-origin and explicit
  URL/egress controls, no credentials/cookies/tokens, and no production or
  deployment authority. A separate read-only planning profile is the fallback;
  the execution source must not be relabeled R1.
security delta: R1 -> R3 expands runner, browser, local-service and egress
  authority, so policy denial, confirmation and fresh security evidence are
  mandatory.
surface delta: Claude/Codex claims remain pending future adapter evidence; a
  SKILL.md format does not prove either host.
risk delta: R1 -> R3.
candidate availability: current candidate is blocked; no passive replacement is
  evidenced.
golden/evaluation impact: retain GOLD-025; add Layer-2 exact-resource cases
  that enforce R3 controls and deny the source under the original R1 contract.
recommended decision: propose the R3 amendment or explicit planning split; keep
  the candidate unfilled pending authority.
required authority: Curator amendment, independent review of source/adapter and
  golden impact, distinct Closure Agent decision.
```

## Independent review addendum — 2026-08-09

Admission Review Agent A independently reviewed the four packets against the
capability taxonomy, GOLD-007/011/024/025/026 and the Resolver risk boundary.
The review rejects a blanket R3 upgrade:

| Profile | Independent review recommendation | Required boundary | Candidate state |
|---:|---|---|---|
| 5 | `R2 → R3`; `SEED_REQUIRED_WITH_SUBSTITUTION` | localhost-by-default browser runner, explicit origin/egress allowlists, no credentials/cookies/storage, confirmation before state changes, pre-provisioned dependencies, bounded actions/time/output, disposable no-secret runner | Blocked; SKILL format does not prove Claude compatibility |
| 7 | `R1 → R2`; `SEED_REQUIRED_WITH_SUBSTITUTION` | path allowlists, expected diff/rollback, confirmation, explicit denial of shell/network/deployment/credential work; Claude/Codex evidence remains required | Blocked; candidate breadth and missing Claude evidence remain |
| 11 | `R0 → R2`; `SEED_REQUIRED_WITH_SUBSTITUTION` | test-only/explicit write paths, no production deletion, expected diff/rollback, confirmation, shell/network/dependency-install denial | Blocked; candidate's generic feature/TDD and blind deletion behavior exceed the amended scope |
| 12 | `R1 → R3`; `SEED_REQUIRED_WITH_SUBSTITUTION` | disposable no-secret runner, isolated test DB/namespace, bounded commands/processes/ports/time, localhost and explicit egress allowlist, no production/deployment authority or credentials | Blocked; browser/DevTools, external-reference and host evidence remain incomplete |

The review conclusion is that execution risk is intrinsic for profiles 5 and
12, while the investigated candidates added unnecessary R3 behavior for
profiles 7 and 11. The narrower R2 amendments preserve useful implementation
and test-file-write coverage without granting shell, network, deployment or
unbounded runner authority. No amendment creates a canonical manifest.

## Closure state

The distinct Closure Agent recorded
`closure/wave003-profile-amendments-5-7-11-12-20260809` and **accepted the
profile-only ceiling corrections**: profile 5 R3, profile 7 R2, profile 11 R2,
and profile 12 R3. This is a design decision only. The following are
explicitly not approved:

- no candidate resource;
- no host adapter;
- no runtime permission;
- no risk override beyond the bounded amendment text;
- no Registry entry.

The historical reconciliation ledger remains `PROFILE_SUBSTITUTION_PROPOSED`
because it records the original packet state; the final coverage authority
records the Closure-accepted corrections. The coverage denominator does not
turn the amendments into admission or waive the Curator → independent Review →
Closure chain for resources.

## Decision status

All four packets remain profile substitutions at the design level, now with
Closure-accepted ceilings. The independent review and Closure decision do not
lower a security requirement silently, change taxonomy, create a canonical
manifest, or authorize a Registry entry. Profiles 5, 7, 11 and 12 remain
unfilled and their candidates remain blocked.
