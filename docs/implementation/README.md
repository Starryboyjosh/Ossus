# Ossus — Implementation Package

**Project name:** Ossus

**Primary interface:** Rust command-line application

**Core domains:** Researcher, Registry, Resolver

**Package status:** active implementation and governance record

**Current implementation WAVE:** WAVE-003

**Language of implementation and active documentation:** English

Ossus is a local-first system for discovering, reviewing, indexing, selecting, and activating agent skills and related reusable resources.

Its core rule is:

> Stored does not mean approved. Approved does not mean installed. Installed does not mean active.

A user may keep a large catalog locally, but an agent should receive only the smallest compatible set required for the current project and task.

## Start here

Read these files in order:

1. `01-master-context/OSSUS_MASTER_CONTEXT.md`
2. `01-master-context/DECISION_LOG.md`
3. `09-roadmap/PHASES_AND_GATES.md`
4. `06-waves/WAVE_INDEX.md`
5. The current WAVE file
6. The matching implementation and review prompts in `07-prompts/`

Do not begin implementation from the historical Spanish documents. They are retained so a future architect can understand the original proposal and intentionally restructure Ossus if needed.

## Package map

```text
00-original-context/  Historical documents, preserved verbatim
01-master-context/    Product charter, decisions, glossary, handoff context
02-architecture/      Rust workspace, CLI, Registry, Resolver, Researcher
03-specifications/    Taxonomy, schemas, examples, policy and configuration
04-security/          Threat model, trust boundaries, risk tiers and gates
05-evaluations/       Golden cases, metrics and seed catalog profiles
06-waves/             Ordered implementation WAVEs with copy-ready prompts
07-prompts/           General, security and human-closure prompt templates
08-operations/        Reports, diagnostics, handoffs and change control
09-roadmap/           Phases, dependency graph and definitions of done
```

## Product order

Ossus is intentionally implemented in this order:

```text
Taxonomy and contracts
        ↓
Registry and local search
        ↓
Project scanner
        ↓
Deterministic Resolver
        ↓
Golden evaluation harness
        ↓
Safe activation boundary
        ↓
Claude Code adapter
        ↓
Complete CLI vertical slice
        ↓
Distribution and private catalogs
        ↓
Codex adapter
        ↓
Researcher
```

The Researcher is last because it is the most dangerous and expensive component. The Resolver is where the earliest practical value exists.

## Agent review authority

ADR-020 and `docs/AGENT_AUTHORITY.md` govern new WAVE decisions. Every WAVE uses an Implementer Agent, independent Review Agent (Security Review Agent when applicable), and distinct Closure Agent with final technical authority. No self-review or self-closure is permitted. Human review is optional additional evidence; it is not a required approval step. Every WAVE's reader summary must contain technical and practical summaries, evidence, dependencies/gates, status, and remaining work.

## Repository status

This package now lives inside the active Git repository. WAVE-001 established the Rust workspace, and subsequent WAVEs preserve their specifications, prompts, reports, and closure records here as governed implementation history.

Use `CURRENT_WAVE.md` for the active state. Historical Spanish originals remain under `00-original-context/`; active authorities and explicitly checked mirrors live at their repository paths.
