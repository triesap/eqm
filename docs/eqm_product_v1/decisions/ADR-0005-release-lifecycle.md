# ADR-0005: Separate readiness signing authorization and deployment

**Status:** Accepted. **Date:** 2026-09-26. **Requirement links:** R20,R27,R28,R29,R63.

## Context

The existing EQM repository has useful typed/pure boundaries but incomplete orchestration. The user's product spans Rust CLI and KMP desktop/mobile roles, with independently produced conformance evidence. The approved pressure test requires accurate claims and explicit safety boundaries rather than architectural theater.

## Decision

Use acyclic records; distinguish pre-release from deployed observations; statements faithfully sign decisions; protected authorization and deployment consumption bind exact subjects and validity.

## Consequences and rejected shortcuts

Reject evidence-decision digest cycles, presence-as-exposure, signed-failure-as-pass, and same-commit artifact substitution.

## Verification and implementation

Use the matching requirements in implementation/TRACEABILITY.md to find acceptance cases and ordered commits. The choice is approved as intent, not verified implementation. Record actual public API/host/dependency decisions in a follow-on implementation ADR when the plan calls for discovery; owner-controlled values in OPEN_QUESTIONS.md remain unresolved.
