# Resolver metrics and gates

## Capability mapping

- micro precision;
- micro recall;
- micro F1;
- macro F1;
- unmapped rate;
- correct low-confidence rate;
- confusion between neighboring capabilities.

## Resource selection

- precision at k;
- required-resource recall;
- exact set match;
- false activations per case;
- redundant resources;
- constraint violations;
- incompatible surface selections;
- correctly blocked high-risk resources.

## Efficiency

- local resolve p50 and p95;
- index size;
- metadata bytes considered;
- selected context estimate;
- savings versus full catalog;
- files materialized;
- external model calls.

## Initial V0 gates

```text
Capability micro-F1                  >= 0.90
Required-resource recall             >= 0.90
False activations per case           <= 0.20
Constraint violations                = 0
Implicit R4 activation               = 0
Local resolution p95                 < 500 ms at 1,000 manifests
Metadata/context reduction           >= 80%
External model calls by default      = 0
Determinism failures                 = 0
```
