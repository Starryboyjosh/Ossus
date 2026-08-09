# Ossus technical implementation workflow

This document is the cumulative technical map. It summarizes history and the
current boundary; WAVE reports, ADRs, schemas, and tests remain authoritative
evidence.

## Stable architecture and trust boundaries

`ossus-cli` orchestrates libraries and owns presentation/exit codes. Domain
logic remains in `ossus-core`, `ossus-registry`, `ossus-policy`,
`ossus-resolver`, `ossus-adapter-claude`, and `ossus-eval` according to
`RUST_WORKSPACE.md`; placeholder crates do not imply their later-WAVE behavior
exists.

```text
external source -> research evidence -> canonical manifest -> SQLite index
                       untrusted          reviewed facts       derived/cache
```

The Registry is an instruction-channel trust boundary. Origin metadata is
evidence, never canonical authority. Stored, approved, installed, and active
are separate states. Normal Registry search is local, deterministic, and reads
metadata rather than resource bodies.

## WAVE-000

### What changed

An adversarial plan review produced dispositioned findings, ADR-016 through
ADR-018, corrected WAVE ownership, and explicit security work for later phases.

### Security decisions and result

Gate S0 closed after architecture and security responsibilities were assigned.
The historical closure remains evidence; ADR-020 governs new agent authority,
independent review, and closure decisions.

## WAVE-001

### What changed

The Rust 2024 workspace established seven initial crates, centralized locked
dependencies, forbidden first-party `unsafe`, pinned and advisory CI jobs,
format/Clippy/test requirements, repository-layout generation, and CLI snapshot
tests. Placeholder commands return stable not-implemented exit code 69.

### Result

The build and verification substrate exists without crossing into Registry,
Resolver, activation, or Researcher implementation.

## WAVE-002

### What changed

`ossus-core` gained typed identifiers and enums. `ossus-registry` gained bounded
TOML loading plus strict schema and semantic validation. Trusted taxonomy and
evaluation inputs use TOML/JSON, not YAML. Validation rejects unknown fields,
aliases as canonical IDs, invalid risk/runtime combinations, non-canonical
source locks, and collection/size budget violations.

### Contracts and security

The canonical manifest schema, capability taxonomy V1, stable diagnostic
classes/reason codes, and executable valid/invalid fixture corpus form the
trusted input contract. Gate S1 closed and authorized WAVE-003.

## WAVE-003

### Registry implementation

`ossus-registry` owns the versioned SQLite schema, migrations, atomic rebuild,
conflict detection, catalog fingerprint, health status, exact lookup, filters,
and FTS5 queries. Rebuild order and serialized metadata are deterministic;
invalid manifests are excluded with diagnostics. The bundled `rusqlite` SQLite
build is used so FTS5 does not depend on the host system library.

`ossus-cli` exposes:

```text
validate
registry status
registry reindex
search
show
```

Human and JSON contracts, usage errors, unknown IDs, empty/corrupt indexes,
invalid-manifest exclusion, conflicts, combined filters, and deterministic
reindex are covered by unit, integration, and CLI tests. Registry-unavailable
conditions use exit code 20; validation keeps schema/taxonomy codes 11/12.

### Catalog and admission

The official catalog accepts only `skill`, `prompt-pack`, and `mcp-server`
manifests. Every seed requires a fixed upstream commit and subpath, reproducible
content hash, license, reviewed capabilities/compatibility/runtime/risk/context,
and separated Curator Agent, Admission Review Agent, and Closure Agent evidence.
Research output cannot approve itself. The current admission ledger has **two
official entries** (profiles 6 and 9): both are standard-only profile
substitutions. Profile 10 needs an enforced read-only adapter; profile 15 has
an accepted surface correction but its independent admission review is blocked
on a dependency-only adapter, freshness protocol and redaction evidence;
profile 16 retains a valid but unfilled profile, profiles 17/18 are
intentionally unresolved, and profile 20's original calculator is rejected
while its replacement is conditional. Profiles 5, 7, 11 and 12 have
Curator-only R3 amendment packets; no amendment or candidate is approved by
those packets.

#### Catalog-pressure invariant

Seed profiles are coverage objectives, not an acceptance quota. Catalog growth
is an outcome of successful Curator → independent Admission Review → Closure
review; it is never a goal that overrides review. Discovery volume must never
create admission pressure. Profile correctness and candidate correctness are
recorded separately, and a profile substitution does not admit a resource.
WAVE-003 closure therefore requires governed final dispositions for every seed
profile, policy-compliant canonical manifests for every admitted resource, and
minimum useful catalog coverage—not an unconditional `official_resources ==
20` test. The reconciliation currently records 16 provisional
admission-bearing slots and four intentionally unresolved profiles; the official
Registry contains two resources and remains empty for every candidate whose
authority chain is incomplete. The two admitted manifests are explicitly excluded from aggregate cross-host
coverage until host adapters exist.

### Current gates and limitations

WAVE-003 does not close a security gate. Local Linux x86_64 release-mode FTS5 is
verified, and pinned hosted Ubuntu, macOS and Windows release FTS5 jobs have
passed. The same hosted run has an unexplained repository-layout job failure;
the exact commit passes that check in a fresh clone, so a follow-up run is
required before hosted CI is considered fully green. The 20-resource catalog
and final admission report are still in progress. One Sol Medium advisory was used for the hardest
profile-15/profile-16 reconciliation; it did not approve anything. No scanning,
Resolver, policy scoring, installation,
activation, host adapter, synchronization, or automated discovery is
implemented.

### Next dependency

WAVE-004, the bounded project scanner, depends on completed WAVE-003. WAVE-005
depends on both and is the first Resolver-core WAVE. Neither may be implemented
as part of WAVE-003.

## Governing references

- `docs/AGENT_AUTHORITY.md` and ADR-020 in the decision log.
- `docs/architecture/RUST_WORKSPACE.md` and `CLI_CONTRACT.md`.
- `specs/schemas/canonical-manifest.schema.json`.
- `specs/taxonomy/capabilities-v1.toml` and `GOVERNANCE.md`.
- `docs/security/THREAT_MODEL.md`, `SUPPLY_CHAIN.md`, and `SECURITY_GATES.md`.
- WAVE reports and reader summaries under
  `docs/implementation/08-operations/`.
