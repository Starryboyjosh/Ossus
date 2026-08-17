# WAVE-003 canonical manifest proposals

These records distinguish proposals from admitted resources. The exact drafts
for profiles 2, 6, 9 and 10 remain in the ignored quarantine path
`research-evidence/wave003-staging/`; the profile-10 draft is still not
official. Profiles 6 and 9 completed the Curator → independent Admission
Review → Closure chain and were materialized as official manifests in
`catalog/official/manifests/`. Profile 2 completed a separate targeted chain
on 2026-08-16 and is now the third official manifest. Their accepted substitution is
`agent-skills-standard` only; no Claude Code, Codex, standalone CLI or
cross-host claim is implied.

Profile-design amendments for execution-heavy profiles 5, 7, 11 and 12 are
recorded in `WAVE-003_PROFILE_AMENDMENT_DECISIONS.md`. They are Curator packets
only; they do not change the canonical taxonomy, approve a candidate, or
authorize a manifest.

## Proposal matrix

| Profile | Proposed ID/type | Required capabilities | Proposed risk/runtime | Source lock |
|---:|---|---|---|---|
| 2 | `wshobson.responsive-design` / `skill` | `frontend.responsive-layout`, `frontend.accessibility` | R0 / `instruction-only`; **official**, standard-only | `wshobson/agents@c4b82b0ad771190355eb8e204b1329732a18449a`, `plugins/ui-design/skills/responsive-design`, `sha256:0c8319415cae9458b074f6306b3e3f96b2a8b7d1a4340efdd36ec35b197a312d`, MIT |
| 3 | `anthropic.frontend-design` / `skill` | `frontend.implementation` | R1 / `filesystem-only` | `anthropics/skills@f17010c9bb483898c1d9c9f42dde2b3a98889434`, `skills/frontend-design`, `sha256:dce2d83607b10db4a0464a0638b94a8f4f5fc5835e245d58a0de0cb155746008`, Apache-2.0 |
| 6 | `mohitagw.technical-spec-template` / `prompt-pack` | `architecture.api-design` | R0 / `instruction-only`; standard-only substitution accepted; **official** | `mohitagw15856/pm-claude-skills@fddfbc4a6caa8b4d3d41a69c666efaaff9d42def`, `skills/technical-spec-template`, `sha256:ab0ad71bfa86b59235f8832f6cdd3ba2088655ea99370652f10a904816c8962e`, MIT |
| 9 | `mohitagw.database-schema-design` / `prompt-pack` | `database.schema-design`, `architecture.data-modeling` | R0 / `instruction-only`; standard-only substitution accepted; **official** | `mohitagw15856/pm-claude-skills@fddfbc4a6caa8b4d3d41a69c666efaaff9d42def`, `skills/database-schema-design`, `sha256:9824fbc8b7e52af48bfbca811c624697d8c7201bbfc187cd6fdea49a7ce5c61b`, MIT |
| 10 | `supabase.postgres-best-practices` / `skill` | `database.migrations`, bounded `quality.code-review` | R1 / `filesystem-only`; exact draft currently standard-only, adapter missing | `supabase/agent-skills@1207767388a0ffb55f21fb4e6988fee96942431d`, `skills/supabase-postgres-best-practices`, `sha256:bcacf5fbb85c8c0e407ebb5eab85240349526b682ea311217da21411e2b695e9`, MIT |
| 17 | `github.multi-stage-dockerfile` / `skill` | `devops.containers` | R1 / pending evidence | `github/awesome-copilot@ab7544d03d4c49fdd07f5958e1888ad39c4118e2`, `skills/multi-stage-dockerfile`, `sha256:f59c1d1d3564f561d6c93374fadf8c50abdf75f81a4801213bef1378e6134549`, MIT |
| 18 | `github.github-actions-hardening` / `skill` | `devops.ci-cd` | R1 / pending evidence | `github/awesome-copilot@ab7544d03d4c49fdd07f5958e1888ad39c4118e2`, `skills/github-actions-hardening`, `sha256:febf13fe2571020b1063e5c75340d2d7560da2ae8036aefbabc842861026c5cb`, MIT |

## Shared proposed fields

Unless a reviewer finds contrary source evidence, the proposals use
`schema_version = "1.0.0"`, `capability_schema = "1.0.0"`, source mode
`remote-index`, distribution mode `source-only`, `notice_required = false`,
project scope, and `portable-standard` only for standard `SKILL.md` content
whose host behavior is actually proven. No upstream compatibility claim is
canonical without host evidence.

Every proposal needs bounded triggers and exclusions. In particular:

- The Supabase proposal must be explicitly Postgres-bound and fail closed if its
  `quality.code-review` mapping is not material.
- The two prompt-pack proposals are curator classifications of static source
  material, not upstream type claims.
- The Anthropic proposal is `filesystem-only` at R1; do not claim Codex
  compatibility without host evidence. The Supabase proposal's code-review
  capability is limited to Postgres migration/schema review.
- The GitHub DevOps proposals must not imply that documentation can mutate CI or
  container state; source prose remains untrusted instructions.
- The Anthropic proposal must not imply a host surface or external tool not
  observed in the selected tree.

The current exact-draft review found two explicit profile substitutions rather
than silent metadata changes: profiles 6 and 9 drop unproven Claude Code and
Codex surfaces and retain only the standard surface. A later targeted review
accepted profile 2 with the same standard-only boundary. Closure accepted
these substitutions and the three manifests are now official, while explicitly
recording that they do not satisfy aggregate cross-host diversity. The profile-10 draft
cannot proceed on exclusions alone; its read-only adapter must be independently
evidenced and enforce denial of mutation, shell, network, credentials and
database access.

## Required next review

Admission Review Agent A's independent review marked profiles 2, 3 and 10
conditional, passed profiles 6 and 9 pending Closure, and identified the
surface/risk corrections above. The distinct Closure Agent accepted profiles 6
and 9; a later distinct Closure Agent accepted profile 2 and its official
manifest now also carries the required review block.
Admission Review Agent B's earlier second pass was interrupted, but its later
independent profile-20 replacement review is recorded in the seed source and
closure reports. Admission Review Agents must independently validate the exact
source tuple, canonical capabilities, type classification, compatibility,
runtime, risk, triggers, exclusions and scopes. A distinct Closure Agent then
decides each proposal. Profile 10, profile 15, and all other unaccepted
candidates must not be copied into the official Registry.

## Exact draft ledger (not official)

The following are the Curator's exact normalized fields. Review status and
Closure evidence are intentionally absent from these drafts.

| ID | Type/categories | Required capabilities | Surfaces/portability | Runtime/risk | Source lock |
|---|---|---|---|---|---|
| `wshobson.responsive-design` | `skill` / `frontend` | `frontend.responsive-layout`, bounded `frontend.accessibility` | `agent-skills-standard` / `portable-standard` | `instruction-only` / R0 | `wshobson/agents@c4b82b0ad771190355eb8e204b1329732a18449a`, `plugins/ui-design/skills/responsive-design`, `sha256:0c8319415cae9458b074f6306b3e3f96b2a8b7d1a4340efdd36ec35b197a312d`, MIT |
| `mohitagw.technical-spec-template` | `prompt-pack` / `backend` | `architecture.api-design` | `agent-skills-standard` / `portable-standard` | `instruction-only` / R0 | `pm-claude-skills@fddfbc4a6caa8b4d3d41a69c666efaaff9d42def`, `skills/technical-spec-template`, `sha256:ab0ad71bfa86b59235f8832f6cdd3ba2088655ea99370652f10a904816c8962e`, MIT |
| `mohitagw.database-schema-design` | `prompt-pack` / `database` | `database.schema-design`, `architecture.data-modeling` | `agent-skills-standard` / `portable-standard` | `instruction-only` / R0 | `pm-claude-skills@fddfbc4a6caa8b4d3d41a69c666efaaff9d42def`, `skills/database-schema-design`, `sha256:9824fbc8b7e52af48bfbca811c624697d8c7201bbfc187cd6fdea49a7ce5c61b`, MIT |
| `supabase.postgres-best-practices` | `skill` / `database`, `quality` | `database.migrations`, `quality.code-review` | `agent-skills-standard` / `portable-with-adapter` | `filesystem-only` / R1 | `supabase/agent-skills@1207767388a0ffb55f21fb4e6988fee96942431d`, `skills/supabase-postgres-best-practices`, `sha256:bcacf5fbb85c8c0e407ebb5eab85240349526b682ea311217da21411e2b695e9`, MIT |

The exact drafts remain evidence-only; the official copies are the three
reviewed files under `catalog/official/manifests/`, each carrying Closure-
approved `[review]` evidence. The staged profile-10 draft remains unapproved
and is not indexed.
