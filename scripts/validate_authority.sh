#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"

traceability="docs/specification/requirements.tsv"
checksums="docs/specification/AUTHORITY.sha256"

fail() {
  echo "authority validation failed: $*" >&2
  exit 1
}

[[ -f "$traceability" ]] || fail "missing traceability index"
[[ -f "$checksums" ]] || fail "missing checksum manifest"
[[ "$(head -n 1 "$traceability")" == $'decision_id\tauthority\tplanned_checkpoint' ]] ||
  fail "invalid traceability header"

checkpoint_file="$(mktemp)"
cleanup() {
  rm -f "$checkpoint_file"
}
trap cleanup EXIT

awk '
  /^## Checkpoint Map/ { in_map = 1; next }
  /^## / { in_map = 0 }
  in_map && /^\| `step_[0-9][0-9][0-9]` / {
    value = $0
    sub(/^\| `step_/, "", value)
    sub(/`.*/, "", value)
    print value
  }
' docs/execution/rcl/eqm_v1_[0-9][0-9]_*.md | LC_ALL=C sort >"$checkpoint_file"

[[ "$(wc -l <"$checkpoint_file" | tr -d ' ')" == "134" ]] ||
  fail "checkpoint sequence does not contain 134 rows"

for number in $(seq 1 134); do
  expected="$(printf '%03d' "$number")"
  actual="$(sed -n "${number}p" "$checkpoint_file")"
  [[ "$actual" == "$expected" ]] || fail "checkpoint sequence expected $expected, found ${actual:-missing}"
done

decision_count=0
while IFS=$'\t' read -r decision authority checkpoint extra; do
  ((decision_count += 1))
  expected="$(printf 'EQM-%03d' "$decision_count")"
  [[ "$decision" == "$expected" ]] || fail "expected $expected, found ${decision:-missing}"
  [[ -z "${extra:-}" ]] || fail "extra traceability column for $decision"
  [[ -f "$authority" ]] || fail "missing authority $authority for $decision"
  [[ "$checkpoint" =~ ^step_[0-9]{3}$ ]] || fail "invalid checkpoint for $decision"
  number="${checkpoint#step_}"
  rg -q "^${number}$" "$checkpoint_file" || fail "unknown checkpoint $checkpoint for $decision"
done < <(tail -n +2 "$traceability")

[[ "$decision_count" == "140" ]] || fail "traceability index does not contain 140 decisions"

register_count="$(rg -o '^\| EQM-[0-9]{3} \|' docs/specification/decision-register.md | wc -l | tr -d ' ')"
[[ "$register_count" == "140" ]] || fail "decision register does not contain 140 decisions"

for number in $(seq 1 140); do
  decision="$(printf 'EQM-%03d' "$number")"
  [[ "$(rg -c "^\\| ${decision} \\|" docs/specification/decision-register.md)" == "1" ]] ||
    fail "decision register entry is missing or duplicated: $decision"
done

checksum_count="$(wc -l <"$checksums" | tr -d ' ')"
(cd "$repository_root" && shasum -a 256 -c "$checksums" >/dev/null) ||
  fail "authority checksum mismatch"

required_links=(
  product.md
  architecture.md
  decision-register.md
  manifest-contracts.md
  vocabularies.md
  schema-inventory.md
  canonicalization.md
  evaluation.md
  protocol.md
  cli.md
  security-and-limits.md
  acceptance.md
  naming-and-no-compat.md
  provenance.md
)
for link in "${required_links[@]}"; do
  rg -q "\`${link}\`" docs/specification/README.md || fail "README omits $link"
  [[ -f "docs/specification/$link" ]] || fail "README target missing: $link"
done

printf '%s\n' \
  'authority-validator schema=1' \
  "authority-checksums=${checksum_count}" \
  'decisions=140' \
  'rcl-checkpoints=134' \
  'traceability-rows=140' \
  'status=ok'
