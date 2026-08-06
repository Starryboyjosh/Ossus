# WAVE-009 — Complete CLI vertical slice

**Phase:** Phase 3  
**Assigned role:** Implementation agent  
**Depends on:** WAVE-008  
**Security WAVE:** no

## Objective

Deliver a coherent end-to-end workflow from initialization to verified activation.

## In scope

- Finalize init, config, search, scan, resolve, explain, activate, deactivate, lock verify and doctor.
- Add versioned JSON outputs and stable exit codes.
- Generate shell completions if low-risk.
- Add an end-to-end fixture repository.
- Write user and maintainer docs.

## Out of scope

- Registry network sync.
- Codex.
- Researcher.
- TUI.

## Expected deliverables

- V0 vertical slice.
- CLI reference.
- End-to-end test script.
- Example project.

## Required tests and evidence

- Clean workflow.
- Offline workflow.
- Low confidence.
- Policy denial.
- Activation/rollback.
- Lock verification.
- JSON contracts.

## Acceptance criteria

- A new user completes the path without editing internal files.
- Default workflow makes zero model calls.
- Exit codes are documented and tested.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt. Keep resolve and activate as separate consent boundaries.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
