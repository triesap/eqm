# Repository baseline and safe discovery

Baseline repository: `https://github.com/triesap/eqm`. Default branch: `master`. Head rechecked during this handoff: `7c8293837c09c627489cdf244a2fbd97b60aa8ed` (2026-08-09T17:30:53Z). Handoff prepared 2026-09-26. No remote files were modified and no Rust/native tests were executed while assembling this package.

## Observed structure

Rust 2024 workspace, resolver 3, package version 0.1.0, pinned rust-version/toolchain 1.97.1. Production crates: `eqm_domain`, `eqm_engine`, `eqm_manifest`, `eqm_protocol`, `eqm_runner`, `eqm_mcp`, plus CLI directory `crates/eqm_cli` whose Cargo package is **eqm**. Support: `eqm_test_support`, `tools/xtask` package `xtask`, benchmark tooling, excluded `tools/fuzz`. Re-read Cargo metadata before using package or feature names after repository changes.

Workspace lint policy forbids unsafe code and denies production dbg/expect/panic/todo/unimplemented/unwrap via Clippy; missing docs are warnings escalated by the standard gate. Production dependencies are exact and Cargo.lock is committed. Do not silently downgrade the toolchain or float dependencies. Author/license metadata remains MIT OR Apache-2.0.

## Commit convention

Recent subjects include `build: harden extended verification lanes`, `chore: enforce the public repository contract`, `docs: publish the canonical integration guide`, and `build: replace shell automation with native xtask`. Follow **type: imperative subject**, lowercase type, no mandatory scope, with an optional explanatory body. Use feat/fix/refactor/test/docs/build/chore as appropriate. The plan supplies a message per step; breaking compatibility is documented in the body and version ADR rather than guessed suffix conventions.

## Baseline gates and conflicts

Standard `cargo xtask check` validates public repository inventory, generated-state rules, legacy-name restrictions, generated schema parity, formatting, locked workspace compilation/tests, Clippy, Rust docs, end-to-end behavior and diff whitespace. `cargo xtask verify` adds security, dependency audit/policy, coverage, mutation, fuzz, benchmarks and reproducible package validation. Dependency tools/nightly availability must be discovered before declaring those lanes runnable.

The public-repository check forbids `.github` and `scripts`, enforces an exact public documentation inventory, and scans generic private-marker strings including “handoff.” Therefore do not simply unpack this bundle into the checkout before S001. The approved spec permits necessary documentation/governance improvements, but not wholesale removal of useful secret/private-data checks. S001 narrowly admits a versioned spec subtree and replaces generic-word blocking with meaningful scoped checks. Actual workflow-host changes remain an explicit later decision; a GitHub URL does not imply GitHub Actions is selected.

Existing instructions prohibit unsafe Rust, legacy readers and implicit network. This spec permits only explicit, separately reviewed native/OS exceptions and explicit versioned migrations; do not treat it as blanket relaxation. Root AGENTS.md must be merged, never blindly replaced by the template in this package.

## Provenance and scope of observations

The prior review was static and identified code paths and architectural gaps, not a reproduced exploit report. The repository's tests, coverage, benchmarks, audit and release readiness were not rerun in this conversation. Absence of observed Actions runs is not proof that the project has no external CI. Revalidate findings at actual HEAD; record resolved/obsolete findings with evidence before changing the plan.

Key references: [AGENTS.md](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/AGENTS.md), [CONTRIBUTING.md](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/CONTRIBUTING.md), [Cargo.toml](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/Cargo.toml), [CLI manifest](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/Cargo.toml), [toolchain](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/rust-toolchain.toml), [agent context](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/docs/agent-context.md), [xtask](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/tools/xtask/src/main.rs).

## Full-source review and baseline execution — 2026-09-26

The follow-up review covered all 132 Rust source files (40,196 lines), associated manifests/schemas/docs/examples/fixtures and the full handoff. At the same pinned HEAD, the standard cargo xtask check passed 258 tests and its formatting/check/Clippy/schema/docs/integration gates. Metadata/help checks passed. This evidence predates refactor implementation; it does not close any new acceptance case. The full extended cargo xtask verify lane and actual native/product/crypto/operational acceptance were not run. Recheck prerequisites before execution.

No local or remote Git tags were found during the scoped compatibility inspection; no EQM dependency consumers were found in the scoped local manifest/lock/Gradle search. That is not proof that no external/published consumer exists. The approved new-namespace direction remains subject to S006's exact documented consumer transition.
