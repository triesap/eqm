# EquivalenceMatrix V1 RCLD 02: Typed Domain Graph

Status: in progress; Step 029 complete
Created: 2026-08-07
Updated: 2026-08-07
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 01
Current checkpoint: `step_030`

## Purpose

Implement the complete I/O-free typed graph model for product intent, targets,
bindings, exposure, evidence, policy, waivers, execution definitions,
inventories, runtime facts, releases, and deterministic indexes.

## Scope Boundary

This RCLD models validated semantic values. It does not parse TOML, inspect the
filesystem, execute policies, resolve cross-entity references, run commands,
or expose domain structs as the public JSON protocol.

## Definition Of Green

- Every entity follows the closed field tables and vocabularies.
- Required owners and revisions are enforced where specified.
- Types preserve fully qualified references and deterministic collections.
- No boolean-heavy or stringly API bypasses a validated enum/newtype.
- The corrected Requirement -> Fragment -> Surface dependency order compiles at
  every checkpoint.
- Crate-level docs make the construction and validation boundary explicit.

## Verification Lane

Run the standard locked workspace lane plus focused entity validation,
construction, ordering, duplicate, and doc tests in `eqm_domain`.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_017` | Capability entity and validation. | Required title/status/owners and deterministic representation pass positive/negative tests. | `feat(domain): define capability model` |
| `step_018` | Journey and transition entities. | Transition event and endpoint types validate locally; cross-reference checks remain engine work. | `feat(domain): define journey model` |
| `step_019` | **Corrected:** Requirement, level, scope, facets, applicability, provider, and risk. | Exhaustive scope/provider/facet constraints pass; prose atomicity is documented as review policy. | `feat(domain): define requirement model` |
| `step_020` | Fragment, revision, requirements, and immutable composition identity. | Duplicate local requirements and invalid revision state fail; overrides are unrepresentable. | `feat(domain): define fragment model` |
| `step_021` | Surface, requirements, and exact fragment-use pins. | Duplicate local requirements and malformed fragment pins fail; surface can compile using committed Requirement/Fragment APIs. | `feat(domain): define surface model` |
| `step_022` | Target root, platform, framework, and owners. | Root path and intentionally extensible identifier rules pass; deployment dimensions are not conflated with target identity. | `feat(domain): define target model` |
| `step_023` | Artifact roles, paths, surface/symbol/selector metadata, and role-specific constraints. | Invalid field/role combinations and duplicate artifact IDs fail. | `feat(domain): define artifact model` |
| `step_024` | Exposure declarations and finite dimension selectors over symbolic cohorts. | Undeclared dimensions/values and individual-identifier forms fail; intended availability stays separate from runtime facts. | `feat(domain): define exposure model` |
| `step_025` | Evidence specifications, closed kinds, runner/selector rules, full requirement coverage IDs, and facets. | Empty coverage/facets and kind-incompatible fields fail. | `feat(domain): define evidence specifications` |
| `step_026` | Immutable evidence result subject, digests, trust, producer, attempts, counts, and outcome. Prerequisite: ADR 0013 closes the nested result shapes before implementation. | Count consistency, skipped/filtered/unstable/retry semantics, and immutability helpers pass. | `feat(domain): define evidence results` |
| `step_027` | Policy, fixed profiles, required targets/facets/trust, and waiver rules. | Strength ordering primitives and invalid weakening representations are tested without performing policy composition. | `feat(domain): define policy model` |
| `step_028` | Scoped waiver, authority, issue, dates, controls, and expiry helpers. | Missing authority/scope and invalid date windows fail; waived cannot be represented as satisfied. | `feat(domain): define waiver model` |
| `step_029` | Runner definition, backend, adapter definition, discovery mode, completeness, limits, guarantees, and placeholders. | Unsupported isolation claims, unbounded resources, and shell-string forms are unrepresentable. | `feat(domain): define runner and adapter models` |
| `step_030` | Inventory, reconciliation facts, runtime-facts snapshot, and release record. | Completeness, subject, target, release identity, and symbolic-profile constraints pass. | `feat(domain): define inventory and release models` |
| `step_031` | Workspace graph container and sorted indexes over all semantic entities. | Stable insertion order, lookup, and duplicate hooks pass; graph resolution remains engine work. | `feat(domain): define workspace graph` |
| `step_032` | Crate-level API documentation and examples. | Public APIs are documented; doc tests pass; examples perform no I/O. | `docs(domain): document domain API` |

## Reconciliation Rules

- No entity absorbs parser source spans or protocol-only presentation fields.
- Cross-reference and filesystem assertions stay deferred to the engine and
  manifest crates respectively.
- Any missing field/vocabulary decision returns to RCLD 00 authority.
- RCLD 03 begins only after complete domain docs and tests are green.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_017` | complete | Capability authority and shared normalized extension values implemented |
| `step_018` | complete | Versioned journeys preserve surface order and canonicalize validated transitions |
| `step_019` | complete | Typed requirements enforce closed facets, scopes, providers, risk, and bounded applicability |
| `step_020` | complete | Versioned immutable fragments own nonempty uniquely keyed requirement sets |
| `step_021` | complete | Surfaces combine unique direct requirements with exact immutable fragment pins |
| `step_022` | complete | Targets keep repository roots separate from extensible platform and framework identity |
| `step_023` | complete | Artifacts use closed roles, typed selectors, bounded metadata, and unique local IDs |
| `step_024` | complete | Intended exposure uses symbolic applicability and remains distinct from runtime facts |
| `step_025` | complete | Evidence specifications enforce coverage sets and kind-compatible runners, selectors, and counts |
| `step_026` | complete | Immutable evidence results bind exact subjects, provenance, payload kinds, retry history, and digests |
| `step_027` | complete | Fixed profiles and policy rules enforce finite selections and monotonic strength axes |
| `step_028` | complete | Waivers require exact scope, external authority, approvals, dates, controls, and visible waived effect |
| `step_029` | complete | Shell-free runners and pinned adapters enforce typed placeholders, bounds, isolation claims, and discovery identity |
| `step_030`-`step_032` | pending | - |
