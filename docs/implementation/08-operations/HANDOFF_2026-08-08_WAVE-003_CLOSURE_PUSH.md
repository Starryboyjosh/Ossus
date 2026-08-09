# Ossus WAVE-003 closure-push handoff — 2026-08-08

## Status

`WAVE-003 — IN PROGRESS`

The Registry implementation is complete and locally verified. The official
Registry remains **0 / 20**. WAVE-004 is not authorized.

## Technical summary

The local SQLite/FTS5 Registry, deterministic rebuild, conflict detection,
metadata-only search/show APIs, filters, human/JSON CLI output, release FTS5
test and deterministic Git-tree hashing are healthy. No implementation defect
was found in this continuation.

The closure push concentrated on Tier-A admission and profile-20 replacement:

- Profiles 6 and 9 have exact immutable MIT source tuples, static prompt-pack
  classifications, R0/instruction-only behavior and valid field drafts. Their
  seed profiles require standard + Claude Code + Codex surfaces, but only the
  standard surface is evidenced. A standard-only profile substitution is
  architecture-compliant in principle; it still needs a distinct Closure
  decision and an aggregate cross-host coverage check.
- Profile 10 has exact MIT source evidence and a bounded Postgres migration/
  schema review mapping, but the upstream scope includes authoring, RLS,
  configuration, restore/import and contributor-test branches. An R1 claim
  requires an immutable, independently reviewed read-only enforcement adapter;
  manifest exclusions are not enforcement.
- Profile 20's original calculator remains rejected. The replacement
  `slettmayer/calc-mcp-server` v0.1.3 is materially safer and has a bounded
  evaluator plus generic stdio MCP, but remains conditional because build
  requirements and launcher metadata are unpinned, `server.json` is stale,
  release workflows/scripts are in the root source, Codex evidence is absent,
  expensive transcendental chains lack an explicit wall-clock bound, and
  rejected inputs are logged verbatim.
- Profile 15 remains a profile-level surface substitution only; its Registry
  admission is blocked. Profile 16 remains unresolved because of composite
  licensing, external references and missing direct Claude evidence.

## Evidence and authority

Curator exact drafts for profiles 6, 9 and 10 are quarantined under the ignored
path `research-evidence/wave003-staging/`. They intentionally omit `[review]`
blocks. Review A re-reviewed the exact fields and returned conditional/blocking
findings. Review B independently reviewed the profile-20 replacement and
returned conditional findings. No worker approved its own candidate, and no
official manifest was created.

The prior distinct Closure Agent decisions remain binding: profiles 2, 3, 6, 9
and 10 are blocked pending complete evidence; 17 and 18 are rejected pending a
fresh evidence bundle; profile 15's substitution is accepted at profile level
but admission is blocked; the original profile-20 candidate is rejected.

## Verification baseline

Previously verified and preserved:

- `cargo fmt --check`
- workspace Clippy with `-D warnings`
- `cargo test --workspace`: 111 tests
- CLI suite: 24 tests
- release FTS5 test: 1 passing locally
- Git hashing tests: 2 passing
- `./scripts/verify.sh`
- inventory and layout checks

The staging drafts were schema-validated with complete temporary review fields
before those fields were removed to avoid premature approval claims. Hosted
Ubuntu/macOS/Windows FTS5 evidence remains unavailable because no push was
authorized.

## Remaining work

1. Obtain a Closure decision for explicit profile-6/profile-9 standard-only
   substitutions, with aggregate surface-diversity evidence.
2. Build and independently review an immutable profile-10 read-only adapter, or
   reject that candidate.
3. Complete profile-15 admission, resolve profile 16, and correct/replace the
   profile-20 MCP source.
4. Add only Closure-accepted manifests to `catalog/official/manifests/` and
   rebuild/search the real Registry incrementally.
5. Obtain hosted release FTS5 logs after explicit push authorization.
6. Rerun full verification, regenerate inventories through the intended
   generator, and write the final WAVE report/summary.

## Operational constraints

- No commit or push was performed.
- Inherited dirty-tree changes were preserved.
- No WAVE-004 functionality was implemented.
- Do not count evaluation profiles or quarantined drafts as Registry entries.
- Do not add an official manifest without the separated Curator → independent
  Admission Review → Closure chain.

## Practical summary

Ossus's local search engine works. The official catalog is empty because the
remaining candidates do not yet prove the exact host surfaces and enforcement
boundaries required by their seed profiles. The safe next step is to close or
reject those gaps, not to fill the count by relabeling resources.
