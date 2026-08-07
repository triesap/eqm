# ADR 0013: Closed Evidence Result Shapes

Status: approved

## Context

The evidence-result protocol requires an exact subject, producer, and
kind-discriminated payload, but the initial field table did not close their
nested shapes. Opaque JSON or free-form producer strings would make replay
binding, canonicalization, and kind validation implementation-dependent.

## Decision

An evidence subject has exactly `repository`, `repository_id_digest`, `scope`,
`source_commit`, `build_id`, `artifact_digest`, and
`target_configuration_digest`. `repository` is an absolute canonical HTTPS URI
without user information, query, fragment, or trailing slash. `source_commit`
is 40 or 64 lowercase hexadecimal characters. `build_id` is optional bounded
normalized text. Optional values are represented by JSON null in this protocol
object. Scope is exactly one tagged form: `target`, `provider`, or a nonempty
sorted `target_set`.

Producer identity uses
`producer://<class>/<authority>/<identity>`. Class is one of `local`, `ci`,
`human`, `adapter`, `runtime`, or `release`; authority is a lowercase ID
segment; identity is a 1-128 byte ASCII token starting alphanumeric and then
using letters, digits, `.`, `_`, or `-`. It is opaque and performs no lookup.

Profile values are sorted objects containing `profile`, positive `revision`,
and a nonempty dimension-to-symbolic-value map. Duplicate profiles or
dimensions are invalid.

Payload is one closed variant matching `kind`:

| Evidence kind | Exact payload fields |
| --- | --- |
| `structural_check`, `test`, `snapshot` | `attempts`, `counts`, `started_at`, `finished_at` |
| `static_inventory` | `inventory_digest`, `counts` |
| `manual_review` | `outcome`, `reviewer`, optional `message` |
| `runtime_snapshot` | `runtime_facts_digest`, `counts` |
| `release_record` | `release_record_digest` |

Executable attempts are nonempty and numbered consecutively from one. Attempt
times and overall times are ordered. Counts satisfy `selected = passed + failed
+ skipped + filtered + quarantined`. Countable payloads require `selected > 0`.
Manual-review outcome is `passed` or `failed`; it is not a waiver. The result
ID is exactly the lowercase `sha256:` wire value of `result_digest`.

## Consequences

Evidence envelopes can be validated and canonicalized without executing a
runner or interpreting provider data. Retry history cannot be erased because
attempts are immutable members of the digest-covered payload.

## Affected Original Steps

Step 026 implements these values. Protocol DTO, canonicalization, trust,
freshness, and evaluation checkpoints consume the same closed shapes.

## Acceptance Evidence

- Subject, producer, profile, payload, count, and attempt invariants have
  positive and negative tests.
- Evidence kind and payload kind must match.
- Result ID and result digest equality is enforced at construction.
