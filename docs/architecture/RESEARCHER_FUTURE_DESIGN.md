# Researcher future design

## Status

Not part of V0.

Researcher development begins only after the Resolver vertical slice passes its gates.

## Responsibility

The Researcher discovers external resources and produces evidence for Curator Agent curation.

It has no authority over trusted Registry state.

## Subsystems

```text
source connectors
  → discovery queue
  → quarantine fetcher
  → inventory
  → external scan adapters
  → reputation evidence
  → provisional classifier
  → review bundle
```

## Source connectors

Initial planned connectors:

1. GitHub repositories and topics.
2. Manual URL or local path.
3. Reddit community references after legal and privacy review.
4. Existing skill or plugin directories.

Connectors produce source references, not approved records.

## Quarantine rules

- separate root outside product repository;
- no secrets;
- no automatic execution;
- submodules disabled initially;
- hooks disabled;
- bounded clone depth and size;
- safe filename handling;
- symlink inventory without following outside root;
- immutable source commit recorded;
- cleanup policy;
- complete provenance log.

## Analysis strategy

Ossus integrates rather than recreates Semgrep, ecosystem dependency scanners, license detection tools, archive/binary inventory and secret scanners where legally and operationally appropriate.

Model analysis receives delimited content as hostile data, has no tools or credentials, emits schema-constrained findings and cannot approve or write trusted files.

## Reputation

Reputation is separate from utility and security.

Evidence may include maintenance activity, releases, independent mentions, issue response, contributor diversity, suspicious popularity patterns, known advisories and provenance consistency.

No popularity threshold grants approval.

## Review bundle

```text
source.json
inventory.json
hashes.json
license-report.json
static-findings.json
dependency-findings.json
community-evidence.json
provisional-mapping.json
review-checklist.md
```

It excludes executable CI from trusted workflows.

## Agent-authorized admission

A Curator Agent prepares the canonical manifest from evidence. An independent Admission Review Agent checks the proposed contribution, and a distinct Closure Agent makes the final admission, rejection, or blocked decision. Human review may be recorded as additional evidence but is not required. The Researcher remains evidence-only and cannot fill any of these roles for a candidate it discovered.

The admitted change is made in the Registry repository independently from the candidate repository.

## Absolute prohibitions

Researcher must never use `pull_request_target` on candidate content, run candidate CI with write tokens, copy candidate workflows into trusted CI, accept mutable branch names as source locks, follow external symlinks, expose user secrets, activate a candidate or auto-merge a Registry contribution.
