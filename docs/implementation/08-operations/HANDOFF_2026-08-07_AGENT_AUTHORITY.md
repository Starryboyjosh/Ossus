# Ossus handoff — Agent-final governance and WAVE inventory

**Date:** 2026-08-07
**Branch:** `main`
**Base revision:** `ec9f1aa` (`feat: add local registry index and search`)
**Commit/push:** not performed

## Scope completed

This repository-wide pass mapped the WAVE plan, delegated independent audits,
made active governance agent-final, and created a technical plus practical
summary for every WAVE from 000 through 024. The working tree is intentionally
uncommitted; inspect it with `git diff`.

### Normative governance

- Added [`docs/AGENT_AUTHORITY.md`](../../AGENT_AUTHORITY.md).
- Added ADR-020 to both decision-log copies.
- New decisions use separate Implementer Agent, Independent Review Agent
  (Security Review Agent for security-sensitive work), and Closure Agent roles.
- The Closure Agent has the final accept/reject/blocked decision; human review
  is optional evidence, not a required approval step.
- Critical/high findings still fail closed and require correction plus
  independent re-review. Trust boundaries, policy monotonicity, hash checks,
  activation controls, and Researcher restrictions remain intact.
- Schema-v1 `light-human`, `full-human`, and `security-human` values remain for
  compatibility and now describe review depth; a future schema-major migration
  may rename them. Rust enum migration was not attempted.
- Historical WAVE-000–002 records were annotated, not rewritten.

The active policy, product, architecture, security, roadmap, WAVE, prompt,
operations, schema/example, and mirror documents were updated. Editing agents
reported passing mirror checks and `git diff --check`.

### Per-WAVE summaries

Created 25 files under
`docs/implementation/08-operations/wave-summaries/` plus its README index:
[`wave-summaries/README.md`](wave-summaries/README.md). Each summary includes
status, technical summary, practical plain-language summary, evidence or
expected evidence, dependencies/gates, and remaining work. Planned WAVEs are
not described as implemented.

### Repository graph

Graphify analyzed 187 supported files (~101,134 words): 42 code files and 145
documents. `graphify-out/` contains `graph.json` (1,521 nodes, 2,266 post-build
edges), `GRAPH_REPORT.md` (147 labeled communities), and `graph.html`.
Graph health reported 82 dangling semantic endpoints and 129 directed / 131
undirected same-endpoint collapses; these warnings were preserved in the report.

### Verification baseline

Before documentation edits, `rtk ./scripts/verify.sh` passed formatting,
Clippy, layout checks, 105 workspace tests plus doctests/CLI snapshots, and
dependency policy checks. Existing unmatched allowed-license warnings were
reported for BSD-3-Clause, ISC, MPL-2.0, and Zlib. Full post-edit verification
has not yet been run.

## WAVE-003 status

WAVE-003 remains **in progress**. The current code provides most local Registry
mechanics: SQLite/FTS5 schema and migration, deterministic staging rebuild,
metadata-only search/show/status/reindex, filters, malformed-manifest exclusion,
duplicate/source conflict handling, and versioned CLI output.

It cannot honestly close yet because:

1. `catalog/official/manifests/` has no 20-entry real seed catalog.
2. The required provenance/license seed report is absent.
3. The WAVE-003 completion report is absent.
4. FTS5 evidence is local/Linux only; documented macOS and Windows release
   evidence still needs runners.
5. CLI tests do not comprehensively validate every committed JSON schema and
   every required error/filter path.

The seed research agent was interrupted for this handoff. Do not fabricate
entries or approval evidence. Use primary upstream sources, immutable commits,
tree/content hashes, license evidence, and Curator Agent → Admission Review
Agent → Closure Agent records.

## Next agent checklist

1. Run `rtk git diff --check` and `rtk ./scripts/verify.sh` after the edits.
2. Run the layout/mirror check and inspect any divergence.
3. Scan active docs for stale mandatory-human or fixed-Opus requirements; keep
   historical annotations.
4. Research and verify 20 concrete WAVE-003 candidates; create manifests,
   provenance/license evidence, and agent admission/closure records.
5. Add the WAVE-003 report and update its summary with final evidence.
6. Add focused CLI/schema-conformance tests and obtain release-mode FTS5
   evidence for supported targets where runners exist.
7. Only then update `CURRENT_WAVE.md` and WAVE-003 status. Do not implement
   WAVE-004 or later behavior during this work.
8. Rebuild Graphify if final-tree graph artifacts are needed.
9. Do not commit or push without explicit owner authorization.

## Constraints to preserve

Stored ≠ approved ≠ installed ≠ active; origin is evidence only; policy denial
precedes scoring/mutation; resolve is local/deterministic; activation verifies
immutable content; Researcher never approves or activates. Missing credentials,
remote runners, signing keys, or network state is an execution block, not a
reason to invent evidence. Publication/deployment/messaging and VCS commit/push
remain explicit operational actions even though the Closure Agent decides
technical readiness.

## Key files

- [`docs/AGENT_AUTHORITY.md`](../../AGENT_AUTHORITY.md)
- [`docs/product/DECISION_LOG.md`](../../product/DECISION_LOG.md)
- [`CURRENT_WAVE.md`](../CURRENT_WAVE.md)
- [`WAVE_INDEX.md`](../06-waves/WAVE_INDEX.md)
- [`03-registry-and-search.md`](../06-waves/03-registry-and-search.md)
- [`wave-summaries/README.md`](wave-summaries/README.md)
- [`WAVE_REPORT_TEMPLATE.md`](WAVE_REPORT_TEMPLATE.md)

All delegated documentation/audit work returned. The remaining seed research
turn and idle summary turn were interrupted so no edits land after this file.
