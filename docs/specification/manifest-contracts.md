# Authored Manifest Contracts

Status: normative

This document defines the complete authored EQM v1 TOML surface. A field not
listed here is rejected, except for a validated `extensions` table. All files
are UTF-8 TOML 1.1, contain exactly one top-level document, and carry the exact
schema URI listed in the schema inventory.

In the tables below, `required` means the key must be authored. A default is
applied before graph finalization and canonicalization. `normative` means the
final value participates in semantic identity. All listed fields are normative
unless marked otherwise. Empty strings, empty identifiers, duplicate array
members, and duplicate semantic authorities are invalid.

## Shared Values

### Document metadata

| Field | Type | Presence/default | Meaning |
| --- | --- | --- | --- |
| `schema` | schema URI | required | Exact current schema identity. |
| `id` | typed ID | required except workspace and lock | Stable fully qualified authority ID. |
| `revision` | positive integer | required for versioned authority | Monotonically increasing authored revision; not a digest. |
| `title` | normalized text, 1-160 characters | required for product authority | Human-readable label. |
| `description` | normalized text, at most 4,096 characters | optional | Normative statement when present. |
| `owners` | nonempty owner-reference array | required for authored authority | Accountable owners, sorted and deduplicated semantically. |
| `extensions` | extension table | default `{}` | Namespaced semantic extension data. |

`title` and `description` are normative because they express approved product
intent. Source paths, comments, formatting, and diagnostic spans are not
fields and never participate in semantic identity.

### Requirement record

| Field | Type | Presence/default | Meaning |
| --- | --- | --- | --- |
| `id` | local requirement ID | required | Unique within its containing surface or fragment. |
| `level` | requirement level | required | Required, recommended, or optional. |
| `scope` | requirement scope | required | Per-target, shared-provider, or end-to-end obligation scope. |
| `statement` | normalized text, 1-4,096 characters | required | One atomic user-observable assertion. |
| `facets` | nonempty facet set | required | Independently evaluated aspects of the assertion. |
| `applicability` | applicability expression | default `{ always = true }` | Finite symbolic condition under which the requirement applies. |
| `risk_class` | risk class | optional | Raises the inherited risk; it may not lower it. |
| `provider` | provider ID | required only for `shared_provider` | Exact shared-provider authority. Forbidden for other scopes. |
| `extensions` | extension table | default `{}` | Namespaced semantic extension data. |

### Applicability expression

An expression is exactly one of the following tagged tables; nested operands
use the same type. Depth is at most 16 and total nodes at most 256.

| Form | Fields | Rule |
| --- | --- | --- |
| constant | `always: boolean` | No other field is allowed. |
| comparison | `dimension`, `operator`, `value` | Operator must accept one value. |
| membership | `dimension`, `operator`, `values` | Operator must accept a nonempty deduplicated value set. |
| conjunction | `all` | Nonempty array of expressions. |
| disjunction | `any` | Nonempty array of expressions. |
| negation | `not` | One expression; double negation is valid but canonicalized structurally. |

Dimensions must be declared by the selected profile family. Values are typed
symbolic values, not executable expressions. Missing dimensions evaluate to
unknown, never false-by-default.

### Fragment use

| Field | Type | Presence/default | Meaning |
| --- | --- | --- | --- |
| `fragment` | fragment ID | required | Imported fragment authority. |
| `revision` | positive integer | required | Exact fragment revision. |
| `digest` | SHA-256 digest | required | Exact normative fragment digest. |
| `prefix` | local ID segment | optional | Prefix applied to imported local requirement IDs. |

## Workspace Config

The repository contains one `eqm.toml` unless `--config` identifies one exact
file. It is configuration, not a graph entity, and has no `id` or `revision`.

| Field | Type | Presence/default | Meaning |
| --- | --- | --- | --- |
| `schema` | workspace schema URI | required | Exact current workspace schema. |
| `contract_sources` | glob array | required | Capability, journey, surface, and fragment sources. |
| `binding_sources` | glob array | required | Binding sources. |
| `policy_sources` | glob array | required | Policy sources. |
| `profile_sources` | glob array | required | Profile sources. |
| `runner_sources` | glob array | required | Runner sources. |
| `waiver_sources` | glob array | required | Waiver sources. |
| `lockfile` | repository path | default `eqm.lock` | Exact import and adapter lock. |
| `generated_root` | repository path | default `.eqm` | Generated-state root; must equal `.eqm` in v1. |
| `targets` | table keyed by target ID | default empty | Target authority records. |
| `extensions` | extension table | default `{}` | Namespaced semantic workspace extensions. |

Each source array is nonempty, uses `/` separators, stays within the VCS root,
and is sorted after lexical normalization. A file may match only one source
class. Generated state is excluded even if a glob would otherwise match it.

### Target record

| Field | Type | Presence/default | Meaning |
| --- | --- | --- | --- |
| `root` | repository path | required | Target source root. |
| `platform` | platform ID | required | Declared implementation platform. |
| `framework` | framework ID | required | Declared framework or `none`. |
| `owners` | nonempty owner-reference array | required | Accountable target owners. |
| `extensions` | extension table | default `{}` | Namespaced semantic target extensions. |

## Contract Documents

### Capability

| Field | Type | Presence/default |
| --- | --- | --- |
| `schema`, `id`, `title`, `owners`, `extensions` | shared metadata | as defined above |
| `status` | lifecycle status | required |
| `description` | normalized text | optional |

A capability ID is the authority prefix for its journeys.

### Journey

| Field | Type | Presence/default |
| --- | --- | --- |
| `schema`, `id`, `revision`, `title`, `owners`, `extensions` | shared metadata | as defined above |
| `capability` | capability ID | required |
| `status` | lifecycle status | required |
| `risk_class` | risk class | required |
| `surfaces` | nonempty ordered surface-ID array | required |
| `transitions` | transition array | default empty |
| `description` | normalized text | optional |

A transition has required `from`, `to`, and `trigger` fields. `from` and `to`
are members of `surfaces`; `trigger` is normalized text of at most 256
characters. Duplicate `(from, to, trigger)` tuples are invalid. Array order is
normative for `surfaces` and non-normative for `transitions`, which sort by the
tuple above.

### Surface

| Field | Type | Presence/default |
| --- | --- | --- |
| `schema`, `id`, `revision`, `title`, `owners`, `extensions` | shared metadata | as defined above |
| `journey` | journey ID | required |
| `status` | lifecycle status | required |
| `requirements` | requirement array | default empty |
| `fragments` | fragment-use array | default empty |
| `description` | normalized text | optional |

At least one direct requirement or fragment use is required. A finalized
surface cannot contain duplicate full requirement IDs after fragment expansion.

### Fragment

| Field | Type | Presence/default |
| --- | --- | --- |
| `schema`, `id`, `revision`, `title`, `owners`, `extensions` | shared metadata | as defined above |
| `risk_class` | risk class | required |
| `requirements` | nonempty requirement array | required |
| `description` | normalized text | optional |

Fragments cannot use other fragments in v1. A fragment reference is valid only
when ID, revision, and computed semantic digest all match.

## Binding Document

| Field | Type | Presence/default |
| --- | --- | --- |
| `schema`, `id`, `revision`, `owners`, `extensions` | shared metadata | as defined above |
| `target` | target ID | required |
| `unit` | capability, journey, or surface ID | required |
| `artifacts` | nonempty artifact array | required |
| `exposures` | exposure array | default empty |
| `evidence` | evidence-specification array | default empty |

The pair `(target, unit)` has exactly one binding authority. Bindings contain
no executable command, shell fragment, or untyped provider rule.

### Artifact record

| Field | Type | Presence/default |
| --- | --- | --- |
| `id` | artifact ID | required |
| `role` | artifact role | required |
| `path` | repository path | required |
| `surface` | surface ID | optional |
| `symbol` | normalized symbol, at most 512 characters | optional |
| `selector` | typed selector | optional |
| `extensions` | extension table | default `{}` |

At least one of `surface`, `symbol`, or `selector` is required for roles that
claim user-visible implementation coverage. Paths must fall under the bound
target root.

### Exposure record

| Field | Type | Presence/default |
| --- | --- | --- |
| `surface` | surface ID | required |
| `state` | intended exposure state | required |
| `applicability` | applicability expression | default `{ always = true }` |
| `route` | normalized route selector | optional |
| `extensions` | extension table | default `{}` |

### Evidence specification

| Field | Type | Presence/default |
| --- | --- | --- |
| `id` | evidence-spec ID | required |
| `kind` | evidence kind | required |
| `requirements` | nonempty full-requirement-ID array | required |
| `facets` | nonempty facet set | required |
| `runner` | runner ID | required for executable kinds; otherwise forbidden |
| `selector` | typed selector | required when the kind selects tests or inventory records |
| `minimum_count` | positive integer | default `1` for countable kinds; otherwise forbidden |
| `freshness` | bounded duration | optional policy override ceiling |
| `extensions` | extension table | default `{}` |

Evidence coverage must be a subset of the named requirements' facets. A
specification describes expected evidence; it is never itself evidence.

## Policy Document

| Field | Type | Presence/default |
| --- | --- | --- |
| `schema`, `id`, `revision`, `title`, `owners`, `extensions` | shared metadata | as defined above |
| `profiles` | nonempty profile-ID array | required |
| `required_targets` | nonempty target-ID array | required |
| `rules` | nonempty policy-rule array | required |
| `waivers` | waiver policy | default deny |
| `description` | normalized text | optional |

A policy rule contains required `selector`, `minimum_level`, `facets`,
`minimum_trust`, and `maximum_age`; optional `minimum_count` defaults to `1`.
The selector may choose unit IDs, requirement IDs, risk classes, and facets
using only the closed selector operations. Multiple matching rules compose by
taking the strongest value on every ordered axis and unioning set obligations.

Waiver policy contains `allowed` (default `false`), `maximum_days` (required
when allowed), `minimum_approvers` (default `1`), and `required_controls`
(default empty facet set). Candidate policy may strengthen but not weaken a
protected policy.

## Profile Document

| Field | Type | Presence/default |
| --- | --- | --- |
| `schema`, `id`, `revision`, `title`, `owners`, `extensions` | shared metadata | as defined above |
| `dimensions` | nonempty dimension array | required |
| `defaults` | table keyed by dimension ID | default empty |
| `description` | normalized text | optional |

A dimension contains `id`, `values`, and optional `description`. `values` is a
nonempty set of symbolic IDs. Every default must name one declared value. The
Cartesian product is bounded by the resource-limit contract; profiles describe
cohorts, never individual users or personal attributes.

## Runner Document

| Field | Type | Presence/default |
| --- | --- | --- |
| `schema`, `id`, `revision`, `owners`, `extensions` | shared metadata | as defined above |
| `backend` | runner backend | required |
| `program` | repository path or approved executable ID | required |
| `args` | argument-template array | required, may be empty |
| `cwd` | repository path template | default target root |
| `environment` | environment binding array | default empty |
| `secrets` | secret binding array | default empty |
| `timeout_ms` | bounded positive integer | required |
| `max_output_bytes` | bounded positive integer | required |
| `max_concurrency` | bounded positive integer | default `1` |
| `guarantees` | guarantee set | default empty |

Argument and path templates may contain only documented typed placeholders.
Environment bindings contain `name`, `source`, and optional `value`; literal
values are allowed only for non-secret bindings. Secret bindings contain
`name` and a secret-provider reference and are redacted from all output.
Runner guarantees must be a subset of guarantees enforced by the backend.

## Waiver Document

| Field | Type | Presence/default |
| --- | --- | --- |
| `schema`, `id`, `revision`, `owners`, `extensions` | shared metadata | as defined above |
| `policy` | policy ID | required |
| `scope` | waiver scope | required |
| `reason` | normalized text, 1-2,048 characters | required |
| `issue` | issue reference | required |
| `approvers` | nonempty owner-reference array | required |
| `starts_on` | calendar date | required |
| `expires_on` | calendar date | required |
| `controls` | facet set | default empty |

The scope contains exactly the target, unit, requirement, facets, and profiles
authorized by the waiver; wildcards are forbidden. `expires_on` must be after
`starts_on`. A waiver can produce conditional conformance only when protected
policy allows it; it never changes evidence status to satisfied.

## Lock Document

`eqm.lock` is generated by an explicit lock-update command and committed. It is
TOML but not discovered through source globs.

| Field | Type | Presence/default |
| --- | --- | --- |
| `schema` | lock schema URI | required |
| `version` | integer | required, exactly `1` |
| `imports` | import-lock array | default empty |
| `adapters` | adapter-lock array | default empty |

An import lock contains `id`, `revision`, `source`, `resolved`, `digest`, and
optional signature/trust metadata. An adapter lock contains `id`, `version`,
`source`, `resolved`, `digest`, `protocol`, and optional signature/trust
metadata. `resolved` is immutable. Floating branches, tags without immutable
resolution, absent digests, duplicate IDs, and ambient credentials in the file
are invalid.
