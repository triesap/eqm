#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"

bash scripts/check_generated_state_clean.sh
bash scripts/test_no_legacy_names.sh
bash scripts/test_validate_authority.sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo doc --workspace --no-deps --locked
bash scripts/check_generated_state_clean.sh
git diff --check
