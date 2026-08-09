# WAVE-022 — Reddit community evidence

## Status

Planned. No repository evidence shows implementation or completion.

## Technical summary

Add a bounded Reddit evidence connector after rechecking official API terms. Use OAuth and rate limits, retain only identifiers, links, timestamps, aggregates, and necessary excerpts, detect independent mentions and promotion, and apply retention/deletion rules. Community evidence cannot alter approval or risk automatically.

## Practical plain-language summary

Use limited community signals as context, while avoiding thread copying, scraping, and popularity-based decisions.

## Expected evidence/deliverables

- Reddit evidence connector, privacy/retention documentation, and community-evidence schema.
- Tests for rate limiting, deletion, self-promotion, cross-post duplicates, API outages, and retention cleanup.
- Current-policy verification and applicable Agent Review Authority review.

## Dependencies/gates

Depends on WAVE-021 and is required before the WAVE-023 end-to-end audit. It does not itself close a named gate.

## Remaining work

Verify current terms and API behavior, implement data minimization and retention controls, test failure and privacy cases, and obtain the appropriate review.
