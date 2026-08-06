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

1. Opus 5 implementation review or plan review;
2. attributed implementation and test evidence, including any Luna Max assistance;
3. explicit findings disposition;
4. human closure.

See `docs/security/SECURITY_GOVERNANCE.md`.
