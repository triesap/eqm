#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
known="$(mktemp)"
trap 'rm -f "$known"' EXIT

find "$repository_root/schemas" -type f -name '*.schema.json' -print0 \
  | sort -z \
  | xargs -0 -n1 jq -r '."$id"' > "$known"
test "$(wc -l < "$known" | tr -d ' ')" -eq 22
test "$(sort -u "$known" | wc -l | tr -d ' ')" -eq 22

uris="$(rg --no-filename -o 'https://raw\.githubusercontent\.com/triesap/eqm/master/schemas/v1/(manifest|protocol)/[a-z-]+\.schema\.json' \
  "$repository_root/examples/android-ios/eqm.toml" \
  "$repository_root/examples/android-ios/eqm" \
  "$repository_root/tests/fixtures/signup" | sort -u)"
while IFS= read -r uri; do
  test -z "$uri" || grep -Fxq "$uri" "$known"
done <<< "$uris"

if grep -Fxq 'https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/not-current.schema.json' "$known"; then
  echo "invalid schema URI accepted" >&2
  exit 1
fi
echo "schema-parity schemas=22 status=ok"
