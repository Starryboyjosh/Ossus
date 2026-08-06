# Source and installation model

## Data locations

Ossus uses platform-native data directories.

Conceptual layout:

```text
$OSSUS_HOME/
├── config/
├── registries/
├── index/
├── store/
├── transactions/
├── audit/
└── research/
    ├── quarantine/
    └── evidence/
```

`OSSUS_HOME` overrides the platform default.

Project state:

```text
project/
├── .ossus/
│   ├── config.toml
│   ├── selections/
│   ├── transactions/
│   └── managed.json
└── skills.lock.json
```

Host materialization is separate, for example `.claude/skills/`.

## Installation modes

### User-global

The user maintains an Ossus Registry and content store available to many projects.

### Project-local

A project may pin catalogs, policy, selection and lock state.

### Custom path

Teams may set a shared or private location, provided permissions and concurrency are handled.

## Source precedence

Default:

```text
project
user
official
```

A configuration can insert private team sources.

Precedence resolves lookup, not trust. Every source still has policy and identity.

## Sync

`ossus registry sync` fetches Registry metadata only, verifies expected source identity, validates snapshot and schemas, builds a temporary index, applies revocations, atomically swaps the index and preserves the previous index for rollback.

Sync never updates active resource bodies automatically.

## Install

Installation is explicit or part of an approved activation plan:

1. resolve immutable source;
2. download to temporary store;
3. enforce size and file limits;
4. verify digest;
5. inventory paths and symlinks;
6. store content-addressed;
7. record provenance;
8. do not activate yet.

## Local modifications

A modified installed resource is marked `clean`, `modified`, `unknown` or `missing`.

Modified content is never represented as the approved version.

Policy chooses whether modified local resources can be activated. Default: warn and require explicit confirmation; high-risk resources are blocked.
