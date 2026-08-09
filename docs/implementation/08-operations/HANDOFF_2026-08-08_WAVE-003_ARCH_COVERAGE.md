# Ossus WAVE-003 Arch coverage handoff — 2026-08-08

## State

`WAVE-003 — IN PROGRESS`

Registry mechanics remain complete and locally verified. The reconciled seed
target is still provisional at 16 admission-bearing profiles, with 2 official
resources (profiles 6 and 9). WAVE-004 remains unauthorized.

This handoff adds the required Arch Linux validation lane. It does not add a
resource, change the Registry schema, or claim native Arch-host coverage.

## Arch validation

The workflow now defines `test-arch-container` on a GitHub-hosted
`ubuntu-latest` runner. It mounts the checked-out repository into an immutable
official Arch Linux `base-devel` image and selects `linux/amd64` explicitly:

```text
archlinux:base-devel@sha256:c1829f370be8434135f43fb3acaef1256780804ac3b2d2eec90dfb1232e1ffdf
```

The container installs the current Arch package updates needed for
certificates and rustup, installs Rust `1.97.1`, logs the package/toolchain
versions, runs the workspace tests with `--all-features --locked`, and runs the
release `ossus-registry` FTS5 test. The source checkout is mounted read-only;
Cargo writes to an isolated runner-temporary target mount, and checkout
credentials are not persisted.
The local reproduction passed both the full workspace suite and release FTS5.
The image index resolves to the Linux/amd64 manifest used by the hosted
runner. This is Arch userspace validation on an Ubuntu kernel, not a native
Arch host or a packaging/AUR test.

Hosted result for this new lane: **PENDING FIRST RUN**.

## Existing hosted evidence

The previous declared matrix remains independently recorded:

| Platform | Environment | Release FTS5 |
|---|---|---|
| Ubuntu | GitHub-hosted native runner | PASS — CI run 16, job `93198830494` |
| macOS | GitHub-hosted native runner | PASS — CI run 16, job `93198830507` |
| Windows | GitHub-hosted native runner | PASS — CI run 16, job `93198830495` |
| Arch Linux | Arch userspace in Ubuntu-hosted Docker container | PENDING hosted; local PASS |

Do not collapse these rows into a generic “Linux” result.

## Verification before push

- Local Arch container: workspace tests and release FTS5 pass on Rust `1.97.1`.
- Existing local baseline: 111 workspace tests, 24 CLI tests, one release FTS5
  test, two Git hashing tests, formatting, Clippy and `./scripts/verify.sh`
  pass.
- Official Registry remains 2 entries with fingerprint
  `fnv1a64:dbada94391f09954`.

## Review and authority

Independent reviewer `/root/seed_admission_review_b` accepted the exact
workflow after the hardening above. Distinct Closure Agent
`/root/wave003_security_closure` accepted the CI change under
`closure/wave003-arch-container-ci-20260809`. That decision applies only to
this workflow diff and does not admit a resource or close WAVE-003. The mutable
`pacman -Syu` package layer, Rustup/Cargo network inputs and the
container-versus-native limitation remain recorded residual risks.

## Remaining WAVE-003 blockers

1. Admit additional resources only after Curator → independent Review → Closure.
2. Keep profiles 10, 16, 17, 18 and 20 fail-closed until their evidence or
   profile decisions change through the governed process.
3. Obtain and record the hosted Arch result, separately from Ubuntu/macOS/
   Windows and with its container limitation stated; retained logs must show
   x86_64, the image/index and resolved amd64 digest, package/toolchain output,
   and both locked test commands.
4. Decide the final seed denominator through architecture authority; do not use
   the target as an admission quota.

No Resolver, scanner, installation, activation, adapter, synchronization or
Researcher automation work is authorized by this handoff.
