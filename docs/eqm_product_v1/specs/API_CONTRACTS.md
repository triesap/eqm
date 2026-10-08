# Public API and presentation contracts

This file specifies behavior and logical payload content. It deliberately does not invent final Rust signatures, JSON member spellings, schema URLs, Kotlin package names, RPC method names, or CLI flags absent from the repository. Freeze those exact representations in the protocol/API steps, generate schemas from source, and update consumers/tests together.

## Operations (R04–R05, R44–R47, R60)

| Logical operation | Inputs | Output/guarantee | Effects |
|---|---|---|---|
| Prepare authority/scope | Explicit repo/authority, exact mode/policy/profiles/subjects and limits | Validated authority and enumerated required universe, or structured errors | Bounded authorized reads only |
| Plan acquisition | Prepared scope, selectors, authorized backend configuration | Concrete plan including executable digest, argv, cwd, environment references, limits and outputs | No execution; secret values absent |
| Acquire/import | Approved plan or authorized imported bytes and verification material | Immutable objects, execution/import receipts and explicit outcomes | Authorized execution/I/O only |
| Evaluate | Frozen validated input manifest and explicit time/trust context | Complete immutable decision with all reasons | Pure; no implicit fetching/execution |
| Query/explain | Snapshot identity, exact filters, pagination cursor and display budget | Coherent snapshot-bound views | Reads only |
| Replay historical decision | Archived exact inputs, evaluator identity and recorded time/context | Reproduced historical conclusion or explicit missing support | No promotion authority |
| Check current acceptance | Decision and current authorized temporal/trust/deployment context | Acceptable/unacceptable/unknown facts with reasons | Does not rewrite history |
| Emit/sign statement | Exact decision, explicit optional signer authority | Accurate unsigned or signed statement, including failed/unknown decisions | Signing only when separately authorized |
| Authorize release | Protected decision/policy, exact artifacts, destination/channel, validity and permitted deviations | Protected authorization or non-authorizing outcome | No implicit publication or deployment |

Retain existing commands (`doctor`, `validate`, `show`, `locate`, `context`, `obligations`, `check`, `matrix`, `verify`, `affected`, `diff`, `reconcile`, release checking and `attest`) where their public grammar is appropriate. Read the actual parser/docs before changing command syntax. Wire the existing commands to these shared use cases. A new operation does not imply permission to invent a new public command family in advance.

## Validated evidence conversion (S059)

S059 exposes `eqm_application::admit_evidence_bytes` for exact bounded v2 bytes
with explicit `EvidenceAdmissionContext`, `EvidenceLineage` and
`EvidenceExecution`. It calls the strict v2 reader and the existing protocol
`admit_evidence` converter, returning the opaque `AdmittedEvidenceV2` with
unchanged originals. `EvidenceConversionError` retains distinct Decode and
Admission causes. This pure conversion supplies no independent verification,
store access, clock, execution or legacy fallback. Later ingress and frozen
evaluation steps consume this same boundary with their own independently
prepared expectations; the existing legacy domain read port is preserved.

## Candidate normalization (S060)

S060 adds pure `normalize_admitted_evidence` over explicit
`AdmittedEvidenceDelivery` references. Engine's `normalize_evidence_candidates`
returns an opaque `NormalizedEvidenceSet` grouped by complete typed subject and
lineage, retaining each semantic assertion's raw/mapping/provenance references.
The inclusive input bound is 10,000 deliveries; exceeding it returns
`CandidateNormalizationError::LimitExceeded` before allocation. Identity
inconsistency is explicit rather than silently merged. Group/transport counts
are not observation credit or independent verification. The normalized
structural coverage API exposes only relevant conflicting attempts after
matching; full-coordinate eligibility and authentication remain separate.

## Decision and query content

S061 strengthens `CoverageExpectation` to include the full primary subject and
runtime-facts reference. `evaluate_complete_evidence_coverage` and application
`match_admitted_evidence` consume explicit complete obligation/identity, frozen
selection and participant-subject claims. The private report exposes structural
eligibility, rejected axes, relevant conflicts and all normalized references;
its status and matching identifiers grant no verification or protected provenance.
Missing context never falls back to expected values. Complete-context work is
bounded at 1,000,000 slots with an explicit error, in addition to 10,000 deliveries.
Protected frozen-input/status integration retains its independent owning gates.

S062 `collect_logical_observations` and application
`collect_admitted_observations` derive structural logical units from complete
matching across every frozen source. Source/case AND members remain visible;
transports and attempts do not add units. Relevant immutable contradictions
remain Unknown across local specification boundaries. Typed record units use
exact referenced digests, preserving one unit for aliases of one record.
The private report retains selected case labels separately from typed units,
missing members, all matching history and explicit independent prerequisites.
Neither completeness nor unit cardinality establishes successful count credit,
verified facts/principals/records or conformance. Threshold and outcome evaluation
remain separate. The 1,000,000 work bound applies cumulatively before repeated
matching, with the shared 10,000 delivery bound and explicit overflow errors.

Every decision binds scope, required universe/completeness, authority and policy/profile revisions, exact participant subjects, selected campaigns, input references, evaluator identity, evaluation time, and relevant validity/trust context. Include obligation statuses, accepted/rejected evidence with reasons, expected missing cases, exposure comparisons, applied waivers and aggregate conclusions.

Display state and diagnostic reason are not interchangeable. A summary status can select precedence while explanations preserve all causes. Show unknown, missing, stale, unstable, failed, conditional and not-applicable distinctly; never collapse them to a green boolean.

A query view identifies the snapshot it belongs to. Cursors are bound to snapshot, filter and ordering; they cannot continue across different generations silently. Truncated views state truncation/omitted content without changing scope or authorization. Incremental notifications point to a new snapshot or bounded delta with clear generation identity. Do not claim an empty findings resource is complete when evaluation never ran.


S063 count constraints use the actual prepared coordinate's composed policy minimum
and exact source specification minima. Engine validates complete frozen manifests,
including all alternative definitions, under a cumulative 1,000,000-work budget
before indexed equality. The engine collects internally from those same validated expectations and accepts
no separately supplied report. Application pins full frozen expectations and
identity to opaque pre-result preparation before adapting admitted bytes; the 10,000-delivery
and S062 complete matching work limits remain in force. Callers supply no minimum.
The opaque report retains policy/specification/effective minima, distinct structural
capacity, every observation/audit reference and remaining independent prerequisites.
Missing AND members and relevant immutable conflicts cannot be hidden by capacity.
Exact referenced-record capacity is at most one. `CapacityMet` grants no successful
eligible outcome, independently verified fact/principal/record, trust, protected
consumer status, conformance or release permission. Engine identity inputs remain
claims; application pre-result binding also grants no independent evidence trust.
Outcome/principal/campaign semantics and protected consumer integration remain
S064–S068/S073 prerequisites.

## Error boundary (R45, R50)

Use typed contextual errors internally. At public boundaries, provide stable error codes, command/operation identity, safe contextual details and source locations where available, without leaking secrets. Do not replace every loader error with a stage name. Requested JSON output remains a valid error envelope where parsing established the requested format; usage errors that occur before format selection must have a documented behavior and exit classification. MCP maps errors to protocol-appropriate responses without calculating different conformance.

Distinguish invalid request, invalid authority, missing/unsupported capability, acquisition failure, malformed evidence, rejected verification, resource limit, storage failure, cancellation and internal failure. Exact names/exit numbers must be resolved against existing docs rather than invented here. Result success and process success must not conflate an empty/partial acquisition with conformance.

## Canonical wire contract (R21, R52)

The schema identity, canonicalization algorithm/version and semantic projection are explicit. Reject duplicate keys and unknown security-relevant semantics. Define numeric safe ranges or decimal-string representation, timestamp precision/UTC rules, absent versus null, and set versus sequence encoding in test vectors before implementation integration. Canonical JSON preserves Unicode data; do not use arbitrary cross-language object-map iteration or implicit normalization.

Generated schemas are committed alongside source DTO changes and checked for parity. Unsupported future schema/status versions are explicit failures or unavailable views, never success. Exact public names and compatibility behavior are governed by the version ADR, not this package's product-spec name.

## Optional native boundary (R49)

When activated, expose batch evaluate/query operations with opaque lifecycle-managed handles where justified. Define allocation/free ownership, invalid handle behavior, cancellation versus completion, thread safety, error translation, version negotiation, numeric conversions, and bounded serialized buffers. Do not expose Rust layout/Vec/String/Arc over a stable ABI or require a new runtime for pure evaluation. Platform adapters own their packaging and platform-specific threading.

## Consumer/UI requirements

Only conformance-report semantics are in EQM scope: show scope, freshness, trust, reasons, explicit deviations and completeness; paginate large matrices and avoid per-row process/native calls. Actual account/task screens, camera flows, business state and UI visual design remain product-owned. No screen mockups or invented UI workflows are required by this handoff.

## Approved interface regression closure

ADR-0012 approves a new wire namespace for incompatible changes; S006 owns exact names, representation ranges and transition vectors. Keep JSON numeric ranges/decimal strings, canonical nanosecond UTC time, null/absence, unordered sets/ordered sequences and digest domains explicit before new exposure. Existing schema URLs must not acquire incompatible semantics.

Actual success and error outputs for every public command family must validate against generated schemas (RA14). Validate supported command/output-format combinations before acquisition or storage (RA20); an accepted format has a renderer. Once an operation has committed, an output-sink failure does not erase its receipt or imply rollback: the error remains truthful and the operation stays discoverable.

MCP preserves startup configuration, selected profiles, authority and one immutable snapshot across tools/resources; refresh and cursor invalidation are explicit (RA17). Bounds include the complete serialized envelope, queued work and pagination state (RA18), with a validated minimum budget for bounded errors. Consumer doctor validates real consumer prerequisites and generated states without importing EQM contributor requirements (RA19).

## Frozen v2 representations (S006)

ADR-0013 freezes the exact representations for incompatible wire work under
namespace `eqm:v2` and schema base
`https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/`. Existing
`schemas/v1` bytes and URIs are preserved and never reinterpreted; there is no
implicit v1/v2 reader.

| Boundary | Frozen rule |
| --- | --- |
| Namespace | `eqm:v2`; new digests use `eqm:v2:<kind>` with the zero-byte delimiter and SHA-256 |
| Integers | v2 full u64/i64 ranges; integer tokens iff within the safe interval; strings iff outside it; reject noncanonical strings, negative zero, overflow, fractions and exponent tokens. Preserve v1 acceptance. |
| Time | v2 years 0001–9999, literal Z, minimal AutoSi fractions, exact nanoseconds; reject leap seconds/offsets/redundant precision. Preserve v1 parser behavior. |
| Absence | Optional members are omitted, never `null`; explicit `null` rejected where absence applies |
| Sets/sequences | Sets are canonically ordered duplicate-free arrays; sequences preserve order |
| Text | Authored text is exact NFC (rejected if not NFC); opaque payload/attachment bytes are neither normalized nor NFC-validated and are hashed exactly |
| Digests | `sha256:` plus 64 lowercase hex; domain-separated |

Public member spellings, command flags, and Kotlin/RPC names remain closed at
their owning steps; this ADR fixes namespace and representation only. The
normative machine-readable vectors live in `eqm_test_support::representation_vectors`.

DEV-002 assigns the initial five real v2 protocol families and their nested
DTOs to S015; explicit null rejection and omitted-optional serialization belong
to that step. Later incompatible shapes receive their own explicit v2 schemas
in their existing owning steps. Generated representational constraints and
runtime validators must be checked against the same positive/negative corpus.

## DEV-003 conformance clarification

DEV-003 in the governing COMMIT_SEQUENCE.md closes reader version isolation,
nested structural validation and schema assertions. A known SchemaUri is not
proof that a reader or generated schema exists. Existing authored/workspace/lock
readers reject v2 until their owning integration steps. The five v2 families
must enforce exact root/nested schema markers and uniform declared byte/count
bounds. v1 schemas and accepted numeric/time forms remain unchanged.

Raw canonical JSON token validation, Draft 2020-12 value validation and runtime
semantic validation are separate obligations. JSON Schema integer includes
integral values written as 1.0 or 1e3; raw v2 readers reject those spellings.
Do not claim a schema proves original token form, duplicate-member rejection,
digest consistency or trust. Conformance tests use an actual standards validator
with date-time format assertions and local-only resolution. v2 UTC patterns
still enforce minimal AutoSi groups; runtime validation retains exact calendar
checks. No canonical input rule is relaxed by this clarification.

## DEV-004 collection and text allocation

S015 is accepted. DEV-004 in the governing COMMIT_SEQUENCE.md freezes S016's
field classification and executable controls: semantic sets sort by complete
canonical member JSON bytes, preserve existing logical-key uniqueness, and
reject duplicates before collection; attempts, diagnostics and related locations
retain sequence order. Strict object maps retain JCS key ordering. Preserve
original bytes separately from canonical semantic representations and retain all
S015 numeric/time/absence/bound checks. No v1 canonical projection is redefined.

Authored metadata is NFC-validated without normalization; opaque source/payload
bytes are preserved exactly. The text-vector input is JSON, its output canonical
JSON; RV-TEXT-2's Unicode escape denotes accepted café and canonical output emits
that literal UTF-8 text. A separate opaque-byte discipline includes decomposed
and non-UTF-8 cases. The governing amendment specifies real-ingress integration
and regression evidence; vector inventory alone is not implementation.

## DEV-005 structural field validation clarification

The governing sequence's DEV-005 field/type table is normative for S016. Exact
NFC is necessary but does not replace typed identifier, vocabulary, text-length
or positive-revision constraints. Shared validation covers each existing v2 root
and embedded inventory. Opaque bytes and arbitrary diagnostic prose retain their
separate classification. The unconsumed EvidenceSelectorV2Dto helper does not
prove consumer validation; the first owning v2 selector ingress must call its
typed validation. No extra schema family or relationship/trust acceptance is
introduced to manufacture that integration. S017 uses a separate v2 binding
projection under the existing BindingDigest domain; all v1 bytes stay frozen.

## DEV-006 component extension type preservation

Every v2 component ExtensionValue is recursively encoded as an object with exactly
`type` and `value`: tags boolean/integer/string/array/object. Integer payloads use
the approved signed numeric forms; array payloads contain ordered tagged values;
object payloads map existing keys to tagged values. The namespace map stays a map
and excludes only dev.equivalencematrix.display. This prevents large Integer values
from colliding with String values containing the same digits, and prevents plain
objects imitating typed tags. V1 remains unchanged. The affected new v2 extension
goldens must change under DEV-006; unaffected base goldens remain exact.

Component APIs retain BindingDigest/EvidenceSpecificationDigest/ContractDigest/
PolicySelectionDigest internally, extracting raw digest text only at explicit
boundaries. S019's exact composition, field spelling, set/sequence allocation and
selection carrier invariants are normative in DEV-006 of COMMIT_SEQUENCE.md.
Unknown selection is a tagged state without value, not null or an omitted dimension.

## DEV-007 target configuration API

Manifest owns project_target_configuration_v2/canonicalize_target_configuration_v2
over a validated Target; CanonicalTargetConfigurationV2 retains bytes and typed
TargetConfigurationDigest. The root is exactly id/root/platform/framework/owners/
extensions with approved v2 forms. Application accepts one PreparedSession and
TargetId, performs exact graph lookup, and returns typed MissingTarget or causal
Projection failure. No missing-target hash or aggregate fallback exists. Missing
future build/source fields are not fabricated. DEV-007 in the governing sequence
allocates the exact regression and later target-field extension obligations.

## DEV-008 provenance completion

ResolvedRunner uses existing RunnerDigest over explicit v2 JCS root definition/
program/approved_guarantees, tagged template semantics and full member ordering.
identity_v2()/canonical_bytes() retain typed identity/bytes; digest() is an explicit
wire adapter. DEV-008 defines fields, bounds, errors, literals and integration.

## DEV-009 checked identity and model allocation

DEV-009 in the sole governing COMMIT_SEQUENCE closes bytes-first lossless v2
semantic evidence, explicit lineage/admission context, recursive source inspection
and the exact shared-source/participant/subject/cohort engineering allocation.
Use its member/version/domain decisions at each owning step; preserve original
criteria, v1 contracts and signed bytes. Parsed identifiers/digests are not trust,
synthetic combinations are not real cohort certification, and new target fields
extend their canonical projection atomically. S043 is the next independent
integration review boundary before routing; full MVP remains S211.

## DEV-030 checked preparation contract

Shared application preparation resolves typed exact policy/profile requests from
one immutable checked session; adapters parse syntax but do not choose authority,
latest revisions or membership. Scenario bindings are explicit preparation input;
development syntax is `--scenario <FULL_REQUIREMENT=SCENARIO_ID@REVISION>`. Protected
modes require the corresponding protected authority input. Duplicate identical or
conflicting entries and missing/ambiguous/extraneous/incompatible refs reject.
A missing active EndToEnd binding yields incomplete/unknown admission, not a
TargetSet fallback. Public finalized-graph/universe promotion bypasses are closed.
Source-generated v2 DTO/schema/reader changes are atomic where a new wire record
is needed; v1 bytes and frozen projection roots remain unchanged.
See [DEV-030](../implementation/COMMIT_SEQUENCE.md) for exact scope/verification.

## DEV-032 admission clarification

An empty participation registry never implies inclusion, including v1 data
without the new axis. Missing scope authority stays non-authorizing while v1
bytes/readers remain unchanged. Checked derivation binds graph, selected exact
policy/profile definitions and applicability values; foreign selections or
contexts cannot shrink scope. Protected unresolved requests require explicit
exact authority rather than unique candidate defaults. Target evidence cannot
prove Provider or multi-participant scope without the exact required subjects
and dependencies. These are existing acceptance closures, governed by
[DEV-032](../implementation/COMMIT_SEQUENCE.md); no new product policy is added.

## DEV-033 admission provenance clarification

Existing exact-context acceptance requires graph semantics, policy/profile
definitions and values, applicability, mode/authority and immutable scope context
to remain bound through derivation and status admission. Equal visible coordinate
labels across graphs with different strength are insufficient. Authorizing status
results require privately validated provenance; a caller-supplied digest/tag or
naked status map is data, not authority. Canonicalization remains application/
manifest owned, the engine pure, and frozen projection roots unchanged.
S054 later extends complete wire identity; present false-success admission must
be repaired in P04-R4. See governing DEV-033 and its independent review.

## DEV-035 S064 selected campaign retry history

Pure engine history validates complete frozen expectations and collects candidates
internally through S063. Application binds actual opaque S054 identity before the
shared S059 admitted-candidate adapter. No supplied earlier report, arbitrary
campaign selector or timestamp can replace frozen context. Matched semantic
assertions remain visible before conflict partition; rejected assertions sharing
the immutable attempt remain audit and never eligible retry history.

Full-subject/logical-case groups retain five-axis lineage and every semantic
assertion across required sources. Same-case failing/passing retries are Unstable;
separate required fail/pass groups compose as Failed. Conflicts and timeout/cancel/
error are Unknown; skip/filter/quarantine/missing source/case is Missing. Preserve
all reasons, raw/mapping/provenance audit, source/case membership and independent
prerequisites. Passed means terminal executable history only, never independently
eligible successful count credit, trust, facts, conformance or release authority.
All non-executable kinds are explicitly pending their independent kind evaluation.
Counts remain a separately retained S063 assessment. Complete incoming executable
payload comparisons across all expectations and retained outcome traversal share
a cumulative 1,000,000-work guard; duplicate/rejected input consumes work without
observation credit. Excess work returns an error without a partial conclusion.
No wire/schema/digest changes. Later owning steps integrate remaining prerequisites.
