# Ossus project workflow

This is the non-technical, cumulative guide to what the repository can do. It
is updated at the end of every WAVE; implementation reports remain the detailed
evidence.

## The three domains

```text
Researcher                  Registry                    Resolver
finds possibilities   ->   stores reviewed facts   ->  chooses for a task
(future automation)        (current work)               (future work)
```

The Researcher produces candidates and evidence, never approvals. The Registry
contains canonical descriptions accepted through separated agent review. The
Resolver will later consider only approved Registry metadata, apply policy
before scoring, and explain its selection. Approval still does not mean a
resource is installed, and installation still does not mean it is active.

## WAVE history

### WAVE-000 — review the plan

Ossus first challenged its architecture and security assumptions. Findings were
assigned to concrete decisions and later WAVEs, and Security Gate S0 was closed.

### WAVE-001 — create the foundation

The project gained a Rust 2024 workspace, a command-line shell, reproducible
tooling, CI checks, layout checks, and tests. Commands belonging to future
features remained explicit placeholders.

### WAVE-002 — define trusted descriptions

Ossus gained strict canonical manifests, a governed capability vocabulary,
bounded parsing, stable diagnostics, and negative test cases. Invalid or
untrusted fields are rejected before they can become Registry facts. Security
Gate S1 was closed.

### WAVE-003 — build local Registry search

WAVE-003 is the current WAVE. The implemented mechanics build a disposable
SQLite/FTS5 index, exclude invalid manifests, detect conflicts, and search
trusted metadata by text, capability, category, host surface, source, runtime,
and risk. The CLI exposes `search`, `show`, `registry status`, and `registry
reindex` in human and JSON forms.

The WAVE is not complete until the reconciled seed profiles have governed
dispositions, admitted resources have immutable provenance/license/hash and
independent Closure evidence, and cross-platform release-mode FTS5 and final
verification are recorded. The final coverage authority recommends **16
current admission-bearing responsibilities**: 9 unchanged required profiles
and 7 required profiles with explicit substitutions. Profiles 10, 17, 18 and
20 are intentionally unresolved future coverage, not a quota reduction. The
official catalog currently contains **2** resources, both R0
Agent-Skills-standard prompt-packs. They provide no admitted skill, MCP,
Claude, Codex, standalone-CLI, generic-MCP, cross-host, R1/R2/R3, or
overlapping competitor coverage. Profiles 6 and 9 are standard-only
admissions; profile 15's surface correction is accepted but its candidate is
useful and deferred pending an enforced adapter, freshness and redaction; and
profiles 5, 7, 11 and 12 have independently reviewed amendment
recommendations (R3, R2, R2 and R3 respectively), not candidate approvals.
These are not approvals.

The pinned hosted release FTS5 jobs for Ubuntu, macOS and Windows, plus the
separate Arch Linux userspace validation job, passed in the documented green CI
run 22. The
inventory generator now hashes canonical Git index/blob bytes, fixing the
checkout-EOL discrepancy. The Arch job runs the workspace and release FTS5
tests inside a pinned Arch userspace container on an Ubuntu-hosted runner;
Arch container validation is not native Arch-host validation. This
platform-evidence work does not close seed admission.

The catalog has no quota. Catalog growth is an outcome of successful review,
never a goal that overrides review, and discovery volume must never create
admission pressure. A profile substitution changes a coverage requirement; it
does not admit a resource.

## What works now

- Validate canonical TOML manifests and taxonomy references.
- Rebuild a local metadata-only Registry index deterministically.
- Inspect index health and FTS5 availability.
- Search and show indexed canonical metadata.
- Produce concise human output or versioned JSON.

## What remains

WAVE-003 must finish seed admission and release evidence. WAVE-004 is the next
authorized WAVE only after WAVE-003 closes; it adds bounded project scanning.
Resolution, installation, activation, host adapters, remote synchronization,
and automated Researcher discovery belong to still later WAVEs.

The final-admission and coverage pass used bounded reviews rather than a
quota-driven discovery sweep. Catalog growth is an outcome of successful
review, and discovery volume must never create admission pressure. Synthetic
fixtures may exercise Resolver negative/risk/overlap behavior, but cannot
replace real Registry provenance or exact-resource seed evidence. The final
coverage authority, amendment review, and temporary governance lessons are
recorded in the WAVE-003 operations documents.

The Closure Agent has the last word on an evidence-backed WAVE decision. The
implementer, independent reviewer, and Closure Agent remain separate roles;
human review may add evidence but is not a required approval step under ADR-020.
The 16 figure is a provisional planning denominator accepted for coverage
analysis only; the active WAVE-003 completion obligation remains the original
20 real seed entries until architecture records an explicit change.
