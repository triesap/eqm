# Manifests

EQM v1 uses strict TOML documents. Unknown fields, duplicate keys, invalid
Unicode normalization, old/future/foreign schemas, path escapes, portable path
collisions, and ambiguous discovery fail closed.

## Schema namespace

Manifest schema identities use:

```text
https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/WORKSPACE.schema.json
```

Replace `WORKSPACE` with `workspace`, `lock`, `capability`, `journey`,
`surface`, `fragment`, `binding`, `policy`, `profile`, `runner`, or `waiver`.
The matching committed JSON files are under `schemas/v1/manifest/`.

## Workspace

`eqm.toml` declares source discovery and target roots:

```toml
schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/workspace.schema.json"

contract_sources = ["eqm/contracts/**/*.toml"]
binding_sources = ["eqm/bindings/**/*.toml"]
policy_sources = ["eqm/policies/**/*.toml"]
profile_sources = ["eqm/profiles/**/*.toml"]
runner_sources = ["eqm/runners/**/*.toml"]
waiver_sources = ["eqm/waivers/**/*.toml"]

[targets.android]
root = "apps/android"
platform = "android"
framework = "compose"
owners = ["owner://team/android"]
```

Discovery is sorted, class-specific, repository-confined, excludes `.eqm` and
nested repositories, and rejects a document matched by multiple source
classes. Target roots must remain within the repository and be portably unique.

## Lock file

`eqm.lock` pins imports and adapters to exact versions and digests. Floating
versions such as `latest` are invalid. Normal validation does not update the
lock or acquire remote content. Review changes with `eqm lock update --dry-run`
before authorizing a write.

## Contracts

Contract files define capabilities, journeys, surfaces, or fragments. Every
entity has a typed ID, owners, lifecycle status, and kind-specific fields.
Surfaces own ordered requirements with stable local IDs, statements, levels,
facets, scopes, applicability, and optional extension values.

Fragments are pinned by semantic digest when composed. A fragment cannot
silently replace an existing requirement. Parent lifecycle and inherited risk
constrain children: a child cannot appear active beneath inactive authority or
weaken inherited risk.

## Bindings

Bindings select a target and map semantic units to repository-relative
artifacts and evidence selectors. Artifact roles and paths are typed; paths
must remain beneath the declared target root and cannot use unsafe symlink
resolution. Expected exposure is authored intent and remains separate from
observed inventory.

## Policies and profiles

Policies select nonempty requirement sets and derive evidence obligations. A
rule states facet, scope, minimum count, maximum age, and minimum trust. Policy
strength may be increased but baseline comparison rejects weakening.

Profiles declare finite dimensions and allowed values. Defaults may support
development, but nonlocal and release decisions require explicit authoritative
selection. Undeclared dimensions or values are errors even when a surrounding
boolean expression would otherwise short-circuit.

## Runners

Runners describe a local backend, executable, typed argument templates,
working directory, declared environment inputs, timeout, output limit, and
concurrency. Programs are executed directly as argv arrays, never through a
shell. Paths, placeholder values, environment, and output destinations are
validated before launch.

## Waivers

Waivers are authored exception authority with exact policy/requirement scope,
approvers, controls, reason, and ordered bounded dates. They must not be
created automatically. A valid waiver changes a visible blocker to conditional
status but never satisfies an evidence obligation.

For complete field shapes, read the committed schemas and the corresponding
files in `examples/android-ios/`.
