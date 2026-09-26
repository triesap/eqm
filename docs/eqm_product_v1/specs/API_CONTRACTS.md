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

## Decision and query content

Every decision binds scope, required universe/completeness, authority and policy/profile revisions, exact participant subjects, selected campaigns, input references, evaluator identity, evaluation time, and relevant validity/trust context. Include obligation statuses, accepted/rejected evidence with reasons, expected missing cases, exposure comparisons, applied waivers and aggregate conclusions.

Display state and diagnostic reason are not interchangeable. A summary status can select precedence while explanations preserve all causes. Show unknown, missing, stale, unstable, failed, conditional and not-applicable distinctly; never collapse them to a green boolean.

A query view identifies the snapshot it belongs to. Cursors are bound to snapshot, filter and ordering; they cannot continue across different generations silently. Truncated views state truncation/omitted content without changing scope or authorization. Incremental notifications point to a new snapshot or bounded delta with clear generation identity. Do not claim an empty findings resource is complete when evaluation never ran.

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
| Integers | Non-negative counts or `\|n\| <= 2^53-1`; larger/exact quantities use decimal strings `^(0\|[1-9][0-9]*)$`; floats, `NaN`, infinity, negatives and fractional counts rejected |
| Time | Canonical UTC RFC 3339 with literal `Z`; `AutoSi` subseconds; exact nanosecond arithmetic; no offsets/leap seconds; no hidden skew |
| Absence | Optional members are omitted, never `null`; explicit `null` rejected where absence applies |
| Sets/sequences | Sets are canonically ordered duplicate-free arrays; sequences preserve order |
| Text | Exact NFC required; non-NFC rejected rather than normalized |
| Digests | `sha256:` plus 64 lowercase hex; domain-separated |

Public member spellings, command flags, and Kotlin/RPC names remain closed at
their owning steps; this ADR fixes namespace and representation only. The
normative machine-readable vectors live in `eqm_test_support::representation_vectors`.
