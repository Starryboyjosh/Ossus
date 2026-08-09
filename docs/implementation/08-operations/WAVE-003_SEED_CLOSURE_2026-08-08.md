# WAVE-003 seed closure decisions — 2026-08-08

## Authority and separation

- Curator/implementer: `/root` Codex agent
- Independent admission review: `/root/seed_admission_review_a` and
  `/root/seed_admission_review_b`
- Closure Agent: `/root/wave003_security_closure`
- Advisory consultation: `/root/hardest_seed_reconciliation` (Sol Medium),
  advisory only and not an approval role
- No candidate content was executed. The two official manifests recorded later
  are metadata-only source locks; no resource body was installed or activated.

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

These decisions do not admit, install, activate, or close WAVE-003. At that
stage the official Registry was empty. Profile 16 remains unresolved; the profile-15
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

No official manifest was authorized by those earlier decisions; the later
profile-6/profile-9 Closure record below supersedes that interim block for
those two exact resources only.

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
  exact source evidence. Only the standard surface is evidenced, so the
  proposed substitution drops the unproven Claude Code and Codex surfaces.
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

## Closure decision — profiles 6 and 9

On 2026-08-08 the distinct Closure Agent recorded
`closure/wave003-r0-standard-6-9-20260808`:

- Profile 6 substitution accepted (`agent-skills-standard` only) and
  `mohitagw.technical-spec-template` admitted.
- Profile 9 substitution accepted (`agent-skills-standard` only) and
  `mohitagw.database-schema-design` admitted.
- Independent review: `admission-review-a/wave003-r0-standard-6-9-20260808`.
- Curator: `/root`; Closure Agent: `/root/wave003_security_closure`.
- Review wire tier: `light-human`; `approved_commit` equals the immutable
  upstream commit `fddfbc4a6caa8b4d3d41a69c666efaaff9d42def`.

The two manifests are now the only official Registry entries. They claim only
the Agent Skills standard surface and do not supply Claude Code, Codex,
standalone-CLI, or aggregate cross-host coverage. Materialization checks remain
future host work and are not inferred as implemented adapters. The official
Registry count is **2 / 16 provisional admission-bearing slots**.

## Final-admission sprint review

The final bounded review did not create a new Closure candidate. Admission
Review Agent A recorded a distinct block for profile 15:
`admission-review-a/profile-15-dependency-audit-20260808`. The pinned MIT
subtree is source-safe and standard-shaped, but the candidate is a broad
security scanner rather than an enforced dependency-only adapter. It lacks a
current advisory freshness protocol, mandatory secret/private-registry
redaction evidence, and a reviewed adapter that restricts scope to manifests and
lockfiles. It is **not ready for Closure** and remains out of the official
Registry.

The same sprint produced Curator-only amendment packets for profiles 5, 7, 11
and 12 in `WAVE-003_PROFILE_AMENDMENT_DECISIONS.md`. Those packets propose
explicit R3 contracts where execution, browser, command or write behavior is
intrinsic; they are not profile approvals or resource admissions. A bounded
profile-16 replacement triage found no clean candidate, so the profile remains
valid and intentionally unfilled. No later-WAVE implementation was started.

## Subsequent profile-only Closure — 2026-08-09

Admission Review Agent A independently narrowed the four amendments: profile 5
R2→R3, profile 7 R1→R2, profile 11 R0→R2, and profile 12 R1→R3. The distinct
Closure Agent recorded
`closure/wave003-profile-amendments-5-7-11-12-20260809` and accepted those
ceilings as **profile-only** decisions. No candidate, adapter, host activation,
canonical manifest or Registry entry was approved. Future resources still need
their own Curator → independent Review → Closure chain.

The same Closure decision accepted 16 only as a provisional planning
denominator; it did not replace the active WAVE completion obligation of 20
real admitted seeds. The final coverage authority is recorded in
`WAVE-003_FINAL_COVERAGE_AUTHORITY.md`.
