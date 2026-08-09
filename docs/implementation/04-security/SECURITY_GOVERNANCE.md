# Security governance

## Roles

- Implementer Agent;
- Independent Review Agent;
- Security Review Agent for security-sensitive work;
- Closure Agent.

## Project assignment

The Implementer Agent prepares the change and evidence. A Security Review Agent independently reviews every security WAVE's final diff and evidence. A distinct Closure Agent makes the final accepted, rejected, or blocked decision. The Closure Agent must not have implemented or independently reviewed the same change. Human review may supplement evidence but is not required; no fixed model has exclusive review or closure authority.

## Required evidence per security WAVE

1. implementation report;
2. tests and commands;
3. changed trust boundaries;
4. threat-model delta;
5. independent Security Review Agent assessment of the final diff and evidence;
6. Implementer, reviewer, and Closure Agent identifiers plus attribution of contributions;
7. explicit finding dispositions;
8. residual risks;
9. Closure Agent decision record, including conflict-of-interest attestation.

## Disagreement process

When an independent review conflicts with implementation evidence or another agent's analysis, preserve every claim, identify evidence and the affected invariant, run deterministic tests or obtain additional independent review, do not average severities, and document the Closure Agent's final decision with residual uncertainty. Missing, contradictory, or unverifiable evidence fails closed.

## Blocking severities

```text
critical  blocks immediately
high      blocks WAVE closure
medium    requires plan or accepted risk
low       may enter backlog
info      documentation
```

## Model limitations

Agent review can find omissions and challenge assumptions.

It cannot guarantee absence of vulnerabilities, replace code execution and tests, certify isolation, or replace the distinct Closure Agent decision.
