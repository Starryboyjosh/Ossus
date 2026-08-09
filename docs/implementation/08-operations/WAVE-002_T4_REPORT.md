# WAVE-002 T4 implementation report

> **Historical evidence:** ADR-020 and `docs/AGENT_AUTHORITY.md` supersede this document's authority requirements for decisions on or after 2026-08-07. This note does not change recorded facts.

## Metadata

- WAVE: WAVE-002, T4 only
- Implementer: Codex subordinate implementation worker
- Model: GPT-5
- Date: 2026-08-05
- Base commit: none; this workspace has no committed `HEAD`
- Final working tree state: uncommitted, no branch or push created
- Gate state: S1 remains open; no WAVE or gate closure was performed

## Objective completed

Implemented bounded canonical TOML manifest loading, typed schema enforcement,
taxonomy loading, deterministic semantic validation, diagnostics, and the
`ossus validate <PATH>...` CLI command. The Registry component now reports
`implemented`; the repository-wide `current_wave` remains `WAVE-000`.

The deep-nesting defense includes an iterative structural walk and a cheap
pre-parse byte scan. The scan was needed proactively because the hostile
10,000-level acceptance inputs must produce a diagnostic before any parser
recursion can exhaust the stack; the test does not lower the nesting count.

## Files added or modified

- `crates/ossus-core/src/enums.rs` — derive `Ord`/`PartialOrd` for `ReviewTier`.
- `crates/ossus-registry/src/lib.rs` — public API and implemented component state.
- `crates/ossus-registry/src/budget.rs` — defaults, monotonic restriction, bounded TOML parsing.
- `crates/ossus-registry/src/diagnostic.rs` — stable diagnostics and bounded messages.
- `crates/ossus-registry/src/manifest.rs` — typed manifest, file loading, UTF-8/size gates, strict fields.
- `crates/ossus-registry/src/taxonomy.rs` — TOML capabilities, aliases, deprecations, deterministic lookup.
- `crates/ossus-registry/src/validate.rs` — semantic validation and reason codes.
- `crates/ossus-registry/tests/wave_t4.rs` — WAVE-T4 contract, budget, taxonomy, and determinism tests.
- `crates/ossus-cli/src/main.rs` — `validate`, global format handling, human/JSON diagnostics, exit codes.
- `crates/ossus-cli/tests/validate.rs` — CLI exit-code and JSON behavior tests.
- `docs/architecture/CLI_CONTRACT.md` — added `ossus validate <PATH>...`.
- `docs/implementation/02-architecture/CLI_CONTRACT.md` — byte-identical mirror update.
- `tests/snapshots/cli_snapshots__root_help_output.snap` — deliberate root-help snapshot update.
- `docs/implementation/08-operations/WAVE-002_T4_REPORT.md` — this report.

No `Cargo.toml`, `Cargo.lock`, JSON schema, taxonomy TOML, later-WAVE code, or
golden-evaluation data was changed. No external crate was added.

## Reason-code table

| Code | Meaning | Where enforced |
|---|---|---|
| `budget.manifest-bytes.exceeded` | Input or bounded read exceeds the byte budget | `budget.rs`, `manifest.rs`, `taxonomy.rs` |
| `budget.nesting-depth.exceeded` | TOML nesting exceeds the configured depth | `budget.rs` pre-parse scan and iterative walk |
| `budget.string-length.exceeded` | TOML string exceeds the configured length | `budget.rs` structural walk |
| `budget.list-items.exceeded` | TOML array exceeds the configured item count | `budget.rs` structural walk |
| `encoding.utf8.invalid` | File bytes are not UTF-8 | `budget.rs`/`manifest.rs` |
| `manifest.toml.invalid` | Source is not valid TOML | `budget.rs` |
| `manifest.file.stat-failed` | Manifest metadata could not be inspected | `manifest.rs` |
| `manifest.file.open-failed` | Manifest could not be opened | `manifest.rs` |
| `manifest.file.read-failed` | Bounded manifest read failed | `manifest.rs` |
| `manifest.field.unknown` | Unknown field rejected at any known object level | `manifest.rs` and `deny_unknown_fields` structs |
| `manifest.field.missing` | Required field absent | `manifest.rs` |
| `manifest.field.shape` | Required value/object/list has the wrong TOML shape | `manifest.rs` |
| `manifest.shape.invalid` | Typed deserialization shape mismatch | `manifest.rs` |
| `field.string.too-short` | Schema string minimum is violated | `validate.rs` |
| `field.string.too-long` | Schema string maximum is violated | `validate.rs` |
| `list.count.out-of-range` | Schema list cardinality is violated | `validate.rs` |
| `list.duplicate` | A governed list contains a duplicate | `validate.rs` |
| `capability.id.overlap` | Capability occurs in both required and optional lists | `validate.rs` |
| `capability-schema.version-mismatch` | Manifest capability-schema major differs from loaded taxonomy | `validate.rs` |
| `source.license.required-for-shared-source` | Shared source omits license | `validate.rs` |
| `distribution.mode.contradicts-source-mode` | Source/distribution pair is forbidden by core policy | `validate.rs`, using `distribution_is_permitted` |
| `distribution.notice-required.missing-for-redistribution` | Vendored redistribution lacks `true` notice flag | `validate.rs` |
| `distribution.notice-required.not-applicable` | Non-redistribution mode sets notice flag `true` | `validate.rs` |
| `source.commit.not-normalized` | Source commit was uppercase/mixed case in the file | `validate.rs` |
| `review.approved_commit.not-normalized` | Approved commit was uppercase/mixed case in the file | `validate.rs` |
| `risk.tier.below-runtime-requirement` | Declared risk is below runtime-implied floor | `validate.rs` |
| `risk.tier.excluded-from-registry` | R5 is excluded from the stable Registry channel | `validate.rs` |
| `review.tier.insufficient-for-risk` | Review tier is below the risk-derived floor | `validate.rs` |
| `context.measured_tokens.out-of-range` | Context measurement exceeds one million tokens | `validate.rs` |
| `capability.id.unmapped` | Capability is not a canonical taxonomy ID | `validate.rs` |
| `capability.id.alias-not-canonical` | Alias was used where canonical capability state is required | `manifest.rs` normalization plus `validate.rs` |
| `capability.id.deprecated` | Required deprecated capability is an error; optional is a warning | `validate.rs` |
| `taxonomy.file.stat-failed` | Taxonomy metadata could not be inspected | `taxonomy.rs` |
| `taxonomy.file.open-failed` | Taxonomy file could not be opened | `taxonomy.rs` |
| `taxonomy.file.read-failed` | Bounded taxonomy read failed | `taxonomy.rs` |
| `taxonomy.shape.invalid` | Taxonomy TOML does not match its typed document | `taxonomy.rs` |
| `taxonomy.schema-version.mismatch` | Capability, alias, and deprecation documents disagree on major | `taxonomy.rs` |
| `capability.id.domain-name-mismatch` | Capability ID differs from `domain.name` | `taxonomy.rs` |
| `capability.id.duplicate` | Taxonomy declares a canonical ID more than once | `taxonomy.rs` |
| `capability.alias.invalid-format` | Alias name is empty or over 128 characters | `taxonomy.rs` |
| `capability.alias.collides-with-id` | Alias collides with a canonical ID | `taxonomy.rs` |
| `capability.alias.target-unmapped` | Alias target is not declared | `taxonomy.rs` |
| `capability.deprecation.unmapped` | Deprecation entry is not declared | `taxonomy.rs` |
| `resource.id.invalid-format` | Core resource ID format rejected | `ossus-core::ResourceId` during manifest loading |
| `capability.id.invalid-format` | Core capability ID format rejected | `ossus-core::CapabilityId` during manifest loading |
| `source.commit.invalid-format` | Commit is not exactly 40/64 hexadecimal characters | `ossus-core::CommitHash` during manifest loading |
| `source.tree-hash.invalid-format` | Tree hash is not lowercase `sha256:<64 hex>` | `ossus-core::TreeHash` during manifest loading |
| `category.name.invalid-format` | Category name violates core format/length | `ossus-core::CategoryName` during manifest loading |
| `version.invalid-format` | Version is not canonical `major.minor.patch` | `ossus-core::Version` during manifest loading |
| `version.unsupported-major` | Schema/taxonomy major is unsupported | `validate_schema_version` in manifest/taxonomy validation |
| `resource-type.unknown` | Resource type is outside the closed enum | `ossus-core::ResourceType` |
| `source.mode.unknown` | Source mode is outside the closed enum | `ossus-core::SourceMode` |
| `distribution.mode.unknown` | Distribution mode is outside the closed enum | `ossus-core::DistributionMode` |
| `portability.unknown` | Portability is outside the closed enum | `ossus-core::Portability` |
| `scope.unknown` | Scope is outside the closed enum | `ossus-core::Scope` |
| `runtime.requirement.unknown` | Runtime requirement is outside the closed enum | `ossus-core::RuntimeRequirement` |
| `risk.tier.unknown` | Risk tier is outside the closed enum | `ossus-core::RiskTier` |
| `review.status.unknown` | Review status is outside the closed enum | `ossus-core::ReviewStatus` |
| `review.tier.unknown` | Review tier is outside the closed enum | `ossus-core::ReviewTier` |
| `compatibility.surfaces.unknown` | Surface is outside the closed, case-sensitive enum | `ossus-core::Surface` |
| `capability.status.unknown` | Taxonomy status is outside the closed enum | `ossus-core::CapabilityStatus` |

The CLI maps fatal schema-class diagnostics to exit 11 and fatal taxonomy-class
diagnostics to exit 12. Schema errors dominate when both classes occur. Warnings
do not fail validation, but are printed to stderr and included in JSON.

## Tests and required verification

The registry tests include the 44-capability count, shipped example, unknown
top-level/nested/F-12 fields, duplicate/overlap/excessive lists, size/string/list
and depth budgets, the 10,000-level inline-table and array hazard inputs, UTF-8,
F-03, F-08's nine pairs, F-09 lengths/case/conflict identity, F-10 surfaces,
risk/review floors, R5, taxonomy mapping/aliases/deprecation warnings, and
deterministic diagnostics. CLI tests cover valid, usage, schema/JSON, and
taxonomy exit behavior.

Required verification output:

```text
$ grep -c '^\[\[package\]\]' Cargo.lock        # before
47
$ grep -c '^\[\[package\]\]' Cargo.lock        # after
47

$ cargo fmt --all -- --check
(no output; pass)

$ cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo clippy: No issues found

$ cargo test --workspace
cargo test: 66 passed (16 suites, 0.05s)

$ cargo tree -p ossus | grep -iE "yaml|yml" ; echo "yaml-grep-exit:$?"
yaml-grep-exit:1

$ cmp docs/architecture/CLI_CONTRACT.md docs/implementation/02-architecture/CLI_CONTRACT.md && echo "CLI_CONTRACT mirror OK"
CLI_CONTRACT mirror OK
```

Exit 1 from the YAML grep is the passing result: no YAML parser was found.

Additional observed output:

```text
ossus status
registry: implemented

ossus validate specs/examples/canonical-manifest.example.toml
/home/starryboyjosh/Dev/Projects/ossus/specs/examples/canonical-manifest.example.toml: valid
next: submit valid manifests for the appropriate Registry review
```

## Snapshot diff

The only changed snapshot is
`tests/snapshots/cli_snapshots__root_help_output.snap`. The deliberate diff is:

```diff
     plan       Show the implementation-plan entry point

+VALIDATION COMMANDS:
+    validate   Validate canonical manifest files
+
 PLANNED COMMAND GROUPS:
```

The repository's current `tests/cli_snapshots.rs` has no status snapshot test,
despite the task note mentioning one. Therefore no status snapshot line changed;
the status behavior was verified directly and now reports `registry: implemented`.
The snapshot suite passes after this update.

## Security and architecture notes

- File loading stats before reading and uses `Read::take(budget + 1)` to enforce
  the budget across a stat/read race.
- UTF-8, TOML parsing, structural budgets, typed conversion, strict unknown-field
  rejection, and semantic validation occur in that order.
- The pre-parse scanner is iterative and ignores brackets in strings/comments.
- No raw candidate content is included in parser diagnostics; value excerpts use
  the core 64-character truncation behavior or an equivalent bounded helper.
- Core remains a leaf: its only change is the requested review-tier ordering derive.
- Registry lookup uses ordered maps for reproducible diagnostics and no per-query
  allocation for canonical ID/alias lookup.
- R5 exclusion is an explicit judgment call required by `RISK_TIERS.md`; it is
  surfaced here for the human approver.

## Contract discrepancies and boundaries

- The JSON schema intentionally leaves `review.approved_commit` without a hex
  pattern (only min/max length). The typed `CommitHash` layer closes that gap and
  the lowercase spelling rule without editing the schema, as instructed.
- The active schema already contains the conditional shared-source license,
  commit pattern, surface enum, and distribution cross-field rules. Code follows
  those existing contracts and adds the required semantic checks.
- The full WAVE-002 document also assigns conversion of goldens, seed profiles,
  and model roles from historical YAML. This T4 task explicitly limits the worker
  to canonical manifests, taxonomy loading, budgets, validation, and CLI
  diagnostics; those unrelated conversions were not attempted.
- No S1 closure record was created, and no later WAVE behavior was implemented.

## Recommended reviewer focus

Review the pre-parse scanner versus TOML lexical edge cases, the alias
normalization/rejection path necessitated by aliases being non-`CapabilityId` wire
strings, schema-versus-taxonomy exit classification, and the full required-field
and semantic diagnostic ordering.

## Opus 5 reviewer addendum

- Reviewer: Opus 5
- Review date: 2026-08-05
- Scope: final T4 source, tests, report claims, and security/architecture evidence
- Disposition: implementation accepted after fixes below; Gate S1 remains open for
  named human closure

The review found and corrected these defects without weakening a schema, budget,
test, or policy:

1. String-budget accounting used Unicode scalar counts although the governed KiB
   limit is a byte limit. It now measures UTF-8 bytes; the JSON Schema field
   limits remain code-point limits.
2. The R1 review floor accepted `light-human`; it now requires `full-human` as
   specified by the risk-tier contract.
3. The pre-parse nesting diagnostic hard-coded the default limit instead of
   reporting the configured restricted budget.
4. Attacker-controlled diagnostic values and table-key field paths were only
   truncated, not made terminal-safe; table-key paths also bypassed truncation.
   Rendering is now centralized in `ossus-core`, bounded where appropriate, and
   escapes control characters, terminal sequences, bidirectional controls, and
   zero-width/reordering characters. Paths remain untruncated but are escaped.
5. Alias validation checked only length, admitting whitespace, uppercase,
   leading hyphens, terminal controls, zero-width characters, and non-ASCII
   combining forms. Aliases now use the governed lowercase ASCII
   `[a-z0-9][a-z0-9-]*` form with a 128-byte maximum.
6. A valid TOML multi-line string could desynchronize the pre-parse nesting
   scanner and disable that first-line depth gate for the rest of the file.
   Escape-aware scanning and complete delimiter-run consumption now preserve
   synchronization for legal basic and literal multi-line endings. The TOML
   parser's own recursion limit absorbed the tested extreme input before this
   fix, so the demonstrated consequence was loss of defense in depth and
   unstable diagnostics, not a process crash.

The implementer's reason-code row at line 88 is superseded: the meaning of
`capability.alias.invalid-format` is now “alias violates the governed lowercase
ASCII format or the 128-byte limit,” not merely “empty or over 128 characters.”
Likewise, the original claim that truncation or an equivalent bounded helper
made every diagnostic safe was disproved by terminal-injection tests and is
superseded by item 4 above.

A status-to-deprecations cross-document rule was investigated but not retained:
the governed documents do not yet identify which source is authoritative, and
adding the rule broke an existing required T4 behavior. This remains visible as
`DIV-6` for human disposition rather than being hidden through a test or contract
change.

Reviewer regression tests are isolated in `wave_t4_review.rs`; each defect test
was observed failing with its corresponding fix reverted, then passing after
restoration. Final independent verification:

```text
cargo fmt --all -- --check
pass

cargo clippy --workspace --all-targets --all-features -- -D warnings
pass

cargo test --workspace
87 passed (17 suites), 0 failed

cargo test --test wave_t4_review -p ossus-registry
11 passed, 0 failed

Cargo.lock package count
47 (unchanged)

YAML parser crates in Cargo.lock
none

python3 scripts/check-repository-layout.py
repository layout: ok
```

This addendum preserves the implementer's original report as attributable
historical evidence. It does not close S1: model review is evidence, not
certification, and the named human closure required by
`HUMAN_SECURITY_CLOSURE.md` is still outstanding.

## T5 fixture corpus and final Gate S1 addendum

- Owner/reviewer: Opus 5
- Date: 2026-08-05
- Scope: executable negative-fixture evidence, final trusted-contract gaps, and
  Gate S1 preparation

T5 converted the checked-in canonical-manifest corpus from static examples into
executable regression evidence. `INDEX.toml` is now the machine-readable
inventory: each valid or invalid TOML fixture is indexed exactly once, every
invalid fixture declares its expected stable reason code, oversized seed cases
are amplified deterministically in memory, invalid UTF-8 is generated as raw
bytes, and F-12 fields are proved unable to deserialize into canonical state.
The approved-commit fixture also exposed a field-attribution defect; the
manifest layer now emits `review.approved_commit.invalid-format` rather than
reusing the source-commit reason code.

The final Gate S1 audit found that the lockfile schema had only an optional,
unconstrained `policy_hash` and no `taxonomy_hash`. The source schema and its
mirror now require both fields with the canonical lowercase
`^sha256:[0-9a-f]{64}$` contract, examples carry both identities, and the layout
regression rejects missing, uppercase, short, long and wrong-prefix values. This
is contract-level support only: calculating these identities and producing a
lockfile belongs to later producer WAVEs and was not implemented early.

Contribution attribution for the completed WAVE:

- T1–T3: delegated implementation, independently inspected and re-executed by
  Opus 5.
- T4: delegated implementation with six Opus 5 corrections and dedicated
  regression tests.
- T5: implemented directly by Opus 5 after the Codex bridge failed before making
  edits because `OMNIROUTE_API_KEY` was unavailable.

The remaining governance divergence is `DIV-6`: capability `status` and the
separate deprecations document duplicate a fact without naming an authoritative
source. No policy was invented. The prepared human package
`WAVE-002_GATE_S1_CLOSURE.md` requires the named approver to ratify the current
V0 behavior or require an ADR-backed correction. WAVE-003 remains blocked.

Final verification against the initial-commit candidate tree:

```text
cargo fmt --all -- --check
pass

cargo clippy --workspace --all-targets --all-features -- -D warnings
pass (no issues)

cargo test --workspace --all-features
92 passed (18 suites), 0 failed

cargo run -q -p ossus -- status
pass; reports current_wave: WAVE-002

python3 scripts/check-repository-layout.py
repository layout: ok

cargo deny check
advisories ok, bans ok, licenses ok, sources ok
(five configured license allowances were unused warnings)
```

The repository manifest and tree were regenerated after this report update. The
staged index is checked separately by `scripts/check-layout.sh` before commit.
