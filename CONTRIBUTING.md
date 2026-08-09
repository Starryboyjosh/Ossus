# Contributing to Ossus

Ossus accepts contributions only when they preserve its trust model and implementation order.

## Before opening a pull request

1. Read `AGENTS.md`.
2. Identify the active WAVE or accepted issue.
3. Confirm that the change does not implement future scope early.
4. Add or update tests.
5. Run the complete verification suite.
6. Update documentation and change control records when required.

## Local checks

```bash
./scripts/verify.sh
```

Or:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Pull request expectations

A pull request must explain:

- the WAVE or issue it implements;
- trust boundaries affected;
- files and contracts changed;
- commands run;
- acceptance criteria evidence;
- residual risks;
- whether security review is required.
- the Implementer, Independent Review, and Closure Agent assignments where applicable;
- links to the WAVE's technical and practical reader summaries.

Use the repository pull request template.

## Changes that require an ADR

- trusted formats;
- new resource types;
- taxonomy major changes;
- Resolver algorithm or tie-breaking;
- risk semantics;
- source precedence;
- activation paths;
- network or external model defaults;
- signature strategy;
- custom execution environments;
- security model-role changes.

## Taxonomy changes

Use the taxonomy issue form. A capability proposal needs a stable brand-neutral ID, scope, positive and negative examples, aliases, neighboring capabilities, and golden-case impact.

## Security changes

Security WAVEs follow Implementer Agent -> Independent Security Review Agent -> Closure Agent workflow. Model names are configuration, and human review is optional evidence rather than a required approval. Do not merge unresolved critical or high findings. See [`docs/AGENT_AUTHORITY.md`](docs/AGENT_AUTHORITY.md).

## Candidate resources

Do not add untrusted candidate repositories, archives, scripts, workflows, or generated evidence to the main repository. The Researcher staging system is implemented later under separate security gates.

## Commits

Keep commits focused and written in English. Do not commit generated secrets, local quarantine data, host caches, or activation transactions.
