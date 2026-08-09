# WAVE-000 preparation and change summary

> **Historical evidence:** ADR-020 and `docs/AGENT_AUTHORITY.md` supersede this document's authority requirements for decisions on or after 2026-08-07. This note does not change recorded facts.

## Status

- WAVE: `WAVE-000 — Opus 5 security and architecture review`
- Date: 2026-08-03
- Repository state: scaffold; no Git metadata is present in the workspace
- Gate: S0 remains open
- Outcome: governance updated; security review execution pending Opus 5

WAVE-000 cannot be claimed complete in this session. The revised plan requires an
Opus 5 security and architecture review, explicit finding dispositions, and human
closure. Opus 5 is not callable in the current environment, so no substitute
report or gate closure was fabricated.

## Change summary

- Removed the former unavailable security-model requirement from the active implementation plan and model-role configuration.
- Assigned Opus 5 as the required owner and final model reviewer for security WAVEs.
- Added Luna Max as optional, attributed implementation and test support without
  security approval authority.
- Replaced the former multi-model reconciliation flow with a single Opus 5 findings
  disposition flow and retained human gate closure.
- Renamed the WAVE-000 and prompt files so active filenames match the new policy.
- Preserved the Spanish original-context documents as immutable historical records.
- Did not change product code, schemas, policies, goldens, or security thresholds.
- Did not commit or push.

## Preparation completed

- [x] Read the mandatory repository documents and WAVE-000 prompts.
- [x] Identify active references to the former model and multi-model review flow.
- [x] Update governance, security gates, roadmap, WAVE assignments, and prompts.
- [x] Update canonical and mirrored implementation documents consistently.
- [x] Define the Opus 5, Luna Max, orchestrator, and human authority boundaries.
- [x] Confirm that no plugin or dependency is required for WAVE-000.
- [x] Preserve historical Spanish source records rather than rewriting history.
- [ ] Run Opus 5 security and architecture plan review.
- [ ] Disposition every Opus 5 finding.
- [ ] Obtain human Security Gate S0 closure.

## Model availability and tool decision

No plugin was installed. Opus 5 and Luna Max are model identities, not repository
plugins. This environment currently exposes neither model as a callable delegated
agent: the Opus 5 call is unavailable, and an attempted Luna worker spawn was
rejected as an unknown model. Their work is therefore pending and is not simulated
by the primary agent.

## Security role logic

```text
                         optional bounded tasks
                      +--------------------------+
                      v                          |
authoritative plan -> Opus 5 security owner -> Luna Max implementation support
                      |                          |
                      +------ verifies ----------+
                      |
                      v
              finding dispositions
                      |
                      v
         primary orchestrator evidence analysis
                      |
                      v
              human Gate S0 decision
```

Opus 5 owns security judgment and must assess the final diff and evidence. Luna
Max or another implementation agent may produce bounded code, tests, or evidence,
but every contribution must be attributed and reviewed by Opus 5. The primary
orchestrator may integrate results, run deterministic checks, and explain the
evidence; it cannot label its own output as Opus 5 work. Only a human closes a
security gate.

This single-model design intentionally gives up the former independent-model
cross-check. That loss of reviewer independence is a residual risk that must stay
visible to the human approver; deterministic tests, complete evidence, and human
closure become correspondingly more important.

## WAVE-000 checklist

### Opus 5 review

- [ ] Opus 5 reviews the full authoritative planning package.
- [ ] The report cites concrete evidence and affected invariants.
- [ ] Findings include severity, required plan change, and test requirement.
- [ ] The report ends with `BLOCK`, `REVISE`, or
  `READY FOR HUMAN DECISION`.
- [ ] Uncertainty and the lack of an independent model reviewer are explicit.

### Finding disposition

- [ ] Every finding has a documented disposition.
- [ ] Critical and high findings remain blocking until resolved by evidence.
- [ ] Medium findings have a correction or named accepted-risk owner.
- [ ] Luna Max or other support contributions are attributed and verified.
- [ ] Required plan, threat-model, trust-boundary, WAVE-order, test, and ADR
  changes are traceable.

### Gate S0 closure

- [ ] No critical architecture finding remains unresolved.
- [ ] No high security finding remains unresolved.
- [ ] Trust boundaries and WAVE order agree.
- [ ] Required deterministic verification evidence passes.
- [ ] No schema, golden, threshold, or security policy was weakened.
- [ ] A named human approver records the decision and residual risks.
- [ ] `CURRENT_WAVE.md` advances only after human closure.
- [ ] The standard WAVE report is completed after all preceding items pass.

## Existing verification baseline

| Command | Result |
|---|---|
| `rustc --version` | PASS — `rustc 1.97.1` |
| `cargo --version` | PASS — `cargo 1.97.1` |
| `./scripts/verify.sh` | STOPPED — existing `cargo fmt --check` differences in `ossus-cli` and `ossus-resolver` |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --workspace --all-features` | PASS — 9 tests across 13 suites |
| `cargo run -q -p ossus -- status` | PASS — WAVE-000 scaffold state |

## Required continuation

1. Make Opus 5 available in a session that can read the complete repository.
2. Run `PLAN_REVIEW_OPUS5.md` against the authoritative planning package.
3. Process the report with `FINDINGS_DISPOSITION.md`.
4. Apply evidence-supported plan, threat-model, test-plan, WAVE-order, and ADR
   corrections.
5. Obtain the human decision using `HUMAN_SECURITY_CLOSURE.md`.
6. Complete the standard WAVE report and advance `CURRENT_WAVE.md` only after
   human Gate S0 closure.

Until those steps are complete, WAVE-001 and later WAVEs remain blocked.
