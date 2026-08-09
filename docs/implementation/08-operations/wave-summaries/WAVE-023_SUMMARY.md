# WAVE-023 — Researcher end-to-end security audit

## Status

Planned. No repository evidence shows this audit or its closure is complete.

## Technical summary

Audit the full Researcher path with adversarial fixtures covering quarantine escapes, CI/token exposure, source substitution, prompt injection, evidence poisoning, admission bypass, privacy, and cleanup. The WAVE adds no product features; it produces corrections where findings require them.

## Practical plain-language summary

Try to break the entire candidate-research workflow before beta, then fix and recheck every meaningful weakness.

## Expected evidence/deliverables

- Security audit, attributed support evidence, final assessment, and scoped correction WAVEs where needed.
- Tests for all Researcher attacks, cross-platform paths, concurrent jobs, interrupted cleanup, compromised upstream updates, and malicious scanner output.
- Gate S7 evidence for Agent Review Authority.

## Dependencies/gates

Depends on WAVE-022. It provides the security basis for Gate S7 and is required before WAVE-024.

## Remaining work

Run the end-to-end attack suite, resolve all critical and high findings, verify Researcher cannot approve or activate or reach privileged CI, re-review corrections, and obtain Gate S7 assessment.
