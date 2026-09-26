# Verification contract

## What was and was not verified in this handoff

Repository metadata/instructions/manifests and relevant primary documentation were read. The handoff artifact is checked for internal consistency, traceability and ZIP integrity. **No EQM Rust compilation, unit tests, fuzzing, dependency audit, native tests, benchmarks, signing, release authorization, deployment or source modification was performed while preparing the package.** Every application acceptance status begins not_run.

## Preflight commands (read first, then execute with authorized tooling)

```sh
git remote -v
git branch --show-current
git rev-parse HEAD
git status --short
git log -6 --format=%s
rustup show active-toolchain
cargo metadata --no-deps --format-version 1 --locked
cargo xtask help
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
```

Inspect existing AGENTS/CONTRIBUTING/xtask before commands. Do not execute arbitrary discovered hooks/adapters or install tools/network dependencies without environment permission. Missing pinned toolchain/tool/subcommands are explicit blockers. Do not silently downgrade Rust or drop --locked. Keep the extracted package outside the repo until S001 admits the spec subtree.

## Per-step required gate

Every numbered step lists exact baseline Rust commands and owning package tests. `cargo xtask check` is the standard full-workspace gate. Direct targeted tests permit fast iteration; the full standard gate still passes before committing. Commands duplicated by xtask can use Cargo incremental caches, but do not invent execution evidence or replace full checks with inferred success.

Run formatting, workspace cargo check, owning package cargo test, workspace Clippy, and cargo xtask check plus git diff --check. Run changed doctests and interface/integration tests when relevant. A Cargo dependency/manifest change intentionally updates Cargo.lock in that same step using the correct scoped Cargo resolution command; then every verification command runs locked. Record the update command and reason rather than blindly running cargo update.

Schema changes: edit source DTOs/generators; execute `cargo xtask schemas generate`; review source/generated diffs together; run `cargo xtask schemas check` and the standard parity gate. Under a new approved wire namespace, adjust generators and parity rules explicitly in the same logical transition. Do not hand-edit generated JSON.

## Known package names

`eqm` = crates/eqm_cli; `eqm_domain`, `eqm_engine`, `eqm_manifest`, `eqm_protocol`, `eqm_runner`, `eqm_mcp`, `eqm_test_support` are workspace crates; `xtask` = tools/xtask. `eqm_application` is new and may be used only from its creation step onward. Discover the benchmark package and actual feature names from metadata before targeting them. Paths in the plan are likely modules: reconcile canonical.rs versus canonical/ and test layouts against HEAD before editing.

## Extended lanes

```sh
cargo test --workspace --doc --locked
cargo xtask schemas check
cargo xtask test security
cargo xtask test coverage
cargo xtask test mutation
cargo xtask test fuzz
cargo xtask benchmark
cargo xtask test package
cargo xtask verify
```

`verify` includes the repository's current supply-chain audit/policy, coverage, mutation, fuzz, benchmark and packaging routines. Inspect tools/xtask source/help for actual prerequisites and pinned nightly/subcommand versions. Never raise thresholds, delete adversarial tests, or ignore surviving critical mutants just to obtain a green run. Fuzz and mutation tests remain bounded and resource-controlled. Do not add shell automation when native xtask can own the lane.

Feature matrix: use `cargo metadata` and manifests to enumerate valid default/no-default/explicit feature combinations. Run matching check/test/clippy/doc lanes. Use --all-features only if combinations are valid; feature removal must not weaken mandatory validation. Native OS guarantees require native execution tests. Cross-compilation is additional evidence, not proof of Windows Job Object or filesystem durability behavior.

## Unknown native/product commands

Q01/Q02 specify actual repos and targets. Before adapter/native steps, read applicable repo instructions, Gradle settings/build scripts and task help, Xcode scheme/test plans where used, and product Rust manifests. Record exact commands for: desktop build/test/smoke, mobile device/simulator integration, provider fixtures, cross-runtime scenario orchestration, native packaging and optional bridge tests. Do not invent `./gradlew` tasks, Xcode schemes, emulator IDs or product selectors. Test in authorized disposable fixtures, not live accounts/data.

## Baseline failures and next-step admission

A newly introduced failure blocks the next step and no knowingly broken code commit is accepted. For a reproduced pre-existing/out-of-scope failure, record baseline and post-change commands, identical failure evidence, lack of causal regression, owner/issue if available and explicit safe-to-proceed rationale. This exception never labels a failure passed. Required security/crypto/host/product acceptance cannot be waived by calling it an environment issue; that readiness claim stays blocked. When a step cannot be verified safely within its scope, stop at the boundary and document rather than broadening silently.

## Review and report

Inspect the staged diff for unrelated edits, trust bypasses, hidden effects, new public API/compatibility changes, unsafe scope, secrets, unbounded inputs, count inflation, omitted obligations, and tests asserting placeholders. Record exact commands with statuses and evidence paths. See STEP_REPORT_TEMPLATE.md. Update the progress ledger in the same logical commit; report its SHA externally after commit (or in the next ledger update) to avoid self-referential amend loops.

## Final gate

All mandatory acceptance cases need executable evidence on their declared matrix. `cargo xtask check`, `cargo xtask verify`, applicable doc/feature/native/product/migration/crypto/distribution lanes and package traceability must pass. Independently review shadow release decisions. Operational authority requires separate explicit owner activation; never publish/deploy as a verification side effect.

## Handoff artifact validation

From the extracted package run `python3 tools/validate_package.py`. After specs/progress evolve during implementation, use `python3 docs/eqm_product_v1/tools/validate_package.py --structure-only` for structural checks; original SHA256SUMS certifies the delivered package only and intentionally changes when edited. Reissue integrity hashes only as a clearly new package revision, not to conceal edits. The validator does not run Rust tests and must not be counted as product acceptance evidence.

## Revision 2 plan validation

The plan revision records approved direction, 15 execution cycles, 211 pending steps and 21 review regression clauses. Validate cycle partition/order, regression/requirement/acceptance/step bidirectional mappings, synchronized Markdown/JSON and reconstruction in addition to existing package checks. Package validation remains distinct from EQM implementation acceptance.

Previous baseline review at the pinned commit ran cargo xtask check successfully (258 tests), cargo metadata --no-deps --format-version 1 --locked and cargo xtask help. This is prior baseline evidence, not a new run or evidence for the refactor. Full extended verify and product/native/crypto/operational acceptance remain unrun; the pinned nightly prerequisite was unavailable during baseline inspection. Revalidate environment and HEAD before S001.

## DEV-002 targeted verification

S015-R1–R5 additionally require application doctests, because all-targets Rust
tests do not execute compile-fail documentation examples. Validate every
applicable numeric/time/absence vector, v1 accepted endpoints and unchanged
artifacts, and the actual versioned schemas/runtime boundaries. Full gate output
counts can include repeated integration invocations; do not present their sum
as a count of distinct tests. The standard gate is not release acceptance.

Human release testing begins only after the corresponding implementation and
external prerequisites exist. The candidate/testing sequence is recorded in
[the supervisor review](reports/CODEX-3da0b50.md#release-testing-disposition).
