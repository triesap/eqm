# Public Protocol Contracts

Status: normative

All EQM JSON protocols are UTF-8 JSON, use exact current schema identities,
reject unknown fields, and enforce the resource limits in the security
contract. Object field order is not semantically significant; emitted objects
and arrays use the deterministic order specified here. Public DTOs are explicit
conversions from domain types and are never direct domain serialization.

## Common Result Envelope

Every CLI JSON response uses the result schema and these exact fields:

| Field | Type | Rule |
| --- | --- | --- |
| `schema` | string | exact `https://schemas.equivalencematrix.dev/v1/result` |
| `tool_version` | semantic-version string | executing EQM version |
| `command` | command identity | exact invoked command, with spaces represented by `_` |
| `workspace_digest` | digest or JSON `null` | null only when a valid finalized graph was not available |
| `context` | invocation context | exact prepared evaluation inputs |
| `result` | command result or JSON `null` | null only when the command could not produce its typed result |
| `diagnostics` | diagnostic array | sorted by the diagnostic ordering contract |

The invocation context has exact fields `mode`, `profiles`, `subject`,
`baseline`, `offline`, and `evaluated_at`. `subject` and `baseline` are JSON
null when not applicable. Profile values are sorted by `(profile, dimension)`.
The command result is a discriminated object whose `kind` equals the command
identity.

## Diagnostic DTO

Schema: `https://schemas.equivalencematrix.dev/v1/diagnostic`.

| Field | Type | Presence |
| --- | --- | --- |
| `code` | `EQM-E` plus four decimal digits | required |
| `severity` | `error`, `warning`, or `note` | required |
| `message` | bounded normalized text | required |
| `source` | source location | optional |
| `related` | source-location array | default empty |
| `unit` | unit ID | optional |
| `target` | target ID | optional |
| `requirement` | full requirement ID | optional |
| `obligation` | obligation ID | optional |
| `facet` | facet | optional |
| `status` | closed status string appropriate to the diagnostic | optional |
| `remediation` | bounded normalized text | optional |

A source location has `uri`, `start`, and `end`; each position has one-based
`line` and `column`. `uri` is repository-relative `file:` data in CLI output or
an `eqm://` resource URI in MCP output. Diagnostics sort by severity, code,
source URI, start position, unit, target, requirement, facet, then message.

## Command Result Variants

All listed fields are required; values that do not apply use an empty array or
explicit JSON null as stated, never omission.

| `kind` | Exact remaining fields |
| --- | --- |
| `init`, `new`, `fmt`, `lock_update` | `dry_run`, `changes`, `written`; changes sort by path and contain `path`, `action`, `before_digest`, `after_digest` |
| `validate` | `valid`, `entity_counts`, `graph_digest` |
| `check` | `status`, `obligation_counts`, `findings` |
| `show` | `entity_kind`, `entity_id`, `entity` |
| `locate` | `unit`, `target`, `locations` sorted by `(role, path, symbol)` |
| `context` | `unit`, `target`, `authority`, `product_data`, `obligations`, `evidence`, `findings`, `waivers`, `truncated`, `omitted_bytes` |
| `matrix` | `matrix_kind`, `rows`, `columns`, `cells` sorted by row then column |
| `obligations` | `filters`, `obligations` sorted by obligation ID |
| `diff` | `baseline_digest`, `candidate_digest`, `changes` sorted by the evaluation contract |
| `affected` | `baseline_digest`, `changed_paths`, `units`, `obligations`, `conservative` |
| `discover` | `adapter`, `target`, `inventory` |
| `reconcile` | `target`, `unit`, `facts`, `comparisons` |
| `verify` | `selection`, `evidence_results`, `summary`; evidence sorts by result ID |
| `attest` | `statement`, `signed`, `signer` where signer is null when unsigned |
| `release_check` | `subject`, `status`, `conformance`, `equivalence`, `exposure`, `waivers` |
| `explain` | `code`, `title`, `authority`, `explanation`, `remediation` |
| `doctor` | `checks`, `status`; checks sort by check ID |

Entity, finding, obligation, matrix, fact, comparison, change, and check records
use their closed domain-to-protocol tables generated with their owning schema.
They cannot contain arbitrary maps except a validated `extensions` value or an
explicitly labeled untrusted-data field.

### Shared report records

| Record | Exact fields |
| --- | --- |
| entity reference | `kind`, `id`, `revision`, `digest` where revision/digest are null only for unversioned capability/target authority |
| location | `role`, `path`, `symbol`, `source`; symbol/source may be null |
| obligation | `id`, `policy`, `profile_values`, `unit`, `requirement`, `scope`, `scope_subject`, `facet`, `minimum_trust`, `maximum_age_ms`, `minimum_count`, `status`, `evidence`, `waiver` |
| finding | `diagnostic_code`, `obligation`, `status`, `evidence`, `waiver`; nullable references are explicit |
| matrix row/column | `id`, `label` |
| matrix cell | `row`, `column`, `status`, `obligations`, `diagnostic_codes` |
| exposure fact | `name`, `value`, `source`, `freshness`, `effective_trust` |
| exposure comparison | `fact`, `expected`, `observed`, `result` |
| semantic change | `unit`, `requirement`, `target`, `facet`, `kind`, `field`, `before`, `after`; non-applicable coordinates and values are null |
| doctor check | `id`, `status`, `message`, `remediation`; remediation may be null |

`before` and `after` are schema-bounded JSON values for the named field, not
arbitrary diagnostic payloads. IDs and reference arrays use the ordering rules
from the evaluation contract. Entity payloads in `show` use the exact semantic
projection for their kind plus a separate `source` location; parser DTO fields
that are absent from the semantic entity are not exposed.

## Normalized Test Result

Schema: `https://schemas.equivalencematrix.dev/v1/test-result`.

| Field | Type | Rule |
| --- | --- | --- |
| `schema` | exact schema URI | required |
| `selector` | evidence selector | required |
| `attempts` | nonempty attempt array | required, sorted by `number` |
| `counts` | count object | required |
| `started_at`, `finished_at` | UTC instants | required and ordered |
| `attachments` | attachment array | default empty, sorted by name |

An attempt has `number`, `outcome`, `started_at`, `finished_at`, and optional
bounded `message`. Counts have exactly `selected`, `passed`, `failed`,
`skipped`, `filtered`, and `quarantined`, all nonnegative integers whose totals
are internally consistent. An attachment has `name`, `media_type`, `digest`,
and `size`; attachment bytes are external and content-addressed.

## Immutable Evidence Result

Schema: `https://schemas.equivalencematrix.dev/v1/evidence-result`.

| Field | Type | Rule |
| --- | --- | --- |
| `schema`, `id` | exact URI and digest-derived result ID | required |
| `subject` | exact subject object | required |
| `target`, `unit` | IDs | required |
| `requirements`, `facets` | sorted nonempty sets | required |
| `kind` | evidence kind | required |
| `evidence_spec_digest`, `contract_digest`, `binding_digest`, `policy_digest` | digests | required |
| `runner_digest`, `adapter_digest`, `runtime_facts_digest`, `release_record_digest` | digest or null | always present |
| `profile_values` | sorted profile-value array | required |
| `producer` | producer identity | required |
| `claimed_trust` | trust level | required |
| `observed_at` | UTC instant | required |
| `payload` | kind-discriminated normalized payload | required |
| `attachments` | attachment array | default empty |
| `result_digest` | digest | required; covers every prior field |

The result ID is derived from `result_digest`. The file name is the digest and
an existing different payload at that name is a hard integrity error. Producer
and claimed trust are assertions; evaluation computes effective trust
independently.

## Inventory And Runtime Facts

Inventory schema: `https://schemas.equivalencematrix.dev/v1/inventory`.

An inventory has exact fields `schema`, `adapter`, `adapter_digest`, `subject`,
`target`, `generated_at`, `completeness`, `entries`, `diagnostics`, and
`inventory_digest`. Entries sort by `(kind, key)` and have exact `kind`, `key`,
`attributes`, and `source` fields. `attributes` is provider-neutral bounded
JSON data. Completeness is `complete`, `partial`, or `unknown`; only `complete`
can prove absence.

Runtime-facts schema:
`https://schemas.equivalencematrix.dev/v1/runtime-facts`. It has exact fields
`schema`, `provider`, `subject`, `target`, `profile_values`, `observed_at`,
`expires_at`, `facts`, `producer`, `claimed_trust`, and `facts_digest`. Facts
sort by `(surface, dimension, key)` and contain exact `surface`, `dimension`,
`key`, and typed `value`. Individual-user identifiers and free-form provider
payloads are forbidden.

## Release Record

Schema: `https://schemas.equivalencematrix.dev/v1/release-record`.

Exact fields are `schema`, `target`, `app_version`, `build_number`,
`source_commit`, `artifact_digest`, `channel`, `released_at`, `producer`,
`claimed_trust`, and `record_digest`. `source_commit` is an immutable full
object ID, not a branch or tag. The record digest covers every preceding field.

## Adapter Protocol

Request schema: `https://schemas.equivalencematrix.dev/v1/adapter-request`.
Response schema: `https://schemas.equivalencematrix.dev/v1/adapter-response`.

| Request field | Type |
| --- | --- |
| `schema` | exact request URI |
| `request_id` | invocation-scoped ID |
| `adapter`, `adapter_digest` | exact locked identity |
| `operation` | `discover` |
| `subject`, `target`, `target_root` | exact prepared inputs |
| `limits` | timeout, input, output, entry, and depth bounds |

The response has exact `schema`, `request_id`, `adapter`, `adapter_digest`,
`status`, `inventory`, and `diagnostics`. Status is `ok`, `partial`, or `error`.
Inventory is required for ok/partial and null for error. A request-ID, adapter,
digest, subject, or target mismatch invalidates the entire response. Adapter
stderr is capped diagnostic data and never part of JSON stdout.

## Attestation

EQM emits an in-toto Statement v1 with:

- `_type` exactly `https://in-toto.io/Statement/v1`;
- nonempty `subject`, each with `name` and a `digest` object containing exactly
  `sha256`;
- `predicateType` exactly
  `https://schemas.equivalencematrix.dev/v1/attestation`;
- `predicate` with exact fields `tool_version`, `command`, `workspace_digest`,
  `policy_digest`, `profile_values`, `evaluation_subject`, `evidence_digests`,
  `runtime_facts_digest`, `release_record_digest`, `trust_config_digest`,
  `evaluated_at`, `conformance`, `equivalence`, `release_status`, and `waivers`.

Evidence digests and waivers sort by identity. Null is used only for a
not-applicable runtime-facts or release-record digest. The predicate itself
does not claim that it is signed.

A DSSE envelope has exact `payloadType`, `payload`, and `signatures` fields.
`payloadType` is `application/vnd.in-toto+json`; payload is base64 of the exact
statement bytes. Each signature has `keyid` and `sig`. An empty signatures
array is unsigned and must be labeled as such. Only the security contract's
allowed algorithms and configured identities may produce trusted signatures.

## SARIF Mapping

SARIF output is SARIF 2.1.0 with one run and the EQM tool driver. It is a
findings view, not the complete EQM result model.

- Each diagnostic code is one driver rule with rule ID equal to the code.
- Error, warning, and note map to SARIF `error`, `warning`, and `note`.
- Message, primary physical location, related locations, and remediation map
  without embedding untrusted text as Markdown commands or links.
- Unit, target, requirement, obligation, facet, and status are string
  properties under a fixed EQM property namespace.
- Results sort in diagnostic order; rules sort by code; artifact locations are
  normalized repository-relative URIs.
- SARIF stdout contains exactly one SARIF document and no EQM JSON envelope.

Commands without findings do not support SARIF. Requesting it is a usage error.

## MCP DTOs

EQM MCP uses stdio JSON-RPC 2.0 and supports only MCP protocol version
`2025-06-18` in v1. Frames are one JSON-RPC message each; protocol stdout has
no logs or progress bytes.

A request has exact `jsonrpc = "2.0"`, `id`, `method`, and `params`. A response
has exact `jsonrpc`, `id`, and exactly one of `result` or `error`. An error has
`code`, `message`, and optional bounded `data`. Notifications omit `id` and
never receive a response.

EQM resource URIs are:

- `eqm://v1/workspace`;
- `eqm://v1/unit/{percent-encoded-id}`;
- `eqm://v1/context/{percent-encoded-id}`;
- `eqm://v1/findings`.

Tool names are exactly `eqm_context`, `eqm_matrix`, `eqm_affected`,
`eqm_check`, `eqm_explain`, and the separately authorized `eqm_verify`. Tool
input schemas are closed projections of the corresponding CLI arguments. Tool
results use the common EQM result envelope as structured content; human text
is an optional rendering and is never the authoritative payload.

Unsupported protocol versions, methods, tools, URI schemes, URI versions,
unknown fields, duplicate IDs, oversized frames, and malformed JSON fail
closed. MCP adds no semantic result field that is absent from the JSON protocol.
