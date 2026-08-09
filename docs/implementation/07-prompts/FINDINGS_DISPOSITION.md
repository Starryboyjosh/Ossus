# Disposition security findings

Process the Independent Security Review Agent report claim by claim. Record the
reviewer's selected model/configuration and run identifier.

Create a table with finding ID, reviewer position, supporting evidence, conflicting implementation or test evidence, affected invariant, deterministic verification, Closure Agent decision, resulting action and residual risk.

Rules:

- do not average severity with another agent's opinion;
- a critical or high concern remains blocking until evidence resolves it;
- uncertainty must remain visible;
- critical and high findings cannot be accepted as residual risk and require correction plus independent re-review;
- medium accepted risk needs bounded rationale, owner, expiry or revisit trigger, and compensating controls;
- implementation-support contributions must be attributed and independently verified;
- corrections become a separate scoped WAVE or patch plan.

The Closure Agent must be separate from both Implementer Agent and Independent
Review Agent. Human evidence is optional and does not replace the final agent decision.
