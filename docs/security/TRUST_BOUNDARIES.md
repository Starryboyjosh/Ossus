# Trust boundaries

## Boundary A — External source to quarantine

Allowed:

- bounded fetch;
- immutable reference resolution;
- inventory;
- hashing;
- passive parsing;
- external scanner invocation in disposable environment.

Forbidden:

- automatic execution;
- secrets;
- trusted CI token;
- following symlinks outside root;
- writing canonical manifests.

## Boundary B — Quarantine to evidence bundle

Only normalized findings and referenced excerpts cross.

Raw instructions remain labeled untrusted.

Evidence is not Resolver input.

## Boundary C — Evidence to canonical Registry

A Curator Agent creates canonical metadata. An independent Admission Review Agent reviews it, and a distinct Closure Agent makes the final admission decision. Human review may supplement evidence but is not required. The Researcher remains evidence-only and may not create, approve, or close admission for a candidate it discovered.

Required:

- source lock;
- hash;
- license decision;
- capability mapping;
- compatibility;
- observed runtime;
- risk tier;
- review record.

## Boundary D — Registry to installed content

Install verifies immutable source and digest.

No automatic activation.

## Boundary E — Installed content to active set

Resolver plus policy creates an explicit selection.

Hash is rechecked.

Only selected resource files are materialized.

## Boundary F — Active set to host

Adapter generates host-specific metadata with least privilege.

The host remains responsible for actual tool permissions.

Ossus must not call selection a sandbox.

## Boundary G — Host to operating system

Outside Ossus control.

Ossus provides warnings, runtime declarations and policy recommendations but cannot guarantee host enforcement.
