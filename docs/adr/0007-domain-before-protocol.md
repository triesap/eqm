# ADR 0007: Domain Checkpoints Precede Protocol Checkpoints

Status: approved

## Context

The source sequence placed shared protocol work before the domain graph was
fully defined. That ordering would force protocol DTOs to invent or duplicate
semantic rules and would weaken the required dependency direction.

## Decision

Implement diagnostics and primitive domain types first, then the complete
validated domain graph, and only then public protocol DTOs and schemas. The
protocol crate may translate domain results but may not define domain meaning.

## Consequences

Steps 019-021 execute as part of the domain-model wave before protocol-schema
work. Domain types remain the semantic source of truth, while serialization
concerns remain at the protocol boundary.

## Affected Original Steps

Steps 019, 020, and 021 move ahead of the protocol implementation that consumes
their types. RCLDs 01-03 encode the corrected dependency order.

## Acceptance Evidence

- Cargo dependencies remain acyclic and match the approved graph.
- Domain tests do not depend on protocol serialization.
- Protocol conversion tests demonstrate explicit domain-to-DTO mapping.
