# Operational risk tiers

## R0 — Declarative

Characteristics: instruction-only, no scripts, no shell, no network, no MCP and no binaries.

Review: automatic validation and focused full-content review by one independent Review Agent, followed by a distinct Closure Agent decision.

Implicit activation may be allowed by policy.

## R1 — Read-only

May read project files through host tools.

Review: complete agent review, activation tests, and one independent Review Agent using a checklist, followed by a distinct Closure Agent decision.

## R2 — Local write

May modify project files.

Review: complete agent review, disposable-environment testing, expected-diff tests, and two independent Review Agents, followed by a distinct Closure Agent decision.

Default policy requires confirmation.

## R3 — Shell or network

May execute commands, call a CLI or access network.

Review: external static analysis, a disposable runner without secrets, a network allowlist where possible, and two independent Review Agents including one Security Review Agent, followed by a distinct Closure Agent decision.

Never silently broaden host permissions.

## R4 — Credentials or remote actions

May access credentials, publish, deploy, send messages or mutate remote systems.

Review: a resource-specific threat model, synthetic credentials, minimum-permission proof, and two independent Security Review Agents, followed by a distinct Closure Agent decision.

Explicit invocation only. Implicit activation is forbidden.

## R5 — Privileged or destructive

Examples: root/administrator actions, unrestricted destructive commands, disabling security controls or persistence outside project/user scope.

Excluded from stable initial Registry. Requires a future RFC and separate channel.

## Schema-v1 review-tier labels

Where schema version 1 serializes `light-human`, `full-human`, or `security-human`, those are compatibility wire labels only. They mean focused agent review, complete agent review, and security-agent review respectively; they do not require a human reviewer. Review depth and the independent Closure Agent decision remain required.
