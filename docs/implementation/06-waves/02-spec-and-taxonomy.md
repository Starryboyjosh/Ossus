# WAVE-002 — Trusted specifications and taxonomy

**Phase:** Phase 1  
**Assigned role:** Implementation agent with architecture review  
**Depends on:** WAVE-001  
**Security WAVE:** no — implementation may be delegated  
**Closes:** **Security Gate S1 — Trusted contracts.** Gate closure requires Opus 5 review of the final diff and evidence, then a named human closing against `07-prompts/HUMAN_SECURITY_CLOSURE.md`. WAVE-003 must not begin before S1 is closed.

## Objective

Implement typed trusted contracts, taxonomy loading and bounded validation.

## In scope

- Implement core IDs, enums and version types.
- Load canonical TOML manifests.
- Maintain JSON schemas.
- Convert the taxonomy and evaluation data out of YAML per **ADR-017**: `capabilities-v1`, `aliases-v1`, `deprecations-v1`, `goldens-v1`, `seed-catalog-profiles` and `model-roles` become TOML in both `specs/`/`evaluations/` and their `docs/implementation/` mirrors. Verify content equivalence against the YAML originals (44 capability IDs, 50 golden cases, round-trip comparison) before deleting the originals. Do not add a YAML crate to satisfy this WAVE.
- Load the 44 capabilities, aliases and deprecations from the converted TOML.
- Enforce parser budgets and semantic validation.
- Reject unknown major versions and unmapped capabilities.
- Apply the schema corrections carried from Gate S0:
  - **F-03** — require `source.license` conditionally, via `if`/`then` on `source.mode != "local-private"`, so a remote or vendored source cannot be indexed without a recorded license while a genuinely private local source is not forced to invent one.
  - **F-08** — resolve the `source.mode` / `distribution.mode` overlap: either collapse `distribution.mode` into `source.mode`, or redefine `distribution` in redistribution terms (for example `source-only`, `approved-install-only`) and state the cross-field invariant explicitly. Contradictory pairs must be rejected by semantic validation.
  - **F-09** — tighten `source.commit` to `^([0-9a-f]{40}|[0-9a-f]{64})$` and normalize to lowercase at ingest, so one commit has exactly one valid spelling.
  - **F-10** — constrain `compatibility.surfaces` to the nine surfaces defined in `docs/HOST_COMPATIBILITY.md`, by enum or by governed semantic validation, and add `maxLength` to its items.
  - **F-11** — no schema change: strict rejection of unknown fields is the intended behavior and `DATA_CONTRACTS.md` has been corrected to match. Do not relax `additionalProperties`.

## Out of scope

- SQLite index.
- Search.
- Task mapping.
- Activation.

## Expected deliverables

- Typed manifest and taxonomy APIs.
- Validation library and CLI diagnostics.
- Version and migration documentation.
- TOML taxonomy and evaluation data with an equivalence report against the retired YAML originals.
- **Gate S1 closure record**, prepared for the distinct Closure Agent.

## Required tests and evidence

- Valid and invalid examples.
- Unknown fields and major versions.
- Duplicate capabilities and excessive triggers.
- Depth, string and file-size budgets.
- Invalid risk/runtime combinations.
- Origin fields cannot populate canonical-only state. Concretely (**F-12**): a negative-fixture set asserting that `origin`, `author_capabilities`, `upstream_triggers` and `stars` are rejected as top-level or nested keys, plus a test that any origin record an importer produces is stored outside canonical state. The full evidence-bundle schema is deferred to WAVE-017.
- **F-03** — a `remote-index` manifest with no `source.license` fails validation with a specific reason code; a `local-private` manifest without a license passes.
- **F-09** — invalid fixtures for 41-, 50- and 63-character commit values and for uppercase values; a conflict-detection fixture using two case variants of the same commit.
- **F-10** — invalid fixtures for `claude-code-CLI` and `not-a-surface`, each rejected with a specific reason code.
- **ADR-017** — `cargo tree -p ossus` contains no YAML parser.

## Acceptance criteria

- All 44 IDs load from TOML.
- All 50 golden cases load from TOML and are byte-equivalent in content to the retired YAML.
- Invalid manifests cannot enter Registry input.
- Validation is deterministic.
- Gate S1 contract requirements pass, and **Gate S1 is closed by a named human** before WAVE-003 begins.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt. Treat every parsed file as malformed until validated.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
