#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"

if [[ -e .eqm ]]; then
  echo "error: generated .eqm state must not exist during repository verification" >&2
  exit 1
fi

if [[ -e target ]]; then
  echo "error: repository-local target output detected; use an external Cargo target" >&2
  exit 1
fi

if [[ -n "$(git ls-files '.eqm/**' 'target/**')" ]]; then
  echo "error: generated state is tracked by Git" >&2
  exit 1
fi
