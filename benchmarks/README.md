# Deterministic scale probe

Run `cargo run --release --manifest-path benchmarks/Cargo.toml --locked` on an
8-core reference-class machine. The probe constructs 10,000 stable unit keys
and 100,000 requirement coordinates, measures cold construction/validation,
warm context lookup, conservative affected expansion, canonical serialization,
and an upper-bound memory estimate, then fails when a v1 threshold is exceeded.

This is a deterministic data-plane regression probe, not an application or
network benchmark. Raw timings are environment-specific and are not committed.
