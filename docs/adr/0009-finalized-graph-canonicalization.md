# ADR 0009: Canonicalize The Finalized Semantic Graph

Status: approved

## Context

Hashing individual source documents before imports, defaults, and references
are resolved would assign different identities to semantically equivalent
graphs and could omit authoritative imported meaning.

## Decision

Canonicalization operates only on the finalized validated semantic graph. The
projection exhaustively selects normative fields, sorts every unordered
collection by documented keys, emits RFC 8785 canonical JSON, and computes a
domain-separated SHA-256 digest. Source paths, spans, comments, formatting, and
generated runtime state do not participate.

## Consequences

Parsing and graph finalization must complete before semantic digests are
available. Cache keys and signatures bind the finalized projection rather than
authored bytes. Independent fixed vectors are required.

## Affected Original Steps

Canonicalization Steps 017-018 provide primitives, but their graph integration
occurs after manifest resolution and before evidence, cache, and attestation
features consume semantic identities.

## Acceptance Evidence

- Equivalent authored layouts produce identical canonical bytes and digests.
- Any normative semantic change changes the digest.
- Independent fixtures verify exact bytes and digest values.
