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

The selected Linux/amd64 child manifest is
`sha256:fae033b815a16f930325c2697e620362be4d2e5d739a301b10ad1fc9c8643a06`.

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

Hosted result for this new lane: **PASS** on CI run
[19](https://github.com/Starryboyjosh/Ossus/actions/runs/31298472061), job
[`93207265220`](https://github.com/Starryboyjosh/Ossus/actions/runs/31298472061/job/93207265220),
commit `65b79e1e21d96f406e099bfcd98b551c4f6198a7`. The job completed the
x86_64 assertion, package/toolchain logging, workspace tests and release FTS5
test. GitHub retains the logs; unauthenticated API access exposes job/step
metadata but cannot download them.

## Existing hosted evidence

The previous declared matrix remains independently recorded:

| Platform | Environment | Release FTS5 |
|---|---|---|
| Ubuntu | GitHub-hosted native runner | PASS — CI run 19, job `93207265221` |
| Arch Linux | Arch userspace in Ubuntu-hosted Docker container | PASS — CI run 19, job `93207265220`; not native Arch |
| macOS | GitHub-hosted native runner | PASS — CI run 19, job `93207265246` |
| Windows | GitHub-hosted native runner | PASS — CI run 19, job `93207265192` |

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
3. Decide the final seed denominator through architecture authority; do not use
   the target as an admission quota.

No Resolver, scanner, installation, activation, adapter, synchronization or
Researcher automation work is authorized by this handoff.
