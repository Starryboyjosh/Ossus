# Canonical manifest fixture corpus

These files are test inputs for bounded canonical-manifest parsing, schema validation, and semantic validation.

Many files under `invalid/` are deliberately malformed or policy-invalid. They must never be indexed, installed, activated, published, or otherwise treated as Registry entries. Files under `valid/` are acceptance controls that prove the validator does not over-reject.

`INDEX.toml` is the machine-readable inventory. Parser-budget seeds require the in-memory amplification described in their index notes. Invalid UTF-8 is intentionally generated as raw bytes by the consuming test instead of being stored as a text fixture.

