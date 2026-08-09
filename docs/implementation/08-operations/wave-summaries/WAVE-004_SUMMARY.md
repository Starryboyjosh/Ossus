# WAVE-004 — Bounded project scanner

## Status

**Planned.** No completion report or current-WAVE evidence indicates implementation.

## Technical summary

WAVE-004 will produce deterministic project profiles from bounded metadata signals: languages, package managers, frameworks, databases, tests, CI, and host signals. It includes ignore handling, traversal budgets, stable JSON and profile hashes.

## Practical plain-language summary

Ossus will learn the shape of a project without broadly reading its source code or following unsafe paths.

## Expected evidence

- `ossus scan`, detector registry, profile schema, fixtures, and visible truncation explanations.
- Tests for common stacks, monorepos, symlink loops, permission errors, huge trees, secret-like filename exclusion, and deterministic hashes.

## Dependencies and gates

Depends on complete WAVE-002. It does not close a gate.

## Remaining work

Implement the bounded scanner and all required test coverage. Task interpretation, selection, network behavior, and secret scanning remain outside this WAVE.
