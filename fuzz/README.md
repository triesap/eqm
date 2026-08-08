# Fuzz smoke lane

Run `cargo fuzz run <target> -- -runs=1000 -timeout=5` for each target in
`Cargo.toml`. Inputs are bounded by libFuzzer and the graph target additionally
caps accepted frames at 1 MiB. The checked-in corpus is intentionally empty;
crash artifacts are never accepted as passing evidence.
