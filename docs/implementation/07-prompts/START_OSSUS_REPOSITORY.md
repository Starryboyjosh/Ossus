# Start Ossus — repository creation and WAVE 001

You are the first implementation agent for Ossus.

The Git repository does not exist yet. Create it only after reading the planning package.

## Required reading

1. `README.md`
2. `01-master-context/OSSUS_MASTER_CONTEXT.md`
3. `01-master-context/DECISION_LOG.md`
4. `01-master-context/PROJECT_CHARTER.md`
5. `02-architecture/RUST_WORKSPACE.md`
6. `02-architecture/CLI_CONTRACT.md`
7. `04-security/THREAT_MODEL.md`
8. `06-waves/01-rust-workspace-bootstrap.md`
9. `07-prompts/GENERAL_IMPLEMENTER.md`

The Spanish documents under `00-original-context/` are historical design records. Read them only when a decision appears ambiguous or a restructuring is being considered. Do not use them as the active implementation contract.

## Assignment

Create a new Git repository named `ossus` and implement only WAVE 001.

The product, source code, comments, user-facing CLI text, documentation, reports and commit messages must be in English.

Use Rust edition 2024 and a Cargo workspace. The binary must be named `ossus`. The internal domains remain:

- Researcher
- Registry
- Resolver

Do not implement Registry indexing, resolution, activation, source discovery or network behavior in this WAVE.

Do not commit or push unless the human explicitly requests it.

Run every command required by WAVE 001. Correct normal failures. When complete, produce the report from `08-operations/WAVE_REPORT_TEMPLATE.md`.

If genuinely blocked after repeated attempts, produce `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md` with complete logs and only the full contents of directly involved code files.
