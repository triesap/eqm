# Chapter 13: Conservative impact analysis and measured optimization

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P13, containing 12 separate ordered commits, S179–S190; it is not one combined commit. All implementation steps are pending.

**Definition of green:** Conservative impact and bounded batching/cache paths match the reference evaluator and measured representative budgets.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q07. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s179"></a>
## S179 — Index source and authority reverse dependencies

**Predecessor:** S178. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Index source and authority reverse dependencies.

### 2. Purpose
Support exact conservative affected selection.

### 3. Exact scope of code changes
Build reverse indexes for shared source components, targets, bindings, contracts, profiles, runners, adapters, trust and evaluator context to affected obligations.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/affected.rs`
- `crates/eqm_application/src/`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Shared commonMain changes reach all dependent outputs; authority changes select their dependent obligations deterministically.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Impact analysis uses the actual dependency model rather than one target root heuristic.

### 8. Commit message
```text
feat: index conformance reverse dependencies
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s180"></a>
## S180 — Handle deleted renamed and unclassified paths

**Predecessor:** S179. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Handle deleted renamed and unclassified paths.

### 2. Purpose
Prevent under-selection when source ownership is uncertain.

### 3. Exact scope of code changes
Extend change ingestion/baseline analysis for deletions/renames and conservative fallback on unknown paths or dependency gaps; keep reasons visible.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/affected.rs`
- `crates/eqm_cli/src/commands/affected.rs`
- `crates/eqm_cli/src/commands/diff.rs`

### 5. Unit tests or integration tests required
Deleted bindings/tests and renamed/shared/unmapped paths never shrink required reevaluation unsafely.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Uncertainty widens impacted work rather than dropping it.

### 8. Commit message
```text
fix: preserve conservative affected scope
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s181"></a>
## S181 — Wire verify affected selection to actual plans

**Predecessor:** S180. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Wire verify affected selection to actual plans.

### 2. Purpose
Repair the parsed-but-discarded baseline path.

### 3. Exact scope of code changes
Use exact baseline/impact results to select acquisition work and record conservative fallback and scope; bind the resulting plan to approval.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_engine/src/affected.rs`

### 5. Unit tests or integration tests required
Targeted baseline changes alter planned executions; unknown changes conservatively include required work; missing baseline rejects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
verify --affected performs tested dependency-based selection.

### 8. Commit message
```text
fix: apply affected analysis to verification plans
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s182"></a>
## S182 — Cache parsing and semantic projections

**Predecessor:** S181. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Cache parsing and semantic projections.

### 2. Purpose
Reuse pure stable work without caching trust decisions implicitly.

### 3. Exact scope of code changes
Add bounded stage cache keyed by exact source/canonicalization/schema identities; preserve component projection semantics and explicit invalidation.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Cached and uncached parse/projection results match; input/schema changes invalidate and limits remain enforced.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Stable preparation work can be reused safely.

### 8. Commit message
```text
perf: cache canonical preparation stages
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s183"></a>
## S183 — Cache prepared graph and obligation derivation

**Predecessor:** S182. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Cache prepared graph and obligation derivation.

### 2. Purpose
Avoid full graph reconstruction for unchanged exact contexts.

### 3. Exact scope of code changes
Cache validated graph/universe results with complete authority/profile/source dependency identities and bounded memory; do not cache caller assertions of completeness.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Changed participation/profile/fragment/policy invalidates correct entries; reference outputs match cached preparation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Warm derivation reuses exact validated work.

### 8. Commit message
```text
perf: reuse validated obligation preparation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s184"></a>
## S184 — Cache verification with trust-context binding

**Predecessor:** S183. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Cache verification with trust-context binding.

### 2. Purpose
Avoid repeated cryptography without perpetual trust.

### 3. Exact scope of code changes
Key reusable checked material by exact bytes/verifier/configuration/subject/material identity and validity, retaining separate historical/current semantics.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Trust-root/verifier/subject/material changes or expiry prevent reuse; immutable historical receipts remain available.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Verification caching cannot survive a relevant authority change silently.

### 8. Commit message
```text
perf: cache exact verification contexts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s185"></a>
## S185 — Expire decision cache entries correctly

**Predecessor:** S184. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Expire decision cache entries correctly.

### 2. Purpose
Prevent stale passes across time and authority transitions.

### 3. Exact scope of code changes
Add decision reuse eligibility with explicit validity boundaries and event invalidation; invalidate on relevant input/campaign/waiver/trust/evaluator changes.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/freshness.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Boundary-time and revocation/waiver/campaign changes reject cached acceptance; exact unchanged eligible input may reuse.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Current acceptance is never extended by cache lifetime.

### 8. Commit message
```text
fix: bound cached decision validity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s186"></a>
## S186 — Batch compatible native test work

**Predecessor:** S185. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Batch compatible native test work.

### 2. Purpose
Reduce process startup without changing observation counts.

### 3. Exact scope of code changes
Group approved cases by compatible runner/target/device/selection context while preserving exact manifest coverage and individual case/attempt lineage.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Batched/unbatched outcomes match; shared executions do not multiply count credit; incompatible devices or contexts remain separated.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Native startup overhead drops without weakening evidence semantics.

### 8. Commit message
```text
perf: batch compatible evidence acquisition
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s187"></a>
## S187 — Avoid redundant snapshot cloning and serialization

**Predecessor:** S186. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Avoid redundant snapshot cloning and serialization.

### 2. Purpose
Improve query efficiency after correctness is stable.

### 3. Exact scope of code changes
Reuse immutable shared graph/decision data and serialize only requested bounded views; avoid new mutable global caches or cross-snapshot aliases.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_engine/src/matrix.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Filtered/paginated output matches full reference slices; concurrent readers remain isolated and bounded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Large report consumers avoid repeated full graph copies and payloads.

### 8. Commit message
```text
perf: streamline immutable snapshot queries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s188"></a>
## S188 — Build representative end-to-end benchmarks

**Predecessor:** S187. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Build representative end-to-end benchmarks.

### 2. Purpose
Replace placeholder warm-context and incomplete cold timings.

### 3. Exact scope of code changes
Extend benchmark fixtures with real targets/profiles/evidence histories/trust/runtime facts/full matrices and measure complete preparation including canonicalization plus stage metrics.

### 4. Files/modules likely involved
- `tools/benchmarks/src/main.rs`
- `crates/eqm_test_support/src/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Benchmark verifies fixture semantics and required counts before timing; populated context and evidence evaluation are actually executed.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Run the owning benchmark/fixture tests and, when the populated benchmark step is reached, the actual cargo xtask benchmark lane on declared hardware; record measurements and host gaps, not thresholds alone.

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Performance reports measure the implemented assurance workload.

### 8. Commit message
```text
test: benchmark the complete conformance pipeline
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s189"></a>
## S189 — Add reference versus optimized differential tests

**Predecessor:** S188. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Add reference versus optimized differential tests.

### 2. Purpose
Protect semantics under caching batching and indexing.

### 3. Exact scope of code changes
Generate bounded randomized and fixed frozen inputs; compare canonical outcomes/reasons/identities for reference versus optimized paths including cache invalidation and reordered inputs.

### 4. Files/modules likely involved
- `crates/eqm_engine/tests/properties.rs`
- `crates/eqm_application/tests/`
- `tools/benchmarks/`

### 5. Unit tests or integration tests required
Any divergence fails; duplicates/missing/unknown/campaign/trust/expiry cases remain identical.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Optimization is continuously checked against the normative reference.

### 8. Commit message
```text
test: compare optimized and reference evaluation
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s190"></a>
## S190 — Establish measured performance budgets

**Predecessor:** S189. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Establish measured performance budgets.

### 2. Purpose
Avoid unsupported super-optimized claims or arbitrary limits.

### 3. Exact scope of code changes
Record actual supported-host workloads, latency distributions, memory/storage growth and contention; set reviewed budgets with Q07 rather than copying baseline constants.

### 4. Files/modules likely involved
- `tools/benchmarks/src/main.rs`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `docs/eqm_product_v1/specs/STORAGE_AND_PERFORMANCE.md`

### 5. Unit tests or integration tests required
Run benchmark lane on declared hardware and publish actual results plus unavailable-host gaps; correctness gates remain independent.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask benchmark
```

**Required command discovery:**
- Run the owning benchmark/fixture tests and, when the populated benchmark step is reached, the actual cargo xtask benchmark lane on declared hardware; record measurements and host gaps, not thresholds alone.

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Performance objectives are measured and reviewable, not inferred from code constants.

### 8. Commit message
```text
perf: establish measured conformance budgets
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
