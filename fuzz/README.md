# Fuzz smoke lane

Run
`cargo +nightly-2026-07-16 fuzz run <target> <temporary-corpus> -- -runs=1000 -timeout=5`
for each target in `Cargo.toml`. The targets call the production manifest parser, protocol DTO
validators, adapter correlation checks, inventory envelope validation,
normalized-evidence reader, canonical digest boundary, and domain graph
constructor. Inputs are bounded by libFuzzer and the graph target additionally
caps accepted frames at 1 MiB and 10,000 constructed units. The checked-in
corpus is intentionally empty; crash artifacts are never accepted as passing
evidence.
