# Current implementation WAVE

```text
WAVE-003 — Registry and local search
```

Status: **in progress**. Security Gate S1 was closed by the named project owner, Starryboyjosh, on 2026-08-06 in `08-operations/WAVE-002_GATE_S1_CLOSURE.md`. This is a historical fact; ADR-020 governs all new admission and closure decisions.

WAVE-003 implements the disposable local SQLite/FTS Registry index, deterministic rebuild, trusted-metadata search and show APIs, Registry status, and the corresponding CLI commands. The final coverage authority recommends 16 current admission-bearing responsibilities (9 unchanged required profiles and 7 required profiles with explicit substitutions) plus four intentionally unresolved future profiles; this is a minimum useful catalog, not a quota. Each admitted resource must be independently admitted by a Curator Agent, Admission Review Agent, and distinct Closure Agent; automated validation alone cannot substitute for that decision, and entries must not be fabricated.

## WAVE-003 acceptance state

| Requirement | State |
|---|---|
| Gate S1 historical named-human closure | Done — historical V0 authority ratification and correlated-review-risk acceptance |
| SQLite schema and versioned migrations | Implemented and locally verified |
| Deterministic atomic rebuild | Implemented and locally verified |
| Exact, capability, category and FTS search | Implemented and locally verified |
| Surface, source, runtime and risk filters | Implemented and locally verified |
| `search`, `show`, `registry status`, `registry reindex` CLI | Implemented and locally verified |
| Seed profile reconciliation | Final coverage authority recorded for all 20 profiles: 9 `SEED_REQUIRED`, 7 `SEED_REQUIRED_WITH_SUBSTITUTION`, and 4 `INTENTIONALLY_UNRESOLVED`; provisional planning denominator is 16, while the active completion obligation remains 20 |
| Concrete candidates independently agent-admitted | 2 — profiles 6 and 9 crossed Curator → independent Review → Closure; 14 current admission slots remain; the final coverage pass admitted no additional candidate |
| Final-admission and coverage authority | Profile 15 is useful but deferred pending adapter/freshness/redaction evidence; Closure accepted profile-only corrections P5 R3, P7 R2, P11 R2 and P12 R3; profile 16 remains valid and unfilled; no candidate was admitted |
| WAVE-003 report and final verification | Coverage authority, reconciliation report, official manifests, amendment packets, handoffs and closure evidence written; local verification passes; hosted Ubuntu, Arch-container, macOS and Windows FTS5 plus complete CI run 21 pass; positive seed diversity and admissions remain pending |

## Preceding WAVEs

- WAVE-000 is complete. Security Gate S0 was closed on 2026-08-04 by the project owner in `08-operations/WAVE-000_GATE_S0_CLOSURE.md`.
- WAVE-001 is implemented and verified.
- WAVE-002 is implemented and independently reviewed. It delivered typed core contracts, canonical TOML manifest loading, bounded parsing, taxonomy loading, deterministic semantic validation, CLI diagnostics, the ADR-017 TOML conversion, and the executable fixture corpus. Evidence is in `08-operations/WAVE-002_T4_REPORT.md`.

## Scope boundary

Do not implement WAVE-004 or later behavior. Remote Registry synchronization, resource-body installation, Resolver behavior, Researcher automation, activation, and future command groups remain outside WAVE-003.
