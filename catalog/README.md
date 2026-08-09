# Registry catalog

This directory is reserved for trusted canonical Registry data.

It must never contain:

- raw Internet candidates;
- cloned upstream repositories;
- unreviewed author manifests;
- candidate CI workflows;
- quarantine contents;
- locally activated resources.

The initial catalog is index-first. Canonical manifests point to immutable source commits and hashes. Real seed entries are curated in WAVE-003.

External import allowlists do not live here. Per ADR-012 and the Gate S0 closure
of 2026-08-04 (decision D5, finding F-06), non-canonical imported indexes are held
outside the privileged repository under the untracked `research-evidence/` tree, or
in a separate staging repository. Nothing under `catalog/` may be external-origin.

Promotion of an imported entry requires the applicable Registry WAVE, immutable
source locks, canonical metadata, license evidence, risk classification, an
independent admission review, and a distinct Closure Agent decision. The
Curator, Admission Review, and Closure Agents are separate; see
[`docs/AGENT_AUTHORITY.md`](../docs/AGENT_AUTHORITY.md).
