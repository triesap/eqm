# Optional native integration — gated ordered extension

**Initial status: not activated.** The approved default is developer tooling/CI and native evidence adapters, not an embedded evaluator. This extension starts only when Q11 supplies a real local-conformance feature and Q01/Q02 identify exact consumer repositories/platforms. Core approval alone is not activation.

Follow the same `type: imperative subject` commit convention and no-skip/no-merge/no-reorder/no-broadening discipline as the mandatory sequence. These are separate independent green commits, not an unattended batch. N002 selects one coherent boundary strategy and records exact native commands; subsequent steps implement only that strategy. Do not build both a sidecar and FFI to avoid making a decision. Features/platforms not included in N001 remain outside scope with an explicit recorded disposition, not a silent unsupported support claim.

Likely paths below are deliberately conditional on that selected boundary and real consumer repositories. Discover actual package/task names before changes and update the step's bounded allocation; do not invent modules or product interfaces beyond the approved contract. Preserve all core Rust gates while running the activated consumer/native lanes. No publication/deployment is implied.

<a id="n001"></a>
## N001 — Activate a concrete local-evaluation feature

**Predecessor:** OPTIONAL_ACTIVATION. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Activate a concrete local-evaluation feature.

### 2. Purpose
Prevent optional architecture from becoming automatic scope.

### 3. Exact scope of code changes
Record explicit owner approval for the feature, exact supported platform matrix, consumer repository and failure/latency needs; otherwise do not start this sequence.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/decisions/`
- `authorized consumer repository instructions`

### 5. Unit tests or integration tests required
Review dependency and product needs against R03/R49; core Rust gates remain green and no bridge is added yet.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
Optional work has explicit product authority and scope.

### 8. Commit message
```text
docs: activate the local conformance integration
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n002"></a>
## N002 — Choose one verified boundary strategy

**Predecessor:** N001. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Choose one verified boundary strategy.

### 2. Purpose
Avoid maintaining multiple speculative bridges.

### 3. Exact scope of code changes
Inspect current consumer targets and select one coherent sidecar or in-process strategy; record packaging, ownership, security and exact native commands before later steps; do not assume JNA supports Kotlin/Native.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/decisions/`
- `authorized consumer build metadata`
- `Cargo.toml`

### 5. Unit tests or integration tests required
Build a minimal disposable compatibility probe on each required target and record limitations; select only a proven supported path.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
One boundary strategy and test matrix are fixed for the remaining sequence.

### 8. Commit message
```text
docs: select the native conformance boundary
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n003"></a>
## N003 — Define the narrow consumer contract

**Predecessor:** N002. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Define the narrow consumer contract.

### 2. Purpose
Keep business logic and media out of the evaluator boundary.

### 3. Exact scope of code changes
Specify batch evaluation/query, structured errors, version negotiation and lifecycle ownership for the selected strategy; no new business accounts/tasks/media APIs.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `proposed optional transport crate`
- `consumer commonMain interface`

### 5. Unit tests or integration tests required
Contract fixtures reject unsupported versions and unbounded inputs; supported scalar/time mappings are exact.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
The consumer interface is small and spec-anchored.

### 8. Commit message
```text
feat: define the local conformance consumer contract
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n004"></a>
## N004 — Isolate the optional transport package

**Predecessor:** N003. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Isolate the optional transport package.

### 2. Purpose
Preserve core purity and dependency cost.

### 3. Exact scope of code changes
Add only the selected boundary package/features with exact dependencies and documented lint/safety scope; no blanket unsafe relaxation or default daemon inclusion.

### 4. Files/modules likely involved
- `Cargo.toml`
- `Cargo.lock`
- `proposed optional transport Cargo.toml`
- `consumer build metadata`

### 5. Unit tests or integration tests required
Core builds without optional transport; declared feature sets compile and production trust remains unavailable to read-only consumer.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
Optional packaging does not contaminate the core or default product distribution.

### 8. Commit message
```text
build: isolate optional conformance transport
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n005"></a>
## N005 — Implement safe boundary ownership

**Predecessor:** N004. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Implement safe boundary ownership.

### 2. Purpose
Prevent resource leaks and invalid object reuse.

### 3. Exact scope of code changes
Implement lifetime-managed buffers/handles for in-process integration or owned session/process resources for a sidecar, according to the selected strategy; document safety invariants.

### 4. Files/modules likely involved
- `selected transport source`
- `consumer platform adapter`
- `transport tests`

### 5. Unit tests or integration tests required
Invalid/double-close/use-after-close and oversized message fixtures reject safely; no raw Rust collection layout crosses an ABI.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
The chosen boundary has explicit tested resource ownership.

### 8. Commit message
```text
feat: enforce local conformance resource ownership
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n006"></a>
## N006 — Implement batch evaluation and query calls

**Predecessor:** N005. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Implement batch evaluation and query calls.

### 2. Purpose
Avoid per-cell or per-obligation foreign calls.

### 3. Exact scope of code changes
Expose the shared application operation and snapshot-bound queries through the selected transport, preserving read-only defaults and full structured results.

### 4. Files/modules likely involved
- `selected transport source`
- `crates/eqm_application/src/query.rs`
- `consumer platform adapter`

### 5. Unit tests or integration tests required
Batched consumer results match direct Rust evaluation; filters do not alter scope or decision.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
Native consumers use the same evaluator without fine-grained cross-language chatter.

### 8. Commit message
```text
feat: expose batched local conformance operations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n007"></a>
## N007 — Implement cancellation and shutdown

**Predecessor:** N006. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Implement cancellation and shutdown.

### 2. Purpose
Bound consumer lifecycle races.

### 3. Exact scope of code changes
Propagate consumer cancellation to owned evaluation/acquisition only where explicitly permitted, close resources deterministically, and preserve committed result discoverability; no detached background tasks.

### 4. Files/modules likely involved
- `selected transport source`
- `consumer platform adapter`
- `transport tests`

### 5. Unit tests or integration tests required
Cancellation before/during/after completion and shutdown under load produce deterministic documented outcomes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
The optional integration remains safe during app closure and interrupted work.

### 8. Commit message
```text
fix: bound local conformance cancellation and shutdown
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n008"></a>
## N008 — Translate errors and compatibility explicitly

**Predecessor:** N007. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Translate errors and compatibility explicitly.

### 2. Purpose
Avoid interpreting foreign failures as success.

### 3. Exact scope of code changes
Map application errors/statuses and supported schema/ABI/protocol negotiation into the consumer; reject unknown versions and preserve safe source/reason detail.

### 4. Files/modules likely involved
- `selected transport source`
- `consumer commonMain/platform adapters`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Unknown status/version, malformed response, invalid numeric mapping and internal failure cannot produce a passing UI state.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
Consumer behavior is exact under failures and version skew.

### 8. Commit message
```text
fix: preserve local conformance error semantics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n009"></a>
## N009 — Secure the selected transport boundary

**Predecessor:** N008. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Secure the selected transport boundary.

### 2. Purpose
Match isolation to actual strategy instead of a slogan.

### 3. Exact scope of code changes
For sidecar use restricted authenticated IPC and explicit process identity; for in-process validate privilege/lifetime boundary and document lack of crash isolation; no default unauthenticated localhost endpoint.

### 4. Files/modules likely involved
- `selected transport source`
- `consumer packaging/security configuration`

### 5. Unit tests or integration tests required
Unauthorized peer/message or forged handle tests reject; aborting-panic behavior is documented and tested only in disposable processes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
The chosen boundary makes accurate security and fault-containment claims.

### 8. Commit message
```text
fix: secure the local conformance transport
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n010"></a>
## N010 — Integrate the declared desktop consumer

**Predecessor:** N009. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Integrate the declared desktop consumer.

### 2. Purpose
Wire the actual supported desktop runtime only.

### 3. Exact scope of code changes
Add the selected platform implementation behind a small common interface; connect report display with scope/freshness/reasons and bounded pagination, not product business operations.

### 4. Files/modules likely involved
- `authorized desktop commonMain/platform sources`
- `selected transport packaging`

### 5. Unit tests or integration tests required
Run discovered desktop build/test/smoke lanes on every supported OS/architecture; no per-row process spawn.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
The desktop feature works on the declared matrix with coherent snapshot views.

### 8. Commit message
```text
feat: integrate desktop local conformance access
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n011"></a>
## N011 — Integrate explicitly approved mobile consumers

**Predecessor:** N010. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Integrate explicitly approved mobile consumers.

### 2. Purpose
Avoid assuming desktop bridge portability.

### 3. Exact scope of code changes
Only for mobile targets included in activation, add platform implementations of the same consumer interface using the chosen proven strategy; keep capture/storage/sync platform-owned.

### 4. Files/modules likely involved
- `authorized mobile commonMain/platform sources`
- `selected transport packaging`

### 5. Unit tests or integration tests required
Run actual supported Android/iOS device/simulator lanes and ownership/cancellation tests; no unsupported JNA-on-Native use.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
Approved mobile local evaluation works or remains an explicit activation blocker.

### 8. Commit message
```text
feat: integrate approved mobile conformance access
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n012"></a>
## N012 — Package and verify native artifacts

**Predecessor:** N011. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Package and verify native artifacts.

### 2. Purpose
Make bridge support a shipped tested capability.

### 3. Exact scope of code changes
Generate/package selected native libraries or sidecar artifacts with exact version/provenance and platform conventions; avoid committing generated build outputs.

### 4. Files/modules likely involved
- `selected transport build tooling`
- `authorized consumer Gradle/Xcode configuration`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
Install/run packaged artifacts on all declared targets; verify architecture, symbols/version, licenses, checksums and incompatible load errors.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
Packaging support is demonstrated rather than inferred from Rust compilation.

### 8. Commit message
```text
build: verify local conformance runtime packaging
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n013"></a>
## N013 — Test cross-runtime semantic and numeric parity

**Predecessor:** N012. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Test cross-runtime semantic and numeric parity.

### 2. Purpose
Guard language-boundary behavior.

### 3. Exact scope of code changes
Run the shared canonical/decision vectors through direct Rust and each activated consumer, including numeric/time/Unicode, errors, pagination and concurrent calls.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `selected transport tests`
- `authorized consumer tests`

### 5. Unit tests or integration tests required
Equivalent inputs produce the same decision identity/reasons; unsupported ranges reject rather than rounding.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
Native results preserve canonical Rust semantics.

### 8. Commit message
```text
test: verify local conformance cross-runtime parity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n014"></a>
## N014 — Measure optional integration overhead

**Predecessor:** N013. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Measure optional integration overhead.

### 2. Purpose
Validate the feature's actual runtime cost.

### 3. Exact scope of code changes
Benchmark startup, memory, batch latency, cancellation and report paging on declared devices/hosts; record owner-approved budgets rather than new arbitrary promises.

### 4. Files/modules likely involved
- `selected transport benchmarks`
- `authorized consumer tests`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
Measured results include complete workloads and compare direct/transport behavior without weakening safety.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.
- Run the owning benchmark/fixture tests and, when the populated benchmark step is reached, the actual cargo xtask benchmark lane on declared hardware; record measurements and host gaps, not thresholds alone.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
Optional integration performance is evidence-backed.

### 8. Commit message
```text
perf: measure local conformance integration overhead
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n015"></a>
## N015 — Complete optional security and lifecycle review

**Predecessor:** N014. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Complete optional security and lifecycle review.

### 2. Purpose
Prove optional readiness independently of core completion.

### 3. Exact scope of code changes
Review any unsafe/native boundary, dependencies, privileges and failure modes; run supported feature/host/consumer gates and record all remaining gaps.

### 4. Files/modules likely involved
- `selected transport source`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `authorized consumer tests`

### 5. Unit tests or integration tests required
No unresolved ownership/security/platform blocker is hidden; core and optional test matrices pass.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
Optional feature readiness is explicitly established or blocked.

### 8. Commit message
```text
test: complete local conformance integration acceptance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.


<a id="n016"></a>
## N016 — Document optional release and support disposition

**Predecessor:** N015. **Contracts:** R03, R49, R50, R51, R52, R53. **Acceptance:** AC003, AC049, AC050, AC051, AC052, AC053.

**Read:** [implementation/VERIFICATION.md](VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Document optional release and support disposition.

### 2. Purpose
Close the extension with reproducible operational context.

### 3. Exact scope of code changes
Update consumer/API support docs, compatibility rules, packaging/runbooks and final status with actual evidence and owner release permission; do not deploy automatically.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/specs/RUNTIME_INTEGRATION.md`
- `docs/eqm_product_v1/runbooks/`
- `authorized consumer documentation`

### 5. Unit tests or integration tests required
Documentation examples and package checks pass; exact support/activation state and rollback are recorded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
```

**Discovery-dependent verification:**
- After N002, use the exact selected transport package/feature and activated consumer build/test/package commands; discover them from Cargo metadata and the approved consumer Gradle/Xcode/native setup before code changes. Do not run both sidecar and FFI strategies.

**Decision/readiness gate:** Requires explicit Q11 activation, Q01/Q02 consumer matrix and Q13 boundary review. No optional implementation is implied by core approval.

### 7. Expected result
A maintainer can operate or disable the optional feature without chat history.

### 8. Commit message
```text
docs: finalize local conformance integration support
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
