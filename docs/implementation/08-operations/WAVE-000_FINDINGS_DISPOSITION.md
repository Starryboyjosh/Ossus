# WAVE-000 — Finding disposition matrix

> **Historical evidence:** ADR-020 and `docs/AGENT_AUTHORITY.md` supersede this document's authority requirements for decisions on or after 2026-08-07. This note does not change recorded facts.

Companion to `WAVE-000_OPUS5_PLAN_REVIEW.md`. Prepared per `docs/implementation/07-prompts/FINDINGS_DISPOSITION.md`.

**Format note.** The prompt specifies nine columns per finding. Rendered as a nine-column table this is unreadable, so each finding is a block with the nine required fields as labelled lines. No field is omitted.

**Rules in force.** Severity is Opus 5's position and is not averaged with another agent's opinion. Critical and high findings remain blocking until evidence resolves them. Uncertainty stays visible. Accepted risk requires a named human approver and a stated reason. Agent contributions are attributed.

**Contribution attribution.** All findings, evidence and proposed changes in this matrix are Opus 5's own. No implementation agent contributed to this WAVE. Luna Max was not invoked.

**Human decision status.** Every `Human decision` field read `PENDING` while Gate S0 was open. **Gate S0 was closed on 2026-08-04** by the project owner (`holograma@unev.edu.hn`); the record is `WAVE-000_GATE_S0_CLOSURE.md`. Each field below now carries the recorded decision, and the resulting edits are applied. No finding was accepted as residual risk. Until closure no plan file, ADR, schema or WAVE file had been modified by this review — `CHANGE_CONTROL.md` requires an ADR for trusted-format, source-precedence, activation-path and security-model changes, and Gate S0 requires human resolution before the plan is edited.

---

## F-01 — Project-scoped configuration can relax user/global security policy

- **Opus 5 position:** Critical. Blocking for Gate S0.
- **Supporting evidence:** `.ossus/config.toml:17-22` exposes `[policy] risk_max`, `allow_implicit_r4`, `block`; `.ossus/policy.toml` exposes `risk_max`, `allow_modified_local`, `require_hash_verification`, `[runtime] deny`, `[surfaces] allow`. `.ossus/config.toml` assigns the project registry priority 300 above user 200 and official 100. `REGISTRY_DESIGN.md:13-21` permits higher-priority override "under explicit conflict rules" that are defined nowhere. `DATA_CONTRACTS.md:125` permits raising security-sensitive limits given "explicit configuration", which an attacker's committed file satisfies. `SUPPLY_CHAIN.md:40` confirms V0 has no source signing. A cloned repository supplies all of `.ossus/` before the user runs anything.
- **Conflicting evidence:** `THREAT_MODEL.md` names "malicious catalog overlay" as an adversary, so the general class is anticipated; its control ("namespaces, source identity and explicit override records") does not address project-scoped *configuration*, only registry entries. Gate D0 requires overrides to be "visible and attributable", which is detection after the fact. No configuration loader exists yet, so no exploit is demonstrable today.
- **Affected invariant:** "Policy denial happens before scoring or mutation"; "Approved does not mean installed. Installed does not mean active"; R4 explicit-invocation-only; R5 exclusion from stable V0; `require_hash_verification`.
- **Deterministic verification:** Adversarial golden case: a fixture project whose `.ossus/config.toml` and `.ossus/policy.toml` attempt `risk_max = "R4"`, `allow_implicit_r4 = true`, `block = []`, `require_hash_verification = false`, plus a priority-300 registry entry shadowing an `official.*` id. Assert (a) effective policy is byte-identical to the user/global policy, (b) any selection requiring the relaxation is denied with the documented policy exit code, (c) the attempted relaxation is reported in `explain` output and the audit event, (d) the shadowing entry is either refused or requires recorded trust and is attributed in output.
- **Human decision:** **DECIDED 2026-08-04 (D1): adopt.** ADR-016 recorded in `DECISION_LOG.md`. Invariant 11 added to `SYSTEM_ARCHITECTURE.md`; adversary and threat added to `THREAT_MODEL.md`; scope-as-trust-zone added to `OSSUS_MASTER_CONTEXT.md` §3.1; monotonicity criterion added to Gate S2; adversarial fixture added to WAVE-005 and to `EVALUATION_STRATEGY.md` Layer 4; `DATA_CONTRACTS.md:125` corrected. Proof is owned by Gate S2.
- **Proposed resulting action:** New **ADR-016 — Policy monotonicity for project-scoped configuration**: project-scoped configuration may only restrict, never relax, user or global policy; security-relevant keys resolve by taking the strictest value (`min` on tiers and limits, intersection on allowlists, union on denylists); a defined set of keys — `require_hash_verification`, `allow_modified_local`, `allow_implicit_r4`, `semantic_fallback`, upward parser-budget changes, registry source registration — is user/global-only and ignored with a warning if present in a project file. Add the invariant to `SYSTEM_ARCHITECTURE.md` core invariants and `OSSUS_MASTER_CONTEXT.md`. Add adversary "hostile repository supplying project-scoped Ossus configuration and registry" to `THREAT_MODEL.md`. Add the control to `SECURITY_GATES.md` Gate S2. Add the verification above to WAVE-005 required tests and to `EVALUATION_STRATEGY.md` Layer 4. Correct `DATA_CONTRACTS.md:125` to state that "explicit configuration" means user/global scope only.
- **Residual risk if accepted as-is:** A cloned repository silently obtains R4 implicit activation, R5 admission and disabled hash verification on the user's machine. This is the highest-impact unmitigated path in the plan.

---

## F-02 — Trusted taxonomy and goldens are YAML, contradicting ADR-006

- **Opus 5 position:** High. Blocking for Gate S0 because it makes WAVE-002 unimplementable as written.
- **Supporting evidence:** `specs/taxonomy/capabilities-v1.yaml` (44 IDs), `aliases-v1.yaml`, `deprecations-v1.yaml`, `evaluations/goldens/goldens-v1.yaml` (50 cases), `evaluations/seed-catalog-profiles.yaml`. `DECISION_LOG.md:39-47` (ADR-006): "TOML: trusted human-authored configuration and canonical manifests. JSON: generated indexes, schemas, machine reports and lockfiles. YAML: external import and host adapter output only." `OSSUS_MASTER_CONTEXT.md:137-145` repeats this and adds that external YAML "is never copied directly into trusted canonical state". `RUST_WORKSPACE.md:130` forbids deprecated YAML crates in the trusted core and lists no YAML crate among recommended dependencies. WAVE-002 in-scope: "Load the 44 capabilities, aliases and deprecations." WAVE-002 acceptance: "All 44 IDs load."
- **Conflicting evidence:** The taxonomy is human-authored and version-governed, so it is arguably closer to "specification" than "configuration"; ADR-006 does not name a format for specifications. The goldens are test data, and one could argue `ossus-eval` is not the trusted core — though it is a shipped crate reachable from the `ossus` binary via `ossus eval`.
- **Affected invariant:** Trusted format policy; bounded parsing; TCB minimality.
- **Deterministic verification:** After the decision, assert that `cargo tree -p ossus` contains no YAML parser (option A), or that the chosen YAML parser is pinned, is not `yaml-rust`, and is exercised by depth, string-length, item-count and file-size budget tests including alias-expansion and anchor-bomb fixtures (option B).
- **Human decision:** **DECIDED 2026-08-04 (D2): Option A — convert to TOML.** ADR-017 recorded. ADR-006 is not amended and no YAML parser enters the TCB. Conversion of `capabilities-v1`, `aliases-v1`, `deprecations-v1`, `goldens-v1`, `seed-catalog-profiles` and `model-roles` is owned by WAVE-002, which must verify 44 capability IDs, 50 golden cases and a round-trip comparison before deleting the YAML originals.
  - **Option A (Opus 5 recommendation):** Convert `capabilities-v1.yaml`, `aliases-v1.yaml`, `deprecations-v1.yaml` to TOML and `goldens-v1.yaml`, `seed-catalog-profiles.yaml` to TOML or JSON. Cost: the 50-case golden file becomes less pleasant to hand-edit. Benefit: ADR-006 holds unamended, and no YAML parser ever enters the TCB. Cheapest now, before any code reads these files.
  - **Option B:** Amend ADR-006 to permit one pinned, hardened YAML parser for trusted human-authored taxonomy and evaluation data, with mandatory budget tests. Cost: a parser class with a poor security history enters the TCB.
- **Proposed resulting action:** New **ADR-017 — Trusted format for taxonomy and evaluation data**, recording the chosen option. Owned by WAVE-002; if Option A, the conversion is added to WAVE-002 in-scope and `RUST_WORKSPACE.md` is left unchanged.
- **Residual risk if unresolved:** WAVE-002's implementer silently adds a YAML dependency to satisfy "All 44 IDs load", expanding the TCB without an ADR and without budget tests — precisely the "hide a large decision inside an implementation report" failure `CHANGE_CONTROL.md` prohibits.

---

## F-03 — `source.license` is optional in the canonical manifest schema

- **Opus 5 position:** High. Blocking for Gate S1; should be dispositioned at S0 because it is a one-line schema change.
- **Supporting evidence:** `specs/schemas/canonical-manifest.schema.json` — `source.required` is `["mode","repository","commit","tree_hash"]`; `license` is present in `source.properties` but not required. `TRUST_BOUNDARIES.md:36` requires a license decision to cross boundary C (evidence → canonical). `SUPPLY_CHAIN.md:34`: "Every indexed resource uses immutable commit/digest, canonical subpath, tree/content hash, upstream license and review record." WAVE-003 deliverable: "Seed license/source report." `catalog/imports/almanac-v0.1/registry/skills.json` records `redistribution: source-only; fetch from upstream and preserve its license` on entries, confirming license is expected per resource.
- **Conflicting evidence:** For `local-private` sources a license may be genuinely inapplicable, so an unconditional `required` may be too blunt.
- **Affected invariant:** Trust boundary C; index-first distribution; "Origin metadata is evidence, never canonical Resolver authority" (the license decision is a curator act, not an imported field).
- **Deterministic verification:** Invalid-fixture test: a manifest with `source.mode = "remote-index"` and no `source.license` fails validation with a specific reason code. Valid-fixture test: `local-private` without license passes only if the plan chooses the conditional form.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** Conditional via `if`/`then` on `source.mode != "local-private"`, recorded in the WAVE-002 in-scope list with the two required fixtures.
- **Proposed resulting action:** Schema change owned by WAVE-002 ("Maintain JSON schemas"); add the invalid fixture to WAVE-002 required tests; no ADR needed (tightening an existing field, not a contract class change).
- **Residual risk if accepted as-is:** Ossus indexes and instructs users to fetch third-party content with no recorded license, and WAVE-003's license report has no enforced input.

---

## F-04 — No crate owns the host-neutral activation transaction

- **Opus 5 position:** High. Blocking for Gate S0 because it is an architecture decision, and deciding it mid-WAVE-007 predictably lands activation logic in the CLI.
- **Supporting evidence:** WAVE-007 deliverable: "Host-neutral activation API"; in-scope includes content-addressed store, hash verification, path/symlink validation, staging, atomic swaps, ownership records, rollback, audit events. `RUST_WORKSPACE.md:105-108` documents only `{registry, resolver, policy, adapters} → core` and lists seven crates. `RUST_WORKSPACE.md:45` states `ossus-core` performs no filesystem mutation. `AGENTS.md:39`: "Keep domain logic outside `ossus-cli`." `crates/ossus-adapter-claude/Cargo.toml` depends only on `ossus-core`. WAVE-001 out-of-scope forbids creating future crates.
- **Conflicting evidence:** The content-addressed store could reasonably be seen as Registry-adjacent, so `ossus-registry` is a defensible alternative home. Nothing in the plan *forbids* adding a crate in WAVE-007; the gap is that no crate is *named*, not that one is prohibited.
- **Affected invariant:** "Installed does not mean active"; adapter trust boundary F; crate dependency direction.
- **Deterministic verification:** A workspace-structure test or CI check asserting that no filesystem-mutating API is exported from `ossus-cli` and that `ossus-adapter-claude` does not depend on the transaction crate in the mutating direction.
- **Human decision:** **DECIDED 2026-08-04 (D3): adopt.** ADR-018 recorded. The crate is created in WAVE-007, not WAVE-001; WAVE-001 still creates exactly seven crates. Direction and the adapter-supplies-a-trait rule are written into `RUST_WORKSPACE.md` and `06-waves/07-activation-security-boundary.md`.
- **Proposed resulting action:** New **ADR-018 — Activation transaction crate boundary** (`CHANGE_CONTROL.md` classes "activation path" and "adapter trust claim"). Add the crate and the dependency direction to `RUST_WORKSPACE.md`. Explicitly note the crate is created in WAVE-007, **not** WAVE-001, so WAVE-001 scope is unchanged.
- **Residual risk if unresolved:** The product's most security-critical code is written under time pressure into whichever crate is convenient, most likely `ossus-cli`, violating an `AGENTS.md` rule inside a security WAVE.

---

## F-05 — Gates S1 and S2 have no owning WAVE or closure step

- **Opus 5 position:** High. Blocking for Gate S0, whose own criterion is that assignments and ordering agree.
- **Supporting evidence:** `SECURITY_GATES.md:13` — S1 required before Registry indexing (before WAVE-003). `SECURITY_GATES.md:22` — S2 required before activation work (before WAVE-007). `PHASES_AND_GATES.md` assigns Phase 2 only Gate R0 and lists S2–S4 under Phase 3. WAVE-002 is `Security WAVE: no` and merely states "Gate S1 contract requirements pass" with no closure step. WAVE-005 implements the entire S2 control set (required tests: "Policy denial before scoring", "R4 implicit denial", "Surface mismatch", "Ambiguous low confidence") and is `Security WAVE: no` with no gate closure. `WAVE_INDEX.md` marks neither as security. `SECURITY_GOVERNANCE.md` requires human closure of every security gate.
- **Conflicting evidence:** None found. This appears to be an omission rather than a deliberate choice.
- **Affected invariant:** Gate ordering; "a human closes every security gate".
- **Deterministic verification:** A repository-layout or documentation check asserting every gate named in `SECURITY_GATES.md` appears as a closure deliverable in exactly one WAVE file. `scripts/check-repository-layout.py` is the natural host and is currently not run by CI (see F-14).
- **Human decision:** **DECIDED 2026-08-04 (D4): adopt.** Gate S1 owner WAVE-002, Gate S2 owner WAVE-005, both requiring Opus 5 review of the final diff and evidence plus human closure. Recorded in `SECURITY_GATES.md`, `PHASES_AND_GATES.md`, `WAVE_INDEX.md` (new **Closes gate** column) and both WAVE files. Implementer assignments unchanged.
- **Proposed resulting action:** Edit `WAVE_INDEX.md`, `PHASES_AND_GATES.md`, `docs/implementation/06-waves/02-spec-and-taxonomy.md` and `05-resolver-core.md` to add the closure deliverable and the human checklist reference.
- **Residual risk if accepted as-is:** WAVE-003 begins Registry indexing without S1 closed and WAVE-007 begins activation without S2 closed, and nobody notices because no WAVE file mentions them.

---

## F-06 — External-origin archive and extracted index committed in the privileged repository

- **Opus 5 position:** Medium-high. Not independently blocking, but it must be either recorded as an ADR-012 exception or relocated before WAVE-001 initializes Git, because `git init` makes the current tree the repository's first commit.
- **Supporting evidence:** `Ossus_v0.1_Almanac.zip` at repository root, 26 files, SHA-256 `dbd449e7…b097ef` (verified), including `install/fetch_skills.py` (7,221 bytes, network fetcher), `install/verify_registry.py`, `ossus.py`, and `overlays/*.md` transformation directives. `.gitignore` does not exclude the archive or `catalog/imports/`. `catalog/imports/almanac-v0.1/registry/skills.json` (22,630 bytes, 50 entries) carries upstream-controlled `notes`, `aliases`, `overlay` and `install_strategy` fields. `ADR-012`: "External candidates never enter a privileged branch of the main Registry repository." `STAGING_AND_CI.md:13`: "Candidate content never enters a privileged branch merely to be reviewed." `README.md:135` describes `catalog/` as "Canonical Registry layout; no candidates".
- **Conflicting evidence:** The import README states plainly that these entries are not canonical and lack immutable commit/tree hashes, which is honest and correct. No Rust source references `catalog/` (verified by grep across `crates/`). CI does not execute the Python. The archive hash is recorded and verifies. A defensible reading is that an index of names and URLs is *evidence*, not *candidate content*, and ADR-012 does not explicitly address that distinction.
- **Affected invariant:** Trust boundary A (external source → quarantine); "Stored does not mean approved"; ADR-012 staging separation.
- **Deterministic verification:** CI check asserting no crate source or build script references `catalog/imports/**` or `*.zip`; packaging check asserting these paths are excluded from any release artifact; layout check asserting `catalog/` contains no non-canonical entries or that `catalog/imports/` is explicitly declared non-canonical.
- **Human decision:** **DECIDED 2026-08-04 (D5): option (b), with one deviation.** `catalog/imports/` and `Ossus_v0.1_Almanac.zip` were moved to the untracked `research-evidence/almanac-v0.1-import/`; `.gitignore` now blocks archives and `catalog/imports/`; `catalog/README.md` and `catalog/official/README.md` corrected; the WAVE-001 layout-check job enforces the exclusion. **The archive was relocated, not deleted** — exclusion from permanent git history is the security requirement, and destroying the only local copy of recorded evidence is irreversible. `README.md:135` needed no correction: the relocation made its claim true.
- **Proposed resulting action:** Whichever option is chosen, correct `README.md:135` so the stated layout matches the tree, and add the exclusion checks before WAVE-001 runs `git init`.
- **Residual risk if accepted as-is:** External-origin executable Python is committed to the trusted repository's history permanently, where a future release-packaging glob, CI job or contributor can reach it, and the README's own trust claim is false.

---

## F-07 — Set-level activation atomicity and locking model unspecified

- **Opus 5 position:** Medium. Not blocking for S0; must be resolved in the WAVE-007 design before implementation begins.
- **Supporting evidence:** `SYSTEM_ARCHITECTURE.md:106` describes an atomic swap. `ACTIVATION_SECURITY.md:14` requires replacing only Ossus-managed paths; `:24` targets controlled subdirectories under `.claude/skills/`; `:50` allows a documented two-phase strategy with crash-recovery tests. `CLI_CONTRACT.md:119` and `PHASES_AND_GATES.md:82` forbid deleting user-authored host skills. Host discovery requires `SKILL.md` at `.claude/skills/<name>/`, so an Ossus-owned parent directory that could be renamed atomically would not be discovered. `SOURCE_AND_INSTALLATION_MODEL.md:48-50` allows a shared custom store path "provided permissions and concurrency are handled" with no owner for that clause. No lock, stale-lock recovery, or sync-during-resolve snapshot contract exists anywhere. `ossus doctor` references "stale transactions".
- **Conflicting evidence:** WAVE-007 already requires the right *tests* — "TOCTOU scenarios where practical", "Crash rollback", "Concurrent activation lock". The gap is design, not intent.
- **Affected invariant:** Gate S3 "failed activation preserves prior state"; "Installed does not mean active".
- **Deterministic verification:** Crash injection between per-resource renames asserting the prior active set is fully restored; two concurrent `ossus activate` runs in one project asserting one succeeds and one fails with a lock reason code; `registry sync` during an in-flight resolve asserting the resolve completes against its snapshot.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** A `Concurrency, atomicity and recovery` section is appended to `ACTIVATION_SECURITY.md` and is named a design precondition of WAVE-007, with the four required evidence items added to that WAVE.
- **Proposed resulting action:** Extend `ACTIVATION_SECURITY.md` with a concurrency and recovery section before WAVE-007: declare the transaction unit to be the whole active set; specify a journalled multi-rename with idempotent rollback; require an exclusive lock on the project `.ossus/` and on `$OSSUS_HOME/transactions` with a documented stale-lock policy; require hash re-verification against staged bytes rather than store bytes to close the TOCTOU window; assign the shared-store concurrency clause an owner.
- **Residual risk:** A crash or a concurrent run leaves a partially-active set that satisfies no policy check, and Gate S3's rollback criterion is met only for single-resource activations.

---

## F-08 — `source.mode` and `distribution.mode` are overlapping, divergent enums

- **Opus 5 position:** Medium.
- **Supporting evidence:** Schema: `source.mode ∈ {remote-index, vendored, local-private}`; `distribution.mode ∈ {index-only, vendored, private-local}`. Same three concepts, two vocabularies, no cross-field constraint. `source.mode = "vendored"` with `distribution.mode = "index-only"` validates.
- **Conflicting evidence:** The two may be intended to differ — where content comes from versus how Ossus may redistribute it. If so, `distribution` needs a redistribution vocabulary (as `catalog/imports/.../skills.json` uses: `source-only`, `approved-install-only`), not a near-duplicate of `source.mode`.
- **Affected invariant:** Data-contract determinism; index-first distribution.
- **Deterministic verification:** Semantic-validation test rejecting contradictory pairs; or, if the fields are redefined, fixtures for each legal combination.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** Recorded in the WAVE-002 in-scope list; contradictory pairs must be rejected by semantic validation.
- **Proposed resulting action:** Schema and `DATA_CONTRACTS.md` change owned by WAVE-002.
- **Residual risk:** Two fields disagree in production manifests and downstream code picks whichever it read first.

---

## F-09 — `source.commit` accepts 40–64 mixed-case hex

- **Opus 5 position:** Medium — a fail-open weakness in a conflict-detection control.
- **Supporting evidence:** Schema pattern `^[0-9a-fA-F]{40,64}$` admits every length 40–64 and both cases, so one commit has many valid spellings. `tree_hash` is correctly `^sha256:[0-9a-f]{64}$` — lowercase-only and exact-length — which demonstrates the intended strictness. WAVE-003 requires detecting "same-version hash conflicts"; naive string comparison across case variants misses them.
- **Conflicting evidence:** None. This reads as an oversight.
- **Affected invariant:** Immutable source reference; supply-chain pinning.
- **Deterministic verification:** Invalid fixtures for 41-, 50- and 63-character values and for uppercase values; a conflict-detection test using two case variants of the same commit.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** Pattern and lowercase normalization recorded in WAVE-002; the case-variant conflict fixture is recorded in WAVE-003.
- **Proposed resulting action:** Schema change owned by WAVE-002; conflict fixture added to WAVE-003.
- **Residual risk:** Case-variant duplicates evade same-version hash-conflict detection.

---

## F-10 — `compatibility.surfaces` is an unconstrained string array

- **Opus 5 position:** Medium.
- **Supporting evidence:** Schema: `surfaces` is `array of string`, `minItems: 1`, `maxItems: 16`, no enum and no `maxLength`, while `portability`, `scopes`, `runtime.requirements`, `risk.tier`, `review.status` and `review.tier` all carry enums. `HOST_COMPATIBILITY.md:5-15` defines exactly nine surfaces. `.ossus/policy.toml` allowlists two. WAVE-002 requires rejecting "unmapped capabilities" and says nothing about unmapped surfaces. WAVE-005 requires a "surface mismatch" test; `ossus search --surface` filters on the value.
- **Conflicting evidence:** Surfaces may be intended to grow faster than schema releases, as capability IDs do — but capability IDs have `CAPABILITY_GOVERNANCE.md` and explicit WAVE-002 semantic validation, and surfaces have neither.
- **Affected invariant:** Compatibility precision; "no false portability".
- **Deterministic verification:** Invalid fixture with `claude-code-CLI` and with `not-a-surface`, asserting rejection with a specific reason code.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** Recorded in the WAVE-002 in-scope list with the two rejection fixtures.
- **Proposed resulting action:** Schema and WAVE-002 required-test change.
- **Residual risk:** A typo silently narrows or widens matching and no test detects it.

---

## F-11 — Forward-compatibility policy contradicts `additionalProperties: false`

- **Opus 5 position:** Medium — documentation defect with a real forward-compatibility cost.
- **Supporting evidence:** `DATA_CONTRACTS.md:131` states unknown optional fields in the same major version "are preserved where safe but not allowed to affect resolution until understood". The schema sets `additionalProperties: false` at every level, so such fields are rejected outright. `CAPABILITY_GOVERNANCE.md` describes MINOR versions as additive, which does not hold for manifests under strict rejection.
- **Conflicting evidence:** Strict rejection is the safer behaviour and should be kept; only the documentation is wrong.
- **Affected invariant:** Fail-closed validation; version compatibility policy.
- **Deterministic verification:** Test asserting an unknown top-level key is rejected — which doubles as the F-12 negative fixture.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** `DATA_CONTRACTS.md` corrected; WAVE-002 explicitly instructed not to relax `additionalProperties`.
- **Proposed resulting action:** Documentation change; no schema change.
- **Residual risk:** An implementer "fixes" the mismatch by relaxing the schema, which `AGENTS.md` prohibits but the contradiction invites.

---

## F-12 — No origin/evidence data contract exists

- **Opus 5 position:** Medium.
- **Supporting evidence:** WAVE-002 required test: "Origin fields cannot populate canonical-only state." `SECURITY_GATES.md:13` Gate S1: "canonical/origin separation explicit." No origin or evidence-bundle schema exists in `specs/`. `RESEARCHER_FUTURE_DESIGN.md` describes evidence conceptually only.
- **Conflicting evidence:** The Researcher is Phase 5, so a full evidence contract now would be premature and possibly wrong.
- **Affected invariant:** "Origin metadata is evidence, never canonical Resolver authority"; trust boundary B.
- **Deterministic verification:** Negative-fixture set asserting that `origin`, `author_capabilities`, `upstream_triggers` and `stars` as top-level or nested keys are rejected; plus a test that any origin record the importer produces is stored outside canonical state.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** The negative-fixture set is recorded in WAVE-002 required tests; the full evidence-bundle schema is deferred to WAVE-017.
- **Proposed resulting action:** WAVE-002 required-tests edit.
- **Residual risk:** Gate S1's separation criterion is closed on a tautological test.

---

## F-13 — `rust-toolchain.toml` pins a floating channel; CI never tests a pinned toolchain

- **Opus 5 position:** Medium. In WAVE-001 scope.
- **Supporting evidence:** `rust-toolchain.toml` sets `channel = "stable"` — a moving target, not a pin. `.github/workflows/ci.yml` installs `stable` only. `RUST_WORKSPACE.md:136-137`: "`rust-toolchain.toml` pins a reviewed stable toolchain for releases. CI tests the pinned toolchain and current stable." WAVE-001 in-scope: "Configure Rust 2024, a pinned stable toolchain". Local toolchain is 1.97.1; `Cargo.toml` declares `rust-version = "1.85"` and `clippy.toml` `msrv = "1.85.0"`, which agree with each other. WAVE-001 deliverable "MSRV ADR proposal" does not exist; `DECISION_LOG.md` has no MSRV ADR.
- **Conflicting evidence:** None.
- **Affected invariant:** Reproducible builds; supply-chain pinning.
- **Deterministic verification:** `rust-toolchain.toml` matches `^\d+\.\d+\.\d+$`; CI matrix contains both the pinned version and `stable`; `cargo build` succeeds at the declared MSRV.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** Recorded as a WAVE-001 repair item with the pinned job required and the floating-`stable` job advisory.
- **Proposed resulting action:** WAVE-001 implementation task (assigned to Luna Max).
- **Residual risk:** A stable release silently changes lints or codegen and CI cannot reproduce release builds.

---

## F-14 — Mutable action references; configured `deny.toml` has no CI job

- **Opus 5 position:** Medium. Partly in WAVE-001 scope, fully closed at Gate S4.
- **Supporting evidence:** `.github/workflows/ci.yml` uses `actions/checkout@v6` — a mutable tag. `SUPPLY_CHAIN.md:11` requires source pinning; Gate S4 requires a workflow pinning policy; `STAGING_AND_CI.md:47` requires restricted workflow modification. `deny.toml` is fully configured (license allowlist, `wildcards = "deny"`, `unknown-registry = "deny"`, `unknown-git = "deny"`) with no workflow invoking `cargo deny`. `scripts/check-repository-layout.py` is also not run by CI. Good baseline: `permissions: contents: read`, `on: pull_request` (not `pull_request_target`), no secrets, ubuntu/macos/windows matrix.
- **Conflicting evidence:** There are currently zero third-party dependencies, so `cargo deny` would check nothing today. WAVE-010 owns supply-chain hardening.
- **Affected invariant:** Supply-chain pinning; least-privilege CI.
- **Deterministic verification:** Workflow lint asserting every `uses:` is a 40-character SHA with a version comment; a `cargo deny check` job; a layout-check job.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** Recorded as a WAVE-001 repair item, including the D5 layout-check job; the written pinning policy and drift detection remain with WAVE-010.
- **Proposed resulting action:** WAVE-001 implementation task; note in WAVE-010 that pinning policy is already partially satisfied.
- **Residual risk:** A compromised action tag executes in CI; a dependency with a disallowed license enters unnoticed at WAVE-002.

---

## F-15 — No transport allowlist for `source.repository`

- **Opus 5 position:** Medium. Specify now, enforce at WAVE-012.
- **Supporting evidence:** Schema: `repository` is `string`, `maxLength: 512`, no pattern or scheme constraint, while `source.mode = "remote-index"` means Ossus directs a fetch to that value. No document defines an allowed transport set. `SOURCE_AND_INSTALLATION_MODEL.md` and `THREAT_MODEL.md` are silent on `file://`, `git://` and SSH-with-agent-forwarding.
- **Conflicting evidence:** Install is WAVE-012, well after V0, so enforcement is not urgent. Manifests are curator-authored, so a hostile value requires a curator error or a compromised registry — but F-01 shows a project-scoped registry can be attacker-supplied.
- **Affected invariant:** Trust boundary A; network default.
- **Deterministic verification:** Invalid fixtures for `file://`, `git://`, `http://` and `ssh://` under `remote-index`; valid fixture for `https://`.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** The `Source transport allowlist` section is added to `DATA_CONTRACTS.md`; enforcement and the per-scheme rejection fixtures are recorded in WAVE-012.
- **Proposed resulting action:** Documentation change now; schema and enforcement at WAVE-012.
- **Residual risk:** An unauthenticated or local-file transport is used for a "remote" source with no integrity guarantee beyond the recorded hash.

---

## F-16 — Repository not initialized; `cargo fmt --check` fails

- **Opus 5 position:** Low-medium. Entirely within WAVE-001 scope; recorded so the WAVE-001 baseline is honest.
- **Supporting evidence:** `git rev-parse --is-inside-work-tree` → `fatal: not a git repository`. `cargo fmt --all -- --check` produces 5 diff hunks: four in `crates/ossus-cli/src/main.rs` (≈ lines 38, 77, 106, 116) and one in `crates/ossus-resolver/src/lib.rs:21` (`use super::{component_state, DEFAULT_ACTIVE_RESOURCE_LIMIT};` → `use super::{DEFAULT_ACTIVE_RESOURCE_LIMIT, component_state};`). `scripts/verify.sh` therefore stops before Clippy and tests. This reproduces the baseline recorded in `WAVE-000_PREPARATION_AND_CHANGE_SUMMARY.md`. Run independently, Clippy and the 9 tests pass.
- **Conflicting evidence:** None. WAVE-001 in-scope item 1 is "Initialize Git and the Cargo workspace" and its acceptance requires a committed `Cargo.lock`, which is impossible without a repository.
- **Affected invariant:** Quality baseline; WAVE-001 acceptance criteria.
- **Deterministic verification:** `scripts/verify.sh` runs to completion; `git status` succeeds; `git ls-files Cargo.lock` returns the file.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** Recorded as a WAVE-001 repair item with `git status` and a passing `cargo fmt --all -- --check` as required evidence.
- **Proposed resulting action:** WAVE-001 implementation task.
- **Residual risk:** None once fixed.

---

## F-17 — CLI exit code 69 outside the stable table; global flags unparsed

- **Opus 5 position:** Low. WAVE-001 scope.
- **Supporting evidence:** `crates/ossus-cli/src/main.rs` defines `EXIT_NOT_IMPLEMENTED: u8 = 69`. `CLI_CONTRACT.md:135-151` defines the stable table `0, 2, 10, 11, 12, 20, 21, 30, 31, 32, 40, 41, 50, 60, 70` and states exit codes are stable public API after V0. The contract's global flags (`--config --project --format --no-color --quiet --verbose --offline --yes`) are neither parsed nor mentioned in help; `ossus --format json status` is treated as unknown command `--format` and exits 2.
- **Conflicting evidence:** WAVE-001 scope is only "help and placeholder command groups", so unimplemented flags are acceptable; the undocumented exit code is not.
- **Affected invariant:** CLI contract stability.
- **Deterministic verification:** A test asserting every exit code the binary can emit appears in the contract table; help snapshot includes the global-flags section.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** Recorded as a WAVE-001 repair item; the code must be added to `CLI_CONTRACT.md` and its mirror, marked valid only while a command group is a placeholder.
- **Proposed resulting action:** WAVE-001 implementation task plus a one-line `CLI_CONTRACT.md` addition.
- **Residual risk:** An undocumented exit code becomes stable public API by accident at V0.

---

## F-18 — `unwrap_used`/`expect_used = "deny"` applies to test targets

- **Opus 5 position:** Low, but worth deciding now to remove later pressure on a security lint.
- **Supporting evidence:** `Cargo.toml` `[workspace.lints.clippy]` sets `expect_used = "deny"` and `unwrap_used = "deny"`; all seven crates opt in via `[lints] workspace = true`; `cargo clippy --all-targets` covers test targets. Current tests use only `assert_eq!` so nothing fires yet. From WAVE-002 onward, fixture-loading tests will hit this constantly.
- **Conflicting evidence:** Denying `unwrap` in tests has real value — a panicking test helper can mask a failure mode.
- **Affected invariant:** "Never weaken a security policy merely to make tests pass" (`AGENTS.md`).
- **Deterministic verification:** Whatever convention is chosen, `cargo clippy --workspace --all-targets --all-features -- -D warnings` must stay green without editing `[workspace.lints]`.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** The convention is documented in `RUST_WORKSPACE.md` under `Lint policy in test targets`, with WAVE-001 instructed not to relax the workspace lint.
- **Proposed resulting action:** Convention recorded in `AGENTS.md` or `RUST_WORKSPACE.md` during WAVE-001.
- **Residual risk:** A future implementer relaxes the workspace lint to make tests compile.

---

## F-19 — Documented crate dependency direction omits existing edges; root `tests/` absent

- **Opus 5 position:** Low.
- **Supporting evidence:** Actual: `resolver → {core, policy, registry}`, `eval → {core, registry, resolver}`, `cli → all six`. `RUST_WORKSPACE.md:105-108` documents only `{registry, resolver, policy, adapters} → core` plus `cli → all`. `RUST_WORKSPACE.md:23` lists a root `tests/` directory that does not exist.
- **Conflicting evidence:** The extra edges are correct and desirable — `resolver → policy` is what makes "policy denial precedes scoring" structural rather than a CLI convention. Only the documentation is stale.
- **Affected invariant:** "Preserve crate dependency direction" (`AGENTS.md:41`).
- **Deterministic verification:** A workspace-graph test asserting the actual edge set equals the documented edge set.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** The complete edge set is written into `RUST_WORKSPACE.md` with an explicit "an edge not listed here is a boundary violation" clause; `tests/` is a WAVE-001 deliverable.
- **Proposed resulting action:** WAVE-001 documentation and structure task.
- **Residual risk:** A reviewer cannot distinguish an intended edge from a boundary violation.

---

## F-20 — WAVE-001's required CLI help snapshot tests are absent

- **Opus 5 position:** Low. Directly blocks WAVE-001 acceptance.
- **Supporting evidence:** WAVE-001 required evidence: "CLI help snapshot tests." `crates/ossus-cli/src/main.rs` contains three in-process unit tests (`help_succeeds`, `unknown_command_fails_with_usage_code`, `future_command_is_explicitly_unavailable`) that call `run()` and assert exit codes only; no output is captured or snapshotted, and the compiled binary is never executed. `RUST_WORKSPACE.md:127` lists `assert_cmd`, `predicates` and `insta` for exactly this.
- **Conflicting evidence:** None.
- **Affected invariant:** WAVE-001 acceptance; CLI contract stability.
- **Deterministic verification:** `tests/cli_help.rs` invoking the built binary via `assert_cmd`, snapshotting `--help`, `--version` and one placeholder command's stderr with `insta`, and asserting exit codes against `CLI_CONTRACT.md`.
- **Human decision:** **DECIDED 2026-08-04 (D6): accept as proposed.** Recorded as a WAVE-001 repair item. `assert_cmd` and `insta` are the workspace's first third-party dependencies, which is exactly why the F-14 `cargo deny` job is added in the same WAVE.
- **Proposed resulting action:** WAVE-001 implementation task.
- **Residual risk:** Help and version output drift silently from the CLI contract.

---

## Summary

| Severity | Count | IDs |
|---|---|---|
| Critical | 1 | F-01 |
| High | 4 | F-02, F-03, F-04, F-05 |
| Medium-high | 1 | F-06 |
| Medium | 9 | F-07 … F-15 |
| Low-medium | 1 | F-16 |
| Low | 4 | F-17, F-18, F-19, F-20 |
| **Total** | **20** | |

Gate S0 acceptance requires no unresolved critical finding. F-01 must be dispositioned by the named human approver before WAVE-001 begins.
