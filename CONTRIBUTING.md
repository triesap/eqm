# Contributing

EquivalenceMatrix is a Rust workspace with current-only v1 schemas. Install the
toolchain pinned by `rust-toolchain.toml`, clone with Git metadata available,
and run `cargo extbuild doctor` where the external build router is installed.
Repository build, dependency, generation, test, benchmark, fuzz, and packaging
commands must use that router on configured development machines.

Before editing, read `AGENTS.md`, the relevant specification in
`docs/specification/`, and the owning crate tests. Keep changes focused and
standalone. Public content must not contain workstation paths, credentials,
private coordination references, generated `.eqm` state, or local `target`
output. Unsafe Rust and compatibility aliases/readers are forbidden.

For ordinary changes run:

```text
cargo extbuild run -- bash scripts/verify.sh
```

The lane checks authority, security coverage, generated schemas, URI parity,
formatting, locked compilation/tests, clippy with warnings denied, docs,
generated-state cleanliness, and diff whitespace. Schema changes must update
the authoritative specification/decision record, DTO/domain boundary,
generators, checked artifacts, examples, positive/negative tests, and schema
parity together. Never hand-edit generated schema JSON.

Dependency updates must be exact, justified, reflected in `Cargo.lock` and any
isolated fuzz/benchmark lockfiles, and pass advisory/license/SBOM review.
Commits should be small, verified, and explain one coherent behavior. Pull
requests must state commands run, skipped lanes, security/schema impact,
generated-file changes, and remaining risk.

Releases follow `docs/release-packaging.md`. Dry-run packaging does not publish
or sign. Versioning, maintenance, deprecation policy, and production
prerequisites are documented in `docs/maintenance.md`. Contributions are
licensed under MIT OR Apache-2.0.
