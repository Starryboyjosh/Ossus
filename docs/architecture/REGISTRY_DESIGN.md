# Registry design

## Responsibility

The Registry answers what resources exist, who approved their canonical metadata, where immutable content is located, which capabilities they cover, which hosts and runtimes they support, what risk and review conditions apply and whether local content matches the approved source.

It does not decide the final selection. That is the Resolver.

## Source model

A Registry source has source ID, priority, kind, location, snapshot identifier, trust policy, signature state and last sync state.

Initial precedence:

```text
project overlay
  > user private Registry
  > official Registry
```

A higher-priority source may override metadata only under explicit conflict rules. It cannot silently impersonate a lower source's namespace.

## Namespaces

Resource IDs are globally stable:

```text
publisher.resource-name
```

Reserved namespaces require maintainership proof.

Private sources should use organization or user namespaces.

## Index first

Canonical manifests contain immutable source references.

Installation fetches only selected resources.

The Registry may cache source archives in a content-addressed store:

```text
$OSSUS_HOME/store/sha256/<digest>/
```

The store is not mounted wholesale into projects.

## Search

Search combines exact resource ID and name, exact capabilities, category filters, aliases, SQLite FTS/BM25 over trusted descriptions, and compatibility/runtime/source/risk filters.

Search results are not activation decisions.

## Conflict handling

Conflicts include same ID with different publisher identity, same version with different hash, incompatible schema versions, source-priority override, local modification, revoked resource and duplicate upstream fork.

A hash conflict blocks installation and activation.

## Revocation

A Registry may publish a snapshot-bound revocation record containing affected resource and source lock, severity, reason, replacement, action and effective date.

Offline users apply the latest synced revocation state. Ossus must state its freshness.

## Reindexing

The index is disposable and fully rebuildable from canonical manifests.

Index migrations never rewrite canonical source files.
