# ADR 0012: Closed External Reference Grammar

Status: approved

## Context

Step 015 requires validated owner, issue, design, catalog, CI, and release
references. Imported examples establish owner and issue forms but the source
authority does not close every component grammar. Deferring those choices to
code would create inconsistent identities and accidental URL behavior.

## Decision

Use the exact non-network URI forms in the vocabulary specification. Each
reference kind has one scheme, fixed component count, bounded ASCII grammar,
and canonical spelling. References are opaque identifiers: parsing never
fetches, resolves, normalizes, redirects, or authenticates an external system.

## Consequences

Provider URLs and shorthand ticket strings are not accepted. Later adapters
may map validated references to provider data only through separate explicit
configuration and authority.

## Affected Original Steps

Step 015 implements the primitives. Manifest conversion, diagnostics, public
protocols, and trust checks consume them without adding alternate forms.

## Acceptance Evidence

- Every approved reference kind round-trips in canonical form.
- Wrong schemes, component counts, case, bounds, queries, and fragments fail.
- Tests prove parsing performs no I/O or provider lookup.
