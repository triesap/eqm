# Logical data model

These are semantic record contracts, not finalized Rust declarations, table DDL, JSON schemas, field spellings, or new CLI flags. Finalize exact representations in the referenced implementation steps after inspecting the existing types. Do not infer a new product backend schema from this model.

## Existing authority to preserve

Capabilities group product value. Journeys order surfaces/transitions. Surfaces own requirements and reusable fragments expand through exact pins. Bindings associate shared units with native artifacts and evidence selectors. Policy/profile selections determine obligations. Retain validated identities, risk/lifecycle invariants, owners, explicit extensions and deterministic maps.

## Target and participant records (R07–R09, R13)

| Logical record | Required meaning and validation |
|---|---|
| Product role | An authored role such as edge/control tower/CLI/provider; it is not a platform enum that forces all product behavior. |
| Shipped target | Stable identity of an independently released output with platform/architecture/configuration metadata when relevant. |
| Source component | Repository-confined source ownership plus explicit shared-source dependencies; shared KMP source sets may contribute to many outputs. |
| Capability participation | Approved requirement/target/profile applicability, with explicit false versus unresolved status. |
| Scenario participant | A role-bearing occurrence within a scenario; two devices of the same target remain distinct actors. |
| Subject manifest | Exact participant-to-build/artifact/source/configuration assignments, including providers/deployments where required. |
| Comparison context | Shared contract/scenario/logical-profile/comparison-rule identity, separate from the distinct participant subjects. |

Provider obligations bind exact provider identity/configuration, not every target indiscriminately without an explicit dependency mapping. Shared-provider and end-to-end evidence cannot be satisfied by duplicate per-target passes. Product-supported version combinations are approved data, not a hardcoded N/N-1 promise.

## Evaluation records (R11, R14, R19)

An obligation coordinate includes selected policy and revision, complete selected profile revisions/values, unit, fully qualified requirement, scope/participant subject, facet, and appropriate immutable candidate/context identity. Strength includes the constraints applicable to that coordinate: required level, accepted evidence/trust requirements, maximum age and count semantics. Public obligation IDs must bind the complete coordinate, not omit profile/release context.

A required-universe record enumerates obligations and explicit not-applicable/unresolved coordinates, expected selection manifests, and protected scope. A prepared evaluation contains that universe, exact inputs and verification conclusions. An evaluation decision contains statuses, all reasons, completeness, scope, evidence acceptance/rejection, exposure comparisons, applied waivers, conformance/comparison summaries and input/evaluator/time identities.

Display labels and file locations remain provenance rather than semantic identity, unless a path is itself a declared behavioral/build input. Use projections deliberately; do not treat every path as globally irrelevant.

## Evidence lifecycle records (R16–R20)

Raw object: original exact bytes with raw content digest, retained according to class. Normalized evidence: schema-specific semantic assertion with its own digest. Execution identity: authorized plan/campaign/run/logical selection/attempt identity. A verification receipt binds raw/semantic subject relationships and verifier/trust context without acting as an unforgeable serialized capability.

An expected selection manifest defines the cases/observations meant to run. An execution receipt accounts for selected outcomes and terminal worker state, including skips, filters, quarantine, timeout, cancellation and malformed result failures. Retry relationships reference the same logical case rather than producing additional minimum-count credit.

A campaign identifies an authorized evaluation/acquisition grouping for an exact subject and selection. Campaign-selection and supersession rules are explicit; producer-controlled observation time is not sole ordering authority. History remains immutable even when a later campaign becomes the current selected one.

A stored import associates raw object identity, semantic evidence identity, execution identity and verification metadata. Duplicate delivery is idempotent. Contradictory authenticated payloads for one immutable attempt become a conflict record. Unrelated rejected evidence must not accidentally poison an otherwise complete scope; relevant malformed/conflicting inputs remain visible and non-authorizing.

## Authority and release records (R23–R29, R63)

Protected authority includes baseline contract/policy/participation/selection/evidence mapping/waiver/trust/evaluator identities. Trust configuration defines authorized identities/issuers/builders, algorithms, required material and temporal constraints. Exact operational values require owner configuration.

Candidate readiness is distinct from a post-deployment observation. A decision can be signed whether it passes or fails. A release authorization references the accepted exact decision, artifact/participant set, destination/channel, explicit allowed deviations, authority and validity. Deployment consumption checks those bindings; it does not execute deployments itself. A deployment receipt records what was actually promoted and supports later conformance evaluation.

No record's identity may depend on a descendant that references it. A decision cannot be part of the input evidence identity needed to construct itself.

## Storage and migrations

Objects are immutable; the local transactional index contains discoverable references and derived query indexes. It cannot mint trust. Persist enough manifests and provenance to reconstruct decisions and recover indexes. Database migration versions are independent of wire/evaluator/ABI versions. Retain original signed payloads through conversions and explicit migrations. See COMPATIBILITY.md for the version decision gate.

## Approved validated-state and observation closure

RA05/RA06 require finalization and prepared completeness to be constructed only after expansion, inherited-risk validation and complete-coordinate accounting. Public hidden constructors and deserialization must not bypass these boundaries; retain pure acyclic crate dependencies when allocating ownership.

Exact raw bytes, semantic assertions and immutable attempts remain separate identities (RA08/RA15). Duplicated delivery preserves provenance without count inflation; same-attempt conflicts remain durable and relevant to their coordinates. RA13 requires one strict envelope/payload/subject/kind/reference/time conversion before admission. Parsing is not verification.

Observed role/inventory/exposure facts are independently obtained for an independently expected subject; copied authored declarations or a document's presence are insufficient (RA21). Waivers bind exact policy/profile revisions, subject and context and enumerate only actual contributions (RA09).
