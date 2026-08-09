# Agent Review Authority

**Effective:** 2026-08-07
**Authority:** ADR-020

Within the Ossus WAVE workflow, the Agent Review Authority has the final word on
technical acceptance, Registry admission, risk disposition, security-gate
closure, and release readiness. Human review may be requested or supplied as
additional evidence, but it is not a required approval step.

This rule changes who decides; it does not weaken what must be proved. Missing,
contradictory, or unverifiable evidence fails closed.

## Required separation of roles

Every WAVE or Registry admission records three roles:

1. **Implementer Agent** — prepares the change and its evidence.
2. **Independent Review Agent** — reviews the final change and evidence. For a
   security WAVE or security-sensitive admission, this is a Security Review
   Agent.
3. **Closure Agent** — makes the final accept, reject, or blocked decision after
   verifying the evidence and reviewer verdict.

The Closure Agent must not have implemented or independently reviewed the same
change. An Implementer Agent may correct findings, but every correction returns
to independent review before closure.

For a Registry entry, the equivalent roles are Curator Agent, Admission Review
Agent, and Closure Agent. The Researcher remains evidence-only and can never
hold any of those admission roles for a candidate it discovered.

## Decision rules

- Critical and high findings cannot be accepted as residual risk. They require
  correction and independent re-review.
- Medium findings require a correction or a bounded accepted-risk record with
  rationale, owner, expiry or revisit trigger, and compensating controls.
- Low and informational findings may enter a tracked backlog.
- No agent may infer approval from storage, installation, popularity, origin
  metadata, or a previous agent's unsupported assertion.
- Deterministic tests and primary-source evidence take precedence over model
  opinion. Conflicts stay visible until resolved.
- A lack of credentials, network access, signing keys, or a remote runner is an
  execution-capability block, not a request for human approval.
- Publication, deployment, messaging, commits, and pushes still require the
  explicit operational authorization applicable to that external action. That
  authorization does not replace or reopen the Closure Agent's technical
  decision.

## Required closure record

The closure record contains:

- WAVE or Registry-entry identifier;
- Implementer, reviewer, and Closure Agent identifiers;
- model/configuration and run identifiers where available;
- base and reviewed revision identifiers;
- evidence and input hashes;
- commands, results, and platform coverage;
- findings and their dispositions;
- conflict-of-interest attestations;
- residual risks and revisit triggers;
- the final `accepted`, `rejected`, or `blocked` decision.

## Per-WAVE reader summary

Every WAVE, including completed and planned WAVEs, has a reader summary under
`docs/implementation/08-operations/wave-summaries/`. Each summary contains at
least:

- current status;
- a technical summary;
- a practical plain-language summary;
- delivered or expected evidence;
- dependencies and gates;
- remaining work.

A completed WAVE cannot close until its summary reflects the final evidence.

## Legacy review-tier wire values

Canonical manifest schema version 1 uses these serialized review-tier values:

| Legacy wire value | Current meaning |
|---|---|
| `light-human` | focused agent review |
| `full-human` | complete agent review |
| `security-human` | security-agent review |

The strings remain unchanged in schema version 1 to preserve compatibility with
WAVE-002 fixtures, manifests, and lock contracts. They describe review depth,
not the species or identity of the reviewer. `reviewer_ids` records agent or
review-run identifiers under ADR-020. A future schema-major migration may rename
the wire values without changing the risk floors.

## Historical records

Dated WAVE-000 through WAVE-002 reports truthfully record the governance in
force when those decisions were made. They remain historical evidence. ADR-020
supersedes their human-final or named-model requirements for decisions made on
or after 2026-08-07; it does not rewrite past signatures or events.
