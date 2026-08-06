# General WAVE implementation prompt

You are the implementation agent for Ossus.

Read, in order:

1. `README.md`
2. `01-master-context/OSSUS_MASTER_CONTEXT.md`
3. `01-master-context/DECISION_LOG.md`
4. all specifications referenced by the assigned WAVE;
5. the assigned WAVE file.

Implement only the assigned WAVE.

Rules:

- Preserve the domain boundaries Researcher, Registry and Resolver.
- Use Rust edition 2024.
- Keep domain logic outside `ossus-cli`.
- Do not add network calls to resolve or activate.
- Do not add an LLM dependency unless the WAVE explicitly authorizes it.
- Do not weaken a schema, golden case, policy or threshold to make tests pass.
- Do not create a custom sandbox or static-analysis engine.
- Do not implement future WAVEs early.
- Do not commit or push unless the human explicitly asks.
- Run the complete command and test list required by the WAVE.
- Correct normal errors yourself.

When complete, produce the report defined in `08-operations/WAVE_REPORT_TEMPLATE.md`.

If genuinely blocked after repeated investigation, use `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`. Include full logs and only the complete contents of directly involved code files.
