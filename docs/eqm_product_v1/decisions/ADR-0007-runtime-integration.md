# ADR-0007: Developer tooling first and optional native embedding

**Status:** Accepted. **Date:** 2026-09-26. **Requirement links:** R02,R03,R48,R49.

## Context

The existing EQM repository has useful typed/pure boundaries but incomplete orchestration. The user's product spans Rust CLI and KMP desktop/mobile roles, with independently produced conformance evidence. The approved pressure test requires accurate claims and explicit safety boundaries rather than architectural theater.

## Decision

Use Rust/KMP native evidence adapters for the product. Embed an evaluator or start a sidecar only for an explicitly approved local-evaluation feature with platform proof.

## Consequences and rejected shortcuts

JNA Kotlin is not automatically Kotlin/Native iOS; one verified strategy replaces multiple speculative bridges.

## Verification and implementation

Use the matching requirements in implementation/TRACEABILITY.md to find acceptance cases and ordered commits. The choice is approved as intent, not verified implementation. Record actual public API/host/dependency decisions in a follow-on implementation ADR when the plan calls for discovery; owner-controlled values in OPEN_QUESTIONS.md remain unresolved.
