$ErrorActionPreference = "Stop"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo is required"
    exit 127
}

cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
python scripts/test-hash-git-resource.py
cargo run -q -p ossus -- status
