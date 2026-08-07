# Vocabularies, Selectors, And Extensions

Status: normative

All wire values in this document are lowercase ASCII and case-sensitive.
Unknown values fail closed. A value is extensible only where this document
explicitly says so.

## Closed Vocabularies

| Vocabulary | Values | Ordering or rule |
| --- | --- | --- |
| requirement level | `optional`, `recommended`, `required` | listed weakest to strongest |
| requirement scope | `each_target`, `shared_provider`, `end_to_end` | unordered |
| risk class | `low`, `medium`, `high`, `critical` | listed lowest to highest |
| lifecycle status | `draft`, `active`, `deprecated`, `retired` | transitions only forward in this list |
| evidence kind | `structural_check`, `static_inventory`, `test`, `snapshot`, `manual_review`, `runtime_snapshot`, `release_record` | unordered; executable kinds are structural check, test, and snapshot |
| facet | `structure`, `reachability`, `behavior`, `accessibility`, `visual`, `analytics`, `runtime_exposure`, `release_presence` | unordered set |
| facet status | `satisfied`, `failed`, `missing`, `stale`, `unknown`, `unstable`, `waived`, `not_applicable` | evaluation truth table defines precedence |
| conformance | `conformant`, `conditionally_conformant`, `nonconformant` | success, conditional, failure |
| equivalence | `equivalent`, `conditionally_equivalent`, `not_equivalent`, `unknown` | explicit four-state result |
| intended exposure state | `required`, `prohibited` | a profile-relative intent |
| inventory completeness | `complete`, `partial`, `unknown` | only complete supports absence claims |
| runner backend | `local`, `container` | no implicit backend |
| runner guarantee | `network_denied`, `read_only_source`, `isolated_process`, `resource_limited` | may be claimed only when enforced |
| artifact role | `entrypoint`, `view`, `route`, `component`, `service`, `test`, `configuration`, `asset` | unordered |
| trust level | `untrusted_local`, `trusted_ci`, `signed_ci` | listed weakest to strongest |
| release channel | `development`, `internal`, `beta`, `production` | unordered identity, not maturity ordering |
| output format | `human`, `json`, `sarif` | command contract restricts availability |
| diagnostic severity | `error`, `warning`, `note` | listed highest to lowest |

Lifecycle transitions may skip forward but never move backward. `retired` is
terminal. A child entity cannot have a later lifecycle state than its parent:
an active surface under a deprecated journey is invalid. Requirement risk may
equal or raise inherited journey/fragment risk but cannot lower it.

## Extensible Identifier Vocabularies

The following are validated IDs rather than closed enums because repositories
must describe real implementation ecosystems without an EQM release:

| Vocabulary | Built-in value or examples | Constraint |
| --- | --- | --- |
| platform ID | `web`, `ios`, `android`, `macos`, `windows`, `linux`, `service` | ID grammar; organization additions use a namespaced ID |
| framework ID | `none` and framework-specific IDs | ID grammar; organization additions use a namespaced ID |
| provider ID | no implicit built-ins | fully qualified ID |
| profile dimension ID | no implicit built-ins | local ID within one profile family |
| symbolic dimension value | no implicit built-ins | local ID within its dimension |
| adapter ID | no implicit built-ins | fully qualified ID and exact lock entry |
| runner ID | no implicit built-ins | fully qualified ID and authored runner authority |

Namespaced organization additions have at least two ID segments. An extensible
identifier never changes parser behavior by itself; behavior comes from typed
authority such as a profile, runner, or exact adapter lock.

## Identifier Types

All IDs use the base grammar in the data-model authority: lowercase ASCII
segments separated by dots, segment length at most 63, total length at most
255. Specific types add these rules:

| Type | Additional rule |
| --- | --- |
| capability ID | at least two segments |
| journey ID | capability ID plus one or more segments |
| surface ID | journey ID plus one or more segments |
| fragment, policy, profile, runner, waiver, binding ID | at least two segments |
| local requirement ID | exactly one segment |
| full requirement ID | surface or fragment ID plus `#` plus one local requirement ID; maximum 320 characters |
| artifact and evidence-spec ID | exactly one segment within a binding |
| target ID | one or more segments; unique in the workspace config |

References are always fully qualified except fields explicitly typed as local
IDs. There is no relative lookup, namespace search, alias, or redirect.

## Selector Contract

Selectors are data, never source code, regular expressions, shell fragments,
or provider query languages.

### Policy selectors

A policy selector is a table containing at least one and any combination of:

| Field | Type | Match rule |
| --- | --- | --- |
| `units` | nonempty unit-ID set | exact identity |
| `requirements` | nonempty full-requirement-ID set | exact identity |
| `risk_classes` | nonempty risk-class set | exact closed value |
| `facets` | nonempty facet set | set intersection |
| `scopes` | nonempty requirement-scope set | exact closed value |

Fields combine with logical AND; values within a field combine with logical OR.
Selectors are normalized by field name and sorted unique values. A selector
that matches nothing is valid but produces a warning; it grants no exemption.

### Artifact selectors

An artifact selector is exactly one tagged table:

| Kind | Required fields | Optional fields |
| --- | --- | --- |
| `symbol` | `kind = "symbol"`, `name` | `language` |
| `route` | `kind = "route"`, `path` | `method` |
| `test` | `kind = "test"`, `framework`, `test_id` | `suite` |
| `inventory` | `kind = "inventory"`, `record_type`, `key` | `value` |

All strings are normalized bounded text. `path` is a provider-neutral route,
not a filesystem path. `method`, when present, is one of `get`, `post`, `put`,
`patch`, `delete`, or `options`.

### Evidence selectors

Evidence selectors use the same tagged shapes as artifact selectors plus:

| Kind | Required fields | Optional fields |
| --- | --- | --- |
| `snapshot` | `kind = "snapshot"`, `snapshot_id` | `variant` |
| `release` | `kind = "release"`, `channel` | none |

The evidence kind restricts selector kinds: tests use `test`, snapshots use
`snapshot`, structural checks use `symbol`, `route`, or `inventory`, static
inventories use `inventory`, and release records use `release`. Manual review
has no selector. Runtime snapshots use `inventory` with provider-neutral keys.

### Applicability operators

| Operator | Operand | Result |
| --- | --- | --- |
| `eq` | one declared value | true when dimension equals it |
| `ne` | one declared value | true when dimension differs |
| `in` | nonempty declared value set | true when dimension is in the set |
| `not_in` | nonempty declared value set | true when dimension is outside the set |

An undeclared dimension or unavailable context value yields unknown. An
undeclared comparison value is a manifest error. There are no numeric,
substring, pattern, ordering, or provider-specific operators in v1.

## Extension Namespace

`extensions` is the only extensibility point in authored documents and nested
records. Each top-level key is a reverse-domain namespace followed by a local
name, for example `dev.example.audit`. The grammar is:

```text
^[a-z][a-z0-9]*(\.[a-z][a-z0-9_]*){2,}$
```

Each namespace value is a JSON-compatible TOML subtree containing strings,
integers, booleans, arrays, and tables. Floats, date/time literals, byte data,
null-like sentinels, and keys outside the base ID-segment grammar are invalid.
Nesting depth is at most 16, total nodes at most 1,024 per document, strings at
most 16 KiB, and serialized extension data at most 256 KiB per document.

Extensions are normative by default and enter the canonical projection under
their sorted namespace key. The only digest-excluded namespace is
`dev.equivalencematrix.display`; it may contain presentation hints only and is
removed in full before canonicalization. It cannot contain IDs, selectors,
policy, trust, evidence, path, runner, exposure, or release meaning. Unknown
namespaces remain valid data but have no built-in behavior. A plugin or adapter
cannot reinterpret an unknown extension to weaken core validation.

## Text And Collection Normalization

- Authored text must be valid UTF-8 and is normalized to Unicode NFC.
- Line endings normalize to LF for text values; trailing spaces remain
  meaningful inside multiline strings.
- Machine identifiers, schema URIs, digests, algorithms, and enum values are
  ASCII and are never case folded.
- Set-valued arrays reject duplicates after normalization and canonicalize in
  ascending Unicode-code-point order of their wire representation.
- Ordered arrays are explicitly identified in the manifest contract; all
  other arrays are sets or sort by a documented tuple.
- TOML integer values must fit the target bounded type. Floats are rejected in
  every authored EQM document.
