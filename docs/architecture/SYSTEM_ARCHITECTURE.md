# Ossus system architecture

## Context

Ossus runs primarily as a local command-line application.

```text
User / automation
       |
       v
    ossus CLI
       |
       +-------------------+
       |                   |
       v                   v
   Registry             Project scanner
       |                   |
       +---------+---------+
                 |
                 v
              Resolver
                 |
        policy + integrity
                 |
                 v
              Adapter
                 |
                 v
      Claude Code / Codex / host
```

The future Researcher writes only evidence into an untrusted staging path.

## Trust-aware component map

```text
UNTRUSTED
External source
  └─ Researcher quarantine
      └─ evidence bundle
          └─ Agent Review Authority boundary

TRUSTED CONTROL PLANE
Canonical manifests
  ├─ taxonomy
  ├─ policy
  ├─ Registry index
  ├─ Resolver
  └─ adapter rules

CONDITIONALLY TRUSTED DATA PLANE
Installed fixed resource content
  └─ selected active subset
      └─ host session
```

## Core invariants

1. The Resolver reads canonical manifests, not origin manifests.
2. Resolve performs no network access by default.
3. Activate verifies content hash immediately before materialization.
4. The active directory contains only selected resources.
5. The global Registry content store is outside the project workspace.
6. A resource cannot grant itself compatibility, permission or trust.
7. Policy denial has priority over score.
8. Selection is deterministic for identical inputs and versions.
9. Every selection has a machine-readable explanation.
10. A host adapter cannot silently broaden runtime requirements.
11. Project-scoped configuration can only restrict policy, never relax it (ADR-016).

## Data paths

### Search

```text
canonical manifests
  → schema validation
  → normalized records
  → SQLite index
  → filtered search results
```

### Resolve

```text
project files + task
  → bounded scanner
  → project profile
  → capability candidates
  → Registry retrieval
  → compatibility filters
  → policy filters
  → minimal coverage
  → confidence decision
  → selection plan
```

### Activate

```text
selection plan
  → source availability
  → immutable source verification
  → adapter validation
  → transactional staging directory
  → atomic swap
  → lockfile
```

### Research

```text
source reference
  → quarantine clone/download
  → bounded inventory
  → external scanners
  → provisional analysis
  → evidence bundle
  → Curator Agent preparation
  → independent admission review
  → Closure Agent decision
  → separate Registry contribution
```

## Failure behavior

Ossus fails closed when schemas are unknown, a hash differs, compatibility is unresolved, a policy cannot be loaded, a source is mutable without a lock, a security tier exceeds the allowed maximum, an adapter cannot prove its destination or a candidate path crosses a trust boundary.

Low-confidence task interpretation does not activate many skills. It produces an explanation and requires refinement or explicit confirmation.
