#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"
cargo test -p eqm --locked \
  renderer::tests::reviewed_signup_goldens_cover_the_public_surface_and_are_byte_stable
cargo test -p eqm --locked commands::mcp::tests::stdio_handshake_lists_and_calls_are_json_only
cargo test -p eqm --locked \
  commands::release_check::tests::parsed_release_cli_exercises_pass_fail_and_unknown_with_exact_inputs
cargo test -p eqm_runner --test signup_fixture --locked
echo "end-to-end: public CLI, MCP stdio, release 0/1/7, and three-target fixture passed"
