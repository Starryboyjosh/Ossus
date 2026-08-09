# Governance

## Roles

### Architecture owner agent

Maintains product coherence, WAVE boundaries, ADRs, and long-term compatibility.

### Taxonomy maintainer agent

Governs the capability vocabulary independently from individual resources.

### Curator Agent

Prepares canonical metadata and source locks. It cannot admit its own entry.

### Implementer Agent

Implements an assigned WAVE and produces evidence. It cannot independently review or close the same WAVE.

### Independent Review Agent

Reviews the final diff and evidence without implementing the reviewed change. Security-sensitive work requires an Independent Security Review Agent.

### Closure Agent

Has final technical, admission, gate, risk-disposition, and release-readiness authority after verifying the evidence and independent review. It cannot have implemented or independently reviewed the same change.

### Implementation support agent

Performs bounded implementation or test work under an Implementer Agent's scope. Support output has no review or closure authority.

## Decision rules

- A Registry admission requires separate Curator, Admission Review, and Closure Agents.
- No Researcher component can write trusted canonical state directly.
- Critical and high security findings block closure.
- Threshold reductions require an ADR and Closure Agent acceptance under the applicable change-control process.
- An implementation failure is not solved by weakening goldens.
- Security model names are implementation configuration, not permanent protocol.
- Every WAVE records a technical summary and practical plain-language summary before closure.

The normative authority and evidence rules are in [`docs/AGENT_AUTHORITY.md`](docs/AGENT_AUTHORITY.md).

## Initial maintainer

The initial repository owner is expected to be `@Starryboyjosh`. Replace or extend CODEOWNERS and maintainer records when the repository organization is created.
