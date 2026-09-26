# Chapter 09: Frozen inputs decisions and coherent query service

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P09, containing 12 separate ordered commits, S132–S143; it is not one combined commit. All implementation steps are pending.

**Definition of green:** Frozen manifests, immutable decisions, replay/current-acceptance distinction and coherent bounded queries agree.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q04, Q07. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s132"></a>
## S132 — Assemble frozen input manifests

**Predecessor:** S131. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Assemble frozen input manifests.

### 2. Purpose
Connect coherent storage authority and subjects to evaluation.

### 3. Exact scope of code changes
Capture exact authority, universe, campaigns, subject manifest, evidence/runtime objects, verification material, evaluator and explicit time from one consistent generation.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Concurrent imports/config changes cannot mix inputs; missing referenced objects prevent successful preparation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
A decision can be reproduced from explicit immutable inputs.

### 8. Commit message
```text
feat: assemble frozen conformance input manifests
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s133"></a>
## S133 — Validate prepared evaluation construction

**Predecessor:** S132. **Contracts:** R04, R05, R11, R22, R24, R26, R38, R44, R45, R46, R50, R63. **Acceptance:** AC004, AC005, AC011, AC022, AC024, AC026, AC038, AC044, AC045, AC046, AC050, AC063.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate prepared evaluation construction.

### 2. Purpose
Prevent partial or unverified maps entering the main evaluator.

### 3. Exact scope of code changes
Combine validated completeness with checked input conversions and mandatory verification facts in a private application preparation boundary; no serde/public boolean bypass.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Missing provider/profile/receipt and forged prepared-input fields cannot produce a trusted complete decision.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Prepared inputs prove accounting completeness without implying success.

### 8. Commit message
```text
feat: validate prepared conformance inputs
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s134"></a>
## S134 — Run the canonical evaluator through application

**Predecessor:** S133. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Run the canonical evaluator through application.

### 2. Purpose
Expose one whole-evaluation operation to adapters.

### 3. Exact scope of code changes
Compose the reference semantics into the application evaluate operation with no new policy logic; return structured results and safe errors for explicit requests.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Valid, missing, stale, failed, unstable, waived and wrong-subject fixtures match reference outcomes; no acquisition spies fire.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
All adapters can invoke one normative evaluation path.

### 8. Commit message
```text
feat: expose shared conformance evaluation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s135"></a>
## S135 — Produce immutable decision snapshots

**Predecessor:** S134. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Produce immutable decision snapshots.

### 2. Purpose
Define stable exact decision identity and complete explanations.

### 3. Exact scope of code changes
Map evaluation outputs to immutable snapshot records with full scope/input/evaluator/time identities, accepted/rejected reasons, applied waivers and aggregate conclusions.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Identical inputs/time reproduce the same semantic decision; changed inputs/time/context are explicit rather than overwritten.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Decisions are replayable immutable records instead of mutable dashboard state.

### 8. Commit message
```text
feat: produce immutable evaluation snapshots
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s136"></a>
## S136 — Separate read-only and writable session access

**Predecessor:** S135. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Separate read-only and writable session access.

### 2. Purpose
Keep inspection from creating generated state or running tests.

### 3. Exact scope of code changes
Provide read-only store/session access that reports absent evidence without initializing DB/directories; explicit acquisition/publication operations retain write permission.

### 4. Files/modules likely involved
- `crates/eqm_application/src/session.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
check/query preparation on a missing .eqm tree does not create it; read-only callers cannot publish or acquire.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Query/evaluation use cases preserve declared side-effect boundaries.

### 8. Commit message
```text
fix: preserve read-only evaluation sessions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s137"></a>
## S137 — Persist decision manifests explicitly

**Predecessor:** S136. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Persist decision manifests explicitly.

### 2. Purpose
Make archived decisions durable without implicit query writes.

### 3. Exact scope of code changes
Add an explicitly authorized publication operation for input/decision manifests using object-first transactions and idempotent IDs; leave ordinary read-only queries non-mutating.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Crash/reopen and duplicate publication retain exact decision references; failed persistence cannot appear as archived success.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Historical decisions are durable only through a visible publication boundary.

### 8. Commit message
```text
feat: archive exact evaluation decisions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s138"></a>
## S138 — Replay historical snapshots

**Predecessor:** S137. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Replay historical snapshots.

### 2. Purpose
Reproduce recorded decisions without present-day reinterpretation.

### 3. Exact scope of code changes
Load exact archived inputs/evaluator/context and reproduce historical results; missing original material/unsupported semantics returns explicit limitations.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
A decision replays after present time/trust changes using original context; unsupported evaluator does not silently migrate acceptance.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Historical audit is reproducible and separate from current permission.

### 8. Commit message
```text
feat: replay historical conformance decisions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s139"></a>
## S139 — Evaluate current acceptance separately

**Predecessor:** S138. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Evaluate current acceptance separately.

### 2. Purpose
Prevent permanent permission from old signed or cached results.

### 3. Exact scope of code changes
Add a pure/current-context check over recorded decisions with present authorized freshness/trust/deployment criteria, without mutating the historic snapshot.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/freshness.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Expiry/revocation/policy changes reject current acceptance while original history remains unchanged.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Current release use must reestablish relevant eligibility.

### 8. Commit message
```text
feat: separate current decision acceptance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s140"></a>
## S140 — Provide shared explanation and query models

**Predecessor:** S139. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Provide shared explanation and query models.

### 2. Purpose
Replace command-specific interpretation with snapshot views.

### 3. Exact scope of code changes
Build query mapping for scope, obligations, reasons, evidence rejection, inventory/exposure, waivers and summaries; retain exact source/provenance information.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Query variants reference the same snapshot and preserve all blockers without implicit recalculation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI/MCP/native consumers get one coherent explanation surface.

### 8. Commit message
```text
feat: expose coherent evaluation explanations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s141"></a>
## S141 — Bind pagination to snapshots and filters

**Predecessor:** S140. **Contracts:** R04, R05, R11, R22, R38, R42, R44, R45, R46, R47, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC042, AC044, AC045, AC046, AC047, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Bind pagination to snapshots and filters.

### 2. Purpose
Keep large views coherent under updates.

### 3. Exact scope of code changes
Add bounded query pagination/cursor validation with snapshot/filter/order identity and explicit truncation metadata; avoid unauthorized cross-context cursor reuse.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Page continuation across changed snapshot/filter rejects; partial display does not alter completeness/authorization.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Large reports remain bounded and truthful.

### 8. Commit message
```text
feat: bind query pagination to evaluation snapshots
```

**Approved review regression contributions:** RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s142"></a>
## S142 — Define safe snapshot change notifications

**Predecessor:** S141. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Define safe snapshot change notifications.

### 2. Purpose
Support incremental consumers without hidden daemons.

### 3. Exact scope of code changes
Expose a bounded in-process/application notification or polling contract with snapshot generation identity and backpressure; no default network server.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_application/tests/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Slow/canceled consumers do not leak tasks or conflate generations; missed deltas trigger explicit resync.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Optional live consumers can refresh coherently without per-row reevaluation.

### 8. Commit message
```text
feat: expose bounded snapshot change signals
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s143"></a>
## S143 — Test end-to-end snapshot coherence

**Predecessor:** S142. **Contracts:** R04, R05, R11, R21, R22, R38, R44, R45, R46, R50, R52. **Acceptance:** AC004, AC005, AC011, AC021, AC022, AC038, AC044, AC045, AC046, AC050, AC052.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Test end-to-end snapshot coherence.

### 2. Purpose
Close application-service wiring before switching public commands.

### 3. Exact scope of code changes
Execute synthetic acquire/import-to-store-to-evaluate-to-query/replay flows using real checked fixture boundaries and expected non-success mutations.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
AC004/AC005/AC011/AC022/AC038/AC044/AC046 cases pass with no side-effect or trust shortcuts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
The shared application pipeline is independently verified before UI/CLI adoption.

### 8. Commit message
```text
test: verify the shared evaluation lifecycle
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
