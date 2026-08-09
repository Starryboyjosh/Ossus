# WAVE-010 — Security hardening and supply chain

**Phase:** Phase 3  
**Assigned role:** Opus 5 security owner; optional Luna Max implementation support  
**Depends on:** WAVE-009  
**Security WAVE:** yes

## Objective

Harden dependencies, CI, release inputs, parsers, audit output and policy bypass resistance.

## In scope

- Integrate dependency advisory and license tooling.
- Audit CI permissions.
- Define workflow action pinning. **F-14 is partially satisfied at WAVE-001**, which pins every `uses:` to a commit SHA, adds a `cargo deny check` job and pins the toolchain to an explicit version. This WAVE owns the remaining work: the written pinning and update policy, automated detection of an unpinned or drifted reference, and the release-path review that WAVE-001 does not cover. Verify the WAVE-001 pins rather than assuming them.
- Add parser fuzz/property tests.
- Test Registry and policy tampering.
- Design checksums/provenance.
- Review data redaction.

## Out of scope

- Public publishing unless separately approved.
- Researcher controls.
- Custom scanner.

## Expected deliverables

- Supply-chain controls.
- Security suite.
- Opus 5 security report.
- Implementation-support attribution and final evidence assessment.
- Distinct Closure Agent Gate S4 decision.

## Required tests and evidence

- Dependency and license audit.
- Malformed TOML/JSON properties.
- Audit redaction.
- Config precedence bypass.
- Namespace shadowing.
- CI permissions.

## Acceptance criteria

- No unresolved high dependency/CI finding.
- Artifacts can be checksum-verified.
- Quiet mode cannot suppress security warnings.
- Gate S4 closes.


## Mandatory security workflow

This is a security WAVE.

- A capable independent Security Review Agent reviews the final implementation and evidence.
- Implementation agents may assist only through attributed, bounded tasks.
- A distinct Closure Agent closes the gate and has the final word.
- Human review is optional additional evidence.


## Copy-ready implementation instruction

Use the security implementation and independent review prompts.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
