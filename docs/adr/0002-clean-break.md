# ADR 0002: Clean Break Without Compatibility

Status: approved

## Decision

Accept only current EquivalenceMatrix v1 identities, schemas, manifests, and
protocols. Do not implement aliases, shims, migrations, deprecated fields,
fallbacks, or old-name readers.

## Consequence

Downstream breaking changes are intentional. Current-only validation fails
closed and the repository scanner enforces the boundary.
