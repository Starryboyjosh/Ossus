# Host compatibility

Ossus models compatibility as independent dimensions rather than a single boolean.

## Initial surfaces

- `agent-skills-standard`
- `claude-code-cli`
- `claude-agent-sdk`
- `claude-api-host`
- `codex-cli`
- `codex-ide`
- `generic-terminal-agent`
- `generic-mcp-client`
- `standalone-cli`

## Example distinction

A resource that invokes a local executable may be:

```text
compatible: claude-code-cli
runtime: shell-required + external-cli-required
portable: portable-with-adapter
```

The same resource may be incompatible with an API-only Claude host because that host does not expose local shell or filesystem capabilities.

## Search examples

```bash
ossus search --category security
ossus search --surface claude-code-cli
ossus search --surface codex-cli --category frontend
ossus search --runtime external-cli-required
ossus search --risk-max R1
```

Host adapters must detect or warn about host version and optional capabilities. They must not broaden permissions silently.
