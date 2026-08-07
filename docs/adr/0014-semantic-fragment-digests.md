# ADR 0014: Semantic Fragment Digests

Status: accepted

## Context

Fragment uses require an exact digest, while the v1 contracts exclude TOML
layout, comments, quoting, source paths, and key order from semantic identity.
A raw source-file hash would make those excluded inputs alter whether a pin
resolves and would therefore contradict finalized graph canonicalization.

## Decision

A v1 fragment digest is SHA-256 over the domain label
`eqm:v1:fragment`, one zero byte, and the RFC 8785 JCS bytes of the fragment's
normative canonical projection. The projection is the fragment object defined
by the canonicalization contract after parsing, normalization, defaults, and
domain validation.

An optional fragment-use prefix forms each expanded local requirement ID as
`<prefix>_<local-id>`. Expansion fails before finalization if this result is not
a valid local requirement ID or collides with a direct or previously expanded
requirement. V1 fragments cannot contain fragment uses, so recursive fragment
cycles are rejected structurally by the closed schema.

## Consequences

- formatting-only edits do not invalidate fragment pins;
- every pin is exact by ID, revision, and semantic digest;
- fragment expansion cannot override authored requirements;
- canonical workspace hashing accepts only the finalized graph wrapper emitted
  after resolution, invariant checks, and successful expansion.
