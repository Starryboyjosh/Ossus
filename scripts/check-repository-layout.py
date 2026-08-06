#!/usr/bin/env python3
from copy import deepcopy
import json
from pathlib import Path
import re
import sys

root = Path(__file__).resolve().parents[1]
required = [
    "README.md",
    "AGENTS.md",
    "Cargo.toml",
    "docs/product/OSSUS_MASTER_CONTEXT.md",
    "docs/security/THREAT_MODEL.md",
    "docs/implementation/06-waves/WAVE_INDEX.md",
    "specs/taxonomy/capabilities-v1.toml",
    "evaluations/goldens/goldens-v1.toml",
]

# ADR-017 restricts trusted canonical state to TOML and JSON. A .yaml or .yml
# file reappearing under a trusted path means either an unconverted import or a
# regression of the conversion, so fail rather than let it be indexed.
trusted_roots = ["specs", "evaluations", "catalog", "docs"]
stray_yaml = sorted(
    str(path.relative_to(root))
    for directory in trusted_roots
    for path in (root / directory).rglob("*")
    if path.is_file() and path.suffix in {".yaml", ".yml"}
)
if stray_yaml:
    for item in stray_yaml:
        print(f"forbidden YAML under a trusted path (ADR-017): {item}", file=sys.stderr)
    raise SystemExit(1)

missing = [item for item in required if not (root / item).exists()]
if missing:
    for item in missing:
        print(f"missing: {item}", file=sys.stderr)
    raise SystemExit(1)


def validate_hash_contract() -> None:
    schema_path = root / "specs/schemas/skills-lock.schema.json"
    example_path = root / "specs/examples/skills-lock.example.json"
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    example = json.loads(example_path.read_text(encoding="utf-8"))
    required_hashes = {"taxonomy_hash", "policy_hash"}
    required_fields = set(schema.get("required", []))
    if not required_hashes <= required_fields:
        raise ValueError("skills-lock schema must require taxonomy_hash and policy_hash")

    properties = schema.get("properties", {})
    hash_pattern = "^sha256:[0-9a-f]{64}$"
    for field in sorted(required_hashes):
        contract = properties.get(field)
        if not isinstance(contract, dict) or contract.get("type") != "string":
            raise ValueError(f"skills-lock {field} must be a string")
        if contract.get("pattern") != hash_pattern:
            raise ValueError(f"skills-lock {field} must use the canonical SHA-256 pattern")
        value = example.get(field)
        if not isinstance(value, str) or re.fullmatch(r"sha256:[0-9a-f]{64}", value) is None:
            raise ValueError(f"skills-lock example has invalid {field}")

    invalid_cases = []
    for field in sorted(required_hashes):
        missing_hash = deepcopy(example)
        del missing_hash[field]
        invalid_cases.append((field, "missing", missing_hash))
        for label, value in [
            ("uppercase", "sha256:" + "A" * 64),
            ("short", "sha256:" + "a" * 63),
            ("long", "sha256:" + "a" * 65),
            ("wrong-prefix", "sha512:" + "a" * 64),
        ]:
            invalid = deepcopy(example)
            invalid[field] = value
            invalid_cases.append((field, label, invalid))

    for field, label, document in invalid_cases:
        value = document.get(field)
        accepted = field in document and isinstance(value, str) and re.fullmatch(
            r"sha256:[0-9a-f]{64}", value
        )
        if accepted:
            raise ValueError(f"invalid {field} case unexpectedly accepted: {label}")


try:
    validate_hash_contract()
except (json.JSONDecodeError, OSError, ValueError) as error:
    print(f"invalid trusted contract: {error}", file=sys.stderr)
    raise SystemExit(1) from error

mirror_pairs = [
    (
        "specs/schemas/skills-lock.schema.json",
        "docs/implementation/03-specifications/skills-lock.schema.json",
    ),
    (
        "specs/examples/skills-lock.example.json",
        "docs/implementation/03-specifications/skills-lock.example.json",
    ),
    (
        "docs/architecture/DATA_CONTRACTS.md",
        "docs/implementation/02-architecture/DATA_CONTRACTS.md",
    ),
]
for source, mirror in mirror_pairs:
    if (root / source).read_bytes() != (root / mirror).read_bytes():
        print(f"mirror differs: {source} != {mirror}", file=sys.stderr)
        raise SystemExit(1)

print("repository layout: ok")
