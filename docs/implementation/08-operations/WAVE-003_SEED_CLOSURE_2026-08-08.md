# WAVE-003 seed closure decisions — 2026-08-08

## Authority and separation

- Curator/implementer: `/root` Codex agent
- Independent admission review: `/root/seed_admission_review_a` and
  `/root/seed_admission_review_b`
- Closure Agent: `/root/wave003_security_closure`
- Advisory consultation: `/root/hardest_seed_reconciliation` (Sol Medium),
  advisory only and not an approval role
- No candidate content was executed; no official manifest was created.

## Profile 15 — dependency-audit CLI substitution

Candidate: `github/awesome-copilot`, commit
`ab7544d03d4c49fdd07f5958e1888ad39c4118e2`, subpath
`skills/security-review`, tree hash
`sha256:cbda6a05f7821345a816febb4dbf677980d55ed7aaefd84e9cafe333e2c65511`,
MIT.

Decision: **profile-level substitution accepted; Registry admission blocked**.

The source is an Agent Skill, not a standalone CLI. The only defensible
substitution is `agent-skills-standard` with `portable-with-adapter`; Claude or
Codex compatibility must not be inferred. A future Curator proposal must limit
the canonical capability to `security.dependency-audit`, exclude installation,
patching, history scans, remediation and full-source security review, and encode
R3 freshness/network controls plus strict redaction of secret and private
registry material. The static watchlist is a heuristic, not current advisory
authority.

## Profile 20 — MCP integration

Candidate: `githejie/mcp-server-calculator`, commit
`3dcaedcd58867206627d121092b401728db202da`, root subpath, tree hash
`sha256:35f80d2a73922bffe48010d65a0a2a8355e961004cc4322835e7370c8cb91341`,
MIT.

Decision: **rejected for admission and profile fill**.

Independent review found unbounded synchronous AST/math evaluation: no input,
depth, numeric-size, operation-count, timeout, cancellation, or memory budget.
One request can block or exhaust the stdio server. The stale lock, mutable CI and
container inputs, and unproven source-to-artifact provenance add supply-chain
concerns. Reconsideration requires a corrected immutable commit, bounded
evaluation, adversarial exhaustion tests, frozen/pinned build inputs, and fresh
independent security review.

## WAVE effect

These decisions do not admit, install, activate, or close WAVE-003. The official
Registry remains empty. Profile 16 remains unresolved; the profile-15
substitution still needs a Curator proposal and full admission chain; profile 20
needs a corrected candidate or an approved architecture-compliant replacement.

## Profile reconciliation checkpoint

The authoritative profile-by-profile design record is
`WAVE-003_SEED_PROFILE_RECONCILIATION.md`. It assigns one governed disposition
to every original profile and separates profile design from candidate
admission. The provisional admission-bearing target is 16; profiles 10, 17,
18, and 20 are intentionally unresolved. This does not admit any resource and
does not convert the target into a quota.

## Proposal decisions 2, 3, 6, 9, 10, 17 and 18

The Closure Agent did not curate, research, or independently review these
proposals. Review A is the counted independent review; Review B was interrupted.

- 2, 3, 6, 9 and 10: **blocked** pending complete canonical manifests,
  bounded triggers/exclusions/scopes/context, proven surfaces, and complete
  review records. Profile 10 must narrow `quality.code-review` to
  Supabase/Postgres migration/schema review.
- 17 and 18: **intentionally unresolved**. Their R1/read-only profile semantics
  remain valid, but independently verified immutable tuple, hash, license,
  content inventory, capability and host evidence are incomplete. A fresh
  evidence bundle is required before a new proposal.

No official manifest is authorized by these decisions.

## Closure-oriented continuation review

The Curator created three complete field drafts under the ignored quarantine
path `research-evidence/wave003-staging/`, deliberately omitting the review
block so that no pending draft could claim approval:

- `mohitagw.technical-spec-template.toml` — profile 6;
- `mohitagw.database-schema-design.toml` — profile 9;
- `supabase.postgres-best-practices.toml` — profile 10.

All three drafts passed the local canonical validator when evaluated with a
review block, confirming that their field shapes, taxonomy IDs, source locks,
hashes, runtime values and distribution values are schema-compatible. This is
validation evidence only.

Admission Review Agent A then independently re-reviewed the exact drafts:

- Profiles 6 and 9 have defensible static R0 prompt-pack classifications and
  exact source evidence, but the seed profiles require
  `agent-skills-standard`, `claude-code-cli` and `codex-cli`. Only the standard
  surface is evidenced. A standard-only profile substitution is permissible in
  principle, but requires this Closure Agent to record the dropped surfaces,
  degraded coverage, and an aggregate cross-host coverage check. No such
  acceptance has been issued.
- Profile 10 has a defensible Postgres-bound capability mapping, but its
  upstream content includes authoring, configuration, restore/import, RLS and
  contributor-test branches. An R1 read-only classification needs an immutable,
  independently reviewed enforcement adapter. Manifest exclusions and an empty
  `external_tools` list do not create that boundary. The profile remains
  blocked.

Admission Review Agent B independently reviewed the proposed profile-20
replacement `slettmayer/calc-mcp-server` v0.1.3 at commit
`805a177573c3d56cfa5e33f28571f9256fbbf92c`, root selection (no `subpath`),
MIT, and hash
`sha256:63342cde3e9642a4d71bf988d70671e5e46e3fb44491a6fb6989ca3d1cdfd0c1`.
The bounded evaluator and generic stdio MCP surface are positive. Admission is
still blocked by unpinned build requirements, unpinned `uvx` launch metadata,
stale `server.json` version metadata, release workflows/scripts in the root
source, missing Codex evidence, no explicit wall-clock bound for expensive
transcendental chains, and input-sensitive logging. The original calculator
remains rejected.

These re-reviews did not authorize a manifest, did not substitute a host
surface silently, and did not change the official Registry count: **0 / 20**.
