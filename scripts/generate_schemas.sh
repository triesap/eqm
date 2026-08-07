#!/usr/bin/env bash
set -euo pipefail

output_root="${1:-schemas}"
cargo run --quiet --locked -p eqm_manifest --bin generate_manifest_schemas -- "${output_root}/manifest"
cargo run --quiet --locked -p eqm_protocol --bin generate_protocol_schemas -- "${output_root}/protocol"
