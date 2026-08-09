# Concepts

## The semantic graph

EQM resolves authored documents into a typed, deterministic graph:

```text
capability
  -> journey
       -> surface
            -> requirement

fragment --exact semantic pin--> reusable requirements
binding -----------------------> target artifacts and evidence
policy + profile --------------> obligations
inventory + evidence ----------> observed state
```

A capability groups product value. A journey orders user-visible surfaces and
transitions. A surface owns requirements. A fragment reuses requirements only
through an exact ID, revision, and semantic digest. Expansion happens before
the final graph digest and must not collide with an existing identity.

IDs are semantic and stable. Paths are provenance and may change without
changing identity. Duplicate authority, dangling references, invalid lifecycle
relationships, risk weakening, and fragment pin mismatch fail closed.

## Targets and bindings

A target identifies one independently shipped implementation, such as
`android` or `ios`, with a repository-confined source root and declared
platform/framework metadata. A binding connects shared units to that target's
artifacts, expected exposure, and evidence selectors.

Artifacts remain native: a Compose screen and a SwiftUI view can satisfy the
same requirement. EQM evaluates the behavior described by the requirement; it
does not compare source syntax or demand shared code.

## Facets, scopes, and obligations

A requirement declares one or more facets such as behavior. Policy selects
requirements and states which facets are required, the evidence scope,
freshness, minimum count, and minimum trust. EQM expands this into exact
obligation coordinates.

Common scopes include one obligation per target and one end-to-end obligation
across an exact target set. An obligation includes policy, unit, requirement,
scope subject, facet, selected profile values, and evidence constraints. It is
therefore not safe to reuse evidence merely because two records have similar
names.

## Three-valued observations

Observed product facts distinguish true, false, and unknown. Absence is proven
only by complete trusted inventory; missing or partial inventory remains
unknown. Reconciliation compares authored expectations with observed facts
without rewriting either.

Conformance answers whether exact obligations are satisfied. Equivalence
compares required target outcomes only after its preconditions are met. An
extra target is visible but does not silently change the required target set.

## Determinism and canonicalization

All public identities use validated newtypes and closed vocabularies. Maps are
deterministic or explicitly sorted. Authored graph semantics are projected to
canonical JSON before semantic hashing. Formatting, source paths, comments,
and input order do not change a semantic digest; semantic changes do.

Public protocol records use exact schemas, closed fields, canonical digest
domains, and stable ordering. A digest is meaningful only with its declared
domain and subject; raw byte equality is not a substitute for semantic
identity.

## Crate boundaries

`eqm_domain` contains pure types and invariants. `eqm_manifest` owns TOML,
discovery, formatting, conversion, and canonical projection. `eqm_engine` owns
pure resolution and evaluation. `eqm_protocol` owns public JSON, SARIF,
adapter, evidence, attestation, and report DTOs. `eqm_runner` alone launches
processes. `eqm_mcp` adapts shared behavior to MCP. The `eqm` crate owns CLI
parsing, orchestration, and rendering.
