# ADR 0001: EquivalenceMatrix V1 Architecture

Status: approved

## Decision

Implement EquivalenceMatrix as a pure-Rust, local-first typed graph with
current-only TOML manifests, content-addressed evidence, policy-relative target
conformance, derived target-set equivalence, stable CLI/JSON contracts, and a
thin optional MCP adapter.

## Consequence

Crate boundaries and deterministic evaluation are hard architecture. A
deviation requires a new approved ADR.
