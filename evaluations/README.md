# Resolver evaluations

The initial Resolver is tuned against reviewed frozen cases, not ad-hoc demonstrations.

- `goldens/goldens-v1.toml`: 50 capability-level cases.
- `seed-catalog-profiles.toml`: 20 profiles that must be filled with reviewed real resources in WAVE-003.
- `fixtures/`: project fixtures added by WAVE-004.
- `reports/`: generated evaluation reports; local output is ignored.

Do not weaken a case to make an implementation pass. Change expected behavior only through reviewed taxonomy or architecture decisions.
