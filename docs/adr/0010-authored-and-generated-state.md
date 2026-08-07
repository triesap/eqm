# ADR 0010: Authored And Generated State Mutability

Status: approved

## Context

Combining source authority with runtime evidence, caches, locks, or reports
would make repository review ambiguous and could allow generated data to alter
the meaning of an authored contract.

## Decision

Only the documented configuration and `eqm/` metadata trees are authored
authority. `.eqm/` is generated local state and is never an authority source.
Read-only commands must not mutate either tree. Mutating commands may change
only their explicitly documented targets, must support dry-run where specified,
and must use atomic replacement for authored data.

## Consequences

Generated evidence and caches are disposable and ignored by version control.
Commands cannot silently rewrite manifests, normalize files in place, or mix
runtime observations into semantic identity.

## Affected Original Steps

Manifest Steps 032-049, CLI mutation Steps 081-083, and operational Steps
131-134 must enforce the same mutability table.

## Acceptance Evidence

- Clean read-only integration tests leave the filesystem byte-identical.
- Dry-run reports intended changes without writing.
- Interrupted authored writes preserve the previous valid state.
