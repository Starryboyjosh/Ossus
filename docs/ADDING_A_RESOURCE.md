# Adding a resource from zero to a project

This is the target workflow. Researcher commands are implemented only in later WAVEs.

## 1. Discover

Find a candidate manually or through a source connector. Record the upstream URL and resolve an immutable commit or digest.

The source may be:

- a general Agent Skills-compatible skill;
- a Claude Code extension;
- a Codex-compatible skill;
- a standalone CLI used by an agent;
- a prompt pack;
- an MCP server.

Discovery is searchable by functional category, host surface, runtime, portability, risk, and source.

## 2. Quarantine

Fetch the source into an external quarantine root, never into the trusted Registry repository or agent-visible project directory.

Do not execute install scripts, Git hooks, submodules, workflows, binaries, or dynamic prompt instructions.

## 3. Build evidence

Collect:

- immutable source reference;
- content/tree hash;
- file inventory;
- license evidence;
- runtime and external-tool observations;
- static-analysis findings;
- maintenance and community signals;
- prompt-injection and instruction-risk notes.

Evidence remains untrusted.

## 4. Classify risk

Assign an operational tier from R0 through R5 based on observed behavior, not author claims.

A declarative R0 resource can use a focused review path. Shell, network, credentials, remote writes, binaries, or privileged behavior require progressively deeper review.

## 5. Create the canonical manifest

A human curator writes or approves a new canonical manifest. Resolver-critical fields are controlled by Ossus:

- capabilities;
- categories;
- triggers and exclusions;
- host surfaces;
- runtime requirements;
- portability;
- scopes;
- risk;
- review state;
- measured context;
- immutable source and hash.

The upstream manifest is evidence only.

## 6. Admit to the Registry

Submit the canonical manifest through the trusted Registry contribution path. Candidate content and candidate CI remain outside privileged branches.

Reviewers verify source stability, license, risk, compatibility, taxonomy mapping, and required reviewer count.

## 7. Sync or install

A user synchronizes Registry metadata, then selectively fetches the fixed resource content. Installation verifies the immutable source and hash but does not activate it.

## 8. Resolve for a project

```bash
ossus scan
ossus resolve --task "Review this frontend and make it responsive" --surface claude-code-cli
ossus explain --last
```

The Resolver uses canonical metadata, local project signals, policy, and deterministic minimal coverage. It does not read every resource body or call a large model by default.

## 9. Activate

```bash
ossus activate --selection <selection-id> --target claude-code
```

Activation re-verifies content, stages only the selected resources, validates host paths, performs a transactional swap, and records ownership.

## 10. Lock and audit

`skills.lock.json` records source commits, hashes, adapter version, target surface, capability reasons, and materialized paths.

Future runs can verify drift, revocation, local modification, and source freshness.
