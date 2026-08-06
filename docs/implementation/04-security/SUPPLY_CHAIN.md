# Supply-chain plan

## Rust dependencies

Required controls:

- committed `Cargo.lock`;
- dependency advisories in CI;
- license policy;
- duplicate and banned dependency review;
- source pinning;
- minimal features;
- no unreviewed git dependencies in releases;
- reproducible build investigation before stable release.

Recommended tools are selected during WAVE 10 after current verification. Ossus should integrate maintained ecosystem tooling rather than invent scanners.

## Release artifacts

- build from protected tagged commit;
- generate checksums;
- publish provenance where practical;
- separate build and release credentials;
- require human approval for release environment;
- retain SBOM or dependency inventory;
- verify installers against release digest.

## Registry snapshots

Each official Registry release has snapshot ID, manifest count, taxonomy version, schema version, content digest, revocation data and optional signature/transparency record.

## Source content

Every indexed resource uses immutable commit/digest, canonical subpath, tree/content hash, upstream license and review record.

Tags alone are not immutable evidence.

## Signature roadmap

V0: hashes and protected Git commits.

Pre-beta: signed release tags or artifact signing.

Stable public Registry: evaluate Sigstore or equivalent transparency-backed signing through an ADR.
