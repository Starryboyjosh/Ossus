# Governance

## Roles

### Architecture owner

Maintains product coherence, WAVE boundaries, ADRs, and long-term compatibility.

### Taxonomy maintainer

Governs the capability vocabulary independently from individual resources.

### Registry curator

Creates or approves canonical metadata and source locks.

### Security implementer

Owns security WAVEs and reviews their final evidence. For this project, the configured model is Opus 5.

### Implementation support

Performs bounded implementation and test work under the security implementer's written scope. Luna Max is the preferred support model when available, but support output has no independent security authority.

### Human approver

Resolves disagreement, accepts residual risk, and closes security gates.

## Decision rules

- No model can approve a Registry entry.
- No Researcher component can write trusted canonical state directly.
- Critical and high security findings block closure.
- Threshold reductions require explicit architecture approval.
- An implementation failure is not solved by weakening goldens.
- Security model names are implementation configuration, not permanent protocol.

## Initial maintainer

The initial repository owner is expected to be `@Starryboyjosh`. Replace or extend CODEOWNERS and maintainer records when the repository organization is created.
