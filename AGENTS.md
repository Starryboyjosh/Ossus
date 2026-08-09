# Agent instructions for Ossus

## Current implementation state

The repository has implemented **WAVE-002** and closed Security Gate S1 under the governance then in force. **WAVE-003 is in progress**; it is not complete until all acceptance evidence, including 20 real seed entries explicitly admitted by the Agent Review Authority, is recorded.

ADR-020 and `docs/AGENT_AUTHORITY.md` govern all new decisions. The Closure Agent has the final word; human review is optional evidence, not a required approval step. Preserve role separation between implementer, independent reviewer, and Closure Agent.

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
- Produce or update the WAVE's technical and practical reader summary under `docs/implementation/08-operations/wave-summaries/` before closure.
- For a genuine repeated block, use `BLOCKED_DIAGNOSTIC_TEMPLATE.md` and include complete logs plus only directly involved code files.

## Security WAVEs

The following require a Security Review Agent and a distinct Closure Agent:

`WAVE-000`, `WAVE-007`, `WAVE-010`, `WAVE-017`, `WAVE-018`, `WAVE-020`, `WAVE-021`, `WAVE-023`.

Any implementation agent may assist with bounded work, but its output must be attributed and independently reviewed. Model names are configuration, not protocol. If no capable independent Security Review Agent and Closure Agent are available, pause the security WAVE rather than allowing an implementer to self-approve.
