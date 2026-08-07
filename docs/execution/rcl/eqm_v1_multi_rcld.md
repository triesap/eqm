# EquivalenceMatrix V1 Full Implementation Multi-RCLD

Status: in progress; RCLDs 00-02 complete; RCLD 03 active
Created: 2026-08-07
Updated: 2026-08-07
Mode: rcl-durable
Repository: `triesap/eqm`
Cargo workspace root: repository root
Planning base commit: `859205c`
Current child RCLD: `03`

## Purpose

Implement the complete EquivalenceMatrix v1 product as a standalone public
Rust workspace. The result is a deterministic, local-first product-conformance
graph, library, CLI, and thin MCP adapter for evaluating policy-relative
semantic equivalence across independently implemented targets.

This program preserves all 134 approved functional checkpoints while
correcting the repository bootstrap, dependency ordering, crate ownership,
canonicalization integration, mutability language, schema closure, and final
verification defects identified before implementation.

An original checkpoint may be split into ordered corrective subcheckpoints
when the approved scope would otherwise be too large for a reviewable green
commit. A split may not merge, omit, reorder, or weaken the original functional
milestone. RCLD 00 records the required Step 007 authority split.

Only one child RCLD and one checkpoint may be active at a time. A checkpoint
is committed only when the active execution directive authorizes commits.

## Repository Boundary

All specifications, RCLD documents, Rust code, schemas, fixtures, generated
goldens, verification scripts, and commits are rooted in this repository.

Public content must be standalone. It must not name a containing workspace,
private coordination document, workstation-specific path, or parent-local
tool. A containing repository's submodule pointer is a separate repository
change and is not part of an EQM source checkpoint.

No checkpoint authorizes a push, pull request, package publication, schema
publication, release, deployment, credential change, or remote mutation.

## Authority Order

Apply authority in this order during implementation:

1. repository-owned product and acceptance specifications;
2. repository-owned architecture, data, API, manifest, canonicalization,
   evaluation, security, trust, and resource contracts;
3. approved ADRs and the v1 decision register;
4. repository-local `AGENTS.md` and contribution/security policy;
5. this governing multi-RCLD and the currently active child RCLD;
6. the corrected repository-owned commit sequence;
7. implementation, tests, generated schemas, fixtures, and verification
   evidence.

If two higher authorities conflict, stop at the active checkpoint and record a
new ADR before implementing behavior. Do not resolve public semantics only in
code or tests.

## Canonical Product Identity

- Product: `EquivalenceMatrix`
- CLI and install package: `eqm`
- Root config: `eqm.toml`
- Authored metadata: `eqm/`
- Generated local state: `.eqm/`
- Rust crate and directory prefix: `eqm_*`
- Diagnostic prefix: `EQM-`
- Environment prefix: `EQM_`
- MCP URI scheme: `eqm://`
- Schema namespace: `https://schemas.equivalencematrix.dev/v1/`

There are no legacy aliases, compatibility readers, deprecated keys, migration
commands, fallback schemas, protocol negotiation fallbacks, ID redirects, or
application-code generators.

## Approved Repository Adaptations

The following corrections are normative for this implementation program.

### Baseline and workspace

- Before a valid Cargo workspace exists, documentation/static checks and
  `git diff --check` are the green lane.
- Full locked Cargo verification starts when Step 003 creates the crates.
- Workspace members are the eight explicit approved crate paths, not a glob.
- Cargo resolver 3, Rust 1.97.1, Edition 2024, and a committed `Cargo.lock` are
  required.
- `eqm_test_support` is unpublished; the CLI directory is `eqm_cli` while its
  Cargo package and binary are `eqm`.
- `.eqm/` is ignored; `Cargo.lock` is not ignored.
- Unrelated web/WASM profiles and UI bootstrap language are removed.

### Domain checkpoint dependency correction

Steps 019 through 021 retain their checkpoint numbers but use this dependency
order:

1. Step 019 defines requirements.
2. Step 020 defines fragments.
3. Step 021 defines surfaces.

### Crate ownership correction

The approved dependency graph is:

```text
eqm_domain

eqm_manifest -> eqm_domain
eqm_engine   -> eqm_domain
eqm_protocol -> eqm_domain
eqm_runner   -> eqm_domain + eqm_protocol

eqm_mcp      -> eqm_engine + eqm_protocol + eqm_runner
eqm_cli      -> all production crates
```

Authored TOML DTOs belong to `eqm_manifest`. `eqm_protocol` owns public JSON,
SARIF, adapter, evidence, attestation, report, and MCP DTOs. Step 035 is adapted
accordingly. Schema identity primitives shared by both crates belong to
`eqm_domain`.

### Canonicalization integration

`eqm_manifest` exposes a pure canonicalizer over a finalized domain
`WorkspaceGraph`. It does not call `eqm_engine`. The orchestration sequence is:

```text
parse
-> strict schema validation
-> normalized domain inputs
-> explicit defaults
-> graph resolution
-> exact-pinned fragment expansion
-> invariant validation
-> normative semantic projection
-> deterministic ordering
-> RFC 8785 JCS
-> SHA-256
```

Step 052 defines the canonicalizer. Steps 057 through 059 create the finalized
graph and prove that only that graph is digestible. CLI orchestration prepares
the same workspace session used by CLI commands and the MCP adapter.

### Mutability correction

- `init`, `new`, `fmt`, and `lock update` are the only commands that mutate
  authored EQM metadata.
- `verify` may atomically write immutable generated results below
  `.eqm/results/`.
- `discover` does not persist results unless the user explicitly requests a
  generated-state output.
- `attest` writes stdout unless an output path is explicit.
- Any explicit `--output` write is atomic.
- Read-only commands never update authored files, locks, caches, evidence, or
  inventories implicitly.

### Contract hardening

Before semantic code, repository-owned authority must define closed manifest
field tables, vocabulary and selector rules, public protocol DTOs, CLI
semantics, canonicalization, evaluation truth tables, diagnostic allocation,
trust and attestation inputs, and hard resource limits. Unknown behavior must
not be invented in later code checkpoints.

### Example and compatibility enforcement

- Repository examples are valid positive examples with real digests.
- Deliberately invalid inputs live only in named negative-fixture paths.
- The no-legacy scan covers production source, schemas, normal docs, examples,
  fixtures, CI, packaging, and release artifacts.
- Narrow exceptions are limited to the rejection specification, scanner data,
  and explicit negative fixtures; those paths are still checked against
  executable compatibility behavior.
- Final schema verification is mandatory and may not suppress failure.

## Non-Negotiable Architecture

- `eqm_domain` is I/O-, process-, network-, Git-, terminal-, MCP-, and
  CLI-independent.
- The core engine is synchronous, pure, deterministic, and panic-free for
  user-controlled input.
- Domain objects and public protocol DTOs remain separate.
- First-party crates forbid unsafe code.
- Collections are deterministic or explicitly sorted at every public boundary.
- Bindings contain approved runner IDs and typed selectors, never shell
  strings.
- The runner crate is the only production crate that launches processes.
- Adapters are out of process and digest-pinned.
- Unknown, stale, missing, failed, unstable, or insufficiently trusted required
  evidence never succeeds.
- A waiver is visible debt and never converts evidence to satisfied.
- Target conformance is evaluated before target-set equivalence.
- Expected, declared, discovered, enabled, released, and conformant remain
  independent facts.

## Organizational Gates

Core implementation may use clearly synthetic fixture authorities. Production
release claims remain blocked until the repository has real values for:

- package, repository, and schema namespace ownership;
- product-contract, architecture, runner/security, waiver, and release owners;
- trusted CI identity, signing policy, and trust roots;
- protected baseline branch and exact resolution rules;
- real pilot roots and build commands;
- evidence-retention policy;
- legal and product-name clearance.

Synthetic examples must never be represented as production authority.

## RCL Execution Contract

For every checkpoint:

1. inspect current authority, repository identity, and working-tree state;
2. state the exact scope, requirements, files, definition of green, and verify
   lane;
3. implement the smallest coherent change;
4. run the narrowest credible verification and inspect generated artifacts;
5. review the complete diff for naming, compatibility, determinism, panic
   safety, security, diagnostics, and scope;
6. repair, split, or block a red checkpoint; never commit it;
7. commit only when the active execution directive authorizes a commit;
8. record the checkpoint result and reconcile every remaining checkpoint.

Checkpoint reports use:

```text
Step:
Commit:
Purpose:
Files changed:
Requirements covered:
Tests added or changed:
Commands run:
Results:
Self-review findings:
Unverified items:
Deviations:
Next-step safety:
```

`Next-step safety` is `safe`, `blocked`, or
`safe with documented pre-existing issue`.

## Standard Verification Lanes

Before Step 003:

```sh
git diff --check
```

From Step 003 onward:

```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo doc --workspace --no-deps --locked
git diff --check
```

Repository-owned schema, deterministic-output, no-legacy, security,
conformance, fuzz, performance, package, and release lanes become mandatory at
the checkpoints that introduce them. A final gate cannot contain an
error-suppressing fallback.

## Ordered Child RCLD Graph

| Order | Child RCLD | Steps | Dominant completion proof |
| --- | --- | --- | --- |
| 00 | Authority and bootstrap | 001-008 | Standalone closed authority, locked workspace, baseline CI |
| 01 | Diagnostics and primitives | 009-016 | Strict validated foundational types |
| 02 | Typed domain graph | 017-032 | Complete I/O-free graph model and docs |
| 03 | Protocol and schemas | 033-041 | Closed public DTOs, SARIF, and generated schema parity |
| 04 | Manifest system | 042-056 | Strict spanned loading, conversion, formatting, and examples |
| 05 | Resolution and conformance | 057-070 | Graph, policy, evidence, waiver, conformance, equivalence proof |
| 06 | Exposure and analysis | 071-076 | Release, diff, affected, matrix, and diagnostic explanation proof |
| 07 | Runners and discovery | 077-090 | Injection-safe bounded execution and inventory reconciliation |
| 08 | CLI query surface | 091-103 | Stable output and non-evidence command behavior |
| 09 | Verification and mutation | 104-111 | Evidence, attestation, release, scaffolding, formatting, lock behavior |
| 10 | MCP | 112-116 | Thin current-version stdio adapter with execution denied by default |
| 11 | End-to-end fixtures | 117-122 | Cross-target signup and release-gate golden proof |
| 12 | Hardening and packaging | 123-128 | Properties, fuzz, security, performance, schema, package proof |
| 13 | Operations and closure | 129-134 | Standalone operational docs and complete final acceptance |

Strict dependency chain:

```text
00 -> 01 -> 02 -> 03 -> 04 -> 05 -> 06
   -> 07 -> 08 -> 09 -> 10 -> 11 -> 12 -> 13
```

Child plans:

- `eqm_v1_00_authority_and_bootstrap_rcld.md`
- `eqm_v1_01_diagnostics_and_primitives_rcld.md`
- `eqm_v1_02_typed_domain_graph_rcld.md`
- `eqm_v1_03_protocol_and_schemas_rcld.md`
- `eqm_v1_04_manifest_system_rcld.md`
- `eqm_v1_05_resolution_and_conformance_rcld.md`
- `eqm_v1_06_exposure_and_analysis_rcld.md`
- `eqm_v1_07_runners_and_discovery_rcld.md`
- `eqm_v1_08_cli_query_surface_rcld.md`
- `eqm_v1_09_verification_and_mutation_rcld.md`
- `eqm_v1_10_mcp_rcld.md`
- `eqm_v1_11_end_to_end_fixtures_rcld.md`
- `eqm_v1_12_hardening_and_packaging_rcld.md`
- `eqm_v1_13_operations_and_closure_rcld.md`

## Program Completion

The program is complete only when all child RCLDs are complete, every original
checkpoint or approved adaptation is recorded, all mandatory gates pass, the
worktree is clean, production readiness does not rely on synthetic authority,
and unresolved organizational inputs are reported without an overstated
release claim.
