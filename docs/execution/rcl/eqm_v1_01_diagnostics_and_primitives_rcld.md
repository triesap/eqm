# EquivalenceMatrix V1 RCLD 01: Diagnostics And Primitives

Status: ready; RCLD 00 complete
Created: 2026-08-07
Updated: 2026-08-07
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 00
Current checkpoint: `step_009`

## Purpose

Implement the validated, deterministic primitives used by every later domain,
manifest, engine, protocol, runner, CLI, and MCP type.

## Scope Boundary

This RCLD is confined to `eqm_domain` primitives and direct tests. It does not
parse files, perform I/O, resolve graphs, serialize public protocol envelopes,
launch processes, or introduce compatibility representations.

## Definition Of Green

- Every primitive is constructible only through validated APIs.
- Validation follows the repository-owned executable contracts exactly.
- Errors produce typed diagnostics without panicking on user input.
- Ordering, equality, hashing, display, and serialization helpers are
  deterministic.
- `eqm_domain` retains no I/O, process, network, Git, terminal, CLI, or MCP
  dependency.

## Verification Lane

Run the standard locked workspace lane plus focused `eqm_domain` unit and
property tests. Negative tests must cover boundaries and malformed input, not
only representative invalid values.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_009` | Define diagnostic code, severity, source/related locations, remediation, stable ordering, and registry linkage. | Codes match the approved allocation; source ordering and complete diagnostic rendering are tested. | `feat(domain): define diagnostics` |
| `step_010` | Define exact-current `SchemaUri`, `SchemaVersion`, and `ToolVersion` primitives. | Malformed, foreign, old, and future schema values fail closed; current values round-trip. | `feat(domain): define schema primitives` |
| `step_011` | Define typed IDs for every entity and local/full requirement identity. | ASCII grammar, segment/full limits, full qualification, ordering, and property tests pass. | `feat(domain): define identifier newtypes` |
| `step_012` | Define lexical `RepoPath` validation and portable collision keys. | Absolute, drive-relative, traversal, NUL, separator, normalization, and case-collision fixtures are covered without filesystem I/O. | `feat(domain): define repository path type` |
| `step_013` | Define fixed SHA-256 content digest parsing, bytes, display, and domain-labeled digest inputs where specified. | Length, prefix, case, invalid hex, and round-trip vectors pass. | `feat(domain): define digest primitives` |
| `step_014` | Define UTC instant, calendar date, and bounded duration-millisecond types. | Exact accepted formats, overflow, non-UTC, invalid date, and deterministic ordering tests pass. | `feat(domain): define time primitives` |
| `step_015` | Define validated owner, issue, design, catalog, CI, and release references. | Only approved schemes and component grammar pass; no URL fetching or scheme fallback exists. | `feat(domain): define external references` |
| `step_016` | Define closed lifecycle and risk vocabularies with approved ordering/elevation rules. | Exhaustive enum, ordering, child-elevation, and invalid-wire-value tests pass. | `feat(domain): define lifecycle and risk enums` |

## Reconciliation Rules

- One primitive checkpoint is active at a time.
- Later checkpoints may consume only committed validated APIs.
- A missing vocabulary or validation rule blocks the checkpoint and returns to
  RCLD 00 authority; it is not invented locally.
- RCLD 02 begins only after the full workspace and focused domain lane pass.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_009`-`step_016` | pending | - |
