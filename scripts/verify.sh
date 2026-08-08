#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"

bash scripts/check_generated_state_clean.sh
bash scripts/test_no_legacy_names.sh
bash scripts/test_validate_authority.sh
bash scripts/check_security_matrix.sh
bash scripts/check_schemas.sh
bash scripts/check_schema_parity.sh
bash scripts/check_supply_chain.sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo doc --workspace --no-deps --locked
bash scripts/check_end_to_end.sh
bash scripts/check_core_coverage.sh
bash scripts/check_critical_mutation.sh
bash scripts/check_fuzz_smoke.sh
bash scripts/check_performance.sh
bash scripts/check_package.sh
bash scripts/check_generated_state_clean.sh
git diff --check
