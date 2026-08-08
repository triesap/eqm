#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output="$repository_root/mutants.out"
base_target="${CARGO_TARGET_DIR:-$repository_root/target}"
mutation_target="$base_target/mutants-critical"

cd "$repository_root"
command -v cargo-mutants >/dev/null
command -v jq >/dev/null
if test -e "$output"; then
  echo "mutation output already exists: $output" >&2
  exit 1
fi
cleanup() {
  if test -d "$output"; then
    find "$output" -depth -delete
  fi
}
trap cleanup EXIT

set +e
CARGO_TARGET_DIR="$mutation_target" cargo mutants -p eqm_engine \
  --file 'crates/eqm_engine/src/{conformance,monotonicity,release}.rs' \
  --in-place --timeout 60 --minimum-test-timeout 20 --colors never
mutation_status=$?
set -e

test -f "$output/outcomes.json"
test "$(jq -r '.outcomes[0].summary' "$output/outcomes.json")" = "Success"
generated="$(jq 'length' "$output/mutants.json")"
caught="$(wc -l < "$output/caught.txt" | tr -d ' ')"
missed="$(wc -l < "$output/missed.txt" | tr -d ' ')"
timed_out="$(wc -l < "$output/timeout.txt" | tr -d ' ')"
unviable="$(wc -l < "$output/unviable.txt" | tr -d ' ')"
test "$generated" -eq "$((caught + missed + timed_out + unviable))"
viable="$((caught + missed + timed_out))"
killed="$((caught + timed_out))"
test "$viable" -gt 0
test "$((killed * 100))" -ge "$((viable * 80))"

printf 'critical mutation: killed %s/%s (%s.%02d%%), missed %s, unviable %s\n' \
  "$killed" "$viable" "$((killed * 100 / viable))" \
  "$((killed * 10000 / viable % 100))" "$missed" "$unviable"
if test "$mutation_status" -ne 0 && test "$missed" -eq 0 && test "$timed_out" -eq 0; then
  echo "mutation runner failed without a classified survivor" >&2
  exit "$mutation_status"
fi
