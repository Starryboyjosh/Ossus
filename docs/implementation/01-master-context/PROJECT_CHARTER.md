# Ossus project charter

## Purpose

Build a dependable local CLI for curating and resolving agent capabilities without forcing users to expose their projects to a remote service or spend model tokens on routine selection.

## Primary users

- individual developers maintaining a personal skill library;
- teams maintaining private catalogs;
- agent-tool authors testing portability;
- security-conscious users who need provenance and activation limits;
- maintainers curating a public Ossus Registry.

## Jobs to be done

1. Find resources by category, host, runtime and risk.
2. Understand what a resource can do and where it works.
3. Keep many resources installed without exposing all of them to an agent.
4. Resolve the smallest useful set for a task.
5. Reproduce the selected environment.
6. Verify that local content still matches an approved source.
7. Investigate new candidates without allowing them into the trusted path.

## V0 scope

- Rust CLI;
- one official/local Registry source;
- one project overlay;
- three resource types;
- 44 capabilities;
- 20 manually curated entries;
- local SQLite search index;
- deterministic project scanner;
- deterministic Resolver;
- 50 golden cases;
- Claude Code adapter;
- lockfile and explanations;
- basic integrity audit.

## Explicitly out of V0

- public website;
- approval without independent review and Closure Agent decision;
- custom sandbox;
- custom static-analysis engine;
- Reddit crawler;
- GitHub crawler;
- automatic model routing;
- opaque cloud service;
- R5 resources;
- automatic execution of candidates;
- full Codex adapter.

## Quality attributes

Priority order:

1. safety of activation;
2. correctness and explainability;
3. reproducibility;
4. local privacy;
5. deterministic behavior;
6. cross-platform installation;
7. performance;
8. extensibility.

## Product constraints

- default network use is off during resolve and activate;
- default external model calls are zero;
- no resource may broaden host permissions silently;
- the full Registry must not be mounted into an agent workspace;
- updates never change active content without verification;
- security warnings cannot be hidden by a manifest;
- schemas and taxonomy are versioned independently from the binary.

## Review and admission governance

ADR-020 assigns final technical acceptance, risk disposition, Registry
admission, and gate closure to a Closure Agent. Every WAVE separates the
Implementer Agent, Independent Review Agent, and Closure Agent; model names are
configuration. Registry admission separately requires a Curator Agent, Admission
Review Agent, and Closure Agent. Human review may provide additional evidence
but is optional. The Researcher remains evidence-only and cannot approve or
activate its candidates.

## License posture

The implementation should use a permissive open-source license unless the repository owner decides otherwise before publication.

Registry entries preserve upstream license metadata. Indexing a source does not imply permission to redistribute its contents.

Vendoring requires a documented license decision.
