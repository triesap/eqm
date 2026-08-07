# Finalized Graph Canonicalization

Status: normative

This contract defines semantic identity for an EQM v1 workspace. No authored
file, partially resolved DTO, or unexpanded graph has a workspace semantic
digest. Implementations must produce the exact projection, bytes, and digest
defined here.

## Required Pipeline

The stages execute in this order and stop on any diagnostic error:

1. decode UTF-8 and parse TOML 1.1;
2. validate the exact current schema and closed source DTO;
3. normalize text, identifiers, paths, and collections;
4. apply documented defaults;
5. convert source DTOs into domain builders;
6. load exact lock entries and pinned imported authority;
7. resolve every typed reference and reject duplicate authority;
8. expand exact fragment uses;
9. validate graph invariants and policy monotonicity;
10. freeze an immutable `WorkspaceGraph`;
11. construct the semantic projection below;
12. serialize it with RFC 8785 JSON Canonicalization Scheme (JCS);
13. hash the domain label, zero delimiter, and exact JCS bytes with SHA-256.

Stages 11-13 accept only the finalized graph type. The API must make it
impossible to canonicalize a source DTO or unresolved builder by accident.

## Digest Domain

The digest input is the following byte concatenation with no newline:

```text
UTF8("eqm:v1:semantic-graph") || 0x00 || JCS_BYTES
```

SHA-256 is the only v1 algorithm. The wire value is `sha256:` followed by 64
lowercase hexadecimal digits. Algorithm labels, uppercase hex, base64, a
missing domain label, or a missing zero delimiter are not equivalent inputs.

## Root Projection

The projection is one JSON object with these exact keys. Every array and the
root `extensions` object is present even when empty.

| Key | Value | Array order |
| --- | --- | --- |
| `schema` | exact `https://schemas.equivalencematrix.dev/v1/semantic-graph` | n/a |
| `capabilities` | capability projections | `id` |
| `journeys` | journey projections | `id` |
| `surfaces` | surface projections | `id` |
| `fragments` | fragment projections | `(id, revision)` |
| `targets` | target projections | `id` |
| `bindings` | binding projections | `(target, unit, id)` |
| `policies` | policy projections | `(id, revision)` |
| `profiles` | profile projections | `(id, revision)` |
| `runners` | runner projections | `(id, revision)` |
| `waivers` | waiver projections | `(id, revision)` |
| `imports` | resolved import-lock projections | `(id, revision, digest)` |
| `adapters` | resolved adapter-lock projections | `(id, version, digest)` |
| `extensions` | normative workspace extensions | object keys sort through JCS |

Sorting compares normalized Unicode scalar values by their UTF-8 byte
sequence. IDs and digests are ASCII, so their ordering is bytewise ascending.
Tuple sorting compares each component in order.

## Entity Projection

After defaults and fragment expansion, each object retains the following exact
fields. An optional field without a default is omitted when absent. A field
with a default is always present in the projection. JSON `null` is never used.

| Object | Always-present fields | Optional fields |
| --- | --- | --- |
| capability | `id`, `title`, `status`, `owners`, `extensions` | `description` |
| journey | `id`, `revision`, `title`, `capability`, `status`, `risk_class`, `owners`, `surfaces`, `transitions`, `extensions` | `description` |
| surface | `id`, `revision`, `title`, `journey`, `status`, `owners`, `requirements`, `fragment_origins`, `extensions` | `description` |
| fragment | `id`, `revision`, `title`, `risk_class`, `owners`, `requirements`, `extensions` | `description` |
| target | `id`, `root`, `platform`, `framework`, `owners`, `extensions` | none |
| binding | `id`, `revision`, `target`, `unit`, `owners`, `artifacts`, `exposures`, `evidence`, `extensions` | none |
| policy | `id`, `revision`, `title`, `owners`, `profiles`, `required_targets`, `rules`, `waivers`, `extensions` | `description` |
| profile | `id`, `revision`, `title`, `owners`, `dimensions`, `defaults`, `extensions` | `description` |
| runner | `id`, `revision`, `owners`, `backend`, `program`, `args`, `cwd`, `environment`, `secrets`, `timeout_ms`, `max_output_bytes`, `max_concurrency`, `guarantees`, `extensions` | none |
| waiver | `id`, `revision`, `owners`, `policy`, `scope`, `reason`, `issue`, `approvers`, `starts_on`, `expires_on`, `controls`, `extensions` | none |
| import lock | `id`, `revision`, `source`, `resolved`, `digest`, `trust` | `signature` |
| adapter lock | `id`, `version`, `source`, `resolved`, `digest`, `protocol`, `trust` | `signature` |

`fragment_origins` records the exact `fragment`, `revision`, `digest`, and
optional `prefix` for each expanded use. The surface `requirements` array
contains both direct and expanded requirements under their final IDs. This
retains import identity while ensuring the digest covers effective meaning.

## Nested Projection

Nested objects retain all semantically valid fields from the authored manifest
contract after defaults. Their exact key sets are:

| Object | Fields |
| --- | --- |
| requirement | `id`, `level`, `scope`, `statement`, `facets`, `applicability`, `extensions`; optional `risk_class`, `provider` |
| transition | `from`, `to`, `trigger` |
| fragment origin | `fragment`, `revision`, `digest`; optional `prefix` |
| artifact | `id`, `role`, `path`, `extensions`; optional `surface`, `symbol`, `selector` |
| exposure | `surface`, `state`, `applicability`, `extensions`; optional `route` |
| evidence specification | `id`, `kind`, `requirements`, `facets`, `extensions`; kind-dependent optional `runner`, `selector`, `minimum_count`, `freshness` |
| policy rule | `selector`, `minimum_level`, `facets`, `minimum_trust`, `maximum_age`, `minimum_count` |
| waiver policy | `allowed`, `minimum_approvers`, `required_controls`; `maximum_days` only when allowed |
| dimension | `id`, `values`; optional `description` |
| environment binding | `name`, `source`; optional `value` |
| secret binding | `name`, `provider` |
| waiver scope | `target`, `unit`, `requirement`, `facets`, `profiles` |
| applicability | exactly the discriminated keys defined by the manifest contract |
| selector | exactly the discriminated keys defined by the vocabulary contract |
| extension subtree | all validated normative extension keys and values |

Durations project as integer milliseconds. Calendar dates and UTC instants
project as their canonical strings. Digests include the lowercase `sha256:`
prefix. Repository paths use `/` separators. Enums and IDs use their exact wire
strings.

## Collection Ordering

| Collection | Semantics and canonical order |
| --- | --- |
| journey `surfaces` | authored order is normative and retained |
| runner `args` | authored argv order is normative and retained |
| requirements | full final requirement ID |
| fragment origins | `(fragment, revision, digest, prefix-or-empty)` |
| transitions | `(from, to, trigger)` |
| artifacts | `id` |
| exposures | `(surface, state, canonical-applicability-bytes, route-or-empty)` |
| evidence specifications | `id` |
| policy rules | canonical selector bytes, then remaining canonical rule bytes |
| dimensions | `id` |
| environment and secret bindings | `name`, then source/provider |
| imports and adapters | root ordering table |
| owners, approvers, facets, IDs, guarantees, controls, profiles, targets, values | sorted unique wire strings |
| applicability `all` and `any` | sorted by canonical child bytes after duplicate rejection |

Objects such as `defaults` and `extensions` use JCS object-key ordering. Arrays
not listed here are invalid until assigned an ordering rule by an ADR.

## Normative And Excluded Inputs

The following table is exhaustive for repository inputs.

| Input | Participation |
| --- | --- |
| finalized entities and nested semantic fields listed above | included |
| documented defaults | included as explicit values |
| validated extensions except the display namespace | included |
| resolved immutable import and adapter lock identity | included |
| workspace target records and normative workspace extensions | included |
| entity schema URI | excluded after exact dispatch; entity class is encoded by its root array |
| workspace source globs, config path, lockfile path, generated root | excluded discovery configuration |
| source file path and source ordering | excluded |
| TOML formatting, comments, quote style, key order, and line endings | excluded |
| source and related diagnostic spans | excluded |
| `dev.equivalencematrix.display` extension subtree | excluded in full |
| generated evidence, inventories, facts, reports, caches, logs, and temporary state | excluded |
| wall clock, current user, environment, locale, terminal, and current directory | excluded |

An excluded value may affect diagnostics or presentation but cannot change the
semantic digest. A value that affects obligations, evaluation, execution,
trust, or subject identity cannot be placed in an excluded field.

## JCS Rules

Serialization follows RFC 8785 without a custom JSON formatter:

- object keys are ordered by the RFC's UTF-16 code-unit comparison;
- strings use the RFC escaping rules and valid Unicode only;
- booleans and integers use JSON literals;
- floats are impossible because EQM normative types reject them;
- arrays preserve the already-defined semantic order;
- no insignificant whitespace, byte-order mark, or trailing newline is emitted.

NFC normalization occurs before projection. JCS does not perform Unicode
normalization and therefore cannot replace the earlier normalization stage.

## Fixed Independent Vectors

Vector fixtures must be implemented independently of the production projector.
The expected bytes below are one physical UTF-8 line with no trailing newline.

### Vector 1: empty finalized graph

Expected JCS bytes (253 bytes):

```json
{"adapters":[],"bindings":[],"capabilities":[],"extensions":{},"fragments":[],"imports":[],"journeys":[],"policies":[],"profiles":[],"runners":[],"schema":"https://schemas.equivalencematrix.dev/v1/semantic-graph","surfaces":[],"targets":[],"waivers":[]}
```

Expected digest:

```text
sha256:2323afb42c366664f47a5f90c597c7968f651f74f875ed95aec4dcc02283994c
```

### Vector 2: capability and target

Expected JCS bytes (523 bytes):

```json
{"adapters":[],"bindings":[],"capabilities":[{"description":"Create an account","extensions":{},"id":"account.create","owners":["owner://team/accounts"],"status":"active","title":"Account creation"}],"extensions":{},"fragments":[],"imports":[],"journeys":[],"policies":[],"profiles":[],"runners":[],"schema":"https://schemas.equivalencematrix.dev/v1/semantic-graph","surfaces":[],"targets":[{"extensions":{},"framework":"sveltekit","id":"web","owners":["owner://team/web"],"platform":"web","root":"apps/web"}],"waivers":[]}
```

Expected digest:

```text
sha256:a22165d85e6f4d5ee0891f17da7116d8eb497122d06893bca9a95e0241e7ebc7
```

The vector suite must additionally construct each graph from at least two TOML
layouts with different file names, key order, comments, set order, and line
endings and prove identical bytes. It must then mutate every included field in
turn and prove a changed digest, and mutate every excluded input in turn and
prove the digest remains unchanged.
