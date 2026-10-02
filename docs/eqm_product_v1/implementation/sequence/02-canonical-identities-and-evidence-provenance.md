# Chapter 02: Canonical identities and evidence provenance

**Current authority:** DEV-011, S025-R2 → S026-R1 → S027–S043;
24 numbered steps accepted, independent integration review before S044.
Historical frontier paragraphs below are superseded by the current sole plan.


**Current authority:** DEV-010 in the sole COMMIT_SEQUENCE; 23 numbered steps
accepted for allocated scope. S022-R2 → S025-R1 → S026–S043; independent
integration review before S044. Historical amendment text below is subordinate
to that current frontier. All original criteria remain.
Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P02, containing 15 separate ordered commits, S013–S027; it is not one combined commit. Live completion is recorded in progress.json; S001–S018 and S020 accepted. DEV-008 dependency repair, S019-R3/S021-R1 and S022 precede all eligible successors to S211.

**Definition of green:** Canonical component/subject/raw/semantic/attempt identities and dirty-source provenance pass mutation vectors.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q03, Q14. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s013"></a>
## S013 — Add domain-specific identity wrappers

**Predecessor:** S012. **Contracts:** R18, R19, R50. **Acceptance:** AC018, AC019, AC050.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Add domain-specific identity wrappers.

### 2. Purpose
Prevent interchange of unrelated digest meanings.

### 3. Exact scope of code changes
Extend digest/ID types with the approved raw-object, semantic-evidence, plan/campaign/attempt and evaluation-context distinctions; use private validated construction and documented domains.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/digest.rs`
- `crates/eqm_domain/src/id.rs`
- `crates/eqm_domain/src/lib.rs`

### 5. Unit tests or integration tests required
Invalid/foreign digest domains reject; round trips and equality remain deterministic; no Default success identity.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Internal APIs cannot accidentally pass a workspace digest where a distinct identity is required.

### 8. Commit message
```text
feat: distinguish conformance identity domains
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s014"></a>
## S014 — Reject duplicate JSON members at ingestion

**Predecessor:** S013. **Contracts:** R18, R19, R21, R42, R64. **Acceptance:** AC018, AC019, AC021, AC042, AC064.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Reject duplicate JSON members at ingestion.

### 2. Purpose
Prevent ambiguous canonical or signed payload interpretation.

### 3. Exact scope of code changes
Add a bounded strict JSON ingestion path for protocol records that detects duplicate object names before conversion/canonicalization; preserve original raw bytes.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_protocol/tests/`

### 5. Unit tests or integration tests required
Duplicate top-level/nested keys reject; valid records retain exact raw bytes; malformed/deep input remains bounded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence is not normalized through an ambiguous last-key-wins parse.

### 8. Commit message
```text
fix: reject duplicate protocol object members
```

**Approved review regression contributions:** RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s015"></a>
## S015 — Enforce approved numeric and temporal wire forms

**Predecessor:** S014. **Contracts:** R18, R19, R21, R52, R64. **Acceptance:** AC018, AC019, AC021, AC052, AC064.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enforce approved numeric and temporal wire forms.

### 2. Purpose
Make cross-runtime digests predictable.

### 3. Exact scope of code changes
Retain DEV-002 S015-R1 through S015-R5 and complete DEV-003 S015-R6 through S015-R9: close exact version/kind ingress and nested inventory validation; repair runtime-facts byte/count bounds and retained subject syntax checks; enforce v2 schema markers and minimal UTC fractions; use actual schema validation with separate raw-token expectations; replace tautological corpus/application/CLI controls and reconcile evidence before S016.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/time.rs`
- `crates/eqm_domain/src/schema.rs`
- `crates/eqm_protocol/src/bin/generate_protocol_schemas.rs`
- `tools/xtask/src/main.rs`
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/`
- `crates/eqm_runner/src/inventory.rs`
- `crates/eqm_test_support/`
- `schemas/`
- `crates/eqm_protocol/tests/`
- `crates/eqm_manifest/src/`
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_protocol/Cargo.toml`

### 5. Unit tests or integration tests required
All DEV-002 and DEV-003 controls pass: full integer endpoints, canonical raw tokens, UTC/calendar/absence rules, exact root/nested versions, v1 preservation, all five schemas validated with Draft 2020-12, actual 16 MiB runtime-facts and fact-count bounds, literal corpus execution, successful reconcile control and fake-reader integration. Application doctests and schema parity pass; JSON-value conformance is distinguished from raw-token and cross-field validation.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_domain -p eqm_manifest -p eqm_application -p eqm -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
cargo test --locked -p eqm_application --doc
git diff --check
```

**Admission gate:** DEV-005: S015 accepted for allocated scope; S016-P1 also accepted. Retain corrective history and proceed to S016-R1. Later semantic/product acceptance remains with its owning steps.

### 7. Expected result
No consumer silently changes a number or time before hashing.

### 8. Commit message
```text
fix: enforce canonical numeric and time representations
```

**Approved review regression contributions:** RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s016"></a>
## S016 — Add canonical set and Unicode vectors

**Predecessor:** S015. **Contracts:** R18, R19, R21, R43, R64. **Acceptance:** AC018, AC019, AC021, AC043, AC064.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../../decisions/ADR-0013-wire-namespace-and-representations.md), [implementation/reports/CODEX-121622f.md](../../implementation/reports/CODEX-121622f.md), [implementation/reports/CODEX-6837831.md](../../implementation/reports/CODEX-6837831.md), [implementation/reports/CODEX-3469fac.md](../../implementation/reports/CODEX-3469fac.md), [implementation/reports/CODEX-1512c9f.md](../../implementation/reports/CODEX-1512c9f.md).

### 1. Step title
Add canonical set and Unicode vectors.

### 2. Purpose
Distinguish semantic unordered collections from ordered scenarios.

### 3. Exact scope of code changes
S016 implementation and RF22 matrix are accepted by Codex DEV-007. Preserve the real-root canonical collection, NFC, raw-byte, duplicate and diagnostic regressions.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/`
- `crates/eqm_test_support/`
- `crates/eqm_engine/Cargo.toml`
- `schemas/v2/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Retain all executed S016-R1/R2/R3 controls and their independent evidence; RF22 is accepted for allocated scope.

### 6. Verification commands
```sh
cargo audit --deny warnings
cargo deny check
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_protocol -p eqm_domain -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

**Admission gate:** DEV-007 accepts S016; next active repair is S019-R1 affecting retained S017/S019 implementations.

### 7. Expected result
Canonicalization is executable protocol behavior rather than serializer convention.

### 8. Commit message
```text
test: lock canonical collection and string semantics
```

**Approved review regression contributions:** RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s017"></a>
## S017 — Hash complete binding semantics

**Predecessor:** S016. **Contracts:** R19. **Acceptance:** AC019.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../../decisions/ADR-0013-wire-namespace-and-representations.md), [implementation/reports/CODEX-6837831.md](../../implementation/reports/CODEX-6837831.md), [implementation/reports/CODEX-3469fac.md](../../implementation/reports/CODEX-3469fac.md), [implementation/reports/CODEX-1512c9f.md](../../implementation/reports/CODEX-1512c9f.md).

### 1. Step title
Hash complete binding semantics.

### 2. Purpose
Correct the ID-only binding digest shortcut.

### 3. Exact scope of code changes
Retain the complete typed v2 binding projection and accepted RF24/RF26 repairs. DEV-007 S019-R1 corrects all/any to canonical semantic sets shared with contract applicability; preserve v1 and unaffected literal oracles.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical.rs`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_domain/src/digest.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Retain typed API, extension type distinction, actual loader formatting/mutation and projected-duplicate controls. Add DEV-007 RF27 genuine all/any permutations, duplicate rejection and heterogeneous JCS-order traps; retain semantic mutation and ordered extension tests.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Binding identity binds actual approved semantics.

### 8. Commit message
```text
fix: bind evidence to canonical binding semantics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s018"></a>
## S018 — Hash complete evidence specifications

**Predecessor:** S017. **Contracts:** R19. **Acceptance:** AC019.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [implementation/reports/CODEX-3469fac.md](../../implementation/reports/CODEX-3469fac.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../../decisions/ADR-0013-wire-namespace-and-representations.md), [implementation/reports/CODEX-1512c9f.md](../../implementation/reports/CODEX-1512c9f.md).

### 1. Step title
Hash complete evidence specifications.

### 2. Purpose
Correct the ID-pair evidence-specification shortcut.

### 3. Exact scope of code changes
Complete v2 evidence-specification projection and typed identity accepted by Codex DEV-007, including recursive extension tags, literal controls and actual manifest-boundary evidence. Retain the original and repair commits. RF25 ordered-child misclassification is superseded by RF27 in S019-R1.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_domain/src/evidence.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Retain selector/kind/count/runner/freshness mutations. Exercise integer-versus-text extension pairs and nested arrays/objects at both component projections, typed return and compile-fail domain substitution, actual loader/display/duplicate/expression controls, unchanged empty-extension and v1 goldens. Execute manifest doctests.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --locked -p eqm_manifest --doc
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence cannot match a changed specification merely by retaining names.

### 8. Commit message
```text
fix: hash exact evidence specification semantics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s019"></a>
## S019 — Separate contract and policy digests

**Predecessor:** S018. **Contracts:** R19, R58. **Acceptance:** AC019, AC058.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [implementation/reports/CODEX-3469fac.md](../../implementation/reports/CODEX-3469fac.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../../decisions/ADR-0013-wire-namespace-and-representations.md), [implementation/reports/CODEX-1512c9f.md](../../implementation/reports/CODEX-1512c9f.md), [implementation/reports/CODEX-eb2f323.md](../../implementation/reports/CODEX-eb2f323.md).

### 1. Step title
Separate contract and policy digests.

### 2. Purpose
Stop populating unrelated provenance with one workspace digest.

### 3. Exact scope of code changes
Retain accepted applicability repair and implemented DEV-006/007 provenance assembly. Finish DEV-008 S019-R3: real minimum rule projection, literal endpoint expectations, two-rule JCS order trap and absent dimension-description control. Existing requirements, crate directions and v1 forms remain.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_domain/tests/`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_manifest/src/lib.rs`
- `crates/eqm_manifest/tests/`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_application/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Retain all S019-R1/R2 controls and execute DEV-008 remaining RF28 literal/production-boundary assertions with valid controls; preserve original complete provenance matrix.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_manifest -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --locked -p eqm_domain -p eqm_manifest -p eqm_application --doc
cargo xtask check
cargo xtask schemas check
git diff --check
```

**Admission gate:** DEV-008: complete DEV008-P1, then S019-R3, S021-R1 and S022 completion before ordered successors. No extra approval pause follows objective green checks.

### 7. Expected result
Internal prepared context has distinct accurate contract/policy identities.

### 8. Commit message
```text
fix: separate contract and policy provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s020"></a>
## S020 — Separate target configuration identity

**Predecessor:** S019. **Contracts:** R19, R22. **Acceptance:** AC019, AC022.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [implementation/reports/CODEX-1512c9f.md](../../implementation/reports/CODEX-1512c9f.md).

### 1. Step title
Separate target configuration identity.

### 2. Purpose
Bind actual target configuration rather than the full graph summary.

### 3. Exact scope of code changes
Implement DEV-007 exact current-target configuration: pure manifest projection over Target with root exactly id/root/platform/framework/owners/extensions, existing TargetConfigurationDigest, and same-PreparedSession exact TargetId lookup with typed missing/projection errors. No invented build/profile/source fields or workspace fallback; retain separate subject/policy axes and later projection-extension obligations.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Literal bytes and independent domain hash; typed API; same-ID root/platform/framework/owner/semantic-extension changes; owner permutations; integer/text distinction; display and actual formatting stability; unrelated-target/policy isolation; exact missing-target error and same-session reuse.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_domain -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** DEV-007 resolves S020 representation; start after green S019-R1/R2. Model only present Target fields; later role/source owners extend projection atomically.

### 7. Expected result
Target evidence can be checked against a meaningful configuration identity.

### 8. Commit message
```text
fix: derive exact target configuration identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s021"></a>
## S021 — Bind runner identity to approved executable bytes

**Predecessor:** S020. **Contracts:** R19, R30. **Acceptance:** AC019, AC030.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [implementation/reports/CODEX-eb2f323.md](../../implementation/reports/CODEX-eb2f323.md).

### 1. Step title
Bind runner identity to approved executable bytes.

### 2. Purpose
Preserve executable provenance in persisted evidence.

### 3. Exact scope of code changes
Retain executable-bound resolution but complete DEV-008 S021-R1: explicit bounded runner-owned v2 JCS projection of full definition, resolved program and approved guarantees; existing RunnerDigest, typed identity_v2/canonical_bytes accessors and explicit digest wire adapter. Preserve v1 shapes/oracles, unresolved rejection and record runtime identity change.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/resolution.rs`
- `crates/eqm_runner/tests/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Execute DEV-008 RF30 literal v2 bytes/domain hash, typed/compile-fail doctests, real executable acquisition/replacement and map-boundary controls, definition/context changes, locked/repository/unresolved cases, full-range quantities, template/literal distinction, set/argv order and extension/display semantics.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --locked -p eqm_runner --doc
cargo xtask check
git diff --check
```

### 7. Expected result
Runner evidence cannot reuse a definition digest after executable substitution.

### 8. Commit message
```text
fix: include executable content in runner identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s022"></a>
## S022 — Normalize repository and source provenance explicitly

**Predecessor:** S021. **Contracts:** R19, R22. **Acceptance:** AC019, AC022.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [implementation/reports/CODEX-eb2f323.md](../../implementation/reports/CODEX-eb2f323.md).

### 1. Step title
Normalize repository and source provenance explicitly.

### 2. Purpose
Prevent display remote strings and dirty HEAD from implying exact clean source.

### 3. Exact scope of code changes
Complete DEV-008 conservative source inspection and clean release gate. Inspect repository-wide tracked/index/untracked/ignored state with only reserved untracked .eqm outputs excluded, reject Git concealment flags, preserve typed identity/error handling and causal failures. Move generated fixture inputs to .eqm before enforcing clean provenance; preserve original release outcomes and separate actual execution/subject claims. DEV-010 S022-R2 requires ancestor symlink and canonical descendant confinement before any nested Git invocation, preserving recursive conservative provenance.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_cli/src/commands/attest.rs`

### 5. Unit tests or integration tests required
Execute DEV-008 actual clean Pass/unconfigured Unknown controls, generated-output neutrality, tracked/staged/deleted/untracked/ignored non-output and tracked-output non-authorizing mutants, concealment flags and submodule/path controls. Preserve normalization failures, original release outcomes and golden evidence.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** DEV-008 resolves the representation block. Execute after DEV008-P1, S019-R3 and S021-R1; no requirement is deferred. Continue every ordered eligible S023-S211 after green completion.

### 7. Expected result
Source identity reflects inspected/executed bytes and shared identity logic replaces copies.

### 8. Commit message
```text
fix: bind source provenance beyond clean head claims
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s023"></a>
## S023 — Model raw and semantic evidence identities

**Predecessor:** S022. **Contracts:** R18, R21, R22, R35. **Acceptance:** AC018, AC021, AC022, AC035.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Model raw and semantic evidence identities.

### 2. Purpose
Preserve exact signed bytes independently of normalized assertions.

### 3. Exact scope of code changes
Add validated raw-object/semantic-evidence relation types and conversion results without storage or trust promotion; keep existing records readable only under the version ADR. DEV-009 S023-R1 repairs raw-record binding and lossless explicit v2 semantic projection; newly added v1 identity helper is not a v2 contract.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_protocol/tests/`

### 5. Unit tests or integration tests required
Alternate whitespace transports differ in raw identity but agree semantically; signed originals are returned unchanged; changed assertions differ. Also execute raw mismatch rejection, neighboring large-value separation, all assertion-axis mutations, canonical sets/ordered attempts, bounds and independently computed complete literal v2 bytes/digest.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence normalization no longer conflates raw collision with semantic identity.

### 8. Commit message
```text
feat: separate raw and semantic evidence identity
```

**Approved review regression contributions:** RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s024"></a>
## S024 — Model plans campaigns and attempts

**Predecessor:** S023. **Contracts:** R16, R17, R18. **Acceptance:** AC016, AC017, AC018.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Model plans campaigns and attempts.

### 2. Purpose
Establish exact acquisition lineage before selection/persistence.

### 3. Exact scope of code changes
Add validated plan/campaign/run/logical-case/attempt identifiers and explicit relationships; reject inconsistent or missing required lineage instead of deriving it from timestamps. DEV-009 S024-R1 adds exact lineage validation against declared execution and independently supplied expected plan/campaign context.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_domain/src/id.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Duplicate attempt names under the same execution reject ambiguous construction; distinct runs/cases remain independent. Test every missing and mismatched lineage axis, attempt membership and run/case independence separately.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
The engine can reason about attempts without counting JSON files.

### 8. Commit message
```text
feat: model evidence execution lineage
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s025"></a>
## S025 — Validate evidence identity relationships

**Predecessor:** S024. **Contracts:** R14, R15, R16, R18, R19, R21, R64. **Acceptance:** AC014, AC015, AC016, AC018, AC019, AC021, AC064.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate evidence identity relationships.

### 2. Purpose
Catch contradictions among envelope and payload subjects.

### 3. Exact scope of code changes
Validate matching scope/target/unit/requirements/facets/kind and identity lineage across protocol-to-domain conversion; require explicit typed absence where allowed. DEV-009 freezes checked bytes-first v2 admission with explicit lineage/execution/expected typed context, recomputed semantic identity, local relationship/count/time checks and no trust promotion or implicit v1 conversion. DEV-010 S025-R1 retains existing typed component expectations and checked access, validates ordered retry chronology at v2 admission, with full one-axis real-ingress controls; v1 stays unchanged. DEV-011 S025-R2 adds actual prepared-session/projection-to-admission integration, real context/access compile-fail controls and every execution-bearing temporal matrix.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_protocol/tests/`
- `crates/eqm_application/tests/`
- `crates/eqm_application/Cargo.toml`
- `Cargo.lock`

### 5. Unit tests or integration tests required
Mismatched payload kind, subject, attempt or semantic digest reject before evaluation; valid fixtures round-trip. Use genuine positive fixtures and one-axis RA02/RA13 mutants, including mutation after DTO parsing; no helper-only or self-hash acceptance.

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
Structurally valid JSON cannot create internally contradictory evidence.

### 8. Commit message
```text
fix: validate evidence envelope relationships
```

**Approved review regression contributions:** RA02, RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s026"></a>
## S026 — Model the acyclic record dependency chain

**Predecessor:** S025. **Contracts:** R20, R28. **Acceptance:** AC020, AC028.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Model the acyclic record dependency chain.

### 2. Purpose
Prevent evidence-to-decision digest cycles.

### 3. Exact scope of code changes
Add candidate/plan/input-manifest/decision/reference distinctions and validate directed references before publication; do not require future release decisions in evidence. DEV-009 allocates typed record-kind references to already-defined supplied registry entries along the approved acyclic lifecycle; reject missing/self/future/cyclic references without storage/publication. DEV-011 S026-R1 freezes explicit definition-history order, already-defined historical deployment inputs for Runtime/FrozenInput, forbidden forward definitions and sorted typed references while preserving cycle/direction checks.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_protocol/src/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Candidate-to-evidence-to-decision chains validate; self/future/cyclic record references reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Readiness records can be created in a realizable order.

### 8. Commit message
```text
feat: enforce acyclic evaluation record identities
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s027"></a>
## S027 — Add cross-domain provenance regression corpus

**Predecessor:** S026. **Contracts:** R18, R19, R20, R21, R22. **Acceptance:** AC018, AC019, AC020, AC021, AC022.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Add cross-domain provenance regression corpus.

### 2. Purpose
Lock identity semantics before target and evaluator changes.

### 3. Exact scope of code changes
Consolidate approved raw/semantic/component/source/lineage vectors into shared fixtures and golden identities without live credentials. DEV-009 includes all repaired raw/semantic/nested-source/runner/lineage controls and complete literal oracles in real owning boundaries, not only declarative vector inventory.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `crates/eqm_test_support/src/`
- `crates/eqm_protocol/tests/`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Mutation of every bound axis prevents inappropriate reuse; formatting-only changes behave as declared; fixture generator is deterministic.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support -p eqm_protocol -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
Future layers consume tested identity contracts with stable regression evidence.

### 8. Commit message
```text
test: lock conformance provenance boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
