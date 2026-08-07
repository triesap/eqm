# EQM V1 Corrected Commit Sequence

Status: normative execution order

The checkpoint maps in `docs/execution/rcl/` define the complete 134-commit
implementation sequence. Execute RCLDs 00 through 13 in numeric order and each
checkpoint within an RCLD in numeric order. A checkpoint advances only after
its scoped definition of green and the standard workspace verification lane
pass, followed by one reviewable commit.

## Wave Order

| Order | RCLD | Checkpoints | Correction incorporated |
| --- | --- | --- | --- |
| 1 | 00 Authority and bootstrap | 001-008 | Valid repository bootstrap precedes product code; Step 007 is implemented as ordered authority subcheckpoints 007a-007h. |
| 2 | 01 Diagnostics and primitives | 009-016 | Stable diagnostics and validated primitives precede all consuming types. |
| 3 | 02 Typed domain graph | 017-032 | Domain meaning, including the relocated 019-021 work, precedes protocol DTOs. |
| 4 | 03 Protocol and schemas | 033-041 | Authored DTO ownership is corrected to the manifest crate. |
| 5 | 04 Manifest system | 042-056 | Canonicalization accepts only a finalized graph and examples use production validation. |
| 6 | 05 Resolution and conformance | 057-070 | Fragment expansion and invariants precede integrated semantic identity and evaluation. |
| 7 | 06 Exposure and analysis | 071-076 | Independent facts and exact baselines precede matrices and explanations. |
| 8 | 07 Runners and discovery | 077-090 | Process/adapters remain bounded, pinned, and out of process. |
| 9 | 08 CLI query surface | 091-103 | Read-only orchestration and machine output stabilize before mutations. |
| 10 | 09 Verification and mutation | 104-111 | Authored/generated mutability and dry-run boundaries are enforced. |
| 11 | 10 MCP | 112-116 | MCP follows stable JSON and denies execution by default. |
| 12 | 11 End-to-end fixtures | 117-122 | All examples and pilots traverse production paths. |
| 13 | 12 Hardening and packaging | 123-128 | Security, fuzz, scale, schema parity, and dry-run packaging gates are unsuppressed. |
| 14 | 13 Operations and closure | 129-134 | Final verification propagates every failure before the task closes. |

## Corrected Rules

- The public repository is the implementation authority; imported provenance
  is recorded without a dependency on an external workspace path.
- Domain types define semantics. Manifest source DTOs and public protocol DTOs
  remain separate adapters around them.
- Semantic hashing occurs after import resolution, fragment expansion, and all
  graph invariants.
- Authored metadata and generated state have disjoint mutation rules.
- Every positive example is validated by production code and every negative
  fixture asserts a stable diagnostic.
- The compatibility scanner has only narrow policy/negative-data allowances.
- The final aggregate gate contains no ignored status, fallback success, or
  failure suppression.

The authority validator extracts checkpoint-map rows and requires the exact
contiguous set 001-134 with no duplicate or missing checkpoint.
