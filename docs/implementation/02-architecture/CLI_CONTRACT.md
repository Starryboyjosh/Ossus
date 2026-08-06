# CLI contract

## Binary

```text
ossus
```

The CLI is scriptable by default and offers human output plus `--format json`.

## Global flags

```text
--config <PATH>
--project <PATH>
--format <human|json>
--no-color
--quiet
--verbose
--offline
--yes
```

`--offline` is the default behavior for commands that do not explicitly require synchronization or installation.

## Command tree

```text
ossus init
ossus config show|validate
ossus validate <PATH>...
ossus registry add|remove|list|sync|status|reindex
ossus search
ossus show
ossus scan
ossus resolve
ossus explain
ossus activate
ossus deactivate
ossus lock verify|diff
ossus doctor
ossus eval
ossus audit
ossus research ...
```

## Core commands

### `ossus init`

Initializes project-local Ossus state.

```bash
ossus init --scope project
ossus init --scope user
ossus init --path ~/Dev/private-ossus
```

Must not install or activate any resource.

### `ossus search`

Searches canonical Registry metadata.

```bash
ossus search "responsive design"
ossus search --category security --risk-max R1
ossus search --surface claude-code-cli
ossus search --surface codex-cli --category frontend
ossus search --runtime external-cli-required
ossus search --source user
```

### `ossus scan`

Produces a bounded project profile.

```bash
ossus scan
ossus scan --format json > project-profile.json
```

Must not read ignored secrets by default. It records file types and selected config metadata, not arbitrary source content unless a detector explicitly requires it.

### `ossus resolve`

```bash
ossus resolve --task "Make this landing page responsive"
ossus resolve --task-file task.txt
ossus resolve --surface claude-code-cli
ossus resolve --risk-max R1
ossus resolve --dry-run
```

Output includes project signals, inferred capabilities, selected resources, excluded high-ranking candidates, policy decisions, confidence, context estimate and activation plan.

It does not activate unless `--activate` is explicitly supplied. V0 may omit the convenience flag and require a separate command.

### `ossus explain`

```bash
ossus explain --last
ossus explain <selection-id>
ossus explain resource <resource-id>
```

### `ossus activate`

```bash
ossus activate --selection <id> --target claude-code
ossus activate --from-lock skills.lock.json
```

Must verify hashes, stage changes, show permissions and risk, perform atomic materialization, write or update lockfile and preserve rollback information.

### `ossus deactivate`

Removes only Ossus-managed materialization.

It must never delete user-authored host skills that are not recorded as Ossus-owned.

### `ossus doctor`

Checks configuration, data paths, Registry index, schema versions, host availability, adapter destination, source integrity, symlink capability, stale transactions and lockfile consistency.

### `ossus eval`

```bash
ossus eval
ossus eval --case GOLD-036
ossus eval --format json
```

## Exit codes

```text
0   success
2   invalid CLI usage
10  configuration error
11  schema error
12  taxonomy error
20  Registry unavailable
21  source verification failure
30  low-confidence resolution
31  policy denial
32  incompatible surface
40  activation failed and rolled back
41  activation failed; manual recovery required
50  evaluation threshold failed
60  Researcher security boundary violation
69  command not implemented (valid only while a command group is a placeholder; a shipped command must not return 69)
70  internal error
```

Exit codes are stable public API after V0.

## Output rules

- Human output is concise and explains next action.
- JSON output is versioned.
- Security warnings go to stderr and remain present in JSON.
- `--quiet` cannot suppress denial or integrity warnings.
- Secrets and raw candidate content are never printed.
- Paths are normalized before display.
