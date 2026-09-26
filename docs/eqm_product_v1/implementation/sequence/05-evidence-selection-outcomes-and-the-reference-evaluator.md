# Chapter 05: Evidence selection outcomes and the reference evaluator

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P05, containing 18 separate ordered commits, S059–S076; it is not one combined commit. All implementation steps are pending.

**Definition of green:** Reference evaluator implements every evidence kind, count, retry, campaign, freshness and waiver rule; meaningful properties pass.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q05. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s059"></a>
## S059 — Convert protocol evidence through one validated boundary

**Predecessor:** S058. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R19, R21, R24, R29, R43, R64. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC021, AC024, AC029, AC043, AC064.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Convert protocol evidence through one validated boundary.

### 2. Purpose
Remove divergent domain construction paths.

### 3. Exact scope of code changes
Provide a single checked DTO-to-domain conversion for all currently supported evidence payloads and lineage; no trust conclusion is created by parsing.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Each valid kind converts; inconsistent or unsupported payload/subject/identity fields reject deterministically.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_domain -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Engine inputs share one strict conversion contract.

### 8. Commit message
```text
refactor: centralize validated evidence conversion
```

**Approved review regression contributions:** RA02, RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s060"></a>
## S060 — Classify duplicate transport and immutable conflicts

**Predecessor:** S059. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R22, R24, R29, R35, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC022, AC024, AC029, AC035, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Classify duplicate transport and immutable conflicts.

### 2. Purpose
Correct duplicate-ID poisoning without hiding contradictory attempts.

### 3. Exact scope of code changes
Add pure candidate-set normalization that deduplicates identical semantic delivery and distinguishes same-attempt conflicts; retain all raw/provenance references.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/coverage.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Duplicate transport leaves counts/status unchanged; conflicting authenticated same-attempt candidates remain non-authorizing in affected scope.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Harmless duplicate delivery no longer manufactures unknown coverage.

### 8. Commit message
```text
fix: distinguish duplicate evidence from attempt conflicts
```

**Approved review regression contributions:** RA03, RA08, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s061"></a>
## S061 — Match complete evidence coordinates

**Predecessor:** S060. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R19, R21, R24, R29, R35, R43, R64. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC021, AC024, AC029, AC035, AC043, AC064.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Match complete evidence coordinates.

### 2. Purpose
Bind every required coverage axis before aggregation.

### 3. Exact scope of code changes
Update coverage expectations for exact component digests, subject manifests, profiles, participant roles and campaign/selection lineage; preserve rejected-axis explanations.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/coverage.rs`
- `crates/eqm_engine/tests/coverage.rs`

### 5. Unit tests or integration tests required
One-axis substitution tests reject wrong build/spec/role/profile/runner/context; irrelevant rejected candidates cannot create success.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Only exact compatible evidence reaches outcome evaluation.

### 8. Commit message
```text
fix: match complete conformance evidence coordinates
```

**Approved review regression contributions:** RA03, RA08, RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s062"></a>
## S062 — Validate distinct logical observations

**Predecessor:** S061. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate distinct logical observations.

### 2. Purpose
Prevent file/attempt multiplication from meeting counts.

### 3. Exact scope of code changes
Implement approved logical-case identity and expected-selection completeness checks across normalized evidence; keep distinct case observations separate from retries.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/tests/outcomes.rs`

### 5. Unit tests or integration tests required
Split reports, duplicate imports and ten retries cannot replace missing distinct expected cases.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Count credit reflects the policy-defined logical unit.

### 8. Commit message
```text
fix: count distinct selected observations
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s063"></a>
## S063 — Apply policy-composed count constraints

**Predecessor:** S062. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Apply policy-composed count constraints.

### 2. Purpose
Resolve inconsistency between specification counts and stronger policy minimums.

### 3. Exact scope of code changes
Apply the coordinate's approved count semantics and strongest relevant minimum during evaluation, independently of runner acquisition summaries.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Policy minimum exceeding specification minimum remains missing; exact counts meet thresholds without file-count shortcuts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Policy count constraints cannot be bypassed by lower acquisition thresholds.

### 8. Commit message
```text
fix: enforce policy evidence count constraints
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s064"></a>
## S064 — Preserve retry history within a campaign

**Predecessor:** S063. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Preserve retry history within a campaign.

### 2. Purpose
Keep failure/pass instability visible.

### 3. Exact scope of code changes
Aggregate all immutable attempts for each selected case within the authorized campaign; implement canonical unknown/unstable/failure precedence and retain reasons.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Fail-then-pass is unstable; timeout/cancel remains unknown where normative; order permutations do not erase history.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Retrying never silently converts an unstable campaign into clean success.

### 8. Commit message
```text
fix: retain campaign retry history
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s065"></a>
## S065 — Evaluate manual review payloads explicitly

**Predecessor:** S064. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate manual review payloads explicitly.

### 2. Purpose
Remove the possibility of catch-all success for failed review.

### 3. Exact scope of code changes
Use explicit manual-review outcome/count/authority semantics in the canonical evaluator and reject unsupported interpretation.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/tests/outcomes.rs`

### 5. Unit tests or integration tests required
Passed, failed, skipped, quarantined, canceled and unverified review fixtures follow the normative table.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Manual review cannot become satisfied merely because its payload parsed.

### 8. Commit message
```text
fix: evaluate manual review outcomes explicitly
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s066"></a>
## S066 — Evaluate executable payload selections explicitly

**Predecessor:** S065. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate executable payload selections explicitly.

### 2. Purpose
Preserve skipped filtered and quarantined case semantics.

### 3. Exact scope of code changes
Unify tests/structural-check/snapshot payload aggregation over validated expected selections and attempts; prohibit terminal zero-exit shortcuts.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_runner/src/normalized.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Passed-with-missing/skipped/filtered/quarantined cases, zero selection and malformed counts remain non-success as specified.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Executable evidence uses the same complete case semantics across consumers.

### 8. Commit message
```text
fix: preserve executable evidence selection outcomes
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s067"></a>
## S067 — Evaluate inventory and runtime payloads explicitly

**Predecessor:** S066. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate inventory and runtime payloads explicitly.

### 2. Purpose
Do not treat non-test payload existence as success.

### 3. Exact scope of code changes
Define supported inventory/runtime/release-reference payload evaluation with exact counts, referenced objects, completeness and supporting prerequisites; reject unspecified kinds.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/src/exposure.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Incomplete inventory, failing counts, missing referenced record and wrong subject cannot satisfy an obligation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
All supported payload families have closed tested semantics.

### 8. Commit message
```text
fix: evaluate non-test evidence prerequisites
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s068"></a>
## S068 — Select campaigns under explicit authority

**Predecessor:** S067. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Select campaigns under explicit authority.

### 2. Purpose
Avoid arbitrary timestamp order and cherry-picked passing runs.

### 3. Exact scope of code changes
Implement pure campaign eligibility/ordering/supersession selection from the approved policy; return ambiguous or missing selection explicitly.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_engine/tests/selection.rs`
- `crates/eqm_domain/src/policy.rs`

### 5. Unit tests or integration tests required
Forged newer timestamps do not replace authority; a later authorized campaign can be selected without erasing older failures.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Current evidence selection is reproducible and audit history remains separate.

### 8. Commit message
```text
feat: select authorized evidence campaigns
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s069"></a>
## S069 — Evaluate exact freshness with explicit clock semantics

**Predecessor:** S068. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Evaluate exact freshness with explicit clock semantics.

### 2. Purpose
Align all consumers on temporal validity.

### 3. Exact scope of code changes
Consolidate age/skew and component-key freshness checks using approved precision and boundary rules; distinguish historical context from current checks without cache integration yet.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/freshness.rs`
- `crates/eqm_engine/tests/freshness.rs`

### 5. Unit tests or integration tests required
Exact expiry/skew boundaries, future times and each changed digest axis produce specified fresh/stale/unknown results.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence freshness is deterministic for explicit input time and context.

### 8. Commit message
```text
fix: unify exact evidence freshness semantics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s070"></a>
## S070 — Represent independent verification facts

**Predecessor:** S069. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R26, R29, R43, R50, R63. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC026, AC029, AC043, AC050, AC063.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Represent independent verification facts.

### 2. Purpose
Keep missing verification distinct from low verified trust.

### 3. Exact scope of code changes
Define the pure engine input for checked identity/issuer/subject/producer/temporal dimensions and their required completeness; no real cryptography in the engine.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_domain/src/`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
A missing mandatory dimension remains unknown; known insufficient authority remains non-success; producer claims cannot raise facts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Later verifier adapters must supply complete independent acceptance facts.

### 8. Commit message
```text
feat: model independent evidence verification facts
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s071"></a>
## S071 — Apply only precise contributing waivers

**Predecessor:** S070. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R19, R24, R29, R43, R58. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC024, AC029, AC043, AC058.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Apply only precise contributing waivers.

### 2. Purpose
Prevent unrelated or expired exceptions changing outcomes.

### 3. Exact scope of code changes
Route canonical waiver evaluation to exact scope/profile/subject/context and protected approval inputs; preserve baseline non-waivable unknown/unstable cases.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/waivers.rs`
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/tests/waivers.rs`

### 5. Unit tests or integration tests required
Expired, broad, unrelated, wrong-profile or unapproved waivers reject; only eligible blockers produce visible conditional state.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Waivers never become satisfied evidence or a graph-wide blanket list.

### 8. Commit message
```text
fix: bind waivers to exact contributing obligations
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s072"></a>
## S072 — Derive canonical facet decisions with full reasons

**Predecessor:** S071. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Derive canonical facet decisions with full reasons.

### 2. Purpose
Make one normative status implementation explainable.

### 3. Exact scope of code changes
Combine validated applicability, context, coverage, outcomes, freshness, verification, supporting checks and waivers into a structured facet decision retaining reasons.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/src/diagnostics.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Exhaustive precedence table tests include simultaneous unknown/failure/unstable/stale/missing and applied-waiver cases.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Summary precedence is explicit and no cause is lost through a single color/status.

### 8. Commit message
```text
refactor: derive explainable canonical facet decisions
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s073"></a>
## S073 — Aggregate complete participant conformance

**Predecessor:** S072. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Aggregate complete participant conformance.

### 2. Purpose
Remove target-only partial-map assurance.

### 3. Exact scope of code changes
Aggregate facets through validated universe and provider/scenario dependency participation; replace caller-supplied policy_resolution_complete booleans at internal callers.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/tests/`
- `crates/eqm_application/src/evaluation.rs`

### 5. Unit tests or integration tests required
Missing provider/end-to-end/unknown coordinates block appropriate participants; complete failures remain valid complete decisions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
No target is graded using a silently incomplete obligation subset.

### 8. Commit message
```text
fix: aggregate complete participant conformance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s074"></a>
## S074 — Derive contract-relative comparison conclusions

**Predecessor:** S073. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Derive contract-relative comparison conclusions.

### 2. Purpose
Compare required cohorts without conflating build equality.

### 3. Exact scope of code changes
Update equivalence evaluation to use approved comparison context and complete subject/cohort results; require declared relational evidence rather than inventing a general comparison language.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/equivalence.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Different legitimate builds compare; missing relational scenario or wrong cohort remains blocking; extra targets do not change required scope.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Equivalence claims are scoped and supported by the required relational evidence.

### 8. Commit message
```text
fix: scope equivalence to complete comparison contracts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s075"></a>
## S075 — Assemble a simple reference evaluation path

**Predecessor:** S074. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Assemble a simple reference evaluation path.

### 2. Purpose
Provide an independent semantic oracle for future optimization.

### 3. Exact scope of code changes
Compose the tested pure operations into an unoptimized reference evaluation over frozen fixtures with explicit provenance/reasons; keep no alternative production policy interpretation.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/`
- `crates/eqm_engine/tests/`
- `crates/eqm_test_support/src/`

### 5. Unit tests or integration tests required
Golden lifecycle cases match manually specified outcomes and all non-success states; input permutations give identical decisions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
A readable reference path establishes expected whole-evaluation behavior.

### 8. Commit message
```text
test: establish reference conformance evaluation
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s076"></a>
## S076 — Lock adversarial evidence regression fixtures

**Predecessor:** S075. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R23, R24, R29, R35, R43, R50. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC023, AC024, AC029, AC035, AC043, AC050.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Lock adversarial evidence regression fixtures.

### 2. Purpose
Validate the complete semantic slice before persistence and host work.

### 3. Exact scope of code changes
Add full fixture campaigns covering duplicates/conflicts/retries/counts/unknown scopes/mixed subjects and real non-test failures; update normative docs when representation choices are resolved.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `crates/eqm_engine/tests/`
- `docs/eqm_product_v1/specs/EVALUATION_SEMANTICS.md`

### 5. Unit tests or integration tests required
Reference path passes the adversarial matrix and stronger-policy/deleted-evidence monotonicity checks.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Semantic defects from the review have focused regression coverage before command routing.

### 8. Commit message
```text
test: cover adversarial evidence evaluation
```

**Approved review regression contributions:** RA03, RA05, RA08.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
