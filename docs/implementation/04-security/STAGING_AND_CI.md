# Candidate staging and CI

## Repository separation

Use:

```text
ossus main repository
ossus-registry main repository, if separated later
ossus-staging repository or fork
```

Candidate content never enters a privileged branch merely to be reviewed.

## CI prohibitions

- no `pull_request_target` execution of candidate code;
- no write token on untrusted jobs;
- no repository secrets;
- no package publishing;
- no release credentials;
- no reusable workflow that grants elevated permissions to candidate-controlled inputs;
- no automatic merge from staging.

## Candidate job permissions

Start from:

```yaml
permissions:
  contents: read
```

Add no permissions unless a reviewed job requires them.

Prefer offline analysis on uploaded immutable artifacts.

## Main-repository path controls

Even trusted contributions must not modify release workflows, signing configuration, admission policy, taxonomy governance or security-role assignments without independent security review and a distinct Closure Agent decision. Human review may provide additional evidence but is not required.

## Artifact handling

Candidate evidence artifacts receive random IDs, have retention limits, are treated as untrusted downloads, are never executed by reviewers, exclude secrets and preserve source hashes.

## Branch protection

For the future public repository: required reviews, status checks, protected release process, no force pushes, restricted workflow modification and dependency review.
