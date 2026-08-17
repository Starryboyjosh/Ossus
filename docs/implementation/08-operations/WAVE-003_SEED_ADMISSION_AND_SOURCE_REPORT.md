# WAVE-003 seed admission and source report

Date: 2026-08-08
State: profile reconciliation completed for the active batch; three candidates
are official Registry entries and the remaining candidates are evidence-only. The
provisional admission-bearing target is 16 of 20 governed profiles. The final
bounded sprint added no official entry: profile 15 is independently blocked and
profiles 5, 7, 11 and 12 have amendment packets only.

## Decision boundary

Research evidence is not Resolver authority. A candidate becomes official only
after canonical normalization, independent review, and a distinct Closure Agent
decision. No manifest was added merely to reach twenty.

## Profile disposition

| # | Seed profile | Candidate | Immutable source evidence | Current disposition |
|---:|---|---|---|---|
| 1 | `seed.frontend-design-review` | Microsoft `frontend-design-review` | `757d…`; `sha256:7a9f…e38b`; MIT | Source acceptable only at R3; profile R0 substitution and closure required |
| 2 | `seed.frontend-responsive-a11y` | wshobson `responsive-design` | `c4b82…`; `sha256:0c831…a312d`; MIT | **Admitted** as an R0 standard-only skill; official manifest present; no Claude/Codex/CLI claim |
| 4 | `seed.frontend-performance` | GitHub `premium-frontend-ui` | `ab7544d03d4c49fdd07f5958e1888ad39c4118e2`; `skills/premium-frontend-ui`; `sha256:842fdc00c81bd078a0381f1486a68c0867d688dac739f9d6805a895490cb2566`; MIT | Conditional R3; dependency/CDN and automatic-change controls required; profile R1 substitution required |
| 5 | `seed.visual-browser-validation` | GitHub `webapp-testing` | `ab7544…`; `sha256:9e221…6ef5`; MIT | Conditional R3; URL, credential, egress and action controls required; profile R2 substitution required |
| 3 | `seed.frontend-implementation` | Anthropic `frontend-design` | `f170…`; `sha256:dce2…6008`; Apache-2.0 | Independent review conditional; write behavior may exceed the profile's R1 ceiling; no manifest |
| 6 | `seed.api-design` | `mohitagw15856/pm-claude-skills` `technical-spec-template` | `fddfbc4a6caa8b4d3d41a69c666efaaff9d42def`; `skills/technical-spec-template`; `sha256:ab0ad71bfa86b59235f8832f6cdd3ba2088655ea99370652f10a904816c8962e`; MIT | **Admitted** after accepted standard-only substitution; official manifest present; no Claude/Codex claim |
| 7 | `seed.backend-api` | OpenAI `aspnet-core` | `49f948faa9258a0c61caceaf225e179651397431`; `skills/.curated/aspnet-core`; `sha256:c5aa64c6e3d9d9a239f2d7b87e1f12d859d002de28d10469d0d96ba39233e2e3`; Apache-2.0 | Source acceptable at R3 with restrictions; profile R1 substitution required |
| 8 | `seed.auth-security` | OpenAI `security-best-practices` | `49f948faa9258a0c61caceaf225e179651397431`; `skills/.curated/security-best-practices`; `sha256:d75eee606ccaed65b2a31d4a6ce8198f8fddfc3191c815142f8b0126df39dab4`; Apache-2.0 | Rejected as supplied: repository-policy elevation and raw-secret reporting defects require a distinct corrected artifact |
| 9 | `seed.schema-design` | `mohitagw15856/pm-claude-skills` `database-schema-design` | `fddfbc4a6caa8b4d3d41a69c666efaaff9d42def`; `skills/database-schema-design`; `sha256:9824fbc8b7e52af48bfbca811c624697d8c7201bbfc187cd6fdea49a7ce5c61b`; MIT | **Admitted** after accepted standard-only substitution; official manifest present; no Claude/Codex claim |
| 10 | `seed.migration-review` | Supabase `postgres-best-practices` | `1207767388a0ffb55f21fb4e6988fee96942431d`; `skills/supabase-postgres-best-practices`; `sha256:bcacf5fbb85c8c0e407ebb5eab85240349526b682ea311217da21411e2b695e9`; MIT | Postgres-bound R1 mapping is conditional; required surfaces and an enforced read-only adapter are missing; no manifest |
| 11 | `seed.unit-testing` | obra/superpowers `test-driven-development` | `44c9…`; `sha256:cce799…c674`; MIT | Conditional R3; destructive instruction and test execution controls required; profile R0 substitution required |
| 12 | `seed.integration-testing` | addyosmani `test-driven-development` | `f493…`; `sha256:215b…cfda`; MIT | Profile R3 amendment/split required; current source is blocked for local writes, shell/test and browser/MCP execution; no admission |
| 13 | `seed.code-review` | addyosmani `code-review-and-quality` | `f493…`; `sha256:bbe388…a2ba`; MIT | Conditional R3; no automatic audits, cross-skill activation or merge authority; profile R0 substitution required |
| 14 | `seed.threat-model` | OpenAI `security-threat-model` prompt template | `49f948faa9258a0c61caceaf225e179651397431`; `skills/.curated/security-threat-model/references/prompt-template.md`; `sha256:2859871a9d51726123f2b97b28edb64c53900bad5195cb6d54fbedfd004b56a1`; Apache-2.0 | Conditional R3 prompt-pack adaptation; embedded prompt is untrusted; profile R0 substitution required |
| 15 | `seed.dependency-audit-cli` | GitHub `security-review` fallback | `ab7544…`; `skills/security-review`; `sha256:cbda6a05f7821345a816febb4dbf677980d55ed7aaefd84e9cafe333e2c65511`; MIT | Closure accepted the profile-level substitution to `agent-skills-standard`; independent review `admission-review-a/profile-15-dependency-audit-20260808` blocks Registry admission pending a bounded adapter, freshness protocol, redaction evidence and a later Closure decision; it is never a standalone CLI |
| 16 | `seed.supply-chain-review` | Microsoft hve-core `supply-chain-security` | `dd0f4920f73bbceae71a045a5344332fc1a6bb2b`; `.github/skills/security/supply-chain-security`; `sha256:858229f7683a85f54a203e2fce98f93de99f6800571202d11ada0b48ce00fde5` | Unresolved/rejected for now: composite incorporated licensing, incomplete external references, and no direct Claude Code evidence |
| 17 | `seed.container-review` | GitHub `multi-stage-dockerfile` | `ab7544…`; `sha256:f59c…4549`; MIT | Intentionally unresolved; profile remains R1/read-only, but fresh exact tuple/hash/license/content/capability and host evidence is required |
| 18 | `seed.ci-review` | GitHub `github-actions-hardening` | `ab7544…`; `sha256:febf…c5cb`; MIT | Intentionally unresolved; profile remains R1/read-only, but fresh exact tuple/hash/license/reference/content and Claude evidence is required |
| 19 | `seed.technical-writing` | Xamfonos `technical-writing-best-practices` | `4c8e…`; `sha256:b825…60f3`; MIT | Candidate is upstream skill; canonical prompt-pack type substitution requires closure |
| 20 | `seed.mcp-integration` | `githejie/mcp-server-calculator`; replacement `slettmayer/calc-mcp-server` v0.1.3 | Original `3dca…`; replacement `805a177573c3d56cfa5e33f28571f9256fbbf92c`; root hash `sha256:63342cde3e9642a4d71bf988d70671e5e46e3fb44491a6fb6989ca3d1cdfd0c1`; MIT | Original rejected. Replacement is conditional: bounded evaluator and generic stdio MCP are positive, but build inputs, artifact pinning, release-tree scope, metadata mismatch, Codex surface and logging/timeout evidence block admission |

Abbreviated hashes in this interim table must be expanded from the agents'
evidence bundles before a canonical proposal is created. They are deliberately
not sufficient for admission.

The replacement profile-20 calculator is materially better-shaped than the
previous MCP root: its evaluator has AST, expression-length, depth, exponent,
factorial and rendered-size bounds, and its stdio surface is narrow. Review B
still found unpinned build requirements, an unpinned `uvx` launcher, stale
`server.json` version metadata, release workflows and scripts in the root
source, no explicit wall-clock bound for expensive transcendental chains, and
input-sensitive logging. It is therefore a conditional future candidate, not
an admission or an official replacement.

## Closure-oriented batch evidence

The Curator staged three complete field drafts (without a review block) under
the ignored quarantine path `research-evidence/wave003-staging/`:

- `mohitagw.technical-spec-template.toml` (profile 6);
- `mohitagw.database-schema-design.toml` (profile 9);
- `supabase.postgres-best-practices.toml` (profile 10).

All three pass the local canonical validator when the review block is present,
and their source tuples, hashes, licenses, type classifications and bounded
capability mappings were independently checked. Review A's field-level
re-review found that profiles 6 and 9 can only proceed through an explicit
standard-only profile substitution, which still requires a distinct Closure
decision and an aggregate cross-host coverage check. Profile 10 additionally
needs an immutable, enforced read-only adapter; manifest exclusions alone do
not establish the R1 boundary. At the time of this staging inspection no
review approval was asserted and no staging manifest was created; the later
Closure record authorized only the separate official profile-6 and profile-9
copies.

## Rejected candidates

- Vercel `react-best-practices`: no immutable license evidence and invokes
  mutable `npx` behavior.
- wshobson `fastapi-templates`: insecure wildcard credentialed CORS example and
  verbose SQL logging.
- wshobson `auth-implementation-patterns`: OAuth token in a query string and
  incomplete JWT issuer/audience checks.
- wshobson `database-migration`: database/CLI mutation and rollback behavior,
  not a bounded review resource.
- UnitOne `dependency-scanning`: required references escape the selected tree;
  runtime declarations conflict with its allowed-tools metadata.
- OWASP dependency/SCA candidate: selected subpath did not close over required
  references; the larger parent was R4 and contained unsafe path relationships.
- GitHub `agent-supply-chain`: its own hash procedure had unsafe path/symlink
  behavior.
- GitHub MCP server: credentialed remote mutation and R4 behavior.
- modelcontextprotocol/servers root: rejected as described for profile 20.
- `apache/airflow-steward` `skills/dependency-audit`: genuine CLI workflow, but
  its selected file requires references outside the immutable subpath.
- `bobmatnyc/claude-mpm-skills` dependency audit: self-contained, but directs
  destructive updates and credential-backed Snyk behavior, exceeding R3 and
  not providing the requested generic standalone surface.
- `githejie/mcp-server-calculator`: rejected for the evaluator and build-chain
  defects described above; retain as a future candidate only after upstream
  correction.

## Closure decisions

The distinct Closure Agent (`/root/wave003_security_closure`) attested that it
did not research, implement, or independently review either candidate.

- Profile 15: **profile-level substitution accepted**, changing only the
  required surface from `standalone-cli` to `agent-skills-standard`, with
  `portable-with-adapter`. **Registry admission blocked** until a Curator
  proposal and a complete independent review record establish the bounded
  `security.dependency-audit` capability, R3 freshness controls, and strict
  secret redaction.
- Profile 20: **rejected**. The calculator's unbounded synchronous evaluator,
  stale lock metadata, mutable build inputs and unproven artifact provenance
  are not accepted as residual risk. A corrected immutable commit would need
  evaluator bounds, adversarial exhaustion tests, frozen build inputs and fresh
  independent security review.

The distinct Closure Agent later recorded
`closure/wave003-r0-standard-6-9-20260808`, accepting the explicit
standard-only substitutions and authorizing the two exact profile-6/profile-9
resources. A separate Closure Agent accepted profile 2 on 2026-08-16. Their
official manifests carry the required review block and three entries are now
indexed. Profile 10 remains blocked by the
missing read-only adapter; profiles 17 and 18 remain intentionally unresolved
until fresh evidence bundles are complete. No other official manifest is
authorized.

## Open admission work

1. Obtain distinct admission Closure decisions for the remaining acceptable
   sources and every proposed profile substitution; official count is 3.
2. Keep profile 16 unresolved unless its licensing, references, capability and
   direct Claude Code evidence are repaired.
3. Resolve profile 15 only as an explicit agent-skill substitution, never as a
   standalone CLI.
4. Correct the replacement profile-20 source/build provenance or find another
   narrow immutable MCP server; then obtain fresh security review.
5. Expand every abbreviated coordinate and normalize capabilities, exclusions,
   surfaces, runtime, risk and permissions into canonical Ossus manifests.
6. Only after Closure accepts exact evidence, add approved manifests to
   `catalog/official/manifests/` and rebuild the Registry incrementally.

## Final-admission sprint result

The sprint used bounded Luna Max reviews rather than another broad discovery
sweep. No candidate met the complete Curator → independent review → Closure
chain beyond the already admitted profiles 6 and 9. Profile 15 has a valid
immutable MIT source tuple and accepted standard-only profile substitution, but
the independent review found that the broad source is not constrained to
dependency manifests/lockfiles, has no immutable adapter, and lacks a current
advisory freshness and secret-redaction protocol. It is therefore **BLOCKED —
NOT READY FOR CLOSURE**. Profiles 5, 7, 11 and 12 are documented as proposed R3
amendments in `WAVE-003_PROFILE_AMENDMENT_DECISIONS.md`; no profile amendment or
candidate admission was silently applied.

## Targeted admission continuation — 2026-08-16

The subsequent targeted packet for profile 2,
`wshobson.responsive-design`, completed the required Curator → independent
Admission Review → Closure chain. The independent review
`admission-review-codex/wave003-responsive-design-20260816-r1` returned
`READY_FOR_CLOSURE`; the distinct Closure Agent
`closure-agent/wave003-responsive-design-20260816-c1` returned `ACCEPTED`.

The accepted manifest is an R0 `skill` with `instruction-only` runtime,
`source-only` distribution, project scope, and the standard Agent Skills
surface only. It covers responsive layout and bounded responsive interaction
accessibility semantics. Claude Code, Codex, standalone CLI, and other host
surfaces remain unproven and are not recorded. The official Registry now
contains three resources; all other candidate dispositions and the active
20-entry WAVE obligation remain unchanged.
