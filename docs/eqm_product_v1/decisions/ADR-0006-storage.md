# ADR-0006: Immutable objects and a transactional local index

**Status:** Accepted direction; dependency/runtime choices pending Q10. **Date:** 2026-09-26. **Requirement links:** R35,R36,R37,R38.

## Context

The existing EQM repository has useful typed/pure boundaries but incomplete orchestration. The user's product spans Rust CLI and KMP desktop/mobile roles, with independently produced conformance evidence. The approved pressure test requires accurate claims and explicit safety boundaries rather than architectural theater.

## Decision

Use object-first durable publication with a SQLite local index, explicit snapshots, recovery, leases, retention and independent historical/current checks.

## Consequences and rejected shortcuts

SQLite/WAL is not cross-device sync; linked runtime must contain the documented fix and pass current audit. Host durability and retention values need evidence.

## Verification and implementation

Use the matching requirements in implementation/TRACEABILITY.md to find acceptance cases and ordered commits. The choice is approved as intent, not verified implementation. Record actual public API/host/dependency decisions in a follow-on implementation ADR when the plan calls for discovery; owner-controlled values in OPEN_QUESTIONS.md remain unresolved.
