# WAVE-003 seed profile reconciliation

**Date:** 2026-08-08
**Authority:** ADR-020; Curator and independent Admission Review evidence are
advisory until a distinct Closure Agent records the decision.  The profile
status below is not, by itself, an admission record.  A candidate is admitted
only after the separate
Curator → independent review → Closure chain and a canonical manifest.

## Decision rule

The original twenty entries are a coverage objective, not an acceptance quota.
Profile correctness and candidate correctness are recorded separately.  A
candidate that exceeds a risk, type, source, or surface contract is rejected;
the profile is amended only when repeated ecosystem evidence shows that the
profile itself is wrong or unnecessarily restrictive.  No profile is lowered
from R0/R1 merely to make a candidate fit.

The resulting **provisional admission-bearing target is 16 resources**.  The
four intentionally unresolved profiles (10, 17, 18, and 20) remain governed
decisions, but do not create admission pressure or count toward that provisional
target.
This target is not a completion claim: each of the 16 still needs a legitimate
admission, and Closure may change the target through an explicit architecture
decision.

## Profile decisions

| Profile | Original requirement | Evidence | Candidate status | Profile status | Proposed/accepted change | Security impact | Final disposition |
|---:|---|---|---|---|---|---|---|
| 1 | `skill`, frontend visual design, standard + Claude + Codex, R0 | Microsoft design-review source performs active design/code work (R3). A passive visual-design review remains feasible. | Current candidate rejected; profile still fillable. | `CANDIDATE_REJECTED_PROFILE_VALID` | No change; replace with a genuinely instruction-only cross-host source. | Preserves R0 and prevents active tooling from being laundered as passive review. | Candidate rejected, profile valid |
| 2 | `skill`, responsive layout + accessibility, standard + Claude + Codex, R0 | `wshobson/agents` subtree is immutable MIT Markdown with credible multi-harness evidence; no disposable Ossus host activation/materialization evidence yet. | Conditional; not admitted. | `UNCHANGED` | Retain all surfaces and R0; add host-fixture evidence. | No relaxation; unsupported host claims remain excluded. | Profile unchanged |
| 3 | `skill`, frontend implementation, standard + Claude, R1 | Anthropic source directs the host to write code; ordinary activation is R2, above the bounded R1 ceiling. | Candidate rejected for this R1 slot. | `CANDIDATE_REJECTED_PROFILE_VALID` | Keep R1. Seek bounded, read-only/proposed-diff guidance rather than raising risk for one candidate. | Avoids granting project-write authority under R1. | Candidate rejected, profile valid |
| 4 | `skill`, frontend performance, Claude + Codex, R1 | `premium-frontend-ui` is generative UI with CDN/dependency/browser behavior, not bounded performance review. | Candidate rejected. | `CANDIDATE_REJECTED_PROFILE_VALID` | Keep R1 and surfaces; replace with static project-file performance review. | Excludes network, installation, browser automation, and automatic changes. | Candidate rejected, profile valid |
| 5 | `skill`, visual/browser validation, Claude, R2 | Playwright/MCP, URL navigation, screenshots, network inspection, and local Node installation are intrinsic to E2E validation. | Candidate blocked pending a complete security admission. | `PROFILE_SUBSTITUTION_PROPOSED` | Amend maximum risk to R3 only with local-origin defaults, URL/egress allowlists, no credentials/cookies, no destructive actions, disposable runner, and explicit output paths. | Expands risk deliberately and adds controls; never infer safety from the title. | Proposed R3 amendment |
| 6 | `prompt-pack`, API design, standard + Claude + Codex, R0 | Locked Mohit subtree is static standard `SKILL.md`; Claude/Codex behavior is not proven. | Admitted after Closure-approved standard-only substitution. | `PROFILE_SUBSTITUTION_ACCEPTED` | Surface substitution to `agent-skills-standard` only; preserve type, capability, category, and R0. It does not count toward aggregate cross-host diversity. | No risk increase; compatibility claims are narrower. | Accepted substitution; admitted |
| 7 | `skill`, backend API implementation, Claude + Codex, R1 | ASP.NET source invokes `dotnet`, writes code, builds/tests, and consults current web material; normal behavior is R3. | Candidate blocked pending bounded adapter/security review. | `PROFILE_SUBSTITUTION_PROPOSED` | Amend maximum risk to R3 only with command/network limits, no deployment/production DB/secrets/auth changes, and explicit confirmation boundaries. | Requires R3 review and removes implicit host/network authority. | Proposed R3 amendment |
| 8 | `skill`, authentication/security, Claude, R1 | Investigated candidate elevates policy and handles raw secrets. | Candidate rejected; profile remains useful. | `CANDIDATE_REJECTED_PROFILE_VALID` | No change; replace with a bounded identity/access review resource. | Keeps secret handling and policy controls strict. | Candidate rejected, profile valid |
| 9 | `prompt-pack`, schema/data modeling, standard + Claude + Codex, R0 | Locked Mohit subtree is static standard `SKILL.md`; host-specific behavior is unproven. | Admitted after Closure-approved standard-only substitution. | `PROFILE_SUBSTITUTION_ACCEPTED` | Surface substitution to `agent-skills-standard` only; retain R0 and capabilities; it does not count toward aggregate cross-host diversity. | No risk increase; no invented Claude/Codex compatibility. | Accepted substitution; admitted |
| 10 | `skill`, migration/code review, Claude + Codex, R1 | Supabase subtree is broad, standard-only, and lacks an immutable read-only adapter; it discusses schema/migration changes, restores, imports, and diagnostics. | Candidate not admissible. | `INTENTIONALLY_UNRESOLVED` | No amendment now. Reconsider only with a physically enforced read-only adapter and direct host evidence. | Fail-closed; metadata exclusions cannot prove R1. | Intentionally unresolved |
| 11 | `skill`, unit testing, standard + Claude + Codex, R0 | Superpowers TDD writes/deletes project code and runs repository commands; active behavior is intrinsic. | Candidate blocked under original R0. | `PROFILE_SUBSTITUTION_PROPOSED` | Amend maximum risk to R3 only with scoped writes, disposable execution, no dependency install/network by default, expected-diff/rollback checks, and security review. | Makes active test authority explicit instead of hiding it in R0. | Proposed R3 amendment |
| 12 | `skill`, integration testing, Claude + Codex, R1 | Current candidate has references outside the selected tree and browser/MCP execution; genuine integration testing is R3, not R1. | Current candidate blocked pending a profile amendment or a strictly passive replacement. | `PROFILE_SUBSTITUTION_PROPOSED` | Propose an explicit R3 executable-test amendment (disposable runner, local-origin/egress controls, bounded commands/time/actions, no credentials) or split to a read-only planning profile; do not silently lower risk or claim host compatibility. | Makes execution authority explicit and adds runner/network controls. | Proposed R3 amendment/split |
| 13 | `skill`, code review, standard + Claude + Codex, R0 | Current source escapes the selected tree and directs merge/test/audit behavior (R3). A passive code-review prompt remains possible. | Current candidate rejected. | `CANDIDATE_REJECTED_PROFILE_VALID` | Retain R0; replace with self-contained, passive review instructions. | Protects R0 from shell, tests, and repository mutation. | Candidate rejected, profile valid |
| 14 | `prompt-pack`, threat modeling, standard + Claude + Codex, R0 | OpenAI template includes optional `rg` repository exploration (R3) and only direct Codex/standard evidence. | Current candidate rejected for the R0 cross-host slot. | `CANDIDATE_REJECTED_PROFILE_VALID` | Keep passive, self-contained R0 prompt-pack and required surfaces. | No shell/filesystem/network activity is allowed under the profile. | Candidate rejected, profile valid |
| 15 | `skill`, dependency audit CLI, `standalone-cli`, R3 | The investigated GitHub security-review source is an Agent Skill, not a CLI. Prior Closure accepted only a documented surface substitution. | Substitution candidate remains unadmitted; no standalone CLI exists yet. | `PROFILE_SUBSTITUTION_ACCEPTED` | Accepted profile correction: `agent-skills-standard` static dependency review; never relabel it as `standalone-cli`. Record lost scanner freshness/transitive/JSON automation. | Narrows behavior and avoids false CLI/host claims; the original CLI coverage remains unfilled. | Accepted substitution; admission pending |
| 16 | `skill`, supply-chain review, Claude, R1 | Microsoft hve-core subtree has composite licensing, material external references, and no direct Claude evidence. | Candidate rejected for admission; replacement search may continue. | `CANDIDATE_REJECTED_PROFILE_VALID` | Retain profile, R1, and Claude requirement. Only a self-contained, clearly licensed, directly Claude-validated replacement may fill it. | No license or compatibility relaxation; fail closed. | Candidate rejected, profile valid |
| 17 | `skill`, container review, Claude + Codex, R1 | GitHub Dockerfile candidate is semantically R1/read-only, but the current proposal lacks complete independent tuple/hash/license/content/capability and host evidence. | Candidate held; fresh complete evidence required. | `INTENTIONALLY_UNRESOLVED` | Retain profile and R1; do not infer capability from repository naming. | Keeps Dockerfile guidance untrusted and prevents auto-build/write/secret insertion. | Intentionally unresolved |
| 18 | `skill`, CI review, Claude, R1 | GitHub Actions candidate is semantically R1/read-only, but the current proposal lacks complete independent tuple/hash/license/reference/content and Claude evidence. | Candidate held; fresh complete evidence required. | `INTENTIONALLY_UNRESOLVED` | Retain profile and R1; require complete immutable scope and host evidence. | Prevents workflow/network behavior entering R1 without proof. | Intentionally unresolved |
| 19 | `prompt-pack`, technical writing, standard + Claude + Codex, R0 | Xamfonos source is a Claude-only `SKILL.md` and directs tutorial/test execution (R3), not a passive prompt pack. | Candidate rejected. | `CANDIDATE_REJECTED_PROFILE_VALID` | Keep R0, prompt-pack, and cross-host requirements; seek passive self-contained writing guidance. | Avoids type and runtime laundering. | Candidate rejected, profile valid |
| 20 | `mcp-server`, generic MCP + Codex, R3 | Original calculator rejected. `slettmayer/calc-mcp-server` is bounded/stdio/MIT but has unpinned build inputs, release metadata mismatch, missing Codex evidence, logging and timeout concerns. | Replacement conditional and blocked. | `INTENTIONALLY_UNRESOLVED` | Do not lower security. Reconsider only after reproducible build provenance, narrow release scope, explicit timeout/logging controls, and direct surface evidence; otherwise propose a generic-only amendment through architecture. | Keeps MCP execution fail-closed and prevents quota-driven admission. | Intentionally unresolved |

## What the evidence says about profile design

The ecosystem evidence identifies three recurring specification problems:

1. Active implementation, browser validation, API implementation, and TDD are
   not R0/R1 passive reviews. Profiles 5, 7, and 11 therefore have proposed R3
   amendments with explicit controls; no candidate is admitted by the proposal.
2. A standard `SKILL.md` package does not prove Claude, Codex, or CLI
   compatibility. Profiles 6, 9, and 15 narrow their surfaces rather than
   inventing host support. Profile 15's accepted substitution still does not
   admit a resource or fill the original standalone-CLI coverage. Profiles 6
   and 9 are the two exceptions: their separate Closure record authorizes the
   official manifests.
3. Incomplete references, composite licensing, raw-secret handling, and
   unbounded build/runtime behavior are admission failures, not reasons to
   lower a profile's requirements. Profiles 1, 2, 3, 4, 8, 10, 12, 13, 14,
   16, 17, 18, 19, and 20 retain valid coverage goals while their current
   candidates remain out.

No profile is currently marked `PROFILE_REDUNDANT` or
`PROFILE_REMOVAL_PROPOSED`. Removal would require evidence that a capability is
duplicated after admission, not merely that the first candidate failed.

## Catalog coverage and next decision

The profile ledger now has 20 governed dispositions, 16 provisional
admission-bearing slots, and four intentionally unresolved slots. Official
Registry entries now number **2** (profiles 6 and 9). The next authorized work is to turn the
strongest evidence bundles into independent reviews and Closure decisions,
starting with profiles 2, 6, and 9, without treating the provisional target as
a quota. A profile substitution is a design decision; it is not a resource
admission.

The WAVE remains in progress because the remaining target has not completed the
full authority chain and hosted FTS5 evidence is still pending.
