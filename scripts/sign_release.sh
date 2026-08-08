#!/usr/bin/env bash
set -euo pipefail

signer="${1:?explicit signer executable required}"
artifact="${2:?artifact required}"
case "$signer" in /*) ;; *) echo "signer must be an explicit absolute executable" >&2; exit 2;; esac
test -x "$signer"
test -f "$artifact"
exec "$signer" "$artifact"
