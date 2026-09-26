# Review findings and pressure-test amendments

All findings below originate in the conversation's static source review at the pinned baseline. They are review observations, not reproduced exploit/test results. Revalidate at implementation HEAD. A documented current fix can make a planned regression/fix step obsolete, but does not permit silent step skipping. Several release risks are currently masked by fail-closed unconfigured trust; repair them before enabling real verified release acceptance. Severity P1 means fix before relying on the workflow, not necessarily an externally exploitable vulnerability.

## Baseline revalidation at S004

**Revalidated:** 2026-09-26 during S004. **Source HEAD:** `7c8293837c09c627489cdf244a2fbd97b60aa8ed` (the reviewed baseline; S001–S003 changed only repository admission, imported specifications, and agent guidance, so no crate source moved).

- The actual standard gate was run on the reviewed baseline and passed: `cargo xtask check` completed with 265 tests. Step gates for S001–S003 also passed. This is local baseline revalidation, not refactor acceptance.
- Every F01–F20 source path still exists and every RA01–RA21 `source_anchors` path resolves within its file at HEAD. No anchor drift was introduced by the admission/import/guidance work.
- Every RA01–RA21 clause has requirements, acceptance cases, owning steps, a precise regression, and a `given`/`when`/`expected` contract. F12, F19, and F20 intentionally carry no dedicated regression clause; their coverage is carried by their mapped owning steps and requirements.
- F19 (repository gates restrict legitimate integration changes) is partially addressed by S001, which narrowed the documentation and private-marker admission while retaining the `.github`/`scripts`, generated-state, schema-parity and legacy-name protections. Its remaining integration-admission scope stays open until the named owning steps land.
- This reconciliation is static and documentary. Every AC001–AC064 and RA01–RA21 status remains `not_run`; no finding is closed by a plan edit or by a green build. Runtime reproduction and acceptance evidence remain assigned to the owning implementation steps.

## F01 — Read-only views do not evaluate stored evidence

**Priority:** P1. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R04,R44,R46.

check unconditionally reports derived obligations missing; conformance/evidence matrices hardcode missing; MCP context/findings resources are incomplete placeholders. Build a shared evaluated snapshot and make every view render it.

**Source anchors:** [crates/eqm_cli/src/commands/check.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/check.rs); [crates/eqm_cli/src/commands/matrix.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/matrix.rs); [crates/eqm_mcp/src/resources.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_mcp/src/resources.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F02 — Production trust and signing are not wired

**Priority:** P1. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R24,R25,R26,R27.

Release execution uses UnconfiguredTrust while the engine requires signed CI trust; attest rejects a signer and deliberately reports unknown conformance. Safe failure is not implemented trusted release capability. Connect a real configured verifier and explicit signer only after the other release defects are fixed.

**Source anchors:** [crates/eqm_cli/src/commands/release_check.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/release_check.rs); [crates/eqm_cli/src/commands/attest.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/attest.rs); [crates/eqm_runner/src/ci.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/ci.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F03 — Release selection uses development authority

**Priority:** P1. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R23,R58.

The release command invokes development derive, selecting CandidateLocal/Development and no exact release context instead of protected release authority. Preserve the engine selection boundary and protected explicit profiles.

**Source anchors:** [crates/eqm_cli/src/commands/evaluation.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/evaluation.rs); [crates/eqm_cli/src/commands/release_check.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/release_check.rs); [crates/eqm_engine/src/selection.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/selection.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F04 — Release scope drops provider and end-to-end obligations

**Priority:** P1. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R11,R12,R13.

obligation_statuses filters only Target scope and then supplies completeness true. Evaluate all required scope types and unknown applicability using a validated universe. Empty input and singleton equivalence must not be mistaken for meaningful assurance.

**Source anchors:** [crates/eqm_cli/src/commands/release_check.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/release_check.rs); [crates/eqm_engine/src/conformance.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/conformance.rs); [crates/eqm_engine/src/obligations.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/obligations.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F05 — Release reimplements inconsistent evidence semantics

**Priority:** P1. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R14,R15,R16.

Release status code checks limited counts, can fall through to satisfied for non-executable payloads, ignores important retry/skip states and counts records rather than canonical observations. Canonical engine ordering differs. Delete duplicate semantics after a normative regression-tested replacement.

**Source anchors:** [crates/eqm_cli/src/commands/release_check.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/release_check.rs); [crates/eqm_engine/src/outcomes.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/outcomes.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F06 — Release exposure and trust completeness are overstated

**Priority:** P1. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R24,R28,R29,R44.

Presence of an exact runtime document is treated as exposure match; optional verification values can be discarded by aggregation; waivers are empty and equivalence hardcoded unknown. Compare actual facts, require every mandatory receipt and evaluate only contributing waivers/comparison inputs.

**Source anchors:** [crates/eqm_cli/src/commands/release_check.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/release_check.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F07 — Runner timeout does not cover whole invocation

**Priority:** P1. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R31,R32.

After the immediate child exits, blocking reader joins can outlive the deadline while descendants hold pipes open. Local output overflow is not signaled promptly; non-Unix tree termination is not equivalent. Supervise descendants/drainage/cancellation/overflow under the entire lifecycle budget.

**Source anchors:** [crates/eqm_runner/src/execution.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/execution.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F08 — Filesystem bounds and observed roles are inconsistent

**Priority:** P1. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R33,R59.

verify reads a result file before bounding it, persistence reads an existing collision file without a prior cap, and structural leaf metadata can miss ancestor symlinks. The CLI copies authored artifact roles into observed roles. Centralize confined bounded access and independent observations.

**Source anchors:** [crates/eqm_cli/src/commands/verify.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/verify.rs); [crates/eqm_cli/src/commands/check.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/check.rs); [crates/eqm_runner/src/persistence.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/persistence.rs); [crates/eqm_engine/src/structure.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/structure.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F09 — Evidence provenance uses incomplete digests

**Priority:** P1. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R19,R22.

Specification and binding fields use an ID-pair hash, several semantic fields reuse workspace digest, and resolved runner evidence identity omits executable bytes. Local HEAD provenance does not bind dirty working content. Global workspace hashing catches many edits but does not repair imprecise component identity.

**Source anchors:** [crates/eqm_cli/src/commands/verify.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/verify.rs); [crates/eqm_runner/src/resolution.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/resolution.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F10 — Single roots and policy-wide target fan-out constrain KMP roles

**Priority:** High product fit. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R07,R08,R09.

Duplicate roots are rejected, artifacts are tied to one target root, and each_target fans out over a policy-wide target set. Separate shipped outputs from shared-source ownership and model role-aware participation/cohorts. Intentional asymmetry is not a waiver.

**Source anchors:** [crates/eqm_manifest/src/loader.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_manifest/src/loader.rs); [crates/eqm_engine/src/structure.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/structure.rs); [crates/eqm_engine/src/obligations.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/obligations.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F11 — verify --affected does not filter actual work

**Priority:** Completion. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R40.

A baseline is prepared and its digest discarded; the ordinary plan is then executed. Wire conservative reverse dependency impact into plan selection and approval.

**Source anchors:** [crates/eqm_cli/src/commands/verify.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/verify.rs); [crates/eqm_engine/src/affected.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/affected.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F12 — Dry-run is not the promised concrete execution plan

**Priority:** Safety/completion. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R30.

Dry-run reports selected binding/specification identifiers before full resolution. It must show safe exact executable/argv/cwd/environment-reference/limit/output details without running anything.

**Source anchors:** [crates/eqm_cli/src/commands/verify.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/verify.rs); [docs/evidence-and-trust.md](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/docs/evidence-and-trust.md).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F13 — Verification acquisition success can overstate conformance

**Priority:** Correctness. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R16,R60.

verify uses specification minimum counts rather than necessarily stronger policy thresholds, and empty plans may succeed. Separate selected-run completion from complete policy acceptance and reject accidental no-op selection.

**Source anchors:** [crates/eqm_cli/src/commands/verify.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/verify.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F14 — Public obligation IDs omit profiles and release context

**Priority:** Identity/API. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R19,R58.

Displayed IDs omit context included in internal keys. Selection also assumes one development policy and duplicate profile requests can overwrite each other. Bind complete public coordinates and exact explicit revision requests with duplicate rejection.

**Source anchors:** [crates/eqm_cli/src/commands/evaluation.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/evaluation.rs); [crates/eqm_engine/src/obligations.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/obligations.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F15 — Error propagation discards useful context

**Priority:** API/code health. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R45,R50.

Session loading replaces detailed errors with stage-only variants; main routes failures to stderr rather than requested machine envelopes. Preserve typed causal/source information and documented machine error behavior.

**Source anchors:** [crates/eqm_cli/src/session.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/session.rs); [crates/eqm_manifest/src/loader.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_manifest/src/loader.rs); [crates/eqm_cli/src/main.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/main.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F16 — CI replay guard is ephemeral and keyed too coarsely

**Priority:** Replay/storage. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R17,R18,R35.

Invocation-local CI-run identity both forgets history across restarts and rejects multiple legitimate evidence results from one run. Persist precise result/attempt identities with idempotent imports and explicit conflicts.

**Source anchors:** [crates/eqm_runner/src/ci.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/ci.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F17 — Host execution integration is incomplete

**Priority:** Portability/completion. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R30,R31,R32,R34,R41.

CLI selects local execution, hardcodes a Unix PATH, supplies no secrets, lacks caller cancellation wiring and runs sequentially. Add explicit host capability/configuration and bounded scheduling without assuming every backend or host is supported.

**Source anchors:** [crates/eqm_cli/src/commands/verify.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/verify.rs); [crates/eqm_runner/src/execution.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/execution.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F18 — Concurrent store initialization and host durability need work

**Priority:** Storage. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R35,R36.

Check-then-create directory initialization can fail under concurrent first writers; platform permission/sync behavior needs native validation. Add revalidation, durable object/index publication and crash/concurrency tests.

**Source anchors:** [crates/eqm_runner/src/persistence.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/persistence.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F19 — Repository gates restrict legitimate integration changes

**Priority:** Governance. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R53,R61.

xtask prohibits .github/scripts, exact docs inventory and generic private words. Narrowly admit approved docs/platform work while retaining safety and explicit CI selection. No observed GitHub Actions run is not proof there is no external CI.

**Source anchors:** [tools/xtask/src/main.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/tools/xtask/src/main.rs); [CONTRIBUTING.md](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/CONTRIBUTING.md); [AGENTS.md](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/AGENTS.md).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## F20 — Benchmarks and some tests validate incomplete paths

**Priority:** Performance/testing. **Status:** open / revalidate; not runtime-reproduced. **Contracts:** R43,R53.

Warm context uses largely empty resources; cold validation excludes canonicalization; evidence/trust/history/profile/matrix workloads are incomplete. Some command tests assert missing/nonempty placeholders. Benchmark real completed work and test the full lifecycle with a reference oracle.

**Source anchors:** [tools/benchmarks/src/main.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/tools/benchmarks/src/main.rs); [crates/eqm_mcp/src/resources.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_mcp/src/resources.rs); [crates/eqm_cli/src/commands/check.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/check.rs); [crates/eqm_cli/src/commands/matrix.rs](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/matrix.rs).

**Closure evidence:** regression and acceptance cases for the listed requirements, plus removal/replacement of the identified path. Traceability maps these to planned steps.

## Pressure-test amendments that supersede ambiguous earlier recommendations

- **P01 (R14,R43):** A shared evaluator is not automatically correct; establish normative status/count/facet semantics and a simple reference oracle.
- **P02 (R08,R11,R12):** Completeness is validated accounting, not success; unknown applicability and empty scopes cannot disappear.
- **P03 (R07,R09):** Separate comparison context from exact per-participant build/artifact identity and support approved mixed-version cohorts.
- **P04 (R07,R13):** Use role-bearing participant occurrences for repeated devices and exact provider dependencies.
- **P05 (R10):** Equivalence is contract-relative; required relational checks cannot be replaced with independent target passes.
- **P06 (R23):** Protect the entire expectation chain including grader, participation, contracts, selections and deletions.
- **P07 (R24,R25,R26):** Signature validity, signer/issuer authorization, builder/subject and temporal material are separate checks.
- **P08 (R27,R63):** Keep signing, authorization and deployment consumption separate; a signed failure stays a failure.
- **P09 (R20,R28):** Use an acyclic candidate-plan-evidence-input-decision-statement-authorization flow and separate pre/post-deployment evaluation.
- **P10 (R18):** Distinguish raw, semantic and attempt identities; duplicate transport is idempotent, contradictory attempts are conflicts.
- **P11 (R16):** Expected selected observations determine counts; files and retries cannot inflate evidence.
- **P12 (R17):** Preserve immutable campaign history without arbitrary timestamp selection or permanent poisoning by unrelated old failures.
- **P13 (R21,R22):** Freeze inputs consistently and retain original signed bytes; canonical JSON and path normalization are different concerns.
- **P14 (R35,R36):** Publish immutable objects before transactional references; retain recoverable orphans rather than phantom completed rows.
- **P15 (R38,R39):** Make historical replay distinct from current trust/time acceptance and cache validity.
- **P16 (R30,R31,R32,R33):** Supervise complete worker lifecycles and treat executable discovery as privileged acquisition; path confinement is not sandboxing.
- **P17 (R42,R46):** Bound parsing, graph expansion, profile products, evidence, attachments, queues and report requests; no silent scope truncation.
- **P18 (R47):** Keep MCP read-only/restricted and untrusted text non-authoritative; no default localhost server.
- **P19 (R03,R49,R51):** Default to developer/CI deployment; native embedding/sidecars are conditional with proven FFI/lifecycle/packaging contracts.
- **P20 (R52):** Version wire/evaluator/ABI/database separately and preserve current-only boundaries until an explicit migration decision.
- **P21 (R39,R40,R41,R43):** Optimize measured bottlenecks with indexes/caches/batches and reference parity, not incomplete benchmark headlines.
- **P22 (R53,R54,R55,R57):** Shadow rollout, independently reviewed enforcement and owned recovery are readiness requirements; approved design is not tested production code.

## Approved full-source regression clauses — revision 2

The owner approved RA01–RA21 on 2026-09-26. They refine F01–F20 and are mandatory regression contracts, not claims of runtime reproduction. The machine source is [review_regressions.json](../implementation/review_regressions.json). Each linked implementation step owns its contribution; final closure needs actual tests/evidence. Source anchors refer to the reviewed baseline and must be revalidated at implementation HEAD.

### RA01 — Validate filters and empty selections

**Findings:** F04, F13. **Requirements:** R11, R12, R58, R60. **Acceptance:** AC011, AC012, AC058, AC060.

**Owning contributions:** S047, S048, S049, S050, S145, S149, S151, S155. **Status:** not_run.

**Given:** An exact scope with known targets/units and unresolved applicability.

**When:** Select a misspelled target/unit, an accidental empty execution scope, or filter a complete decision.

**Expected:** Reject invalid selectors before effects; retain unresolved coordinates and the protected denominator. Empty release scopes need protected explicit permission and do not establish cross-target equivalence.

**Source evidence:** [crates/eqm_cli/src/commands/check.rs:37](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/check.rs#L37); [crates/eqm_cli/src/commands/verify.rs:54](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/verify.rs#L54).

### RA02 — Validate counts against outcomes

**Findings:** F05, F13. **Requirements:** R14, R15, R16. **Acceptance:** AC014, AC015, AC016.

**Owning contributions:** S025, S059, S062, S063, S064, S065, S066. **Status:** not_run.

**Given:** Arithmetically valid totals containing failed cases or a skipped attempt claiming passing cases.

**When:** Convert and aggregate each supported execution payload.

**Expected:** Reject contradictory totals/outcomes; failed required cases cannot satisfy a minimum merely because another case passed. Invalid required evidence remains unresolved/non-authorizing.

**Source evidence:** [crates/eqm_domain/src/evidence_result.rs:405](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_domain/src/evidence_result.rs#L405); [crates/eqm_domain/src/evidence_result.rs:432](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_domain/src/evidence_result.rs#L432).

### RA03 — Separate conjunction, failures, retries and conflicts

**Findings:** F05, F13, F16. **Requirements:** R14, R15, R16, R17, R18. **Acceptance:** AC014, AC015, AC016, AC017, AC018.

**Owning contributions:** S053, S060, S061, S062, S063, S064, S065, S066, S067, S068, S072, S075, S076. **Status:** not_run.

**Given:** A frozen required selection, explicit alternative groups, multiple cases, and immutable attempt history.

**When:** Combine passing/missing cases, separate failing/passing cases, retries, or same-attempt contradictory assertions.

**Expected:** Required selections are conjunctive; only protected explicit alternatives may substitute. Distinct required fail/pass cases yield Failed; same-case fail/pass retries yield Unstable; same immutable attempt conflicts yield Unknown. Skips/filter/quarantine/missing never satisfy; cancellation/timeout is Unknown. Copies and retries add no count credit; selected campaign identity is explicit.

**Source evidence:** [crates/eqm_engine/src/outcomes.rs:35](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/outcomes.rs#L35); [crates/eqm_engine/src/outcomes.rs:74](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/outcomes.rs#L74).

### RA04 — Preserve the protected universe when strengthening policy

**Findings:** F03, F05. **Requirements:** R14, R23, R58. **Acceptance:** AC014, AC023, AC058.

**Owning contributions:** S051, S057, S118. **Status:** not_run.

**Given:** A protected policy selecting Recommended and Required obligations.

**When:** Raise the minimum-level selection floor or otherwise change matching rules.

**Expected:** Compare derived protected obligation inclusion and same-coordinate constraints. Removing a protected obligation is weakening even if an enum comparison says stronger.

**Source evidence:** [crates/eqm_domain/src/policy.rs:297](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_domain/src/policy.rs#L297); [crates/eqm_engine/src/obligations.rs:120](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/obligations.rs#L120).

### RA05 — Preserve fragment risk through validated expansion

**Findings:** F03, F10. **Requirements:** R14, R23, R50. **Acceptance:** AC014, AC023, AC050.

**Owning contributions:** S004, S006, S049, S057, S076. **Status:** not_run.

**Given:** A fragment and destination journey with different default risks, with absent or explicit requirement risk.

**When:** Expand a pinned fragment and prepare obligations.

**Expected:** Inherited risk is the stronger fragment/journey default; absence inherits it, explicit lower risk rejects, and the expanded graph is revalidated before finalization. DEV-001 explicitly amends S049; S004/S006 record/freeze this contract, not its runtime implementation.

**Source evidence:** [crates/eqm_engine/src/expand.rs:20](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/expand.rs#L20); [crates/eqm_engine/src/expand.rs:132](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/expand.rs#L132); [crates/eqm_engine/src/expand.rs:158](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/expand.rs#L158).

### RA06 — Enforce validated construction boundaries

**Findings:** F02, F04. **Requirements:** R11, R24, R26, R50, R63. **Acceptance:** AC011, AC024, AC026, AC050, AC063.

**Owning contributions:** S049, S070, S120, S133, S172. **Status:** not_run.

**Given:** Public graph/preparation/verification/authorization APIs and serialized inputs.

**When:** Attempt unchecked construction, deserialization, omitted coordinates or manually asserted verified/completeness flags.

**Expected:** Only the appropriate checked boundary constructs these states. Complete failing/unknown decisions remain representable. API/compile-fail and runtime tests cover bypasses without claiming protection against malicious same-privilege code.

**Source evidence:** [crates/eqm_domain/src/graph.rs:302](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_domain/src/graph.rs#L302); [crates/eqm_engine/src/conformance.rs:141](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/conformance.rs#L141).

### RA07 — Compose only facet-relevant constraints

**Findings:** F05. **Requirements:** R14, R58. **Acceptance:** AC014, AC058.

**Owning contributions:** S051, S057. **Status:** not_run.

**Given:** Behavior and accessibility rules with different trust/count/age constraints.

**When:** Derive each facet obligation.

**Expected:** Each facet combines only its matching rules; unrelated facet strength does not leak across obligations.

**Source evidence:** [crates/eqm_engine/src/obligations.rs:130](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/obligations.rs#L130).

### RA08 — Make duplicate transport idempotent and conflicts relevant

**Findings:** F05, F16. **Requirements:** R17, R18, R35. **Acceptance:** AC017, AC018, AC035.

**Owning contributions:** S060, S061, S076, S084. **Status:** not_run.

**Given:** Identical delivery, irrelevant rejected duplicates, independent results from one CI run, and conflicting same-attempt assertions.

**When:** Normalize/import repeatedly and reopen the store.

**Expected:** Identical delivery changes neither counts nor conclusions; unrelated rejected duplicates do not poison coverage; independent results from one run remain valid; relevant authenticated immutable conflicts persist across restart.

**Source evidence:** [crates/eqm_engine/src/coverage.rs:121](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/coverage.rs#L121); [crates/eqm_runner/src/ci.rs:63](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/ci.rs#L63).

### RA09 — Bind exact contributing waivers

**Findings:** F06. **Requirements:** R19, R29, R58. **Acceptance:** AC019, AC029, AC058.

**Owning contributions:** S071, S128, S166, S170. **Status:** not_run.

**Given:** A waiver and obligations sharing names but differing in policy/profile revision, subject or release context.

**When:** Evaluate and emit a decision/statement.

**Expected:** Only exact authorized scope/context matches apply; list only contributing waivers. Unknown and unstable remain non-waivable; expired or wrong-authority waivers do not contribute.

**Source evidence:** [crates/eqm_engine/src/waivers.rs:119](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/waivers.rs#L119); [crates/eqm_cli/src/commands/attest.rs:127](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/attest.rs#L127).

### RA10 — Fall back conservatively from incomplete impact indexes

**Findings:** F11. **Requirements:** R40, R43. **Acceptance:** AC040, AC043.

**Owning contributions:** S179, S180, S181, S189. **Status:** not_run.

**Given:** Missing/partial dependency indexes, removed/renamed paths, new targets and unclassified changes.

**When:** Compute affected plans and optimized decisions.

**Expected:** Select the conservatively complete fallback instead of returning no work; optimized evaluation equals reference evaluation for the same frozen input.

**Source evidence:** [crates/eqm_engine/src/affected.rs:60](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/affected.rs#L60); [crates/eqm_engine/src/affected.rs:91](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_engine/src/affected.rs#L91).

### RA11 — Supervise input, output and descendants through cleanup

**Findings:** F07, F17. **Requirements:** R31, R32. **Acceptance:** AC031, AC032.

**Owning contributions:** S104, S105, S106, S107, S115. **Status:** not_run.

**Given:** A request above pipe capacity; workers refusing input, writing before reading, breaking stdin, flooding output or exiting while descendants retain pipes.

**When:** Execute, cancel or reach a deadline during input/drain/cleanup.

**Expected:** Supervision starts before potentially blocking input; all owned input/output/tasks/descendants terminate and join within the declared deadline/shutdown budget. Broken-pipe and every error path clean up; overflow signals prompt termination.

**Source evidence:** [crates/eqm_runner/src/adapter.rs:78](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/adapter.rs#L78); [crates/eqm_runner/src/adapter.rs:125](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/adapter.rs#L125); [crates/eqm_runner/src/execution.rs:169](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/execution.rs#L169).

### RA12 — Bound and confine every file ingress

**Findings:** F08, F18. **Requirements:** R33, R35, R42. **Acceptance:** AC033, AC035, AC042.

**Owning contributions:** S079, S080, S081, S100, S101, S114, S115. **Status:** not_run.

**Given:** Config/lock/source/executable/result/import/collision inputs including oversized regular files, FIFO/devices, ancestor symlinks, replacement and growth races.

**When:** Read through every ingestion path on supported hosts.

**Expected:** Bound allocation and reads using the validated opened handle and host confinement rules; reject unsupported file types/races before unsafe use. Path metadata followed by unbounded reopen is insufficient; no mutation before confinement validation.

**Source evidence:** [crates/eqm_manifest/src/config.rs:76](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_manifest/src/config.rs#L76); [crates/eqm_manifest/src/lockfile.rs:82](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_manifest/src/lockfile.rs#L82); [crates/eqm_manifest/src/validation.rs:105](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_manifest/src/validation.rs#L105); [crates/eqm_cli/src/commands/verify.rs:135](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/verify.rs#L135).

### RA13 — Validate evidence relationships at a single boundary

**Findings:** F05, F09. **Requirements:** R18, R19, R21, R64. **Acceptance:** AC018, AC019, AC021, AC064.

**Owning contributions:** S014, S015, S016, S025, S059, S061. **Status:** not_run.

**Given:** Payloads with one-axis mismatches in kind/subject/target/unit/requirement/facet/reference digests or temporal lineage.

**When:** Ingest through import, persistence and evaluation entry points.

**Expected:** All entry points use the same checked conversion and reject contradictions before admission. Parsing or a self-consistent transport hash never creates verification authority.

**Source evidence:** [crates/eqm_protocol/src/evidence.rs:413](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_protocol/src/evidence.rs#L413); [crates/eqm_runner/src/persistence.rs:74](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/persistence.rs#L74).

### RA14 — Validate real command outputs against schemas

**Findings:** F01, F15. **Requirements:** R21, R44, R45, R52. **Acceptance:** AC021, AC044, AC045, AC052.

**Owning contributions:** S058, S143, S152, S155, S163, S204. **Status:** not_run.

**Given:** Representative actual success and failure outputs for every public command family and MCP response shape.

**When:** Generate public schemas and validate those outputs against the correct schema.

**Expected:** Every advertised result/error shape validates; a validate-only generic result schema cannot stand in for all commands. DTOs, generators, URI/version checks, fixtures and consumers change together.

**Source evidence:** [crates/eqm_protocol/src/bin/generate_protocol_schemas.rs:74](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_protocol/src/bin/generate_protocol_schemas.rs#L74); [schemas/v1/protocol/result.schema.json:496](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/schemas/v1/protocol/result.schema.json#L496).

### RA15 — Separate exact bytes from semantic persistence identity

**Findings:** F09, F16, F18. **Requirements:** R18, R22, R35. **Acceptance:** AC018, AC022, AC035.

**Owning contributions:** S023, S060, S080, S081, S082, S083, S084. **Status:** not_run.

**Given:** Equivalent assertions encoded with distinct JSON whitespace/member order and identical logical attempts.

**When:** Persist both byte representations and reopen/import again.

**Expected:** Preserve both original raw identities/material while sharing semantic/attempt identity; no false immutable-byte collision, duplicated observation or lost provenance.

**Source evidence:** [crates/eqm_runner/src/persistence.rs:85](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/persistence.rs#L85); [crates/eqm_runner/src/persistence.rs:129](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/persistence.rs#L129).

### RA16 — Load and select exact authority revisions

**Findings:** F03, F14. **Requirements:** R58. **Acceptance:** AC058.

**Owning contributions:** S046. **Status:** not_run.

**Given:** Two authored revisions for one policy/profile ID, and a duplicate definition of one exact revision.

**When:** Load through the real manifest loader and select an exact revision.

**Expected:** Distinct revisions load and resolve exactly; duplicate authority for the same revision rejects. A constructed-graph-only test is insufficient.

**Source evidence:** [crates/eqm_manifest/src/validation.rs:55](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_manifest/src/validation.rs#L55); [crates/eqm_manifest/src/validation.rs:129](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_manifest/src/validation.rs#L129).

### RA17 — Preserve one MCP session authority and snapshot

**Findings:** F01. **Requirements:** R04, R44, R47, R58. **Acceptance:** AC004, AC044, AC047, AC058.

**Owning contributions:** S158, S159, S160, S164. **Status:** not_run.

**Given:** Nondefault startup config/profiles/authority and a workspace modified after session preparation.

**When:** Read tools/resources and explicitly refresh.

**Expected:** Tools and resources refer to the same selected snapshot and authority; startup globals are retained; generation changes are explicit and stale cursors reject.

**Source evidence:** [crates/eqm_cli/src/commands/mcp.rs:12](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/mcp.rs#L12); [crates/eqm_cli/src/commands/mcp.rs:54](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/mcp.rs#L54).

### RA18 — Bound complete transport envelopes and queues

**Findings:** F01, F17. **Requirements:** R42, R44, R46, R47. **Acceptance:** AC042, AC044, AC046, AC047.

**Owning contributions:** S114, S141, S148, S162. **Status:** not_run.

**Given:** Outputs near byte limits plus envelope overhead, large resources and queued requests.

**When:** Serialize CLI/MCP responses and paginate snapshots.

**Expected:** Limits cover the full serialized envelope and queues; pagination/truncation is explicit and snapshot-bound. Required scope/conclusions never shrink to fit; even limit errors obey a validated minimum response budget.

**Source evidence:** [crates/eqm_mcp/src/server.rs:283](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_mcp/src/server.rs#L283); [crates/eqm_cli/src/commands/context.rs:162](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/context.rs#L162).

### RA19 — Diagnose consumer requirements and valid store states

**Findings:** F01. **Requirements:** R44, R48, R53. **Acceptance:** AC044, AC048, AC053.

**Owning contributions:** S154, S155, S157, S200, S206. **Status:** not_run.

**Given:** An unmodified non-Rust consumer and generated store/work directories produced by supported operations.

**When:** Run consumer doctor without copying EQM contributor toolchain files into the fixture.

**Expected:** Validate actual consumer prerequisites and legitimate generated states; contributor Rust version/components are not consumer requirements.

**Source evidence:** [crates/eqm_cli/src/commands/doctor.rs:142](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/doctor.rs#L142); [crates/eqm_cli/src/commands/doctor.rs:191](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/doctor.rs#L191); [crates/eqm_cli/src/test_support.rs:11](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/test_support.rs#L11).

### RA20 — Validate formats before effects and preserve receipts on output failure

**Findings:** F13, F15. **Requirements:** R05, R44, R45, R60. **Acceptance:** AC005, AC044, AC045, AC060.

**Owning contributions:** S152, S154, S155. **Status:** not_run.

**Given:** Every accepted command/output-format combination, invalid usage and an output sink failing after durable acquisition.

**When:** Parse, execute and render with effect spies and receipt lookup.

**Expected:** Unsupported formats reject before execution/storage; accepted formats have renderers; requested machine errors use documented envelopes. Post-commit output failure truthfully reports the failure and leaves the committed operation discoverable without pretending rollback.

**Source evidence:** [crates/eqm_cli/src/cli.rs:605](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/cli.rs#L605); [crates/eqm_cli/src/commands/verify.rs:403](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/verify.rs#L403); [crates/eqm_cli/src/main.rs:48](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/main.rs#L48).

### RA21 — Require independently verified current observations

**Findings:** F06, F08. **Requirements:** R24, R28, R59. **Acceptance:** AC024, AC028, AC059.

**Owning contributions:** S055, S056, S126, S153, S196. **Status:** not_run.

**Given:** Complete but stale/unsigned/wrong-subject/revoked inventory and authored role declarations.

**When:** Reconcile absence and compare runtime/structural expectations.

**Expected:** Only current independently verified complete inventory for an independently expected subject proves absence. Partial/missing/untrusted inventory is Unknown. Roles and exposure facts require independent observation/comparison, not document presence or copied declarations.

**Source evidence:** [crates/eqm_runner/src/inventory.rs:79](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/inventory.rs#L79); [crates/eqm_runner/src/reconciliation.rs:41](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_runner/src/reconciliation.rs#L41); [crates/eqm_cli/src/commands/reconcile.rs:174](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/reconcile.rs#L174); [crates/eqm_cli/src/commands/check.rs:158](https://github.com/triesap/eqm/blob/7c8293837c09c627489cdf244a2fbd97b60aa8ed/crates/eqm_cli/src/commands/check.rs#L158).
