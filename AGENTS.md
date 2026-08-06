# Agent instructions for Ossus

## Current implementation state

The repository has implemented **WAVE-002** and is awaiting named-human closure of Security Gate S1. WAVE-003 remains blocked until that closure is recorded.

Do not implement later WAVEs merely because their folders or command names exist.

## Mandatory reading order

1. `README.md`
2. `docs/product/OSSUS_MASTER_CONTEXT.md`
3. `docs/product/DECISION_LOG.md`
4. `docs/security/THREAT_MODEL.md`
5. `docs/roadmap/PHASES_AND_GATES.md`
6. `docs/implementation/06-waves/WAVE_INDEX.md`
7. the assigned WAVE
8. the matching prompt under `docs/implementation/07-prompts/`

The Spanish documents under `docs/implementation/00-original-context/` are historical design records. Use them only when a restructuring is being considered or an active document is ambiguous.

## Permanent product invariants

- Stored does not mean approved.
- Approved does not mean installed.
- Installed does not mean active.
- The approved Registry is an instruction channel into the host agent.
- Origin metadata is evidence, never canonical Resolver authority.
- Policy denial happens before scoring or mutation.
- Resolve is local and deterministic by default.
- Activation verifies immutable source content.
- Only selected resources enter host-visible paths.
- Researcher never approves or activates.
- No custom sandbox or static-analysis engine.
- No network or external model call in normal resolve.

## Rust rules

- Rust edition 2024.
- Keep domain logic outside `ossus-cli`.
- Preserve crate dependency direction.
- `unsafe` is forbidden unless a separately approved security ADR changes that rule.
- Use typed errors in libraries.
- Use platform path APIs, never path string concatenation.
- Bound parsing, traversal, and collection sizes.
- Do not silently recover from integrity or policy failures.

## Implementation workflow

- Implement only the assigned WAVE.
- Do not commit or push unless the human explicitly requests it.
- Correct normal errors before escalating.
- Never weaken a schema, golden case, threshold, or security policy merely to make tests pass.
- Produce `docs/implementation/08-operations/WAVE_REPORT_TEMPLATE.md` on completion.
- For a genuine repeated block, use `BLOCKED_DIAGNOSTIC_TEMPLATE.md` and include complete logs plus only directly involved code files.

## Security WAVEs

The following require Opus 5 security ownership and human closure:

`WAVE-000`, `WAVE-007`, `WAVE-010`, `WAVE-017`, `WAVE-018`, `WAVE-020`, `WAVE-021`, `WAVE-023`.

Luna Max or another implementation agent may assist with bounded implementation and test work, but its output must be attributed and reviewed by Opus 5. If Opus 5 is unavailable, pause the security WAVE. Do not silently substitute another model.
