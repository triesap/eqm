# ADR 0011: Fixtures, Compatibility Scanning, And Final Verification

Status: approved

## Context

Examples that are not parsed by the production path drift into misleading
documentation. Compatibility scans with broad exclusions can miss executable
fallbacks. A final verification command that suppresses failures cannot serve
as release evidence.

## Decision

Every positive metadata example must parse and validate through production
code. Negative fixtures must assert a specific stable diagnostic. The
no-compatibility scanner permits only narrowly identified policy prose and
negative test data. Final verification runs all required gates without error
suppression and fails on any dirty generated or golden state.

## Consequences

Fixture validity, scanner behavior, and final verification are repository-owned
tests. Allowlist expansion requires review and cannot authorize runtime aliases
or fallback parsing.

## Affected Original Steps

Example and fixture work across Steps 006, 043-049, 120-126, and 131-134 must
use production validators. Step 134 is an unsuppressed aggregate release gate.

## Acceptance Evidence

- Positive examples pass the production parser and semantic validator.
- Negative fixtures fail with their expected diagnostic IDs.
- The scanner proves both a clean repository and a detected forbidden fixture.
- The aggregate verification command propagates every failing exit status.
