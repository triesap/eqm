# EquivalenceMatrix V1 RCLD 04: Manifest System

Status: in progress; Step 046 complete
Created: 2026-08-07
Updated: 2026-08-07
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 03
Current checkpoint: `step_047`

## Purpose

Implement strict, source-located, deterministic loading and formatting of the
complete current-v1 authored workspace, plus canonical semantic projection and
a comprehensive fixture corpus.

## Scope Boundary

The crate may perform filesystem input/output needed for discovery, loading,
formatting, and safe path resolution. It does not evaluate conformance, launch
runners/adapters, inspect remote refs during normal commands, or own public
report DTOs.

## Definition Of Green

- TOML parsing is strict and source-located.
- One explicit workspace config and deterministic source discovery are
  enforced within the VCS boundary.
- Unknown fields, wrong schemas, duplicate authorities, invalid paths, and
  floating lock references fail closed.
- Conversion produces validated domain inputs without losing diagnostic spans.
- Formatting is comment-preserving and idempotent.
- Canonicalization accepts only a finalized graph and follows the exact
  normative projection contract.
- Every repository example is valid; deliberately invalid data is isolated in
  named negative fixtures.

## Verification Lane

Run the standard locked workspace lane plus manifest fixture, span, schema,
format-idempotence, deterministic discovery, path/symlink, canonical vector,
and example-validation tests.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_042` | TOML 1.1 parser wrapper with syntax diagnostics and byte/source spans. | Valid/invalid syntax, Unicode, duplicate keys, and stable span fixtures pass without panic. | `feat(manifest): parse toml with spans` |
| `step_043` | Config selection, VCS-boundary discovery, `--config` semantics, one-config enforcement, target/source declarations. | Missing, duplicate, nested, explicit-config, and boundary fixtures pass deterministically. | `feat(manifest): load workspace config` |
| `step_044` | Deterministic lexical glob discovery for every authored source class, excluding generated state. | Ordering, portable collisions, duplicate authorities, ignored state, and symlink-boundary tests pass. | `feat(manifest): discover sources deterministically` |
| `step_045` | Exact current schema dispatch, unknown-field denial, extension namespace validation, and input limits. | Old/future/foreign schema, unknown field, malformed extension, and oversized fixtures fail with source diagnostics. | `feat(manifest): validate strict schemas` |
| `step_046` | Convert capability, journey, requirement, fragment, and surface DTOs into domain inputs with spans. | Positive and field/reference-local negative contract fixtures pass. | `feat(manifest): convert contracts` |
| `step_047` | Convert binding artifacts, exposures, and evidence specs; reject command strings and invalid role relationships. | Valid target binding passes; command/path/selector/coverage negatives fail. | `feat(manifest): convert bindings` |
| `step_048` | Convert policies/profiles and finite declared dimensions. | Profile, facet, trust, selector, strengthening primitive, and symbolic-cohort fixtures pass. | `feat(manifest): convert policies` |
| `step_049` | Convert runner definitions and validate argv placeholders, cwd, resource bounds, backend guarantees, and environment allowlist. | Shell strings, unknown placeholders, escapes, zero/unbounded resources, and unsupported guarantees fail. | `feat(manifest): convert runners` |
| `step_050` | Convert scoped waivers with approver, issue, dates, controls, and authority. | Missing, overlong, reversed, expired-at-evaluation, and invalid-scope fixtures are classified correctly. | `feat(manifest): convert waivers` |
| `step_051` | Parse `eqm.lock` entries for exact imports/adapters, digests, configuration, and trust metadata. | Floating refs, duplicate entries, digest mismatch shape, and old schema fail closed. | `feat(manifest): parse lockfile` |
| `step_052` | Implement pure normative projection, deterministic ordering, RFC 8785 JCS, and SHA-256 over finalized `WorkspaceGraph`. | Independent JCS vectors, ordering permutations, normative/non-normative changes, extensions, and digest vectors pass. | `feat(manifest): canonicalize semantic graph` |
| `step_053` | Orchestrate config selection, discovery, parse, validation, conversion, lock loading, source maps, and diagnostics into graph inputs. | Complete valid input loads deterministically; aggregate diagnostics retain stable source locations. | `feat(manifest): build workspace loader` |
| `step_054` | Comment-preserving manifest formatter with check/dry-run backend and atomic-write primitive. | Golden, idempotence, newline, comment, and no-semantic-change tests pass. | `feat(manifest): format manifests` |
| `step_055` | Add comprehensive valid and invalid manifest workspace corpus. | Corpus covers schema, field, path, collision, duplicate, limit, Unicode, lock, and source-discovery boundaries. | `test(manifest): add fixture corpus` |
| `step_056` | Import valid current examples and validate them through the real loader; replace all placeholder digests. | Every positive example loads; real fragment digest matches; negative examples remain only in negative fixtures. | `test(manifest): validate examples` |

## Canonicalization Integration Gate

Step 052 defines canonicalization over a manually constructed finalized graph.
RCLD 05 Step 059 must add the integration proof that parsing, resolution,
fragment expansion, invariant validation, and canonicalization execute in the
approved order. No pre-resolution or pre-expansion digest is authoritative.

## Reconciliation Rules

- A source span is diagnostic metadata and never enters a normative digest.
- Filesystem canonicalization cannot weaken lexical `RepoPath` validation.
- Normal commands never fetch remote imports or adapters.
- RCLD 05 begins only after the complete example workspace loads
  deterministically.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_042` | complete | Bounded UTF-8 TOML 1.1 parsing reports stable byte-derived spans for syntax and duplicate-key failures |
| `step_043` | complete | Repository-rooted config selection enforces one config, explicit paths, required source declarations, generated-root policy, and nested VCS boundaries |
| `step_044` | complete | Lexically validated source globs expand in repository-path order while excluding generated state, nested repositories, symlinks, portable collisions, and cross-class matches |
| `step_045` | complete | Exact-current schema dispatch, closed DTO decoding, recursive extension validation, aggregate input limits, duplicate authorities, and source-associated failures are enforced |
| `step_046` | complete | Capability, journey, surface, fragment, requirement, applicability, transition, and exact fragment-pin DTOs convert through domain invariants with source and field attribution |
| `step_047`-`step_056` | pending | - |
