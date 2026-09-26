# ADR-0003: Use raw semantic and execution identities with campaigns

**Status:** Accepted. **Date:** 2026-09-26. **Requirement links:** R16,R17,R18,R19.

## Context

The existing EQM repository has useful typed/pure boundaries but incomplete orchestration. The user's product spans Rust CLI and KMP desktop/mobile roles, with independently produced conformance evidence. The approved pressure test requires accurate claims and explicit safety boundaries rather than architectural theater.

## Decision

Deduplicate transport without count inflation, preserve contradictory attempts, expected selections and immutable within-campaign history, and select campaigns by explicit authority.

## Consequences and rejected shortcuts

Reject record-count thresholds, timestamp-only latest-pass selection, and permanent poisoning by irrelevant old campaigns.

## Verification and implementation

Use the matching requirements in implementation/TRACEABILITY.md to find acceptance cases and ordered commits. The choice is approved as intent, not verified implementation. Record actual public API/host/dependency decisions in a follow-on implementation ADR when the plan calls for discovery; owner-controlled values in OPEN_QUESTIONS.md remain unresolved.
