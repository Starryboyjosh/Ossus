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
verification are recorded. The official catalog is currently **2** against a
provisional 16 admission-bearing slots; profiles 10, 17, 18, and 20 are
intentionally unresolved. Profiles 6 and 9 are standard-only admissions with
no Claude/Codex claim; profile 10 needs an enforced read-only adapter; profile
15 has an accepted surface correction but its candidate is blocked after an
independent review found no bounded adapter, freshness protocol or redaction
evidence; profile 16 remains valid but unfilled; and the original profile-20
MCP candidate is rejected while a replacement is conditional. Profiles 5, 7,
11 and 12 have explicit amendment packets, not approvals. These are not
approvals.

The pinned hosted release FTS5 jobs for Ubuntu, macOS and Windows passed on the
latest pre-Arch checkpoint. The inventory generator now hashes canonical Git
index/blob bytes, fixing the checkout-EOL discrepancy; CI run 18 is fully green
for the then-declared jobs. A separate Arch Linux validation job now runs the
workspace and release FTS5 tests inside a pinned Arch userspace container on an
Ubuntu-hosted runner. Local reproduction passes; its first hosted result is
pending. Arch container validation is not native Arch-host validation. This
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

The final-admission sprint used bounded reviews rather than a quota-driven
discovery sweep. Catalog growth is an outcome of successful review, and
discovery volume must never create admission pressure. The amendment packets
and temporary governance lessons are recorded in the WAVE-003 operations
documents.

The Closure Agent has the last word on an evidence-backed WAVE decision. The
implementer, independent reviewer, and Closure Agent remain separate roles;
human review may add evidence but is not a required approval step under ADR-020.
