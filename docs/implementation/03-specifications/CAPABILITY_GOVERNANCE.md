# Capability taxonomy governance

## Authority

The taxonomy is a product specification independent from the binary.

Roles:

- taxonomy maintainer agent;
- independent domain review agent;
- Closure Agent.

## Adding a capability

A proposal must include:

1. stable brand-neutral ID;
2. concise definition;
3. included behaviors;
4. excluded behaviors;
5. neighboring capabilities;
6. at least three positive examples;
7. at least three negative examples;
8. aliases;
9. at least one changed or new golden case;
10. migration impact.

Acceptance requires an independent domain review agent and a Closure Agent, both
separate from the proposing implementer. Human evidence may be attached but is
optional and does not replace the Closure Agent decision.

## Naming rules

```text
domain.capability
```

- lowercase;
- maximum two levels in V1;
- no framework or product brands;
- no vague verbs such as `fix`;
- no compound that mixes technology and outcome;
- synonyms are aliases, not new capabilities.

## Versioning

- PATCH: documentation and aliases that do not change meaning.
- MINOR: additive capability or compatible deprecation.
- MAJOR: split, merge, removal or semantic change.

## Unmapped resources

Allowed states:

```text
unmapped
needs-taxonomy-review
out-of-scope
```

An unmapped resource cannot participate in automatic resolution.

## Anti-explosion rules

- a new framework does not create a capability;
- a new host does not create a capability;
- a task phrasing does not create a capability;
- a resource must not define private capabilities used by the official Resolver;
- broad “full-stack” capabilities are rejected in favor of composed coverage.
