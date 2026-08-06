# Ossus WAVE index

Each WAVE is scoped, ordered and gated.

The **Security** column marks WAVEs Opus 5 owns and implements. A WAVE marked `No` may still **close a security gate**, which requires Opus 5 review of its final diff and evidence plus human closure; those are listed in the **Closes gate** column. Delegating the implementation of such a WAVE is permitted; delegating its gate closure is not.

| WAVE | Phase | Title | Assigned role | Security | Closes gate | Depends on |
|---|---|---|---|---|---|---|
| WAVE-000 | Phase 0 | Opus 5 security and architecture review | Opus 5 security reviewer; optional Luna Max evidence support | Yes | S0 | None |
| WAVE-001 | Phase 1 | Rust workspace bootstrap | Implementation agent | No | — | WAVE-000 |
| WAVE-002 | Phase 1 | Trusted specifications and taxonomy | Implementation agent with architecture review | No | S1 | WAVE-001 |
| WAVE-003 | Phase 2 | Seed Registry and local search | Implementation agent | No | — | WAVE-002 |
| WAVE-004 | Phase 2 | Bounded project scanner | Implementation agent | No | — | WAVE-002 |
| WAVE-005 | Phase 2 | Deterministic Resolver core | Implementation agent | No | S2 | WAVE-003 and WAVE-004 |
| WAVE-006 | Phase 2 | Golden evaluation harness | Implementation agent and independent reviewer | No | — | WAVE-005 |
| WAVE-007 | Phase 3 | Activation security boundary | Opus 5 security owner; optional Luna Max implementation support | Yes | S3 | WAVE-006 |
| WAVE-008 | Phase 3 | Claude Code adapter | Implementation agent with security review | No | — | WAVE-007 |
| WAVE-009 | Phase 3 | Complete CLI vertical slice | Implementation agent | No | — | WAVE-008 |
| WAVE-010 | Phase 3 | Security hardening and supply chain | Opus 5 security owner; optional Luna Max implementation support | Yes | S4 | WAVE-009 |
| WAVE-011 | Phase 3 | Ossus V0 release candidate | Implementation agent and human release owner | No | — | WAVE-010 |
| WAVE-012 | Phase 4 | Registry synchronization and selective install | Implementation agent with security review | No | — | WAVE-011 |
| WAVE-013 | Phase 4 | Private catalogs and source precedence | Implementation agent | No | — | WAVE-012 |
| WAVE-014 | Phase 4 | Codex adapter | Implementation agent with security review | No | — | WAVE-012 and WAVE-013 |
| WAVE-015 | Phase 4 | Optional local semantic fallback | Implementation agent | No | — | WAVE-014 |
| WAVE-016 | Phase 4 | Cross-platform distribution | Implementation agent and release owner | No | — | WAVE-015 |
| WAVE-017 | Phase 5 | Researcher security design | Opus 5 security owner; optional Luna Max implementation support | Yes | — | WAVE-016 |
| WAVE-018 | Phase 5 | Passive quarantine intake and evidence | Opus 5 security owner; optional Luna Max implementation support | Yes | S5 | WAVE-017 |
| WAVE-019 | Phase 5 | GitHub discovery connector | Implementation agent with mandatory Opus 5 review | No | — | WAVE-018 |
| WAVE-020 | Phase 5 | External analysis integrations | Opus 5 security owner; optional Luna Max implementation support | Yes | — | WAVE-018 |
| WAVE-021 | Phase 5 | Human review and admission workflow | Opus 5 security owner; optional Luna Max implementation support | Yes | S6 | WAVE-019 and WAVE-020 |
| WAVE-022 | Phase 5 | Reddit community evidence | Implementation agent with privacy/security review | No | — | WAVE-021 |
| WAVE-023 | Phase 5 | Researcher end-to-end security audit | Opus 5 security auditor; optional Luna Max evidence support | Yes | — | WAVE-022 |
| WAVE-024 | Phase 6 | Ossus beta release | Release owner with architecture/security approval | No | S7 | WAVE-023 |
