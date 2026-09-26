# ADR-0011: Separate implementation readiness from operational authorization

**Status:** Accepted. **Date:** 2026-09-26. **Requirement links:** R53,R54,R55,R57,R62.

## Context

The existing EQM repository has useful typed/pure boundaries but incomplete orchestration. The user's product spans Rust CLI and KMP desktop/mobile roles, with independently produced conformance evidence. The approved pressure test requires accurate claims and explicit safety boundaries rather than architectural theater.

## Decision

Require known-good incremental commits, supported native/crypto tests, shadow comparison, independent review, exact activation and nonpermissive rollback.

## Consequences and rejected shortcuts

No artifact/spec approval, green build or signature alone authorizes deployment. Unknown live inputs remain gates.

## Verification and implementation

Use the matching requirements in implementation/TRACEABILITY.md to find acceptance cases and ordered commits. The choice is approved as intent, not verified implementation. Record actual public API/host/dependency decisions in a follow-on implementation ADR when the plan calls for discovery; owner-controlled values in OPEN_QUESTIONS.md remain unresolved.
