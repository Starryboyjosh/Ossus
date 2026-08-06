# Security gates

## Gate S0 — Plan accepted

Required before WAVE 01:

- Opus 5 security and architecture plan review;
- explicit disposition of every finding;
- human resolution;
- no unresolved critical finding;
- implementation assumptions updated.

## Gate S1 — Trusted contracts

Required before Registry indexing:

- schemas reject unknown major versions;
- parser budgets tested;
- canonical/origin separation explicit;
- taxonomy and policy hashes supported.

Owner: **WAVE-002**. Closed by a named human against `docs/implementation/07-prompts/HUMAN_SECURITY_CLOSURE.md` after Opus 5 review of the WAVE-002 diff and evidence.

## Gate S2 — Resolver policy

Required before activation work:

- policy denial precedes scoring;
- risk and surface tests pass;
- low-confidence behavior fails closed;
- R4 implicit activation test equals zero;
- policy monotonicity holds: a project-scoped configuration or registry file cannot relax effective policy (ADR-016), proven by the adversarial fixture in the golden suite.

Owner: **WAVE-005**. Closed by a named human against `docs/implementation/07-prompts/HUMAN_SECURITY_CLOSURE.md` after Opus 5 review of the WAVE-005 diff and evidence.

## Gate S3 — Activation boundary

Required before Claude adapter:

- path traversal tests;
- symlink escape tests;
- hash verification;
- transactional rollback;
- ownership record;
- managed/unmanaged deletion tests.

## Gate S4 — Supply chain

Required before V0 release:

- dependency audit;
- CI token review;
- release permission review;
- checksum generation;
- workflow pinning policy;
- no high unresolved findings.

## Gate S5 — Researcher intake

Required before any Internet source connector:

- separate quarantine;
- no candidate execution;
- size/time limits;
- no secrets;
- immutable source lock;
- safe archive and symlink behavior.

## Gate S6 — Admission workflow

Required before Researcher beta:

- evidence/canonical boundary;
- two-person controls by risk;
- no direct write to main Registry;
- staging CI restrictions;
- complete audit log.

## Gate S7 — Public beta

Required before WAVE 24:

- full Opus 5 security audit;
- attributed implementation and test evidence;
- human residual-risk acceptance;
- incident and revocation procedure;
- release artifact verification.
