# WAVE-003 final seed coverage authority

**Date:** 2026-08-09
**WAVE:** WAVE-003 — Seed Registry and local search
**Authority:** ADR-020, `docs/AGENT_AUTHORITY.md`, the WAVE-003 profile
reconciliation, frozen capability goldens, and the independent evidence
packets listed below.

**Coverage packet roles:** Curator/implementer `/root`; independent coverage
and amendment reviewer `/root/seed_admission_review_a`; independent blocker and
source reviewer `/root/seed_admission_review_b`; profile-only Closure Agent
`/root/wave003_security_closure`. These roles did not approve any candidate in
this pass. The profile-only closure record is
`closure/wave003-profile-amendments-5-7-11-12-20260809`.

## Decision in brief

The original seed target is **20 governed profile responsibilities**. The
coverage audit recommends a **provisional current admission-bearing planning
set of 16**:

- **9 `SEED_REQUIRED` profiles:** 1, 2, 3, 4, 8, 13, 14, 16, 19.
- **7 `SEED_REQUIRED_WITH_SUBSTITUTION` profiles:** 5, 6, 7, 9, 11, 12, 15.
- **4 `INTENTIONALLY_UNRESOLVED` profiles:** 10, 17, 18, 20.

No profile is currently justified as `REDUNDANT_FOR_SEED`, `INVALID_ORIGINAL_PROFILE`,
or removed. The four intentionally unresolved profiles remain useful future
coverage, but their current candidates do not create admission pressure for
the provisional minimum catalog used by the next product stages. This is a
governed planning minimum, not a quota: a profile in the denominator still needs a complete
Curator → independent Admission Review → Closure chain and a canonical
manifest.

Closure Agent record `closure/wave003-profile-amendments-5-7-11-12-20260809`
accepted the profile-only ceiling corrections (5: R3, 7: R2, 11: R2, 12: R3).
That record explicitly did not approve candidates, manifests, host activation,
or Registry admission. It also accepted 16 only as a provisional planning
denominator; the active WAVE completion obligation remains the original 20
real seed entries until an explicit architecture decision/ADR changes it.

This decision does **not** close WAVE-003. Only two resources are admitted,
both R0 Agent Skills-standard prompt-packs. The official catalog does not yet
provide enough positive capability, type, surface, risk, or overlap diversity
for meaningful Resolver evaluation, and the WAVE-003 acceptance requirement
for host-exclusive, cross-host, and genuine standalone-CLI examples is not
met. WAVE-003 therefore remains **IN PROGRESS**.

## What the seed Registry must prove

The seed catalog is evidence for the later Registry and Resolver; it is not a
claim that every golden capability already has a production resource.

### Capability coverage

The 20 profiles govern 24 distinct taxonomy capabilities. The frozen
`goldens-v1.toml` contains 50 cases and 41 distinct expected capabilities. The
17 expected capabilities absent from the profile set are:

```text
frontend.motion              backend.authorization
backend.background-jobs      backend.data-access
database.query-performance   quality.debugging
quality.refactoring          security.secrets-handling
security.secure-coding       ai.prompt-design
devops.deployment            devops.observability
documentation.api-reference  workflow.git
workflow.release             data.analysis
data.visualization
```

This mismatch is evaluation scope debt, not permission to stuff extra claims
into an admitted manifest. Positive real resources are required for exact
resource goldens and for useful Resolver selection. A missing capability may
also be tested as a deterministic low-confidence/no-candidate result. Later
evaluation work must explicitly decide how the 17 unprofiled capabilities
participate in Layer 1 metrics; it must not silently shrink the frozen goldens.

### Resource-type coverage

The approved initial types are `skill`, `prompt-pack`, and `mcp-server`.
WAVE-003's active acceptance contract calls for real positive examples of the
three types. The current Registry has only two prompt-packs: no admitted skill
and no admitted MCP server exists. Synthetic fixtures can exercise type
mismatch and policy denial, but cannot substitute for a real admitted
provenance/license/closure record when a type is part of the seed contract.

### Compatibility-surface coverage

The profile and acceptance contracts require truthful evidence for the
following surface families:

```text
agent-skills-standard   claude-code-cli   codex-cli
standalone-cli          generic-mcp-client
```

The two admitted manifests prove only `agent-skills-standard`. The accepted
profile-15 substitution also narrows a candidate to that surface; it does not
create a standalone CLI. A standard `SKILL.md` is not evidence of Claude,
Codex, or CLI compatibility. Surface-negative behavior can be covered with
synthetic fixtures, but the missing real positive surfaces remain a genuine
WAVE-003 gap.

### Risk coverage

The official catalog should contain truthful low-risk and executable examples
where the admission evidence supports them. The current minimum useful
coverage calls for real R0, R1, and R3 examples; R2 is not required merely for
numeric variety. R4/R5 and unsafe R0/R1 claims belong in adversarial fixtures
and must be denied before scoring. No R1/R2/R3 resource is currently official.

### Selection overlap and negative cases

The Resolver must eventually demonstrate positive selection, competition,
exclusion, surface filtering, risk denial, and redundancy removal. The two
official resources have disjoint capabilities and cannot exercise those
positive competition behaviors. Synthetic fixtures may supply deterministic
tie/overlap, forbidden-capability, incompatible-surface, missing-runtime,
revoked-source, R1→R3, R4-confirmation, and policy-monotonicity cases. They do
not replace real seed IDs in Layer 2 exact-resource goldens.

## Profile authority table

The profile classification below is separate from candidate and admission
state. `SEED_REQUIRED_WITH_SUBSTITUTION` means the profile is still needed,
but its original requirement must be corrected before a candidate can be
admitted. It never approves a resource.

| # | Profile | Required capability(s) | Resolver/golden role | Candidate/admission state | Governed profile disposition |
|---:|---|---|---|---|---|
| 1 | frontend design review | `frontend.visual-design` | GOLD-002, 005, 008; passive visual choice | Current active candidate rejected; profile remains valid | `SEED_REQUIRED` |
| 2 | responsive/a11y | `frontend.responsive-layout`, `frontend.accessibility` | GOLD-001, 006; bundled two-capability choice | Immutable MIT candidate conditional; host evidence missing | `SEED_REQUIRED` |
| 3 | frontend implementation | `frontend.implementation` | GOLD-005, 009; implementation must not be confused with design | Current R1 candidate rejected because writing code exceeds R1 | `SEED_REQUIRED` |
| 4 | frontend performance | `frontend.performance` | GOLD-004; bounded read-only performance review | Browser/generative candidate rejected; profile remains valid | `SEED_REQUIRED` |
| 5 | visual/browser validation | `testing.visual`, `testing.end-to-end` | GOLD-007, 026; executable browser validation | Closure-accepted R3 profile amendment; candidate still needs full admission evidence | `SEED_REQUIRED_WITH_SUBSTITUTION` |
| 6 | API design | `architecture.api-design` | GOLD-012; design must not select implementation | Standard-only substitution accepted; `mohitagw.technical-spec-template` admitted | `SEED_REQUIRED_WITH_SUBSTITUTION` |
| 7 | backend API | `backend.api-implementation` | GOLD-011, 017; distinct from API design | Closure-accepted R2 scoped-write amendment; candidate blocked pending admission evidence | `SEED_REQUIRED_WITH_SUBSTITUTION` |
| 8 | auth/security | `backend.authentication`, `security.identity-access` | GOLD-013, partial GOLD-014/034 | Current candidate rejected for policy elevation/raw-secret handling; profile remains valid | `SEED_REQUIRED` |
| 9 | schema/data modeling | `database.schema-design`, `architecture.data-modeling` | GOLD-019; schema design positive coverage | Standard-only substitution accepted; `mohitagw.database-schema-design` admitted | `SEED_REQUIRED_WITH_SUBSTITUTION` |
| 10 | migration review | `database.migrations`, `quality.code-review` | GOLD-020, 022; must not claim query performance or generic review | No physically enforced read-only adapter; candidate is held | `INTENTIONALLY_UNRESOLVED` |
| 11 | unit testing | `testing.unit` | GOLD-024; test-file writes are intrinsic but shell/network execution is not | Closure-accepted R2 test-only scoped-write amendment; candidate unadmitted | `SEED_REQUIRED_WITH_SUBSTITUTION` |
| 12 | integration testing | `testing.integration` | GOLD-025; executable integration behavior | Closure-accepted R3 runner/browser amendment; candidate unadmitted | `SEED_REQUIRED_WITH_SUBSTITUTION` |
| 13 | generic code review | `quality.code-review` | GOLD-027 and part of GOLD-022; generic review competition | Current source rejected for escaped references/R3 behavior | `SEED_REQUIRED` |
| 14 | threat model | `security.threat-modeling` | GOLD-031; passive security prompt coverage | Current prompt rejected because repository exploration is R3 | `SEED_REQUIRED` |
| 15 | dependency audit | `security.dependency-audit` | GOLD-033; dependency-only adapter must not become generic scanning | Surface substitution accepted to Agent Skills standard; candidate useful but deferred pending adapter, freshness, redaction, and scope evidence | `SEED_REQUIRED_WITH_SUBSTITUTION` |
| 16 | supply-chain review | `security.supply-chain` | GOLD-033, 035; unique security capability | Current composite-license/external-reference candidate rejected; clean replacement still needed | `SEED_REQUIRED` |
| 17 | container review | `devops.containers` | GOLD-039, 043; future operations coverage | Tuple/hash/license/content/host evidence incomplete | `INTENTIONALLY_UNRESOLVED` |
| 18 | CI review | `devops.ci-cd` | GOLD-035, 040; distinct from supply-chain review | Immutable/reference/Claude evidence incomplete | `INTENTIONALLY_UNRESOLVED` |
| 19 | technical writing | `documentation.technical-writing` | GOLD-045; passive documentation coverage | Current Claude/R3 tutorial candidate rejected; profile remains valid | `SEED_REQUIRED` |
| 20 | MCP integration | `ai.mcp-integration` | GOLD-037, 049; MCP policy and surface behavior | Original calculator rejected; replacement blocked by provenance/build/timeout/logging/Codex evidence | `INTENTIONALLY_UNRESOLVED` |

No profile is `REDUNDANT_FOR_SEED` or `INVALID_ORIGINAL_PROFILE`. The
`quality.code-review` overlap between profiles 10 and 13 is intentional and
should be constrained by scope: migration review must not win a generic pull
request task. Profiles 10, 17, 18 and 20 are intentionally unresolved because
their current evidence is incomplete or unsafe, not because their niches are
invalid.

## Amendment authority: profiles 5, 7, 11 and 12

The amendment packets correctly identify intrinsic execution behavior and do
not launder it as R0/R1. The recommended changes are:

| Profile | Original ceiling | Recommended correction | Required controls | Current authority state |
|---:|---|---|---|---|
| 5 | R2 | R3 browser validation | local-origin defaults, URL/egress allowlists, no credentials/cookies, no destructive actions, disposable bounded runner, explicit output paths | Closure-accepted profile amendment; no resource admission |
| 7 | R1 | R2 scoped API implementation | path allowlists, expected-diff/rollback, confirmation, command/network/deployment/credential denial, and host evidence | Closure-accepted profile amendment; no resource admission |
| 11 | R0 | R2 test-only scoped writes | test-file/explicit-path allowlists, no production deletion, expected-diff/rollback, confirmation, shell/network/dependency-install denial | Closure-accepted profile amendment; no resource admission |
| 12 | R1 | R3 integration testing | disposable runner, bounded commands/time/actions, local-origin/egress controls, no credentials/cookies/tokens, source-reference closure | Closure-accepted profile amendment; no resource admission |

These amendments are classified as `SEED_REQUIRED_WITH_SUBSTITUTION` for the
provisional coverage denominator, but the classification is not a candidate
approval. A resource still needs a fresh canonical proposal, independent review
(including security review for any R3 authority), and a distinct Closure Agent
decision.
If those controls cannot be enforced by an adapter, the candidate must be
rejected and the profile remains unfilled rather than being relabeled.

## Profile 15 blocker decision

Profile-level substitution from `standalone-cli` to
`agent-skills-standard` is accepted and remains seed-required. The candidate is
**USEFUL_BUT_DEFERRED**, not ready for Closure. Required evidence is an
immutable dependency-only adapter, bounded manifests/lockfiles and project
paths, current advisory freshness/TTL/provenance, fail-closed stale/offline
behavior, mandatory pre-send/pre-log redaction, and source-to-adapter hash
closure. Claude, Codex, and standalone-CLI compatibility must not be inferred.
The accepted substitution does not satisfy the separate aggregate
standalone-CLI example requirement.

## Coverage result and remaining genuine gaps

The official Registry contains two entries and only three of the 24 profile
capabilities (`architecture.api-design`, `database.schema-design`,
`architecture.data-modeling`). It lacks:

- every admitted `skill` and `mcp-server` positive;
- every admitted Claude, Codex, generic MCP, and standalone CLI surface;
- every R1, R2, and R3 positive;
- frontend, testing, security, quality, devops, documentation, and ai-agents
  categories;
- a second candidate with overlapping capability for deterministic competition;
- 21 of the 24 denominator capabilities.

These are actual positive seed gaps, not merely a low count. Synthetic
fixtures can cover negative and adversarial policy behavior, but cannot make
the official Registry satisfy the missing real type/surface/provenance or
Layer-2 exact-resource requirements.

The 17 golden capabilities outside the profile set are a separate later
evaluation-scope decision. They must not be silently assigned to an unrelated
manifest, and their absence must not be used as a reason to weaken existing
profiles.

## Final authority recommendation

```text
original_profiles                 = 20
seed_required                     = 9
seed_required_with_substitution   = 7
provisional planning denominator  = 16
active WAVE completion obligation  = 20 until architecture amendment
useful-but-deferred (profile)     = 0
intentionally_unresolved           = 4 (10, 17, 18, 20)
redundant_for_seed                 = 0
invalid_original_profile           = 0
official_admitted                  = 2
```

The four intentionally unresolved profiles are useful-but-deferred in
product terms, but are recorded under the single governed profile disposition
`INTENTIONALLY_UNRESOLVED` so that every original profile has exactly one
classification. The denominator is not a quota and may change only through a
new architecture decision supported by evidence.

Because positive catalog diversity is insufficient, 14 provisional planning
slots remain without Closure-approved resources, and the active 20-entry
completion obligation has not been met, the closure recommendation is:

```text
WAVE-003 — IN PROGRESS
WAVE-004 AUTHORIZATION RECOMMENDED: NO
```

The next authorized action remains targeted admission work for genuine gaps,
not broad discovery or Resolver implementation. A future closure packet may
recommend `WAVE-004 AUTHORIZATION RECOMMENDED` only after the required real
coverage and authority records exist; it may not treat synthetic fixtures or
the denominator itself as completion.

## Evidence references

- `docs/implementation/05-evaluations/goldens-v1.toml`
- `docs/implementation/05-evaluations/seed-catalog-profiles.toml`
- `docs/implementation/05-evaluations/EVALUATION_STRATEGY.md`
- `docs/implementation/05-evaluations/METRICS.md`
- `docs/implementation/02-architecture/RESOLVER_DESIGN.md`
- `docs/implementation/06-waves/03-registry-and-search.md`
- `docs/implementation/06-waves/05-resolver-core.md`
- `docs/implementation/08-operations/WAVE-003_SEED_PROFILE_RECONCILIATION.md`
- `docs/implementation/08-operations/WAVE-003_PROFILE_AMENDMENT_DECISIONS.md`
- `docs/implementation/08-operations/WAVE-003_SEED_CLOSURE_2026-08-08.md`
- `docs/implementation/08-operations/WAVE-003_REPORT.md`
