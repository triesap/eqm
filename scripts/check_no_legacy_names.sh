#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"

legacy_product='Feature''Matrix'
legacy_cli='fm''tx'
legacy_upper='FM''TX'
legacy_hidden='\.fm''tx'
legacy_config='fm''tx\.toml'
forbidden_pattern="${legacy_product}|${legacy_cli}|${legacy_upper}|${legacy_hidden}|${legacy_config}"

scan_args=(
  --hidden
  --line-number
  --glob '!.git/**'
  --glob '!target/**'
  --glob '!scripts/check_no_legacy_names.sh'
  --glob '!docs/specification/naming-and-no-compat.md'
)

if [[ "${EQM_NO_LEGACY_INCLUDE_NEGATIVE:-0}" != "1" ]]; then
  scan_args+=(--glob '!tests/fixtures/no_legacy/negative/**')
fi

if rg "${scan_args[@]}" "$forbidden_pattern" .; then
  echo "error: forbidden compatibility identifier detected" >&2
  exit 1
fi
