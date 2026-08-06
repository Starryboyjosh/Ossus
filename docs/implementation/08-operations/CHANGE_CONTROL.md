# Change control

## Changes requiring an ADR

- trusted format;
- resource type;
- taxonomy major change;
- Resolver algorithm;
- risk semantics;
- source precedence;
- activation path;
- adapter trust claim;
- network default;
- external model default;
- signature strategy;
- custom execution environment;
- security model assignment.

## Changes requiring golden updates

- capability meaning;
- task mapping;
- selection scoring;
- tie-breaking;
- risk filtering;
- compatibility behavior;
- activation limits.

## Prohibited shortcut

Do not update tests, schemas or thresholds solely to make a failing implementation pass.

## WAVE scope changes

If a WAVE becomes substantially larger:

1. stop;
2. write a scope-delta note;
3. split the WAVE;
4. update dependencies and gates;
5. obtain architecture approval.

Do not hide a large redesign inside an implementation report.
