# Getting started

## Clone and inspect

```bash
git clone <repository-url> ossus
cd ossus
```

Read `AGENTS.md` before assigning the repository to a coding agent.

## Verify the Rust scaffold

```bash
./scripts/verify.sh
cargo run -p ossus -- status
```

The scaffold reports `WAVE-000`. Planned commands deliberately fail with a not-implemented exit code.

## Begin implementation correctly

1. Run the Opus 5 plan review prompt.
2. Disposition every finding and apply the required plan corrections.
3. Record human Security Gate S0 closure.
4. Update `docs/implementation/CURRENT_WAVE.md` to WAVE-001.
5. Implement only `docs/implementation/06-waves/01-rust-workspace-bootstrap.md`.

The repository already contains the workspace that WAVE-001 expected to create. The WAVE-001 implementation agent must audit, correct, and complete it rather than recreate it blindly.
