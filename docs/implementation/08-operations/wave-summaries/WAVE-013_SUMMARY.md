# WAVE-013 Summary — Private catalogs and source precedence

## Status

Planned.

## Technical summary

Supports project, user, and private catalog overlays through explicit source priority, namespace rules, override records, conflict explanations, and project-local canonical entries. It must prevent silent identity impersonation.

## Practical plain-language summary

Teams can use their own catalogs alongside official entries, while Ossus visibly explains which source won and why.

## Expected evidence/deliverables

- Source-management CLI, precedence resolver, and conflict audit output.
- Tests for official/private ID collisions, project overrides, unknown publishers, revoked lower-priority sources, and hash conflicts.
- Visible overrides and lockfiles that record source identity.

## Dependencies/gates

Depends on WAVE-012. Changes affecting trust boundaries or policy behavior require Agent Review Authority review.

## Remaining work

Implement source precedence and audit records, add collision and provenance tests, and verify that no namespace can be silently impersonated.
