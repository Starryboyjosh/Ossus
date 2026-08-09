# WAVE-016 Summary — Cross-platform distribution

## Status

Planned.

## Technical summary

Defines the supported target matrix and produces archive/installer layouts, release automation, completions, upgrade behavior, platform-native data paths, and Windows path/link fallback handling.

## Practical plain-language summary

This makes Ossus practical to install and upgrade across supported operating systems without silently changing a user’s shell setup.

## Expected evidence/deliverables

- Release automation, platform smoke tests, and installation documentation.
- Clean-install, custom `OSSUS_HOME`, spaces/Unicode paths, Windows copy-fallback, and upgrade/rollback tests.
- Target CLI and activation smoke-test results plus published checksums.

## Dependencies/gates

Depends on WAVE-015. Auto-update is out of scope unless separately approved through an ADR and security WAVE; relevant release-path changes require Agent Review Authority review.

## Remaining work

Implement the distribution matrix and installers, validate platform data-path and fallback behavior, run smoke tests, and publish verified checksums.
