# WAVE-020 — External analysis integrations

**Phase:** Phase 5  
**Assigned role:** Opus 5 security owner; optional Luna Max implementation support  
**Depends on:** WAVE-018  
**Security WAVE:** yes

## Objective

Integrate mature scanners into disposable jobs and normalize findings.

## In scope

- Semgrep CLI adapter.
- Dependency scanner adapters.
- License evidence adapter.
- Timeout/resource limits.
- Tool-version capture.
- Normalized findings schema.

## Out of scope

- Custom language scanner.
- Executing candidate tests.
- Treating clean scans as approval.

## Expected deliverables

- Experimental analyze command.
- Scanner interfaces.
- Disposable-runner docs.
- Opus 5 security report and implementation-support attribution.

## Required tests and evidence

- Missing scanner.
- Timeout.
- Malicious output.
- Huge findings.
- Version mismatch.
- Network disabled.

## Acceptance criteria

- Scanner output cannot write canonical state.
- Failure never becomes a clean result.
- No secrets enter jobs.


## Mandatory security workflow

This is a security WAVE.

- Opus 5 owns the assigned security work and reviews the final implementation or audit evidence.
- Luna Max or another implementation agent may assist only through attributed, bounded tasks.
- The human closes the gate.
- There is no automatic replacement if Opus 5 is unavailable.
- Model review is evidence, not certification.


## Copy-ready implementation instruction

Use security implementation and review prompts.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
