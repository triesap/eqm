# Chapter 14: Rust KMP adapters and product acceptance integration

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P14, containing 11 separate ordered commits, S191–S201; it is not one combined commit. All implementation steps are pending.

**Definition of green:** Actual authorized Rust/KMP/provider tasks, approved product scenarios and supported cohorts pass; optional embedding disposition recorded.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q01, Q02, Q06, Q12. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s191"></a>
## S191 — Discover real product targets and native commands

**Predecessor:** S190. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Discover real product targets and native commands.

### 2. Purpose
Avoid invented Gradle/Xcode selectors and unsupported platform promises.

### 3. Exact scope of code changes
Resolve Q01/Q02/Q06/Q12 from authorized repos/owners; record product commits, native tasks, selectors, output artifacts, roles and supported cohorts without editing product code yet.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`
- `docs/integrations/`
- `examples/`

### 5. Unit tests or integration tests required
Read manifests/help and verify discovered commands in disposable authorized fixtures; distinguish synthetic versus live lanes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Real integration has exact known inputs or an explicit blocker instead of fabricated tasks.

### 8. Commit message
```text
docs: record native product integration contracts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s192"></a>
## S192 — Define native result adapter contracts

**Predecessor:** S191. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Define native result adapter contracts.

### 2. Purpose
Preserve expected selection and subject identity across test frameworks.

### 3. Exact scope of code changes
Specify and implement minimal adapters' common normalized result/receipt boundary for native selectors, cases, attempts and artifacts; do not build a new test framework.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/normalized.rs`
- `crates/eqm_protocol/src/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Equivalent Rust/Kotlin native fixture results normalize identically; missing cases and wrong subject/attempt reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Native producers can supply exact conformance evidence independently of UI frameworks.

### 8. Commit message
```text
feat: define native evidence adapter contracts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s193"></a>
## S193 — Integrate the Rust product CLI test adapter

**Predecessor:** S192. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Integrate the Rust product CLI test adapter.

### 2. Purpose
Verify the product CLI without confusing it with EQM's binary.

### 3. Exact scope of code changes
Bind discovered Rust product tests/inspection/export scenarios to exact EQM selectors and normalized results; implement an authorized adapter or fixture harness without inventing product business logic.

### 4. Files/modules likely involved
- `examples/`
- `tests/fixtures/`
- `crates/eqm_runner tests`
- `authorized product Rust test adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Actual fixture/test commands produce bounded subject-bound expected-case results; malformed/missing selection fails.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Rust product CLI evidence participates in the common conformance model.

### 8. Commit message
```text
feat: integrate rust product conformance evidence
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s194"></a>
## S194 — Integrate desktop KMP test evidence

**Predecessor:** S193. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Integrate desktop KMP test evidence.

### 2. Purpose
Connect actual control-tower tests to EQM contracts.

### 3. Exact scope of code changes
Bind discovered desktop tasks/selectors/artifacts and normalize account/task projection/review fixtures; keep business UI and source ownership in the product repo.

### 4. Files/modules likely involved
- `docs/integrations/`
- `examples/`
- `tests/fixtures/`
- `authorized desktop KMP test adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Native desktop lane validates current/stale/conflicting projection fixtures and exact subject/selection mapping; no JVM/native assumption is fabricated.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Desktop conformance evidence comes from real native tests where access exists.

### 8. Commit message
```text
feat: integrate desktop kmp conformance evidence
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s195"></a>
## S195 — Integrate mobile capture and lifecycle evidence

**Predecessor:** S194. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Integrate mobile capture and lifecycle evidence.

### 2. Purpose
Verify edge behavior through actual platform tests.

### 3. Exact scope of code changes
Bind discovered mobile photo/video/voice/annotation/save-restart/permission fixtures and native task selectors; keep media handling out of EQM's engine.

### 4. Files/modules likely involved
- `docs/integrations/`
- `examples/`
- `tests/fixtures/`
- `authorized mobile test adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Supported device/simulator lanes verify acknowledged persistence/interruption and correct normalized expected cases; unsupported platforms stay explicit.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Mobile edge evidence is native and lifecycle-aware rather than a parser-only fixture claim.

### 8. Commit message
```text
feat: integrate mobile edge conformance evidence
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s196"></a>
## S196 — Integrate provider and runtime observations

**Predecessor:** S195. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R24, R28, R30, R48, R49, R55, R59. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC024, AC028, AC030, AC048, AC049, AC055, AC059.

**Read:** [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Integrate provider and runtime observations.

### 2. Purpose
Bind shared services and actual exposure to exact subjects.

### 3. Exact scope of code changes
Add discovered provider/deployment observation producers and bounded normalization for required sync/exposure facts; distinguish partial from complete trusted inventory.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/inventory.rs`
- `examples/`
- `tests/fixtures/`
- `authorized provider adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Wrong provider/config/build and partial/stale observations cannot satisfy required dependencies.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Shared providers have independent exact evidence rather than inferred client success.

### 8. Commit message
```text
feat: integrate exact provider runtime evidence
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s197"></a>
## S197 — Implement the cross-runtime lifecycle scenario

**Predecessor:** S196. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Implement the cross-runtime lifecycle scenario.

### 2. Purpose
Verify the approved capture-to-review-to-export chain.

### 3. Exact scope of code changes
Orchestrate existing native test operations under one approved scenario/campaign: offline capture, edit, terminate/restart, reconnect/retry, desktop review and CLI inspect/export; use synthetic account/task data.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `examples/`
- `crates/eqm_application tests`
- `authorized native integration test paths from Q01`

### 5. Unit tests or integration tests required
Intended revision appears exactly once under the correct task with no acknowledged-state loss or cross-account disclosure.

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
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
End-to-end evidence is independently produced for the actual participant set.

### 8. Commit message
```text
test: verify the evidence lifecycle across runtimes
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s198"></a>
## S198 — Add product failure and account-boundary scenarios

**Predecessor:** S197. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Add product failure and account-boundary scenarios.

### 2. Purpose
Exercise failure contracts beyond the happy path.

### 3. Exact scope of code changes
Extend the real scenario with approved account switching, permission revocation, concurrent edits, interrupted uploads and stale projections; use owner-defined conflict semantics only.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `authorized product integration test paths from Q01`
- `docs/eqm_product_v1/specs/ACCEPTANCE_CRITERIA.md`

### 5. Unit tests or integration tests required
Each interruption/security case follows explicit product invariants and produces distinct expected outcomes; no invented conflict algorithm.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Product assurance covers the approved lifecycle failure boundaries.

### 8. Commit message
```text
test: cover cross-runtime failure and account boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s199"></a>
## S199 — Verify approved mixed-version cohorts

**Predecessor:** S198. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Verify approved mixed-version cohorts.

### 2. Purpose
Demonstrate independently released target compatibility.

### 3. Exact scope of code changes
Execute the scenario for the owner-approved old/new client/provider combinations and role occurrences; add explicit relational outcome comparison where required.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `authorized product integration test paths from Q01`
- `examples/`

### 5. Unit tests or integration tests required
Approved combinations meet declared outcomes; undeclared or mismatched cohorts cannot borrow passing evidence.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Compatibility assurance is tied to tested exact cohorts, not a global same-version assumption.

### 8. Commit message
```text
test: verify approved runtime compatibility cohorts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s200"></a>
## S200 — Publish a complete consumer integration example

**Predecessor:** S199. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R44, R48, R49, R53, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC044, AC048, AC049, AC053, AC055.

**Read:** [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Publish a complete consumer integration example.

### 2. Purpose
Make the intended Rust/KMP adoption reconstructable.

### 3. Exact scope of code changes
Add a small copyable synthetic workspace with shared-source roles, bindings, policies, profiles, native adapter contracts and safe dry-run/check instructions; mark real task substitutions explicitly.

### 4. Files/modules likely involved
- `examples/`
- `docs/integrations/`
- `docs/getting-started.md`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
Example validates and safe fixture smoke tests pass; generated state remains ignored and no fake real-product evidence is committed.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Consumers can adopt one real slice without duplicating intent or confusing business/conformance evidence.

### 8. Commit message
```text
docs: publish the multi-runtime integration example
```

**Approved review regression contributions:** RA19.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s201"></a>
## S201 — Record optional embedding disposition

**Predecessor:** S200. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Record optional embedding disposition.

### 2. Purpose
Preserve the approved default without silently starting bridge work.

### 3. Exact scope of code changes
Record that default EQM deployment is developer/CI; evaluate whether an actual local evaluation feature activates the separate optional sequence; do not add bridges/daemons otherwise.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`
- `docs/eqm_product_v1/decisions/`
- `docs/eqm_product_v1/specs/RUNTIME_INTEGRATION.md`

### 5. Unit tests or integration tests required
Package/dependency review confirms no unactivated native/daemon surface; activation decision references exact use case and matrix if granted.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Mandatory v1 scope is closed independently of optional embedding.

### 8. Commit message
```text
docs: record native embedding activation status
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
