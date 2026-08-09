# WAVE-018 — Passive quarantine intake and evidence

## Status

Planned. No repository evidence shows implementation or closure.

## Technical summary

Build passive intake for manual URLs and local paths: resolve immutable commits, quarantine candidates, inventory within limits, apply archive/symlink/submodule rules, hash content, and generate untrusted evidence bundles. It must not execute candidates or write canonical manifests.

## Practical plain-language summary

Safely collect and describe potential resources without running them or treating their claims as trusted.

## Expected evidence/deliverables

- Experimental ingest command plus quarantine and evidence APIs.
- Attack tests for hooks, submodules, symlink escapes, archive traversal, excessive size/count, reserved or Unicode names, and interrupted cleanup.
- Agent Review Authority security assessment and Gate S5 implementation evidence.

## Dependencies/gates

Depends on WAVE-017 and implements the Gate S5 design. It is required before the discovery and analysis work in WAVEs 019 and 020.

## Remaining work

Implement the bounded intake path, demonstrate non-execution and secret exclusion, type evidence as untrusted, run attack tests, and obtain Agent Review Authority assessment.
