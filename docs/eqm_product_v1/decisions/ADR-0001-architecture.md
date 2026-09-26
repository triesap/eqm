# ADR-0001: Single Rust evaluator and shared application boundary

**Status:** Accepted. **Date:** 2026-09-26. **Requirement links:** R01,R04,R05,R06,R50.

## Context

The existing EQM repository has useful typed/pure boundaries but incomplete orchestration. The user's product spans Rust CLI and KMP desktop/mobile roles, with independently produced conformance evidence. The approved pressure test requires accurate claims and explicit safety boundaries rather than architectural theater.

## Decision

Keep the pure domain/engine and introduce the shared effectful application layer. Adapters render its complete immutable decisions; acquisition remains privileged and separate.

## Consequences and rejected shortcuts

Reject rewrite, parallel Kotlin evaluator, giant manager and microservice decomposition by default.

## Verification and implementation

Use the matching requirements in implementation/TRACEABILITY.md to find acceptance cases and ordered commits. The choice is approved as intent, not verified implementation. Record actual public API/host/dependency decisions in a follow-on implementation ADR when the plan calls for discovery; owner-controlled values in OPEN_QUESTIONS.md remain unresolved.
