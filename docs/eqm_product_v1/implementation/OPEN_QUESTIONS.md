# Decisions, remaining inputs and activation gates

Revision 2 records owner approval of all review recommendations in [ADR-0012](../decisions/ADR-0012-approved-review-closure.md). Approved direction is distinguished from missing concrete inputs below. Do not ask the owner to approve the same recommendation again. Engineering chooses implementation details from evidence; actual product/security authority and access cannot be invented.

Keep exactly one mandatory step active and preserve order. Generic fail-closed implementation may proceed when unavailable values are explicit; missing inputs block the first affected step when safe implementation or required verification depends on them. Optional embedding remains deliberately inactive. Counts of decision records in the package manifest describe the catalog, not a claim that every direction is unresolved.

## Retained planning allocations

The new eqm_application crate initially owns application authority/storage/trust/acquisition/evaluation/query boundaries. Admit docs/eqm_product_v1 only after S001; use synthetic non-production fixtures, existing standard xtask gates and exact pins. Private stdio/report consumption is the default; no automatic service. Missing authority is unavailable/non-authorizing.

## Q01 — Real product repositories and workflows

**Kind:** Owner/access decision. **Status:** discovery_required.

**Approved direction:** Discover actual authorized Rust CLI, KMP desktop/mobile and provider repositories/tasks; no synthetic substitute for live acceptance.

**Remaining inputs:** Actual repository paths, commits, owners, tasks, selectors and outputs.

**Resolution:** Discover actual authorized Rust CLI, KMP desktop/mobile and provider repositories/tasks; no synthetic substitute for live acceptance. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Generic EQM adapters/fixtures can be built; live product acceptance remains blocked.

## Q02 — Supported hosts and native matrix

**Kind:** Owner decision. **Status:** direction_approved_evidence_pending.

**Approved direction:** Initial EQM execution/storage targets are macOS arm64 and Linux x86_64; Windows is outside the initial support promise. Product targets are separate.

**Remaining inputs:** Native host evidence plus actual product OS/architecture/SDK/device/JVM-native matrix.

**Resolution:** Initial EQM execution/storage targets are macOS arm64 and Linux x86_64; Windows is outside the initial support promise. Product targets are separate. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Do not claim unsupported execution/storage/native readiness; portable contracts remain implementable.

## Q03 — Wire-version and compatibility transition

**Kind:** Contract decision. **Status:** direction_approved_engineering_pending.

**Approved direction:** Use an explicit new namespace for incompatible wire changes; preserve originals, reject unsupported versions, no implicit legacy readers.

**Remaining inputs:** S006 exact namespace/URI/representation choice and any newly discovered conflicting external consumer obligation.

**Resolution:** Use an explicit new namespace for incompatible wire changes; preserve originals, reject unsupported versions, no implicit legacy readers. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Do not silently reuse v1 identities or add legacy readers/aliases; new internal types may be additive.

## Q04 — Production verification and signer authority

**Kind:** Owner/security decision. **Status:** configuration_required.

**Approved direction:** Implement real reviewed verification with synthetic test-only material; real authority remains independent and unconfigured until supplied.

**Remaining inputs:** Authorized identities, issuers, builders, roots, signer providers and operator configuration.

**Resolution:** Implement real reviewed verification with synthetic test-only material; real authority remains independent and unconfigured until supplied. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Production trust/enforcement remains unconfigured and non-authorizing; never synthesize SignedCi.

## Q05 — Policy thresholds and campaign selection

**Kind:** Product/security decision. **Status:** semantics_approved_profiles_pending.

**Approved direction:** Use ADR-0012 conjunction, logical-count, retry/conflict, campaign, monotonicity, fragment-risk and exact-time rules; explicit configurable validated policy.

**Remaining inputs:** Concrete product minimum counts, ages/skew, authorized alternative groups/campaigns and permitted deviations.

**Resolution:** Use ADR-0012 conjunction, logical-count, retry/conflict, campaign, monotonicity, fragment-risk and exact-time rules; explicit configurable validated policy. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** No arbitrary thresholds or timestamp-based pass selection; unresolved required policies fail closed.

## Q06 — Supported mixed-version combinations

**Kind:** Product owner decision. **Status:** discovery_required.

**Approved direction:** Use only explicitly approved participant/build cohorts; no invented compatibility window.

**Remaining inputs:** Actual mixed-version support matrix and scenario expectations.

**Resolution:** Use only explicitly approved participant/build cohorts; no invented compatibility window. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Model general subject manifests; do not promise a particular compatibility window.

## Q07 — Retention, privacy and performance values

**Kind:** Operator/product decision. **Status:** configuration_required.

**Approved direction:** Implement explicit bounded policy and benchmark representative inputs; use synthetic public fixtures and protect sensitive artifacts.

**Remaining inputs:** Retention/privacy/access/encryption rules, operational resource ceilings, representative volumes and SLO values.

**Resolution:** Implement explicit bounded policy and benchmark representative inputs; use synthetic public fixtures and protect sensitive artifacts. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Implement validated limits/classes and safe bounded failure; do not invent legal retention obligations or numerical SLO claims.

## Q08 — Execution isolation and secrets

**Kind:** Security/operator decision. **Status:** configuration_required.

**Approved direction:** Resolve exact plans against independent host capabilities; reject unsupported guarantees and default missing privileges to unavailable.

**Remaining inputs:** Actual worker isolation/source immutability, network/secrets policies and provider configuration.

**Resolution:** Resolve exact plans against independent host capabilities; reject unsupported guarantees and default missing privileges to unavailable. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Unsupported guarantees reject execution; no false sandbox promise or blanket local privilege.

## Q09 — CI/deployment host and enforcement

**Kind:** Owner/operator decision. **Status:** discovery_required.

**Approved direction:** Integrate selected existing CI/promotion environment in shadow mode; EQM supplies records/validation without deployment orchestration.

**Remaining inputs:** Existing CI host/configuration, protected branch expectations, destinations and consumption integration.

**Resolution:** Integrate selected existing CI/promotion environment in shadow mode; EQM supplies records/validation without deployment orchestration. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Implement host-neutral contracts and shadow fixtures; never publish/deploy or enforce live gates automatically.

## Q10 — Storage implementation allocation

**Kind:** Engineering decision. **Status:** engineering_pending.

**Approved direction:** Engineering selects and pins an audited wrapper and actual linked fixed SQLite runtime with explicit host durability.

**Remaining inputs:** Dependency/features/runtime/license/advisory evidence and chosen durability settings at S077.

**Resolution:** Engineering selects and pins an audited wrapper and actual linked fixed SQLite runtime with explicit host durability. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Default no implicit DB/network install; no assumptions that the system SQLite or network filesystem is safe.

## Q11 — Optional embedded evaluator activation

**Kind:** Product decision. **Status:** deferred_by_design.

**Approved direction:** Native embedding stays not_activated; default developer/CI deployment is approved.

**Remaining inputs:** Only a future explicitly approved in-app local-evaluation feature and exact matrix can activate N001-N016.

**Resolution:** Native embedding stays not_activated; default developer/CI deployment is approved. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Native bridge and persistent sidecar stay deferred by design, not unfinished mandatory work.

## Q12 — Business revision/conflict/normalization rules

**Kind:** Product decision. **Status:** discovery_required.

**Approved direction:** Derive executable scenarios from actual product behavior; EQM does not invent business media/edit/sync/auth rules.

**Remaining inputs:** Approved business revision/conflict/normalization/authorization rules and expected scenarios.

**Resolution:** Derive executable scenarios from actual product behavior; EQM does not invent business media/edit/sync/auth rules. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Use labeled synthetic scenarios; real business conformance cannot be claimed until rules and adapters exist.

## Q13 — Native/OS unsafe exception

**Kind:** Engineering/security decision. **Status:** conditional_engineering_gate.

**Approved direction:** Prefer safe maintained APIs; pure core remains unsafe-free; isolate and document any unavoidable boundary exception before code.

**Remaining inputs:** Evidence that a specific required host mechanism cannot use safe APIs, if such an exception is needed.

**Resolution:** Prefer safe maintained APIs; pure core remains unsafe-free; isolate and document any unavoidable boundary exception before code. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Never relax domain/engine unsafe rules or assume bridge safety from compilation.

## Q14 — Public command and exact API shape

**Kind:** Engineering/compatibility decision. **Status:** engineering_pending.

**Approved direction:** Engineering freezes minimal spec-backed symbols/fields/flags using existing command grammar where appropriate; no aliases or duplicate truth paths.

**Remaining inputs:** Exact versioned API/DTO/CLI representations and consumer tests at S006 and owning interface steps.

**Resolution:** Engineering freezes minimal spec-backed symbols/fields/flags using existing command grammar where appropriate; no aliases or duplicate truth paths. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** No invented compatibility aliases, duplicated command families or claimed existing symbols.

## Q15 — Operational ownership and activation evidence

**Kind:** Owner decision. **Status:** ownership_required.

**Approved direction:** Keep implementation/local/platform/operational status separate; require independent shadow review and explicit final activation disposition.

**Remaining inputs:** Named semantics/trust/runner/storage/platform/release owners, actual access and sign-off evidence.

**Resolution:** Keep implementation/local/platform/operational status separate; require independent shadow review and explicit final activation disposition. Record concrete evidence at the owning step; do not re-request settled direction.

**Safe behavior:** Code completion does not authorize real promotion or replace independent review.
