# Chapter 08: Protected authority and real verification

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P08, containing 16 separate ordered commits, S116–S131; it is not one combined commit. All implementation steps are pending.

**Definition of green:** Independent protected authority and real test cryptography reject every missing/wrong trust dimension; no production test bypass.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q04. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s116"></a>
## S116 — Model protected authority configuration

**Predecessor:** S115. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Model protected authority configuration.

### 2. Purpose
Define authority independently of candidate files.

### 3. Exact scope of code changes
Add strict operator configuration for baseline expectations, required scope, verifier/evaluator identity and authorized trust dimensions; no live values or permissive defaults.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/`

### 5. Unit tests or integration tests required
Missing authority, duplicate identities and candidate-only release origin reject; test-only configs remain labeled.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Protected configuration is explicit validated input.

### 8. Commit message
```text
feat: configure protected evaluation authority
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s117"></a>
## S117 — Load protected baseline expectations

**Predecessor:** S116. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Load protected baseline expectations.

### 2. Purpose
Preserve protected contract participation and selections.

### 3. Exact scope of code changes
Acquire exact protected authority through an authorized loader independent of candidate paths; bind baseline contract/policy/participation/evidence mappings and profile values.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Candidate policy/default/path changes cannot substitute authority; missing exact baseline fails closed.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Protected evaluation receives exact independent expectations.

### 8. Commit message
```text
feat: load exact protected evaluation baselines
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s118"></a>
## S118 — Detect protected scope weakening

**Predecessor:** S117. **Contracts:** R14, R23, R24, R25, R26, R29, R38, R51, R58, R64. **Acceptance:** AC014, AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC058, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Detect protected scope weakening.

### 2. Purpose
Include deletions and reclassification in policy comparison.

### 3. Exact scope of code changes
Extend monotonicity/diff checks to required targets, requirements, participation, critical test selections and evidence mappings; allow changes only through approved authority transition.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/monotonicity.rs`
- `crates/eqm_engine/src/diff.rs`
- `crates/eqm_application/src/authority.rs`

### 5. Unit tests or integration tests required
Deleting/reclassifying a critical requirement/test/target cannot shrink protected work; approved versioned transitions are explicit.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
A candidate cannot pass by deleting what would fail.

### 8. Commit message
```text
fix: detect protected conformance scope weakening
```

**Approved review regression contributions:** RA04.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s119"></a>
## S119 — Bind the trusted evaluator identity

**Predecessor:** S118. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Bind the trusted evaluator identity.

### 2. Purpose
Prevent candidate code from grading itself in release mode.

### 3. Exact scope of code changes
Include approved EQM evaluator identity/provenance in protected preparation and receipts; reject candidate-modified/unapproved grader execution as trusted.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Substituted grader binary/version/config cannot produce accepted trusted preparation merely using the same candidate workspace.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Release assurance is tied to independently approved grading infrastructure.

### 8. Commit message
```text
feat: bind protected evaluator identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s120"></a>
## S120 — Define structured verification adapter results

**Predecessor:** S119. **Contracts:** R11, R23, R24, R25, R26, R29, R38, R50, R51, R63, R64. **Acceptance:** AC011, AC023, AC024, AC025, AC026, AC029, AC038, AC050, AC051, AC063, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Define structured verification adapter results.

### 2. Purpose
Avoid a single trust-level integer standing for all checks.

### 3. Exact scope of code changes
Implement verifier request/result contracts covering raw bytes, identity/issuer, signer authorization, builder, subject, policy and temporal material; construction crosses trusted adapter boundary.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust.rs`
- `crates/eqm_runner/src/ci.rs`
- `crates/eqm_engine/src/conformance.rs`

### 5. Unit tests or integration tests required
Missing required dimension stays explicit; serialized verified flags cannot manufacture accepted facts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Verifiers return specific checked claims rather than unconditional SignedCi.

### 8. Commit message
```text
refactor: expose structured verification conclusions
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s121"></a>
## S121 — Pin a reviewed verification implementation

**Predecessor:** S120. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Pin a reviewed verification implementation.

### 2. Purpose
Connect real cryptography without custom primitives.

### 3. Exact scope of code changes
Inspect current Sigstore bundle support and select an audited exact library or safely supervised verifier tool behind the neutral interface; update pins/lock/ADR and feature implications.

### 4. Files/modules likely involved
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_application/Cargo.toml`
- `crates/eqm_application/src/trust/`
- `docs/eqm_product_v1/decisions/`

### 5. Unit tests or integration tests required
License/advisory/feature checks pass; compile the real implementation; no disabled subject/claim verification flags.

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

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
A production-capable cryptographic path exists without live authority being invented.

### 8. Commit message
```text
build: pin the reviewed evidence verifier
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s122"></a>
## S122 — Verify bundle integrity and exact signed bytes

**Predecessor:** S121. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Verify bundle integrity and exact signed bytes.

### 2. Purpose
Make signature verification real and byte-bound.

### 3. Exact scope of code changes
Implement bounded parsing and cryptographic verification of supported bundle material over the original payload, retaining exact raw/semantic relationships.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Valid test-only signed material verifies; tampering, malformed bundle, wrong payload/digest and unsupported algorithms reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Cryptographic verification runs rather than trusting a fixture boolean.

### 8. Commit message
```text
feat: verify exact evidence bundle integrity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s123"></a>
## S123 — Check signer identity issuer and authorization

**Predecessor:** S122. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Check signer identity issuer and authorization.

### 2. Purpose
Separate valid signatures from authorized evidence.

### 3. Exact scope of code changes
Enforce configured exact identities/issuers/allowed signers and scope authorization using verifier output; reject unrecognized or overbroad authority.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Correct signature from wrong issuer/signer or unauthorized scope rejects; correct configured fixture succeeds.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Signature validity alone cannot establish trusted EQM evidence.

### 8. Commit message
```text
feat: enforce authorized evidence signer identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s124"></a>
## S124 — Check builder source and artifact expectations

**Predecessor:** S123. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Check builder source and artifact expectations.

### 2. Purpose
Bind verified evidence to what was actually intended.

### 3. Exact scope of code changes
Validate producer/builder/repository/source/build/artifact/configuration/selection against protected expected subjects; reject unrecognized security-relevant inputs.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
One-axis subject/build/source substitution and unexpected builder parameters fail without dropping other reasons.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Verified evidence cannot be replayed for a different product subject.

### 8. Commit message
```text
feat: verify exact evidence producer expectations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s125"></a>
## S125 — Check temporal and trust-material eligibility

**Predecessor:** S124. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Check temporal and trust-material eligibility.

### 2. Purpose
Make missing/current verification material explicit.

### 3. Exact scope of code changes
Apply configured temporal/trust-root/revocation requirements and bounded material retrieval only through authorized acquisition; distinguish historical verification from current acceptance.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Expired/missing/wrong-time material and denied network retrieval yield explicit non-acceptance; frozen historical fixtures reproduce.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Trust material has explicit validity and no hidden network fallback.

### 8. Commit message
```text
feat: validate verification material eligibility
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s126"></a>
## S126 — Verify every mandatory input type

**Predecessor:** S125. **Contracts:** R23, R24, R25, R26, R28, R29, R38, R51, R59, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC028, AC029, AC038, AC051, AC059, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Verify every mandatory input type.

### 2. Purpose
Close gaps where absent record/runtime verification disappears.

### 3. Exact scope of code changes
Apply the checked verification path to evidence, candidate/release records and runtime observations; build a complete required verification set without filter_map loss.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Missing record or runtime receipt blocks despite signed evidence; unrelated optional rejected inputs remain distinct from required gaps.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Every required input has an explicit acceptance conclusion.

### 8. Commit message
```text
fix: require complete input verification
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s127"></a>
## S127 — Persist checked verification provenance

**Predecessor:** S126. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Persist checked verification provenance.

### 2. Purpose
Connect actual verifier receipts to durable objects.

### 3. Exact scope of code changes
Atomically associate real checked results/material with exact raw/semantic objects and authority/evaluator context; import still revalidates the needed boundary.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Reopen/reimport preserves verification provenance without trusting caller-set flags; altered receipt/material rejects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Durable verification supports reproducible audit without minting authority.

### 8. Commit message
```text
feat: persist checked verification provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s128"></a>
## S128 — Enforce protected waiver approvals

**Predecessor:** S127. **Contracts:** R19, R23, R24, R25, R26, R29, R38, R51, R58, R64. **Acceptance:** AC019, AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC058, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enforce protected waiver approvals.

### 2. Purpose
Keep waiver authority outside candidate/agent control.

### 3. Exact scope of code changes
Load and validate waiver approval/scope/time provenance through protected configuration; supply only eligible applications to the pure evaluator.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/waivers.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Candidate-created/widened/renewed waivers cannot self-approve; expired/unrelated approvals reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Conditional outcomes require explicit protected exception authority.

### 8. Commit message
```text
feat: verify protected waiver authority
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s129"></a>
## S129 — Disable synthetic trust in production surfaces

**Predecessor:** S128. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Disable synthetic trust in production surfaces.

### 2. Purpose
Ensure test helpers cannot bypass the real pipeline.

### 3. Exact scope of code changes
Remove or isolate production-reachable synthetic SignedCi factories/flags; keep explicit unconfigured development state without representing it as verified release trust.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_runner/src/ci.rs`
- `crates/eqm_application/src/trust/`
- `Cargo.toml`

### 5. Unit tests or integration tests required
Production API/feature tests cannot construct unconditional signed trust; test fixtures remain usable only in test boundaries.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Trusted release facts originate only from the configured checked path.

### 8. Commit message
```text
fix: exclude synthetic trust from production paths
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s130"></a>
## S130 — Add the cryptographic negative acceptance suite

**Predecessor:** S129. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Add the cryptographic negative acceptance suite.

### 2. Purpose
Pressure-test trust across all attack dimensions.

### 3. Exact scope of code changes
Combine exact positive signatures with wrong identity/issuer/builder/subject/attempt/replay/material/config/evaluator fixtures and persistent import cases.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
Actual verifier tests pass; unsupported live identity/network lanes remain explicitly unconfigured, not marked verified.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Trust assurance is backed by cryptographic and authorization regressions.

### 8. Commit message
```text
test: cover conformance verification attack cases
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s131"></a>
## S131 — Document trust activation and rotation

**Predecessor:** S130. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Document trust activation and rotation.

### 2. Purpose
Prepare operators without granting live signer permissions.

### 3. Exact scope of code changes
Add validated config examples using non-production placeholders, key/material rotation and historical/current acceptance procedures; record Q04/Q15 live blockers.

### 4. Files/modules likely involved
- `docs/evidence-and-trust.md`
- `docs/eqm_product_v1/runbooks/TRUST_OPERATIONS.md`
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`

### 5. Unit tests or integration tests required
Fixture config validation and offline rotation/revocation rehearsal pass; no real credentials/signers are embedded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask test security
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Production activation is explicit and operationally reviewable.

### 8. Commit message
```text
docs: define verified trust activation procedures
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
