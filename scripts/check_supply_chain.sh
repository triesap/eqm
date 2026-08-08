#!/usr/bin/env bash
set -euo pipefail

cargo audit --deny warnings
cargo deny check
