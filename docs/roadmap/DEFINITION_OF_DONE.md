# Definition of done

## WAVE completion

A WAVE is complete only when:

- implementation stays inside the approved scope;
- each acceptance criterion maps to evidence;
- required tests and complete workspace checks pass;
- schemas and documentation match runtime behavior;
- no unresolved critical or high review finding remains;
- medium findings have an owner, correction WAVE or approved residual-risk record;
- a standard WAVE report exists;
- the working tree and base commit are recorded;
- a handoff identifies the next prerequisites;
- the relevant human gate is closed when security-sensitive.

## Security WAVE completion

Additionally:

- Opus 5 performed the assigned security work or reviewed the final implementation and evidence;
- contributions from Luna Max or another implementation agent are explicitly attributed;
- every finding has an explicit disposition rather than an averaged severity;
- attack and negative-path tests pass;
- threat model and trust boundaries are updated;
- a human signed the closure checklist.

## Phase completion

A phase is complete only when its gate passes. A feature is not complete because a command worked once on one machine.

## Release completion

A release is complete only after clean-install, checksum, upgrade, rollback, golden, activation and security checks pass on the declared target matrix.
