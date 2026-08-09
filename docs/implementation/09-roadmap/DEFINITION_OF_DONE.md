# Definition of done

## WAVE completion

A WAVE is complete only when:

- implementation stays inside the assigned scope;
- each acceptance criterion maps to evidence;
- required tests and complete workspace checks pass;
- schemas and documentation match runtime behavior;
- no unresolved critical or high review finding remains;
- medium findings have an owner, correction WAVE or Closure Agent accepted-risk record with rationale, expiry or revisit trigger, and compensating controls;
- a standard WAVE report exists;
- the working tree and base commit are recorded;
- a handoff identifies the next prerequisites;
- the WAVE reader summary includes both a technical summary and a practical plain-language summary, plus status, evidence, dependencies/gates, and remaining work;
- the distinct Closure Agent has closed the relevant gate when security-sensitive.

## Security WAVE completion

Additionally:

- the configured Security Review Agent performed the assigned security work or independently reviewed the final implementation and evidence;
- Implementer Agent contributions are explicitly attributed;
- every finding has an explicit disposition rather than an averaged severity;
- attack and negative-path tests pass;
- threat model and trust boundaries are updated;
- an independent Closure Agent recorded the final decision, evidence hashes, reviewer verdict, and conflict-of-interest attestation.

## Phase completion

A phase is complete only when its gate passes. A feature is not complete because a command worked once on one machine.

## Release completion

A release is complete only after clean-install, checksum, upgrade, rollback, golden, activation and security checks pass on the declared target matrix.
