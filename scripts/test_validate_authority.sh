#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"

first="$(mktemp)"
second="$(mktemp)"
cleanup() {
  rm -f "$first" "$second"
}
trap cleanup EXIT

bash scripts/validate_authority.sh >"$first"
bash scripts/validate_authority.sh >"$second"
cmp "$first" "$second"
cat "$first"
