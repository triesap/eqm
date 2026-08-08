# Operator guide

EquivalenceMatrix is a local-first conformance graph. Start with `eqm init
--dry-run`, review the proposed current-schema files, then run `eqm init` in a
Git repository. Authored contracts live under `eqm/`; generated evidence and
local state live under `.eqm/` and must not be committed.

Use `eqm validate` for schema/graph validity and `eqm check` for non-executing
policy conformance. Select profiles explicitly with `--profile
audience.default=region:us`. Pull-request and release decisions require exact
authority: pass a commit, path, or semantic digest to `--baseline`; floating
branch names are rejected. `eqm affected --baseline COMMIT` is conservative
and expands unknown paths rather than omitting work.

Runner and adapter definitions must be locally digest-pinned, use typed argv
templates, bounded time/output/concurrency, explicit working directories, and
declared environment inputs. `eqm discover --adapter ID --target TARGET`
observes inventory; `eqm reconcile --target TARGET` compares that inventory to
authored exposure without modifying contracts. `eqm verify --dry-run` reviews
the exact evidence plan. Omit `--dry-run` only under explicit execution
authority; immutable results are written under `.eqm/results`.

Evidence trust is independently verified and cannot be raised by a producer's
claim. Waivers require authored policy scope, approvers, controls, and bounded
dates; they make a visible blocker conditional and never satisfy evidence.
Runtime facts and release records must bind exact repository, target, source,
build, artifact, profile, and digest identities. Run release gates with an
explicit release profile and repository-relative record:

```text
eqm --profile audience.default=region:us release check --release-record releases/web.json
```

Use `--offline` to prohibit nonlocal resolution. `eqm doctor` checks the local
toolchain, generated-state boundary, schema currency, symlinks, and forbidden
names without executing runners. Recovery is fail-closed: retain the authored
workspace, remove only disposable `.eqm` state after review, restore immutable
evidence from its trusted source, rerun `validate` and `check`, then explicitly
re-authorize execution. Never repair a failure by weakening policy, trust,
baseline, or waiver scope.
