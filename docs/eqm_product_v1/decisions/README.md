# Architecture decision records

Accepted ADRs record decisions approved in the conversation. Pending values and implementation-specific choices remain explicit. Update status/supersession with evidence; do not overwrite history to justify an ad hoc shortcut.

- [ADR-0001: Single Rust evaluator and shared application boundary](ADR-0001-architecture.md) — Accepted.
- [ADR-0002: Separate source ownership comparability and exact subjects](ADR-0002-target-and-subject-model.md) — Accepted.
- [ADR-0003: Use raw semantic and execution identities with campaigns](ADR-0003-evidence-identity.md) — Accepted.
- [ADR-0004: Protect independent authority and use real verification](ADR-0004-protected-trust.md) — Accepted.
- [ADR-0005: Separate readiness signing authorization and deployment](ADR-0005-release-lifecycle.md) — Accepted.
- [ADR-0006: Immutable objects and a transactional local index](ADR-0006-storage.md) — Accepted direction; dependency/runtime choices pending Q10.
- [ADR-0007: Developer tooling first and optional native embedding](ADR-0007-runtime-integration.md) — Accepted.
- [ADR-0008: Authorize exact plans and supervise complete workers](ADR-0008-execution.md) — Accepted.
- [ADR-0009: Version identities explicitly and preserve signed originals](ADR-0009-compatibility.md) — Accepted principle; exact transition pending Q03.
- [ADR-0010: Optimize only against a reference evaluator](ADR-0010-incremental-performance.md) — Accepted.
- [ADR-0011: Separate implementation readiness from operational authorization](ADR-0011-rollout.md) — Accepted.

- [ADR-0012: Approved review closure and execution scope](ADR-0012-approved-review-closure.md) — Accepted; operational values and exact engineering representations remain explicit gates.
- [ADR-0013: Freeze the v2 wire namespace and canonical representations](ADR-0013-wire-namespace-and-representations.md) — Accepted; closes S006 namespace, numeric/time/null/set/digest representations under ADR-0012.
