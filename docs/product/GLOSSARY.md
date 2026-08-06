# Glossary

**Active resource** — A resource materialized for a specific project and host after resolution.

**Adapter** — Host-specific code that converts an Ossus selection into a structure recognized by Claude Code, Codex or another surface.

**Canonical manifest** — Trusted metadata controlled by Ossus curators and used by the Registry and Resolver.

**Candidate** — An external resource that passed minimum automated checks but is not approved.

**Capability** — A governed semantic identifier such as `frontend.accessibility`.

**Catalog source** — A Registry index available to Ossus, such as the official catalog, a private team catalog or a project overlay.

**Compatibility surface** — The host environment in which a resource can function, such as `claude-code-cli` or `codex-cli`.

**Evidence bundle** — A non-authoritative package containing provenance, inventory, scan results and reviewer material for a candidate.

**Installed resource** — Content fetched and verified locally but not necessarily active.

**Origin metadata** — Claims supplied by an external author. It is evidence, not trusted Resolver input.

**Policy** — Deterministic rules that allow, deny or require confirmation for sources, risks, permissions and activation.

**Quarantine** — A local area for untrusted source content that is outside trusted Registry and agent paths.

**Registry** — The trusted catalog of canonical manifests and source references.

**Researcher** — The domain that discovers and prepares candidate evidence.

**Resolver** — The domain that maps project plus task to a minimal compatible resource set.

**Risk tier** — Operational classification from R0 declarative to R5 privileged or destructive.

**Selection** — The resolved resource set before host materialization.

**Skill** — A directory of reusable instructions and optional supporting files following an Agent Skills-compatible shape or an adapted equivalent.

**Source lock** — A fixed commit, release digest or immutable content hash.

**Trust zone** — A boundary with defined permissions and allowed transitions.
