# Chapter 11: Read-only MCP and coherent agent access

Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P11, containing 7 separate ordered commits, S158–S164; it is not one combined commit. All implementation steps are pending.

**Definition of green:** MCP tools/resources retain one authority/snapshot with explicit refresh, permissions, pagination and full-envelope bounds.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q07, Q14. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s158"></a>
## S158 — Share application snapshots with MCP sessions

**Predecessor:** S157. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Share application snapshots with MCP sessions.

### 2. Purpose
Remove CLI coupling without reloading/reinterpreting evidence.

### 3. Exact scope of code changes
Adapt MCP session ownership to shared application snapshot/query services while preserving transport lifecycle and no dependency cycle.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/session.rs`
- `crates/eqm_cli/src/commands/mcp.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
Session tests borrow/reference one immutable snapshot and do not implicitly reacquire data or create state.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP can expose the same prepared decision as the CLI.

### 8. Commit message
```text
refactor: connect mcp to shared evaluation snapshots
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s159"></a>
## S159 — Populate MCP context and findings resources

**Predecessor:** S158. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Populate MCP context and findings resources.

### 2. Purpose
Replace empty placeholder resource records.

### 3. Exact scope of code changes
Render obligations/evidence/rejections/findings/applied waivers from shared query models with actual completeness/trust/truncation metadata.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/resources.rs`
- `crates/eqm_application/src/query.rs`
- `crates/eqm_mcp tests`

### 5. Unit tests or integration tests required
Resource results agree with check/context for passing and negative fixtures; empty findings do not imply evaluated completeness unless established.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Agent resources contain real evaluated context.

### 8. Commit message
```text
fix: populate evaluated mcp resources
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s160"></a>
## S160 — Route MCP tools through shared read operations

**Predecessor:** S159. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Route MCP tools through shared read operations.

### 2. Purpose
Avoid transport-specific status calculation.

### 3. Exact scope of code changes
Map supported inspection/query tools to shared application use cases with exact request validation; retain read-only defaults.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/tools.rs`
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp tests`

### 5. Unit tests or integration tests required
Tool/resource/CLI outputs reference the same decision and unauthorized execution-shaped requests reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP tools remain thin coherent readers.

### 8. Commit message
```text
refactor: route mcp tools through shared queries
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s161"></a>
## S161 — Enforce MCP permission and untrusted-data boundaries

**Predecessor:** S160. **Contracts:** R04, R05, R44, R45, R46, R47, R50. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md).

### 1. Step title
Enforce MCP permission and untrusted-data boundaries.

### 2. Purpose
Prevent text or read authority from becoming execution permission.

### 3. Exact scope of code changes
Add explicit operation admission checks and safe labeling for product descriptions/logs/attachments; ensure no signer/waiver/acquisition capability is exposed by default.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp/src/tools.rs`
- `crates/eqm_mcp tests`

### 5. Unit tests or integration tests required
Inject untrusted textual instructions and privilege-escalation requests; no execution/signing/mutation occurs.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Read-only agent access cannot grant itself stronger capabilities.

### 8. Commit message
```text
fix: enforce mcp operation authority boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s162"></a>
## S162 — Bound MCP requests pagination and lifecycle

**Predecessor:** S161. **Contracts:** R04, R05, R42, R44, R45, R46, R47, R50. **Acceptance:** AC004, AC005, AC042, AC044, AC045, AC046, AC047, AC050.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Bound MCP requests pagination and lifecycle.

### 2. Purpose
Keep agent access resource-safe and generation-correct.

### 3. Exact scope of code changes
Apply input/output/queue budgets, snapshot-bound pagination, cancellation and explicit unsupported transport behavior; do not add a default HTTP daemon.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp/src/resources.rs`
- `crates/eqm_cli/src/commands/mcp.rs`

### 5. Unit tests or integration tests required
Oversized/deep requests, stale cursors, slow clients and shutdown cannot leak work or falsely complete a truncated view.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP lifecycle and views are bounded and truthful.

### 8. Commit message
```text
fix: bound mcp request and snapshot lifecycles
```

**Approved review regression contributions:** RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s163"></a>
## S163 — Map structured errors into MCP responses

**Predecessor:** S162. **Contracts:** R04, R05, R21, R44, R45, R46, R47, R50, R52. **Acceptance:** AC004, AC005, AC021, AC044, AC045, AC046, AC047, AC050, AC052.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Map structured errors into MCP responses.

### 2. Purpose
Preserve causes without transport divergence or secret leakage.

### 3. Exact scope of code changes
Translate shared typed errors into supported protocol error/result shapes and stable diagnostics; retain request identity and context safely.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp/src/tools.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Invalid authority/query/resource and unavailable snapshot fixtures return valid protocol errors matching CLI diagnostic semantics.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP errors are actionable without implementing another evaluator.

### 8. Commit message
```text
fix: preserve structured mcp diagnostics
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s164"></a>
## S164 — Verify and document secure MCP integration

**Predecessor:** S163. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Verify and document secure MCP integration.

### 2. Purpose
Establish parity and least-privilege evidence.

### 3. Exact scope of code changes
Add stdio/restricted-transport smoke/security tests and update documentation with actual supported methods, permissions, limits and snapshot semantics.

### 4. Files/modules likely involved
- `crates/eqm_mcp tests`
- `crates/eqm_cli/src/commands/mcp.rs`
- `docs/mcp.md`
- `docs/agent-context.md`

### 5. Unit tests or integration tests required
Cross-interface fixture suite and unauthorized-operation tests pass; no unverified public server support claim is added.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask test security
```

### 7. Expected result
MCP is usable as a coherent read-only agent interface.

### 8. Commit message
```text
test: verify secure mcp snapshot access
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
