#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo is required" >&2
  exit 127
fi

cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
python3 scripts/test-hash-git-resource.py
cargo run -q -p ossus -- status

if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  ./scripts/check-layout.sh
else
  echo "skip: layout check requires a Git work tree"
fi

if command -v cargo-deny >/dev/null 2>&1; then
  cargo deny check
else
  echo "skip: cargo-deny is not installed"
fi
