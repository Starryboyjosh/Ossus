# WAVE-003 Arch validation CI review

## Scope

This record covers the supplemental `test-arch-container` job added to
`.github/workflows/ci.yml`. It validates an Arch Linux userspace on an
Ubuntu-hosted GitHub runner; it does not claim a native Arch host, Arch kernel,
systemd, AUR or package-artifact result.

## Independent review

- Reviewer: `/root/seed_admission_review_b`
- Review type: independent CI and trust-boundary review
- Review date: 2026-08-08
- Verdict: **ACCEPT WITH HARDENING APPLIED; HOSTED RESULT STILL REQUIRED**

The reviewer verified that the immutable OCI index digest resolves to a
Linux/amd64 child (`sha256:fae033b815a16f930325c2697e620362be4d2e5d739a301b10ad1fc9c8643a06`),
that explicit `--platform linux/amd64` selects it, and that the host-side
checkout avoids requiring Node or Git in the Arch image. The workspace mount,
Rust `1.97.1` installation, locked workspace tests and locked release FTS5
test are structurally correct.

The reviewed hardening is present in the workflow:

- `actions/checkout` uses `persist-credentials: false`;
- the source checkout is mounted read-only;
- Cargo writes to a separate runner-temporary target mount;
- the host asserts `x86_64` before selecting the amd64 container;
- package versions and the active Rust toolchain are logged;
- top-level workflow permissions remain `contents: read`.

## Residual risk

`pacman -Syu` resolves current rolling Arch repositories, so the package layer
is not bit-reproducible even though the base image digest and Rust version are
pinned. The job records package versions and keeps the image digest authoritative.
A future reviewed maintenance change may replace this with an Arch repository
snapshot or a reviewed prebuilt validation image; that is not required to
claim the current userspace result.

The container runs as root inside an ephemeral hosted job, but source is
read-only and build output is isolated in a temporary mount. No token,
credential, Docker socket, release secret or `pull_request_target` path is
passed to the container.

## Closure

- Closure Agent: `/root/wave003_security_closure`
- Closure record: `closure/wave003-arch-container-ci-20260809`
- Decision: **ACCEPT**, limited to this exact workflow diff

The acceptance does not admit a resource and does not close WAVE-003. Any
change to the image digest, mounts, permissions, bootstrap commands or test
commands requires a new independent review and Closure decision. The accepted
residual risks are the mutable `pacman -Syu` package layer, Rustup/Cargo network
inputs, the absence of a publisher-signature policy, and the distinction
between Arch userspace and a native Arch host.

Operational evidence is still required: a successful hosted run for the
materialized commit, retained logs showing the platform/image/package/toolchain
and both locked test commands, and a report entry with the run/job URL and the
resolved amd64 child digest.
