# Chapter 06: Immutable object store and transactional SQLite index

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P06, containing 20 separate ordered commits, S077–S096; it is not one combined commit. All implementation steps are pending.

**Definition of green:** Pinned linked SQLite and immutable objects pass concurrency, crash/reopen, migration/restore, receipt and retention tests.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q02, Q07, Q10. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s077"></a>
## S077 — Select and pin the storage implementation

**Predecessor:** S076. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Select and pin the storage implementation.

### 2. Purpose
Establish safe runtime and durability assumptions before persistence code.

### 3. Exact scope of code changes
Resolve Q10 from actual dependencies/hosts; choose an audited exact SQLite wrapper/runtime containing the WAL-reset fix and record linking/features/durability design; update lockfile intentionally.

### 4. Files/modules likely involved
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_application/Cargo.toml`
- `docs/eqm_product_v1/decisions/`

### 5. Unit tests or integration tests required
Inspect linked SQLite runtime/fix and license/advisory results; compile chosen features; no network-filesystem assumption.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Storage uses a justified fixed audited runtime rather than an unchecked system dependency.

### 8. Commit message
```text
build: pin the reviewed local storage runtime
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s078"></a>
## S078 — Define store operations and typed failures

**Predecessor:** S077. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Define store operations and typed failures.

### 2. Purpose
Keep storage contracts independent of SQL and privilege assumptions.

### 3. Exact scope of code changes
Add bounded object/index/read-snapshot interfaces and typed storage errors for unavailable/corrupt/conflict/limit conditions; avoid a generic unrestricted file API.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/mod.rs`
- `crates/eqm_application/src/error.rs`

### 5. Unit tests or integration tests required
In-memory contract tests demonstrate missing objects and conflicts never produce verified/success states.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Application operations can rely on explicit storage semantics without trusting metadata.

### 8. Commit message
```text
refactor: define immutable evidence store boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s079"></a>
## S079 — Initialize protected store roots safely

**Predecessor:** S078. **Contracts:** R18, R22, R26, R33, R35, R36, R37, R38, R42, R50, R52. **Acceptance:** AC018, AC022, AC026, AC033, AC035, AC036, AC037, AC038, AC042, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Initialize protected store roots safely.

### 2. Purpose
Handle concurrent creation and confinement.

### 3. Exact scope of code changes
Create private staging/object/index roots with checked existing-directory behavior, concurrent AlreadyExists revalidation and explicit development/trusted location selection.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_runner/src/persistence.rs`

### 5. Unit tests or integration tests required
Concurrent first initialization succeeds safely; symlink/wrong-type roots and candidate-writable trusted configurations reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Store initialization is idempotent and does not race into unsafe destinations.

### 8. Commit message
```text
fix: initialize evidence storage safely
```

**Approved review regression contributions:** RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s080"></a>
## S080 — Write bounded immutable staging objects

**Predecessor:** S079. **Contracts:** R18, R22, R26, R33, R35, R36, R37, R38, R42, R50, R52. **Acceptance:** AC018, AC022, AC026, AC033, AC035, AC036, AC037, AC038, AC042, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Write bounded immutable staging objects.

### 2. Purpose
Prevent unbounded buffering before persistence validation.

### 3. Exact scope of code changes
Stream or bounded-read staged objects, compute raw digest, validate limits and retain handles; keep new objects invisible to the index.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/objects.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Oversized/changing input and short writes fail without publication; exact bytes/digest are retained for valid input.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Staging cannot allocate or publish unbounded unvalidated evidence.

### 8. Commit message
```text
feat: stage bounded immutable evidence objects
```

**Approved review regression contributions:** RA12, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s081"></a>
## S081 — Install objects durably without clobbering

**Predecessor:** S080. **Contracts:** R18, R22, R26, R33, R35, R36, R37, R38, R42, R50, R52. **Acceptance:** AC018, AC022, AC026, AC033, AC035, AC036, AC037, AC038, AC042, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Install objects durably without clobbering.

### 2. Purpose
Make object publication crash-safe and collision-aware.

### 3. Exact scope of code changes
Implement durable no-clobber install and supported-host file/directory sync behavior; bound existing-file comparisons and retain raw collision distinction.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/objects.rs`
- `crates/eqm_runner/src/persistence.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Identical installs are idempotent; different bytes under same raw identity reject; oversized existing files are not read unbounded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
An installed object is immutable and safely published before any index reference.

### 8. Commit message
```text
feat: publish immutable objects durably
```

**Approved review regression contributions:** RA12, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s082"></a>
## S082 — Create versioned local metadata tables

**Predecessor:** S081. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Create versioned local metadata tables.

### 2. Purpose
Introduce the smallest index schema matching established identities.

### 3. Exact scope of code changes
Add schema version metadata and tables/constraints for raw/semantic objects, execution lineage and committed references; do not store trusted booleans as authority.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/src/storage/migrations/`

### 5. Unit tests or integration tests required
Fresh initialization/version mismatch/constraint tests pass; incompatible writers reject explicitly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Local metadata has explicit versioning and identity constraints.

### 8. Commit message
```text
feat: initialize the versioned evidence index
```

**Approved review regression contributions:** RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s083"></a>
## S083 — Commit import references atomically

**Predecessor:** S082. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Commit import references atomically.

### 2. Purpose
Enforce object-first/index-second publication.

### 3. Exact scope of code changes
Add transactions that reference already validated installed objects and atomically index semantic/execution relationships; no visibility before commit.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Inject rollback before commit and reopen; no completed row references unpublished bytes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Import publication has a clear atomic visibility boundary.

### 8. Commit message
```text
feat: commit evidence references atomically
```

**Approved review regression contributions:** RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s084"></a>
## S084 — Persist idempotent imports and attempt conflicts

**Predecessor:** S083. **Contracts:** R17, R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC017, AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Persist idempotent imports and attempt conflicts.

### 2. Purpose
Make replay safety survive process restarts.

### 3. Exact scope of code changes
Enforce uniqueness by appropriate semantic/attempt identity, retain alternate raw transports, and persist contradictory attempt conflicts instead of run-only rejection.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_runner/src/ci.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Reimport after restart stays idempotent; many results from one CI run work; same-attempt contradictions remain recorded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Replay handling is persistent, precise and not overly coarse.

### 8. Commit message
```text
fix: persist exact evidence replay identities
```

**Approved review regression contributions:** RA08, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s085"></a>
## S085 — Persist campaigns and execution receipts

**Predecessor:** S084. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Persist campaigns and execution receipts.

### 2. Purpose
Preserve history and selection authority across restarts.

### 3. Exact scope of code changes
Store plan/campaign/case/attempt lineage, terminal worker receipts and explicit supersession/selection metadata with validated references.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Interrupted acquisitions remain distinguishable from completed campaigns; old attempts survive new campaign selection.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Execution history supports deterministic current selection and audit.

### 8. Commit message
```text
feat: persist evidence campaign history
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s086"></a>
## S086 — Store runtime facts and release subject records

**Predecessor:** S085. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Store runtime facts and release subject records.

### 2. Purpose
Unify bounded validated supporting inputs.

### 3. Exact scope of code changes
Index immutable runtime observations, candidate/subject manifests and release-related records by exact subjects/context; reject malformed referenced objects.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Wrong-subject, missing-reference and duplicate delivery cases behave consistently with evidence objects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Supporting facts can be loaded coherently rather than enumerating arbitrary directories.

### 8. Commit message
```text
feat: store exact runtime and release inputs
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s087"></a>
## S087 — Persist verification receipts without minting authority

**Predecessor:** S086. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Persist verification receipts without minting authority.

### 2. Purpose
Retain cryptographic context for replay and current checks.

### 3. Exact scope of code changes
Store receipts and supporting material references as records requiring verification on import; expose raw facts through a checked retrieval path.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/src/trust.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Forged serialized verified fields are not accepted; missing trust material remains unavailable; originals remain immutable.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Storage preserves receipts but cannot convert them into live authority.

### 8. Commit message
```text
feat: retain verification receipt provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s088"></a>
## S088 — Read consistent index snapshots

**Predecessor:** S087. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Read consistent index snapshots.

### 2. Purpose
Prevent mixed-generation evaluation inputs.

### 3. Exact scope of code changes
Add bounded read transactions/snapshot generation handles and validate referenced immutable objects before preparing inputs; do not hold transactions during test execution.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/src/evaluation.rs`

### 5. Unit tests or integration tests required
Concurrent import during preparation cannot alter selected input generation; missing/corrupt objects fail explicitly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Evaluation preparation observes one coherent store state.

### 8. Commit message
```text
feat: read consistent evidence snapshots
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s089"></a>
## S089 — Implement indexed coordinate queries

**Predecessor:** S088. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Implement indexed coordinate queries.

### 2. Purpose
Avoid scanning and parsing every evidence file.

### 3. Exact scope of code changes
Query by exact subject/specification/profile/campaign/attempt coordinates using appropriate indexes; preserve deterministic ordering and rejected-record explanations.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Indexed queries match a full-scan reference on synthetic histories, including conflicts and irrelevant records.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Evidence lookup is precise and testably equivalent to the simple reference.

### 8. Commit message
```text
feat: index exact evidence coordinates
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s090"></a>
## S090 — Bound writers and own checkpoints

**Predecessor:** S089. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Bound writers and own checkpoints.

### 2. Purpose
Make SQLite contention behavior explicit.

### 3. Exact scope of code changes
Implement short writer transactions, bounded busy handling and checkpoint ownership/configuration; refuse unsupported local-store configurations as specified.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Concurrent readers/writers/checkpoints have bounded outcomes and no silent partial commits; linked runtime check is exercised.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
WAL operation has explicit contention and durability guarantees.

### 8. Commit message
```text
fix: bound evidence index contention
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s091"></a>
## S091 — Recover staged and orphaned objects

**Predecessor:** S090. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Recover staged and orphaned objects.

### 2. Purpose
Make crash cleanup safe rather than destructive.

### 3. Exact scope of code changes
Implement recovery classification for staged/orphan/committed objects with conservative handling of active operations; never fabricate missing committed objects.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/recovery.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Faults at each publication boundary recover coherently; live staged work is not deleted and missing committed bytes are reported.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Restart recovery does not produce phantom completed evidence.

### 8. Commit message
```text
feat: recover interrupted evidence publication
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s092"></a>
## S092 — Protect live readers and acquisitions with leases

**Predecessor:** S091. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Protect live readers and acquisitions with leases.

### 2. Purpose
Prevent cleanup racing active use.

### 3. Exact scope of code changes
Add explicit bounded lease/reference ownership for ongoing reads/acquisitions and release archives, with tested expiry/recovery semantics from operator policy.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/retention.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Concurrent cleanup cannot remove objects used by readers/runs; stale orphan lease handling follows explicit rules.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Object lifecycle accounts for active consumers and workers.

### 8. Commit message
```text
feat: protect live evidence references
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s093"></a>
## S093 — Implement policy-driven retention and GC

**Predecessor:** S092. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Implement policy-driven retention and GC.

### 2. Purpose
Reclaim only authorized unreachable data.

### 3. Exact scope of code changes
Add retention classes and reachability-based cleanup using configured policy and leases; preserve required signed originals/release archives and sensitive-data boundaries.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/retention.rs`
- `crates/eqm_application/tests/`
- `docs/eqm_product_v1/runbooks/`

### 5. Unit tests or integration tests required
Class/lease/archive tests prevent premature deletion; eligible orphans/history are reclaimed deterministically.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Immutability is compatible with safe explicit retention, not infinite unbounded growth.

### 8. Commit message
```text
feat: reclaim eligible evidence objects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s094"></a>
## S094 — Rebuild indexes from retained manifests

**Predecessor:** S093. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Rebuild indexes from retained manifests.

### 2. Purpose
Make the index replaceable without losing trust/history.

### 3. Exact scope of code changes
Add bounded reconstruction from immutable manifests/receipts with conflict preservation and validation; do not trust filenames or create receipt authority during rebuild.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/recovery.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Delete only a disposable fixture index and rebuild; identities, history, conflicts and decisions match original records.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Recovery can reconstruct discoverability without inventing accepted evidence.

### 8. Commit message
```text
feat: rebuild evidence indexes safely
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s095"></a>
## S095 — Test explicit database migration and restore

**Predecessor:** S094. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Test explicit database migration and restore.

### 2. Purpose
Verify compatibility before schema evolution.

### 3. Exact scope of code changes
Implement approved migration coordination/backup-or-restore mechanics and one controlled fixture transition; retain original immutable/signature objects.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/migrations/`
- `crates/eqm_application/tests/`
- `docs/eqm_product_v1/specs/COMPATIBILITY.md`

### 5. Unit tests or integration tests required
Upgrade, interruption, reopen, incompatible writer and rollback/restore tests pass on declared hosts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Database version transitions are atomic or recoverable and never silent reinterpretations.

### 8. Commit message
```text
test: verify evidence store migration recovery
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s096"></a>
## S096 — Complete storage fault-injection coverage

**Predecessor:** S095. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Complete storage fault-injection coverage.

### 2. Purpose
Close persistence guarantees before production routing.

### 3. Exact scope of code changes
Add publication/transaction/checkpoint/cleanup interruption points and bounded corrupt-file fixtures; document which hosts/filesystems were actually tested.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
AC018/AC033/AC035–AC038/AC052 storage cases pass on supported hosts or remain explicit blocked gates.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Store readiness is supported by failure tests rather than filesystem assumptions.

### 8. Commit message
```text
test: cover evidence storage failure boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
