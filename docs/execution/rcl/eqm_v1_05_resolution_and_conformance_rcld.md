# EquivalenceMatrix V1 RCLD 05: Resolution And Conformance

Status: in progress; Step 059 complete
Created: 2026-08-07
Updated: 2026-08-07
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 04
Current checkpoint: `step_060`

## Purpose

Implement the pure graph-resolution and semantic evaluation core from domain
inputs through target conformance and target-set equivalence.

## Scope Boundary

`eqm_engine` performs no filesystem, Git, process, terminal, network, manifest
parsing, or public rendering. Evaluation accepts explicit graph, policy,
baseline, clock, subject, digest, evidence, inventory, facts, and release
inputs. It does not acquire any of them.

## Definition Of Green

- Duplicate and dangling authorities, cycles, and orphan active units fail with
  stable diagnostics.
- Fragments expand only from exact ID/revision/digest pins and cannot override
  requirements.
- Canonicalization integration occurs only after resolution, expansion, and
  invariants.
- Applicability and policy behavior match exhaustive repository truth tables.
- Policy composition is monotonic against an exact protected baseline.
- Evidence specs, results, freshness, trust, and attempts remain distinct.
- Waivers never satisfy evidence and only produce visible conditional status.
- Unknown never yields conformance or equivalence.
- Engine outputs are deterministic under all input insertion permutations.

## Verification Lane

Run the standard locked workspace lane plus resolution fixtures, truth-table
tests, independent digest integration vectors, seeded permutation/property
tests, and the three-target signup evaluation corpus.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_057` | Resolve domain inputs into indexed `WorkspaceGraph`; detect duplicate authorities and dangling references. | Valid graph resolves deterministically; duplicate/dangling fixtures produce stable source-linked diagnostics. | `feat(engine): resolve graph` |
| `step_058` | Enforce fragment/journey cycles, invalid transitions, ownership/lifecycle invariants, and orphan active surfaces. | Cycle, orphan, invalid-transition, and inactive-parent fixtures match the approved invariant tables. | `feat(engine): check graph invariants` |
| `step_059` | Expand exact-pinned fragments without overrides and integrate finalized-graph canonicalization. | Pin mismatch/cycle/override tests fail; expanded digest matches an independent vector and changes only for normative semantics. | `feat(engine): expand fragments` |
| `step_060` | Evaluate equality, membership, and exclusion selectors over declared finite dimensions. | Exhaustive truth table, undeclared dimension/value, and deterministic applicability tests pass. | `feat(engine): evaluate applicability` |
| `step_061` | Select development, pull_request, and release policy profiles with risk/unit filters and explicit non-local profile selection. | Default/local and explicit CI/release profile behavior matches contract; invalid/missing profile fails. | `feat(engine): select policy profiles` |
| `step_062` | Compare candidate and exact protected baseline contracts, policies, runners, waiver authority, and trust controls for monotonicity. | Strengthening passes; every weakening class fails unless represented by a separately valid waiver where allowed. | `feat(engine): enforce monotonic policy` |
| `step_063` | Derive obligations by unit, target, requirement, scope, facet, trust, profile, and release context. | Signup and exhaustive level/scope/risk/profile fixtures produce stable complete obligations without duplicates. | `feat(engine): derive obligations` |
| `step_064` | Evaluate non-executing structure against an injected repository view. | Existence, role, confined path, portable collision, and symlink-resolution cases pass without engine I/O. | `feat(engine): evaluate structure checks` |
| `step_065` | Map evidence specifications to obligations by exact requirement, facet, target, kind, runner, and context. | Exact/partial/duplicate/incompatible/missing coverage fixtures match the coverage table. | `feat(engine): evaluate evidence coverage` |
| `step_066` | Evaluate all freshness keys, expiry, and current graph/runtime context. | Each stale-key dimension is independently tested; unchanged exact keys remain fresh. | `feat(engine): evaluate evidence freshness` |
| `step_067` | Aggregate attempts and counts into passed, failed, skipped, filtered, unstable, timed_out, and cancelled outcomes. | Zero-match, retry-after-failure, quarantine, inconsistent count, and terminal outcome tables pass. | `feat(engine): aggregate evidence outcomes` |
| `step_068` | Validate and apply scoped, approved, unexpired waivers without satisfying evidence. | Scope, authority, issue, duration, expiry, and compensation rules pass; waiver non-satisfaction property holds. | `feat(engine): evaluate waivers` |
| `step_069` | Combine obligations, facets, structure, evidence, trust, freshness, waivers, exposure, and release into target conformance. | Exhaustive status-precedence table passes; conditional status requires visible valid waivers; unknown never conforms. | `feat(engine): evaluate target conformance` |
| `step_070` | Derive target-set equivalence from required target conformance under one exact evaluation context. | Equivalent, conditional, not-equivalent, and unknown three-target fixtures match the normative table. | `feat(engine): evaluate target-set equivalence` |

## Determinism And Property Gates

At wave end, seeded permutations of input files, maps, evidence-result order,
policy order, and target order must yield byte-identical semantic outputs and
digests. Property tests must prove monotonic policy behavior and that a waiver
never converts an unsatisfied facet into `satisfied`.

## Reconciliation Rules

- The engine receives an injected clock; it never reads wall time.
- Protected baselines are exact prepared inputs; the engine never resolves Git
  references.
- A missing truth-table case returns to RCLD 00 authority.
- RCLD 06 begins only after the full signup conformance/equivalence corpus is
  deterministic and green.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_057` | complete | Pure resolution detects every indexed authority and binding-coordinate duplicate before construction, resolves typed graph references across contracts, bindings, policies, profiles, runners, waivers, targets, imports, and transitions, and emits deterministic source-linked EQM-E0300/E0301 findings |
| `step_058` | complete | Post-resolution validation enforces identifier ownership, bidirectional journey/surface membership, transition membership, active-surface orphan rejection, parent lifecycle compatibility, and non-lowering journey/fragment risk; typed schemas make fragment nesting and parent-type cycles unrepresentable |
| `step_059` | complete | Domain-separated canonical fragment digests verify exact ID/revision/content pins; expansion applies the specified prefix transform, rejects missing content, mismatches, invalid IDs, and overrides, and emits a finalized graph type required by canonicalization with fixed digest and formatting-exclusion integration vectors |
| `step_060`-`step_070` | pending | - |
