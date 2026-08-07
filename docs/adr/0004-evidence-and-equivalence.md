# ADR 0004: Evidence, Conformance, And Equivalence

Status: approved

## Decision

Keep evidence specifications separate from immutable results. Evaluate target
conformance first, then derive target-set equivalence under one exact contract,
policy, profile, subject, trust, freshness, runtime, and release context.
Unknown never succeeds.

## Consequence

Waivers remain visible conditional debt and never transform evidence into a
pass.
