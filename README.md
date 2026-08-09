# EquivalenceMatrix

EquivalenceMatrix (`eqm`) is a local-first product-conformance graph for teams
that ship the same product behavior across multiple application targets. It
turns shared intent, target mappings, policy, and independently produced
evidence into deterministic answers about conformance and equivalence.

EQM does not generate application code or require targets to share an
implementation. An Android app can remain Jetpack Compose, an iOS app can
remain SwiftUI, and both can be evaluated against the same behavioral contract.

```text
authored contracts + target bindings + policy + evidence
                         |
                         v
             conformance and equivalence results
```

## Start here

- [Documentation map](docs/README.md)
- [Getting started](docs/getting-started.md)
- [Android and iOS integration](docs/integrations/android-ios.md)
- [Agent context](docs/agent-context.md)
- [Complete example](examples/android-ios/README.md)

The public JSON Schema namespace is rooted at:

```text
https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/
```

Schemas are committed with the source and validation is local; schema URIs are
stable identities, not a requirement for runtime network access.

## Repository layout

```text
crates/                 production Rust crates
docs/                   integration and usage documentation
examples/android-ios/   complete consumer workspace example
schemas/v1/             versioned manifest and protocol schemas
tests/                   cross-crate fixtures and security cases
tools/                   xtask, benchmarks, and fuzz targets
```

## Contributing

Install the toolchain pinned by `rust-toolchain.toml`, then run `cargo xtask
check`. The extended release-candidate gate is `cargo xtask verify`. See
[CONTRIBUTING.md](CONTRIBUTING.md) for the contributor contract.

## License

Licensed under either Apache-2.0 or MIT, at your option.
