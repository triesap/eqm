# ADR-0004: Protect independent authority and use real verification

**Status:** Accepted. **Date:** 2026-09-26. **Requirement links:** R23,R24,R25,R26,R64.

## Context

The existing EQM repository has useful typed/pure boundaries but incomplete orchestration. The user's product spans Rust CLI and KMP desktop/mobile roles, with independently produced conformance evidence. The approved pressure test requires accurate claims and explicit safety boundaries rather than architectural theater.

## Decision

Protect expectations and grader independently of candidate; verify every mandatory input dimension with reviewed cryptography. Sigstore bundle integration is the default direction, not a granted live identity.

## Consequences and rejected shortcuts

A scalar trust claim, serialized verified flag or synthetic test signer is not production authority. Exact providers/credentials remain gated.

## Verification and implementation

Use the matching requirements in implementation/TRACEABILITY.md to find acceptance cases and ordered commits. The choice is approved as intent, not verified implementation. Record actual public API/host/dependency decisions in a follow-on implementation ADR when the plan calls for discovery; owner-controlled values in OPEN_QUESTIONS.md remain unresolved.
