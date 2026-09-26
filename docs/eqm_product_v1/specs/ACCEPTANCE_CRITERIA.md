# Acceptance catalog and definition of done

All cases below are required for their declared scope. The optional native case applies only after embedding activation. Native/product/operational cases may require external repositories, hosts and owner authority; local fixtures do not close live acceptance. Every status is **not run** at package creation. Map implemented tests and exact execution evidence to these IDs in the progress ledger.

Code-complete, locally verified, supported-platform verified and operationally authorized are separate states. No one state implies the next. All mandatory code, schema, quality and adversarial gates must pass before shipping supported functionality. Release enforcement additionally requires protected real verification configuration, host guarantees, product scenarios, deployment-consumption checks and independent approval.

## AC001 — Business-domain separation (R01)

**Given:** Product account/task/media fixtures exist.

**When:** Review dependencies and public domain models.

**Required result:** EQM evaluates requirements without becoming a business database, sync implementation or UI framework.

**Evidence lane:** Architecture review. **Initial status:** not run.

## AC002 — Distinct runtime roles (R02)

**Given:** CLI, desktop, mobile and provider participants are declared.

**When:** Generate a target/scenario view.

**Required result:** Roles and participant occurrences remain distinct from the EQM developer CLI.

**Evidence lane:** Unit/integration. **Initial status:** not run.

## AC003 — No implicit embedding (R03)

**Given:** Default build/tooling is configured.

**When:** Inspect artifacts and dependencies.

**Required result:** No mobile evaluator, bridge or daemon is introduced without optional activation.

**Evidence lane:** Package/architecture. **Initial status:** not run.

## AC004 — One decision everywhere (R04)

**Given:** One frozen input produces a decision.

**When:** Render CLI, matrix, MCP and statement views.

**Required result:** Statuses and contributing reasons agree on the same snapshot.

**Evidence lane:** Cross-interface integration. **Initial status:** not run.

## AC005 — Pure evaluation (R05)

**Given:** Inputs and explicit time are supplied with execution/network spies.

**When:** Evaluate and query missing inputs.

**Required result:** No process/network/ambient read is attempted; missing facts remain explicit.

**Evidence lane:** Unit/security. **Initial status:** not run.

## AC006 — Acyclic layering (R06)

**Given:** All crate/module dependencies are available.

**When:** Run dependency/architecture tests.

**Required result:** No domain/engine I/O dependency or CLI/MCP-to-application cycle appears.

**Evidence lane:** Build/architecture. **Initial status:** not run.

## AC007 — Shared KMP sources (R07)

**Given:** Two output targets use one common source component.

**When:** Load bindings and change the shared source.

**Required result:** Both outputs remain valid and both are affected without relaxed confinement.

**Evidence lane:** Unit/integration. **Initial status:** not run.

## AC008 — Role-specific applicability (R08)

**Given:** Mobile-only capture and an unresolved profile coordinate exist.

**When:** Derive the universe.

**Required result:** Intentional exclusions are justified and unknown coordinates are retained.

**Evidence lane:** Property. **Initial status:** not run.

## AC009 — Different exact builds (R09)

**Given:** Desktop N+1 and mobile N are an approved cohort.

**When:** Compare them, then substitute one artifact.

**Required result:** Legitimate distinct builds compare; unauthorized substitution fails exact binding.

**Evidence lane:** Integration. **Initial status:** not run.

## AC010 — Contract-relative equivalence (R10)

**Given:** Independent target passes exist without required relational evidence.

**When:** Request cross-runtime assurance.

**Required result:** The relational obligation remains blocking and the claim is explicitly scoped.

**Evidence lane:** Unit/integration. **Initial status:** not run.

## AC011 — Completeness coverage (R11)

**Given:** An expected provider/target/profile coordinate is omitted.

**When:** Construct a prepared decision.

**Required result:** Completeness cannot be asserted; omission is visible and non-authorizing.

**Evidence lane:** Unit/security. **Initial status:** not run.

## AC012 — Empty/singleton scope (R12)

**Given:** An empty release scope or one-target comparison is requested.

**When:** Evaluate without explicit protected empty-scope permission.

**Required result:** No vacuous release authorization or meaningful cross-target equivalence claim.

**Evidence lane:** Unit. **Initial status:** not run.

## AC013 — All scope types (R13)

**Given:** Target, provider and repeated-role scenario obligations exist.

**When:** Omit end-to-end evidence while target evidence passes.

**Required result:** No full release authorization; missing dependencies are explained.

**Evidence lane:** Integration. **Initial status:** not run.

## AC014 — Facet policy composition (R14)

**Given:** Two facets have different matching policy strengths.

**When:** Derive and evaluate constraints.

**Required result:** Only relevant rules compose and canonical precedence is stable.

**Evidence lane:** Table/property. **Initial status:** not run.

## AC015 — Non-success payloads (R15)

**Given:** Failed manual review, skipped/quarantined tests and unknown verification are valid inputs.

**When:** Evaluate each supported payload.

**Required result:** No catch-all success; each remains its normative blocking/conditional state.

**Evidence lane:** Table/property. **Initial status:** not run.

## AC016 — Count integrity (R16)

**Given:** A report is split, duplicated and retried while expected cases are unchanged.

**When:** Import and evaluate minimum count.

**Required result:** Only defined distinct observations count; missing expected cases remain missing.

**Evidence lane:** Property/integration. **Initial status:** not run.

## AC017 — Campaign selection (R17)

**Given:** One campaign has a fail/pass retry and a later approved campaign exists.

**When:** Select under explicit policy.

**Required result:** Retries remain visible; selection is authorized and old unrelated history is not permanent poison.

**Evidence lane:** Unit/integration. **Initial status:** not run.

## AC018 — Transport and attempt identity (R18)

**Given:** Equivalent JSON transports and contradictory same-attempt assertions arrive.

**When:** Import repeatedly.

**Required result:** Duplicates are idempotent; conflicting attempts persist and block affected acceptance.

**Evidence lane:** Storage/property. **Initial status:** not run.

## AC019 — Digest binding (R19)

**Given:** Runner executable or a binding's semantics change without its ID changing.

**When:** Compare resulting identities and cache eligibility.

**Required result:** Appropriate component digest changes and incompatible evidence is rejected.

**Evidence lane:** Property. **Initial status:** not run.

## AC020 — Acyclic lifecycle (R20)

**Given:** A candidate plan precedes its evidence and decision.

**When:** Validate references including a synthetic cycle.

**Required result:** Valid flow works; future/self-referential input identities are rejected.

**Evidence lane:** Unit. **Initial status:** not run.

## AC021 — Canonical wire vectors (R21)

**Given:** Duplicate keys, large integers, Unicode and set/sequence fixtures exist.

**When:** Round-trip each permitted representation.

**Required result:** Rejection/encoding is specified and digests agree across supported consumers.

**Evidence lane:** Protocol/property. **Initial status:** not run.

## AC022 — Frozen provenance (R22)

**Given:** Sources/index entries change during preparation or a local tree is dirty.

**When:** Acquire/evaluate a snapshot.

**Required result:** One consistent generation is bound; dirty bytes never masquerade as clean HEAD.

**Evidence lane:** Integration/security. **Initial status:** not run.

## AC023 — Protected authority (R23)

**Given:** Candidate deletes a critical requirement/test or changes grader/policy.

**When:** Run protected evaluation.

**Required result:** Protected expectations and evaluator remain authoritative; weakening cannot self-authorize.

**Evidence lane:** Security. **Initial status:** not run.

## AC024 — Independent trust dimensions (R24)

**Given:** Signature is valid but subject, issuer, builder or authority is wrong.

**When:** Verify mandatory records.

**Required result:** Each incorrect dimension rejects acceptance; absent receipts are not discarded.

**Evidence lane:** Crypto/security. **Initial status:** not run.

## AC025 — Real cryptographic path (R25)

**Given:** Reviewed verifier and test-only signed fixtures are configured.

**When:** Verify positive and tampered bundles.

**Required result:** Cryptography actually runs; no synthetic production trust or disabled claim checks.

**Evidence lane:** Crypto integration. **Initial status:** not run.

## AC026 — Receipt import boundary (R26)

**Given:** Input contains a serialized verified flag or test-only authority marker.

**When:** Import/replay through production APIs.

**Required result:** Verification boundary is reestablished; flags cannot create authority.

**Evidence lane:** Security. **Initial status:** not run.

## AC027 — Signing is not approval (R27)

**Given:** A failed/unknown decision is accurately signed.

**When:** Attempt release authorization.

**Required result:** Statement remains valid as a failure record but cannot authorize promotion.

**Evidence lane:** Integration. **Initial status:** not run.

## AC028 — Readiness versus deployment (R28)

**Given:** A runtime snapshot is present but facts mismatch, or post-deploy data does not yet exist.

**When:** Evaluate readiness/post-deploy use cases.

**Required result:** Actual facts are compared; no presence-as-match or circular prerequisite.

**Evidence lane:** Integration. **Initial status:** not run.

## AC029 — Precise waivers (R29)

**Given:** Expired, unrelated, and valid scoped waivers are provided.

**When:** Evaluate affected obligations and reports.

**Required result:** Only authorized contributing waivers apply and none becomes satisfied evidence.

**Evidence lane:** Table/security. **Initial status:** not run.

## AC030 — Concrete dry-run (R30)

**Given:** Manifest declares a runner with explicit inputs.

**When:** Plan, then change executable/argv before execution.

**Required result:** Plan is complete and non-executing; changed plan invalidates approval.

**Evidence lane:** Runner integration. **Initial status:** not run.

## AC031 — Host isolation (R31)

**Given:** Untrusted worker attempts to modify authority/store or access signer material.

**When:** Execute under approved host policy.

**Required result:** Access is denied or unsupported isolation rejects execution before launch.

**Evidence lane:** Native security. **Initial status:** not run.

## AC032 — Whole invocation bounds (R32)

**Given:** Parent exits with descendant pipes open or streams flood during cancellation.

**When:** Supervise through deadline/shutdown.

**Required result:** Owned work and drainage terminate within declared bounds with explicit outcome.

**Evidence lane:** Native adversarial. **Initial status:** not run.

## AC033 — Filesystem safety (R33)

**Given:** Parent symlink, replaced result path or oversized collision file is supplied.

**When:** Inspect/read/persist using bounded handles.

**Required result:** No escape, unbounded read or false structural role confirmation.

**Evidence lane:** Security/storage. **Initial status:** not run.

## AC034 — Secrets/environment (R34)

**Given:** Ambient environment contains a marker and approved secret is used in fixture output.

**When:** Run and inspect plan/log/error retention.

**Required result:** Only declared inputs exist; secrets are absent/redacted from retained views.

**Evidence lane:** Security. **Initial status:** not run.

## AC035 — Crash-safe publication (R35)

**Given:** Fault injection stops each object/index publication boundary.

**When:** Reopen/recover.

**Required result:** No visible completed record lacks installed bytes; orphans are safely recoverable.

**Evidence lane:** Storage fault injection. **Initial status:** not run.

## AC036 — SQLite runtime and contention (R36)

**Given:** Multiple readers/writers/checkpoint attempts use the linked runtime.

**When:** Inspect fix/runtime and stress transactions.

**Required result:** Audited fixed runtime, bounded contention and coherent local-host WAL semantics.

**Evidence lane:** Native/storage. **Initial status:** not run.

## AC037 — Retention and GC leases (R37)

**Given:** An object is live in a reader/run/archive while cleanup runs.

**When:** Apply configured retention/GC.

**Required result:** Live/release references survive; only eligible unreachable objects are removed.

**Evidence lane:** Storage/property. **Initial status:** not run.

## AC038 — Historical/current split (R38)

**Given:** Evidence expires or trust changes after an accepted decision.

**When:** Replay history and check current acceptance.

**Required result:** History reproduces; present authorization reevaluates and may reject.

**Evidence lane:** Integration. **Initial status:** not run.

## AC039 — Cache validity (R39)

**Given:** A bound component or validity boundary changes.

**When:** Reuse a staged cache entry.

**Required result:** Invalid or expired acceptance is not reused; exact unchanged work may be reused.

**Evidence lane:** Property. **Initial status:** not run.

## AC040 — Affected selection (R40)

**Given:** Shared source, deletion, rename and unmapped path changes exist.

**When:** Run affected analysis and verify --affected planning.

**Required result:** Dependent work is selected and uncertainty widens rather than drops scope.

**Evidence lane:** Integration. **Initial status:** not run.

## AC041 — Batch scheduling (R41)

**Given:** Multiple requirements share cases/devices with bounded capacity.

**When:** Schedule and cancel batched acquisition.

**Required result:** No duplicate count credit/device conflict; deterministic results and bounded work.

**Evidence lane:** Concurrency integration. **Initial status:** not run.

## AC042 — Resource ceilings (R42)

**Given:** Inputs exceed configured parser/graph/profile/evidence/request budgets.

**When:** Prepare/evaluate/query.

**Required result:** Explicit non-authorizing limit failure; no silent obligation truncation.

**Evidence lane:** Fuzz/security. **Initial status:** not run.

## AC043 — Reference parity (R43)

**Given:** Representative random/fixed workloads use reference and optimized paths.

**When:** Compare decisions and record performance.

**Required result:** Semantics match; measurements include full workloads and actual host evidence.

**Evidence lane:** Differential/benchmark. **Initial status:** not run.

## AC044 — Real stored evidence in views (R44)

**Given:** Fresh passing evidence is persisted, then made stale/failed.

**When:** Run check/matrix/context/MCP/release views.

**Required result:** Views move coherently; no hardcoded missing/empty-success placeholders.

**Evidence lane:** Cross-interface integration. **Initial status:** not run.

## AC045 — Structured errors (R45)

**Given:** Invalid manifests/evidence and I/O failures occur under JSON/MCP mode.

**When:** Run relevant operations.

**Required result:** Machine-readable errors preserve safe detail/source context and correct exit semantics.

**Evidence lane:** CLI/MCP integration. **Initial status:** not run.

## AC046 — Snapshot-bound presentation (R46)

**Given:** Filter/page/truncate a large decision while a new snapshot appears.

**When:** Continue reading.

**Required result:** Cursor binds the original view; completeness/authorization do not change.

**Evidence lane:** Protocol/integration. **Initial status:** not run.

## AC047 — MCP least privilege (R47)

**Given:** Read-only session receives execution/signing or untrusted textual instructions.

**When:** Process requests.

**Required result:** No privilege escalation, implicit execution or unauthenticated server default.

**Evidence lane:** MCP security. **Initial status:** not run.

## AC048 — Product lifecycle scenario (R48)

**Given:** Authorized synthetic or real product fixtures support capture/edit/restart/sync/review/export.

**When:** Run full cross-runtime workflow.

**Required result:** Correct revision exactly once under correct task; no loss/disclosure; real acceptance distinguished from fixtures.

**Evidence lane:** Native/product integration. **Initial status:** not run.

## AC049 — Optional native boundary (R49)

**Given:** Embedding is explicitly activated with a declared platform matrix.

**When:** Test handles/cancellation/errors/packaging on each target.

**Required result:** Ownership and compatibility are correct; unsupported JNA-on-Native assumptions fail explicitly.

**Evidence lane:** Optional native. **Initial status:** not run.

## AC050 — Code-health invariants (R50)

**Given:** New types and public APIs are implemented.

**When:** Review constructors/errors/globals/dependencies and run lints.

**Required result:** No unchecked success factories, production panic shortcuts or ambient evaluation inputs.

**Evidence lane:** Build/review. **Initial status:** not run.

## AC051 — Unsafe isolation (R51)

**Given:** Host/FFI implementation needs platform APIs.

**When:** Inspect lint scopes and safety review.

**Required result:** Core remains unsafe-free; any exception is narrow, documented and tested.

**Evidence lane:** Build/security review. **Initial status:** not run.

## AC052 — Versioned migration (R52)

**Given:** Unsupported schema/ABI or interrupted DB migration occurs.

**When:** Read/migrate/recover/replay.

**Required result:** No silent alias/reinterpretation or signed-byte rewrite; compatibility fails explicitly.

**Evidence lane:** Migration/package. **Initial status:** not run.

## AC053 — Known-good commits (R53)

**Given:** A proposed step changes behavior/schema/dependencies.

**When:** Run tests/gates and review before committing.

**Required result:** Step is green relative to reproduced baseline; introduced failures block advancement.

**Evidence lane:** Verification process. **Initial status:** not run.

## AC054 — Shadow rollout and rollback (R54)

**Given:** Candidate evaluator runs before enforcement or must be rolled back.

**When:** Compare decisions and switch under authorized procedure.

**Required result:** Independent review precedes enforcement; rollback never becomes permissive.

**Evidence lane:** Operational drill. **Initial status:** not run.

## AC055 — Owned guarantees (R55)

**Given:** Release support and incident procedures are proposed.

**When:** Review owner/matrix/runbook evidence.

**Required result:** Named owners and tested supported guarantees exist before operational approval.

**Evidence lane:** Operational review. **Initial status:** not run.

## AC056 — Spec-anchored sequencing (R56)

**Given:** An agent resumes at a pending step with an apparent shortcut.

**When:** Inspect plan/spec and repo evidence.

**Required result:** No silent skip/merge/reorder/scope invention; deviations documented only when justified.

**Evidence lane:** Handoff/process validation. **Initial status:** not run.

## AC057 — Honest readiness (R57)

**Given:** A native/crypto/external lane was not run.

**When:** Prepare completion report.

**Required result:** Unverified gates remain explicit; no production-ready or passing claim is fabricated.

**Evidence lane:** Review. **Initial status:** not run.

## AC058 — Exact selection (R58)

**Given:** Duplicate profile requests or candidate-local release defaults are supplied.

**When:** Select policy/profiles.

**Required result:** Duplicates reject; protected mode uses explicit exact authority without fallback.

**Evidence lane:** Unit/security. **Initial status:** not run.

## AC059 — Inventory authority (R59)

**Given:** Partial/untrusted inventory or self-declared artifact roles are supplied.

**When:** Perform structure/exposure evaluation.

**Required result:** Absence/roles are not falsely proven; incomplete observation stays unknown.

**Evidence lane:** Unit/security. **Initial status:** not run.

## AC060 — Acquisition outcome (R60)

**Given:** Runner exits zero with an empty plan or inadequate policy count.

**When:** Render verify and evaluate.

**Required result:** Acquisition and conformance remain distinct; accidental empty work is visible failure.

**Evidence lane:** CLI integration. **Initial status:** not run.

## AC061 — Repository gates preserved (R61)

**Given:** Approved spec docs/platform changes coexist with private/unsafe artifacts.

**When:** Run public-repository/xtask checks.

**Required result:** Legitimate contract files are admitted without discarding meaningful safety checks.

**Evidence lane:** Xtask/package. **Initial status:** not run.

## AC062 — Per-step evidence (R62)

**Given:** A commit-sized change has been reviewed.

**When:** Record its report and commit.

**Required result:** Files, exact checks/results, risks, SHA and next-step safety are available without circular self-SHA edits.

**Evidence lane:** Process validation. **Initial status:** not run.

## AC063 — Deployment consumes exact authorization (R63)

**Given:** Artifact/destination or validity differs after readiness/signing.

**When:** Validate authorization at promotion boundary.

**Required result:** Substitution/expiry rejects; transformed artifacts require provenance or reevaluation.

**Evidence lane:** Release integration. **Initial status:** not run.

## AC064 — Boundary distinctions (R64)

**Given:** Valid JSON from storage claims trusted complete release state.

**When:** Send through public input APIs.

**Required result:** Local validation, verification, completeness and authorization remain independent checks.

**Evidence lane:** Security/property. **Initial status:** not run.

## Approved regression obligations

RA01–RA21 in [the full-source review](../repo/REVIEW_FINDINGS.md#approved-full-source-regression-clauses--revision-2) and [machine catalog](../implementation/review_regressions.json) are required clauses of the mapped AC cases. Their status is not_run. A green baseline suite or a static review does not close them. Record concrete test names, commands, host/mode and results; retain the complete existing AC001–AC064 scope.
