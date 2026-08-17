# WAVE-003 independent acceptance review — 2026-08-16

## Decision record

- **WAVE:** WAVE-003 — Seed Registry and local search
- **Run ID:** `wave003-independent-acceptance-20260816-r1`
- **Reviewer:** independent read-only acceptance reviewer (Huygens)
- **Base:** `HEAD d3ed70690f65c9200e739175e92832b4c25c38e2` plus the working tree
  under review
- **Verdict:** **BLOCKED**
- **Conflict of interest:** the reviewer did not edit, approve, materialize,
  commit, or push any resource

The review confirms that the Registry mechanics and the three existing
manifests validate, but that the WAVE acceptance contract is not satisfied.
This is an acceptance blocker, not a request to weaken the seed target or to
count synthetic fixtures as real entries.

## Findings

### HIGH — F-001: insufficient real seed coverage

The official catalog contains three manifests for profiles 2, 6, and 9. The
provisional planning denominator has 13 unfilled slots, and the active
completion obligation remains 20 real admitted seed entries. The profile-2
admission did not amend that obligation.

### HIGH — F-002: required diversity is absent

The three official resources are one `skill` and two `prompt-pack` resources.
There is no admitted `mcp-server`, no positive real evidence for Claude, Codex,
standalone CLI, or generic MCP surfaces, no R1/R2/R3 resource, and no admitted
overlapping competitor for selection behavior. All three current entries are
R0 and standard-only.

### MEDIUM — F-003: authority records were stratified

The dated body of `WAVE-003_FINAL_COVERAGE_AUTHORITY.md` and the original
`WAVE-003_SEED_PROFILE_RECONCILIATION.md` described the pre-profile-2 count as
current/conditional. Both now identify those statements as historical
snapshots and provide a dated current-state addendum.

### MEDIUM — F-004: reproducible snapshot needed materialization

The profile-2 manifest and its two dated evidence records were initially
untracked, so the repository inventories generated from `git ls-files` omitted
them. They must be staged and the inventories regenerated from the staged Git
index before the checkpoint is committed.

## Candidate disposition

Only the three existing resources are admission-ready within their recorded
scopes. The remaining researched candidates are not admission-ready: they
require new source/adapter evidence, freshness or redaction controls, exact
host/surface proof, security review for executable behavior, or replacement
proposals. In particular, profiles 10, 17, 18, and 20 remain intentionally
unresolved, and profile 15 remains deferred.

## Minimum evidence before WAVE Closure

1. Admit 20 real entries, or first record an explicit architecture decision/ADR
   that changes the active obligation.
2. Provide a complete canonical manifest and immutable source evidence for each
   entry, including tree hash, license, inventory, capabilities, type,
   surfaces, runtime/risk, triggers, exclusions, and role chain.
3. Add real positive examples for `mcp-server`, host-exclusive/cross-host and
   standalone-CLI surfaces, at least one higher-risk resource with its required
   controls, and a capability-overlap pair.
4. Regenerate and verify the tracked repository inventories from the final Git
   index, including the profile-2 materialization and all current evidence.
5. Rerun manifest validation, deterministic reindex/status/FTS5 checks, exact
   and capability search, rebuild/conflict/malformed/F-09/JSON tests, and the
   hosted release-FTS5 matrix.
6. Record a distinct WAVE Closure decision with roles, run IDs, revisions,
   hashes, commands/results, findings/dispositions, conflicts of interest,
   residual risks, and the final decision.

## Review conclusion

The correct state after this review is:

```text
official_admitted                  = 3
active_completion_obligation      = 20
WAVE-003                           = IN PROGRESS / BLOCKED FOR CLOSURE
WAVE-004 authorization             = NO
```
