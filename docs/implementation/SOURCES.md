# External references for implementation

These sources were checked while preparing the plan on 1 August 2026. Reverify version-sensitive behavior during the WAVE that depends on it.

## Host behavior

- Claude Code skills: https://code.claude.com/docs/en/skills
  - project skills under `.claude/skills/`;
  - progressive loading;
  - Agent Skills support plus Claude-specific controls;
  - explicit and model-invoked behavior.

- Claude Code plugin marketplaces: https://code.claude.com/docs/en/plugin-marketplaces
  - plugin distribution and versioning;
  - marketplace source and strictness controls.

- Codex skills: https://learn.chatgpt.com/docs/build-skills
  - repository and user locations under `.agents/skills`;
  - progressive disclosure;
  - explicit and implicit invocation;
  - symlink support and plugin distribution.

## Rust implementation

- Cargo workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Cargo manifests: https://doc.rust-lang.org/cargo/reference/manifest.html
- clap: https://docs.rs/clap/
- rusqlite: https://github.com/rusqlite/rusqlite
- Serde: https://docs.rs/serde/
- TOML: https://docs.rs/toml/
- Schemars: https://docs.rs/schemars/
- jsonschema: https://docs.rs/jsonschema/
- tracing: https://docs.rs/tracing/
- sha2: https://docs.rs/sha2/

## Security integration

- Semgrep local and CLI scans: https://semgrep.dev/docs/category/local-and-cli-scans

## Format note

The formerly common `serde_yaml` crate is deprecated and unmaintained. Ossus therefore does not use YAML as its trusted canonical format. A maintained parser for external YAML must be selected and reviewed separately when an adapter or importer needs it.
