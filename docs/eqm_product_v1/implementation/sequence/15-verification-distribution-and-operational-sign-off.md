# Chapter 15: Verification distribution and operational sign-off

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P15, containing 10 separate ordered commits, S202–S211; it is not one combined commit. All implementation steps are pending.

**Definition of green:** Standard/extended/host/product/crypto/compatibility/distribution/shadow gates have evidence; final report separates code/platform/operational status.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q02, Q04, Q07, Q09, Q15. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s202"></a>
## S202 — Complete the supported feature and host matrix

**Predecessor:** S201. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/BASELINE.md](../../repo/BASELINE.md), [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Complete the supported feature and host matrix.

### 2. Purpose
Make packaging capability claims explicit.

### 3. Exact scope of code changes
Inspect actual Cargo features and supported targets, add tests for every supported combination and ensure optional capabilities never disable mandatory validation; record unsupported hosts deliberately.

### 4. Files/modules likely involved
- `Cargo.toml`
- `crates/*/Cargo.toml`
- `tools/xtask/src/main.rs`
- `docs/eqm_product_v1/implementation/VERIFICATION.md`

### 5. Unit tests or integration tests required
Discover and run valid no-default/default/feature combinations plus native host lanes; no blanket all-features command for incompatible combinations.

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
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Build and support claims match tested actual configurations.

### 8. Commit message
```text
test: verify supported feature and host combinations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s203"></a>
## S203 — Expand fuzz and mutation coverage to new boundaries

**Predecessor:** S202. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/BASELINE.md](../../repo/BASELINE.md), [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Expand fuzz and mutation coverage to new boundaries.

### 2. Purpose
Test the newly centralized critical paths rather than only old helpers.

### 3. Exact scope of code changes
Extend fuzz corpus/targets and focused mutation coverage for strict parsing, scope completeness, evidence conflicts, trust acceptance, authorization and storage records while retaining bounds.

### 4. Files/modules likely involved
- `tools/fuzz/`
- `tools/xtask/src/main.rs`
- `crates/eqm_engine/tests/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Bounded fuzz and mutation lanes pass with meaningful seeded valid/invalid inputs; surviving critical mutants are investigated, not ignored by threshold changes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Extended verification targets the actual production decision path.

### 8. Commit message
```text
test: harden critical conformance verification lanes
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s204"></a>
## S204 — Verify schema API and migration compatibility

**Predecessor:** S203. **Contracts:** R21, R43, R44, R45, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC021, AC043, AC044, AC045, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/BASELINE.md](../../repo/BASELINE.md), [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Verify schema API and migration compatibility.

### 2. Purpose
Close explicit version-transition obligations.

### 3. Exact scope of code changes
Run all approved wire/ABI-if-activated/DB compatibility fixtures, original-signed-byte preservation, generator parity and unsupported-version errors; update transition docs with actual evidence.

### 4. Files/modules likely involved
- `crates/eqm_protocol/tests/`
- `crates/eqm_application/tests/`
- `tools/xtask/src/main.rs`
- `docs/eqm_product_v1/specs/COMPATIBILITY.md`

### 5. Unit tests or integration tests required
Old/new supported behavior matches ADR, unsupported versions fail visibly, interrupted migrations recover and signed originals remain intact.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_application -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Compatibility is tested rather than inferred from deserialization success.

### 8. Commit message
```text
test: verify explicit compatibility transitions
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s205"></a>
## S205 — Verify reproducible distribution and supply chain

**Predecessor:** S204. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/BASELINE.md](../../repo/BASELINE.md), [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Verify reproducible distribution and supply chain.

### 2. Purpose
Ensure shipped artifacts match declared dependencies and capabilities.

### 3. Exact scope of code changes
Extend existing dist/package checks for new crate/dependencies/runtime libraries, SPDX/SBOM/provenance inputs and checksums; keep outputs unsigned/local unless separately authorized.

### 4. Files/modules likely involved
- `tools/xtask/src/main.rs`
- `Cargo.lock`
- `deny.toml if present`
- `docs/eqm_product_v1/implementation/VERIFICATION.md`

### 5. Unit tests or integration tests required
Package/reproducibility and current dependency audit/policy lanes pass; inspect actual SQLite/verifier/native library versions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Distribution artifacts have reproducible provenance without implicit publication.

### 8. Commit message
```text
build: verify conformance distribution provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s206"></a>
## S206 — Reconcile public docs and remove transitional code

**Predecessor:** S205. **Contracts:** R43, R44, R48, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC044, AC048, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/BASELINE.md](../../repo/BASELINE.md), [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Reconcile public docs and remove transitional code.

### 2. Purpose
Finish code health without broad cosmetic churn.

### 3. Exact scope of code changes
Remove obsolete helpers/features/placeholder fixtures only after tests prove replacement; update README/public docs/error references and generated schemas; retain approved historical records.

### 4. Files/modules likely involved
- `README.md`
- `docs/`
- `crates/`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
Standard gate, doc tests, examples and dependency/lint scans pass; no hardcoded production missing/unknown-as-final placeholders or synthetic trust bypass remains.

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
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
The repository has one documented implemented architecture without abandoned transition paths.

### 8. Commit message
```text
refactor: finalize the shared conformance implementation
```

**Approved review regression contributions:** RA19.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s207"></a>
## S207 — Rehearse storage trust and release incidents

**Predecessor:** S206. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/BASELINE.md](../../repo/BASELINE.md), [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Rehearse storage trust and release incidents.

### 2. Purpose
Make runbooks executable and owners explicit.

### 3. Exact scope of code changes
Assign owner roles using Q15 and run disposable crash/restore, key/material rotation, stale decision, failed authorization and evaluator rollback drills; record results and limitations.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/runbooks/`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Recovery preserves history and trust boundaries; rollback pauses or uses known-good evaluation, never permissive promotion.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Operational procedures are evidenced and responsibility is explicit.

### 8. Commit message
```text
docs: record conformance operational recovery drills
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s208"></a>
## S208 — Run shadow release comparison

**Predecessor:** S207. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/BASELINE.md](../../repo/BASELINE.md), [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Run shadow release comparison.

### 2. Purpose
Validate decisions independently before enforcement.

### 3. Exact scope of code changes
Integrate host-neutral shadow reporting into approved existing CI/deployment environment; compare representative positive/negative decisions with independently reviewed expected outcomes.

### 4. Files/modules likely involved
- `approved CI integration path discovered in Q09`
- `docs/eqm_product_v1/runbooks/RELEASE_OPERATIONS.md`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
Wrong-subject/weakening/missing-scope/expiry fixtures remain blocked; independent review records discrepancies and no live promotion occurs.

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
- Discover authorized CI/promotion validation commands and exact protected identities/destinations through Q04/Q09/Q15. Do not execute publication/deployment or invent operational approval as part of this code step.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Gate behavior has operational comparison evidence before activation.

### 8. Commit message
```text
test: verify shadow release decision parity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s209"></a>
## S209 — Complete final acceptance and release-candidate gates

**Predecessor:** S208. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/BASELINE.md](../../repo/BASELINE.md), [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Complete final acceptance and release-candidate gates.

### 2. Purpose
Establish readiness from actual evidence rather than approved design.

### 3. Exact scope of code changes
Map every mandatory R/AC/F item to committed implementation/tests, execute standard/extended/native/product/feature lanes and document unresolved blockers; do not mark optional embedding required.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/TRACEABILITY.md`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
cargo xtask check and verify plus all declared supported lanes pass; no new failures are waived and every unrun lane is explicit.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
cargo xtask verify
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Code/platform readiness is accurately established or explicitly blocked.

### 8. Commit message
```text
test: complete product v1 acceptance verification
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s210"></a>
## S210 — Record enforcement approval or explicit non-activation

**Predecessor:** S209. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/BASELINE.md](../../repo/BASELINE.md), [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Record enforcement approval or explicit non-activation.

### 2. Purpose
Keep operational permission separate from code completion.

### 3. Exact scope of code changes
Have the authorized owner review actual identities/destinations/cohorts/host guarantees and shadow evidence; record enforce-or-remain-shadow disposition without deploying from this coding task.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/runbooks/RELEASE_OPERATIONS.md`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `approved operator config path from Q09`

### 5. Unit tests or integration tests required
Activation record binds exact evaluator/authority/configuration; absent approval leaves enforcement disabled; deployment consumer rechecks exact authorization.

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
- Discover authorized CI/promotion validation commands and exact protected identities/destinations through Q04/Q09/Q15. Do not execute publication/deployment or invent operational approval as part of this code step.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
No one mistakes a green build or signed report for permission to promote.

### 8. Commit message
```text
docs: record release gate activation disposition
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s211"></a>
## S211 — Finalize the implementation continuation record

**Predecessor:** S210. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/BASELINE.md](../../repo/BASELINE.md), [runbooks/OPERATIONS.md](../../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Finalize the implementation continuation record.

### 2. Purpose
Leave a reconstructable audited handoff after coding.

### 3. Exact scope of code changes
Update progress ledger, completed step reports, deviations, open decisions and final report with actual commit SHAs/tests/risks; no claimed test runs are inferred or fabricated.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/`
- `docs/eqm_product_v1/summary.txt`

### 5. Unit tests or integration tests required
Traceability/link/integrity checks pass for evolved docs; Git history matches reports and final status separates code/platform/operational completion.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
cargo xtask verify
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Another agent or maintainer can reconstruct exactly what shipped and what remains unverified.

### 8. Commit message
```text
docs: finalize the product v1 implementation record
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
