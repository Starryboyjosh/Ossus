# Security governance

## Roles

- security implementer;
- implementation support;
- architecture owner;
- human security approver.

## Project assignment

- Opus 5: required security owner and final model reviewer.
- Luna Max or another implementation agent: optional, attributed implementation and test support.
- Human: final authority.

## Required evidence per security WAVE

1. implementation report;
2. tests and commands;
3. changed trust boundaries;
4. threat-model delta;
5. Opus 5 security assessment of the final diff and evidence;
6. attribution of all implementation-agent contributions;
7. explicit finding dispositions;
8. residual risks;
9. human closure checklist.

## Disagreement process

When the Opus 5 assessment conflicts with implementation evidence or another agent's analysis, preserve every claim, identify evidence and the affected invariant, run deterministic tests or obtain human technical review, do not average severities, and document the final decision with residual uncertainty.

## Blocking severities

```text
critical  blocks immediately
high      blocks WAVE closure
medium    requires plan or accepted risk
low       may enter backlog
info      documentation
```

## Model limitations

Model review can find omissions and challenge assumptions.

It cannot guarantee absence of vulnerabilities, replace code execution and tests, certify isolation, approve a resource or close a security WAVE.
