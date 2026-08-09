# Agent Closure checklist (legacy filename)

This legacy filename remains a redirect for existing WAVE references. Under
ADR-020, a separate Closure Agent closes the security WAVE; human evidence is
optional and is not a required approval step.

The Closure Agent records its identity, model/configuration, run identifier,
and conflict-of-interest attestation, then confirms:

- [ ] Implementer Agent, Independent Security Review Agent, and Closure Agent are recorded and are separate roles.
- [ ] Independent security implementation or review report exists from the configured Security Review Agent.
- [ ] Every implementation-support contribution is attributed and independently reviewed.
- [ ] All critical findings are resolved.
- [ ] All high findings are resolved.
- [ ] Medium findings have a correction or bounded accepted-risk record with rationale, owner, expiry/revisit trigger, and compensating controls.
- [ ] Required attack tests pass.
- [ ] Threat model and trust boundaries are updated.
- [ ] No security threshold was weakened without ADR.
- [ ] No secrets appear in reports or fixtures.
- [ ] Residual risks are understandable.
- [ ] The next WAVE does not depend on an unresolved control.
- [ ] Technical and practical WAVE summaries reflect final evidence.
- [ ] Closure decision is recorded as `accepted`, `rejected`, or `blocked`.
