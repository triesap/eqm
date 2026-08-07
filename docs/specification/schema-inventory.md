# Schema Inventory And Dispatch

Status: normative

EQM v1 accepts only the exact schema identities below. Schema dispatch occurs
before document-specific deserialization. URI normalization, redirects,
fragments, query strings, alternative hosts, omitted versions, and version
ranges are not accepted.

## Authored TOML Schemas

| Document | Exact schema URI | Source class | Authority key |
| --- | --- | --- | --- |
| workspace | `https://schemas.equivalencematrix.dev/v1/workspace` | explicit `eqm.toml` | singleton config |
| capability | `https://schemas.equivalencematrix.dev/v1/capability` | contract | `id` |
| journey | `https://schemas.equivalencematrix.dev/v1/journey` | contract | `id` |
| surface | `https://schemas.equivalencematrix.dev/v1/surface` | contract | `id` |
| fragment | `https://schemas.equivalencematrix.dev/v1/fragment` | contract | `id` plus revision/digest at use sites |
| binding | `https://schemas.equivalencematrix.dev/v1/binding` | binding | `(target, unit)`; `id` is stable record identity |
| policy | `https://schemas.equivalencematrix.dev/v1/policy` | policy | `id` |
| profile | `https://schemas.equivalencematrix.dev/v1/profile` | profile | `id` |
| runner | `https://schemas.equivalencematrix.dev/v1/runner` | runner | `id` |
| waiver | `https://schemas.equivalencematrix.dev/v1/waiver` | waiver | `id` |
| lock | `https://schemas.equivalencematrix.dev/v1/lock` | explicit `eqm.lock` | singleton lock |

Every schema is closed: JSON Schema object definitions set
`unevaluatedProperties: false`, unions are discriminated and exclusive, and
string/integer/array bounds mirror the repository specification. The Rust
manifest DTO validator remains authoritative when JSON Schema cannot express a
cross-reference or graph invariant.

## Public JSON Schemas

These identities are reserved as one coordinated v1 protocol set. Their exact
field contracts are defined by the protocol checkpoint before schema files are
implemented.

| Payload | Exact schema URI |
| --- | --- |
| result envelope | `https://schemas.equivalencematrix.dev/v1/result` |
| diagnostic | `https://schemas.equivalencematrix.dev/v1/diagnostic` |
| evidence result | `https://schemas.equivalencematrix.dev/v1/evidence-result` |
| inventory | `https://schemas.equivalencematrix.dev/v1/inventory` |
| runtime facts | `https://schemas.equivalencematrix.dev/v1/runtime-facts` |
| release record | `https://schemas.equivalencematrix.dev/v1/release-record` |
| attestation | `https://schemas.equivalencematrix.dev/v1/attestation` |
| adapter request | `https://schemas.equivalencematrix.dev/v1/adapter-request` |
| adapter response | `https://schemas.equivalencematrix.dev/v1/adapter-response` |
| MCP request | `https://schemas.equivalencematrix.dev/v1/mcp-request` |
| MCP response | `https://schemas.equivalencematrix.dev/v1/mcp-response` |

## Discovery And Duplicate Authority

1. Select the explicit config or locate exactly one `eqm.toml` inside the VCS
   root without following directory symlinks.
2. Normalize and sort each source pattern, then expand it lexically relative
   to the config directory.
3. Reject absolute paths, parent traversal, portable case collisions, symlink
   escapes, files outside the VCS root, generated-state paths, and a file
   matched by more than one source class.
4. Sort discovered files by normalized repository path.
5. Parse TOML syntax, extract `schema`, and dispatch only on the exact URI.
6. Validate the closed DTO, convert to typed domain input, and collect its
   authority key.
7. Reject duplicate IDs, duplicate binding `(target, unit)` authorities,
   duplicate target roots under portable comparison, and conflicting fragment
   revision/digest claims.

Discovery never uses current working directory order, filesystem enumeration
order, environment interpolation, network lookup, or implicit default source
patterns. Normal validation is offline.

## Unknowns And Version Coordination

- A missing `schema` is an error.
- An authored schema from another product, version, or document class is an
  error even if its fields appear compatible.
- An unknown field outside `extensions` is an error.
- An unknown closed-enum value is an error.
- An unknown extension namespace is retained as inert normative data.
- Imported manifests, adapters, evidence, and public result DTOs must all use
  the exact coordinated v1 identities expected by the consuming operation.
- A mixed-version graph or result set is rejected before evaluation.

Schema documents are generated or checked from the same field authority as the
Rust DTOs. A CI comparison must fail if checked-in schemas and source authority
diverge.
