#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
nightly="nightly-2026-07-16"
campaign_root="$(mktemp -d)"
trap 'find "$campaign_root" -depth -delete' EXIT

cd "$repository_root"
command -v cargo-fuzz >/dev/null
for target in toml protocol adapter inventory evidence canonicalization graph; do
  mkdir -p "$campaign_root/$target/corpus" "$campaign_root/$target/artifacts"
  cargo "+$nightly" fuzz run "$target" "$campaign_root/$target/corpus" -- \
    -runs=1000 -timeout=10 -artifact_prefix="$campaign_root/$target/artifacts/"
done
echo "fuzz smoke: 7 production targets x 1000 runs passed"
