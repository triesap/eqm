# Evaluation Truth Tables

Status: normative

This document defines the total EQM v1 evaluation functions. Every function is
pure over explicit prepared inputs and an injected UTC clock. No function reads
files, Git, processes, environment variables, network state, or wall time.
Unless a table states otherwise, unknown input produces unknown output and
never success.

## Evaluation Context

One evaluation context contains all of these exact values:

| Field | Type | Rule |
| --- | --- | --- |
| `mode` | `development`, `pull_request`, or `release` | explicit outside local development; no environment inference |
| `contract_digest` | semantic graph digest | one finalized graph |
| `policy` | policy ID, revision, digest | exact selected protected policy |
| `profiles` | profile IDs, revisions, and selected dimension values | all dimensions resolved or explicitly unknown |
| `required_targets` | target-ID set | derived from selected policy |
| `subject` | exact subject identity | source/build/release components defined below |
| `baseline` | optional exact semantic digest and subject | required for monotonicity, diff, and affected analysis |
| `trust_config_digest` | digest | exact trust roots and algorithm policy |
| `runtime_facts_digest` | digest | exact prepared facts set |
| `evaluated_at` | UTC instant | injected clock value |

Results from different context values cannot be combined into one conformance
or equivalence decision.

## Applicability

Applicability uses three values: `true`, `false`, and `unknown`.

### Leaf comparison

| Dimension state | `eq x` | `ne x` | `in S` | `not_in S` |
| --- | --- | --- | --- | --- |
| known value equals/matches | true | false | true | false |
| known value differs/is outside | false | true | false | true |
| missing or unknown value | unknown | unknown | unknown | unknown |

An undeclared dimension or comparison value is a manifest error, not an
evaluation result.

### Boolean composition

`all` and `any` fold their nonempty operand arrays with these complete binary
tables. Associativity makes the result independent of fold grouping.

| `all` | right true | right false | right unknown |
| --- | --- | --- | --- |
| left true | true | false | unknown |
| left false | false | false | false |
| left unknown | unknown | false | unknown |

| `any` | right true | right false | right unknown |
| --- | --- | --- | --- |
| left true | true | true | true |
| left false | true | false | unknown |
| left unknown | true | unknown | unknown |

Negation maps true to false, false to true, and unknown to unknown. Constant
`always` maps directly to its Boolean value.

## Policy Selection And Composition

| Mode | Selection |
| --- | --- |
| `development` | explicit policy/profile if supplied; otherwise the repository-declared development default |
| `pull_request` | explicit policy and profile required from trusted invocation context |
| `release` | explicit protected policy and profile required; candidate-local defaults are ignored |

Missing, ambiguous, inactive, or mismatched policy/profile authority is an
error. Policy rules match requirements by their closed selector. A requirement
becomes an obligation when its applicability is true and its level is at least
the matching rule's `minimum_level`. False produces no obligation. Unknown
produces an unknown applicability blocker, not omission.

When rules overlap, composition is monotonic:

| Axis | Composition |
| --- | --- |
| minimum requirement level | strongest ordered value |
| required facets, targets, profiles, controls | set union |
| minimum trust | strongest ordered value |
| maximum evidence age | smallest duration |
| minimum evidence count | largest integer |
| waiver allowed | logical AND |
| maximum waiver days | smallest duration when allowed |
| minimum approvers | largest integer |

An unmatched active required requirement is a policy error. An unmatched
recommended or optional requirement produces no obligation and a stable
warning.

### Protected baseline monotonicity

| Candidate change | Result |
| --- | --- |
| add requirement, target, facet, control, or trust root | strengthening |
| raise level, risk, trust, count, or approver minimum | strengthening |
| reduce freshness or waiver duration ceiling | strengthening |
| prohibit a previously allowed waiver class | strengthening |
| remove or narrow an obligation or required target | weakening, reject |
| lower level, risk, trust, count, or approver minimum | weakening, reject |
| increase freshness or waiver duration ceiling | weakening, reject |
| enable a previously prohibited waiver class | weakening, reject |
| replace an immutable authority with an incomparable authority | unknown, reject |
| semantic equality | unchanged |

A waiver never authorizes changing protected policy. It applies only to a
derived obligation after the candidate passes monotonicity checks.

## Obligation Derivation

An obligation key is:

```text
(policy, profile-values, unit, requirement, scope-subject, facet, release-context)
```

| Requirement scope | Derived scope subjects |
| --- | --- |
| `each_target` | one obligation per policy-required target |
| `shared_provider` | one provider obligation plus a reference from every required target; evidence executes once |
| `end_to_end` | one obligation over the sorted complete required-target set |

Each selected facet becomes one independently evaluated obligation. Duplicate
derivations with the same key merge using the strongest composed policy.
Different keys never share a conclusion, though they may explicitly reference
the same immutable evidence result when its coverage permits it.

## Evidence Coverage

An evidence result covers an obligation only when every row is true:

| Check | Required equality or relation |
| --- | --- |
| evidence specification | exact specification ID and digest |
| requirement | obligation full requirement ID is explicitly listed |
| facet | obligation facet is explicitly listed |
| target/provider/set | exact scope subject equality |
| contract and binding | exact semantic digests |
| kind | result kind equals specification kind |
| runner and adapter | exact digests when applicable |
| profile and release context | exact evaluation values |

No prefix, ancestor, unit-wide, filename, selector-text, or best-effort match is
allowed. Duplicate result IDs or two conflicting immutable payloads with the
same identity produce unknown and a diagnostic.

## Attempt And Count Aggregation

Individual attempt outcomes are `passed`, `failed`, `skipped`, `filtered`,
`quarantined`, `timed_out`, `cancelled`, and `error`.

| Attempts and counts | Aggregate facet status before freshness/trust |
| --- | --- |
| no matching result or selector matched zero items | `missing` |
| all terminal attempts passed and passed count meets minimum | `satisfied` |
| any failed attempt followed or preceded by a pass | `unstable` |
| any `timed_out`, `cancelled`, or `error` | `unknown` |
| any `skipped`, `filtered`, or `quarantined` needed for minimum | `missing` |
| terminal failure with no pass | `failed` |
| internally inconsistent totals, duplicate attempts, or impossible sequence | `unknown` |
| passes below minimum count | `missing` |

Retries cannot erase history. A clean pass requires every recorded attempt in
the immutable result set to be consistent with passing; a pass after any fail
is unstable.

## Freshness And Cache Identity

A freshness key is the exact tuple:

```text
(subject, contract_digest, binding_digest, evidence_spec_digest,
 runner_digest, adapter_digest, policy_digest, profile_values,
 target_configuration_digest, runtime_facts_digest, release_record_digest,
 trust_config_digest, producer_identity, tool_version)
```

Not-applicable members use an explicit typed `none`, never omission. Cache hits
require byte-for-byte key equality and a verified payload digest.

| Condition | Freshness result |
| --- | --- |
| every key equals and age is within composed maximum | fresh |
| any semantic key differs | stale |
| evidence timestamp plus maximum age is before `evaluated_at` | stale |
| evidence timestamp is later than `evaluated_at` plus 5-minute clock tolerance | unknown |
| timestamp, maximum age, or required key is absent/invalid | unknown |

Stale and unknown results remain visible but cannot satisfy an obligation.
Expiry is evaluated at the injected clock and boundary equality is fresh:
`observed_at + maximum_age >= evaluated_at`.

## Trust

Trust levels, weakest to strongest, are `untrusted_local`, `trusted_ci`, and
`signed_ci`. Effective trust is the weakest of the result's claimed level and
the independently verified producer, transport, signature, and subject trust.

| Verification state | Effective ceiling |
| --- | --- |
| unsigned local result with exact local producer | `untrusted_local` |
| result from an authenticated configured CI identity | `trusted_ci` |
| valid allowed signature from a configured CI key over the exact envelope | `signed_ci` |
| missing authority, invalid signature, wrong subject, revoked key, or unsupported algorithm | unknown/untrusted; result cannot satisfy |

Development policy may require any level. Pull-request policy defaults to at
least `trusted_ci`; release policy defaults to `signed_ci`. A repository
candidate cannot add a trust root used to approve itself against a protected
baseline.

## Waiver Validity

A waiver is valid for one obligation only when all conditions are true:

- exact policy, target/provider/set, unit, requirement, facets, and profile
  scope match without wildcard expansion;
- selected protected policy allows that waiver class;
- start date is on or before the evaluation date and expiry is strictly after
  it;
- duration is within policy maximum;
- approvers are distinct authorized identities and meet the minimum;
- issue reference and reason are valid and nonempty;
- every required compensating control is present and satisfied;
- waiver authority is not supplied solely by the candidate being evaluated.

Invalid, expired, ambiguous, or unverifiable waivers do not apply. A valid
waiver maps a waivable blocking status to `waived`; it never maps any status to
`satisfied`. `unknown`, `unstable`, subject mismatch, invalid trust, and absent
evaluation context are not waivable in v1.

## Facet Status Precedence

For each derived obligation facet, evaluate in this order and stop at the first
matching row:

| Priority | Condition | Status |
| --- | --- | --- |
| 1 | applicability is false | `not_applicable` |
| 2 | applicability is unknown | `unknown` |
| 3 | subject/context mismatch, invalid trust, invalid envelope, or internal inconsistency | `unknown` |
| 4 | attempt aggregate is unstable | `unstable` |
| 5 | fresh trusted aggregate is failed | `failed` |
| 6 | result is stale | `stale` |
| 7 | required result/count is absent | `missing` |
| 8 | valid waiver covers a `failed`, `stale`, or `missing` status | `waived` |
| 9 | fresh trusted aggregate passes and all coverage checks pass | `satisfied` |

Priority 8 is applied to the provisional result from priorities 5-7 before it
is finalized. Evidence not selected by an obligation is reported separately
and cannot influence its status.

## Target Conformance

Only policy-derived obligations participate.

| Complete target facet set | Conformance |
| --- | --- |
| every status is `satisfied` or `not_applicable` | `conformant` |
| at least one `waived`, all others `satisfied` or `not_applicable` | `conditionally_conformant` |
| any `failed`, `missing`, `stale`, `unknown`, or `unstable` | `nonconformant` |
| required target or obligation set cannot be constructed | no target result; evaluation error |

An empty obligation set is conformant only when policy resolution completed
and explicitly derived zero obligations. Evaluation errors cannot be converted
into conformance.

## Target-Set Equivalence

First require one exact context, one complete required-target set, and one
conformance result for each target. If any precondition is absent, mismatched,
or unknown, equivalence is `unknown`.

| Complete required-target conformance set | Equivalence |
| --- | --- |
| all `conformant` | `equivalent` |
| one or more `conditionally_conformant`, none `nonconformant` | `conditionally_equivalent` |
| one or more `nonconformant` | `not_equivalent` |

Extra non-required targets are reported but do not change the required-set
result. Conditional equivalence lists every contributing waiver.

## Exposure Reconciliation

For each `(profile-values, target, surface, release-subject)`, keep six facts
independent:

| Fact | Source | Values |
| --- | --- | --- |
| expected | contract and policy | required, prohibited, unknown |
| declared | binding exposure | true, false, unknown |
| discovered | prepared inventory | true, false, unknown |
| enabled | prepared runtime facts | true, false, unknown |
| released | exact release record | true, false, unknown |
| conformant | target evaluation | true, conditional, false, unknown |

Absence is `false` only from an authoritative complete input. Absence from a
partial/failed adapter, missing fact set, or stale result is unknown. No fact
implies another.

For each Boolean observed fact (`declared`, `discovered`, `enabled`, or
`released`), reconciliation is:

| Expected | Observed true | Observed false | Observed unknown |
| --- | --- | --- | --- |
| required | match | mismatch | unknown |
| prohibited | mismatch | match | unknown |
| unknown | unknown | unknown | unknown |

Conformance is reported alongside these comparisons and does not overwrite
them. A prohibited but conformant implementation is still an exposure mismatch.

## Release Gate

An exact release subject is `(target, app_version, build_number, source_commit,
artifact_digest, channel)`. The release record, evidence, runtime facts,
contract, policy, profile values, trust configuration, and evaluation clock
must all bind that exact subject.

| Complete release state | Gate result |
| --- | --- |
| exact subject; required exposure matches; conformance conformant; all release facets satisfied; trust sufficient | pass |
| same, but all deviations are validly waived and visible | conditional |
| complete exact inputs with any unwaived mismatch or nonconformance | fail |
| missing, stale, ambiguous, inexact, invalid, or unverifiable input | unknown |

Release policy defaults to `signed_ci`. Unknown and subject mismatch are never
conditional and never pass.

## Semantic Diff

Diff compares exact finalized baseline and candidate projections. Changes are
sorted by `(unit, requirement, target, facet, kind, field)`.

| Change | Classification |
| --- | --- |
| add requirement/target/facet or strengthen ordered policy | `strengthened` |
| remove requirement/target/facet or weaken ordered policy | `weakened` |
| add/remove entity without ordered policy meaning | `added` / `removed` |
| change evidence specification, runner, adapter, or trust input | `evidence` |
| add/change/remove waiver | `waiver` |
| change intended exposure | `exposure` |
| change only excluded projection metadata | `nonnormative` |

Baseline-to-candidate `added` is candidate-to-baseline `removed`; strengthened
is weakened in reverse. Evidence, waiver, exposure, and nonnormative records
retain their kind and swap before/after values.

## Affected Set

Affected analysis is conservative:

| Change input | Required affected result |
| --- | --- |
| contract unit or requirement | that unit, dependents, fragment consumers, bindings, obligations, and required target sets |
| fragment | every transitive consumer and derived obligation |
| policy/profile/trust | every obligation selected by the changed authority |
| binding/artifact/exposure/evidence spec | bound unit and target obligations |
| runner/adapter | every evidence specification that references it and downstream obligations |
| waiver | every exactly scoped obligation |
| known changed file mapped to artifacts | their bound units and downstream obligations |
| changed target file with no mapping | all units bound to that target |
| changed repository file with no target classification | all workspace units and obligations |

The algorithm may over-report but must never omit a potentially affected
obligation. An exact empty affected set is allowed only when every changed
input is classified as nonnormative and the mapping is complete.
