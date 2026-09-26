# ADR-0009: Version identities explicitly and preserve signed originals

**Status:** Accepted principle; exact transition pending Q03. **Date:** 2026-09-26. **Requirement links:** R21,R52.

## Context

The existing EQM repository has useful typed/pure boundaries but incomplete orchestration. The user's product spans Rust CLI and KMP desktop/mobile roles, with independently produced conformance evidence. The approved pressure test requires accurate claims and explicit safety boundaries rather than architectural theater.

## Decision

Keep wire/evaluator/ABI/database version axes independent. Generate schemas and explicitly decide any current-v1 transition before exposing incompatible semantics.

## Consequences and rejected shortcuts

Do not add implicit legacy readers/aliases or rewrite signed bytes. Product-spec v1 is not the wire namespace version.

## Verification and implementation

Use the matching requirements in implementation/TRACEABILITY.md to find acceptance cases and ordered commits. The choice is approved as intent, not verified implementation. Record actual public API/host/dependency decisions in a follow-on implementation ADR when the plan calls for discovery; owner-controlled values in OPEN_QUESTIONS.md remain unresolved.

## Revision 2 disposition

ADR-0012 records owner approval of an explicit new namespace for incompatible wire changes. [ADR-0013](ADR-0013-wire-namespace-and-representations.md) closes the exact namespace/URI spelling (`eqm:v2`, `schemas/v2/`) and the numeric/time/null/set/sequence/digest-domain representation vectors under S006; no evidenced external consumer conflicts with the monotonic successor. The original signed-byte and independent-version-axis decisions remain unchanged. Generator and URI-parity extension happens with the first v2 schema in the owning S013+ steps.
