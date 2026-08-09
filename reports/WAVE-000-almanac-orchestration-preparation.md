# WAVE preparation report — Almanac catalog import and Codex orchestration

> **Historical evidence:** ADR-020 and `docs/AGENT_AUTHORITY.md` supersede this document's authority requirements for decisions on or after 2026-08-07. This note does not change recorded facts.

## Metadata

- WAVE: WAVE-000 preparation only; Security Gate S0 remains open
- Implementer: Codex primary orchestrator
- Model: gpt-5.6-luna, max reasoning; two parallel read-only Luna workers used for independent checks
- Date: 2026-08-03
- Base commit: unavailable; the workspace `.git` directory is empty in this environment
- Final working tree state: local catalog, project Codex configuration, and this report added; no commit or push performed

## Objective completed

- Verified and decompressed `Ossus_v0.1_Almanac.zip` into a temporary inspection root.
- Verified every file listed by the archive's embedded `SHA256SUMS`.
- Imported the exact `registry/skills.json` allowlist containing 50 unique skill IDs into `catalog/imports/almanac-v0.1/`.
- Kept official canonical manifests empty and did not install or activate any skill body.
- Added project-scoped Codex settings for a primary orchestrator with up to eight concurrent Luna-max workers.
- Added a bounded worker role that explicitly reports to the primary orchestrator and cannot close gates or approve resources.
- Applied the same validated multi-agent defaults to the global Codex configuration after explicit approval.

## Scope implemented

The Almanac import is evidence/seed input, not canonical Registry state. Its source statuses are preserved as origin metadata only. The archive does not provide the immutable commits, tree hashes, canonical capabilities, risk observations, or Ossus reviewer records required by the canonical manifest schema, so none were invented.

The project and global Codex configurations enable `multi_agent`, set the worker cap to eight (excluding the primary thread), and default spawned workers to `gpt-5.6-luna` with `max` reasoning. The primary model is not pinned by the project file and remains selectable by the user/session.

## Files changed

- `.codex/config.toml`
- `.codex/agents/luna-worker.toml`
- `catalog/README.md`
- `catalog/official/README.md`
- `catalog/imports/README.md`
- `catalog/imports/almanac-v0.1/README.md`
- `catalog/imports/almanac-v0.1/registry/skills.json`
- `reports/WAVE-000-almanac-orchestration-preparation.md`

External approved configuration change:

- `/home/starryboyjosh/.codex/config.toml`

## Architecture decisions made

1. The Almanac list is stored under `catalog/imports/`, separate from `catalog/official/manifests/`, so stored evidence cannot be mistaken for approved or active resources.
2. No canonical manifest fields were fabricated from Almanac categories or statuses. Admission remains a later Registry-WAVE responsibility with human review.
3. The primary Codex thread is the only user-facing orchestrator. Workers receive delegated tasks and return evidence; they do not receive authority to broaden scope, approve resources, activate content, or close a security gate.
4. The worker pool is capped at eight concurrent threads. This is a project default, not a replacement for the required Opus 5 security role.

## Tests and commands

- `rtk sha256sum -c SHA256SUMS` — PASS; all 26 archive files verified.
- `rtk cmp /tmp/ossus-almanac-inspect/Ossus_v0.1_Almanac/registry/skills.json catalog/imports/almanac-v0.1/registry/skills.json` — PASS; byte-identical.
- `rtk jq -e 'type=="array" and length==50 and (map(.id)|unique|length==50)' catalog/imports/almanac-v0.1/registry/skills.json` — PASS.
- Python `tomllib` validation for both Codex TOML files — PASS.
- `rtk codex features list` — PASS; `multi_agent` is stable and effective `true`.
- Global `/home/starryboyjosh/.codex/config.toml` copy-back — PASS after explicit approval; TOML validation and byte-for-byte comparison with the prepared file passed.
- `rtk python3 scripts/check-repository-layout.py` — PASS.
- `rtk cargo clippy --workspace --all-targets --all-features -- -D warnings` — PASS.
- `rtk cargo test --workspace --all-features` — PASS; 9 tests passed across 13 suites.
- `rtk cargo run -q -p ossus -- status` — PASS; reports `current_wave: WAVE-000` and scaffold state.
- `rtk ./scripts/verify.sh` — BASELINE FAILURE at `cargo fmt --all -- --check`; it reports pre-existing formatting differences in `crates/ossus-cli/src/main.rs` and `crates/ossus-resolver/src/lib.rs`. Clippy, tests, CLI status, and layout were run separately and passed. Those unrelated formatting differences were not changed.
- `rtk graphify extract . --code-only --no-cluster` — ENVIRONMENT FAILURE; the AST subprocess was denied with `Operation not permitted` and produced an empty graph. The invalid generated output was moved to `/tmp/ossus-graphify-failed` and is not part of the repository.

## Acceptance criteria

| Criterion | Evidence |
| --- | --- |
| Almanac skills are represented exactly | Imported JSON is byte-identical to the archive source and contains 50 unique IDs. |
| No unreviewed skill body enters active paths | No `vendor/skills/` directory or official manifest was created; import README marks the data noncanonical and non-activatable. |
| Source provenance is retained | Archive filename, archive SHA-256, source registry path, version, and embedded checksum verification are documented. |
| Multiple Luna-max workers are available | `.codex/config.toml` enables multi-agent operation, caps eight concurrent workers, and sets Luna/max defaults; the worker role repeats the boundary. |
| Primary orchestrator retains authority | Project config does not replace the primary model; worker instructions identify the primary thread as the sole user-facing orchestrator. |
| WAVE-000 security workflow is preserved | No Opus 5 report or human S0 closure was fabricated; `CURRENT_WAVE.md` remains WAVE-000 and open. |

## Known limitations

- The Almanac entries still need immutable upstream commits/digests, tree/content hashes, canonical capability mappings, runtime observations, risk classification, license evidence, independent Admission Review Agent review, and a distinct Closure Agent decision.
- The import is not a Resolver source and cannot be installed or activated.
- The current Codex process may need a new session/restart to reload the newly written global and project settings. The global file already reports Luna/max as the current primary model and `multi_agent` as enabled.
- The graphify AST subprocess is unavailable in this restricted environment.
- The scaffold's existing Rust formatting check remains failing as described above.

## Security impact

No archive script, installer, workflow, binary, upstream repository, or skill body was executed. No network fetch was performed. The import preserves a clear boundary between external selection evidence and trusted canonical metadata. The worker role reduces the chance that a delegated worker silently changes WAVE scope or security authority, but model instructions are not a security boundary.

## Performance impact

No product runtime path changed. The worker pool permits up to eight concurrently open spawned-agent threads, excluding the primary thread; actual usage remains subject to service availability and account limits.

## Deferred work

- Opus 5 adversarial WAVE-000 plan review.
- Explicit findings disposition and human Security Gate S0 closure.
- Registry admission and canonicalization in the assigned future Registry WAVE.
- Immutable source resolution and license/risk evidence for each Almanac skill.
- Any resource fetching, installation, activation, Resolver indexing, or host materialization.

## Residual risks

- Almanac metadata is curated evidence, not proof that upstream content is safe or unchanged.
- A worker can still produce incorrect or malicious output; the primary orchestrator must treat it as untrusted evidence and validate changes.
- Multi-agent concurrency can create conflicting edits if delegated write scopes overlap; future prompts should assign disjoint write sets and require review before integration.

## Recommended reviewer focus

- Confirm the import/official-catalog separation and the absence of fabricated approval data.
- Confirm that project-level `[features]` and `[agents]` settings are accepted by the supported Codex CLI version after a fresh session.
- Confirm that the primary thread remains the only user-facing orchestrator when workers run in parallel.
- Keep WAVE-000 paused until the specified Opus 5 review, findings disposition, and human S0 evidence exists.

## Handoff summary

The exact Almanac skill allowlist is now preserved as noncanonical catalog input, with official manifests intentionally empty. Ossus is prepared for bounded parallel Luna-max workers under a primary orchestrator, while the WAVE-000 security gate and all later Registry/Resolver implementation remain deferred.

## Amendment — 2026-08-04, Gate S0 closure

This report describes the state of the tree before Security Gate S0 was closed. Finding F-06 of `docs/implementation/08-operations/WAVE-000_OPUS5_PLAN_REVIEW.md` established that holding an external-origin archive and its extracted index inside the privileged repository contradicts ADR-012 and `STAGING_AND_CI.md`. Gate S0 decision D5 chose relocation.

The paths named above therefore no longer exist in the repository:

| Former path | Current path |
|---|---|
| `Ossus_v0.1_Almanac.zip` | `research-evidence/almanac-v0.1-import/Ossus_v0.1_Almanac.zip` (untracked) |
| `catalog/imports/README.md` | `research-evidence/almanac-v0.1-import/catalog-imports/README.md` (untracked) |
| `catalog/imports/almanac-v0.1/` | `research-evidence/almanac-v0.1-import/catalog-imports/almanac-v0.1/` (untracked) |

`research-evidence/` is excluded by `.gitignore`, so no external-origin content entered the repository's history when WAVE-001 initialized Git. The archive was relocated rather than destroyed: the security requirement is exclusion from the privileged repository, not destruction of the evidence, and its SHA-256 `dbd449e700f7718d1558cab41a56c98fc64bef980273830f041535de02b097ef` and contents remain recorded in the relocated import README. The verification commands recorded above were run against the former paths and are not re-runnable from the repository.
