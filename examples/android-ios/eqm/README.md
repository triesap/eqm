# Authored EquivalenceMatrix Metadata

This directory contains source-controlled EquivalenceMatrix definitions for
this workspace. Generated evidence, inventories, caches, and temporary state
belong under the ignored `.eqm/` directory instead.

The root `eqm.toml` declares every source class explicitly. Authored TOML files
must use exact current-v1 schema identifiers and must not contain secrets,
environment interpolation, commands, or compatibility fields.

Subdirectories:

- `contracts/`: capabilities, journeys, surfaces, and fragments;
- `bindings/`: target-specific artifacts, exposure, and evidence declarations;
- `policies/`: obligation, assurance, trust, and waiver rules;
- `profiles/`: declared finite evaluation dimensions and profile inputs;
- `runners/`: approved bounded runner definitions;
- `waivers/`: visible approved expiring exceptions.
