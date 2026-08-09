# Official Registry source

Canonical manifests accepted for the official source by the Registry admission
workflow will live under `manifests/`.

The directory is empty in the scaffold by design. Do not fabricate approvals merely to populate it.

The Almanac seed list is not part of this repository. Per the Gate S0 closure of
2026-08-04 (decision D5, finding F-06) it is held as untracked evidence under
`research-evidence/almanac-v0.1-import/`. It is an import allowlist, not an
official acceptance and not an activation source. Admission follows the
separated-agent policy in [`docs/AGENT_AUTHORITY.md`](../../docs/AGENT_AUTHORITY.md).
