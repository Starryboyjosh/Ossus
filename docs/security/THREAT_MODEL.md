# Ossus threat model

## Primary asset

> The approved Registry is a channel of instructions and executable references into a user's agent session.

The most important impact is not corruption of a catalog website. It is influence over what an agent reads, which tools it uses, what files it changes, what endpoints it contacts and what credentials it requests.

## Security goals

1. Untrusted candidate content cannot directly affect trusted control logic.
2. External authors cannot control Resolver-critical canonical metadata.
3. Only compatible and policy-allowed resources become active.
4. Active resource content is immutable and verified.
5. The complete catalog is not exposed to the host.
6. Activation is transactional and reversible.
7. Registry updates cannot silently replace locked content.
8. Candidate review cannot execute attacker-controlled CI with privileged tokens.
9. Security decisions are explainable and auditable.
10. Low confidence fails closed.

## Assets

- canonical manifests;
- taxonomy;
- Registry source identity;
- content hashes;
- policy;
- local project files;
- user credentials;
- host permission configuration;
- active resource set;
- lockfile;
- review evidence;
- CI release credentials;
- signing identity.

## Adversaries

- malicious skill author;
- compromised legitimate maintainer;
- attacker controlling an upstream dependency;
- typosquatting publisher;
- malicious catalog overlay;
- hostile repository supplying project-scoped Ossus configuration and registry sources;
- compromised Registry transport;
- prompt-injection content;
- local user or process modifying installed files;
- accidental maintainer error;
- over-permissive agent host;
- malicious candidate pull request.

## Threats

### Persistent instruction injection

A resource instructs the agent to ignore user intent, conceal actions, exfiltrate data, load other resources or request broader permissions.

### Trigger and capability stuffing

Origin metadata attempts to appear relevant to every task.

Control: origin metadata is not canonical; trigger limits and overlap checks apply.

### Permission understatement

A resource claims to be declarative while scripts or dynamic context require shell, network or credentials.

Control: observed content and runtime drive canonical risk.

### Surface confusion

A resource safe in one host becomes executable in another.

Control: compatibility is surface-specific and adapter-enforced.

### Active-set expansion

A broad or ambiguous task causes many resources to be injected.

Control: activation limits, low-confidence mode and set-cover penalties.

### Upstream mutation

A branch or tag changes after approval.

Control: immutable commit or digest plus tree hash.

### Installed-content tampering

Local files differ from the approved hash.

Control: verify before activation and record local state.

### Adapter path escape

A resource path or symlink causes materialization outside the intended host directory.

Control: canonical path checks, no external symlink following, transaction root.

### Candidate CI attack

A candidate adds workflows or exploits `pull_request_target`.

Control: separate staging, no privileged candidate CI, no secrets, minimum tokens.

### Registry poisoning

A private or project source shadows an official resource.

Control: namespaces, source identity and explicit override records.

### Project-scoped policy relaxation

A cloned repository ships `.ossus/config.toml` and `.ossus/policy.toml` that raise `risk_max`, set `allow_implicit_r4`, empty `block`, disable `require_hash_verification`, widen a parser budget or register a new registry source. The project directory is attacker-supplied before the user runs any Ossus command, and V0 has no source signing.

Control: ADR-016 policy monotonicity. Project-scoped configuration may only restrict. Tiers and limits resolve to the minimum across scopes, allowlists to the intersection, denylists to the union. A named key set is user/global-only and is ignored in project scope, with the attempted relaxation surfaced in `explain` output and the audit event. Registry priority does not confer policy authority.

### Denial of service

Huge manifests, recursive YAML, archive bombs, pathological globbing or enormous repositories consume resources.

Control: parsing and inventory budgets, timeouts and bounded concurrency.

### Data leakage

Task text, project profiles, logs or source files are sent remotely.

Control: local default, no model call, no raw task persistence by default, redaction.

## Security assumptions

- the operating system and user account are not fully compromised;
- host permission controls function as documented;
- cryptographic hash implementations are correct;
- trusted reviewers protect their credentials;
- immutable source references remain retrievable or cached;
- external sandboxes used later are configured correctly.

## Non-goals

Ossus does not sandbox agent hosts, prevent a user from explicitly approving dangerous actions, guarantee a resource has no semantic prompt injection, prove upstream source correctness, replace endpoint security or make R4 resources safe for implicit activation.
