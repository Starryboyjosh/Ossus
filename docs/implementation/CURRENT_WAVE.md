# Current implementation WAVE

```text
WAVE-002 — Trusted specifications and taxonomy
```

Status: **implemented and independently reviewed; Security Gate S1 closure is pending named-human approval**. WAVE-003 remains blocked until the project owner records that decision in `08-operations/WAVE-002_GATE_S1_CLOSURE.md`.

WAVE-002 delivered typed core contracts, canonical TOML manifest loading, bounded parsing, taxonomy loading, deterministic semantic validation, CLI diagnostics, the ADR-017 TOML conversion, and an executable negative-fixture corpus. The implementation report and Opus 5 review evidence are in `08-operations/WAVE-002_T4_REPORT.md`.

## Gate S1 evidence

| Requirement | State |
|---|---|
| schemas reject unknown major versions | Done — canonical fixtures and typed schema-version checks fail closed |
| parser budgets tested | Done — byte, UTF-8, nesting, string and collection limits have executable tests |
| canonical/origin separation explicit | Done — F-12 origin fields are rejected and cannot deserialize into canonical state |
| taxonomy and policy hashes supported | Done — the lockfile schema requires canonical lowercase `sha256:<64 hex>` identities and layout regression checks reject missing or malformed values |
| Opus 5 final review | Done — review addendum and final evidence recorded in the WAVE report |
| named-human closure | **Open** — model review is evidence, not certification |

## Preceding WAVEs

- WAVE-000 is complete. Security Gate S0 was closed on 2026-08-04 by the project owner in `08-operations/WAVE-000_GATE_S0_CLOSURE.md`.
- WAVE-001 is implemented and verified. Its remote CI evidence can be collected after the authorized first commit is pushed.

## Next WAVE

WAVE-003 must not begin until Gate S1 is closed by a named human against `07-prompts/HUMAN_SECURITY_CLOSURE.md`.
