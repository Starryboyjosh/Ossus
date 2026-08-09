# WAVE-009 Summary — Complete CLI vertical slice

## Status

Planned.

## Technical summary

Completes the offline CLI path: initialization, configuration, Registry search, project scan, deterministic resolve and explain, verified activation/deactivation, lock verification, and doctor. Outputs must be versioned JSON with stable exit codes; shell completions are optional and low-risk.

## Practical plain-language summary

This is the point where a new user should be able to go from setting up Ossus to safely activating selected resources without editing internal files.

## Expected evidence/deliverables

- V0 CLI vertical slice, reference documentation, end-to-end fixture, and example project.
- Clean and offline workflow tests, low-confidence and policy-denial cases, activation rollback, lock verification, and JSON-contract tests.
- Documented, tested exit codes and zero default model calls.

## Dependencies/gates

Depends on WAVE-008. Resolve and activate remain separate consent boundaries. Changes affecting trust boundaries, host paths, permissions, network behavior, updates, or CI require Agent Review Authority review.

## Remaining work

Implement the scoped commands and documentation, add the fixture and contract tests, and collect the required acceptance evidence.
