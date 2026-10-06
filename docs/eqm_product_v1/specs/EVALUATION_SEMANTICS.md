# Normative evaluation semantics

This contract is the basis for a small reference evaluator and golden/property tests. Centralization is not permission to bless every current engine behavior. Compare existing functions and tests, document ambiguous policy semantics, and resolve them explicitly before wiring production.

## Evaluation order and completeness (R08, R11–R15, R58)

First validate schema/local invariants and resolve protected or development authority appropriate to mode. Select exact policy/profile revisions and values; reject duplicate requests and out-of-policy values. Protected modes cannot use candidate-local defaults.

Enumerate the required target/provider/scenario universe before evidence selection. Applicability is evaluated at the appropriate requirement/participant/profile coordinate. Explicit false under approved authority may be non-applicable. Unknown stays in the visible universe and is non-authorizing. Missing profile context cannot make a requirement disappear. Empty release scopes require protected explicit permission; otherwise no release authorization. A one-member cohort does not support a meaningful cross-target equivalence claim.

Derive scope- and facet-specific constraints. Compose relevant matching policy rules monotonically: stronger minimum trust/count and tighter age constrain rather than weaken. Do not accidentally apply one facet's unrelated rule to another. Whether selectors express required conjunctions or approved alternatives must be explicit and tested; do not invent an OR interpretation to obtain a pass.

Then map evidence by exact semantic subject, specification, binding, contract, profiles, execution and release/candidate context; verify trust independently; evaluate temporal eligibility and attempt/count semantics; perform actual structure/inventory/exposure checks; apply only authorized waivers; aggregate complete target/provider/scenario results and contract-relative comparison conclusions.

## Facet states and precedence

Preserve the current canonical distinction among NotApplicable, Satisfied, Waived, Failed, Stale, Missing, Unstable and Unknown. Do not infer ordering from enum discriminants. The baseline pure facet evaluator uses: explicit inapplicability first; unknown applicability/context/coverage/outcome/freshness/trust/supporting facts next; unstable history next; failure, stale, then missing; otherwise satisfied. Preserve all contributing reasons even when one summary state takes precedence. A conflicting immutable identity can make coverage unknown; unstable retry history is separately visible.

The baseline permits a valid waiver over failed/stale/missing provisional states, not over unknown/unstable states. Preserve that fail-closed boundary unless a separately reviewed contract change defines something stricter or otherwise explicit. A waiver is not a successful evidence outcome. Insufficient known trust and unknown verification are distinguishable, but neither satisfies an obligation.

Every supported payload must have explicit evaluation: tests, structural checks, snapshots, inventories, runtime snapshots, manual review and release-related evidence. A catch-all payload arm returning success is prohibited. A failed manual review, quarantined selection, missing expected case, zero selected cases, retry failure, timeout, and cancellation all have explicit non-success effects.

## Counts and campaigns (R16–R18)

Count the declared logical units, not files, imported copies or attempts. Bind the expected selection manifest to the plan/campaign. A threshold is applied to the required coordinate using policy-composed constraints; acquisition may report a successful execution without claiming policy satisfaction. Minimum-count semantics for different evidence kinds must be closed before exposing those kinds as acceptable; a release-record reference cannot automatically satisfy an arbitrary observation count.

Within a campaign, preserve immutable attempt history. Duplicate transport never changes aggregate counts. Contradictory results for the same immutable attempt are a conflict. A later passing retry does not erase earlier failing history. Across campaigns, explicit authorized selection determines current eligibility while old results remain available for audit. No arbitrary latest timestamp and no permanent failure poisoning from unrelated superseded campaigns.

## Supporting checks (R28, R59)

A file's declared role is not an observed role. Structural success requires the needed independent observation, correct type/confinement, and valid role evidence. Trusted complete current inventory can prove absence; partial, missing or untrusted inventory remains unknown. Presence of a runtime-facts document is not exposure match: compare its actual facts to authored expectations for the same exact subject/profile/time.

## Aggregates and reporting

Aggregate complete participating target/provider/scenario obligations. A target cannot ignore its provider or end-to-end dependencies. Missing participant evaluations yield unknown comparison, not a reduced target cohort. Extraneous targets are visible but do not change the protected required set. Relational checks compare specified normalized outcomes from the same scenario; separate target passes do not replace them.

Satisfied and explicitly not-applicable facets can form conformance; valid applied waivers may form conditional conformance; blocking states remain nonconformant or unknown as defined by the normative table. Release authorization rejects unknown and requires explicit protected treatment of conditional outcomes. Do not force all interfaces into the same display word if their objects differ, but ensure they reference one decision and never contradict its facts.

Filters, pagination, truncation and resource limits are presentation/execution concerns. They cannot modify the completeness denominator, required cohort, selected campaign, or release conclusion. A resource limit preventing complete evaluation must be explicit and non-authorizing.

## Properties required in tests

Permuting semantically unordered inputs leaves results stable. Duplicate delivery leaves results stable. Changing any bound subject/authority/profile/executable/evidence component prevents inappropriate reuse. Removing required evidence cannot strengthen a decision. Weakening candidate authority cannot weaken protected requirements. Adding irrelevant rejected records cannot manufacture a pass. Reference and optimized evaluation agree for the same frozen input. Time and trust changes affect current acceptance without mutating historical decisions.

## Approved precise semantics — ADR-0012

The owner approved the following closure rules; RA01–RA09 and RA21 carry executable regression obligations.

Required expected-selection members compose with AND. Alternative groups exist only when explicitly authorized by policy and bound into the frozen selection; observed pass/fail results cannot opportunistically select a different alternative. First account for all required cases, then apply the coordinate's count threshold to distinct eligible logical units. Enough passing observations cannot hide a required failing or missing case.

For schema-valid eligible evidence, separate required cases containing fail/pass aggregate to Failed. Fail/pass retries of the same logical case within the selected campaign aggregate to Unstable. Contradictory assertions for the same immutable attempt create Unknown conflict. Malformed count/outcome relationships reject admission and leave affected required evidence unresolved. Timeout/cancellation remains Unknown; skip/filter/quarantine/missing remains non-success. Existing facet precedence and contributing reasons still apply after these outcome calculations.

Count units must be declared per kind: expected logical test cases; independently checked structural/inventory/runtime facts; explicit review slots and authorized distinct principals where independence is required; or an exact eligible independently verified referenced record. One release-record reference provides at most one eligible record unit and cannot meet an arbitrary minimum. Repeated files/transports/attempts never increase credit. Fixture thresholds are synthetic; real policy supplies validated minimums, maximum ages, skew and allowed deviations.

Campaign eligibility is an explicit authorized selection bound to the decision. Supersession is an explicit trusted relationship; neither latest timestamps nor unrelated old campaigns determine current acceptance. Keep all historical attempts for audit.

Policy strengthening compares inclusion of the protected required universe plus constraints for corresponding coordinates. Increasing `minimum_level` can remove obligations and must not be treated as automatically stronger. Compose trust/count/age only from the rules matching each facet. Exact policy/profile revisions and context remain part of the coordinate.

Expanded fragment requirements inherit the stronger fragment-default and destination-journey risk. Absence inherits that value; explicit risk below it rejects. Validate the expanded graph before finalized/prepared construction. Preserve this rule through canonical projection and real manifest loading; see DEV-001.

Freshness uses explicit evaluation time and policy, preserving exact nanosecond arithmetic rather than flooring to milliseconds. Freeze supported canonical forms and all equality/expiry/skew boundary vectors in S006/S069. Do not inherit an undocumented fixed skew as production policy.

## Frozen count, outcome and risk vectors (S006)

ADR-0013 freezes these rules together with the representation boundaries; the
machine-readable set is `eqm_test_support::{REPRESENTATION_VECTORS, vectors_for}`,
and its unit tests execute the timestamp and digest vectors against `eqm_domain`
and prove every rule has a literal input with an exact accept output or
rejection. Enforcement and semantic
regressions land in the owning steps S013–S076.

- Required expected selections compose with AND; alternatives need explicit
  protected authorization bound into the frozen selection, never a pass-driven
  OR interpretation.
- Distinct required fail/pass cases aggregate to Failed; fail/pass retries of
  one logical case in the selected campaign aggregate to Unstable; contradictory
  assertions for the same immutable attempt aggregate to Unknown conflict.
- Timeout/cancellation stays Unknown; skip/filter/quarantine/missing stays
  non-success; enough passing observations cannot hide a required failing or
  missing case.
- Count units are logical cases/facts/review slots/eligible verified records per
  kind; one release-record reference is at most one unit; copies, transports and
  retries add no credit.
- Campaign eligibility is an explicit authorized selection; supersession is an
  explicit trusted relationship, never a latest timestamp.
- Expanded fragment requirements inherit the stronger fragment/journey risk;
  absence inherits it and an explicit lower value rejects, validated after
  expansion before finalized/prepared construction (DEV-001).
- Freshness uses explicit evaluation time and policy over exact nanoseconds;
  fixture thresholds and instants are synthetic and never production policy.

## DEV-030 complete coordinate admission

Checked conformance includes every relevant target/provider/scenario coordinate
and every unresolved applicability/participation outcome. Omitting end-to-end or
provider work cannot turn reported per-target passes into Conformant. Public empty
universe construction is not proof of complete policy resolution. Combine existing
three-valued requirement and coordinate participation semantics; missing declared
participation is Unknown, not implicit global inclusion. Preserve exact subjects,
profiles, contexts and repeated role-bearing scenario actors before filtering
evidence. Provider dependers use the declared checked dependency closure.

Expected selection manifests precede results: all required logical cases compose
with AND unless an explicit protected frozen alternative applies. A broad selector
does not discover expected cases from observed passes. Strengthening retains the
full visible denominator, including unresolved coordinates, and independently
checks any authority transition. Implementation order and frozen representation
are governed by [DEV-030](../implementation/COMMIT_SEQUENCE.md).

## DEV-032 admission clarification

An empty participation registry never implies inclusion, including v1 data
without the new axis. Missing scope authority stays non-authorizing while v1
bytes/readers remain unchanged. Checked derivation binds graph, selected exact
policy/profile definitions and applicability values; foreign selections or
contexts cannot shrink scope. Protected unresolved requests require explicit
exact authority rather than unique candidate defaults. Target evidence cannot
prove Provider or multi-participant scope without the exact required subjects
and dependencies. These are existing acceptance closures, governed by
[DEV-032](../implementation/COMMIT_SEQUENCE.md); no new product policy is added.

## DEV-033 admission provenance clarification

Existing exact-context acceptance requires graph semantics, policy/profile
definitions and values, applicability, mode/authority and immutable scope context
to remain bound through derivation and status admission. Equal visible coordinate
labels across graphs with different strength are insufficient. Authorizing status
results require privately validated provenance; a caller-supplied digest/tag or
naked status map is data, not authority. Canonicalization remains application/
manifest owned, the engine pure, and frozen projection roots unchanged.
S054 later extends complete wire identity; present false-success admission must
be repaired in P04-R4. See governing DEV-033 and its independent review.
