# Official Registry source

Canonical manifests accepted for the official source by the Registry admission
workflow live under `manifests/`.

WAVE-003 currently contains three Closure-approved static R0 manifests
(profiles 2, 6 and 9). Do not add a manifest without the separated Curator → independent
Admission Review → Closure chain; do not fabricate approvals merely to grow
the catalog.

The Almanac seed list is not part of this repository. Per the Gate S0 closure of
2026-08-04 (decision D5, finding F-06) it is held as untracked evidence under
`research-evidence/almanac-v0.1-import/`. It is an import allowlist, not an
official acceptance and not an activation source. Admission follows the
separated-agent policy in [`docs/AGENT_AUTHORITY.md`](../../docs/AGENT_AUTHORITY.md).
