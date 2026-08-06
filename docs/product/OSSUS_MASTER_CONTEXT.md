# Ossus master context

## 1. Mission

Ossus is a local-first, CLI-centered platform that manages reusable capabilities for coding agents.

It has three domains:

1. **Researcher** — discovers and prepares external candidates.
2. **Registry** — stores trusted canonical metadata and reproducible source references.
3. **Resolver** — chooses and activates the smallest compatible set for a project and task.

The first useful release is not an Internet crawler. It is a measured Resolver operating over a manually curated Registry.

## 2. Product thesis

Native hosts already support skill discovery and progressive loading. Ossus does not replace those mechanisms.

Ossus adds:

- catalog-level trust and provenance;
- cross-host compatibility modeling;
- risk-aware filtering;
- local policy;
- deterministic minimal selection;
- reproducible activation;
- measurable resolver quality;
- private catalog overlays;
- future evidence-driven discovery.

## 3. Non-negotiable principles

### 3.1 Trust zones are explicit

```text
External source
  → quarantine
  → candidate evidence
  → approved Registry entry
  → installed resource
  → active resource
  → host session
```

Each transition is a separate decision.

Scope is a trust zone too. User and global configuration express user intent. Project-scoped configuration arrives with the repository and is untrusted input, so it may only restrict policy, never relax it (ADR-016).

### 3.2 The approved catalog is an instruction channel

A skill that reaches the active set may influence model behavior, tool use, filesystem access, network actions, credential requests and other skills.

Approval means accepted under documented conditions. It never means universally safe.

### 3.3 Canonical metadata is controlled by Ossus

External authors do not control the fields used by the Resolver.

Capabilities, triggers, exclusions, runtime requirements, observed permissions, compatibility, risk, context cost and review state belong to the canonical manifest written by curators.

### 3.4 Deterministic by default

The base Resolver must work with:

- project signals;
- a governed capability vocabulary;
- aliases;
- local full-text search;
- policy filters;
- deterministic weighted set cover;
- stable tie-breaking.

External model calls default to zero.

### 3.5 Index first

The Registry stores canonical manifests, provenance, fixed commits and hashes. Resource contents are fetched at installation time.

Vendoring is exceptional and requires license and security justification.

### 3.6 Human review is risk-scaled

Human authority remains final, but review depth depends on risk.

R0 declarative resources receive focused full-content review. R2 and above require deeper execution and security review. R5 is excluded from the stable initial catalog.

### 3.7 The Researcher never approves

The Researcher may discover, clone into quarantine, inventory, collect evidence, suggest a provisional mapping and generate a review bundle.

It may never write the final canonical manifest, approve an entry, activate a resource, modify admission policy or write directly to the main Registry.

## 4. Initial resource types

Only three resource types are in the initial schema:

- `skill`
- `prompt-pack`
- `mcp-server`

Other concepts may still be indexed as external tools or compatibility dependencies, but adding a first-class resource type requires an RFC and schema change.

## 5. Compatibility dimensions

A resource is classified independently by type, functional category, capability coverage, host surface, runtime requirement, portability, installation scope, source, risk tier and activation mode.

Do not use a single boolean such as `claude = true`.

Initial surfaces include:

- `agent-skills-standard`
- `claude-code-cli`
- `claude-agent-sdk`
- `claude-api-host`
- `codex-cli`
- `codex-ide`
- `generic-terminal-agent`
- `generic-mcp-client`
- `standalone-cli`

A standalone CLI may be useful to Claude Code when shell access is available, while still being unsuitable for an API-only Claude host. Ossus must model that difference.

## 6. Implementation language

Ossus is implemented in Rust as a Cargo workspace using Rust edition 2024.

Reasons:

- a single distributable binary;
- predictable local performance;
- strong typed contracts;
- cross-platform CLI support;
- no mandatory runtime installation;
- good fit for deterministic parsing, indexing and policy;
- easier confinement of the trusted core.

Rust does not make hostile content safe. Parsing limits, filesystem boundaries, process isolation and policy remain mandatory.

## 7. Canonical file formats

Trusted human-authored configuration uses TOML.

Generated interchange and schemas use JSON.

Host-facing adapters may generate YAML frontmatter or host-specific files.

External YAML is treated as an import format and is never copied directly into trusted canonical state.

This applies to the taxonomy and the frozen evaluation data as well: they are trusted human-authored inputs and are TOML, not YAML (ADR-017). No YAML parser exists in the trusted computing base.

## 8. First vertical slice

```text
20 curated canonical manifests
        ↓
local Registry index
        ↓
project scan
        ↓
task-to-capability mapping
        ↓
risk and compatibility filters
        ↓
minimal deterministic selection
        ↓
explanation
        ↓
Claude Code materialization
        ↓
skills.lock
```

The vertical slice must pass the frozen golden evaluation suite before release.

## 9. Initial CLI

The binary name is `ossus`.

Core command groups:

```text
ossus init
ossus config
ossus registry
ossus search
ossus scan
ossus resolve
ossus explain
ossus activate
ossus deactivate
ossus lock
ossus doctor
ossus eval
ossus audit
ossus research
```

Research commands are unavailable or marked experimental until the Researcher phase.

## 10. Security implementation rule

The following WAVEs are security WAVEs and require Opus 5 security ownership:

- WAVE 00
- WAVE 07
- WAVE 10
- WAVE 17
- WAVE 18
- WAVE 20
- WAVE 21
- WAVE 23

Luna Max or another implementation agent may assist with bounded implementation and test work. Opus 5 must review the final diff and evidence, and a human closes each gate.

Other WAVEs may still require a security review when they modify trust boundaries, activation, parsing, networking, source handling, installation, update logic or CI.

## 11. Escalation rule

Normal errors must be investigated and corrected by the implementer.

Only after repeated failures or a serious technical block, produce a diagnostic file containing:

- complete commands;
- complete logs;
- complete error output;
- environment and toolchain versions;
- repository status;
- hypotheses already tested;
- full contents of only the directly involved code files;
- no unrelated secrets or files.

Use `08-operations/BLOCKED_DIAGNOSTIC_TEMPLATE.md`.

A diagnostic is not a substitute for attempting a correction.

## 12. Definition of success

Ossus V0 is successful when it can:

- validate trusted manifests;
- search and filter a local catalog;
- detect common project signals;
- map tasks to governed capabilities;
- select a small compatible set deterministically;
- block prohibited risk and host combinations;
- explain selection and exclusion;
- activate only selected resources for Claude Code;
- verify source hashes;
- create a reproducible lockfile;
- pass quality thresholds over 50 frozen goldens;
- operate without an external model call.
