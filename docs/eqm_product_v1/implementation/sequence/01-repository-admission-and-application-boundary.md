# Chapter 01: Repository admission and application boundary

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P01, containing 12 separate ordered commits, S001–S012; it is not one combined commit. All implementation steps are pending.

**Definition of green:** Approved package admitted; baseline reconciled; S006 representation vectors frozen; application extraction and typed ports remain acyclic.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q03, Q05, Q14. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s001"></a>
## S001 — Admit the approved spec subtree

**Predecessor:** PREFLIGHT. **Contracts:** R53, R56, R61. **Acceptance:** AC053, AC056, AC061.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/BASELINE.md](../../repo/BASELINE.md).

### 1. Step title
Admit the approved spec subtree.

### 2. Purpose
Keep the standard repository gate usable without discarding security checks.

### 3. Exact scope of code changes
Narrowly allow docs/eqm_product_v1 content and replace generic-word handoff rejection with scoped meaningful private-data checks; keep unrelated directory/generated-state restrictions.

### 4. Files/modules likely involved
- `tools/xtask/src/main.rs`
- `tools/xtask tests`
- `docs/README.md`

### 5. Unit tests or integration tests required
Add fixtures admitting the versioned spec subtree and rejecting arbitrary top-level/private/generated material; run public-repository checks.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Approved contract docs can be imported while existing meaningful safety rules still fail on forbidden inputs.

### 8. Commit message
```text
build: admit versioned product specifications
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s002"></a>
## S002 — Import durable contracts and the ordered plan

**Predecessor:** S001. **Contracts:** R56, R57, R61, R62. **Acceptance:** AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [repo/BASELINE.md](../../repo/BASELINE.md).

### 1. Step title
Import durable contracts and the ordered plan.

### 2. Purpose
Establish versionable approved intent before implementation.

### 3. Exact scope of code changes
Add the handoff under docs/eqm_product_v1, including required contract files, requirements/acceptance catalogs and initial pending progress; preserve existing public docs and mark code/tests unimplemented.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/`
- `docs/README.md`

### 5. Unit tests or integration tests required
Run the package validator at its imported path and verify internal links/IDs; assert no new generated .eqm state or secrets.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Agents can reconstruct intent and resume from an explicit pending step without chat history.

### 8. Commit message
```text
docs: record the eqm product v1 contract
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s003"></a>
## S003 — Merge concise agent guidance

**Predecessor:** S002. **Contracts:** R50, R53, R56, R62. **Acceptance:** AC050, AC053, AC056, AC062.

**Read:** [PROMPT.md](../../PROMPT.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md).

### 1. Step title
Merge concise agent guidance.

### 2. Purpose
Keep repository-local operating rules aligned with the approved plan.

### 3. Exact scope of code changes
Merge repo/AGENTS.md guidance into existing root instructions; point to durable specs and one-step reporting; clarify approved changes versus gated unsafe/version decisions.

### 4. Files/modules likely involved
- `AGENTS.md`
- `docs/agent-context.md`
- `docs/eqm_product_v1/implementation/`

### 5. Unit tests or integration tests required
Validate documentation references and compare instructions for conflicting execution/trust permissions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Root guidance is short, preserves existing protections, and routes an agent to the next exact step.

### 8. Commit message
```text
docs: anchor agent work to the product specification
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s004"></a>
## S004 — Record baseline findings and decision gates

**Predecessor:** S003. **Contracts:** R14, R23, R50, R53, R57, R62. **Acceptance:** AC014, AC023, AC050, AC053, AC057, AC062.

**Read:** [implementation/STATUS.md](../../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Record baseline findings and decision gates.

### 2. Purpose
Prevent static observations being mistaken for tested current defects.

### 3. Exact scope of code changes
Reconcile F01-F20 and RA01-RA21 against the actual HEAD, incorporate the recorded full-source review and baseline checks, and preserve unrun implementation acceptance. Revalidate the approved decisions in ADR-0012 and record only remaining external facts; apply DEV-001 to the planning views before its owning implementation step.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/repo/REVIEW_FINDINGS.md`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`

### 5. Unit tests or integration tests required
Verify every F/RA item has source evidence, requirements, acceptance, owning steps and a precise regression; distinguish static review from runtime reproduction and prior baseline checks from new implementation evidence.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
The implementation baseline and unresolved blockers are evidenced without invented passing tests.

### 8. Commit message
```text
docs: record implementation baseline and open gates
```

**Approved review regression contributions:** RA05.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s005"></a>
## S005 — Add deterministic conformance fixture builders

**Predecessor:** S004. **Contracts:** R14, R26, R50, R53. **Acceptance:** AC014, AC026, AC050, AC053.

**Read:** [implementation/VERIFICATION.md](../../implementation/VERIFICATION.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Add deterministic conformance fixture builders.

### 2. Purpose
Supply reusable regression inputs without production trust shortcuts.

### 3. Exact scope of code changes
Extend test support with injected time, exact subject/selection builders and synthetic evidence helpers; keep every authority stand-in test-only.

### 4. Files/modules likely involved
- `crates/eqm_test_support/src/`
- `crates/eqm_engine/tests/support/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Builders reject malformed coordinates and produce repeatable IDs; production dependency/feature inspection shows no synthetic trusted factory.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Later steps can add focused regressions without duplicating fragile fixture construction.

### 8. Commit message
```text
test: add deterministic conformance fixtures
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s006"></a>
## S006 — Freeze semantic and version representation decisions

**Predecessor:** S005. **Contracts:** R14, R16, R21, R23, R50, R52, R58. **Acceptance:** AC014, AC016, AC021, AC023, AC050, AC052, AC058.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Freeze semantic and version representation decisions.

### 2. Purpose
Resolve protocol ambiguity before new wire meanings ship.

### 3. Exact scope of code changes
Materialize ADR-0012 approval: explicit new wire namespace, required conjunction/authorized alternatives, per-kind logical count units, retry/conflict/campaign rules and inherited fragment risk. Complete engineering decisions for the exact namespace/URI transition, numeric ranges, timestamp precision, null/absence, sets/sequences, digest domains and minimal public API names. Audit consumers/releases; stop only for a newly evidenced obligation conflicting with the approved direction or a still-missing required contract.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/decisions/`
- `docs/eqm_product_v1/specs/API_CONTRACTS.md`
- `docs/eqm_product_v1/specs/EVALUATION_SEMANTICS.md`

### 5. Unit tests or integration tests required
Add conformance vectors for approved outcome/count/risk rules and all representation boundaries. Cross-check schema generators, URI parsing and consumer expectations; retain originals and reject unsupported versions explicitly. Record fixture policies as synthetic; do not invent production threshold/skew values.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** ADR-0012 records owner approval; do not re-request the settled direction. S006 closes exact engineering representations and any newly evidenced conflicting consumer obligation. Live product/authority values remain governed by Q01-Q15; DEV-001 defines the S049 amendment.

### 7. Expected result
New wire and policy work has explicit normative representations; unresolved external compatibility blocks exposure.

### 8. Commit message
```text
docs: freeze evaluation and protocol representations
```

**Approved review regression contributions:** RA05.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s007"></a>
## S007 — Create the shared application crate

**Predecessor:** S006. **Contracts:** R04, R06, R50. **Acceptance:** AC004, AC006, AC050.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md).

### 1. Step title
Create the shared application crate.

### 2. Purpose
Introduce the approved effectful boundary without moving semantics.

### 3. Exact scope of code changes
Add eqm_application to workspace with exact local dependencies and inherited lint/license metadata; expose only a documented minimal module boundary and retain existing CLI behavior.

### 4. Files/modules likely involved
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_application/Cargo.toml`
- `crates/eqm_application/src/lib.rs`

### 5. Unit tests or integration tests required
Compile the new crate, verify no CLI/MCP dependency cycle, and test its empty/minimal public boundary without success placeholders.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Workspace remains green and the application layer exists without changing release decisions.

### 8. Commit message
```text
refactor: introduce the shared application layer
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s008"></a>
## S008 — Move prepared workspace ownership into the application layer

**Predecessor:** S007. **Contracts:** R04, R06, R22. **Acceptance:** AC004, AC006, AC022.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Move prepared workspace ownership into the application layer.

### 2. Purpose
Share immutable preparation without CLI coupling.

### 3. Exact scope of code changes
Move or extract PreparedSession/SessionRequest loading ownership into eqm_application; leave MCP adaptation in the transport/CLI layer; update call sites without changing evaluation semantics.

### 4. Files/modules likely involved
- `crates/eqm_application/src/session.rs`
- `crates/eqm_cli/src/session.rs`
- `crates/eqm_cli/src/commands/`
- `crates/eqm_mcp/src/session.rs`

### 5. Unit tests or integration tests required
Existing session fixtures retain graph digest/source maps; test immutable borrowing and no reload/cycle across MCP adaptation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_mcp
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI behavior is preserved while other adapters can use the same prepared workspace.

### 8. Commit message
```text
refactor: share prepared workspace sessions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s009"></a>
## S009 — Preserve manifest error causes

**Predecessor:** S008. **Contracts:** R45, R50. **Acceptance:** AC045, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md).

### 1. Step title
Preserve manifest error causes.

### 2. Purpose
Restore actionable parse/load diagnostics.

### 3. Exact scope of code changes
Replace stage-only load/conversion error erasure with typed contextual variants retaining source path, field and causal error where safe; do not alter schema acceptance.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/loader.rs`
- `crates/eqm_manifest/src/validation.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Invalid TOML, conversion, discovery and lock fixtures retain appropriate path/field information without secret leakage.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Manifest failures remain diagnosable at higher boundaries.

### 8. Commit message
```text
fix: preserve manifest loading diagnostics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s010"></a>
## S010 — Preserve session preparation errors

**Predecessor:** S009. **Contracts:** R45, R50. **Acceptance:** AC045, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md).

### 1. Step title
Preserve session preparation errors.

### 2. Purpose
Keep new shared preparation failures typed and contextual.

### 3. Exact scope of code changes
Map resolve/expand/canonicalization failures into application errors with causes; retain stable command behavior until public error-envelope step.

### 4. Files/modules likely involved
- `crates/eqm_application/src/session.rs`
- `crates/eqm_application/src/error.rs`
- `crates/eqm_cli/src/session.rs`

### 5. Unit tests or integration tests required
Malformed graph/fragment pin and unavailable file fixtures retain causes across application/CLI conversion.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Preparation no longer discards useful errors while production commands remain fail-closed.

### 8. Commit message
```text
fix: retain prepared session error context
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s011"></a>
## S011 — Inject evaluation time consistently

**Predecessor:** S010. **Contracts:** R05, R22, R38. **Acceptance:** AC005, AC022, AC038.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Inject evaluation time consistently.

### 2. Purpose
Remove hidden wall-clock reads from shared semantic preparation.

### 3. Exact scope of code changes
Add explicit host-captured evaluation time to application requests and pass it through existing query/evaluation call sites; keep acquisition timing and monotonic process timers separate.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Repeated requests with the same time are deterministic; boundary-time fixtures prove no internal SystemTime drift in evaluation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evaluation has explicit time without changing process timeout mechanisms.

### 8. Commit message
```text
refactor: inject evaluation clock context
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s012"></a>
## S012 — Define narrow application capability interfaces

**Predecessor:** S011. **Contracts:** R04, R05, R06, R64. **Acceptance:** AC004, AC005, AC006, AC064.

**Read:** [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/TRUST_AND_RELEASE.md](../../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Define narrow application capability interfaces.

### 2. Purpose
Separate read/evaluate operations from privileged acquisition.

### 3. Exact scope of code changes
Introduce documented ports for authority/evidence access, verification and execution with explicit requests; no default global runtime or permissive fallback implementations.

### 4. Files/modules likely involved
- `crates/eqm_application/src/lib.rs`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/src/acquisition.rs`

### 5. Unit tests or integration tests required
Spy implementations show read/evaluate cannot launch runners or fetch network; missing capabilities return explicit errors.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Application use cases can be tested independently without implicit host privileges.

### 8. Commit message
```text
refactor: separate application capability boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
