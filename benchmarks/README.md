# Deterministic scale probe

Run `cargo run --release --manifest-path benchmarks/Cargo.toml --locked` on an
8-core reference-class machine with POSIX `ps`. The probe materializes a
workspace configuration, lockfile, and exactly 10,000 discovered current-schema
authority documents containing 10,000 semantic units and 100,000 requirements.
Its worker then uses the production manifest loader, graph
resolution, invariant/fragment finalization, canonicalization, MCP context
resource, and affected-set analyzer. A supervisor samples the worker's actual
resident memory through `ps`; no allocation multiplier or memory estimate is
used. The command fails when any v1 time or memory threshold is exceeded.

This is a deterministic local production-path regression probe, not a network
benchmark. Raw timings are environment-specific and are not committed.
