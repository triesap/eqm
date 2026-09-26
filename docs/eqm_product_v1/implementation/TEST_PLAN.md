# Test plan

## Layers

Domain/unit: validated identities, relationships, counts, scopes, applicability, per-facet constraints, time boundaries, waiver eligibility and reference decision precedence. Each fix includes a focused regression in the same green commit. Never commit red tests or let public constructors/Deserialize bypass trusted preparation.

Protocol: strict parsing/duplicate-member rejection, unknown schemas/statuses, bounded depth/bytes, canonical numeric/time/Unicode/set semantics, source-to-generated-schema parity, semantic/raw identity relationships and original-signed-byte preservation. Store exact vectors in shared fixtures and run them through activated native consumers.

Engine properties: input-order invariance, duplicate-delivery idempotence, required-evidence removal cannot strengthen, protected weakening cannot shrink requirements, complete scope accounting, subject substitution rejection, campaign/attempt/count behavior, reference/optimized equality. Keep random generators bounded and reproducible on failure.

Application integration: authority selection -> plan -> authorized acquisition/import -> object/index publication -> complete frozen inputs -> evaluation -> check/matrix/context/MCP -> historical replay/current acceptance -> statement/signing -> authorization -> exact consumption. Test each edge and complete synthetic lifecycles; no production synthetic trust.

Storage: concurrent initialization, bounded object writes, no-clobber raw collision, semantic duplicate transport, persistent attempt conflict, transactions/checkpoints/read snapshots, object-first publication crash points, missing/corrupt files, leases/GC, index rebuild and interrupted version migration/restore. Run durability tests on declared native hosts/filesystems.

Execution/security: argv boundaries, explicit environment, absent secret authority, result-file replacement/ancestor symlinks, oversized input/retained files, stdout/stderr floods, immediate parent exit with open descendant pipes, cancellation/shutdown races, worker attempts to write protected state, executable discovery authorization and unsupported backend guarantees. Host isolation is tested natively, not inferred from cfg compilation.

Cryptographic: real fixture signatures plus tampered bytes, wrong identity/issuer/builder/source/artifact/configuration/attempt, missing required verification material, trust rotation/expiry and forged receipt fields. Test exact statement signatures for failed/unknown as well as passing results. Live signer activation is separate from fixture validity.

Product/native: actual discovered Rust/KMP/provider tasks test capture/edit/save acknowledgment/kill/restart/reconnect/retry/review/export, account switching, revocation, concurrent edits, interrupted uploads, stale state and approved mixed-version cohorts. Product-owned business rules define expected conflict/revision semantics; EQM does not invent them.

Performance: populated cold/warm/delta evaluation, source sharing, realistic evidence histories, large profiles/matrices, verification overhead, storage contention, report serialization and activated native boundary. Benchmark correctness and reference parity before timing; document hardware and distributions. Existing constants are not accepted measurements.

Operational: shadow comparison and independent review, wrong-artifact/destination promotion refusal, key/material rotation, corrupted/missing archive recovery, nonpermissive evaluator rollback and unavailable-host behavior. Explicit support/activation records are part of acceptance.

## Fixture discipline

Use small deterministic fixtures for unit cases and bounded representative fixtures for scale. Put shared helpers in eqm_test_support only when reusable. Mark synthetic test data/keys clearly and keep them out of production trust factories. Do not use live business media/accounts or uncontrolled network fixtures. Regenerate golden identities from reviewed source fixtures only; a changed golden is not proof of correctness by itself.

Prefer existing test frameworks and repository conventions. Add property/fuzz/dependency tooling only with exact justified pins. Record failing seeds and minimize cases. Use temporary roots and explicit clocks; do not let cleanup delete real .eqm archives or product data. Every test that authorizes fixture release must also test a nearby non-authorizing mutation.

## Acceptance mapping

AC001–AC064 in specs/ACCEPTANCE_CRITERIA.md define observable outcomes. implementation/TRACEABILITY.md and traceability.json map requirements/findings/steps. Record actual test names, commands and results after implementation; the planned suite names in commit scope are not claims that tests already exist or ran.

## Review focus

Verify tests establish semantics, not merely array length, nonempty cells, fixed missing states or an always-unknown release. Positive trusted lifecycle tests are required alongside fail-closed negatives. Inspect aggregation against the independent normative table and campaign count semantics. Compare CLI/MCP/report snapshots with identical inputs/time; separate invocations with legitimately changed inputs/time may produce different decisions and must not be forced equal by hiding freshness.

## Approved review regression execution

RA01–RA21 are machine-indexed in review_regressions.json and mapped into every owning step, requirement and acceptance case. Implement each step's relevant Given/When/Expected contribution with its code; do not commit knowingly failing tests or defer all regression work to the final chapter. Later integration steps close cross-boundary claims after their dependencies exist.

Exercise semantic edge cases with expected outcomes independent of the implementation; process/filesystem hazards with adversarial disposable workers and native host lanes; schemas with actual command output; and interface parity with one persisted immutable decision. Baseline tests and static source review do not close these clauses. Record test names, exact commands, host/mode, results and evidence under the corresponding RA/AC records.
