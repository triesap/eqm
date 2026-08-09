# Contributing

EquivalenceMatrix is a Rust 2024 workspace with current-only v1 contracts.
Install the toolchain pinned by `rust-toolchain.toml` and keep `Cargo.lock`
committed. Normal Cargo target directories are ignored; generated EQM consumer
state under `/.eqm/` must never be committed.

Before changing behavior, read [docs/agent-context.md](docs/agent-context.md),
the relevant usage document, the owning crate, and its tests. Preserve crate
boundaries and deterministic ordering. Do not add unsafe Rust, shell command
strings, floating production dependencies, legacy readers, schema aliases, or
implicit network acquisition.

The standard contributor gate is:

```text
cargo xtask check
```

It verifies generated schemas and URI parity, formatting, locked compilation,
all workspace tests, Clippy with warnings denied, Rust documentation,
end-to-end behavior, generated-state boundaries, and diff whitespace.

The release-candidate gate is:

```text
cargo xtask verify
```

It adds the adversarial security matrix, dependency audit and policy checks,
coverage and mutation thresholds, bounded fuzzing, the production-scale
benchmark, and reproducible distribution verification. These extended lanes
require the Cargo subcommands named by their output.

Never hand-edit generated schema JSON. Use `cargo xtask schemas generate`,
review source and generated changes together, and run `cargo xtask schemas
check`. Dependency changes must be exact, justified, locked, and pass the
supply-chain lanes. Pull requests should state the commands run, skipped lanes,
schema or security impact, generated changes, and remaining risk.

Distribution archives are local, unsigned build outputs. `cargo xtask dist
OUTPUT_DIRECTORY` produces a deterministic archive, checksum, SPDX package
inventory, and provenance inputs; it does not publish or sign anything.

Contributions are licensed under MIT OR Apache-2.0.
