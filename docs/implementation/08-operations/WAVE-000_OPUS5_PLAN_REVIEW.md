# WAVE-000 — Opus 5 adversarial security and architecture plan review

**WAVE:** WAVE-000
**Reviewer:** Opus 5 (`claude-opus-5`), acting as `security_implementer` per `specs/config/model-roles.yaml`
**Implementation-agent contribution:** none. No Luna Max or other implementation-agent output was used as evidence for this review.
**Date:** 2026-08-04
**Base commit:** none — the working tree is not a Git repository (`git rev-parse --is-inside-work-tree` → `fatal: not a git repository`)
**Working tree state at review:** unmodified; this review created no product code and no repository.
**Prompt executed:** `docs/implementation/07-prompts/PLAN_REVIEW_OPUS5.md`

## Verdict

**REVISE.**

One critical finding (F-01) and four high findings (F-02, F-03, F-04, F-05) must be dispositioned by the human before WAVE-001 begins. None of them is unfixable, and none invalidates the product architecture. They are plan-level defects of exactly the kind Gate S0 exists to catch: an unclosed trust boundary in the configuration layer, a trusted-format contradiction that makes WAVE-002 unimplementable as written, a mandatory supply-chain control that the schema does not enforce, an unassigned home for the most security-critical component, and two security gates with no owning WAVE.

Gate S0 cannot be closed on model review alone. Per `docs/implementation/06-waves/00-plan-security-review.md`, model review is evidence, not certification. The human closure package is `WAVE-000_GATE_S0_CLOSURE.md`.

## Method and evidence base

Reviewed in full: `AGENTS.md`, `README.md`, `CLAUDE.md`, all of `docs/product/`, `docs/architecture/`, `docs/security/`, `docs/roadmap/`, `docs/HOST_COMPATIBILITY.md`, `docs/implementation/` (waves, prompts, specifications, evaluations, operations), `specs/schemas/canonical-manifest.schema.json`, `specs/config/model-roles.yaml`, `specs/taxonomy/`, `.ossus/config.toml`, `.ossus/policy.toml`, the seven crate manifests and their sources, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `clippy.toml`, `deny.toml`, `.gitignore`, `.github/workflows/ci.yml`, `.github/dependabot.yml`, `scripts/`, `catalog/imports/almanac-v0.1/`, and `Ossus_v0.1_Almanac.zip`.

Deterministic checks executed:

| Check | Result |
|---|---|
| `git rev-parse --is-inside-work-tree` | FAIL — not a Git repository |
| `rustc --version` | `rustc 1.97.1 (8bab26f4f 2026-07-14)` |
| `cargo --version` | `cargo 1.97.1 (c980f4866 2026-06-30)` |
| `cargo fmt --all -- --check` | FAIL — 5 diff hunks (4 in `crates/ossus-cli/src/main.rs`, 1 in `crates/ossus-resolver/src/lib.rs:21`) |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --workspace --all-features` | PASS — 9 tests, 13 suites |
| `cargo run -q -p ossus -- status` | PASS |
| Doc mirror drift: `docs/{product,architecture,security,roadmap}` vs `docs/implementation/{01,02,04,09}-*` | PASS — all 25 pairs byte-identical |
| Capability count in `specs/taxonomy/capabilities-v1.yaml` | 44 — matches `README.md` |
| Golden case count in `evaluations/goldens/goldens-v1.yaml` | 50 `GOLD*` — matches `README.md` |
| `sha256sum Ossus_v0.1_Almanac.zip` | `dbd449e7…b097ef` — matches the value recorded in the import README |
| Rust source references to `catalog/`, `specs/`, `evaluations/` | none (one doc comment only) |

## Answers to the eleven review questions

### 1. Is the trusted computing base small enough for a security tool?

Mostly yes, with one unresolved expansion and one unassigned component.

The TCB is seven first-party crates plus a recommended dependency set. `unsafe_code = "forbid"` is set in `[workspace.lints.rust]` and every crate opts in with `[lints] workspace = true`; `crates/ossus-cli/src/main.rs:1` additionally carries `#![forbid(unsafe_code)]`. Today there are zero third-party dependencies and `Cargo.lock` contains only the seven first-party crates. That is a genuinely small starting TCB.

Two problems. First, `RUST_WORKSPACE.md` lists no YAML parser, and `ADR-006` confines YAML to external import and host-adapter output — yet the trusted taxonomy the Resolver depends on is YAML (F-02). Implementing WAVE-002 as written silently adds a YAML parser to the TCB or violates the ADR. Second, `rusqlite` with bundled SQLite plus FTS5 will be the single largest non-Rust component in the trusted binary. `ADR-007` is honest about this ("accepted with validation spike") and WAVE-003 requires proving FTS5 in release builds, which is the right control. I accept the SQLite decision; I do not accept an undecided YAML parser.

### 2. Are canonical and origin metadata truly separated?

Structurally, yes — by absence, which is stronger than by convention. `specs/schemas/canonical-manifest.schema.json` sets `additionalProperties: false` at every object level and contains no `origin` block anywhere. An upstream `origin`, `author_capabilities` or `triggers` key cannot enter a canonical manifest; validation rejects it. `ADR-005` and `THREAT_MODEL.md` back this with policy.

The gap is testability, not design. WAVE-002 lists a required test — "Origin fields cannot populate canonical-only state" — and Gate S1 requires "canonical/origin separation explicit". But no origin or evidence-bundle data contract exists anywhere in `specs/`, so there is no origin type to attempt the confusion with. As written the test can only be a tautology (F-12). The fix is cheap and should not wait for the Researcher: define the evidence-bundle shape now, or restate the WAVE-002 test as a concrete negative-fixture set asserting that unknown top-level keys are rejected.

### 3. Can untrusted content influence scoring or policy in ways not yet controlled?

Yes — through a vector the threat model does not name. This is the critical finding.

Ossus reads project-scoped configuration from `.ossus/config.toml` and project-scoped registries at precedence 300, above user (200) and official (100). The shipped `.ossus/config.toml` demonstrates that this file carries security-determining keys: `[policy] risk_max`, `allow_implicit_r4`, `block`, plus `minimum_confidence`, `semantic_fallback` and registry precedence. `.ossus/policy.toml` carries `risk_max`, `allow_modified_local`, `require_hash_verification`, `[runtime] deny` and `[surfaces] allow`.

A project directory is attacker-controlled the moment you clone a repository. Nothing in the plan states that project-scoped configuration may only restrict, never relax, user or global policy. Under the plan as written, a hostile repository can ship a `.ossus/` that sets `risk_max = "R4"`, `allow_implicit_r4 = true`, `block = []`, `require_hash_verification = false` and a priority-300 registry shadowing `official.*` namespaces — and `ossus resolve` in that directory would honour it. That defeats the R4-explicit-invocation rule, the R5 exclusion, and hash verification simultaneously, with no user interaction beyond `git clone && cd`.

`DATA_CONTRACTS.md` makes it worse rather than better: "Limits are configurable downward by policy. Increasing security-sensitive limits requires explicit configuration and audit output." An attacker's committed file *is* explicit configuration under that wording.

`REGISTRY_DESIGN.md` says a higher-priority source "cannot silently impersonate a lower source's namespace" and permits override "only under explicit conflict rules" — but those conflict rules are defined nowhere, and V0 has no source signing (`SUPPLY_CHAIN.md`: "V0: hashes and protected Git commits"). See F-01 and F-15.

Everything else in this area is sound. Resolver Stage 3 retrieves over curator-owned canonical descriptions, not resource bodies. Task text is explicitly not parsed as configuration. `persist_raw_task = false` is a good default. Search never reads bodies (WAVE-003 acceptance).

### 4. Does index-first distribution create unresolved fetch or license risk?

License risk: yes, and it is enforceable today. `TRUST_BOUNDARIES.md` requires a license decision to cross boundary C (evidence → canonical). `SUPPLY_CHAIN.md` requires "upstream license and review record" on every indexed resource. WAVE-003 requires a "seed license/source report". But `source.license` is **optional** in the schema — `source.required` is `["mode","repository","commit","tree_hash"]`. A canonical manifest with no license passes validation, enters the index, and directs a user to fetch third-party content with no recorded license. In an index-first model where Ossus is the party telling the user what to fetch, that is a real legal and provenance defect, and it is one line of schema (F-03).

Fetch risk: partly unresolved. `source.repository` is a free string with `maxLength: 512` and no transport constraint, so `remote-index` mode admits `file://`, unauthenticated `git://`, or SSH URLs that engage a forwarded agent. No document defines an allowed transport set (F-15). `source.commit`'s pattern `^[0-9a-fA-F]{40,64}$` accepts every length from 40 to 64 and mixed case, so the same commit has many valid spellings — which defeats the "same version with different hash" conflict detection WAVE-003 must implement (F-09). Note that `tree_hash` is correctly constrained to lowercase `^sha256:[0-9a-f]{64}$`; the inconsistency between the two fields is itself evidence of the intended strictness.

Separately, `source.mode` (`remote-index|vendored|local-private`) and `distribution.mode` (`index-only|vendored|private-local`) encode the same concept with two different vocabularies and no stated cross-field invariant, so `source.mode = "vendored"` with `distribution.mode = "index-only"` validates cleanly (F-08).

### 5. Is activation transactional and race-resistant?

The intent is right and WAVE-007 asks for the correct tests — "TOCTOU scenarios where practical", "Crash rollback", "Concurrent activation lock", "Unmanaged preservation". The mechanism is unspecified, and two documented requirements are in direct tension.

`SYSTEM_ARCHITECTURE.md` describes an "atomic swap". `ACTIVATION_SECURITY.md` requires replacing "only Ossus-managed paths". `PHASES_AND_GATES.md` and `CLI_CONTRACT.md` require never deleting user-authored host skills. These cannot all hold with a single directory rename: `.claude/skills/` contains user-authored siblings, so it cannot be swapped wholesale; and host discovery requires `SKILL.md` at `.claude/skills/<name>/`, so an Ossus-owned parent directory that *could* be swapped atomically would not be discovered. The remaining option is per-resource directory renames, which are individually atomic but not atomic across the set — and a crash mid-set leaves a partially-active set, violating Gate S3's "failed activation preserves prior state" (F-07).

There is also no locking model anywhere in the plan: no lock on `$OSSUS_HOME`, no project lock, no stale-lock recovery protocol, and no statement about `ossus registry sync` swapping the index underneath an in-flight `resolve` or `activate` that holds a snapshot ID. `ossus doctor` mentions "stale transactions", which implies crashes leave recoverable state, but the recovery contract is undefined. `SOURCE_AND_INSTALLATION_MODEL.md` says a shared custom store path is allowed "provided permissions and concurrency are handled" — that clause has no owner. WAVE-007 mandates the test without mandating the design, which is backwards for a security WAVE.

### 6. Are host compatibility claims precise enough to avoid false portability?

Yes at the modelling level, no at the enforcement level. Modelling compatibility as independent dimensions — surface, portability, scope, runtime requirement — is the right decision and `HOST_COMPATIBILITY.md`'s worked example (compatible with `claude-code-cli`, `shell-required`, `portable-with-adapter`, incompatible with an API-only host) is precisely the distinction most tools get wrong.

But `compatibility.surfaces` is `{"type":"array","items":{"type":"string"}}` with no enum and no `maxLength`, while `HOST_COMPATIBILITY.md` defines exactly nine surfaces and `.ossus/policy.toml` allowlists two. Portability, scopes, runtime requirements and risk tiers all have enums; surfaces — the primary compatibility gate, exercised by WAVE-005's required "surface mismatch" test and `ossus search --surface` — does not. WAVE-002 requires rejecting "unmapped capabilities" but says nothing about unmapped surfaces, so a typo like `claude-code-CLI` passes schema and semantic validation and silently changes matching behaviour (F-10). Capability IDs are also free strings in the schema, but WAVE-002 explicitly assigns them semantic validation, so that one is covered.

### 7. Does the Rust crate graph preserve trust boundaries?

Partly, and there is a hole where the most security-critical component should be.

The implemented graph is `cli → {core, registry, resolver, policy, adapter-claude, eval}`, `resolver → {core, policy, registry}`, `eval → {core, registry, resolver}`, everything else `→ core`. `RUST_WORKSPACE.md` documents only "registry, resolver, policy and adapters depend on core" and does not name the resolver→policy or resolver→registry edges. Those edges are correct — having the resolver call policy directly is what makes "policy denial precedes scoring" a structural property rather than a CLI sequencing convention — but the document must say so, or a reviewer cannot distinguish an intended edge from a violation (F-19).

The hole: WAVE-007's deliverable is a "host-neutral activation API" — content-addressed store, hash verification, path canonicalization, symlink rejection, staging, ownership records, rollback, audit events. No crate owns it. It cannot live in `ossus-cli` (`AGENTS.md`: "Keep domain logic outside `ossus-cli`"), it must not live in `ossus-adapter-claude` (that crate is the Claude-specific boundary, and it currently depends only on `ossus-core`), and `ossus-core` "performs no filesystem mutation" per `RUST_WORKSPACE.md`. So under the documented graph the only crate permitted to mutate the filesystem is the one forbidden to hold domain logic (F-04). This needs an eighth crate, decided now and created in WAVE-007 — not discovered mid-WAVE by an implementation agent under time pressure, which is exactly how activation logic ends up in the CLI.

### 8. Can private source precedence enable impersonation?

Yes, and F-01 is the sharp version of it. Beyond that vector: namespaces are `publisher.resource-name`, "reserved namespaces require maintainership proof", and no proof mechanism, reservation registry or source signature exists in V0. `SUPPLY_CHAIN.md` defers signing to post-V0. Gate D0 requires that "every override is visible and attributable", which is a detection control, not a prevention control, and WAVE-013 (private catalogs and source precedence) is marked "Implementation agent / Security: No" despite owning source precedence — a change class that `CHANGE_CONTROL.md` explicitly requires an ADR for. WAVE-013's security marking should be revisited when Phase 4 is planned; I record it as an observation rather than a blocking finding because it is four phases out.

### 9. Are security WAVEs placed before dependent functionality?

Mostly, with two unowned gates.

The ordering is right where it is defined: WAVE-007 before the adapter, WAVE-010 before the release candidate, WAVE-017/018 before the connectors, WAVE-023 before beta. Gate S0 before WAVE-001. That is a coherent spine.

But `SECURITY_GATES.md` places S1 "before Registry indexing" (i.e. before WAVE-003) and S2 "before activation work" (i.e. before WAVE-007), while `PHASES_AND_GATES.md` lists S2–S4 as Phase 3 gates and assigns Phase 2 only Gate R0. No WAVE file contains an S1 or S2 closure step. WAVE-002 says "Gate S1 contract requirements pass" but is marked `Security WAVE: no` with no human closure. WAVE-005 implements the entire S2 control set — "Policy denial before scoring", "R4 implicit denial", "Surface mismatch", "Ambiguous low confidence" — and is also marked `Security WAVE: no` with no gate closure. `SECURITY_GOVERNANCE.md` says a human closes every security gate. So S1 and S2 are required, tested, and unowned (F-05). Gate S0's own criterion is that assignments and ordering agree, so this is in scope for this review to block on.

### 10. Would the golden suite detect an unsafe false activation?

For the Resolver, yes. WAVE-005 requires R4-implicit-denial and surface-mismatch tests; WAVE-006 requires adversarial Resolver fixtures, zero constraint violations and zero implicit R4 activations at Gate R0; `EVALUATION_STRATEGY.md` Layer 4 names the right adversarial classes including "claimed R0 with shell files". Freezing Layer 1 before scoring exists is the correct discipline and it has actually been done — 50 cases are committed.

Two caveats. Layer 4's filesystem-facing cases — symlink escape, path traversal — belong to WAVE-007, not WAVE-006, and no document says which layer runs at which gate, so an implementer could reasonably read WAVE-006 as owning all of Layer 4 and defer none of it. Second, and this is the substantive one: the golden suite measures whether the Resolver selects the right resources given a trusted policy. It does not measure whether the *policy itself* was subverted. F-01 is invisible to every current golden case because all of them assume the effective policy is the one the user configured. The adversarial layer needs at least one case where a hostile project configuration attempts to relax policy and the assertion is that the effective policy is unchanged.

### 11. Is the Researcher still capable of contaminating trusted CI?

By design, no — the isolation model is sound. `STAGING_AND_CI.md` forbids candidate content in privileged branches and prefers offline analysis on immutable artifacts; `ADR-012` mandates a separate staging repository; `.gitignore` excludes `research-quarantine/`, `research-evidence/` and `research-work/`; the Researcher is Phase 5, after two dedicated security WAVEs. `.github/workflows/ci.yml` uses `permissions: contents: read`, triggers on `pull_request` (not `pull_request_target`), and holds no secrets.

In practice, the contamination has already partly happened — before the Researcher exists. `Ossus_v0.1_Almanac.zip` sits at the repository root and is not in `.gitignore`. It contains 26 files of external origin, including `install/fetch_skills.py` (7,221 bytes, a network fetcher) and `ossus.py`, plus `overlays/*.md` transformation directives. Its extracted index lives at `catalog/imports/almanac-v0.1/registry/skills.json` — 50 entries carrying upstream-controlled `notes`, `aliases`, `overlay` and `install_strategy: scan-frontmatter` fields — inside `catalog/`, which `README.md` describes as "Canonical Registry layout; no candidates". This contradicts ADR-012 and the README's own claim (F-06).

The mitigations are real: no Rust source references these paths, the archive hash is recorded and verifies, CI does not execute the Python, and the import README correctly states the entries are not canonical. But "the trusted repository currently contains external-origin executable code that a future CI job or release-packaging glob could pick up" is precisely the class of thing the Researcher WAVEs exist to prevent, and the plan should not have to relitigate it in Phase 5.

Also flagged in this area: `actions/checkout@v6` is a mutable tag, not a pinned SHA, while `SUPPLY_CHAIN.md` requires source pinning and Gate S4 requires a workflow pinning policy; and `deny.toml` is fully configured with no CI job invoking it (F-14).

## Findings

Severity is stated as Opus 5's position. Per `FINDINGS_DISPOSITION.md`, severity is not averaged with any other agent's opinion, and critical or high findings remain blocking until evidence resolves them.

| ID | Severity | Title | Primary evidence |
|---|---|---|---|
| F-01 | **Critical** | Project-scoped configuration can relax user/global security policy; no monotonicity invariant | `.ossus/config.toml:17-22`, `.ossus/policy.toml`, `REGISTRY_DESIGN.md:13-21`, `DATA_CONTRACTS.md:125` |
| F-02 | **High** | Trusted taxonomy and goldens are YAML, contradicting ADR-006; WAVE-002 is unimplementable without violating the ADR or silently expanding the TCB | `specs/taxonomy/*.yaml`, `evaluations/goldens/goldens-v1.yaml`, `DECISION_LOG.md:39-47`, `RUST_WORKSPACE.md:130`, WAVE-002 in-scope |
| F-03 | **High** | `source.license` is optional although three documents make license recording mandatory at trust boundary C | `canonical-manifest.schema.json` (`source.required`), `TRUST_BOUNDARIES.md:36`, `SUPPLY_CHAIN.md:34` |
| F-04 | **High** | No crate owns the host-neutral activation transaction; the only filesystem-mutating crate in the documented graph forbids domain logic | `RUST_WORKSPACE.md:23,45,105-108`, `AGENTS.md:39`, WAVE-007 deliverables, crate manifests |
| F-05 | **High** | Gates S1 and S2 have no owning WAVE, assigned reviewer or closure step | `SECURITY_GATES.md:13,22`, `PHASES_AND_GATES.md`, WAVE-002/005 headers, `WAVE_INDEX.md` |
| F-06 | Medium-high | External-origin archive with network-fetching scripts, and its extracted index, are committed in the privileged repository | `Ossus_v0.1_Almanac.zip`, `catalog/imports/almanac-v0.1/`, `.gitignore`, `ADR-012`, `STAGING_AND_CI.md:13`, `README.md:135` |
| F-07 | Medium | Set-level activation atomicity and the locking/recovery model are unspecified and in tension with unmanaged-file preservation | `SYSTEM_ARCHITECTURE.md:106`, `ACTIVATION_SECURITY.md:14,24,50`, `CLI_CONTRACT.md:119`, WAVE-007 required tests |
| F-08 | Medium | `source.mode` and `distribution.mode` are overlapping enums with divergent vocabularies and no cross-field invariant | `canonical-manifest.schema.json` |
| F-09 | Medium | `source.commit` accepts 40–64 mixed-case hex, admitting non-canonical and case-variant digests | `canonical-manifest.schema.json`, WAVE-003 "same-version hash conflicts" |
| F-10 | Medium | `compatibility.surfaces` is an unconstrained string array despite being the primary compatibility gate | `canonical-manifest.schema.json`, `HOST_COMPATIBILITY.md:5-15`, `.ossus/policy.toml` |
| F-11 | Medium | Forward-compatibility policy ("unknown optional fields preserved") contradicts `additionalProperties: false` | `DATA_CONTRACTS.md:131`, schema, `CAPABILITY_GOVERNANCE.md` MINOR rule |
| F-12 | Medium | No origin/evidence data contract exists, so WAVE-002's canonical/origin separation test and Gate S1's criterion have nothing to assert against | WAVE-002 required tests, `SECURITY_GATES.md:13`, absence of any origin schema in `specs/` |
| F-13 | Medium | `rust-toolchain.toml` pins the floating `stable` channel; CI never tests a pinned toolchain | `rust-toolchain.toml`, `.github/workflows/ci.yml`, `RUST_WORKSPACE.md:136-137`, WAVE-001 in-scope |
| F-14 | Medium | GitHub Actions referenced by mutable tag; configured `deny.toml` has no CI job | `.github/workflows/ci.yml`, `deny.toml`, `SUPPLY_CHAIN.md:11` |
| F-15 | Medium | No transport allowlist for `source.repository` | schema, `SOURCE_AND_INSTALLATION_MODEL.md`, `THREAT_MODEL.md` |
| F-16 | Low-medium | Repository is not initialized; `cargo fmt --check` fails; `scripts/verify.sh` stops before tests | `git rev-parse`, `cargo fmt --all -- --check` |
| F-17 | Low | CLI exit code 69 is outside the stable exit-code table; global flags unparsed and undocumented in help | `crates/ossus-cli/src/main.rs`, `CLI_CONTRACT.md:135-151` |
| F-18 | Low | `unwrap_used`/`expect_used = "deny"` applies to test targets with no documented exemption, creating future pressure to weaken a security lint | `Cargo.toml` `[workspace.lints.clippy]` |
| F-19 | Low | Documented crate dependency direction omits edges that already exist; root `tests/` directory absent | `RUST_WORKSPACE.md:23,105-108`, crate manifests |
| F-20 | Low | WAVE-001's required CLI help snapshot tests are absent; existing tests call `run()` in process | `crates/ossus-cli/src/main.rs`, WAVE-001 required evidence |

Full evidence, proposed plan change, required test and disposition for each finding are in `WAVE-000_FINDINGS_DISPOSITION.md`.

## Uncertainties

Stated explicitly, per the prompt's requirement that uncertainty remains visible.

1. **F-01 exploitability depends on unwritten code.** No configuration loader exists yet, so I am reviewing intent, not behaviour. It is possible the implementer would independently reach the right answer. I do not think a security plan may rely on that, but the severity rests on the absence of a stated invariant rather than on an observed vulnerability.
2. **F-02's correct resolution is a genuine trade-off I cannot settle alone.** Converting the taxonomy and 50 goldens to TOML preserves ADR-006 and keeps a parser out of the TCB, at a real cost in human editability for the golden suite. Amending ADR-006 to permit a bounded YAML subset for trusted taxonomy is defensible if the parser is chosen for hardening rather than convenience. This is a `CHANGE_CONTROL.md` "trusted format" decision and belongs to the human.
3. **F-04's crate boundary is my recommendation, not the only sound one.** Placing the transaction in a new `ossus-activation` crate is what I would do. Placing it in `ossus-registry` alongside the content-addressed store is also arguable. I am confident it belongs in neither `ossus-cli` nor `ossus-adapter-claude`.
4. **F-06's residual risk may already be accepted.** The import README argues the entries are metadata-only evidence rather than candidate content, which is a real distinction ADR-012 does not explicitly address. If the project owner intends this as a recorded ADR-012 exception, that is a legitimate outcome; it is not currently recorded as one.
5. **I have not validated FTS5 availability** in release builds on any platform. ADR-007's spike is WAVE-003's job and I did not pre-empt it.
6. **Performance claims are unverified.** The resolve p95 < 500 ms at 1,000 manifests target has no measurement behind it yet; nothing in this review either supports or challenges it.
7. **No independent cross-model security review exists.** ADR-014's 2026-08-03 revision records this as an accepted residual risk. This review inherits it: a single model's adversarial pass is correlated with that model's blind spots, and human closure is the only compensating control.

## Recommended reviewer focus for the human

In order: F-01 (does the project-scoped-configuration threat change your mental model of the product?), F-02 (which trusted format, and who pays the editing cost?), F-04 (do you accept an eighth crate?), F-05 (who closes S1 and S2?), F-06 (exception or relocation?).

Everything from F-07 down is normal engineering correction and can be dispositioned as a batch.
