# ADR-0010: Optimize only against a reference evaluator

**Status:** Accepted. **Date:** 2026-09-26. **Requirement links:** R39,R40,R41,R43,R46.

## Context

The existing EQM repository has useful typed/pure boundaries but incomplete orchestration. The user's product spans Rust CLI and KMP desktop/mobile roles, with independently produced conformance evidence. The approved pressure test requires accurate claims and explicit safety boundaries rather than architectural theater.

## Decision

Use component identities, staged caches, reverse dependencies, batching and bounded snapshot queries with reference equivalence and representative measured budgets.

## Consequences and rejected shortcuts

No cache may extend time/trust acceptance; no benchmark of placeholder context proves production performance.

## Verification and implementation

Use the matching requirements in implementation/TRACEABILITY.md to find acceptance cases and ordered commits. The choice is approved as intent, not verified implementation. Record actual public API/host/dependency decisions in a follow-on implementation ADR when the plan calls for discovery; owner-controlled values in OPEN_QUESTIONS.md remain unresolved.
