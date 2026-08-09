# Current implementation WAVE

```text
WAVE-003 — Registry and local search
```

Status: **in progress**. Security Gate S1 was closed by the named project owner, Starryboyjosh, on 2026-08-06 in `08-operations/WAVE-002_GATE_S1_CLOSURE.md`. This is a historical fact; ADR-020 governs all new admission and closure decisions.

WAVE-003 implements the disposable local SQLite/FTS Registry index, deterministic rebuild, trusted-metadata search and show APIs, Registry status, and the corresponding CLI commands. It also requires 20 real seed resources with provenance and license evidence. Each resource must be independently admitted by a Curator Agent, Admission Review Agent, and distinct Closure Agent; automated validation alone cannot substitute for that decision, and entries must not be fabricated.

## WAVE-003 acceptance state

| Requirement | State |
|---|---|
| Gate S1 historical named-human closure | Done — historical V0 authority ratification and correlated-review-risk acceptance |
| SQLite schema and versioned migrations | Implemented and locally verified |
| Deterministic atomic rebuild | Implemented and locally verified |
| Exact, capability, category and FTS search | Implemented and locally verified |
| Surface, source, runtime and risk filters | Implemented and locally verified |
| `search`, `show`, `registry status`, `registry reindex` CLI | Implemented and locally verified |
| Seed profile reconciliation | Governed dispositions recorded for all 20 profiles; provisional admission-bearing target is 16, with profiles 10, 17, 18 and 20 intentionally unresolved |
| Concrete candidates independently agent-admitted | Pending — official count remains zero; no candidate has crossed Curator → independent Review → Closure |
| WAVE-003 report and final verification | Reconciliation report, interim report/handoff and closure-oriented review written; local verification passes; hosted FTS5 and admission Closure pending |

## Preceding WAVEs

- WAVE-000 is complete. Security Gate S0 was closed on 2026-08-04 by the project owner in `08-operations/WAVE-000_GATE_S0_CLOSURE.md`.
- WAVE-001 is implemented and verified.
- WAVE-002 is implemented and independently reviewed. It delivered typed core contracts, canonical TOML manifest loading, bounded parsing, taxonomy loading, deterministic semantic validation, CLI diagnostics, the ADR-017 TOML conversion, and the executable fixture corpus. Evidence is in `08-operations/WAVE-002_T4_REPORT.md`.

## Scope boundary

Do not implement WAVE-004 or later behavior. Remote Registry synchronization, resource-body installation, Resolver behavior, Researcher automation, activation, and future command groups remain outside WAVE-003.
