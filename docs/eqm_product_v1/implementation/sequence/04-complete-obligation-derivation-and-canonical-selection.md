<!-- DEV-034 supersedes current batch: P04-R4A only; all original criteria and later steps remain pending. -->
<!-- Current execution: DEV-033 in COMMIT_SEQUENCE.md; preserved partial P04-R4 then S052–S076. -->
# Chapter 04: Complete obligation derivation and canonical selection

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P04, containing 15 separate ordered commits, S044–S058; it is not one combined commit. Allocated S045/S046/S050/S051 are locally accepted; other scope remains unfinished.

DEV-033 retaining DEV-032 in the sole governing sequence authorizes P04-R4 then S052–S076,
continuing across P04/P05. Read its checked graph/universe/participation repairs,
shared request preparation, exact scenario bindings and expected manifest decisions.
Preserve every original step criterion below; Codex reviews before S077.

**Definition of green:** Complete protected universe, exact revision loading, fragment risk, prepared construction and facet constraints satisfy negative tests.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q05. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s044"></a>
## S044 — Move exact selection orchestration into application

**Predecessor:** S043. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Move exact selection orchestration into application.

### 2. Purpose
Remove CLI-only development selection assumptions.

### 3. Exact scope of code changes
Extract policy/profile request preparation into application operations with explicit evaluation mode and authority input; keep host verification separate.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_engine/src/selection.rs`

### 5. Unit tests or integration tests required
Development baseline behavior remains explicit and protected requests cannot take an implicit development branch.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Selection is reusable without deriving authority from the calling command name.

### 8. Commit message
```text
refactor: share exact policy selection preparation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s045"></a>
## S045 — Reject duplicate profile requests

**Predecessor:** S044. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Reject duplicate profile requests.

### 2. Purpose
Prevent silent last-write-wins profile selection.

### 3. Exact scope of code changes
Replace collection-overwrite request parsing with duplicate detection for profiles/dimensions/revisions and retain exact diagnostics.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_engine/src/selection.rs`

### 5. Unit tests or integration tests required
Duplicate same/conflicting profile values reject; distinct complete selections work.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Public selection cannot silently discard caller intent.

### 8. Commit message
```text
fix: reject duplicate profile selections
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s046"></a>
## S046 — Select exact policy and profile revisions

**Predecessor:** S045. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Select exact policy and profile revisions.

### 2. Purpose
Remove the one-development-policy restriction without guessing authority.

### 3. Exact scope of code changes
Accept exact selected references according to the approved API decision; resolve multiple revisions deterministically and require explicit selection when ambiguous.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_cli/src/cli.rs`
- `crates/eqm_manifest/src/validation.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Multiple stored policies/revisions no longer force arbitrary selection; ambiguous/missing authority rejects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Explicit policy selection works across development and protected modes.

### 8. Commit message
```text
feat: select exact policy and profile revisions
```

**Approved review regression contributions:** RA16.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s047"></a>
## S047 — Enumerate required evaluation coordinates

**Predecessor:** S046. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59, R60. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059, AC060.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enumerate required evaluation coordinates.

### 2. Purpose
Establish the full denominator before reading evidence.

### 3. Exact scope of code changes
Introduce a pure required-universe result for participating targets/providers/scenarios and profile/facet coordinates using approved authority.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Expected coordinates are complete and deterministic for asymmetric/shared-provider/repeated-actor fixtures.

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
Later evidence filtering cannot shrink required scope.

### 8. Commit message
```text
feat: enumerate the required evaluation universe
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s048"></a>
## S048 — Retain unknown applicability coordinates

**Predecessor:** S047. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59, R60. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059, AC060.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Retain unknown applicability coordinates.

### 2. Purpose
Repair skipped requirements that can disappear from completeness.

### 3. Exact scope of code changes
Carry explicit unknown and justified not-applicable coordinates alongside derived obligations; preserve requirement references and reasons.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/src/applicability.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Missing profile context remains in scope and cannot yield a vacuous conformant target.

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
Unknown applicability is visible and non-authorizing.

### 8. Commit message
```text
fix: retain unresolved applicability in evaluation scope
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s049"></a>
## S049 — Validate completeness construction

**Predecessor:** S048. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R23, R24, R26, R50, R58, R59, R60, R63. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC023, AC024, AC026, AC050, AC058, AC059, AC060, AC063.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md), [implementation/DEVIATIONS.md](../../implementation/DEVIATIONS.md).

### 1. Step title
Validate completeness construction.

### 2. Purpose
Replace freely asserted completeness with a checked boundary.

### 3. Exact scope of code changes
Apply DEV-001: preserve the stronger fragment/journey inherited risk, reject explicitly lower requirement risk, and revalidate the expanded graph before finalization. Close unchecked finalized-graph construction with a justified acyclic crate boundary. Add a validated prepared-scope constructor accounting for every expected coordinate and participant; restrict unchecked construction and deserialization of completeness.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/tests/`
- `crates/eqm_engine/src/expand.rs`
- `crates/eqm_engine/src/invariants.rs`
- `crates/eqm_domain/src/graph.rs`
- `crates/eqm_manifest/src/canonical.rs`

### 5. Unit tests or integration tests required
Test absent and explicit fragment risks in both lower/higher destination journeys, invalid expanded graphs, public construction bypasses, omitted/duplicate coordinates, and complete failing/unknown scope. Exercise the real manifest-to-expansion-to-preparation path; use compile-fail/API tests where appropriate.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** ADR-0012 records owner approval; do not re-request the settled direction. S006 closes exact engineering representations and any newly evidenced conflicting consumer obligation. Live product/authority values remain governed by Q01-Q15; DEV-001 defines the S049 amendment.

### 7. Expected result
Complete evaluation cannot be forged by a boolean or partial map.

### 8. Commit message
```text
feat: validate complete evaluation scope
```

**Approved review regression contributions:** RA01, RA05, RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s050"></a>
## S050 — Reject accidental empty release scope

**Predecessor:** S049. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59, R60. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059, AC060.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Reject accidental empty release scope.

### 2. Purpose
Avoid vacuous release success and meaningless comparison claims.

### 3. Exact scope of code changes
Add explicit admission semantics for empty required release scope and singleton comparisons according to protected policy; retain local inspection usefulness.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_engine/src/equivalence.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Empty unauthorized release is non-authorizing; singleton is labeled no meaningful comparison; approved explicit exception is not invented by defaults.

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
Empty work cannot become a release pass.

### 8. Commit message
```text
fix: reject vacuous release assurance
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s051"></a>
## S051 — Compose policy constraints per facet

**Predecessor:** S050. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R23, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC023, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Compose policy constraints per facet.

### 2. Purpose
Prevent unrelated rule strengths from leaking across facets.

### 3. Exact scope of code changes
Derive facet-specific matching rule sets and strength using the approved normative contract. Compare protected obligation-universe inclusion plus per-coordinate constraints when checking policy strengthening; increasing a minimum-level selection floor is not automatically stronger. Preserve all matched-rule reasons.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Test different facets with different trust/count/age constraints and compare derived universes before/after policy changes, including Recommended-to-Required selection-floor changes.

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
Per-facet constraints match authored policy intent.

### 8. Commit message
```text
fix: compose obligation strength per facet
```

**Approved review regression contributions:** RA04, RA07.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s052"></a>
## S052 — Derive provider and end-to-end dependencies

**Predecessor:** S051. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Derive provider and end-to-end dependencies.

### 2. Purpose
Build complete non-target scopes for later aggregation.

### 3. Exact scope of code changes
Emit exact provider/scenario obligations with participant/dependency linkage; do not flatten cross-target coverage into separate target passes.
DEV-030 freezes explicit requirement-to-scenario preparation inputs, exact provider
dependencies and the separate scope-binding projection. Include domain, application,
manifest, CLI, usage docs and their owning test lanes in this allocation.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Missing provider/end-to-end evidence remains independently required; repeated same-target actors are preserved.

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
All approved scope kinds are present in the required universe.

### 8. Commit message
```text
feat: derive exact cross-runtime obligations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s053"></a>
## S053 — Bind expected selection manifests

**Predecessor:** S052. **Contracts:** R08, R11, R12, R13, R14, R15, R16, R17, R18, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Bind expected selection manifests.

### 2. Purpose
Make missing test cases observable independently of result files.

### 3. Exact scope of code changes
Represent and derive required logical case/observation selection manifests from approved binding/specification policy; reject ambiguous count units.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence.rs`
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Report with all reported cases passing but an expected case missing remains incomplete; selectors/count units stay exact.

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
The evaluator knows what should have run before trusting what was reported.

### 8. Commit message
```text
feat: bind expected evidence selections
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s054"></a>
## S054 — Generate complete public obligation identities

**Predecessor:** S053. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Generate complete public obligation identities.

### 2. Purpose
Fix IDs that omit profiles and release/candidate context.

### 3. Exact scope of code changes
Replace ad hoc string formatting with a versioned complete coordinate identity/projection; update query DTO references coherently.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Profile/revision/subject/context changes cannot collide; deterministic identical coordinates retain identity.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Public references uniquely identify the actual obligation.

### 8. Commit message
```text
fix: include full context in obligation identities
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s055"></a>
## S055 — Prepare structural observations independently

**Predecessor:** S054. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R24, R28, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC024, AC028, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Prepare structural observations independently.

### 2. Purpose
Avoid echoing authored artifact roles as observations.

### 3. Exact scope of code changes
Define a read-only observation boundary that supplies validated file kind/containment and independently supported roles; absent role authority stays unknown.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/structure.rs`
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/check.rs`

### 5. Unit tests or integration tests required
A declared service role on an unverified file does not prove role match; independent adapter roles are checked.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Structural evaluation consumes observations rather than self-confirming declarations.

### 8. Commit message
```text
fix: separate authored roles from structural observations
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s056"></a>
## S056 — Require trusted complete inventory for absence

**Predecessor:** S055. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R24, R28, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC024, AC028, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Require trusted complete inventory for absence.

### 2. Purpose
Preserve three-valued observation semantics.

### 3. Exact scope of code changes
Consolidate inventory completeness/currentness/trust prerequisites in prepared supporting facts; retain provenance and absence/unknown distinction.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/exposure.rs`
- `crates/eqm_runner/src/inventory.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Partial/missing/stale/untrusted inventories cannot prove absence; complete verified inventory can.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Presence/absence conclusions have explicit supporting authority.

### 8. Commit message
```text
fix: preserve inventory absence prerequisites
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s057"></a>
## S057 — Add normative derivation property tests

**Predecessor:** S056. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R23, R50, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC023, AC050, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Add normative derivation property tests.

### 2. Purpose
Test policy rules independently of orchestration shortcuts.

### 3. Exact scope of code changes
Add table/property coverage for ordering, unknown scope, empty scope, monotonic policy composition and protected candidate deletions using reference expected universes.

### 4. Files/modules likely involved
- `crates/eqm_engine/tests/obligations.rs`
- `crates/eqm_engine/tests/properties.rs`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Input permutations preserve identities; stronger requirements never shrink required work; omissions never increase assurance.

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
Derivation invariants are executable and independently reviewable.

### 8. Commit message
```text
test: lock complete obligation derivation semantics
```

**Approved review regression contributions:** RA04, RA05, RA07.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s058"></a>
## S058 — Expose complete obligation query DTOs

**Predecessor:** S057. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R21, R44, R45, R52, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC021, AC044, AC045, AC052, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Expose complete obligation query DTOs.

### 2. Purpose
Make required/unresolved/non-applicable scope visible to consumers.

### 3. Exact scope of code changes
Map the prepared universe into versioned protocol records with reasons and context without loading/evaluating evidence or inventing final statuses.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/query.rs`
- `crates/eqm_cli/src/commands/obligations.rs`
- `schemas/`

### 5. Unit tests or integration tests required
Query contains every required coordinate and explicit pending/unknown facts; filtering is presentation-only.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Obligation inspection truthfully explains scope before acquisition.

### 8. Commit message
```text
feat: expose complete obligation scope views
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
