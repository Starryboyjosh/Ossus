# Operational risk tiers

## R0 — Declarative

Characteristics: instruction-only, no scripts, no shell, no network, no MCP and no binaries.

Review: automatic validation, focused human reading of all content, one approver.

Implicit activation may be allowed by policy.

## R1 — Read-only

May read project files through host tools.

Review: complete human reading, activation tests and one approver with checklist.

## R2 — Local write

May modify project files.

Review: complete human reading, disposable-environment test, expected diff tests and two approvers.

Default policy requires confirmation.

## R3 — Shell or network

May execute commands, call a CLI or access network.

Review: external static analysis, disposable runner without secrets, network allowlist where possible and two approvers including one security reviewer.

Never silently broaden host permissions.

## R4 — Credentials or remote actions

May access credentials, publish, deploy, send messages or mutate remote systems.

Review: resource-specific threat model, synthetic credentials, minimum permission proof and two security approvers.

Explicit invocation only. Implicit activation is forbidden.

## R5 — Privileged or destructive

Examples: root/administrator actions, unrestricted destructive commands, disabling security controls or persistence outside project/user scope.

Excluded from stable initial Registry. Requires a future RFC and separate channel.
