# Chapter 02: Canonical identities and evidence provenance

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P02, containing 15 separate ordered commits, S013–S027; it is not one combined commit. Live completion is recorded in progress.json; S015 and S016-P1/R1 behavior are accepted; S016-R3 is next under DEV-006.

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

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../../decisions/ADR-0013-wire-namespace-and-representations.md), [implementation/reports/CODEX-121622f.md](../../implementation/reports/CODEX-121622f.md), [implementation/reports/CODEX-6837831.md](../../implementation/reports/CODEX-6837831.md), [implementation/reports/CODEX-3469fac.md](../../implementation/reports/CODEX-3469fac.md).

### 1. Step title
Add canonical set and Unicode vectors.

### 2. Purpose
Distinguish semantic unordered collections from ordered scenarios.

### 3. Exact scope of code changes
Retain S016-P1/R1/R2 and accepted RF20/RF21 repairs. Complete DEV-006 S016-R3 for the explicit missing exact-duplicate, attachment-permutation, runtime-profile conflict and diagnostic-vector regressions; preserve prior behavior and v1 forms. RF22 is partial until this matrix passes.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/`
- `crates/eqm_test_support/`
- `crates/eqm_engine/Cargo.toml`
- `schemas/v2/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Run the concrete DEV-006 RF22 matrix at owning v2 roots with valid controls, exact and logical-key duplicate distinctions, recomputed mutant digests, and JCS comparison of returned semantic values. Diagnostic order/multiplicity tests must change the diagnostic vector itself; retain related-location, attempt, NFC, raw-byte and S015 controls.

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

**Admission gate:** DEV-006: S016-R1 behavior accepted; execute S016-R3, then S018-R1/R2 contributions for S017/S018, then S019-A/B/C and ordered unblocked successors through S211.

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

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../../decisions/ADR-0013-wire-namespace-and-representations.md), [implementation/reports/CODEX-6837831.md](../../implementation/reports/CODEX-6837831.md), [implementation/reports/CODEX-3469fac.md](../../implementation/reports/CODEX-3469fac.md).

### 1. Step title
Hash complete binding semantics.

### 2. Purpose
Correct the ID-only binding digest shortcut.

### 3. Exact scope of code changes
Retain the separate complete v2 binding projection. Close DEV-006 RF24/RF25/RF26 via S018-R1/R2: typed BindingDigest output, recursively tagged ExtensionValue semantics, actual manifest parse-provenance/duplicate/ordered-expression controls, corrected reports. Preserve all v1 bytes and unaffected v2 oracles; update only approved extension goldens.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical.rs`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_domain/src/digest.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Retain same-ID semantic mutation and literal controls. Add typed API/compile-fail proof, large integer-versus-text extension distinction including nested values, real loaded comment/formatting and semantic mutations, reachable post-display-exclusion duplicate rejection, and ordered all/any expression controls under DEV-006.

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

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [implementation/reports/CODEX-3469fac.md](../../implementation/reports/CODEX-3469fac.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../../decisions/ADR-0013-wire-namespace-and-representations.md).

### 1. Step title
Hash complete evidence specifications.

### 2. Purpose
Correct the ID-pair evidence-specification shortcut.

### 3. Exact scope of code changes
Retain complete v2 evidence-specification projection and shared helpers. Complete DEV-006 S018-R1/R2 jointly with S017 contributions: typed EvidenceSpecificationDigest/BindingDigest APIs, recursive type-tagged extensions that cannot collapse integer into text, independent changed-extension oracles, actual manifest-boundary controls and accurate evidence.

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

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [implementation/reports/CODEX-3469fac.md](../../implementation/reports/CODEX-3469fac.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../../decisions/ADR-0013-wire-namespace-and-representations.md).

### 1. Step title
Separate contract and policy digests.

### 2. Purpose
Stop populating unrelated provenance with one workspace digest.

### 3. Exact scope of code changes
Implement DEV-006 S019-A/B/C: validated domain PolicySelectionInputV2/SelectedProfileInputV2 carriers; explicit v2 contract projection over capabilities/journeys/surfaces/fragments plus normative graph extensions; complete selected policy/profile definitions and known/unknown selections; typed ContractDigest/PolicySelectionDigest and same-session application construction. Preserve engine selection algorithms, crate directions, v1 records and workspace aggregate identity.

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
Prove literal bytes/digests, integer endpoints and tagged extension distinctions, exact selections and explicit unknown versus missing/defaults, isolated contract/policy/profile/graph-extension mutations, unselected-definition stability, duplicate/undeclared input failures, ordered journey/expression arrays, canonical sets, real manifest formatting and same-session application mode/authority errors.

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

**Admission gate:** DEV-006 resolves the decision-required S019 block. After S016-R3 and S018-R1/R2, execute independently green S019-A, S019-B, S019-C; mark S019 complete only after C. Continue all ordered unblocked successors through S211.

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

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Separate target configuration identity.

### 2. Purpose
Bind actual target configuration rather than the full graph summary.

### 3. Exact scope of code changes
Define target configuration projection with relevant build/profile/source-component references from available data; preserve explicit absence rather than inventing tool metadata.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Target configuration changes invalidate its identity; unrelated display formatting does not; missing required config is explicit.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_domain -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

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

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md).

### 1. Step title
Bind runner identity to approved executable bytes.

### 2. Purpose
Preserve executable provenance in persisted evidence.

### 3. Exact scope of code changes
Extend resolved runner identity to include independently verified executable digest plus semantic definition and relevant approved toolchain context; keep unresolved programs non-executable.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/resolution.rs`
- `crates/eqm_runner/tests/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Replacing executable bytes at the same path changes the recorded runner identity; definition-only changes also invalidate it.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
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

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Normalize repository and source provenance explicitly.

### 2. Purpose
Prevent display remote strings and dirty HEAD from implying exact clean source.

### 3. Exact scope of code changes
Centralize repository identity acquisition/normalization and exact source-tree provenance in application code; retain unsupported/missing cases as errors; do not silently trust arbitrary remotes.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_cli/src/commands/attest.rs`

### 5. Unit tests or integration tests required
Equivalent supported remote encodings map deliberately; modified/untracked fixture content is labeled dirty and cannot satisfy clean release provenance.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

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
Add validated raw-object/semantic-evidence relation types and conversion results without storage or trust promotion; keep existing records readable only under the version ADR.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_protocol/tests/`

### 5. Unit tests or integration tests required
Alternate whitespace transports differ in raw identity but agree semantically; signed originals are returned unchanged; changed assertions differ.

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
Add validated plan/campaign/run/logical-case/attempt identifiers and explicit relationships; reject inconsistent or missing required lineage instead of deriving it from timestamps.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_domain/src/id.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Duplicate attempt names under the same execution reject ambiguous construction; distinct runs/cases remain independent.

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
Validate matching scope/target/unit/requirements/facets/kind and identity lineage across protocol-to-domain conversion; require explicit typed absence where allowed.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_protocol/tests/`

### 5. Unit tests or integration tests required
Mismatched payload kind, subject, attempt or semantic digest reject before evaluation; valid fixtures round-trip.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_domain
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
Add candidate/plan/input-manifest/decision/reference distinctions and validate directed references before publication; do not require future release decisions in evidence.

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
Consolidate approved raw/semantic/component/source/lineage vectors into shared fixtures and golden identities without live credentials.

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
