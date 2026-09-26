# ADR-0002: Separate source ownership comparability and exact subjects

**Status:** Accepted. **Date:** 2026-09-26. **Requirement links:** R07,R08,R09,R13.

## Context

The existing EQM repository has useful typed/pure boundaries but incomplete orchestration. The user's product spans Rust CLI and KMP desktop/mobile roles, with independently produced conformance evidence. The approved pressure test requires accurate claims and explicit safety boundaries rather than architectural theater.

## Decision

Roles, outputs, shared components, occurrence-based scenario participants, exact subject manifests and shared comparison context are distinct. Cohorts express approved independent release cadence.

## Consequences and rejected shortcuts

A single target-root model and identical-build comparison cannot represent the approved KMP/asymmetric product accurately.

## Verification and implementation

Use the matching requirements in implementation/TRACEABILITY.md to find acceptance cases and ordered commits. The choice is approved as intent, not verified implementation. Record actual public API/host/dependency decisions in a follow-on implementation ADR when the plan calls for discovery; owner-controlled values in OPEN_QUESTIONS.md remain unresolved.
