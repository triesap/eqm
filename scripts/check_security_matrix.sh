#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
matrix="$repository_root/tests/security/adversarial-cases.tsv"

test "$(sed -n '1p' "$matrix")" = $'case\tsource\ttest'
count=0
while IFS=$'\t' read -r case_name source test_name; do
    test -n "$case_name"
    test -f "$repository_root/$source"
    rg -q "fn ${test_name}[(]" "$repository_root/$source"
    count=$((count + 1))
done < <(sed -n '2,$p' "$matrix")
test "$count" -eq 12
echo "security-matrix cases=$count status=ok"
