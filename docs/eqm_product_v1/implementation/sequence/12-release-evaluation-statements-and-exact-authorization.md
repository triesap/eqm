# Chapter 12: Release evaluation statements and exact authorization

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P12, containing 14 separate ordered commits, S165–S178; it is not one combined commit. All implementation steps are pending.

**Definition of green:** Complete release decisions, statements, signing and exact authorizations pass positive/negative consumption tests; shadow remains default.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q04, Q06, Q09. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s165"></a>
## S165 — Route release selection through protected preparation

**Predecessor:** S164. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Route release selection through protected preparation.

### 2. Purpose
Close the development-authority bypass.

### 3. Exact scope of code changes
Make release requests use exact protected authority/profile values and candidate subjects from application preparation, never the development derive helper/defaults.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/selection.rs`

### 5. Unit tests or integration tests required
Candidate-local policy/defaults and absent protected baseline reject; exact authorized fixture selection succeeds.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release evaluation cannot select its own weaker development authority.

### 8. Commit message
```text
fix: use protected authority for release evaluation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s166"></a>
## S166 — Evaluate releases from complete snapshots

**Predecessor:** S165. **Contracts:** R13, R19, R23, R24, R27, R28, R29, R38, R44, R52, R54, R58, R63. **Acceptance:** AC013, AC019, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC058, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate releases from complete snapshots.

### 2. Purpose
Remove target-only and duplicate outcome logic.

### 3. Exact scope of code changes
Route release checking through the shared complete decision including provider/scenario/unknown coordinates, canonical counts/retries/trust and applied waivers.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/release.rs`

### 5. Unit tests or integration tests required
Target passes with missing provider/end-to-end or failed manual review cannot pass; canonical facet results match check/matrix.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release conformance uses the same normative semantics and full denominator.

### 8. Commit message
```text
fix: evaluate complete release conformance snapshots
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s167"></a>
## S167 — Compare real release exposure facts

**Predecessor:** S166. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Compare real release exposure facts.

### 2. Purpose
Replace runtime-document presence with semantic match.

### 3. Exact scope of code changes
Use exact verified runtime observations and authored expectations to derive exposure comparisons for the candidate/deployment context; include mismatch reasons.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/exposure.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Present but wrong/stale/untrusted facts remain mismatch/unknown; exact complete verified facts satisfy only declared expectations.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release exposure reflects observed behavior rather than document existence.

### 8. Commit message
```text
fix: compare actual release exposure facts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s168"></a>
## S168 — Separate readiness and post-deployment records

**Predecessor:** S167. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Separate readiness and post-deployment records.

### 2. Purpose
Make the release lifecycle realizable and non-circular.

### 3. Exact scope of code changes
Expose approved candidate-readiness versus deployed-conformance record/use-case distinctions, with exact prior-record references and explicit time/material prerequisites.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/`
- `schemas/`

### 5. Unit tests or integration tests required
Initial readiness works without impossible future observations; post-deploy evaluation requires exact later receipts and rejects cyclic references.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release context can be evaluated before and after deployment honestly.

### 8. Commit message
```text
feat: separate readiness from deployed conformance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s169"></a>
## S169 — Require actual subject verification in release gates

**Predecessor:** S168. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Require actual subject verification in release gates.

### 2. Purpose
Remove self-confirming expected/observed record shortcuts.

### 3. Exact scope of code changes
Supply expected subjects independently from protected manifests and observed verified subjects from inputs; require every mandatory receipt and current acceptance before eligibility.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Wrong artifact/build/channel, missing record/runtime verification and stale acceptance reject despite otherwise signed evidence.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Gate inputs represent actual checked facts, not copied expected values.

### 8. Commit message
```text
fix: require verified exact release subjects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s170"></a>
## S170 — Emit attestations from evaluated snapshots

**Predecessor:** S169. **Contracts:** R13, R19, R23, R24, R27, R28, R29, R38, R44, R52, R54, R58, R63. **Acceptance:** AC013, AC019, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC058, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Emit attestations from evaluated snapshots.

### 2. Purpose
Replace unknown placeholders and ad hoc conformance calculations.

### 3. Exact scope of code changes
Build statements from one exact decision and its input references, actual contributing waivers and per-participant artifact subjects; preserve multi-target identities rather than requiring all subjects identical.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/attest.rs`
- `crates/eqm_application/src/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Passing/failed/unknown statements reproduce snapshot facts; different legitimate subjects are represented correctly and evidence subsets cannot alter the claim.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Attestation reports actual decision provenance without promoting evidence claims.

### 8. Commit message
```text
fix: attest exact evaluated conformance snapshots
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s171"></a>
## S171 — Add explicit signing through approved authority

**Predecessor:** S170. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Add explicit signing through approved authority.

### 2. Purpose
Support real signing without making it a release decision.

### 3. Exact scope of code changes
Invoke the selected reviewed signing provider only with explicit operator authority, bounded inputs and exact decision bytes; unsigned output remains available and accurately labeled.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_cli/src/commands/attest.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Test-only cryptographic signatures verify; missing authority rejects signing; signed failed/unknown statements remain non-authorizing.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Signing is a separate auditable operation rather than implicit trust promotion.

### 8. Commit message
```text
feat: sign decisions with explicit authority
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s172"></a>
## S172 — Model protected release authorization

**Predecessor:** S171. **Contracts:** R11, R13, R23, R24, R26, R27, R28, R29, R38, R44, R50, R52, R54, R63. **Acceptance:** AC011, AC013, AC023, AC024, AC026, AC027, AC028, AC029, AC038, AC044, AC050, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Model protected release authorization.

### 2. Purpose
Define the promotion boundary without building a deployment platform.

### 3. Exact scope of code changes
Add validated authorization request/record for exact decision/artifacts/participants/destination/channel/validity/allowed deviations under protected authority; finalize wire shape under version ADR.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/`
- `schemas/`

### 5. Unit tests or integration tests required
Arbitrary successful DTOs cannot construct authority; missing exact artifact/destination/policy facts reject.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release permission is a distinct exact protected record.

### 8. Commit message
```text
feat: model exact release authorization
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s173"></a>
## S173 — Authorize only currently acceptable decisions

**Predecessor:** S172. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Authorize only currently acceptable decisions.

### 2. Purpose
Keep unknown and unapproved conditional states from promotion.

### 3. Exact scope of code changes
Implement the protected authorization evaluator using complete current acceptance, verified exact subjects and explicit deviation permissions; do not deploy or publish.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Unknown/failure/stale/wrong-authority decisions reject; conditional requires exact approved deviations; real fixture pass authorizes only its configured subject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Signing or historical success alone never grants promotion.

### 8. Commit message
```text
feat: enforce protected release authorization
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s174"></a>
## S174 — Validate authorization at deployment consumption

**Predecessor:** S173. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Validate authorization at deployment consumption.

### 2. Purpose
Bind what is promoted to what was approved.

### 3. Exact scope of code changes
Add a host-neutral validator/receipt contract consumed by an existing deployment integration for exact artifacts/destination/channel/time; no new infrastructure orchestration.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Artifact/destination substitution, expired interval and replay for another deployment reject; exact permitted consumption produces a receipt.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Deployment consumers can enforce exact authorization rather than trusting a generic pass.

### 8. Commit message
```text
feat: validate exact promotion authorization
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s175"></a>
## S175 — Handle transformed artifacts explicitly

**Predecessor:** S174. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Handle transformed artifacts explicitly.

### 2. Purpose
Avoid accepting new bytes merely because the source commit matches.

### 3. Exact scope of code changes
Validate an approved transformation provenance relationship or require reevaluation when deployment artifacts differ; actual transformation policies remain owner-configured.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Same-commit different artifact without provenance rejects; configured verified transformation fixture follows its explicit policy.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Post-evaluation packaging transformations cannot silently bypass subject binding.

### 8. Commit message
```text
fix: verify transformed release artifact provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s176"></a>
## S176 — Default release enforcement to shadow mode

**Predecessor:** S175. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Default release enforcement to shadow mode.

### 2. Purpose
Prevent accidental production activation during implementation.

### 3. Exact scope of code changes
Add explicit operational configuration/admission separating evaluation/shadow reporting from enforcement; production identities/destinations and independent approval remain required.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/release_check.rs`
- `docs/eqm_product_v1/runbooks/RELEASE_OPERATIONS.md`

### 5. Unit tests or integration tests required
Unconfigured environment evaluates honestly but cannot enforce/promote; activation/reversion requires exact protected configuration.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
The completed gate is not automatically enabled against live releases.

### 8. Commit message
```text
feat: gate release enforcement activation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s177"></a>
## S177 — Test real positive and negative release lifecycle

**Predecessor:** S176. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Test real positive and negative release lifecycle.

### 2. Purpose
Prove trusted functionality rather than only fail-closed placeholders.

### 3. Exact scope of code changes
Execute fixture acquisition/import/verification/snapshot/signing/authorization/consumption plus subject/tamper/waiver/scope/expiry failures using real test-only cryptography.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `crates/eqm_cli tests`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Exact configured positive case succeeds through all boundaries; each mandatory negative case remains non-authorizing.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Production-capable release logic is exercised without live deployment or credentials.

### 8. Commit message
```text
test: verify exact release authorization lifecycle
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s178"></a>
## S178 — Remove duplicate release and attestation evaluators

**Predecessor:** S177. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Remove duplicate release and attestation evaluators.

### 2. Purpose
Leave one semantic implementation after parity tests.

### 3. Exact scope of code changes
Delete old count/freshness/status/trust/digest helpers and constant conclusions replaced by the shared pipeline; preserve only presentation and explicit capability wiring.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_cli/src/commands/attest.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`

### 5. Unit tests or integration tests required
Full CLI/MCP/snapshot/release parity and schema gates pass with no duplicated evaluator paths.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
cargo xtask test security
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
All release and statement conclusions are anchored to the canonical decision.

### 8. Commit message
```text
refactor: remove duplicate release decision logic
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
