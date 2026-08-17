# WAVE-003 targeted admission packet — responsive design skill

**Date:** 2026-08-16
**Role:** Curator / implementation evidence only
**Candidate:** `wshobson.responsive-design`
**Profile:** 2 — responsive layout and accessibility
**Decision state:** `ACCEPTED — MATERIALIZED AS OFFICIAL ENTRY`

This packet records a bounded follow-up to the WAVE-003 final-coverage
authority. It began as Curator evidence and was later reviewed and accepted by
separate roles. The staged TOML draft remains quarantined under
`research-evidence/wave003-staging/`; the accepted canonical copy is
`catalog/official/manifests/wshobson.responsive-design.toml`.

## Why this candidate

The current official Registry contains only two prompt-packs and no admitted
`skill`. This candidate is a small, MIT-licensed Agent Skill subtree that can
add a real `skill` type and frontend capability coverage without introducing
scripts or runtime dependencies. The upstream catalog describes the skill as
responsive layout guidance using CSS Grid, Flexbox, and container queries.

The existing WAVE-003 proposal already identifies this source for profile 2,
but marks it conditional because the required Claude Code and Codex host
evidence is absent. This packet therefore proposes an explicit
`agent-skills-standard`-only surface. It does not claim Claude Code, Codex,
standalone CLI, or cross-host compatibility.

## Immutable source verification

Source tuple:

```text
repository: https://github.com/wshobson/agents
commit:     c4b82b0ad771190355eb8e204b1329732a18449a
subpath:    plugins/ui-design/skills/responsive-design
tree_hash:  sha256:0c8319415cae9458b074f6306b3e3f96b2a8b7d1a4340efdd36ec35b197a312d
```

The source was fetched into an isolated temporary checkout and inspected
without executing candidate content. The selected tree contains exactly five
regular `100644` Markdown blobs:

| File | Bytes |
|---|---:|
| `SKILL.md` | 1,989 |
| `references/breakpoint-strategies.md` | 10,658 |
| `references/container-queries.md` | 9,913 |
| `references/details.md` | 10,667 |
| `references/fluid-layouts.md` | 10,731 |

Total selected content is 43,958 bytes. No executable file, symlink, Gitlink,
script, package manifest, or build input is inside the selected subpath. The
repository root `LICENSE` is the MIT License, copyright Seth Hobson (2024).

Verification command and result:

```text
python3 scripts/hash-git-resource.py \
  /tmp/ossus-wshobson-agents \
  c4b82b0ad771190355eb8e204b1329732a18449a \
  plugins/ui-design/skills/responsive-design

sha256:0c8319415cae9458b074f6306b3e3f96b2a8b7d1a4340efdd36ec35b197a312d
```

The computed digest matches the proposed source lock and the earlier WAVE-003
evidence packet.

## Proposed canonical classification

- Type: `skill`; category: `frontend`.
- Required capabilities: `frontend.responsive-layout` and bounded
  `frontend.accessibility` semantics. The latter is limited to responsive
  interaction guidance such as touch targets, logical properties, and
  `aria-expanded`/`aria-controls`; it is not a WCAG or screen-reader audit.
- Risk: R0; runtime `instruction-only`; no external tools.
- Distribution: `source-only`; project scope.
- Compatibility: `agent-skills-standard` only; `portable-standard`.
- License: MIT.

The exact Curator fields are in
`research-evidence/wave003-staging/wshobson.responsive-design.toml`.

## Independent review and Closure

The independent Admission Review Agent
`admission-review-codex/wave003-responsive-design-20260816-r1` returned
`READY_FOR_CLOSURE`, with no critical, high, or medium findings. It verified
the immutable tuple, tree closure, MIT license, type, bounded capabilities,
triggers, exclusions, R0/instruction-only classification, and standard-only
surface. Its one low finding about the risk note was corrected before Closure.

The distinct Closure Agent
`closure-agent/wave003-responsive-design-20260816-c1` returned `ACCEPTED` for
the exact scope in the canonical manifest. The full acta is recorded in
`WAVE-003_RESPONSIVE_DESIGN_CLOSURE_2026-08-16.md`.

## Review questions answered

- The tuple, subtree closure, MIT license, and hash were confirmed.
- The Markdown-only source supports `skill`, R0, and `instruction-only`.
- The accessibility mapping was accepted only as bounded responsive
  interaction semantics, not a full audit.
- The surface was accepted as `agent-skills-standard` only; Claude/Codex and
  standalone CLI remain unproven.
- The distinct Closure decision was recorded before materialization.

## Residual gaps

- Claude Code, Codex, and standalone-CLI compatibility remain unproven.
- Aggregate cross-host diversity and the active WAVE-003 seed obligation remain
  incomplete.

## Completed follow-up

The accepted manifest was materialized after Closure. Post-materialization
evidence is recorded: schema validation returned no diagnostics; the
disposable Registry reindexed three resources with no exclusions and healthy
FTS5/integrity status; capability search and exact `show` returned the entry;
and local formatting, Clippy, workspace tests, CLI snapshots, release FTS5,
deterministic hash, and repository-layout checks passed. The inventory check
passed against the current Git index; the new files remain unstaged because
this work did not authorize a commit or push.
