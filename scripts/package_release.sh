#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output="${1:?usage: package_release.sh OUTPUT_DIRECTORY}"
version="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$repository_root/Cargo.toml" | head -1)"
test -n "$version"
mkdir -p "$output"
output="$(cd "$output" && pwd)"
staging="$(mktemp -d)"
trap 'rm -rf "$staging"' EXIT

cd "$repository_root"
cargo build --release --locked -p eqm
package="eqm-${version}-$(rustc -vV | sed -n 's/^host: //p')"
root="$staging/$package"
mkdir -p "$root/bin" "$root/schemas"
cp "$(cargo metadata --format-version 1 --no-deps | jq -r '.target_directory')/release/eqm" "$root/bin/eqm"
cp -R schemas/. "$root/schemas/"
cp README.md LICENSE-APACHE LICENSE-MIT "$root/"
cargo metadata --format-version 1 --locked \
  | jq -S '{spdxVersion:"SPDX-2.3",name:"eqm",packages:[.packages[]|{name,versionInfo:.version,licenseDeclared:.license}]}' \
  > "$root/SBOM.spdx.json"
jq -nS --arg commit "$(git rev-parse HEAD)" --arg lock "$(shasum -a 256 Cargo.lock | cut -d ' ' -f1)" \
  '{builder:"local-dry-run",source_commit:$commit,cargo_lock_sha256:$lock,production_signature:false}' \
  > "$root/provenance-inputs.json"
bash scripts/check_no_legacy_names.sh "$root"
find "$root" -exec touch -t 202608080000 {} +
(
  cd "$staging"
  find "$package" -type f -print | LC_ALL=C sort | COPYFILE_DISABLE=1 tar -cf "$output/$package.tar" -T -
)
gzip -n -f "$output/$package.tar"
shasum -a 256 "$output/$package.tar.gz" > "$output/$package.tar.gz.sha256"
echo "$output/$package.tar.gz"
