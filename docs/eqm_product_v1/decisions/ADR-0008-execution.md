# ADR-0008: Authorize exact plans and supervise complete workers

**Status:** Accepted. **Date:** 2026-09-26. **Requirement links:** R30,R31,R32,R33,R34,R42.

## Context

The existing EQM repository has useful typed/pure boundaries but incomplete orchestration. The user's product spans Rust CLI and KMP desktop/mobile roles, with independently produced conformance evidence. The approved pressure test requires accurate claims and explicit safety boundaries rather than architectural theater.

## Decision

Use independent host capability checks, concrete plan approval, confined bounded handles, immutable sources, isolated outputs and whole-invocation cancellation/deadline/overflow ownership.

## Consequences and rejected shortcuts

Direct argv/process groups are not hostile-code isolation. Unsupported host guarantees reject instead of falling back.

## Verification and implementation

Use the matching requirements in implementation/TRACEABILITY.md to find acceptance cases and ordered commits. The choice is approved as intent, not verified implementation. Record actual public API/host/dependency decisions in a follow-on implementation ADR when the plan calls for discovery; owner-controlled values in OPEN_QUESTIONS.md remain unresolved.
