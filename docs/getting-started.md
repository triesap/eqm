# Getting started

## Prerequisites

Build or install the `eqm` binary and work in a Git repository containing the
application targets. EQM is local-first: ordinary validation and checking do
not require a hosted service.

## Bootstrap

Preview initialization before writing files:

```text
eqm init --dry-run
eqm init
```

Alternatively, copy `examples/android-ios/` into a new repository root. Keep
its `eqm.toml`, `eqm.lock`, and `eqm/` layout, then replace sample paths,
owners, commands, and product identities.

The minimal loop is:

```text
eqm doctor
eqm validate
eqm check
eqm matrix conformance
eqm obligations
```

`validate` establishes that authored documents form a current, resolved,
internally valid graph. `check` derives obligations and evaluates already
available evidence without running application tests. Missing evidence is an
expected explicit result while bootstrapping.

## Model one vertical slice

Choose one user-visible behavior, such as signup identifier entry.

1. Define its shared requirement in a surface contract.
2. Add Android and iOS bindings that map the requirement to implementation
   artifacts and evidence selectors.
3. Add a policy requiring the relevant facet for each target.
4. Add bounded runner definitions for the target-native tests.
5. Validate and inspect derived obligations.
6. Preview execution with `eqm verify --dry-run`.
7. With explicit authority, execute and re-run `eqm check`.

Inspect the model throughout:

```text
eqm show surface auth.signup.identifier
eqm locate auth.signup.identifier
eqm context auth.signup.identifier --max-bytes 32768 --max-depth 4
eqm obligations --unit auth.signup.identifier
eqm matrix conformance --unit auth.signup.identifier
```

## Selection and releases

Profiles are finite declared dimensions, not arbitrary environment strings.
Select them explicitly:

```text
eqm --profile audience.default=region:us check
```

For change impact, supply an exact baseline:

```text
eqm --baseline 23f3d83a51788ac88863cdc95e21b8c77c3832c7 affected
```

For a release decision, use the release profile and an exact release record:

```text
eqm --profile audience.default=region:us release check \
  --release-record releases/ios.json
```

Release results distinguish pass, fail, and unknown. Never coerce unknown into
pass in wrapper automation.
