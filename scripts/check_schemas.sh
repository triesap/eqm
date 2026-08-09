#!/usr/bin/env bash
set -euo pipefail

temporary_root="$(mktemp -d)"
trap 'rm -rf "${temporary_root}"' EXIT
bash scripts/generate_schemas.sh "${temporary_root}/schemas/v1"
diff -ru schemas/v1 "${temporary_root}/schemas/v1"
