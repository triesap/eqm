# Chapter 03: Shared sources target roles and exact subjects

**Current authority:** DEV-015 in the sole COMMIT_SEQUENCE; 40 allocated numbered steps accepted. S039-R2 → S041-R1 → P03-R4 → S043-R1; required independent P03 gate remains before S044. Historical frontiers below are subordinate.

**Historical authority:** DEV-014 in the sole COMMIT_SEQUENCE; 38 allocated numbered steps accepted. P03-R2 → P03-R3 → S039-R1 → S040–S043; mandatory independent review before S044. Retained historical frontiers below do not replace this frontier.

**Historical authority:** DEV-012 in the sole COMMIT_SEQUENCE; 32 allocated numbered steps accepted.
Six integration repairs then every S039–S043; mandatory independent review before S044.
Historical frontier paragraphs below are subordinate to the current amendment.

**Historical authority:** DEV-011, S025-R2 → S026-R1 → S027–S043;
24 numbered steps accepted, independent integration review before S044.
Historical frontier paragraphs below are superseded by the current sole plan.


**Historical authority:** DEV-010 in the sole COMMIT_SEQUENCE; 23 numbered steps
accepted for allocated scope. S022-R2 → S025-R1 → S026–S043; independent
integration review before S044. Historical amendment text below is subordinate
to that current frontier. All original criteria remain.
Read the [global execution rules](../COMMIT_SEQUENCE.md) and [verification contract](../VERIFICATION.md) first. This is cycle EQM-P03, containing 16 separate ordered commits, S028–S043; it is not one combined commit. Current implementation and acceptance states are recorded in progress.json; earlier frontier statements are historical.

**Definition of green:** Shared-source dependencies, participant roles and exact multi-subject records validate without forced target symmetry.

**Verification lane:** Each step: listed owning regressions, cargo xtask check and git diff --check; cycle exit additionally requires its declared platform/product/security lanes.

**Admission inputs:** Q02, Q06. Approved directions are in [ADR-0012](../../decisions/ADR-0012-approved-review-closure.md); unavailable real inputs remain explicit.

<a id="s028"></a>
## S028 — Model product roles separately from targets

**Predecessor:** S027. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model product roles separately from targets.

### 2. Purpose
Support asymmetric CLI/control-tower/edge behavior.

### 3. Exact scope of code changes
Add validated role metadata to the logical target model without hardcoding the user's whole product; keep existing target IDs stable. DEV-007: bind the newly modeled exact role ID in the target configuration projection in this same step; preserve v1 forms and prove same-target role mutation changes its v2 identity. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/id.rs`
- `crates/eqm_domain/tests/`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Distinct target outputs may share a role; role changes do not accidentally rename targets or imply participation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Product role is explicit and independent of platform/build identity.

### 8. Commit message
```text
feat: separate product roles from shipped targets
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s029"></a>
## S029 — Model source components

**Predecessor:** S028. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model source components.

### 2. Purpose
Represent shared implementation input without duplicate target roots.

### 3. Exact scope of code changes
Add source-component identities and confined roots with explicit target-to-component relations; retain domain-only validation. DEV-007: bind newly modeled target-to-source-component ID references in the target projection as canonical ID sets in this same step; exact source content remains a separate subject axis. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/graph.rs`
- `crates/eqm_domain/tests/`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Shared components attach to multiple targets; duplicate authority and invalid path identities reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Shared KMP/Rust source ownership is representable without filesystem exceptions.

### 8. Commit message
```text
feat: model shared source components
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s030"></a>
## S030 — Resolve source dependency edges

**Predecessor:** S029. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Resolve source dependency edges.

### 2. Purpose
Make transitive shared-source ownership explicit.

### 3. Exact scope of code changes
Resolve component references, target dependencies and cycles in graph preparation; reject dangling/cyclic unsupported dependencies deterministically. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/resolve.rs`
- `crates/eqm_engine/src/invariants.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Shared dependency DAG resolves; missing nodes/cycles and ambiguous ownership produce stable diagnostics.

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
Source dependency graph is validated before affected analysis.

### 8. Commit message
```text
feat: resolve shared source dependencies
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s031"></a>
## S031 — Extend authored target and source schemas

**Predecessor:** S030. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Extend authored target and source schemas.

### 2. Purpose
Expose the approved model through generated manifests.

### 3. Exact scope of code changes
Add target role/source-component DTOs using the version decision; regenerate schemas and examples atomically without implicit old aliases. DEV-007: round-trip all new target role/source references into the target identity through authored schemas and the real loader; no new normative target field may be dropped from its projection. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/dto/`
- `crates/eqm_manifest/src/conversion/`
- `schemas/`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Strict decode/encode/schema vectors cover shared components and unknown fields; current explicit version behavior is tested.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Authored shared-source metadata has a versioned strict representation.

### 8. Commit message
```text
feat: author shared source target metadata
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s032"></a>
## S032 — Load overlapping shared-source targets safely

**Predecessor:** S031. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Load overlapping shared-source targets safely.

### 2. Purpose
Remove duplicate-root rejection only where explicit shared ownership permits it.

### 3. Exact scope of code changes
Update loader root handling to distinguish output identity from shared source roots, preserve portable collision checks, and reject ambiguous undeclared overlaps. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF47: S032-R1 uses the same checked transitive component closure for overlap and binding ownership, retaining confinement and unrelated-root rejection.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/loader.rs`
- `crates/eqm_manifest/tests/`

- `crates/eqm_engine/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Two approved outputs share a component; undeclared root collision and path escape reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Natural KMP layouts load without weakening repository containment.

### 8. Commit message
```text
fix: load explicitly shared target source roots
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s033"></a>
## S033 — Validate artifacts against declared source ownership

**Predecessor:** S032. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Validate artifacts against declared source ownership.

### 2. Purpose
Stop requiring every valid shared artifact under one output root.

### 3. Exact scope of code changes
Update binding conversion and structural expectations to use declared source-component ownership; keep wrong-target and undeclared paths blocking. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF46: S033-R1 routes resolved shared ownership into production CLI structural checking and executes declared-shared versus undeclared/escaped controls.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/conversion/`
- `crates/eqm_engine/src/structure.rs`
- `crates/eqm_engine/tests/`

- `crates/eqm_cli/src/commands/check.rs`
- `crates/eqm_application/tests/`
- `crates/eqm_cli/tests/`

### 5. Unit tests or integration tests required
Shared commonMain artifact validates for dependent outputs; unrelated component and escaped paths fail.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_engine -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Shared code is valid only through explicit dependency authority.

### 8. Commit message
```text
feat: validate artifacts through source ownership
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s034"></a>
## S034 — Model capability participation rules

**Predecessor:** S033. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model capability participation rules.

### 2. Purpose
Express intentional role differences independently of waivers.

### 3. Exact scope of code changes
Add validated requirement/target/profile participation selectors aligned with existing applicability vocabulary; reject ambiguous selectors and duplicate authority. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/policy.rs`
- `crates/eqm_domain/tests/`

### 5. Unit tests or integration tests required
Mobile-only capture and desktop-only review fixtures validate; unspecified required context stays representably unknown.

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
Product asymmetry can be authored without permanent waiver abuse.

### 8. Commit message
```text
feat: model target capability participation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s035"></a>
## S035 — Author and resolve participation metadata

**Predecessor:** S034. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Author and resolve participation metadata.

### 2. Purpose
Make participation part of protected graph semantics.

### 3. Exact scope of code changes
Add strict DTO/conversion/resolution and semantic hashing for approved participation rules; regenerate schemas together. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF50: S035-R1 rejects duplicate requirement/target coordinates at real graph preparation and public participation evaluation, never first-ID wins.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/dto/`
- `crates/eqm_manifest/src/conversion/`
- `crates/eqm_engine/src/resolve.rs`
- `schemas/`

- `crates/eqm_domain/src/graph.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Round-trip participation rules; undeclared targets/profiles and weakening graph mutations are diagnosable.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine -p eqm_manifest -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Participation is versioned semantic authority, not a UI-only filter.

### 8. Commit message
```text
feat: resolve authored capability participation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s036"></a>
## S036 — Evaluate applicability per participant coordinate

**Predecessor:** S035. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Evaluate applicability per participant coordinate.

### 2. Purpose
Preserve target-specific applicability and unknown context.

### 3. Exact scope of code changes
Extend engine context so participation/applicability can be evaluated for exact requirement/participant/profile combinations; keep true/false/unknown distinct. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF50: retain coordinate evaluation contribution but reject ambiguous supplied authorities through S035-R1 before accepting this step.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/applicability.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Same requirement is true for edge, false for explicit exclusion, unknown for missing profile; no early global drop.

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
Applicability no longer incorrectly applies one pre-fanout conclusion to all targets.

### 8. Commit message
```text
fix: evaluate applicability for exact participants
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s037"></a>
## S037 — Model role-bearing scenario occurrences

**Predecessor:** S036. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model role-bearing scenario occurrences.

### 2. Purpose
Represent multiple actors of one shipped target.

### 3. Exact scope of code changes
Add scenario participant occurrence IDs/roles and exact dependency mappings for end-to-end scope; do not collapse repeated devices into a set. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF48: S037-R1 includes exact v2 scenario reader/schema/discovery/conversion/index/reference validation and complete graph identity, not domain-only modeling.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_domain/tests/`

- `crates/eqm_manifest/src/`
- `crates/eqm_engine/src/`
- `crates/eqm_application/tests/`
- `schemas/v2/manifest/scenario.schema.json`

### 5. Unit tests or integration tests required
Two mobile occurrences remain distinct; duplicate occurrence IDs and missing roles reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine -p eqm_manifest -p eqm_protocol -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Multi-device journeys have unambiguous participant identity.

### 8. Commit message
```text
feat: model scenario participant occurrences
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s038"></a>
## S038 — Model exact multi-subject manifests

**Predecessor:** S037. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model exact multi-subject manifests.

### 2. Purpose
Bind each actor to its own source build artifact and configuration.

### 3. Exact scope of code changes
Add validated subject manifests for target/provider/scenario participants with explicit required identity fields and typed absence for allowed development cases. DEV-007: exact subjects retain separate target-configuration, source snapshot/component content and build artifact identities; a target digest alone cannot satisfy missing source/build inputs. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF49: S038-R1 aligns symbolic id/domain/wire, rejects logical-key duplicates, validates exact occurrence/context assignments with explicit development absence, and projects a separately typed subject identity.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_protocol/tests/`

- `crates/eqm_domain/src/subject_manifest.rs`
- `crates/eqm_application/tests/`
- `crates/eqm_manifest/src/canonical/`

### 5. Unit tests or integration tests required
Different valid mobile/desktop builds coexist; substituted artifact/build/config mismatches reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol -p eqm_manifest -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Cross-target context does not require identical build digests.

### 8. Commit message
```text
feat: bind exact multi-participant subjects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s039"></a>
## S039 — Separate comparison context identity

**Predecessor:** S038. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Separate comparison context identity.

### 2. Purpose
Prevent exact build identity from blocking valid comparability.

### 3. Exact scope of code changes
Define shared contract/scenario/logical-profile/comparison-rule identity separately from subject manifest/evaluation identity and project it canonically. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-014 supplies retained clarification: complete RF51/RF52/RF53 corrections before S040, graph-backed provider/cohort matching and genuine asymmetric parity; preserve all original tests, dependencies and criteria. DEV-015 governs checked comparison coverage, exact retained cohort/configuration authority at preparation, full definition/schema invariants and real asymmetric consumer parity; all original criteria and the independent S043 gate remain.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/digest.rs`
- `crates/eqm_engine/src/equivalence.rs`
- `crates/eqm_manifest/src/canonical/`

### 5. Unit tests or integration tests required
Approved differing builds share comparison context; different scenario/profile/contract does not compare.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Comparison eligibility and exact subject verification remain independent.

### 8. Commit message
```text
feat: distinguish comparison context from exact subjects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s040"></a>
## S040 — Bind provider obligations explicitly

**Predecessor:** S039. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Bind provider obligations explicitly.

### 2. Purpose
Avoid unscoped provider participation in unrelated targets.

### 3. Exact scope of code changes
Represent provider subject/configuration and target/scenario dependency links; adjust domain/graph checks while deferring final aggregation to evaluator steps. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-014 supplies retained clarification: complete RF51/RF52/RF53 corrections before S040, graph-backed provider/cohort matching and genuine asymmetric parity; preserve all original tests, dependencies and criteria.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_engine/src/resolve.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Missing required provider blocks its dependents; an unrelated provider is not automatically attached to every target.

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
Shared services have exact subject and dependency semantics.

### 8. Commit message
```text
feat: bind shared provider dependencies
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s041"></a>
## S041 — Author compatibility cohorts

**Predecessor:** S040. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Author compatibility cohorts.

### 2. Purpose
Support approved mixed-version combinations without inventing a support window.

### 3. Exact scope of code changes
Add strict data representation and validation for declared required participant/build cohorts and scenario coverage; leave operational cohort values explicit. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-014 supplies retained clarification: complete RF51/RF52/RF53 corrections before S040, graph-backed provider/cohort matching and genuine asymmetric parity; preserve all original tests, dependencies and criteria. DEV-015 governs checked comparison coverage, exact retained cohort/configuration authority at preparation, full definition/schema invariants and real asymmetric consumer parity; all original criteria and the independent S043 gate remain.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/policy.rs`
- `crates/eqm_manifest/src/dto/`
- `crates/eqm_manifest/src/conversion/`
- `schemas/`

### 5. Unit tests or integration tests required
Approved N/N+1 fixture cohort works; undeclared combination and absent participant reject.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Independently released targets can be tested against explicit compatibility policy.

### 8. Commit message
```text
feat: define explicit compatibility cohorts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s042"></a>
## S042 — Add the asymmetric multi-runtime graph fixture

**Predecessor:** S041. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Add the asymmetric multi-runtime graph fixture.

### 2. Purpose
Exercise the target/source/participant model as a whole.

### 3. Exact scope of code changes
Add synthetic Rust CLI, desktop, mobile and provider authority with shared source sets, repeated actors and role-specific requirements; no invented real product build tasks. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-014 supplies retained clarification: complete RF51/RF52/RF53 corrections before S040, graph-backed provider/cohort matching and genuine asymmetric parity; preserve all original tests, dependencies and criteria. DEV-015 governs checked comparison coverage, exact retained cohort/configuration authority at preparation, full definition/schema invariants and real asymmetric consumer parity; all original criteria and the independent S043 gate remain.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `crates/eqm_test_support/src/`
- `crates/eqm_manifest/tests/`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Load/resolve/canonicalize fixture and assert stable roles/dependencies/cohorts; intentional exclusions are not waivers.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support -p eqm_manifest -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Later evaluator tests have a representative but honestly synthetic product graph.

### 8. Commit message
```text
test: add asymmetric multi-runtime graph coverage
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s043"></a>
## S043 — Verify target model schema and identity parity

**Predecessor:** S042. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Verify target model schema and identity parity.

### 2. Purpose
Close graph-model changes before obligation routing.

### 3. Exact scope of code changes
Update public usage examples and schema parity assertions for new target/subject/participation records, preserving explicit compatibility decisions. DEV-007: include regressions proving role/source-reference changes invalidate target configuration while separately bound source/build content changes invalidate the appropriate subject; missing axes never become success. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF45-RF50: cycle exit verifies production v2 graph identity/version routing, checked transitive structural ownership, nonambiguous participation and exact scenario/subject/provider/cohort ingress with public parity. DEV-014 supplies retained clarification: complete RF51/RF52/RF53 corrections before S040, graph-backed provider/cohort matching and genuine asymmetric parity; preserve all original tests, dependencies and criteria. DEV-015 governs checked comparison coverage, exact retained cohort/configuration authority at preparation, full definition/schema invariants and real asymmetric consumer parity; all original criteria and the independent S043 gate remain.

### 4. Files/modules likely involved
- `docs/concepts.md`
- `docs/manifests.md`
- `examples/`
- `crates/eqm_protocol/tests/`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
Full schema regeneration comparison, example validation and component-digest regressions pass.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
Authored/protocol/domain target models agree under the approved version contract.

### 8. Commit message
```text
test: verify shared target model parity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
