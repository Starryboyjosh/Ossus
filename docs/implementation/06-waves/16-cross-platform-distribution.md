# WAVE-016 — Cross-platform distribution

**Phase:** Phase 4  
**Assigned role:** Implementation agent and release owner  
**Depends on:** WAVE-015  
**Security WAVE:** no

## Objective

Provide tested binary distribution and platform-native data paths.

## In scope

- Define target matrix.
- Build archive/installer layouts.
- Add completions.
- Implement upgrade behavior.
- Test Windows path/link fallbacks.
- Document macOS/Linux locations.

## Out of scope

- Auto-update daemon.
- Unsigned execution bypass.

## Expected deliverables

- Release automation.
- Platform smoke tests.
- Installation docs.

## Required tests and evidence

- Clean install.
- Custom OSSUS_HOME.
- Spaces/Unicode paths.
- Windows copy fallback.
- Upgrade/rollback.

## Acceptance criteria

- Targets pass CLI and activation smoke tests.
- Checksums publish.
- Installer never silently edits shell profiles.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt. Auto-update requires a separate ADR and security WAVE.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
