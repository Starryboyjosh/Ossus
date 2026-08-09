# WAVE-019 — GitHub discovery connector

## Status

Planned. No repository evidence shows implementation or completion.

## Technical summary

Add GitHub candidate discovery with minimum-scope credentials, category and ecosystem searches, rate-limit caching, metadata evidence, immutable commit resolution, and fork/copy deduplication. Discovery returns references only and feeds passive intake.

## Practical plain-language summary

Find potentially useful GitHub projects while keeping access narrow and never treating popularity or search results as approval.

## Expected evidence/deliverables

- GitHub discovery command, cache records, evidence records, and rate-limit behavior.
- Tests for pagination, rate limits, deleted/private repositories, duplicates, branch mutation, and malformed responses.
- Credential-scope and redaction review by Agent Review Authority.

## Dependencies/gates

Depends on WAVE-018. Its output is an input to WAVE-021; it does not itself close a named gate.

## Remaining work

Design the access model, implement bounded discovery and caching, ensure all intake is commit-pinned, test adverse API conditions, and obtain the required review.
