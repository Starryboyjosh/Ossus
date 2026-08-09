# Ossus master implementation plan

## Executive summary

Ossus is a Rust CLI and local catalog system for agent skills, prompt packs and MCP servers.

It is composed of:

- **Registry** — trusted canonical metadata and source locks;
- **Resolver** — project-aware minimal selection;
- **Researcher** — later discovery and evidence preparation.

Implementation begins with the Registry and Resolver. The Researcher is deferred until the trusted contracts, evaluation suite, activation boundary and host adapter are proven.

## Why CLI first

A CLI works in existing developer workflows, can be called by Claude Code, Codex, scripts and CI, supports local/private catalogs, avoids requiring a hosted service, makes deterministic behavior observable and can distribute as a single Rust binary.

The primary command is `ossus`.

## Product behavior

```bash
ossus init
ossus registry status
ossus search --category frontend --surface claude-code-cli
ossus scan
ossus resolve --task "Improve the landing page and make it responsive"
ossus explain --last
ossus activate --selection <id> --target claude-code
ossus lock verify
```

## Resolver economics

Ossus does not send thousands of skill bodies to a model.

It works from compact trusted metadata: capabilities, categories, triggers, project signals, compatibility, runtime, risk and context estimate.

A local model is optional later, only for ambiguous task-to-capability mapping.

## Security thesis

The final dangerous step is activation, not discovery.

Even an approved skill can influence an agent. Therefore canonical metadata is curator-owned, policy filters precede scoring, resource content is fixed and hashed, active sets are small, host materialization is transactional, R4 is never implicit and candidates remain outside trusted CI and Registry state.

Technical acceptance, risk disposition, Registry admission, and security-gate
closure follow ADR-020. An Implementer Agent prepares evidence, an Independent
Review Agent evaluates it, and a separate Closure Agent makes the final decision.
Registry entries use Curator Agent, Admission Review Agent, and Closure Agent
roles. Human review is optional supporting evidence; model names are deployment
configuration. The Researcher remains evidence-only and cannot approve or
activate candidates.

## WAVE reader summaries

Every WAVE has a summary under
`docs/implementation/08-operations/wave-summaries/` with a technical summary,
a practical plain-language summary, status, evidence, dependencies/gates, and
remaining work. Final evidence must be reflected before a WAVE closes.

## MVP

The V0 release contains 44 capabilities, 20 manually curated entries, a local SQLite Registry, bounded project scanner, deterministic Resolver, 50 goldens, Claude Code adapter, lockfile, integrity audit and cross-platform CLI release candidate.

## Expansion

After V0:

1. Registry synchronization.
2. Private catalogs.
3. Codex adapter.
4. Optional local semantic fallback.
5. Cross-platform distribution.
6. Researcher security design.
7. Passive intake.
8. GitHub discovery.
9. External analysis integration.
10. Agent admission workflow.
11. Reddit community evidence.
12. Researcher security audit.
13. Beta release.
