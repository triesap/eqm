#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
package_root="$(mktemp -d)"
trap 'find "$package_root" -depth -delete' EXIT
first="$package_root/first"
second="$package_root/second"
mkdir -p "$first" "$second"

cd "$repository_root"
first_archive="$(bash scripts/package_release.sh "$first")"
second_archive="$(bash scripts/package_release.sh "$second")"
cmp "$first_archive" "$second_archive"
test "$(cut -d ' ' -f1 "$first_archive.sha256")" = \
  "$(cut -d ' ' -f1 "$second_archive.sha256")"

listing="$package_root/listing"
tar -tzf "$first_archive" | LC_ALL=C sort > "$listing"
test "$(wc -l < "$listing" | tr -d ' ')" -eq 27
test "$(sort "$listing" | uniq -d | wc -l | tr -d ' ')" -eq 0
archive_root="$(sed -n '1s#/.*##p' "$listing")"
tar -xOf "$first_archive" "$archive_root/SBOM.spdx.json" \
  | jq -e '.spdxVersion == "SPDX-2.3" and .name == "eqm" and (.packages | length > 0)' >/dev/null
tar -xOf "$first_archive" "$archive_root/provenance-inputs.json" \
  | jq -e '.builder == "local-dry-run" and .production_signature == false and (.source_commit | length == 40) and (.cargo_lock_sha256 | length == 64)' >/dev/null
(cd "$first" && shasum -a 256 -c "$(basename "$first_archive").sha256")
bash scripts/check_no_legacy_names.sh "$package_root"
echo "package: two byte-identical 27-file archives with valid SBOM and provenance inputs"
