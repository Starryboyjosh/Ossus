# WAVE-005 — Deterministic Resolver core

**Phase:** Phase 2  
**Assigned role:** Implementation agent  
**Depends on:** WAVE-003 and WAVE-004  
**Security WAVE:** no — implementation may be delegated  
**Closes:** **Security Gate S2 — Resolver policy.** Gate closure requires Opus 5 review of the final diff and evidence, then a named human closing against `07-prompts/HUMAN_SECURITY_CLOSURE.md`. WAVE-007 must not begin before S2 is closed.

## Objective

Map task plus project to a minimal compatible resource set with explanations.

## In scope

- Normalize tasks and expand governed aliases.
- Generate capability evidence.
- Retrieve Registry candidates.
- Apply compatibility and policy hard filters.
- Implement deterministic weighted set cover and stable tie-breaking.
- Implement confidence and explanations.
- Add dry-run `resolve` and `explain`.

## Out of scope

- Activation.
- External LLM fallback.
- Embeddings.
- Registry sync.

## Expected deliverables

- Selection-plan contract.
- Versioned algorithm.
- Human and JSON output.
- Reason codes.
- Scope-aware policy resolution implementing **ADR-016**.
- **Gate S2 closure record**, prepared for the distinct Closure Agent.

## Required tests and evidence

- Coverage and redundancy.
- Policy denial before scoring.
- Surface mismatch.
- R4 implicit denial.
- Ambiguous low confidence.
- Stable tie-breaking.
- Active-count limits.
- Byte-equivalent normalized output.
- **Policy monotonicity (F-01, ADR-016).** An adversarial fixture project whose `.ossus/config.toml` and `.ossus/policy.toml` attempt `risk_max = "R4"`, `allow_implicit_r4 = true`, `block = []`, `require_hash_verification = false`, an upward parser-budget change, and a priority-300 registry entry shadowing an `official.*` id. Assert:
  1. effective policy is byte-identical to the user/global policy;
  2. any selection that would require the relaxation is denied with the documented policy exit code;
  3. every attempted relaxation appears in `explain` output and in the audit event;
  4. the shadowing registry entry is either refused or requires recorded trust, and is attributed in output.

## Acceptance criteria

- Zero external model calls.
- Zero network calls.
- Forbidden resources never select.
- Low confidence does not overselect or activate.
- A project-scoped file cannot relax effective policy by any path.
- Gate S2 requirements pass, and **Gate S2 is closed by a named human** before WAVE-007 begins.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt. Do not weaken future goldens to accommodate the algorithm.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
