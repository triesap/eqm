# Chapter 07: Authorized acquisition and host supervision

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P07, containing 19 separate ordered commits, S097–S115; it is not one combined commit. All implementation steps are pending.

**Definition of green:** Authorized concrete plans and all ingress/process paths obey actual host capabilities and bounded lifecycle guarantees.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q02, Q07, Q08, Q13. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s097"></a>
## S097 — Define host execution capabilities

**Predecessor:** S096. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Define host execution capabilities.

### 2. Purpose
Stop treating authored guarantees as operator authority.

### 3. Exact scope of code changes
Add explicit host/backend capability configuration and validation, including paths, permissions, network/secrets and limits; declarations cannot self-authorize capabilities.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/resolution.rs`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/tests/`

### 5. Unit tests or integration tests required
Unsupported guarantees reject before spawn; local declarations cannot claim isolation not supplied by the host.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Resolution is constrained by independent host authority.

### 8. Commit message
```text
fix: resolve runners against actual host capabilities
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s098"></a>
## S098 — Build complete non-executing acquisition plans

**Predecessor:** S097. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Build complete non-executing acquisition plans.

### 2. Purpose
Make dry-run a concrete safety artifact.

### 3. Exact scope of code changes
Resolve executable identity, argv, cwd, selector/expected cases, environment references, limits and outputs into a deterministic plan without launching programs.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/substitution.rs`
- `crates/eqm_cli/src/commands/verify.rs`

### 5. Unit tests or integration tests required
Spawn/network spies stay unused; every required plan field is present; unresolved executables and selectors fail explicitly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Dry-run reveals exact work rather than only specification IDs.

### 8. Commit message
```text
feat: prepare concrete acquisition plans
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s099"></a>
## S099 — Bind explicit approval to exact plans

**Predecessor:** S098. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Bind explicit approval to exact plans.

### 2. Purpose
Prevent execution after meaningful plan substitution.

### 3. Exact scope of code changes
Introduce a checked approval boundary for plan identity/authority; verify approved immutable inputs immediately before execution and reject changed executable/argv/context.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/resolution.rs`
- `crates/eqm_runner/tests/`

### 5. Unit tests or integration tests required
Changing any approved plan axis invalidates approval; read-only callers cannot create execution permission.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Authorized execution is tied to the plan the operator reviewed.

### 8. Commit message
```text
feat: bind execution approval to exact plans
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s100"></a>
## S100 — Centralize bounded confined file access

**Predecessor:** S099. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Centralize bounded confined file access.

### 2. Purpose
Remove validate-then-unbounded-read shortcuts.

### 3. Exact scope of code changes
Add a narrowly scoped safe handle-based filesystem utility for source/result/import reads using reviewed APIs; preserve byte/type/ancestor containment checks and document host differences.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_runner/src/`
- `crates/eqm_manifest/src/`
- `Cargo.toml`
- `Cargo.lock`

### 5. Unit tests or integration tests required
Parent-symlink, replacement, oversized read and invalid-type fixtures reject; any dependency/unsafe exception is reviewed before use.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Callers have one tested confinement and bounded-read boundary.

### 8. Commit message
```text
fix: centralize bounded confined file access
```

**Approved review regression contributions:** RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s101"></a>
## S101 — Validate result files after execution

**Predecessor:** S100. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate result files after execution.

### 2. Purpose
Ensure pre-created temp paths cannot bypass limits.

### 3. Exact scope of code changes
Replace verify fs::read with bounded handle acquisition and post-execution validation; reject replaced/symlink/oversized/malformed result files before normalization.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_runner/src/normalized.rs`
- `crates/eqm_application/src/acquisition.rs`

### 5. Unit tests or integration tests required
Worker replaces output path, emits huge file or inconsistent selector; all fail safely without unbounded allocation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Only safe bounded exact runner results enter evidence normalization.

### 8. Commit message
```text
fix: validate runner output files before loading
```

**Approved review regression contributions:** RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s102"></a>
## S102 — Prepare immutable worker input and output roots

**Predecessor:** S101. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Prepare immutable worker input and output roots.

### 2. Purpose
Separate candidate execution from authoritative state.

### 3. Exact scope of code changes
Materialize or verify approved immutable sources and dedicated staging outputs; expose no evaluator/authority/index/signing write access; record dirty development mode separately.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/src/container.rs`

### 5. Unit tests or integration tests required
Worker attempts writes to protected fixture roots fail or unsupported configuration rejects before launch; input mutation invalidates identity.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Trusted acquisition does not execute against mutable self-authorizing inputs.

### 8. Commit message
```text
feat: isolate worker inputs and output staging
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s103"></a>
## S103 — Resolve explicit host environments and secrets

**Predecessor:** S102. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Resolve explicit host environments and secrets.

### 2. Purpose
Remove hardcoded PATH and empty-secret wiring.

### 3. Exact scope of code changes
Provide operator-approved host environment and secret references through the application boundary; retain env_clear and avoid logging values; no implicit inheritance or retrieval.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_cli/src/commands/verify.rs`

### 5. Unit tests or integration tests required
Environment marker absence, exact allowlist, missing secret failure and host-specific PATH fixtures pass without real credentials.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Native acquisition gets explicit portable inputs without ambient privilege.

### 8. Commit message
```text
feat: resolve authorized runner environment inputs
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s104"></a>
## S104 — Signal output overflow immediately

**Predecessor:** S103. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Signal output overflow immediately.

### 2. Purpose
Stop draining unlimited output until process exit.

### 3. Exact scope of code changes
Use signaled bounded stdout/stderr readers and supervised bounded stdin delivery; activate cancellation/deadline ownership before any potentially blocking request write. Terminate promptly on overflow; every input-write error retains child/task cleanup ownership.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/src/adapter.rs`
- `crates/eqm_runner/tests/security.rs`

### 5. Unit tests or integration tests required
Exercise input beyond pipe capacity, a worker refusing stdin or writing before reading, early stdin closure, output floods and cancellation during request delivery. Assert prompt bounded termination and cleanup, not only retained buffer size.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Output caps constrain execution promptly, not only final classification.

### 8. Commit message
```text
fix: terminate acquisition on output overflow
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s105"></a>
## S105 — Supervise pipes and descendants through completion

**Predecessor:** S104. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Supervise pipes and descendants through completion.

### 2. Purpose
Close the parent-exit hang.

### 3. Exact scope of code changes
Keep deadline/cancellation active through input delivery, descendant ownership, output drainage and cleanup. Implement bounded joins and cleanup after immediate-child exit; handle broken pipes and every supervisor error path without orphaning owned work.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/tests/security.rs`

### 5. Unit tests or integration tests required
Test a child exiting while descendants retain pipes, cancellation/deadline during input/drain, broken-pipe cleanup and bounded shutdown on each supported native host.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Immediate-child exit cannot move blocking reader joins outside supervision.

### 8. Commit message
```text
fix: bound descendant and pipe shutdown
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s106"></a>
## S106 — Wire cooperative and host cancellation

**Predecessor:** S105. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Wire cooperative and host cancellation.

### 2. Purpose
Make cancellation reachable from real callers.

### 3. Exact scope of code changes
Propagate cancellation from application/CLI lifecycle into owned execution and bounded cleanup; specify completion/publication races and retain committed results discoverably.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_cli/src/main.rs`
- `crates/eqm_runner/src/execution.rs`

### 5. Unit tests or integration tests required
Cancel before spawn, during execution/drain and around publication; no detached work or contradictory committed state.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Cancellation is an explicit lifecycle outcome rather than an unused token.

### 8. Commit message
```text
feat: propagate acquisition cancellation safely
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s107"></a>
## S107 — Harden Unix process ownership

**Predecessor:** S106. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Harden Unix process ownership.

### 2. Purpose
Make Unix tree cleanup and errors explicit.

### 3. Exact scope of code changes
Use approved safe OS abstractions for owned process groups/session handling; remove fragile assumptions about external kill invocation where needed and document containment limits.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/tests/security.rs`
- `docs/eqm_product_v1/specs/EXECUTION_SECURITY.md`

### 5. Unit tests or integration tests required
Unix-native child/grandchild/error cleanup tests pass; escape attempts are contained by approved host isolation or reject unsupported guarantees.

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

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Unix supervision claims match tested host capability rather than best-effort direct-child kill.

### 8. Commit message
```text
fix: harden unix worker lifecycle ownership
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s108"></a>
## S108 — Implement supported Windows process ownership

**Predecessor:** S107. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Implement supported Windows process ownership.

### 2. Purpose
Avoid claiming tree termination from Child.kill alone.

### 3. Exact scope of code changes
After Q02/Q13, implement safe reviewed Windows Job Object ownership and kill-on-close/cancellation behavior, with explicit configuration for unsupported nested host cases.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/`
- `crates/eqm_runner/tests/security.rs`
- `Cargo.toml`
- `Cargo.lock`

### 5. Unit tests or integration tests required
Windows-native descendant/pipe/cancel/exit tests and target compilation pass; do not substitute Linux-only tests for this gate.

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

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Windows support is implemented and evidenced, or recorded unsupported by an approved support-matrix decision.

### 8. Commit message
```text
feat: supervise supported windows worker trees
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s109"></a>
## S109 — Dispatch only supported acquisition backends

**Predecessor:** S108. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Dispatch only supported acquisition backends.

### 2. Purpose
Connect host-authorized execution paths coherently.

### 3. Exact scope of code changes
Add application dispatch for the repository's supported local/container/locked program mechanisms; unsupported resolution/execution capabilities return explicit errors without fallback.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/container.rs`
- `crates/eqm_runner/src/resolution.rs`

### 5. Unit tests or integration tests required
Backend selection, immutable locked identities, unavailable runtime and guarantee mismatch fixtures reject or execute exactly as approved.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Backend declarations correspond to real implemented capabilities.

### 8. Commit message
```text
feat: dispatch authorized acquisition backends
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s110"></a>
## S110 — Treat executable discovery as acquisition

**Predecessor:** S109. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Treat executable discovery as acquisition.

### 2. Purpose
Remove any read-only illusion around build-tool discovery.

### 3. Exact scope of code changes
Route discovery adapters that execute programs through approved planning/supervision; keep static bounded inspection separate and read-only.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/discover.rs`
- `crates/eqm_runner/src/adapter.rs`
- `crates/eqm_application/src/acquisition.rs`

### 5. Unit tests or integration tests required
A read-only discover request cannot run Gradle/scripts; explicitly approved discovery uses the same bounded worker path.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Discovery has the same privilege and result-validation rules as other execution.

### 8. Commit message
```text
fix: require authority for executable discovery
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s111"></a>
## S111 — Persist terminal acquisition receipts

**Predecessor:** S110. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Persist terminal acquisition receipts.

### 2. Purpose
Keep failures and canceled work visible in expected selections.

### 3. Exact scope of code changes
Record successful and failed/timeout/cancel/overflow terminal receipts with plan/campaign/case context where safely possible; keep malformed results as failures not absent successes.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_runner/src/normalized.rs`

### 5. Unit tests or integration tests required
Restart after failed acquisition retains expected missing/failed work; zero exit with malformed result never becomes evidence success.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Acquisition history explains terminal non-success across process restarts.

### 8. Commit message
```text
feat: retain terminal acquisition outcomes
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s112"></a>
## S112 — Bound scheduling and device ownership

**Predecessor:** S111. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Bound scheduling and device ownership.

### 2. Purpose
Establish safe concurrency before throughput optimization.

### 3. Exact scope of code changes
Add bounded work queues, concurrency admission and exclusive device/resource leases using explicit plan limits; deterministic result collection and owned shutdown.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Concurrent plans sharing a device do not overlap illegally; queue saturation/cancellation is explicit and no work is silently dropped.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Scheduling respects resource authority and cancellation.

### 8. Commit message
```text
feat: bound acquisition scheduling and device use
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s113"></a>
## S113 — Harden retained-output redaction

**Predecessor:** S112. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Harden retained-output redaction.

### 2. Purpose
Keep sensitive values out of plans reports and stored logs.

### 3. Exact scope of code changes
Centralize bounded secret redaction and safe error formatting, including truncation/multibyte/chunk-boundary cases and declared overlapping secrets; document limits against hostile exfiltration.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_application/src/error.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Test secrets spanning read chunks, truncated outputs, overlapping values and malformed UTF-8; no real secrets in fixtures.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Retained outputs obey the declared redaction contract without claiming sandbox security.

### 8. Commit message
```text
fix: harden bounded acquisition output redaction
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s114"></a>
## S114 — Enforce application-wide resource budgets

**Predecessor:** S113. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R44, R46, R47, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC044, AC046, AC047, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enforce application-wide resource budgets.

### 2. Purpose
Prevent graph/protocol/request workloads bypassing runner limits.

### 3. Exact scope of code changes
Apply configured byte/depth/expansion/profile/candidate/attachment/queue budgets at acquisition/preparation boundaries; no silent truncation of required scope.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/`
- `crates/eqm_runner/src/`

### 5. Unit tests or integration tests required
Oversized/deep/explosive fixtures terminate with explicit limit diagnostics and cannot authorize release.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_protocol -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Bounded behavior covers the whole input pipeline.

### 8. Commit message
```text
feat: enforce conformance resource budgets
```

**Approved review regression contributions:** RA12, RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s115"></a>
## S115 — Complete host adversarial acquisition tests

**Predecessor:** S114. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Complete host adversarial acquisition tests.

### 2. Purpose
Evidence the promises made by each backend.

### 3. Exact scope of code changes
Consolidate environment/confinement/flood/descendant/cancel/source-mutation/protected-write tests for each declared host, updating capability docs honestly.

### 4. Files/modules likely involved
- `crates/eqm_runner/tests/security.rs`
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
Run native host/security lanes and record unavailable/unsupported guarantees; no new blanket capability claims.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask test security
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Acquisition security is backed by explicit host execution evidence.

### 8. Commit message
```text
test: verify adversarial worker boundaries
```

**Approved review regression contributions:** RA11, RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
