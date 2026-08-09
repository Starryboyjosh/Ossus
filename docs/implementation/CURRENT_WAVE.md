# Current implementation WAVE

```text
WAVE-003 — Registry and local search
```

Status: **in progress**. Security Gate S1 was closed by the named project owner, Starryboyjosh, on 2026-08-06 in `08-operations/WAVE-002_GATE_S1_CLOSURE.md`. This is a historical fact; ADR-020 governs all new admission and closure decisions.

WAVE-003 implements the disposable local SQLite/FTS Registry index, deterministic rebuild, trusted-metadata search and show APIs, Registry status, and the corresponding CLI commands. It reconciles 20 seed profiles into governed dispositions and currently has a provisional 16 admission-bearing target. Each admitted resource must be independently admitted by a Curator Agent, Admission Review Agent, and distinct Closure Agent; automated validation alone cannot substitute for that decision, and entries must not be fabricated.

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
| Concrete candidates independently agent-admitted | 2 — profiles 6 and 9 crossed Curator → independent Review → Closure; 14 provisional admission-bearing slots remain; the final sprint admitted no additional candidate |
| Final-admission sprint | Profile 15 independently blocked; profiles 5, 7, 11 and 12 have explicit R3 amendment packets; profile 16 remains valid and unfilled; profiles 1, 3, 4 and 14 were confirmed candidate-rejected/profile-valid |
| WAVE-003 report and final verification | Reconciliation report, official manifests, amendment packets, sprint handoff and closure review written; local verification passes; hosted FTS5 evidence and remaining admissions pending |

## Preceding WAVEs

- WAVE-000 is complete. Security Gate S0 was closed on 2026-08-04 by the project owner in `08-operations/WAVE-000_GATE_S0_CLOSURE.md`.
- WAVE-001 is implemented and verified.
- WAVE-002 is implemented and independently reviewed. It delivered typed core contracts, canonical TOML manifest loading, bounded parsing, taxonomy loading, deterministic semantic validation, CLI diagnostics, the ADR-017 TOML conversion, and the executable fixture corpus. Evidence is in `08-operations/WAVE-002_T4_REPORT.md`.

## Scope boundary

Do not implement WAVE-004 or later behavior. Remote Registry synchronization, resource-body installation, Resolver behavior, Researcher automation, activation, and future command groups remain outside WAVE-003.
