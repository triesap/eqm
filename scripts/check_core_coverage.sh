#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
nightly="nightly-2026-07-16"
report="$(mktemp)"
trap 'find "$report" -delete' EXIT

cd "$repository_root"
command -v cargo-llvm-cov >/dev/null
command -v jq >/dev/null
cargo "+$nightly" llvm-cov -p eqm_engine --all-targets --locked --branch \
  --json --output-path "$report" >/dev/null

core_pattern='eqm_engine/src/(applicability|conformance|coverage|equivalence|exposure|freshness|matrix|monotonicity|release)\.rs$'
line_count="$(jq --arg pattern "$core_pattern" '[.data[0].files[] | select(.filename | test($pattern)) | .summary.lines.count] | add' "$report")"
line_covered="$(jq --arg pattern "$core_pattern" '[.data[0].files[] | select(.filename | test($pattern)) | .summary.lines.covered] | add' "$report")"
branch_count="$(jq --arg pattern "$core_pattern" '[.data[0].files[] | select(.filename | test($pattern)) | .summary.branches.count] | add' "$report")"
branch_covered="$(jq --arg pattern "$core_pattern" '[.data[0].files[] | select(.filename | test($pattern)) | .summary.branches.covered] | add' "$report")"

test "$line_count" -gt 0
test "$branch_count" -gt 0
test "$((line_covered * 100))" -ge "$((line_count * 90))"
test "$((branch_covered * 100))" -ge "$((branch_count * 85))"

printf 'core coverage: lines %s/%s (%s.%02d%%), branches %s/%s (%s.%02d%%)\n' \
  "$line_covered" "$line_count" \
  "$((line_covered * 100 / line_count))" "$((line_covered * 10000 / line_count % 100))" \
  "$branch_covered" "$branch_count" \
  "$((branch_covered * 100 / branch_count))" "$((branch_covered * 10000 / branch_count % 100))"
