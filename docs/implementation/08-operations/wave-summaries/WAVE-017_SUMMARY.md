# WAVE-017 — Researcher security design

## Status

Planned. The repository records WAVE-003 as in progress; no completion evidence for this later WAVE was found.

## Technical summary

Define the Researcher threat model, quarantine budgets and locations, evidence-bundle schema, connector interface, staging/CI model, and legal/privacy decision points before Researcher code exists. Candidate execution and approval are excluded.

## Practical plain-language summary

Set the safety rules for handling untrusted candidate material before building the feature that collects it.

## Expected evidence/deliverables

- Researcher security specification and attributed implementation-support evidence.
- Threat scenarios for archives, links, hooks, submodules, large repositories, malicious manifests, CI tokens, and evidence/canonical separation.
- Agent Review Authority design-gate assessment.

## Dependencies/gates

Depends on WAVE-016. It establishes the design basis for Gate S5 and WAVE-018.

## Remaining work

Complete the design, define bounded attacker inputs and destinations, prepare the required attack-test design, and obtain Agent Review Authority assessment.
