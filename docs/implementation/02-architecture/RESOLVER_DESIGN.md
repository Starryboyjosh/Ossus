# Resolver design

## Goal

Select the smallest compatible, policy-allowed resource set that covers the capabilities required by the project and task.

## Inputs

- normalized task;
- project profile;
- target surface;
- local policy;
- Registry snapshot;
- taxonomy and aliases;
- optional user inclusions and exclusions.

## Pipeline

### Stage 1 — Project detection

Detect deterministic signals from bounded files and metadata: languages, package managers, frameworks, databases, test tools, infrastructure, CI, host surfaces and repository shape.

A framework is a project signal, not a capability.

### Stage 2 — Task normalization

- Unicode normalization;
- lowercasing for matching while retaining original;
- tokenization;
- bounded phrase extraction;
- alias expansion;
- explicit negation detection;
- user-specified resource mentions;
- command intent.

Do not execute shell interpolation or parse task text as configuration.

### Stage 3 — Capability candidate generation

Evidence sources, in descending authority:

1. explicit capability or resource requested by the user;
2. exact governed phrase;
3. alias;
4. project-plus-task rule;
5. trusted FTS match;
6. optional local semantic fallback in a later phase.

Each capability receives evidence and confidence, not only a score.

### Stage 4 — Candidate retrieval

Apply hard filters before scoring:

- resource is approved and not revoked;
- schema supported;
- source permitted;
- surface compatible;
- runtime available;
- risk within policy;
- activation mode allowed;
- source lock immutable;
- adapter exists.

### Stage 5 — Deterministic scoring

Suggested normalized components:

```text
capability coverage               0.35
task evidence                     0.20
project compatibility             0.15
trust and review strength         0.10
surface fit                       0.10
local availability                0.05
maintained source evidence        0.05
```

Penalties: redundant overlap, context cost, extra runtime requirements, additional permissions, host-exclusive behavior and manual-confirmation requirement.

Policy denial is not a penalty. It is exclusion.

### Stage 6 — Minimal coverage

Use deterministic greedy weighted set cover for V0:

1. start with uncovered required capabilities;
2. choose the allowed candidate with greatest weighted uncovered value per cost;
3. apply stable tie-breakers;
4. repeat until covered or impossible;
5. run a redundancy-removal pass;
6. validate hard constraints.

Tie-break order:

1. more required capability coverage;
2. lower risk;
3. fewer runtime requirements;
4. lower measured context;
5. stronger review;
6. lexicographically smaller resource ID.

The algorithm is documented and versioned.

### Stage 7 — Confidence

Low confidence occurs when the task is overly broad, required capabilities conflict, multiple very different sets are near-equal, no candidate covers a required capability, project signals are insufficient or user intent implies high-risk activation without explicit request.

Low confidence returns a plan with no activation and a concise refinement request or manual selection options.

### Stage 8 — Explanation

For every selected resource explain covered capabilities, matching evidence, project compatibility, policy decision, risk, context estimate and why alternatives lost.

For important exclusions explain policy denial, incompatibility, redundancy, missing runtime, excessive risk or lower coverage.

## Activation limit

Default maximum active resources: `5`.

Hard default ceiling without explicit override: `8`.

A policy can lower the limit.

## Determinism test

With identical Ossus version, taxonomy, Registry snapshot, policy, project profile, task and target surface, the Resolver must produce byte-equivalent normalized selection JSON except for explicitly excluded timestamps.
