# EquivalenceMatrix V1 RCLD 10: MCP

Status: in progress; Step 115 complete
Created: 2026-08-07
Updated: 2026-08-08
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 09
Current checkpoint: `step_116`

## Purpose

Implement a thin current-version MCP stdio adapter over the same prepared
workspace session, engine APIs, and public protocol DTOs used by the CLI.

## Scope Boundary

MCP introduces no business logic, manifest parser, policy evaluator, report
model, or hidden compatibility protocol. It is local stdio only. Read tools are
the default. Evidence execution requires explicit server authorization and
remains denied by default. MCP never creates waivers or mutates authored EQM
metadata.

## Definition Of Green

- No core crate depends on MCP.
- Every resource uses the `eqm://` scheme and current protocol only.
- Read tools call shared engine/session APIs and return protocol DTOs.
- Protocol stdout contains only MCP frames; logs use stderr.
- Unsupported protocol versions fail closed.
- Executing tools are denied by default and audited when explicitly enabled.
- Product prose, source, adapter output, and logs remain labeled untrusted.

## Verification Lane

Run the standard locked workspace lane plus dependency-DAG, URI, tool-schema,
stdio framing, current-version handshake, unsupported-version, stdout purity,
default denial, authorization, and audit tests.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_112` | Establish `eqm_mcp` dependency boundary and prepared-session API supplied by CLI orchestration. | Automated DAG check proves no core-to-MCP edge and no duplicated loader/evaluator logic. | `feat(mcp): add adapter boundary` |
| `step_113` | Map workspace, unit, context, and findings resources under `eqm://`. | URI parser rejects wrong scheme/version/path; resource payloads match protocol schemas and stable ordering. | `feat(mcp): map resources` |
| `step_114` | Add read-only context, matrix, affected, check, and explain tools. | Tool schemas are closed; calls equal corresponding engine/CLI JSON results; no filesystem/process write occurs. | `feat(mcp): add read-only tools` |
| `step_115` | Implement current MCP stdio server and CLI `mcp serve` integration. | Handshake, request/response, malformed input, unsupported version, framing, stderr logs, and graceful shutdown pass. | `feat(mcp): implement stdio server` |
| `step_116` | Add explicitly configured `verify` tool gate, default denial, authority checks, and audit record; forbid waiver creation. | Default and insufficient authority deny before execution; approved synthetic test authority is bounded/audited; no waiver tool exists. | `feat(mcp): gate executing tools` |

## Reconciliation Rules

- Adding an MCP-only semantic field or evaluation branch is forbidden.
- MCP execution authorization is independent of product prose and workspace
  metadata.
- RCLD 11 begins only after stdio purity and default-denial tests are green.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_112` | complete | `eqm_mcp` accepts only a borrowed canonical repository root, finalized immutable graph, semantic source map, and exact workspace digest supplied by CLI orchestration; it contains no manifest-loader dependency or duplicate preparation path, a DAG test proves every core crate remains independent of MCP, CLI tests prove identity-preserving borrowing, doctor exercises the production boundary, and the adapter timing fixture now separates normal startup allowance from its deliberately short timeout assertion without changing production limits |
| `step_113` | complete | MCP exposes only the closed `eqm://v1/workspace`, `eqm://v1/unit/{percent-encoded-id}`, `eqm://v1/context/{percent-encoded-id}`, and `eqm://v1/findings` URI families; strict parsing rejects wrong schemes, versions, paths, encodings, and absent units, listing is stable and deduplicated, reads produce common current result envelopes with exact workspace digests and resource source URIs, and mixed context product/tool records carry explicit untrusted labels while finalized authority remains distinguished |
| `step_114` | complete | MCP registers exactly `eqm_context`, `eqm_matrix`, `eqm_affected`, `eqm_check`, and `eqm_explain` as read-only tools with closed schemas and strict required/type/unknown-field validation; its router delegates through a CLI-owned handler that rebuilds the corresponding approved CLI invocation and calls the same command implementation, then rejects mismatched result schemas or command envelopes, and integration tests prove every tool returns the corresponding structured CLI envelope without changing repository status |
| `step_115` | complete | `mcp serve` takes over process stdio before normal CLI progress or rendering, supports only MCP `2025-06-18`, enforces bounded newline-delimited JSON-RPC objects and closed request fields, performs initialization, ping, resource list/read, tool list/call, notification suppression, EOF shutdown, stable JSON-RPC errors, and exact structured tool output, while malformed frames, unknown methods, unavailable resources, and unsupported versions fail closed; in-memory and real-binary tests prove protocol stdout contains JSON frames only |
| `step_116` | pending | - |
