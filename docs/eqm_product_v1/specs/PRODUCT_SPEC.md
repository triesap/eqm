# Product specification — eqm_product_v1

Status: approved intent. Requirement IDs R01–R64 are stable traceability keys, not wire-schema versions. `eqm_product_v1` names this product specification; it does not authorize silently changing `schemas/v1` semantics.

## Purpose and users (R01–R03, R10)

EquivalenceMatrix is a local-first graph of shared product contracts, target-specific bindings, policy/profiles, independently produced evidence, and deterministic conformance/equivalence decisions. Developers, test/release engineers, and authorized operators use it to identify missing work and establish appropriately scoped assurance. It is not a source generator and does not require native targets to share implementations.

The user's product has a Rust command-line application, a Kotlin Multiplatform desktop control tower where users view account/task state and review workflows, and a Kotlin Multiplatform mobile edge application where users add/edit business evidence using photo/video/voice and related annotations. Relevant backend/shared providers participate where cross-runtime contracts depend on them. Exact operating systems, supported versions, source repositories, product API routes, and business rules were not supplied.

Business evidence belongs to the product's authorization, storage, revision and synchronization domain. Conformance evidence belongs to EQM. Do not store raw user media in the conformance engine or treat a content hash as proof of a real-world assertion. A user account ID or task ID in a synthetic scenario is not a new EQM account/task data model.

## Required capabilities

Preserve stable semantic IDs for capabilities, journeys, surfaces, requirements, fragments, targets, bindings, policies, profiles, evidence selectors, runners, adapters, and approved waivers. Resolve and validate authored authority; reject duplicate identities, invalid references, weakened protected scope, invalid fragment pins, and unsupported schemas. Derive exact obligations, acquire/import evidence only under explicit permission, and evaluate completeness, coverage, trust, freshness, attempts, inventory and exposure without hidden execution.

Make every conclusion explainable through an immutable input manifest and a complete evaluated snapshot. CLI checks, matrices, MCP, context, release checks and statements must display the same underlying conclusions. Efficient selective views may omit display rows, never required evaluation scope.

Support deliberate target asymmetry. Shared business invariants may apply broadly; camera capture may apply only to mobile; bulk review may apply only to desktop; synchronization scenarios span roles. A permanent design difference is modeled as approved participation, not an exception waiver.

## Claim boundaries

“Conformant” means the exact subject satisfies the declared obligations in the selected authority and profile context. “Equivalent” means contract-relative equivalence within a declared comparison cohort; it is not proof of universal program equivalence. Where output agreement is required, evidence must include a relational check over the same scenario and approved normalization rules. A screenshot or two independent target passes do not satisfy a separately required cross-target scenario.

Complete evaluation is not successful evaluation. Unknown, missing, stale, failed, unstable, insufficient trust, and waived evidence remain explicit. Signing an accurate failure record does not make it successful. Promotion requires separate protected release authorization for the exact artifact set and destination.

## Workflows (R44, R48, R60)

Development: inspect/validate authority; select exact scope; inspect a fully resolved dry-run plan; authorize acquisition separately; persist immutable normalized evidence and receipts; evaluate frozen inputs; inspect findings; rerun only justified affected work. A runner's zero exit is an acquisition result, not product conformance.

Pull request: use trusted invocation/protected baseline as required; compare candidate changes including deletions; evaluate candidate implementations against protected expectations; conservatively expand unknown changed dependencies; never execute candidate-controlled grading or signing authority as trusted infrastructure.

Release readiness: select protected authority and a candidate subject manifest, complete all required target/provider/scenario obligations, verify every mandatory input, decide readiness, optionally sign the decision, and separately authorize promotion. Post-deployment conformance uses observations that exist after deployment; it cannot be a circular prerequisite for the first deployment.

Product integration: adopt one real capture/edit/sync/review/export slice before multiplying features. Bind real native test selections, runtime observations and build outputs. Synthetic fixtures establish EQM semantics, not actual product readiness.

## Non-goals and conditional work

No EQM business backend, UI framework, feature-flag service, cloud orchestration platform, standalone deployment system, full Kotlin reimplementation, mandatory sidecar, or mandatory mobile evaluator. No authorization to publish, deploy, select live signers, or alter other product repositories is implied by approval of this spec. Native embedding is documented as an optional gated extension, not added to the default runtime surface.

## Success and rollout

A successful implementation closes the static-review gaps, satisfies the acceptance catalog, passes the relevant supported-host and native lanes, and makes all production guarantees evidence-backed. Start release gating in shadow mode, independently review decisions, then require explicit approval to enforce. Rollback uses a known-good evaluator or pauses promotion; it never becomes permissive.
