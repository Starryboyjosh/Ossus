# Data contracts

## Canonical manifest

Trusted source format:

```text
canonical-manifest.toml
```

The manifest is parsed into a typed Rust structure, serialized to JSON for schema validation and indexing, then subject to semantic validation.

Required dimensions:

- identity and schema versions;
- resource type;
- source lock and hash;
- capabilities;
- categories;
- compatibility surfaces;
- runtime requirements;
- portability;
- installation scopes;
- risk tier;
- review state;
- context measurement;
- distribution mode;
- adapter permissions.

`source.mode` and `distribution.mode` are orthogonal axes. `source.mode` records provenance—where the content comes from—and retains the values `remote-index`, `vendored` and `local-private`. `distribution.mode` records redistribution rights—what Ossus may do with the content downstream:

- `source-only` — Ossus may record an index entry and point at the upstream source. Content is never copied into the local store.
- `approved-install-only` — content may be installed locally after approval, but not redistributed further.
- `vendored-redistributable` — content may be copied into a catalog and redistributed.

The axes have this cross-field invariant:

| `source.mode` | Permitted `distribution.mode` |
|---|---|
| `remote-index` | `source-only`, `approved-install-only` |
| `vendored` | `approved-install-only`, `vendored-redistributable` |
| `local-private` | `source-only`, `approved-install-only` |

`remote-index` never grants a redistribution right, so pairing it with `vendored-redistributable` is a contradiction. `vendored` means a copy already exists, so pairing it with `source-only` is a contradiction. `local-private` is private by definition and can never be redistributed. Validation rejects all contradictory pairs.

`notice_required` is meaningful only for `vendored-redistributable`, where a license notice must accompany the copy.

## Generated Registry record

The SQLite record is derived and reproducible. It is not edited directly.

Core tables:

```text
resources
capabilities
resource_capabilities
categories
resource_categories
surfaces
resource_surfaces
runtime_requirements
resource_runtime_requirements
sources
reviews
fts_resources
```

The database stores normalized metadata, never candidate bodies.

## Project profile

Generated JSON:

```json
{
  "schema_version": "1.0.0",
  "root": ".",
  "languages": [],
  "frameworks": [],
  "package_managers": [],
  "databases": [],
  "ci_systems": [],
  "host_signals": [],
  "files_considered": 0,
  "bytes_considered": 0,
  "limits_reached": []
}
```

## Selection plan

```json
{
  "schema_version": "1.0.0",
  "selection_id": "sel_...",
  "catalog_snapshot": "...",
  "taxonomy_version": "1.0.0",
  "project_profile_hash": "sha256:...",
  "task_hash": "sha256:...",
  "surface": "claude-code-cli",
  "policy_hash": "sha256:...",
  "confidence": 0.0,
  "required_capabilities": [],
  "selected": [],
  "excluded": [],
  "warnings": []
}
```

Raw task text is optional in persisted state. Default persistence stores its hash and a user-controlled summary to reduce sensitive data retention.

## Lockfile

Generated JSON named:

```text
skills.lock.json
```

The familiar phrase `skills.lock` remains the product concept, but V0 uses an explicit JSON filename.

The lockfile records Ossus version, taxonomy version, a SHA-256 digest of the exact governed taxonomy inputs, Registry snapshots, a SHA-256 policy hash, target surface and detected version, resource IDs and versions, source commit or digest, tree/content hash, adapter version, materialization path, reason and capability coverage, activation mode, timestamp and local modification state. `taxonomy_hash` and `policy_hash` use canonical lowercase `sha256:<64 hex>` identities so version labels cannot silently conceal changed trusted inputs.

## Parsing budgets

Every parser accepts a budget object.

Initial defaults:

```text
manifest bytes                  256 KiB
task bytes                       64 KiB
config bytes                    256 KiB
maximum string length            32 KiB
maximum list items                2,000
maximum nesting depth                32
maximum manifests per source     50,000
maximum project files scanned   100,000
maximum project bytes sampled    64 MiB
```

Limits are configurable downward by policy at any scope.

Increasing a security-sensitive limit is a **user/global-scope-only** operation and produces audit output. Per ADR-016, an upward budget change in a project-scoped file is ignored rather than honored: a cloned repository's committed configuration is attacker-supplied input, not explicit user configuration. Budgets resolve across scopes by taking the minimum.

## Source transport allowlist

`source.repository` is a fetch target whenever `source.mode = "remote-index"`, so its transport is a trust decision, not a formatting detail.

| `source.mode` | Allowed form |
|---|---|
| `remote-index` | `https://` only |
| `vendored` | `https://` only, recording the upstream the vendored copy came from |
| `local-private` | an absolute local path, resolved with platform path APIs |

`file://`, `git://`, `http://`, `ssh://` and scp-style `user@host:path` references are rejected. SSH is excluded specifically because agent forwarding would make a manifest able to borrow the user's credentials.

The schema pattern and its enforcement are added in WAVE-012 with the install path; the contract is fixed here so no earlier WAVE has to guess.

## Forward compatibility

Unknown major schema versions fail closed.

Canonical manifests set `additionalProperties: false` at every level, so an unknown field in any version is **rejected**, not preserved. This is deliberate: silently carrying an unrecognized field through a trusted contract is how origin metadata acquires canonical authority. Adding an optional field is therefore a schema release, not a free minor change, and the schema must never be relaxed to admit a field that an implementation wants.

Migrations are explicit and produce a diff.
