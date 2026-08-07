# Current implementation WAVE

```text
WAVE-003 — Registry and local search
```

Status: **in progress**. Security Gate S1 was closed by the named project owner, Starryboyjosh, on 2026-08-06 in `08-operations/WAVE-002_GATE_S1_CLOSURE.md`.

WAVE-003 implements the disposable local SQLite/FTS Registry index, deterministic rebuild, trusted-metadata search and show APIs, Registry status, and the corresponding CLI commands. It also requires 20 real seed resources with provenance and license evidence. Those resources are not approved until the named human reviews the concrete candidate package; automated validation cannot substitute for that decision.

## WAVE-003 acceptance state

| Requirement | State |
|---|---|
| Gate S1 named-human closure | Done — V0 authority ratified and correlated-review risk accepted |
| SQLite schema and versioned migrations | In progress |
| Deterministic atomic rebuild | In progress |
| Exact, capability, category and FTS search | In progress |
| Surface, source, runtime and risk filters | In progress |
| `search`, `show`, `registry status`, `registry reindex` CLI | In progress |
| 20 real seed candidates researched | Pending |
| 20 concrete candidates explicitly human-approved | Pending — mandatory checkpoint |
| WAVE-003 report and final verification | Pending |

## Preceding WAVEs

- WAVE-000 is complete. Security Gate S0 was closed on 2026-08-04 by the project owner in `08-operations/WAVE-000_GATE_S0_CLOSURE.md`.
- WAVE-001 is implemented and verified.
- WAVE-002 is implemented and independently reviewed. It delivered typed core contracts, canonical TOML manifest loading, bounded parsing, taxonomy loading, deterministic semantic validation, CLI diagnostics, the ADR-017 TOML conversion, and the executable fixture corpus. Evidence is in `08-operations/WAVE-002_T4_REPORT.md`.

## Scope boundary

Do not implement WAVE-004 or later behavior. Remote Registry synchronization, resource-body installation, Resolver behavior, Researcher automation, activation, and future command groups remain outside WAVE-003.
