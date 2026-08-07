# EquivalenceMatrix V1 RCLD 03: Protocol And Schemas

Status: complete; Steps 033-041 green
Created: 2026-08-07
Updated: 2026-08-07
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 02
Current checkpoint: none

## Purpose

Implement the closed, versioned public protocol for reports, diagnostics,
evidence, adapters, attestations, SARIF, and schema publication while preserving
the separation between internal domain objects and public DTOs.

## Scope Boundary

This RCLD does not parse authored TOML, resolve graphs, evaluate product state,
run commands, or expose a network service. Authored manifest DTO ownership is
corrected to `eqm_manifest`; Step 035 creates those DTO types without adding a
manifest-to-protocol dependency.

## Definition Of Green

- Every public DTO follows a repository-owned exact field table and schema URI.
- Unknown fields and unsupported versions fail closed.
- Machine collections serialize in specified deterministic order.
- Domain types are converted explicitly rather than serialized as protocol.
- Generated JSON Schemas use Draft 2020-12 and match checked-in files.
- SARIF is a findings view while EQM JSON remains the complete authoritative
  model.

## Verification Lane

Run the standard locked workspace lane plus JSON round-trip, unknown-field,
golden, schema-generation, and schema-instance tests for `eqm_protocol` and the
manifest DTO checkpoint.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_033` | Common report envelope, metadata, diagnostic DTO, command identity, workspace digest, result, and diagnostics. | Exact required fields, deterministic ordering, conversion, and golden round-trip pass. | `feat(protocol): create report envelope` |
| `step_034` | Public current-v1 report/protocol schema constants backed by shared domain schema identity primitives. | Constants match approved namespace; old/future schemas are absent. | `feat(protocol): add schema constants` |
| `step_035` | **Corrected ownership:** authored workspace, contract, binding, policy, profile, runner, waiver, and lock DTOs in `eqm_manifest/src/dto/**`. | TOML DTOs deny unknown fields, preserve source conversion needs, and add no `eqm_manifest -> eqm_protocol` dependency. | `feat(manifest): add authored manifest dtos` |
| `step_036` | Validate, check, show, locate, context, matrix, obligations, diff, affected, discover, reconcile, release, and doctor report DTOs. | Each result has a closed schema, exhaustive state representation, and JSON round-trip/golden coverage. | `feat(protocol): add report dtos` |
| `step_037` | Normalized test-result and immutable evidence-result DTOs. | Count/outcome/trust/digest constraints and unknown-field rejection pass. | `feat(protocol): add evidence dtos` |
| `step_038` | Adapter request/response and inventory DTOs with limits, subject, target, and completeness. | Invalid schema/version, oversized response metadata, and unknown variants fail. | `feat(protocol): add adapter protocol` |
| `step_039` | EQM in-toto predicate and DSSE-compatible statement payload. | Subject binding, replay fields, trust metadata, and golden JSON match the approved trust profile. | `feat(protocol): add attestation dto` |
| `step_040` | Deterministic diagnostic-to-SARIF 2.1.0 mapping. | Stable rules, locations, levels, remediation, ordering, and golden SARIF pass. | `feat(protocol): add sarif mapping` |
| `step_041` | Generate and verify protocol-owned JSON Schemas; establish the combined manifest/protocol schema parity entry point. | Generation is deterministic; clean regeneration produces no diff; positive/negative instances validate. | `feat(protocol): generate json schemas` |

## Schema Ownership Rules

- `eqm_manifest` generates schemas for authored TOML shapes.
- `eqm_protocol` generates schemas for JSON/SARIF/adapter/evidence/attestation
  and MCP shapes.
- A repository script invokes both owners and compares all generated files.
- No generated schema is edited by hand.
- Schema constants and generated `$id` values must agree exactly.

## Reconciliation Rules

- Public schema changes require the corresponding authority and golden update
  in the same checkpoint.
- Do not add a compatibility field or serde alias to preserve an earlier
  draft.
- RCLD 04 begins only after clean deterministic schema regeneration.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_033` | complete | Exact-current generic report envelopes validate command discriminants, contexts, diagnostics, and deterministic JSON |
| `step_034` | complete | Public protocol schema constants derive from the shared exact-current domain schema inventory |
| `step_035` | complete | Manifest-owned strict DTOs cover workspace, contracts, bindings, policies, profiles, runners, waivers, and locks |
| `step_036` | complete | Closed outer DTOs cover every command result and shared record with structural deterministic ordering |
| `step_037` | complete | Normalized test and immutable evidence DTOs retain exact subjects, retries, counts, payloads, trust, and digests |
| `step_038` | complete | Adapter requests, responses, and inventories bind exact pins, subjects, limits, completeness, and correlation IDs |
| `step_039` | complete | In-toto Statement v1 and DSSE DTOs retain exact predicate fields, ordered subjects, and explicit signature state |
| `step_040` | complete | Diagnostics map deterministically to one SARIF 2.1.0 run with stable rules, spans, levels, and plain-text remediation |
| `step_041` | complete | Both DTO owners generate deterministic Draft 2020-12 schemas and the aggregate verifier rejects drift |
