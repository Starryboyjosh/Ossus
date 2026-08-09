# Security policy

## Current status

Ossus is pre-release architecture and scaffold code. It must not be treated as a security boundary, sandbox, or production admission system.

## Reporting a vulnerability

Use GitHub private vulnerability reporting or a private security advisory when enabled for the repository.

Do not open a public issue containing exploit details, secrets, candidate payloads, or a working path traversal / activation bypass.

If private reporting has not yet been enabled, contact the repository owner privately through their verified GitHub account before disclosure. The maintainer must enable a dedicated private reporting channel before the first public beta.

## Scope

Security-sensitive areas include:

- canonical manifest validation;
- taxonomy and policy loading;
- Registry source identity;
- immutable source and hash verification;
- Resolver policy ordering;
- active-set limits;
- filesystem paths and symlinks;
- activation transactions and rollback;
- host adapter metadata;
- Researcher quarantine and staging CI;
- release and Registry supply chain.

## Security promises not made

Ossus does not currently claim to:

- sandbox agents;
- prevent all semantic prompt injection;
- certify third-party skills;
- make privileged resources safe;
- replace endpoint security;
- guarantee upstream source correctness.

## Review process

Security WAVEs require:

1. an Implementer Agent's attributed implementation and test evidence;
2. an Independent Security Review Agent's review of the final diff and evidence;
3. explicit findings disposition;
4. a distinct Closure Agent's final decision;
5. technical and practical WAVE summaries linked to the closure record.

Human review may be added as evidence but is not required for final technical authority. See [`docs/AGENT_AUTHORITY.md`](docs/AGENT_AUTHORITY.md) and `docs/security/SECURITY_GOVERNANCE.md`.
