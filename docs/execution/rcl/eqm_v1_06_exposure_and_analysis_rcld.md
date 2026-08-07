# EquivalenceMatrix V1 RCLD 06: Exposure And Analysis

Status: in progress; Step 075 complete
Created: 2026-08-07
Updated: 2026-08-07
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 05
Current checkpoint: `step_076`

## Purpose

Complete the pure analysis engine for exposure reconciliation, release gates,
semantic diffs, conservative affected sets, stable matrices, and diagnostic
explanations.

## Scope Boundary

This RCLD consumes explicit graph and fact inputs. It does not discover files,
resolve Git references, invoke adapters/runners, load release evidence, or
render terminal output.

## Definition Of Green

- Expected, declared, discovered, enabled, released, and conformant remain
  separate facts in data and output.
- Release evaluation binds exact build, source, artifact, evidence, runtime,
  policy, trust, and waiver context.
- Semantic diff classification distinguishes normative changes from metadata.
- Affected-set analysis is conservative and never misses an obligation.
- Matrices and explanations are stable and exhaustive.

## Verification Lane

Run the standard locked workspace lane plus reconciliation/release fixtures,
golden diff and matrix tests, affected-set properties, and diagnostic-registry
completeness checks.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_071` | Reconcile expected, declared, discovered, enabled, released, and conformant facts independently. | Full cross-product fixtures prove no fact implies another and adapter failure yields unknown rather than absence. | `feat(engine): reconcile exposure` |
| `step_072` | Evaluate exact release subjects against policy, facets, evidence, facts, trust, freshness, and waivers. | Passing/failing/unknown/conditional release fixtures bind the exact build and default release trust correctly. | `feat(engine): evaluate release gate` |
| `step_073` | Classify added, removed, strengthened, weakened, evidence, waiver, exposure, and nonnormative metadata changes. | Golden diffs are symmetric where specified, deterministically ordered, and baseline/candidate explicit. | `feat(engine): classify diffs` |
| `step_074` | Compute reverse indexes and conservative affected units/obligations from changed files and semantic changes. | Mapped changes are precise; unmapped target changes affect all target units; conservatism property passes. | `feat(engine): analyze affected set` |
| `step_075` | Generate exposure, conformance, evidence, release, and equivalence matrix data. | Golden matrices include every required target/unit/facet and use stable row/column ordering. | `feat(engine): generate matrices` |
| `step_076` | Map every stable diagnostic code to title, explanation, authority reference, and remediation. | Every emitted code is registered exactly once; every registry entry has tests and no dead code exists. | `feat(engine): implement explain registry` |

## Reconciliation Rules

- No analysis API accepts an ambiguous branch name where an exact baseline is
  required.
- Metadata-only changes do not alter normative digests.
- RCLD 07 begins only after matrices, diffs, release status, and explanations
  are deterministic and complete.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_071` | complete | Expected, declared, discovered, enabled, released, and conformant remain separate typed facts; every required/prohibited/unknown by true/false/unknown comparison follows the complete cross-product, partial or failed observation remains unknown, and conformance never overwrites exposure mismatches |
| `step_072` | complete | Release gates bind target, version, build, source commit, artifact, channel, contract, policy, profile values, evidence set, runtime facts, trust configuration, release record, and injected clock; verified signed-CI exact inputs pass, visible wholly waived deviation is conditional, complete unwaived mismatch fails, and absent, stale, inexact, invalid, or unverifiable input is unknown |
| `step_073` | complete | Explicit finalized baseline/candidate projections classify protected-axis additions/removals and ordered changes as strengthened/weakened, unordered entities as added/removed, and evidence, waiver, exposure, and excluded metadata in their stable classes; output sorts by the normative coordinate and reverses directional classes while swapping values |
| `step_074` | complete | Explicit reverse indexes map artifacts, targets, semantic coordinates, dependency and fragment-consumer edges, and derived obligations; mapped files remain precise, direct impacts expand through transitive dependents, unmapped target files conservatively select every target unit, repository-global or unclassified authority changes select the full workspace, and metadata-only semantic changes may remain empty |
| `step_075` | complete | Exposure, conformance, evidence, release, and equivalence share a closed matrix family with typed target/unit/facet axes, sorted labels, row-major cells, obligation and diagnostic attribution, explicit unknown cells for every unprepared required coordinate, and rejection of empty or out-of-axis inputs |
| `step_076` | pending | - |
