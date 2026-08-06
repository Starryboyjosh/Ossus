# Evaluation strategy

## Purpose

Resolver scoring must be adjusted against frozen expected behavior, not intuition.

## Layers

### Layer 1 — Capability goldens

The 50 cases in `goldens-v1.toml` define expected and forbidden capabilities. Per ADR-017 this file is TOML; the YAML original is retired by WAVE-002 after a verified equivalence check. Freezing the case content is what matters, not the serialization it was frozen in.

Freeze this layer before implementing scoring.

### Layer 2 — Exact resource goldens

After selecting the 20 seed entries, add required resource IDs, allowed alternatives, forbidden IDs, maximum selected count and expected policy decision.

Scoring work cannot be declared complete until Layer 2 exists.

### Layer 3 — Synthetic scale

Generate 1,000 and 10,000 valid synthetic manifests to test index time, query time, Resolver p95, memory, description truncation handling, trigger overlap and deterministic tie-breaking.

### Layer 4 — Adversarial cases

Include trigger stuffing, claimed R0 with shell files, namespace collision, same version with different hash, symlink escape, path traversal, unsupported schema, ambiguous broad task, R4 implicit activation and malicious origin metadata.

**Policy monotonicity (F-01, ADR-016).** A hostile-repository case is mandatory here and is the evidence Gate S2 closes against. The fixture project ships `.ossus/config.toml` and `.ossus/policy.toml` attempting to raise `risk_max`, set `allow_implicit_r4 = true`, empty `block`, set `require_hash_verification = false`, raise a parser budget and register a priority-300 registry source shadowing an `official.*` id. The case asserts that effective policy is byte-identical to the user/global policy, that any selection requiring the relaxation is denied with the documented policy exit code, that every attempted relaxation is reported in `explain` and in the audit event, and that the shadowing entry is refused or requires recorded trust and is attributed.

This case may never be weakened to accommodate a configuration-precedence implementation. If it fails, the implementation is wrong.

## Dataset governance

- cases are reviewed before use;
- changes require rationale;
- threshold reductions require an RFC;
- implementation bugs are not fixed by weakening expectations;
- case IDs remain stable;
- fixtures contain no real secrets.

## Reports

`ossus eval --format json` produces suite version, binary version, Registry snapshot, per-case result, aggregate metrics, constraint violations, performance, context savings and failures grouped by capability.
