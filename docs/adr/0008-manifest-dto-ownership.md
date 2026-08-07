# ADR 0008: Manifest DTO Ownership

Status: approved

## Context

Authored TOML syntax needs source spans, optional fields, and parse-oriented
representations that do not belong in semantic domain types or public result
DTOs. Ambiguous ownership would create circular dependencies or leak source
layout into graph identity.

## Decision

The manifest crate owns authored/imported document DTOs, parsing, schema-facing
validation, source discovery, and conversion into domain builders. The domain
crate owns only validated semantic types. The protocol crate owns only public
command and result DTOs.

## Consequences

Manifest DTOs may preserve source detail without affecting canonical semantic
identity. Conversion is an explicit validation boundary, and neither domain nor
protocol code depends on manifest DTOs.

## Affected Original Steps

Manifest and schema work in Steps 032-049 must use manifest-owned source DTOs;
domain Steps 009-031 and protocol Steps 022-031 retain their existing crate
boundaries.

## Acceptance Evidence

- The workspace dependency graph contains no manifest-to-protocol or
  domain-to-manifest dependency.
- Parser tests distinguish source DTO validation from semantic graph validation.
- Public result schemas expose no parser-only fields or source-layout identity.
