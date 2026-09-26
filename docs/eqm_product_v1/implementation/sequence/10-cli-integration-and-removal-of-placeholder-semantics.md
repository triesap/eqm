# Chapter 10: CLI integration and removal of placeholder semantics

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P10, containing 14 separate ordered commits, S144–S157; it is not one combined commit. All implementation steps are pending.

**Definition of green:** All CLI commands use shared application decisions; lifecycle/output-schema/format/error/consumer-doctor regressions pass.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q14. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s144"></a>
## S144 — Route obligations through shared scope preparation

**Predecessor:** S143. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Route obligations through shared scope preparation.

### 2. Purpose
Remove CLI-specific policy derivation copies.

### 3. Exact scope of code changes
Make obligations use exact application selection/universe/query models, including unknown/not-applicable/provider/scenario coordinates and complete IDs.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/obligations.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
CLI JSON fixtures include complete scope and duplicate-profile/ambiguous-selection errors.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Obligation inspection uses the same authority and identities as evaluation.

### 8. Commit message
```text
refactor: route obligation queries through application
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s145"></a>
## S145 — Route check through stored-evidence evaluation

**Predecessor:** S144. **Contracts:** R04, R11, R12, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC011, AC012, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Route check through stored-evidence evaluation.

### 2. Purpose
Replace unconditional missing findings.

### 3. Exact scope of code changes
Load bounded prepared evidence through read-only application evaluation and render actual facet/structure findings; no runner execution or implicit state creation.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/check.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_cli tests`

### 5. Unit tests or integration tests required
Persist passing evidence then stale/failed/untrusted variants; check reflects each and remains non-executing.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
The intended verify/import-to-check lifecycle becomes functional.

### 8. Commit message
```text
fix: evaluate stored evidence in conformance checks
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s146"></a>
## S146 — Render conformance and evidence matrices from snapshots

**Predecessor:** S145. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Render conformance and evidence matrices from snapshots.

### 2. Purpose
Remove hardcoded missing matrix values.

### 3. Exact scope of code changes
Map complete snapshot decisions to conformance/evidence cells with stable axes, obligation references, reasons and presentation-only filters.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_engine/src/matrix.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
Passing/stale/failed/unknown/mixed cells match check and complete universe; filter does not recompute assurance.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Matrix content reflects evaluated evidence rather than placeholders.

### 8. Commit message
```text
fix: render evaluated conformance matrices
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s147"></a>
## S147 — Render exposure release and comparison matrices

**Predecessor:** S146. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Render exposure release and comparison matrices.

### 2. Purpose
Complete remaining matrix kinds using real evaluated inputs.

### 3. Exact scope of code changes
Populate exposure/release/equivalence views from appropriate snapshot facts and conclusions, with unavailable states where data genuinely lacks verification.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_application/src/query.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Runtime mismatch and missing end-to-end/cohort cases agree with engine; no presence-only exposure match.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Every supported matrix view has a real semantic source.

### 8. Commit message
```text
feat: render complete assurance matrix views
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s148"></a>
## S148 — Route context and explanation through snapshots

**Predecessor:** S147. **Contracts:** R04, R30, R40, R42, R44, R45, R46, R47, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC042, AC044, AC045, AC046, AC047, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Route context and explanation through snapshots.

### 2. Purpose
Replace empty or fragmented evidence/findings context.

### 3. Exact scope of code changes
Populate context/explain with approved authority, exact obligations, evidence acceptance/rejection and contributing waivers; keep product text labeled untrusted and bounded.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/context.rs`
- `crates/eqm_cli/src/commands/explain.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
Context matches check/matrix and respects byte budgets without claiming truncated scope is complete.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Human/agent investigation gets actionable consistent reasons.

### 8. Commit message
```text
feat: expose evaluated context and explanations
```

**Approved review regression contributions:** RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s149"></a>
## S149 — Render concrete verify dry-run plans

**Predecessor:** S148. **Contracts:** R04, R11, R12, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC011, AC012, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Render concrete verify dry-run plans.

### 2. Purpose
Fulfill the execution-inspection contract publicly.

### 3. Exact scope of code changes
Serialize the shared fully resolved plan in human/JSON output with all safe executable/argv/environment-reference/limit/output details and no secret values.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_cli/src/renderer.rs`

### 5. Unit tests or integration tests required
Dry-run includes exact planned work, invokes no programs, rejects unresolved selectors/programs and never outputs secret bytes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Users can inspect actual execution before authorizing it.

### 8. Commit message
```text
fix: render complete verification dry-run plans
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s150"></a>
## S150 — Execute verify through authorized acquisition

**Predecessor:** S149. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Execute verify through authorized acquisition.

### 2. Purpose
Remove ad hoc runner authority and unbounded result reads.

### 3. Exact scope of code changes
Route verify execution to application approval/campaign/supervision/persistence path, retaining explicit cancellation and typed execution outcomes.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/`

### 5. Unit tests or integration tests required
Local fixture execution persists exact receipts/evidence; unsupported guarantees/secret absence fail visibly; canceled work is retained correctly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI execution uses the same bounded independently authorized acquisition boundary.

### 8. Commit message
```text
refactor: route verification through authorized acquisition
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s151"></a>
## S151 — Separate acquisition summaries from conformance

**Predecessor:** S150. **Contracts:** R04, R11, R12, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC011, AC012, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Separate acquisition summaries from conformance.

### 2. Purpose
Fix empty-plan and low-minimum success ambiguity.

### 3. Exact scope of code changes
Report selected/acquired outcome distinctly from evaluated policy result; reject accidental empty selection under documented semantics and never claim policy success from exit zero.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_protocol/src/`
- `docs/cli.md`

### 5. Unit tests or integration tests required
Empty plan, inadequate policy minimum and successful runner with missing expected case remain nonconformant/explicit failure.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
verify output cannot mislead callers about complete product assurance.

### 8. Commit message
```text
fix: distinguish acquisition from policy conformance
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s152"></a>
## S152 — Emit structured CLI failure envelopes

**Predecessor:** S151. **Contracts:** R04, R05, R21, R30, R40, R44, R45, R46, R52, R58, R59, R60. **Acceptance:** AC004, AC005, AC021, AC030, AC040, AC044, AC045, AC046, AC052, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Emit structured CLI failure envelopes.

### 2. Purpose
Preserve requested machine output on command failures.

### 3. Exact scope of code changes
Map application/parser/runner/storage/trust errors to stable safe machine envelopes and documented exit classes. Validate command/output-format compatibility before acquisition/storage; implement renderers for accepted formats or reject them during preflight. Preserve discoverable durable receipts and truthful errors if output fails after commit.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/main.rs`
- `crates/eqm_cli/src/renderer.rs`
- `crates/eqm_protocol/src/`
- `docs/cli.md`

### 5. Unit tests or integration tests required
Validate actual success/error envelopes against generated schemas for every command family; test all accepted format pairs, early usage/format rejection with effect spies, and output failure after durable receipt publication.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Machine consumers no longer receive only generic stderr stage failures.

### 8. Commit message
```text
fix: emit structured command failure envelopes
```

**Approved review regression contributions:** RA14, RA20.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s153"></a>
## S153 — Align discovery and reconciliation output

**Predecessor:** S152. **Contracts:** R04, R24, R28, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC024, AC028, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Align discovery and reconciliation output.

### 2. Purpose
Use independent inventory observations and shared evaluation facts.

### 3. Exact scope of code changes
Route static/executable discovery permission distinctions and reconciliation status into coherent DTOs; do not rewrite authored expectations from observations.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/discover.rs`
- `crates/eqm_cli/src/commands/reconcile.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Partial/untrusted inventory remains unknown, approved executable discovery uses acquisition, and reconciliation is non-mutating.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Observed state and intended authority stay separate and visible.

### 8. Commit message
```text
refactor: align discovery and reconciliation contracts
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s154"></a>
## S154 — Align inspect and diagnostic commands

**Predecessor:** S153. **Contracts:** R04, R05, R30, R40, R44, R45, R46, R48, R53, R58, R59, R60. **Acceptance:** AC004, AC005, AC030, AC040, AC044, AC045, AC046, AC048, AC053, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Align inspect and diagnostic commands.

### 2. Purpose
Preserve accurate provenance and root/schema/capability errors.

### 3. Exact scope of code changes
Update show/locate/doctor/validate to use shared prepared types/errors. Separate consumer prerequisites from EQM contributor toolchain checks and recognize valid generated store/work states. Keep read commands free of accidental acquisition and honor format preflight.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/show.rs`
- `crates/eqm_cli/src/commands/locate.rs`
- `crates/eqm_cli/src/commands/doctor.rs`
- `crates/eqm_cli/src/commands/validate.rs`

### 5. Unit tests or integration tests required
Run doctor on an unmodified non-Rust consumer without copied EQM Cargo/toolchain files, after supported generated-state creation, and with invalid store entries; test accepted output formats and read-only effect boundaries.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Supporting commands remain useful after shared-layer extraction.

### 8. Commit message
```text
refactor: align inspection commands with prepared context
```

**Approved review regression contributions:** RA19, RA20.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s155"></a>
## S155 — Add CLI lifecycle golden tests

**Predecessor:** S154. **Contracts:** R04, R05, R11, R12, R21, R30, R40, R44, R45, R46, R48, R52, R53, R58, R59, R60. **Acceptance:** AC004, AC005, AC011, AC012, AC021, AC030, AC040, AC044, AC045, AC046, AC048, AC052, AC053, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Add CLI lifecycle golden tests.

### 2. Purpose
Replace tests that merely assert missing or nonempty placeholder output.

### 3. Exact scope of code changes
Test actual passing and negative evidence lifecycle across obligations/check/matrix/context/verify and machine errors with deterministic time/snapshot inputs.

### 4. Files/modules likely involved
- `crates/eqm_cli tests`
- `tests/fixtures/`
- `crates/eqm_test_support/src/`

### 5. Unit tests or integration tests required
Views agree for the same snapshot; stale/trust/failed/missing changes are reflected and no .eqm writes occur on read-only paths.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI coverage exercises the promised product workflow rather than placeholder shape.

### 8. Commit message
```text
test: verify the full cli evidence lifecycle
```

**Approved review regression contributions:** RA01, RA14, RA19, RA20.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s156"></a>
## S156 — Remove superseded CLI evaluation helpers

**Predecessor:** S155. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Remove superseded CLI evaluation helpers.

### 2. Purpose
Eliminate duplicate truth calculators after adoption tests.

### 3. Exact scope of code changes
Delete unused status/count/freshness/profile/digest implementations now replaced by application/engine; retain only command orchestration and rendering.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_cli/src/commands/check.rs`
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_cli/src/commands/verify.rs`

### 5. Unit tests or integration tests required
Golden lifecycle tests remain unchanged; dependency/lint scan finds no new divergent evaluator.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI is thin and semantic maintenance has one authoritative implementation.

### 8. Commit message
```text
refactor: remove duplicate cli evaluation logic
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s157"></a>
## S157 — Update CLI usage and safe adoption examples

**Predecessor:** S156. **Contracts:** R04, R30, R40, R44, R45, R46, R48, R53, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC048, AC053, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Update CLI usage and safe adoption examples.

### 2. Purpose
Make docs reflect implemented behavior and exact command grammar.

### 3. Exact scope of code changes
Update docs/cli.md, getting started, evidence/trust and existing examples for actual invocation/exit/dry-run/selection behavior; do not invent product tasks.

### 4. Files/modules likely involved
- `docs/cli.md`
- `docs/getting-started.md`
- `docs/evidence-and-trust.md`
- `examples/`

### 5. Unit tests or integration tests required
Run documented example validation and safe smoke commands discovered from the repo; examples pass without live credentials or implicit network.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
Public instructions no longer promise placeholder or unimplemented behavior.

### 8. Commit message
```text
docs: document the completed evaluation workflow
```

**Approved review regression contributions:** RA19.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
