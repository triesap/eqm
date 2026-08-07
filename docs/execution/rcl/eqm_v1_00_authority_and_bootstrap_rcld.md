# EquivalenceMatrix V1 RCLD 00: Authority And Bootstrap

Status: in progress; Steps 001-005 complete
Created: 2026-08-07
Updated: 2026-08-07
Mode: rcl-durable
Repository: `triesap/eqm`
Base commit: `859205c`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Current checkpoint: `step_006`

## Purpose

Establish complete standalone implementation authority, correct the invalid
bootstrap, close every public contract needed by later code, install repository
governance, and leave a reproducible locked Rust workspace with baseline CI.

## Scope Boundary

This RCLD changes repository policy, specifications, schemas or schema source
contracts, verification scripts, metadata examples, CI, and empty Rust crate
scaffolding. It does not implement product evaluation, runner execution,
adapters, application-code generation, release publication, or production
trust identities.

The approved source package is imported with public provenance and digests.
Provenance may identify the source artifact but must not contain a private
filesystem path or containing-workspace reference.

## Definition Of Green

- Repository-owned authority is complete, checksummed, linked, and standalone.
- All approved review corrections are recorded in ADRs and the corrected
  commit sequence.
- Manifest fields, vocabularies, canonicalization, evaluation tables, protocol
  shapes, CLI semantics, security/trust inputs, diagnostics, and resource
  limits are explicit enough that later code need not invent public behavior.
- Cargo uses explicit members, resolver 3, Rust 1.97.1, Edition 2024, and a
  committed lockfile.
- All eight approved crates build, test, lint, and document successfully.
- The no-legacy scanner proves a positive repository and a negative fixture.
- CI invokes repository-owned scripts and locked Cargo gates.
- Documentation and package metadata contain no unrelated UI/WASM bootstrap
  claims.

## Verification Lane

Before `step_003`:

```sh
git diff --check
```

From `step_003` onward:

```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo doc --workspace --no-deps --locked
git diff --check
```

From `step_005` onward also run the repository no-legacy check. At `step_007`,
run the authority/spec validator twice and require byte-identical output.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_001` | Record repository identity, current files, unsupported Cargo baseline, command surface, commit convention, dirty-state boundary, and approved adaptation in `docs/implementation-notes.md`. | The initial red Cargo state is attributed; public notes are standalone; `git diff --check` passes. | `docs(eqm): record implementation baseline` |
| `step_002` | Add root `AGENTS.md` with authority order, crate boundaries, naming, security, RCL, verification, deviation, and reporting rules. | Instructions link only repository-owned paths and forbid compatibility, codegen, untrusted instructions, and unapproved execution. | `docs(eqm): add repository agent instructions` |
| `step_003` | Replace the invalid workspace with eight explicit crates; use resolver 3, Rust 1.97.1, Edition 2024; create package/binary `eqm`; mark test support unpublished; commit `Cargo.lock`; unignore it; ignore `.eqm/`; remove unrelated profiles. | Metadata resolves with `--locked`; every crate compiles and tests; package names and dependency directions match the approved graph. | `chore(eqm): bootstrap rust workspace` |
| `step_004` | Add toolchain components, inherited workspace lints, crate-level unsafe forbids, public-doc policy, and deterministic baseline conventions. | fmt, check, test, Clippy, and rustdoc gates pass with no warning allowance. | `chore(eqm): pin rust toolchain and lints` |
| `step_005` | Add a repository-owned no-legacy scanner, narrow authority/negative-fixture allowances, positive/negative tests, and local command entry point. CI wiring is deferred to Step 008. | Normal repository scan passes; a synthetic forbidden alias fails; allowlisted prose cannot become executable compatibility behavior. | `test(eqm): add no legacy scan` |
| `step_006` | Add valid `eqm.toml`, explicit authored metadata directories, generated-state ignore rules, current-schema examples, and placeholder-free contract skeletons. | Every positive TOML file parses structurally; `.eqm/` is ignored; no old schema/name appears outside controlled rejection data. | `chore(eqm): add canonical metadata layout` |
| `step_007` | Complete the ordered Step 007 authority subcheckpoints below: import product authority and provenance, add corrective ADRs, close manifest/vocabulary, canonicalization, evaluation, protocol/CLI, security/limits, traceability, and corrected-sequence contracts. | Every Step 007 subcheckpoint is green; authority hashes and links validate; every required entity/command/status has an explicit contract; validator output is deterministic; no parent/private path exists. | Completed by the Step 007 subcheckpoint commits |
| `step_008` | Add baseline CI for format, locked check/test/Clippy/rustdoc, authority validation, no-legacy scanning, and clean generated-state enforcement through repository-owned scripts. | Workflow syntax validates; every local equivalent gate passes; no secret, signing, publication, or release authority is implied. | `ci(eqm): add rust verification pipeline` |

## Contract-Hardening Deliverables

Step 007 must provide repository-owned authority for:

- every authored/imported manifest field and default;
- closed and intentionally extensible vocabularies;
- extension namespace validation and digest exclusion rules;
- deterministic source discovery and duplicate-authority handling;
- exact canonical semantic projection and digest domain;
- obligation, evidence, freshness, trust, waiver, conformance, equivalence,
  exposure, release, diff, and affected-set tables;
- command arguments, defaults, mutability, output, offline behavior, and exit
  precedence;
- every JSON/SARIF/adapter/evidence/attestation/MCP envelope;
- signature, trust-root, subject, replay, redaction, and resource-limit rules;
- stable diagnostic allocation and requirement-to-test traceability.

Lifecycle atomicity or product meaning that cannot be validated mechanically
must be labeled as review policy rather than implemented as a misleading
parser heuristic.

## Step 007 Authority Subcheckpoints

These subcheckpoints are an approved split of the original Step 007 functional
milestone. They execute in order and each leaves independently reviewable
authority.

| Substep | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_007a` | Import product, architecture, data, API, evidence, security, naming, acceptance, decision register, and ADR source authority; add public provenance and checksums. | Required files are complete, standalone, link-valid, and checksum-valid; source and repository identities are distinguished without private paths. | `docs(spec): import approved eqm v1 authority` |
| `step_007b` | Add corrective ADRs for baseline adaptation, Step 019-021 ordering, manifest DTO ownership, canonicalization integration, mutability, example validity, compatibility scanning, and unsuppressed final verification. | Every approved correction has rationale, consequences, affected original steps, and acceptance evidence. | `docs(adr): record implementation corrections` |
| `step_007c` | Define exact manifest field tables, defaults, authority, normative participation, closed vocabularies, extensible IDs, selector dimensions/operators, extension namespace behavior, and schema inventory. | Every authored/imported document and enum has an exhaustive machine-checkable contract; unknown fields and values fail closed. | `docs(spec): close manifest and vocabulary contracts` |
| `step_007d` | Define normalization, finalized-graph projection, ordering, extension treatment, RFC 8785 serialization, SHA-256 domain, and independent canonical vectors. | Normative/nonnormative classification is exhaustive and independent vectors have fixed expected bytes/digests. | `docs(spec): define canonicalization contract` |
| `step_007e` | Define applicability, risk, policy composition, obligation, evidence, freshness, trust, waiver, conformance, equivalence, exposure, release, diff, and affected-set truth tables. | Every input state has one explicit output; unknown never succeeds; waiver never satisfies evidence. | `docs(spec): define evaluation tables` |
| `step_007f` | Define every public JSON/SARIF/adapter/evidence/attestation/MCP shape and every CLI argument, default, mutability, offline, output, dry-run, and exit-precedence rule. | All commands and DTOs have closed field tables and current schema identities with no compatibility surface. | `docs(spec): close protocol and cli contracts` |
| `step_007g` | Define diagnostic allocation, trust roots, algorithms, signed subjects, replay binding, redaction, path/symlink behavior, resource limits, adapter/runner guarantees, and privacy rules. | Security behavior is testable; unsupported guarantees and unconfigured organizational values are explicit. | `docs(spec): close security and limits contracts` |
| `step_007h` | Add machine-readable requirement traceability and deterministic authority validator; publish corrected repository-owned commit sequence incorporating all approved changes. | Every requirement maps to authority and planned tests; validator rejects missing/stale/duplicate references and emits byte-identical output twice. | `test(spec): validate executable authority` |

## Reconciliation Rules

- Keep only the checkpoint named by `Current checkpoint` active.
- Record each green checkpoint and its commit before advancing the pointer.
- Any new semantic decision requires an ADR before dependent code.
- RCLD 01 cannot begin until Step 008 and the complete RCLD 00 wave gate are
  green.

## Checkpoint Ledger

| Step | Status | Commit | Result |
| --- | --- | --- | --- |
| `step_001` | complete | this checkpoint | Repository baseline and supported pre-workspace gate recorded |
| `step_002` | complete | this checkpoint | Repository-local agent authority installed |
| `step_003` | complete | this checkpoint | Explicit locked eight-crate Rust workspace established and fully verified |
| `step_004` | complete | this checkpoint | Toolchain components, shared lints, and explicit unsafe forbids verified |
| `step_005` | complete | this checkpoint | Compatibility scanner and positive/negative self-test installed |
| `step_006` | pending | - | - |
| `step_007` | pending | - | - |
| `step_008` | pending | - | - |
