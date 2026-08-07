# EquivalenceMatrix V1 Implementation Baseline

Status: recorded
Recorded: 2026-08-07
Repository: `triesap/eqm`
Baseline commit: `9fb2d7a69006a6d894823065594ca682b811d7f7`

## Repository Identity

This repository is the standalone public implementation of EquivalenceMatrix
v1. Its Cargo workspace root is the repository root. The canonical remote is
`git@github.com:triesap/eqm.git`, and the initial implementation branch is
`master`.

Source, specifications, fixtures, generated schemas, verification scripts, and
implementation commits belong to this repository. Changes in a containing
repository, if any, are outside the EQM source history.

## Existing Baseline

Before implementation, the repository contained:

- dual MIT and Apache-2.0 license files;
- a minimal README, changelog, and generic contribution guide;
- a virtual Cargo manifest with no member crates;
- a Rust 1.97.1 toolchain file and Rust 2024 formatter configuration;
- a generic ignore file;
- the approved EQM v1 governing multi-RCLD and child plans.

The repository had no Rust source crate, lockfile, CI workflow, verification
script, root agent instructions, authored EQM metadata, schema, or product
fixture.

## Known Unsupported Cargo State

The baseline Cargo manifest named `crates/*` as a workspace member even though
no crate directories existed. A locked metadata probe failed because Cargo
could not read `crates/*/Cargo.toml`.

Other known bootstrap mismatches were:

- Cargo resolver 2 instead of resolver 3;
- workspace Rust version 1.92.0 instead of 1.97.1;
- `Cargo.lock` ignored even though v1 requires a committed lockfile;
- no `.eqm/` generated-state ignore;
- unrelated web/WASM build-output ignores and release profile;
- generic UI/accessibility contribution text unrelated to EQM.

This is an attributed pre-implementation condition. It does not authorize a
red implementation commit. Step 003 must establish the first supported locked
Cargo workspace and pass the complete Rust gate.

## Verification Baseline

Before Step 003, the supported checkpoint lane is:

```sh
git diff --check
```

From Step 003 onward, every implementation checkpoint uses the locked workspace
format, check, test, Clippy, rustdoc, and diff gates defined by the governing
plan, plus checkpoint-specific verification.

The workstation external-build diagnostic was green when this baseline was
recorded. That workstation mechanism is execution context, not a portable
repository requirement; public repository commands remain ordinary Cargo
commands.

## Commit Convention

The initial repository commit established no detailed message convention. EQM
implementation therefore uses Conventional Commit summaries with an area
scope, followed by a concise four-bullet technical body for implementation
checkpoints.

## Approved Adaptations

The governing multi-RCLD records the approved corrections to:

- bootstrap verification and explicit workspace members;
- Requirement, Fragment, and Surface checkpoint ordering;
- authored manifest DTO ownership;
- finalized-graph canonicalization integration;
- authored versus generated-state mutability;
- executable schema, vocabulary, evaluation, protocol, trust, and limits
  authority;
- valid positive examples and controlled negative fixtures;
- compatibility scanning and mandatory final schema verification.

No further semantic adaptation is implied. A new conflict requires an ADR
before dependent implementation proceeds.
