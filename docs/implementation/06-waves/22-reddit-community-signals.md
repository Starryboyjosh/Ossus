# WAVE-022 — Reddit community evidence

**Phase:** Phase 5  
**Assigned role:** Implementation agent with privacy/security review  
**Depends on:** WAVE-021  
**Security WAVE:** no

## Objective

Use Reddit as bounded community evidence, never authority or a content mirror.

## In scope

- Reverify current official API and terms.
- Implement OAuth/rate-limit behavior.
- Store IDs, links, times, aggregates and minimal necessary excerpts.
- Detect independent mentions and obvious promotion.
- Implement retention/deletion policy.

## Out of scope

- Training on Reddit data.
- Copying complete threads.
- Sentiment-based approval.
- Unapproved HTML scraping.

## Expected deliverables

- Reddit evidence connector.
- Privacy/retention docs.
- Community evidence schema.

## Required tests and evidence

- Rate limits.
- Deleted posts.
- Self-promotion.
- Cross-post duplicates.
- API unavailable.
- Retention cleanup.

## Acceptance criteria

- Evidence cannot automatically change approval or risk.
- Stored content is minimized.
- Terms and API behavior are reverified at implementation time.


## Review workflow

Use an implementation agent and a reviewer. Require Opus 5 security review whenever the work changes a trust boundary, network source, host path, permission, update mechanism or CI configuration.


## Copy-ready implementation instruction

Use the general implementer prompt, browse current Reddit policy, and obtain Opus 5 privacy/security review.

Read `README.md`, the master context, decision log, referenced specifications and this WAVE. Implement only this WAVE. Produce the standard WAVE report. Do not commit or push unless the human explicitly requests it.

## Escalation

Correct normal failures yourself. Only after repeated attempts or a serious technical block, create the diagnostic defined in `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.
