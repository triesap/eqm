#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"

scripts/check_no_legacy_names.sh

if EQM_NO_LEGACY_INCLUDE_NEGATIVE=1 scripts/check_no_legacy_names.sh >/dev/null 2>&1; then
  echo "error: negative compatibility fixture was not rejected" >&2
  exit 1
fi
