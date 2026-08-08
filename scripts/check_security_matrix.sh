#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
matrix="$repository_root/tests/security/adversarial-cases.tsv"

test "$(sed -n '1p' "$matrix")" = $'case\tsource\tpackage\ttarget\ttest'
log_root="$(mktemp -d "${TMPDIR:-/tmp}/eqm-security-matrix.XXXXXX")"
trap 'rm -rf "$log_root"' EXIT
count=0
while IFS=$'\t' read -r case_name source package target test_name; do
    test -n "$case_name"
    test -f "$repository_root/$source"
    short_name="${test_name##*::}"
    rg -Fq "fn ${short_name}(" "$repository_root/$source"
    log="$log_root/$case_name.log"
    case "$target" in
        lib)
            cargo test -p "$package" --lib --locked "$test_name" -- --exact --nocapture >"$log" 2>&1
            ;;
        test:*)
            cargo test -p "$package" --test "${target#test:}" --locked "$test_name" -- --exact --nocapture >"$log" 2>&1
            ;;
        bin:*)
            cargo test -p "$package" --bin "${target#bin:}" --locked "$test_name" -- --exact --nocapture >"$log" 2>&1
            ;;
        *)
            echo "unsupported security target: $target" >&2
            exit 1
            ;;
    esac
    rg -Fq "test $test_name ... ok" "$log"
    ! rg -Fq "sensitive-value" "$log"
    ! rg -Fq "secret://vault/token" "$log"
    echo "security-case name=$case_name status=ok"
    count=$((count + 1))
done < <(sed -n '2,$p' "$matrix")
test "$count" -eq 12
echo "security-matrix cases=$count status=ok"
