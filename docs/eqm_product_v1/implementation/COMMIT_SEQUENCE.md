# Commit-by-commit implementation sequence — eqm_product_v1

## DEV-035 — Codex takeover and independent release-matcher review

Codex now owns implementation, tests, planning and engineering decisions within
the approved MVP. The owner requests an execution command; product implementation
and green checkpoint commits are authorized by the active `pfc all` directive. Earlier
implementation-agent role restrictions are superseded. Original 64 R/AC, 21 RA,
Q01–Q15, S001–S211, ordering and acceptance gates remain unchanged.

Candidate b442b806e26a7ddd08ca8543b802c1b3d91251f0 is committed, not an
uncommitted tree. RF79 is independently accepted for its allocated fail-closed
matcher scope: ordinary target evidence cannot authorize Provider/TargetSet
coordinates. P04-R4A is accepted; complete provider/scenario semantics still
belong to later owning steps. Preserve all 47 numbered acceptances and earlier
RF73/RF77 acceptance. At that review S044/S047/S048/S049 remained partial. After P04-R4B/C
standard and independent aggregate review, S044/S048/S049 receive numbered local
acceptance (50 total). S052-A/B/C subsequently passed their allocated gates (51
numbered acceptances). S047 residual identity/aggregation remains S054/S073.
See [aggregate repair disposition](reports/P04-R4C.md) and [scope report](reports/S052.md).
S053 subsequently passed its allocated gates (52 numbered local acceptances).
S054 and S055 subsequently passed their allocated gates (54 numbered local
acceptances). S056 subsequently passed its allocated gates (55 numbered local
acceptances). S057 subsequently passed its allocated gates (56 numbered local
acceptances). S047 aggregation plus S058–S211 remain unfinished.
See [independent review](reports/CODEX-b442b80.md).

**Next order:** S058–S076, mandatory independent complete-universe
and reference-evaluator acceptance before S077, then S077–S211 subject to their
existing dependencies and gates. RF84 is accepted for its allocated P04-R4C
contribution. Retain DEV-033 representation/provenance decisions: exact checked
same-graph policy/context/status authority; protected actual release callers use
explicit policy/profile revisions; no weaker-graph status admission or implicit
protected defaults. No new owner decision is currently required.


### P04-R4B verification slices

Implement P04-R4B in two coherent green commits: P04-R4B1 binds evaluated
facet carriers to checked full semantic graph, exact definitions/values,
applicability, mode, origin, release context and composed strength; P04-R4B2
integrates protected graph authority and exact selection in the actual release
consumer. This split changes no acceptance or ordering. Preserve independent
review before aggregate acceptance. No serializer dependency enters the engine;
application-computed canonical projections remain descriptive identities and do
not replace checked semantic provenance. A caller-supplied digest grants no trust.

P04-R4B2 uses the existing AuthorityProvider capability, loads once, and derives
the protected denominator before evidence acquisition. Candidate validation is
non-authorizing; conservative semantic graph equality rejects differing authority
until S129 owns baseline change comparisons. Exact missing selections remain
typed errors even without a provider; absent/unavailable protected authority
returns Unknown. Record context uses the validated immutable record digest and
the already captured evaluation time. Every mandatory record/runtime/evidence
trust conclusion must be present before computing minimum trust; an absent
conclusion cannot disappear through optional iteration. No synthetic signing
adapter grants graph authority in production. These choices enforce existing
requirements and do not implement or waive later host/cryptographic acceptance.

P04-R4C also verifies current strength through the real release ingress: supported
EachTarget evidence with one exact observation passes count 1 and remains Missing
and non-authorizing at count 2 under corresponding explicit synthetic graph
authority. Keep subject/context digests valid for each fixture so stale identity
or unsupported scope cannot mask the count control. Prepared fact maps remain
non-authorizing inputs to pure classification; previously evaluated carriers
cannot cross contexts. Complete lineage/count admission remains S059/S061.

P04-R4C additionally requires a loaded-graph UnknownApplicability-only control:
all remaining facets satisfied still yields Nonconformant, without another
unresolved cause masking it. A complete prepared provider universe must pass its
positive completeness control, while omitting the provider fact yields typed
IncompletePolicyResolution. Genuine v1 workspace/lock ingestion stays supported
but cannot manufacture participation authority. Protected usage examples require
exact policy/profile revisions and every dimension; absent graph authority remains
Unknown. Exact provider dependency, scenario occurrence and repeated-actor scope
integration remains S052/S054/S073; this checkpoint does not claim that later work.

### S052 verification slices

After green P04-R4C, implement S052-A typed exact ScenarioRef and duplicate-
rejecting RequirementScenarioBindings plus the separate canonical projection and
digest; S052-B checked provider dependency closure and exact scenario derivation,
including private evaluation-context bindings and non-authorizing missing scope;
S052-C application/protected-authority/CLI integration and aggregate owning tests.
Each slice must be independently green; S052 stays incomplete until all three
slices and original acceptance controls pass. No later step begins between them.
The existing target-only Provider membership and TargetSet reduction are unsafe
for S052 exact scope; replace their authorizing use with exact dependers and
occurrences, preserving unrelated target semantics and earlier safety controls.
A missing provider assignment stays unresolved. Unrelated non-dependers cannot
block or satisfy provider work. Protected scope bindings come from authority;
CLI values are claims and cannot replace them. Binding/scenario semantic context
is checked before accepting statuses; S054 adds complete public identities.

S052-B retains the complete selected scenario for every outcome. The approved
spec does not define an atomic reduction rule for mixed true/false actor
participation: do not invent AND/OR semantics or prune occurrences. Under true
requirement applicability, all relevant participation true requires the full
scope; all explicitly false is visible N/A; any unknown or mixed true/false
retains incomplete/non-authorizing scope with the complete actor/edge data.
Actors outside the policy-required target set remain visible in scenario authority
without enlarging that target cohort; missing actor participation stays unknown.
Provider-only scenarios affect actual selected dependers through declared provider
assignments and checked target dependencies. No assigned depender is missing scope
authority, never empty success or global provider participation. Preserve explicit
requirement-level false/N/A. These are fail-closed ambiguity dispositions within
existing completeness requirements, not a new scenario composition policy.

Exact provider/scenario scopes must not be downconverted into legacy provider-ID
or target-set waiver authority. Until the owning exact waiver representation
lands, these unrepresentable subjects reject with the explicit Subject reason.
Keep one checked membership operation for engine and report adapters; legacy
reference subjects do not grant exact scope authority. If safe derivation needs
its application/CLI migration to remain green, combine dependent B/C work in one
coherent checkpoint and disclose the allocation; never retain an unsafe implicit
scope merely to manufacture an intermediate green commit.

Codex selects a combined S052-B/C checkpoint: replacing implicit scope changes
the engine output consumed by the application and all report adapters, and the
existing full-scope omission controls require exact scenario fixtures/inputs.
Land checked derivation, protected request consumption, adapter membership and
those dependent controls together. A remains separate; no S053 work is included
and S052 acceptance still requires the full original aggregate gate.

Protected preparation loads one immutable authority bundle containing the finalized
graph and explicit scope-binding map. Candidate supplied maps are prevalidated
claims and, when supplied, must equal that protected map; omitted candidate maps
never cause fallback to candidate scope. Development passes explicit claims.
No extra load, clock capture, latest lookup or runner effect is introduced.
Read/report adapters must retain unresolved scope/applicability/participation as
non-authorizing diagnostics. Allocate EQM-E0501 within the existing diagnostic
range for required unresolved scope; update its registry/explanation and owning
checks. An otherwise empty check cannot pass by dropping those coordinates.

**RF84 — restore causal dirty-source regression coverage.** The dirty-source
release test uses the original full-scope fixture, which is already non-authorizing
after RF79. Use a separately authored supported EachTarget-only synthetic clean
Pass control, then mutate dirty source and require non-authorization; retain the
original full-scope negative unchanged. Prove the source-clean guard regression
is detected independently of non-target scope rejection. Audit nearby negative
controls for the same masking, without weakening typed-error or original scope
assertions. This restores existing acceptance evidence, not a new product rule.

Execute one bounded checkpoint at a time; retain compact resume state and external
logs. Continue eligible work after green checkpoints. Implementation by Codex does
not waive independent review: use a separate reviewer at mandatory gates and do
not self-certify whole-feature acceptance. Missing automation is software work;
only actual external authority/platform inputs block their dependent acceptance.
`pfc all` does not authorize push, signing, deployment or publication.

## Historical DEV-034 — completed execution batch

## DEV-034 — Owner-requested smaller implementation batch

The owner requests a smaller batch after the preceding implementation session
ended without product edits or a commit. Preserve all uncommitted work and all
47 numbered acceptances. Original requirements, acceptance criteria, verification
and the full MVP through S211 remain unchanged. DEV-033 technical decisions are
retained; its 26-checkpoint execution authorization is superseded for this run.

**Current command:** `pfc through P04-R4A`. Complete only the focused release
matching repair below, one green checkpoint, then return for Codex review. Do not
start P04-R4B/P04-R4C or S052. This smaller endpoint is explicitly owner-requested.

### P04-R4A — Reject incomplete non-target evidence (RF79)

Commit `fix: reject incomplete non-target release evidence`.
Remove the any-member TargetSet and unrestricted Provider matcher shortcuts.
Ordinary target evidence cannot satisfy provider/target-set/scenario obligations
without complete exact subject/configuration/participant/dependency authority.
Until the later owning steps integrate that complete representation, these
coordinates remain Missing/Unknown and cannot produce Pass/Conditional.

Add same-ingress single-member and wrong-provider negative controls. Preserve
all original signup end-to-end obligations and their denominator. Keep a separate,
explicitly authored supported EachTarget-only synthetic positive control; it
must not masquerade as full signup/scenario coverage or remove required work
from the original fixture. Regenerate only causally changed reviewed goldens and
disclose them. No premature S052/S054/S059/S061/S073 implementation.

Retain the existing partial participation/selection edits. They may be included
with approved pending planning in this first verified repair commit, with honest
partial dispositions; this does not close their outstanding acceptance. RF82
provenance and RF81 protected consumer integration remain open and non-authorizing
where required. No claim that green tests make all release admission complete.

**Green:** real release ingress rejects incomplete non-target evidence, supported
positive control passes under explicit synthetic authority, original complete
scope remains visible; original owning tests, fmt, locked workspace check,
Clippy, full `cargo xtask check`, relevant doctests, schema parity, package
structure and diff pass after final edits. No thresholds or assertions weakened.

### Remaining aggregate repair and sequence (not this batch)

- P04-R4B: RF78/RF81/RF82 complete checked graph/context/status provenance and
  protected actual consumer preparation, retaining DEV-033 decisions.
- P04-R4C: remaining RF75/RF81 durable matrix, RF83 real-CLI v1/v2 controls,
  version/usage guidance and final aggregate admission/integration evidence.
- Only after all three allocated slices and aggregate criteria are verified may
  P04-R4 be considered implemented. Codex independently accepts it. Then original
  S052–S076, independent evaluator review before S077, and the full S211 plan.

Load only current amendment, active finding/criterion and relevant source/tests.
Use targeted tests while iterating and full gates when the slice is ready. Keep
large logs external, inspect concise failure excerpts, and update compact resume
state. Do not load the whole ledger, native history or later-step specifications.
No product scope reduction, acceptance waiver or public private metadata.

## Historical DEV-033 — superseded execution horizon; decisions retained

## DEV-033 — Resume preserved repair and complete P04/P05

**Codex disposition:** the returned candidate is the uncommitted working tree at
`3265bbee657d08ae43b801cf6f5beaec42be8c23`, not a new commit. Preserve every
implementation edit and all 47 earlier numbered acceptances. P04-R4 is partial;
S052–S076 are unstarted. [Independent review](reports/CODEX-P04-R4-partial.md)
records verified contributions and remaining authority defects. DEV-032 and
DEV-030 representation decisions below remain binding; this amendment clarifies
execution and the already-required status provenance. Original 64 R/AC, 21 RA,
Q01–Q15 and full S211 scope remain unchanged.

**Batch:** `pfc through EQM-P05`, finish P04-R4 then S052–S076 inclusive (26
coherent checkpoints). Continue across S058/S059; independent complete-universe/
reference-evaluator review at S076 before S077. No new repair-only review gate.
No genuine owner-only blocker or human release-test milestone is reached.

### Preserve and finish P04-R4

- Retain removal of the empty participation fallback, mode-aware unresolved
  selection with typed missing/ambiguous causes, same-graph policy comparison,
  explicit synthetic v2 participation and formatting repairs. These are partial
  contributions, not complete RF75/RF78/RF81 acceptance: durable adversarial
  controls and the full original admission contract must still land.
- RF79 remains unchanged in production. Reject ordinary target evidence for
  Provider/TargetSet/scenario scope unless complete exact subjects/configuration/
  participants/dependencies are validated. Fail closed now; complete scope
  integration remains allocated to S052/S054/S059/S061/S073. Preserve supported
  single-target positive controls, same-ingress non-target negatives and complete
  scenario positives at their owning steps. Do not delete required end-to-end
  work, invent authority, or treat separate target passes as a scenario proof.
- **RF82 (remaining RF78 contract):** graph policy/context equality at derivation
  does not bind status admission. A graph with minimum_count=1 and a graph with
  minimum_count=2 produce identical current keys; the stronger graph accepts
  the weaker graph's satisfied status map. Bind graph semantics, exact selected
  policy/profile definitions and values, applicability, mode/authority and
  immutable scope context to derivation and its status carrier. Privately validate
  the carrier and reject cross-context admission even when visible keys match.
  Keep the engine pure; canonical identity is computed by the application through
  existing manifest projection boundaries, with no manifest→engine production
  edge. Do not let a freely caller-assigned identity or origin tag establish
  provenance. Same immutable semantic context may be reused without artificial
  object-address identity. S054 adds the later complete wire obligation identity;
  it does not defer this present admission safety. A naked BTreeMap is data, not
  an authorizing evaluated result. Pure reference/fixture status inputs remain
  explicitly non-authorizing. Update actual consumers together.
- RF81 integration remains open: release_check calls
  evaluation::derive_with_policy_universe, which hardcodes Development. The new
  mode-aware resolver is not yet used as protected release preparation. Do not
  promote a candidate-local development snapshot to a protected result. Use
  exact permitted protected selection in the application; absent host authority
  stays explicitly non-authorizing. Synthetic explicitly injected authority may
  test local semantics but cannot prove live trust readiness. Do not build the
  later host trust system prematurely or fabricate production inputs.
- Require real preparation controls for empty/asymmetric/false/unknown
  participation; foreign weaker policy/applicability/profile definition; changed
  graph with identical coordinate labels and different strength; absent provider/
  scenario coverage; single-member/wrong-provider evidence; protected omitted
  policy/revisions/profiles/values versus exact complete input; typed missing vs
  ambiguous failures. Retain private-constructor compile-fail proofs.
- **RF83:** retain an independent real-CLI valid v1 workspace/lock positive
  control alongside v2 and unsupported-version/wrong-kind negatives. The changed
  V1_WORKSPACE constant now names v2, so the comment claiming an unmutated v1
  control is false. Name fixtures/constants accurately and synchronize authored
  format/agent usage guidance with the actually supported v1/v2 families;
  migrating the shared
  example must not erase the earlier v1 reader regression coverage.
- The approved v2 example migration is synthetic authored authority. Frozen v1
  readers/schema bytes remain intact. Isolated v1 projection tests may omit v2
  participation only when they assert projection/expansion, never conformance.
  Record old/new example and reviewed golden digests; do not regenerate semantic
  goldens merely to hide a coverage regression. P04-R4 report and progress must
  describe all preserved uncommitted work and the actual completed controls.

### Continuation and context discipline

All scenario-binding, provider dependency, expected-selection and evaluator
choices in DEV-030/DEV-032 are settled. Resume implementation rather than reload
all historical ledgers/logs. Read the current amendment and review, then use
scoped structured ledger queries and the active step's exact acceptance criteria.
Keep logs externally; summarize failures with relevant lines rather than dump
whole repeated logs. Maintain a concise external resume receipt after each green
checkpoint. Preserve every required lane and threshold. A long conversation or
passing workspace subset is not completion; continue until S076 or a genuine
runtime limit/user stop/new hard dependency. Record observed limit evidence
separately from an estimate; do not assert a hard runtime error without one.

All standard/owning/schema/structure/diff gates in DEV-032 remain required after
final product edits and before each authorized green commit. First P04-R4 commit
includes the approved supervisor planning files and preserved implementation.
No Codex product changes, scope waiver, public private metadata, parent gitlink
staging, push, deployment, signing or native activation is authorized.

## Historical DEV-032 — prior repair issuance

## DEV-032 — Finish exact admission repairs and the full reference evaluator

**Date/author:** 2026-10-06, Codex under owner delegation.
**Reviewed candidate:** `3265bbee657d08ae43b801cf6f5beaec42be8c23`.
[Independent review](reports/CODEX-3265bbe.md) retains both repair commits.
P04-R2/R3 are partial, not independently accepted as complete checkpoints.
The engine-owned private finalization/token and raw-data manifest API repair
(RF73), and documentation link repair (RF77), are allocated accepted contributions.
Preserve all47 earlier numbered local acceptances. S044/S047/S048/S049 remain
changes-required; no full AC/RA/platform/live gate is closed by these subsets.
DEV-031 active observation is now superseded by the completed candidate.
All original64R/AC21RAQ01–Q15S211 and retained DEV-030/DEV-028 decisions remain.

**Batch:** `pfc through EQM-P05`, P04-R4 then S052–S076 inclusive,26 separate
coherent green checkpoints. Complete the admission repair and all25 later steps,
continuing across S058/S059. Stop after S076 for required independent full
required-universe/reference-evaluator review before S077 storage integration.
No new review gate is inserted after the repair or after P04. Original dependencies
and available verification remain binding; genuinely unavailable external inputs
block their dependent acceptance, not other approved configurable/local work.

### P04-R4 — Bind derivation to explicit complete scope

Commit `fix: bind derivation to explicit complete scope`.
Owns remaining RF72/RF74/RF75 and new RF78/RF79/RF80/RF81.

1. **No empty-registry fallback (RF75).** Remove `participation_active`/None→true
   bypass. Missing exact participation is Unknown in every mode and wire version;
   an empty registry is not version detection. Raw v1 bytes/readers remain their
   declared contract, but missing new scope authority cannot authorize. Synthetic
   positive fixtures may migrate deliberately to explicit v2 authored participation
   and real loader input where required. Do not infer rules from target lists,
   invent live values or add a legacy/compatibility reader. Preserve explicit false
   and every exact unresolved coordinate. Evaluate and validate all declared
   applicability axes, including combined false/unknown controls.
2. **Same checked derivation inputs (RF78).** Current public derivation accepts a
   selected policy borrowed from a different graph; same IDs/revisions can shrink
   the denominator and make an omitted original target Conformant. Bind selection
   and applicability to the same immutable checked graph and exact mode/authority/
   profile values, with private engine-owned validated preparation provenance.
   Public authorizing derivation must either consume that checked preparation or
   verify exact same-graph selection/definition/context before deriving; raw helper
   requests cannot promote authority. A foreign policy or context is an error,
   not an empty scope or fallback. Bind semantic graph/scope identity to later
   status admission so identically spelled keys across different contract inputs
   cannot masquerade as the same denominator. Engine remains pure/no serializer;
   application supplies canonical identity through existing manifest projection
   boundaries. Exact graph identity and current context are verified, not parsed
   tags. Keep no production manifest→engine dependency.
3. **No target evidence promotion (RF79).** Current release matcher accepts any
   target member for TargetSet and any binding for Provider. A passing web-target
   test cannot prove an ios+web scenario or exact provider obligation. Reject or
   retain Unknown/Missing for unsupported/incomplete scope evidence now; remove
   permissive matching shortcuts. Non-target evidence must match complete scope,
   participant/dependency/subject/configuration authority. S052/S054/S059/S061/S073
   integrate full representations at their original steps; do not implement those
   later steps inside this repair. Preserve a same-ingress negative regression
   for one-member/wrong-provider evidence. If a v1 fixture cannot express the
   required scope, it stays explicitly non-authorizing; document legitimate
   semantic golden changes and retain independent supported complete-scope
   positive controls without deleting required-work or adversarial assertions.
4. **Explicit protected preparation (RF81/RF74).** New typed resolution is shared,
   but `prepare_obligations_from_request` fills omitted policy/revisions even in
   Release with ProtectedBaseline. Unique candidate lookup is Development only.
   Make mode explicit at the unresolved request admission boundary. PullRequest/
   Release require explicit exact policy and complete selected profile revisions/
   values from their permitted authority; reject absent policy/revision/profile
   instead of filling it. No implicit defaults or authority from an origin tag.
   Existing host/trust integration remains separate; synthetic origins prove pure
   negative semantics only. Distinguish zero stored revisions (NotFound) from
   multiple (Ambiguous); both reject, retaining typed causal diagnostics.
5. **Actual green/evidence discipline (RF80/RF76).** Current standard check fails
   rustfmt at selection_preparation.rs:224; P04-R3 did not run the combined standard
   gate after its final edits. Pi repairs formatting and runs the full actual gate
   on the final tree before committing this repair and each later checkpoint.
   Update reports/cursors/actual SHAs and keep implemented/verified/accepted
   distinct. Preserve unique failed attempt logs with true upstream exits.
   Cosmetic progress indentation is normalized by the supervisor; do not
   reserialize it wholesale or overwrite acceptance records.

**Required durable controls:** empty registry; positive/false/unknown/asymmetric
participation via real preparation; foreign same-ID weaker policy and foreign
applicability; changed graph with identical coordinate labels; omitted provider/
scenario coverage; wrong provider and single-target evidence; protected omitted
policy/revisions/profiles versus explicit complete inputs; exact missing versus
ambiguous cause; existing private-constructor compile-fail tests. Original
unmatched advisory warnings do not become new mandatory obligations.

**Green/verification:** original S044/S047/S048/S049 lanes, all-target domain/
manifest/protocol/engine/application/CLI tests, relevant owning doctests and actual
application/release ingress; fmt/check/Clippy/full `cargo xtask check`, schemas
parity, package structure and diff. All run after the final edits, logs retain
failed and successful attempts. Do not commit a red checkpoint or substitute
helper/component tests for the required full standard gate.

### Eligible successor decisions remain settled

S052 uses explicit FullRequirementId→exact ScenarioRef bindings as immutable
preparation authority, repeated role-bearing actor/dependency scope and existing
provider dependency closure; the separate `eqm:v2:requirement-scenario-bindings`
projection/domain is frozen in DEV-030/ADR-0013. No guessed/all-scenario/TargetSet
fallback. S053 expected logical manifests precede results, AND by default,
protected frozen alternatives only, broad selectors need explicit approved sets.
S054 complete identity binds graph/profile/scope/expectations; S055–S058 inventory/
strengthening/query rules and all S059–S076 evidence/count/retry/campaign/freshness/
waiver/participant/reference-evaluator contracts remain exactly as approved.
Continue all26 checkpoints; no correction-only handback after a green repair.
Q05 missing live business values do not block pure configurable semantics or
local/synthetic automation. All original external acceptance gates remain explicit.

## Historical DEV-030 — Close checked scope boundaries and complete P04/P05

**Date/author:** 2026-10-05, Codex under owner delegation.
**Reviewed candidate:** `7a5b2465d293f482c22f2e8e4271a6b1507915c2`.
[Independent review](reports/CODEX-7a5b246.md) retains all ten commits and all
43 earlier numbered acceptances; accepts P04-R1/RF70 and allocated S045, S046,
S050, S051 (47 numbered steps total). S044/S047/S048/S049 are partial and
changes-required; inherited risk/post-expansion validation from S049 is retained.
No whole acceptance criterion or regression is closed by these subsets.
RF72–RF75 below are software repairs. RF76 ledger reconciliation is documentary.
The prior missing-run observation is historical; this candidate is matched.
All original 64 R/AC, 21 RA, Q01–Q15 and S211 requirements remain binding.
DEV-030 supersedes the execution frontier, retaining DEV-028's P05 decisions.

**Batch:** `pfc through EQM-P05`: P04-R2 → P04-R3 → S052–S076 inclusive,
27 green checkpoints. Repairs precede the remaining 25 implementation steps.
No stop at S058/S059, a commit, a progress report or a long conversation.
S076 is the independent complete-universe/reference-evaluator review boundary
before S077 storage integration. Original dependencies, cycle admission and
verification remain. External inputs block only their genuinely dependent work;
synthetic pure semantics, local integration and available automation remain
eligible. Pi reports implementation/evidence; Codex owns independent acceptance.

### P04-R2 — Close graph, denominator and participation admission

Commit `fix: enforce checked graph and complete scope admission`.
**Owns:** RF72/RF73/RF75; remaining S047/S048/S049 and RA01/RA05/RA06 contributions.

1. Move the opaque `FinalizedWorkspaceGraph` authority token to `eqm_engine`;
   private construction is reachable only after engine resolution/expansion and
   complete invariant validation. Remove the domain's public `from_engine`
   bypass; `doc(hidden)` is not privacy. No compatibility alias or unchecked
   promotion. Pure manifest projections consume `&WorkspaceGraph` as data,
   never authority; authorizing application/engine operations consume the engine
   checked token. Callers project its immutable graph. Do not introduce a
   production manifest→engine/protocol dependency or engine serialization.
2. Remove public `Default`/unchecked construction of authorizing
   `RequiredUniverse`. Derive it from the same checked graph, exact selection,
   applicability and immutable scope context. Bind those inputs so status maps,
   selections and universes from distinct sessions/contexts cannot be mixed.
   Inspect and close other public promotion paths with the same authority flaw.
3. Resolve completeness over the complete relevant universe: target, exact
   provider dependency, current target-set and later scenario coordinates.
   Missing coordinates reject or remain explicit Missing/Unknown; unknown
   applicability/participation cannot yield Conformant, even when every reported
   status passes. Preserve coordinate identity for unresolved outcomes, including
   exact subject/profile/context, rather than one requirement-only marker.
   Explicit false alone supports N/A; a caller cannot assert it to delete work.
   Empty authorizing scope still requires protected explicit permission.
   Keep the full denominator visible before evidence filtering or pagination.
   Preserve original required/advisory policy classification; an unmatched optional
   warning does not become a newly invented mandatory obligation.
4. Consume existing `evaluate_participation` for each exact requirement/target/
   selected-profile coordinate and combine its truth value with requirement
   applicability. Preserve its established no-rule→Unknown and ambiguity→error
   contracts in every mode; no global inclusion fallback. Explicit false is
   visible N/A; unknown stays non-authorizing. Evaluate all declared axes, never
   just the first profile. Existing synthetic examples may gain explicit fixture
   participation declarations where intended; do not fabricate live authority.
5. Migrate actual authorizing callers away from freely asserted completeness.
   Durable tests must include omitted provider/target-set, unknown applicability,
   unknown/asymmetric participation, forged empty universe, invalid raw graph,
   cross-session selection/context and unrelated coordinate controls. Include
   compile-fail/public-boundary proof that invalid finalized tokens cannot be
   constructed externally and real application/CLI admission regressions.

**Green/verification:** exact existing semantics and accepted risk fixes retained;
no incomplete coordinate can authorize. Run original S044/S047/S048/S049 lanes,
all-target domain/engine/manifest/application/CLI tests, owning doctests, standard
check, schema parity, structure validator and diff check. Repair goldens only
for reviewed semantic changes; retain all negative assertions and disclose them.

### P04-R3 — Complete shared exact request preparation

Commit `refactor: centralize exact selection request preparation`.
**Owns:** RF74 and remaining S044; RF76 evidence/progress hygiene; RF77 rustdoc links.
Move semantic policy/profile authority lookup, exact revision/ambiguity and
membership resolution from CLI `development_request`/resolver helpers into the
shared application operation. CLI parses syntax into typed inputs; MCP and other
adapters use the same immutable-session preparation. Modes/authority are explicit,
protected requests cannot use development defaults, and no reload/latest lookup/
clock/runner effects occur. Preserve duplicate rejection and exact revisions
accepted in S045/S046. Test identical CLI/application results, ambiguous/missing
revisions, duplicate identical/conflicting requests and protected-mode negatives.
Update actual usage docs/help for `--policy <ID[@REVISION]>` and profile revisions;
examples must distinguish unverified inputs from trusted authority. Backfill
actual commits, preserve failed attempt evidence and separate implemented,
locally verified and independently accepted states. Use distinct per-attempt log
filenames; overwritten logs cannot retroactively prove their failed attempts.
Repair the existing rustdoc links in application source observations (`clean`)
and protocol positive numeric schema (`unsigned_schema`) without changing semantics.
Run original S044–S046 lanes, owning doctests, standard/schema/structure/diff gates.

### S052 representation decision — exact scope bindings

This resolves the implementation agent's scenario-composition blocker.
Use existing domain scenario/provider/target records, not a new product service
or a change to v1 requirement bytes. Add typed domain `ScenarioRef` (ID plus exact
revision) and `RequirementScenarioBindings` mapping fully qualified requirement
IDs to exact scenario refs. An application preparation request carries the map;
protected modes obtain it from their explicit protected authority input, never
candidate claims. Development supplies explicit input. Add thin repeatable CLI
syntax `--scenario <FULL_REQUIREMENT=SCENARIO_ID@REVISION>` where evaluation is
prepared and document it. Duplicate entries (even identical), unknown/missing/
ambiguous refs, incompatible non-EndToEnd requirements and revision mismatch
reject. No active EndToEnd mapping means explicit incomplete/unknown admission,
never an implicit TargetSet or guessed/all-scenarios selection. An unrelated
extra binding must not enlarge or authorize the selected scope.

Add an exact scenario scope retaining ID/revision, occurrence identities, roles,
tagged target/provider subjects and dependency edges from the same checked graph.
Repeated actors of the same target stay distinct. Reuse checked cohort/subject
manifest inputs for concrete build/artifact/configuration assignments as the
owning later steps integrate them; no fabricated configurations. Provider
participation derives from existing `Target.providers` and checked `depends_on`
closure, never all targets indiscriminately. Missing provider assignment or
required scope authority remains explicit incomplete/non-authorizing.

Freeze a pure canonical scope-binding projection rooted exactly at `bindings`,
a canonical set of `{requirement, scenario: {id, revision}}`; use existing v2
NFC/numeric/JCS and duplicate rules and digest domain
`eqm:v2:requirement-scenario-bindings`. Give it a distinct private digest carrier.
Bind the map and scenario authority in S054 obligation/evaluation identity and
later evidence context. Do not silently expand the frozen contract, policy-
selection or graph projection roots, relabel v1 digests or add a legacy reader.
If a public DTO is introduced, its v2 schema/readers/validators must land atomically
with literal independent digest and cross-layer tests. Application owns effects;
domain/engine remain pure, manifest owns canonical representations.

**S052 green:** all approved scope kinds and exact dependers are in the frozen
required universe; omitted provider/end-to-end evidence cannot pass via per-target
success; repeated actor and wrong-scenario/revision controls execute through real
preparation. Expand S052's file allocation to domain/application/manifest/CLI and
usage docs, and run their all-target tests plus original engine/standard/schema/
structure gates. This is an allocation clarification, not a new backend feature.

A mapped active scenario with no actor or assigned provider depender in the
selected cohort is incomplete scope, not an obligation invisible to every
required target. Retain its full occurrences/edges and emit MissingScopeAuthority;
this closes the existing non-empty exact denominator criterion. It does not
enlarge the selected cohort or invent actor participation.

Shared-provider affected membership includes True/Unknown actual dependers,
not an explicitly False depender merely because a different depender is Unknown.
Unknown requirement applicability combined with known False coordinate
participation is visible NotApplicable under the existing three-valued AND.
All-False exact provider/scenario participation is N/A; mixed scenario
participation remains unresolved with full actors retained. Missing scope never
uses these rules to create an implicit subject. An unknown requirement with
missing scope is still MissingScopeAuthority, including an exact outside-only
scenario or an unassigned provider; empty affected membership cannot hide it.

### S053 and remaining P04/P05 decisions

Use typed `ExpectedSelectionManifest` input frozen independently of results,
keyed to exact binding/specification/obligation scope, logical case IDs and the
kind's count unit. Reuse existing LogicalCaseId/CampaignId/PlanId carriers.
Required selections compose with AND; only explicit protected alternatives
frozen before evidence may choose. A selector determines a singleton only if
its declared semantics uniquely identify that logical case. Globs/filters and
multiple selections require an explicit approved expected manifest; observed
passing results cannot define it. Missing/ambiguous expectation is Unknown/
incomplete, not success. Synthetic injected manifests prove pure semantics,
not product values. Include exact manifest digest in S054 and later lineage.

**DEV-035 S053 implementation allocation:** domain manifests are explicit claims,
not trust constructors. Retain exact binding ID/revision/component digest and
binding-local specification ID/component digest, kind/count unit, plan/campaign,
required logical IDs and explicitly preselected alternative groups. Reject empty,
duplicate or kind-incompatible selections. Never interpret SelectorText as a
LogicalCaseId or discover expected IDs from observed outcomes.

The application validates component hashes against the same immutable prepared
graph, specification requirement/facet coverage and checked scope membership.
Protected expectations come from a separate configured expected-selection port;
candidate claims must equal that immutable provider response. Load it once; no
clock, execution, graph reload or candidate fallback. Development expectations
are explicit non-protected claims. Engine binding retains the complete map keyed
by pre-identity ObligationKey in the private universe context; omission remains
visible incomplete. Do not include a final ObligationDigest in the manifest,
avoiding the S054 manifest/obligation identity cycle. Manifest serialization owns
a distinct `eqm:v2:expected-selection-manifest` domain, separate from plan/spec.

Development claims containing any chosen alternative reject with a typed
UnprotectedAlternative error; ordinary required AND claims remain supported.
Only the configured protected graph and expected-selection operations may admit
a chosen alternative. Domain/pure projection constructors remain claims.

Protected preparation additionally requires a private application marker set only
after the configured graph-provider operation succeeds; caller origin/mode tags
are insufficient. Count units and selected plan/campaign are uniform within one
coordinate. Coverage requires every binding/specification association, but count
credit uses distinct logical IDs, never multiple declarations of the same unit.
Prior Unknown coverage dominates a missing expected member and cannot become
waiver-eligible Missing.

An expected-aware evaluation operation checks exact plan/campaign/component
lineage, selected-case completeness and distinct declared units before admitting
prepared facet facts. Missing/unselected/foreign cases cannot make a coordinate
Satisfied; ordinary status evaluation on an expectation-bound universe cannot
bypass this check. Observation claims still require owning evidence admission,
trust and attempt/outcome validation in S059–S068. Keep original legacy consumer
controls and track remaining transport integration under its owning steps; no
whole RA03/feature closure follows from synthetic scope/case fixtures. Extend the
S053 owning lane to manifest/application and doctests for the actual operation.

**DEV-035 S054 implementation allocation:** freeze a separate pre-evidence
coordinate projection under `eqm:v2:obligation` and a distinct typed
ObligationDigest. Domain input carriers are claims; only the application creates
an opaque prepared identity map from one PreparedExpectedSelections universe and
its matching immutable finalized graph. Do not combine unrelated visible-key
maps, reselect policy/profiles, reload authority or capture time. Read-only engine
views expose the exact policy/profiles, mode/origin, release context and composed
obligations already bound into private context.

The root contains unit, requirement, facet, policy_selection (the existing exact
policy/profile definition and explicit Known/Unknown projection),
semantic_graph_digest, contract_digest, scope_bindings_digest,
participation_digests, subject, expected_selections, strength, mode and
authority_origin. Optional release_context is omitted when absent and contains
its exact digest when present; null is never an absence substitute. Bind every
strength axis with the existing v2 numeric forms. Manifest digest collections and
participation identities are canonical member-byte sets with duplicate rejection.
Expected selections explicitly distinguish Missing from Present; the application
cannot produce a present empty coordinate manifest. Pure projections do not
confer expected-selection authority.

Subjects have a closed target/provider/scenario tag. Targets bind the actual
TargetConfigurationDigest. Checked providers retain actual configuration identity,
all selected dependers (including explicit False), assignments, qualified directed
dependency edges, target configurations and participation states. Checked scenarios
retain the exact scenario reference/digest and every occurrence, role, tagged
subject and directed edge, plus selected targets, full provider dependencies,
configuration references and participation. Legacy Provider/TargetSet labels
cannot produce a complete v2 identity; reject them without lossy fallback.

This is a complete pre-evidence coordinate reference, not an independently checked
candidate build/source subject or release authorization. Full candidate evaluation
identity separately binds checked SubjectManifest inputs at S061/S132. Do not
select graph cohorts implicitly, substitute graph/configuration hashes for a
SubjectManifestDigest or claim whole R19 closure from this checkpoint. Evidence
outcomes, current status, host time, observed files and presentation filters never
enter the coordinate identity.

Prepare the full identity map before filtering and use the explicit
`eqm:v2:obligation:` reference prefix in all existing obligations/context/check/
affected/matrix references. Preserve frozen v1 DTO shapes/schema bytes. Legacy
Known-only profile presentation continues to reject unknown values truthfully;
S058 owns explicit v2 complete/unresolved query DTOs, never drop or coerce Unknown.
Add independent literal JCS/domain hash controls, every identity-axis substitution,
permutation invariance and same-ingress production query reference/filter controls.
Expand the owning S054 lane to domain/manifest/engine in addition to the original
application/CLI/protocol tests; original commands and criteria remain mandatory.

**DEV-035 S055 implementation allocation:** application prepares read-only
structural observations and reports for the complete finalized session graph
before transport filters, using each target's exact owned source-root closure.
The pure engine distinguishes unavailable role observation from an independently
observed empty/mismatching role set; unavailable authority is Unknown and cannot
satisfy structure. A configured read-only role observer receives only independently
inspected path/kind information, never a Binding or its declared role. Match the
returned path exactly; absent, failed or foreign role observations remain Unknown.
No filename/extension inference or default observer is permitted. The CLI has no
configured role observer and must expose Unknown for present unverified artifacts.
This port is independent observation, not imported verified flags or release trust;
later evidence/trust admission retains its original requirements.

Inspect every repository-relative path component without following symlink
ancestors or forbidden leaf symlinks. Reject special non-file kinds, check canonical
root containment and compare metadata before/after an observer call; detected
changes or inspection errors are non-authorizing, never reported as absence.
Only actual NotFound may produce a local physical missing-file finding. Do not
claim pathname metadata checks prove race-safe immutable acquisition: no payload
is opened/executed here, and stronger acquisition/storage confinement remains
with its owning steps. Bound complete path preparation visibly without truncation.
Keep source-component ownership positive/negative controls; add actual application
and CLI causal controls for missing observer, independently matched/mismatched
roles, foreign response, changed files, special files and symlink ancestors.
Review intentional report/golden changes without hiding structural Unknown or
weakening existing missing-scope controls. Keep frozen schemas and dependencies
unchanged. Original S055 owning commands remain mandatory. This is an allocated
RA21 contribution; inventory/currentness/trust/exposure integration is not closed.

**DEV-035 S056 implementation allocation:** raw domain/runner inventories are
validated claims and cannot independently prove presence or absence. Extract a
clearly named locked-adapter claim-validation operation for the CLI, without a
self-echo expected-subject request. Preserve exact legacy wire validity and all
syntax/digest/limit/ordering checks. Raw reconciliation remains Unknown.

Application owns opaque PreparedInventoryFacts with no deserialization or public
success constructor. Preparation takes immutable original inventory bytes and
their validated claim, independently supplied exact subject/target/adapter identity,
explicit evaluation instant, configured maximum age/future skew/minimum trust and
exact trust-configuration digest, and an explicit read-only InventoryVerifier port.
Bound raw bytes before parsing/copying; strict reparsing must equal the validated
claim. Independently compare every subject/adapter/target axis and exact nanosecond
age/skew before invoking the configured verifier once. Missing inputs, stale/future
claims, content substitution or unavailable/rejected verification remain Unknown
with retained reasons and provenance. Verifier success means independently checked
signature, identity/issuer/producer/builder/execution eligibility, exact bytes and
subject, time/revocation and configured trust; no public producer verified flag or
deserialized receipt is accepted. There is no default verifier or hidden fetching,
execution, clock capture or production test stand-in. A Result-only capability
response grants no transferable receipt; facts remain bound to this evaluation.

Authoritative legacy target inventory additionally requires independently expected
build/artifact identity and target scope matching the target. Missing axes or lossy
provider/target-set scope stay Unknown; this does not narrow legacy wire parsing or
replace later exact v2 subject-manifest/cohort/profile/source-component integration.
Verified current present entries may establish presence; only verified current
Complete inventory establishes missing entries as absent. Partial/Unknown missing
coordinates remain Unknown. The real application reconciliation consumes these
private prepared facts, retaining snapshot time, raw identity and policy context.
CLI has no independent subject/policy/verifier configured today, so loaded claims
remain Unknown and cannot be labelled current or authorize absence.

Require same-ingress checked positive presence/complete-absence controls and
one-axis negatives for missing inputs, partial absence, stale/future boundaries
(including one nanosecond), byte/subject/adapter substitutions and revoked/rejected
verification. Keep synthetic verifier implementations cfg(test) only. Add actual
CLI complete/present inventory controls preserving Unknown and no writes. Real
cryptographic implementation remains S116–S131; no whole RA21/R/AC closure follows.
Original S056 gates remain mandatory; domain/CLI tests join the declared owning lane
because their consuming authority shortcuts change. No schema/dependency changes.

**DEV-035 S057 implementation allocation:** repair the selection-floor ordering
at its domain source: lowering the floor includes more requirement levels;
raising it may remove protected work and is not strengthening. Correct existing
positive fixtures that endorsed the reversed direction. Requirement severity
itself retains its existing increasing order. No authored requirement is lowered.

Add a pure checked required-universe comparison over actual opaque derivations.
Require exact immutable finalized graph authority, mode/origin, selected profiles
and values, applicability, scenario map, release context and expected-selection
map. Different graph replacements remain explicitly incomparable; their protected
integration stays with the owning later steps. Policy revisions may differ only
after a checked transition between exact definitions in that same graph: same
policy ID, preserved profiles/targets, no weaker waiver/rule constraints and
unchanged unsupported policy metadata. Do not strip revisions before that gate.
Compare exact semantic requirement/unit/subject/profile/facet/release coordinates
and per-facet trust/count/age. The selection floor is coverage, not a replacement
for evidence strength. Retain every baseline visible unresolved coordinate and
reason, except UnmatchedPolicy may become nonempty actual candidate-derived
obligations at the same exact visible coordinate after the checked transition.
Unknown, N/A and missing-scope reasons cannot be dropped or substituted.
The legacy subset helper must reject incomparable context and include unresolved
visibility and composed evidence strength rather than only key-set inclusion.

Replace tautological seeded hashing/set tests with real resolution, expansion,
selection and derivation over independently enumerated expected coordinates and
constraints. Permute only semantically unordered inputs; keep journey sequences.
Cover explicit unknown/false and empty scopes, advisory-versus-required unmatched
work, protected candidate omissions/context substitutions, Recommended/Required
floor deletion and legitimate advisory promotion, independent facet constraints,
and inherited fragment/journey risk through final policy matching with literal
risk tables and weaker-override rejection. No test-only production constructor,
implicit authority, schema or dependency change. Separate allocated review and
original S057 standard/engine owning gates remain mandatory; domain owning tests
join because the ordering source changes. These contribute to RA04/RA05/RA07;
whole regression and complete-evaluator acceptance remain open, with mandatory
independent whole-universe/reference-evaluator review before S077.

**DEV-035 S064 implementation allocation:** retain all immutable matched attempts
for each selected logical case in the exact frozen plan/campaign. Engine history
construction validates and internally collects the same complete expectations and
candidates through S063; no reusable externally supplied report or latest-result
selector is accepted. Application binds opaque S054 identities before adapting
actual S059 admissions. Preserve exact matched semantic assertions before S061
conflict partition, including their full subject, five lineage identities and raw/
mapping/provenance audit; rejected assertions sharing an attempt never enter the
eligible history. Existing global cross-source immutable conflicts stay Unknown.

Group retry outcomes by full execution subject and logical case within the selected
campaign, across required sources without multiplying count units. Different
required cases or subjects compose conjunctively: a fail-only group plus a passing
group is Failed. Failing/passing immutable retries of the same full-subject case
are Unstable. Contradictory same-attempt assertions, timeout, cancellation and
execution errors are Unknown. Skips, filters, quarantine and absent required
source/case members remain Missing. Preserve every contributing typed reason and
all immutable histories irrespective of summary precedence. Duplicate transport,
input ordering, newer passing times and unrelated campaign/subject claims never
erase relevant history or poison the selected cohort.

This step evaluates terminal retry history for executable payloads, with a pure
domain terminal-outcome aggregator; it does not mint independent successful count
credit, trust, facts or conformance. All non-executable kinds retain explicit
pending kind evaluation and existing independent prerequisites, never automatic
success. S065 manual review, S066 complete executable selection/count eligibility,
S067 inventory/runtime facts, S068 campaign relations and S073 protected consumers
remain their owning work.
Retain S063 full structural capacity and AND/conflict audit separately from retry
history. Charge every incoming executable payload across all expected sources
before normalization (including rejected/duplicate transport), and retained
attempt/outcome traversal cumulatively to the existing
1,000,000 work bound, with explicit error and no partial conclusion; retain delivery
and full frozen-context bounds.

Use real prepared graph/policy/spec/frozen selection and actual admitted bytes for
literal fail/pass retries, separate fail/pass cases, timeout/cancel/error, skipped/
filtered/quarantined/missing cases, same-attempt contradiction, duplicates, reverse
order, foreign campaign/component/subject/identity and cumulative bound controls.
Keep precise full-subject and source/case histories, rejected audit and no-effects
snapshots. No wire, schema, digest, dependency or legacy helper reinterpretation.
Original domain/engine owning and full standard gates plus application owning and
mandatory separate allocated acceptance are required. RA02/RA03 remain local
contributions. Continue S065 after the green commit and retain the mandatory
independent complete-universe/reference-evaluator gate before S077.

**DEV-035 S063 implementation allocation:** apply the actual prepared coordinate's
composed count minimum to the S062 distinct typed observation report. Obtain the
minimum from the immutable RequiredUniverse and exact graph specifications, never
an arbitrary caller threshold, raw totals or runner acquisition success. Retain the
policy minimum, exact source acquisition minima and strongest effective minimum
for audit. Match the complete coordinate and all frozen source manifests; missing,
extra, altered or duplicate expectations and foreign observation context reject.
Application binds every expectation to the opaque S054 pre-result identities and
full frozen selections before adapting actual S059 admissions through S062. The
engine count operation collects internally from that same validated complete
expectation slice; a separately supplied observation report cannot be reused.

This step supplies a pure count constraint, not independent successful credit.
Return explicit structural capacity met, Missing or Unknown and preserve the full
S062 report with its missing AND pairs, conflicts, rejected history and independent
prerequisites. Enough units never hides a missing required source/case or relevant
immutable conflict. All seven count units remain explicit; exact referenced-record
capacity is at most one. A met structural threshold never becomes Satisfied,
verified trust, conformance, protected authority or release authorization. Required
case outcomes and retry/principal/campaign eligibility remain S064–S068; S073 binds
those results into the protected complete consumer. Existing claim-only helpers
remain claims and cannot consume this assessment as verified authority.

Use real prepared graph/policy/binding/specification/frozen selections and actual
admitted bytes. Independently enumerate exact threshold boundaries, stronger policy
versus lower specification acquisition minimum, stronger relevant specification,
facet-rule isolation, copies/retries, missing AND members despite enough other
units, immutable conflicts and all seven typed units including record aliases.
Reject a substituted policy/profile/identity, omitted or altered frozen expectation
and lower caller threshold; no caller threshold parameter is exposed. Retain pure
bounds, permutations, exact literals and no-effects snapshots. Count-context
validation also charges all selected/unselected alternative definitions and
complete expectation contexts to a cumulative 1,000,000-work bound before indexed
manifest equality; retain the S062 delivery/matching work limits separately. No wire, schema,
digest or dependency change. Original engine owning and full standard gates remain
mandatory; add application owning and separate allocated acceptance. RA02/RA03
contributions remain local until outcomes and protected consumer integration.
Continue S064 after the green commit, and preserve the mandatory independent whole
complete-universe/reference-evaluator gate before S077.

**DEV-035 S062 implementation allocation:** derive distinct structural logical
observations from the S061 complete matcher, never caller-supplied count totals
or standalone SelectionObservation claims. One bounded pure operation consumes
all pre-result expectations for an exact coordinate/identity and the complete
normalized delivery input. Require compatible count units, selected plan/campaign,
complete grouped subjects and unique exact binding/local-specification sources;
inconsistent or empty expectations fail explicitly before partial output.

Each expected source composes with AND. Retain missing selected source/case pairs,
matched/rejected/conflicting assertions and every original delivery reference.
Deduplicate logical case IDs at the shared coordinate across local specifications;
repeated transports, split reports, run IDs and retry attempts add no logical
units. Relevant immutable conflicts remain Unknown, even when other cases match.
Unrelated rejected history remains auditable without creating units or poisoning
compatible matching. Frozen alternatives come from the existing expected manifest,
never observed outcomes. Count thresholds remain S063; required-case outcomes,
retry semantics and campaign authorization remain S064–S068.

Handle all seven declared count units explicitly and retain their independently
required outcome/fact/principal/referenced-record checks. A structurally observed
logical unit is not successful, independently verified credit, protected authority
or conformance. In particular, inventory/runtime totals, claimed review principals
and release references cannot mint independently checked facts or verification.
The existing one-record selection bound stays intact. Protected frozen-input/status
integration remains S073. Preserve the 10,000-delivery bound and cumulatively
charge the existing 1,000,000 complete-context-work bound across all expectations
before repeated matching or report allocation. No wire/schema/digest/dependency
change. Application adapts actual opaque admissions through the shared operation.

Use real independently prepared graph/binding/specification/frozen selections and
actual admitted bytes: split selected cases, alternate raw encodings, ten retries,
missing AND cases, irrelevant components/campaigns, selected alternatives,
conflicting immutable attempts, duplicate case across local specifications and
all seven count units with explicit remaining verification prerequisites.
Literal expected sets and permutations are independent controls. Retain original
domain/engine owning and full standard gates; add application owning and separate
allocated acceptance. RA02/RA03 contributions remain local until their later
outcome and protected consumer integration. Continue S063 after the green commit.

**DEV-035 S061 implementation allocation:** strengthen the existing structural
expectation to match every EvidenceSubject field and exact runtime-facts reference,
with distinct rejected-axis diagnostics. Add a complete pure matcher over explicit
data: the structural expectation, exact ObligationKey and pre-result
ObligationDigest, frozen ExpectedSelectionManifest, and (for checked grouped
scopes) the complete SubjectManifest with its role-bearing participant mapping.
Candidate coordinates/manifests remain claims; missing context is rejected, never
filled from expected values. Legacy lossy provider/target-set scopes cannot prove
complete eligibility. Grouped subjects require complete source/build/artifact and
exact occurrence/configuration-kind association with the checked scope.

Match exact plan/campaign and selected logical case before considering immutable
attempt conflicts. Distinct run/attempt IDs remain history, not extra observation
credit; S062 owns counting, S064–S068 outcomes and authorized campaign relations.
Return complete deterministic accepted/rejected structural references and relevant
conflicts, retaining S060 raw/mapping/provenance records. Preserve the inclusive
10,000-delivery limit and add a 1,000,000-slot cumulative complete-context work
budget, charged before matching or normalization allocation; overflow fails
explicitly. Occurrence associations use indexed lookup rather than quadratic scans.
No parsed context tag, digest, public struct or matching status grants independent
verification, protected provenance, conformance or release authority. Application
adapts actual opaque S059 admissions with explicit context claims through this one
matcher; protected frozen-input/status integration remains in its original owning
steps, including S073. No wire/schema/digest-domain/dependency change.

Use independently prepared graph/component/obligation/selection expectations and
actual admitted bytes for causal build/spec/profile/runner/context controls.
Exercise every subject and lineage axis, role/occurrence/manifest substitutions,
typed absence, missing context, unrelated rejected candidates, duplicate delivery,
relevant conflicts, permutations and bounds. Retain the original engine owning
and full standard gates; add application owning coverage for its adapter and
separate independent allocated acceptance. RA03/RA08/RA13 receive local eligibility
contributions only; no whole regression or MVP closure. Continue S062 when green.

**DEV-035 S060 implementation allocation:** repair pure coverage's duplicate-ID
poisoning: repeated equal records remain one compatible assertion; only different
matching records claiming one immutable result identity are Unknown. Rejected
unrelated duplicates remain diagnostics and cannot poison compatible coverage.
Preserve duplicate-ID diagnostics and add explicit conflicting-result diagnostics.

Engine adds bounded pure normalization of admitted candidate data, keyed by the
complete typed EvidenceSubject and all five explicit EvidenceLineage identities.
An attempt retains every distinct SemanticEvidenceDigest, exact RawObjectDigest,
local EvidenceSpecId and explicit SelectorText provenance reference. Identical
delivery and alternate encodings share one assertion; different logical cases,
runs, campaigns, plans or subjects remain independent. Contradictory assertions
for one attempt remain visible; no last-writer-wins. Deterministic pure ordering
uses additive Ord on existing subject/scope/lineage values, not wire canonicalization.
Validate semantic/result identity agreement and reject conflicting domain data
for one semantic identity. Bound input deliveries at the existing 10,000-record
application read limit; overflow fails explicitly without truncation.

Application consumes actual opaque AdmittedEvidenceV2 plus explicit mapping and
provenance references, then calls this engine normalizer without I/O or trust.
Protocol retains the already-computed raw identity alongside original bytes and
exposes a typed accessor, with no wire/schema change. A normalized coverage view
matches the existing structural expectation first; only multiple compatible
assertions for one immutable attempt cause Unknown. Rejected assertions remain
auditable. This is structural data, never authentication or conformance; S061
still owns complete-coordinate/eligible-input integration and later trust/storage
steps independently authenticate inputs and persist conflicts across restart.

Use actual S059 checked application admission, prepared component projections,
literal raw/provenance sets, identical/alternate transport, pass/fail same-attempt
contradiction, independent cases within one run, every lineage/subject separation,
permutation/idempotency, irrelevant rejection and inclusive bound controls.
Preserve original engine/application owning, standard and independent acceptance
gates. RA03/RA08/RA15 receive local normalization contributions only; no whole
regression, storage or outcome closure. Continue S061 after the green commit.

**DEV-035 S059 implementation allocation:** reuse the existing protocol
`admit_evidence` as the sole DTO-to-domain converter. Application adds the pure
`admit_evidence_bytes` entry point: strict explicit v2 parsing of exact bounded
bytes, then that converter with independently supplied EvidenceAdmissionContext,
EvidenceLineage and EvidenceExecution. Return its opaque AdmittedEvidenceV2 and
preserve exact originals and semantic identity; typed EvidenceConversionError
distinguishes strict decoding from relationship admission with causal errors.
No I/O, clock, reader, executor or verifier capability is accepted or invoked.
Parsed claimed trust remains a claim. Existing v1 readers/domain carriers and
direct protocol admission remain unchanged; no new wire/schema/dependency.

Use real prepared manifest/component projections in application conversion
controls for all seven currently supported payload kinds, with actual
kind-compatible evidence definitions. Check direct protocol/application parity,
alternate transport with identical semantic identity and original bytes,
deterministic one-axis kind/subject/target/unit/requirement/facet/component/profile/
runtime/release reference mismatches, invalid identity, exact lineage/retry time
and arithmetic-valid count/outcome contradictions for all executable kinds.
Strict malformed/unsupported-version/over-bound bytes fail before admission.
Missing expected selection/campaign integration remains assigned to S061–S068;
do not invent trust, storage, duplicate classification or final outcome semantics
here. This is S059's RA02/RA13 contribution, not whole regression closure.
Original protocol/domain/application owning, full standard and independent
allocated acceptance gates remain; record actual failures and continue S060
only after the local green checkpoint.

**DEV-035 S058 implementation allocation:** project the existing opaque
PreparedObligationIdentities and its complete expected-bound RequiredUniverse
through one pure application query boundary. Accept the same finalized graph and
reject foreign graph context before projection; do not reconstruct membership
from filters, evidence or adapter-supplied coordinates. Manifest may expose its
existing pure exact scope projection for unresolved records without changing any
frozen obligation identity bytes or digest domains.

Freeze `ObligationsReportV2Dto` under the existing versioned Result SchemaKind:
`schemas/v2/protocol/result.schema.json`. This new closed success/error family
uses exact schema/tool/obligations-command markers and an `outcome` discriminator;
never place it under the v1 envelope or change v1 bytes/readers. Success contains
explicit `context` and `result`; error contains a bounded typed `error` with a
stable closed code and safe causal message. Preserve established execution-error
exit 6. Scope this JSON execution-error integration to obligations; other command
families and pre-format parser/usage errors remain with their later owning steps.

Context retains canonical semantic graph and scope-binding digests, exact policy
ID/revision, selected profile IDs/revisions and tagged known/unknown dimensions,
mode, authority origin, optional immutable release context, offline flag and the
session's single captured evaluation time in canonical nanosecond UTC form.
Required records carry `kind:required`, `state:pending`, the prepared v2 reference
and a closed typed `coordinate` matching the complete existing canonical identity
projection. All coordinate axes, per-facet constraints, expected selections and
exact target/provider/scenario configuration/dependency/participation context
stay visible, including repeated role-bearing actors. Decode and validate that
projection explicitly; no opaque unrestricted semantic map or label fallback.
Unresolved records carry `kind:unresolved`, their exact unit/requirement/profiles,
optional exact subject/release context and explicit reason. Only actual
NotApplicable has `state:not_applicable`; all other retained reasons have
`state:unknown`. Missing scope remains absent rather than fabricated. Neither
record form invents evidence satisfaction, IDs, waivers, verifier facts or a final
release status. A coordinate reference remains an identity, never trust.

Freeze typed presentation `filters` (optional unit/target and canonical status
set), pre-filter `denominator.required` and `denominator.unresolved` counts,
`returned_count`, `presentation_filtered` and canonical duplicate-free `records`.
Unresolved count includes justified N/A; every unresolved reason stays in the
pre-filter denominator. Status filter vocabulary is pending/unknown/not_applicable;
unsupported evaluated-status values fail explicitly, with no v1 alias. A target
filter retains missing/unknown-subject records whose membership cannot be
excluded; known subjects use exact checked participation. Unit/status/target
filters affect presentation only and never mutate the retained universe, identities
or denominator. This pre-evidence view is not a persisted evaluated snapshot.

Register the real v2 schema and generator atomically. Strict readers and writers
reject wrong versions/markers, unknown fields, duplicates, malformed identifiers,
noncanonical integer/time/absence/set forms, inconsistent reference/context/counts
and unsupported semantic variants. Bound complete serialized envelopes at 16 MiB
and visible records at 100,000; exceeded bounds produce a bounded non-authorizing
error, never silent truncation. Reuse existing wire/domain validation and pinned
workspace schema-validator tooling; no new production dependency or unsafe code.
Document the v2 transition and exact filter/error behavior and update actual
obligations goldens/consumers in the same checkpoint.

Required controls use real graph/selection/scope/identity preparation, literal
complete coordinates and every unresolved reason, unknown dimensions, exact
provider and repeated-actor scenario context, foreign graph rejection and
filtered/unfiltered denominator/reference invariance. Validate real parsed CLI
success and execution-failure outputs against the generated schema, with strict
negative/representational parity controls. Assert no evidence reads, runner or
verifier invocation and no workspace writes. Preserve original S058 schema,
protocol/application/CLI owning and full standard gates; run all applicable P04
cycle lanes and changed doctests, record unavailable real product inputs without
fabricating thresholds. Separate allocated source acceptance remains required.
This is RA14's local contribution; no whole RA/R/AC closure or S047 aggregation
closure follows. Continue S059 when green and retain the mandatory independent
whole-universe/reference-evaluator acceptance before S077.

All other S054–S076 decisions from DEV-028 and original step/spec/RA contracts
remain. No new supervisor stop between P04 and P05. Q05's unavailable real
policy values do not block configurable semantics and local negative automation;
never invent their values or equate synthetic trust with protected acceptance.

## Historical DEV-029 execution observation

No matching latest returned instruction/run was located in the available native
session stores. The checkout is clean at exact issued planning revision
`3124cc516baac87b27db3a6e47b06f5590ea0342`; no source/test/schema/lock delta from
accepted `1496c286`, and no later implementation commit. No new acceptance.
Preserve43 numbered local acceptances and all existing findings/external gates.
DEV-028 remains the sole technical amendment. All34 checkpoints remain pending:
P04-R1 then S044–S076 inclusive, `pfc through EQM-P05`, complete P04 and P05 before
independent reference-evaluator review/S077. Reconcile and preserve any work found
elsewhere before repeating implementation. The next run must record its exclusive
external startup receipt before code so actual repository/revision/runtime and
available native identity are discoverable. A missing returned log does not
change scope or block already-approved pure software work. Original64R/AC21RA,
Q01–Q15,S211 unchanged; whole RA02/RA13 and platform/live gates remain open.
This is execution evidence revalidation, not a new technical requirements plan.


## Historical DEV-028 — Accept local P03 and implement complete selection/evaluation

**Date/author:** 2026-10-05, Codex under owner delegation.
**Reviewed candidate:** `1496c28616a769b9c958965c0377cb3b83ded32e`.
[Independent review](reports/CODEX-1496c28.md) accepts allocated RF68/RF69 and
S043/P03 local scope, preserving 42 earlier numbered acceptances (43 total).
Direct actual-owner isolated probes supplement the committed numeric predicate
proxies. RF70 requires durable constructor-based coverage; RF71 stale counts are
corrected. RF45 P03 parity contribution accepted, remaining evidence-boundary
contribution stays S059/S061; whole RA02/RA13 remain open. Original 64 R/AC, 21 RA,
Q01–Q15,S211 unchanged. This amendment is retained under DEV-030; prior decisions
remain except superseded frontiers. Historical missing-run observations below
are superseded by the now matched completed candidate.

**Batch:** `pfc through EQM-P05`, P04-R1 then S044–S076 inclusive, 34 separate
checkpoints. Complete both P04 and P05; P04 completion alone is not a stop.
S043's required independent gate is now satisfied. Stop after S076 for independent
review of complete required-universe and reference-evaluator semantics before
S077 persistent storage integration. This is a coherent foundational integration
boundary, not permission to skip or merge steps. Pi records local verification;
Codex accepts candidate implementations. No intermediate supervisor gate is added
between S058 and S059; original step dependencies and cycle verification remain.

### P04-R1 — Preserve actual-owner scalar regression evidence

Commit `test: retain actual owning scalar boundary checks`.
Repair RF70 using actual PositiveCount::new, Revision::new, DurationMillis::new,
checked u32 conversion plus PositiveDays::new, and RunnerLimits::new with valid
other limits. Use separate exact-owner decisions for counts and revisions.
No copied predicates as owning proof. Retain existing literals and 196 evidence
combinations. Add all forbidden C0/C1 points (embedded and trailing), allowed
LF/TAB, empty, exact ASCII maximum and over-maximum for each text carrier; Unicode
NFC/UTF-8 byte limits remain separate explicit layers. Correct P03-R16/S043-R6
claims without deleting history. Run protocol corpus plus original standard,
schema, structure and diff gates; leave production scalar contracts unchanged.

### Decisions for S044–S058 (complete P04)

Application owns explicit mode and authority preparation; domain/engine stay pure
and unsafe-free. Reuse existing PolicyRef/ProfileRequest and checked graph/
canonical provenance carriers; expose exact IDs plus revisions as typed request
fields, keeping transport parsing thin. Development candidate authority is
explicit; protected modes reject missing/inappropriate authority and never take
a development default. Production trust is not inferred from parsed origin tags.
Duplicate profiles/dimensions/revision requests reject even when identical;
ambiguous/missing stored authority requires explicit selection, never latest.
Preserve explicit unknown values and graph-bound subjects/cohorts/configurations.

Enumerate complete required coordinates before selecting evidence. Unknown
applicability stays visible/non-authorizing; false may justify N/A. Fragment risk
is stronger(fragment default, journey risk), explicit lowering rejects; close
unchecked finalized/prepared construction per DEV-001. Empty release scope needs
protected explicit permission; one participant cannot establish equivalence.
Facet constraints compose independently: max count/trust, min age, required-
universe inclusion; stronger required-level floors can weaken by deleting cases.
Exact provider/scenario dependencies retain repeated actors. Required expected
logical selections use AND; only an explicit protected frozen alternative allows
choice, never pass-driven selection. Obligation identity includes exact profiles,
revisions, subjects and context. Authored roles are not observed structural facts;
only complete/current/verified inventory can prove absence. Property tests and
versioned query DTOs preserve original RA01/03/04/05/07/14/16/21 contributions.

### Decisions for S059–S076 (complete P05)

Use one checked DTO-to-domain admission and lineage boundary for every supported
kind, including all RA02/RA13 relationship checks. Parsing grants no trust. Bind
exact coordinates, subjects, component/specification identity, expected logical
selection and campaign before contributing evidence. Distinct logical observations
count; copies/files/retries do not. Release record contributes at most one unit.
Different-case fail/pass is Failed; same-case retry fail/pass is Unstable;
same immutable attempt contradiction is Unknown. Timeout/cancel is Unknown;
skipped/filtered/quarantined/missing are non-success. Campaign selection follows
explicit authorized manifests/supersession, never latest passing timestamps.
Manual review needs explicit complete review data. Executable payloads match
expected selection; inventory/runtime facts compare actual observed data.

Use explicit captured evaluation time and exact nanoseconds; policy supplies age
and skew, no hidden grace/default. Independent verification facts remain
non-authorizing until protected trust integration. Waivers contribute precisely
to allowed facets/cases, never make Unknown/Unstable successful. Preserve exact
facet reasons, complete participant conformance and contract-relative comparisons.
Build simple independent reference evaluation plus adversarial fixtures/property
checks; helper-only tests do not prove application consumer integration.

### Scope, gates and evidence

Read each original S044–S076 entry and referenced contracts/regression cases;
all listed criteria/tests remain binding. Q05 direction is approved; real product
thresholds remain unavailable. Implement explicit configuration and synthetic
positive/negative tests without inventing production values; their absence does
not block pure local semantics. Missing protected provider fails closed. No
production signer, host assertion or real product authority is fabricated.
Exact public additions must follow API/identity contracts and source-driven v2
schemas atomically; preserve v1 bytes/explicit readers. No alias/fallback, domain
loosening, trust promotion, implicit effects or later storage implementation.

One coherent green commit per checkpoint with original owning tests and full
standard gate; ledger/report updated before final gates. Retain distinct attempt
logs, exact commands/upstream exits and repaired failures. At S058 run all P04
cycle lanes then continue S059 when green. At S076 run P04/P05 owning integration,
doctests, standard/schema/structure/diff and declared reference/adversarial lanes.
A newly introduced failure blocks advancement; real external gates/user stops/
actual runtime limit may stop. No stop merely for a commit/report or long context.
Local automation is not inherently hardware-blocked. No publication, deployment,
signing, optional-native activation or changes to unrelated repositories.

## Historical execution revalidation — 2026-10-05

No matching latest returned instruction/run or subsequent product delta is
available for review. The checkout remains clean at the exact issued planning
revision `897c2cea00b8d1271e7508c95f8ba33cc943b2df`; product source, generated
schemas and lock are unchanged from reviewed `1198cc4`. No new acceptance.
DEV-026 remains the sole technical amendment: **P03-R14 → P03-R15 → P03-R16 →
S043-R6**, all four substantial numeric/text audit, differential corpus and
consumer-exit checkpoints remain pending. Preserve42 numbered acceptances and
allocated RF66/RF67 closure; RF68/RF69/RF45 and whole RA02/RA13 remain open as
previously recorded. Independent P03 acceptance still precedes S044. Reconcile
and preserve implementation located elsewhere before repeating or overwriting it.
Original64R/AC,21RA,Q01–Q15,S211 unchanged. This is execution revalidation only,
not a replacement technical plan or acceptance of an unidentified run.

## DEV-026 — Finish numeric and normalized-text owning contracts

**Date/author:** 2026-10-04, Codex under owner delegation.
**Reviewed candidate:** `1198cc4c3dfe6fea6443e0f4b1998a539d564d20`.
[Independent review](reports/CODEX-1198cc4.md) retains all four returned commits
and accepts RF66/RF67 allocated repairs, including additional existing local
combinations. Preserve42 numbered acceptances. RF68 PositiveDays range and RF69
forbidden control characters are independently reproduced. RF45 whole parity is
partial; S043/P03 remain changes-required, whole RA02/RA13 open. Original64R/AC,
21RA,Q01–Q15,S211 unchanged. This is the current amendment; retain DEV-025 and
earlier decisions except the superseded frontier. Ordinary engineering decisions
below are settled; no owner-only blocker or release/human-testing milestone.

Execute **P03-R14 → P03-R15 → P03-R16 → S043-R6**, **pfc through EQM-P03**.
All four broad owning-contract checkpoints, not only the two reproduced literals.
The mandatory independent P03 acceptance before S044 still justifies this boundary;
no later sequence dependency is waived. Continue after every green checkpoint.

### P03-R14 — Exact numeric ownership and range audit

Commit `fix: align graph numeric bounds with owning values`.
Repair RF68: maximum_days is PositiveDays/NonZeroU32, range1..4294967295;
4294967296 must reject. These values are representation limits, not production
waiver thresholds. For EVERY numeric member of introduced authored/graph/comparison
forms, map actual owning carrier, constructor, width, zero policy, canonical wire
forms and any local bound. Repair every faithful expressible omission found.
Do not apply the day bound to u64 revisions/counts/durations. Preserve ADR-0013's
number versus decimal-string transition, signed extension integers and strict
raw-token layer. Add literal minimum/maximum/over-boundary same-input owning/schema
controls and relevant real-loader/projection controls. Regenerate source-driven
schemas atomically, v1 unchanged; no generic fallback or domain parser loosening.

### P03-R15 — Existing normalized-text control policy

Commit `fix: enforce owning normalized text character constraints`.
Repair RF69 at all affected exact fields/helpers. Read actual Title/Description/
SelectorText/RouteSelector/RequirementStatement/TransitionTrigger/WaiverReason and
other relevant owning predicates. Faithfully express their forbidden-control
subset; preserve LF/TAB and valid NFC non-ASCII/length endpoints. Test C0 and C1
controls, including NUL/CR/DEL, with exact owning/schema decisions. Different
owning types may have different policies: never normalize raw input or apply the
rule globally to observed payloads, opaque bytes, diagnostics or extension values.
NFC and Unicode byte lengths stay explicit semantic layers where schema cannot
faithfully express them. Inspect canonical consumers and retain original semantics.

### P03-R16 — Complete differential scalar and combination evidence

Commit `test: complete owning scalar boundary parity`.
Replace broad no-other-mismatch assertions with an executed field/variant mapping
for the remaining numeric/text rows and their actual owners. Exact input and exact
nested pointer are required; type-class proxies/schema-only helpers do not prove
owning decisions. Include integer limits and canonical forms, forbidden versus
allowed characters and every selector variant (including release channels), with
positive/negative boundary controls. Retain meaningful same-input combination
coverage: all7 evidence kinds × runner present/absent × absent/all6 selector kinds
× count present/absent (196 cases), plus prior local collection/axis controls.
Explicitly separate token/Unicode/referential/lossless-projection layers and give
evidence for exceptions. Preserve original independent literal bytes/digests and
all previously accepted regressions. Repair any further approved-scope discrepancy
discovered within this audit; do not implement later evaluator/trust work early.

### S043-R6 — Actual consumer parity and original P03 exit

Commit `test: verify complete P03 scalar consumer parity`.
Run representation/text positives and one-axis negatives through actual owning
construction, canonical projection and real loader/preparation/public CLI wherever
those fields are authored/consumed. Graph-only values use their true owners;
never invent authored fields for test routing. Retain original asymmetric source/
subject/actor/cohort/context/revision/component and comparison obligations.
Execute ALL original S039/S041/S043/P03 exit lanes, owning regressions/doctests,
standard/schema/structure/diff gates. Reconcile reports/status/progress before
final gates. Retain each failed/green attempt separately with exact command/exit;
correct earlier overclaims without deleting history. Missing local automation is
not inherently hardware-blocked. Pi records implementation/evidence only; Codex
owns independent P03 acceptance before S044. No private coordination metadata or
publication/signing/deployment/activation authority belongs in public records.

## DEV-025 — Complete actual owning-rule parity before P03 acceptance

**Date/author:** 2026-10-04, Codex under owner delegation.
**Reviewed candidate:** `00bec96b0e97c09898975700bbd1790c0c27de79`.
[Independent review](reports/CODEX-00bec96.md) retains all four returned commits
and accepts RF65 for its allocated scope. Preserve all 42 numbered acceptances.
RF66 wrong fragment prefix type and RF67 omitted local collection conditions are
independently reproduced. RF45 remains partial; S043/P03 changes-required.
All original 64 R/AC,21 RA,Q01–Q15,S211 and prior accepted authority/identity
behavior remain unchanged. This is the current binding amendment, retaining
DEV-022 and earlier technical decisions except the frontier replaced here.
The previously missing returned run has now been matched across two attempts.

Execute **P03-R11 → P03-R12 → P03-R13 → S043-R5**, **pfc through EQM-P03**.
Endpoint S043-R5 inclusive; continue every eligible checkpoint after each green
commit. Independent P03 acceptance must precede S044: this hard integration gate
justifies the endpoint. No ordinary engineering decision remains owner-blocked.

### P03-R11 — Actual owning types and scalar parity

Commit `fix: align graph scalar schemas with owning types`.
Repair RF66: FragmentUse prefix is Option<LocalRequirementId>, not SelectorText.
Preserve owning vocabulary, optionality and accepted maxima. Read actual field
carriers/constructors/projections and correct analogous type/constraint mappings
throughout the existing introduced authored/graph/comparison audit. Repair every
approved-scope schema-expressible discrepancy discovered, not just this probe.
Add exact-field literal positives/negatives run through real schema validation
and the corresponding owning API. No domain loosening or new wire/policy fields.

### P03-R12 — Local collection and combination parity

Commit `fix: enforce owning graph collection invariants`.
Repair RF67. Journey surfaces remain ordered but must be nonempty and unique;
minItems and uniqueItems do not require sorting. Surface composition requires
at least one requirement or fragment origin; preserve owning direct-only,
fragment-only and combined valid forms. Use faithful local alternatives, not
requirements that both collections be populated. Audit and repair analogous
locally expressible cardinality, uniqueness and conditional-presence rules.
Local multi-field rules must not be mislabeled unexpressible relationships.
Genuine referential integrity, NFC, Unicode-byte counts, token distinctions and
normalized collisions remain separate owning layers where appropriate. Test
same-input positive and one-axis negative owning/schema counterparts.

### P03-R13 — Differential corpus grounded in actual owners

Commit `test: prove schema parity against owning consumers`.
Complete the field/variant matrix required by DEV-022 using actual typed APIs or
checked loaders for each applicable owning rule. Schema-only helper assertions
and copying the schema expectation into a test do not prove parity. Establish
a genuinely finalized positive graph counterpart and document precise layers
for intentionally different wire/projection values. Map every audit row to its
actual owner/source anchor and executed case, including optional prefix, ordered
journey collection and surface alternatives. Preserve independent literal bytes/
digests, v1 controls and all prior adversarial regressions. Repair additional
approved-scope omissions found during the differential audit within this batch.
Do not implement later trust/evaluator semantics early.

### S043-R5 — Real consumers and full original cycle exit

Commit `test: verify owning P03 consumer parity end to end`.
Exercise repairs through actual owning projection, checked loader/preparation and
public CLI boundaries wherever those inputs are authored/consumed. Graph-only
projections require genuine owning construction/canonical consumer controls;
do not invent an authored field merely to route a test. Retain original
asymmetric ownership, actor/build, N/N+1 cohort, exact revision/configuration/
context, component mutation and literal-oracle obligations. Execute every
original S039/S041/S043 and P03 exit lane, affected tests/doctests, standard gate,
schema parity and package structure validation. Regenerate from source atomically;
retain v1 bytes and original criteria. Update reports/progress before final gates,
retain failures and explicit skipped checks. Independent acceptance stays Codex's.
No private tracker/session/handoff metadata belongs in this public repository.
No publication, signing, deployment or optional-native activation is authorized.

## DEV-022 — Complete every owning field before P03 acceptance

**Date/author:** 2026-10-03, Codex under owner delegation.
**Reviewed candidate:** `1836b3512272a78b28f7548debd2a61f4ebbfd84`.
[Independent review](reports/CODEX-1836b35.md) preserves all four returned commits,
accepts S041 and allocated RF53/RF59/RF61/RF62/RF63/RF64 corrections, retaining
41 earlier numbered acceptances (**42 total**). S043/P03 remain changes-required:
RF65 demonstrates missing nested reference vocabulary and source-root cardinality.
RF45 remains partial; whole RA02/RA13 and original 64 R/AC,21 RA,Q01–Q15,S211 remain.
This is the sole current amendment, retaining DEV-020 and earlier decisions.

Execute **P03-R8 → P03-R9 → P03-R10 → S043-R4**, **pfc through EQM-P03**; all four
substantial checkpoints. S044 requires independent P03 acceptance. This existing
hard integration dependency is the reason for the boundary; no owner-only blocker.
No later evaluator/trust/operational behavior is implemented early.

### P03-R8 — Repair nested references and source collection contracts

Commit `fix: constrain nested scenario references and source roots`.
Apply owning TargetId/ProviderId constraints to both ScenarioSubjectV2Dto tagged
variants; field-name/kind shapes and valid boundaries must be preserved. Enforce
nonempty source-component roots at authored and graph roots, plus duplicate-free
exact values where expressible. Root collection bounds are separate from RepoPath
NFC/Unicode-byte/collision semantics. Add faithful schema-expressible portable
path lexical constraints with owning accepted controls; do not tighten RepoPath.
Fix all direct RF65 cases, regenerate source-driven schemas atomically, retain
v1 bytes, no new fields/digests/parser or policy changes. Run actual domain and
loader/checked boundaries with same-ingress positives and one-axis negatives.

### P03-R9 — Finish field-complete owning schema audit

Commit `fix: complete owning graph member value constraints`.
The existing generic table is insufficient. Enumerate each serialized field and
enum variant of every introduced authored and semantic graph/comparison DTO,
including nested existing projections introduced into the graph wire surface.
Map each to the owning constructor/projection, exact constraint, schema expression
or justified separate layer, and specific test case. Include discriminated subject
references, source root collections, target/source/provider references, applicability
variants, contract/entity IDs/text/revisions/owners, binding/evidence/runner/waiver
projection members, profile defaults/selections and extension recursion. Read
actual domain rules rather than inventing universal rules for differently typed
values. This is completion of the already approved owning graph value contract.
Repair all faithfully schema-expressible omissions discovered in this audit within
the batch, not only RF65. Do not require pointer identity or introduce unrelated
identity bindings. ASCII vocabulary/bounds, exact-one, lexical path subset and
collection bounds must not be mislabeled cross-field/Unicode exceptions.
NFC/raw tokens/Unicode byte counts, normalized collisions, referential integrity
and genuine domain relationships stay explicit owning layers when schemas cannot
faithfully prove them. Preserve valid forms and all accepted authority/identity
behavior; no trust promotion, wire compatibility alias or manual generated JSON.

### P03-R10 — Execute every field and variant boundary

Commit `test: cover every owning graph field boundary`.
Independent literal positive/negative input matrix covers every field/variant row
from P03-R9 across real local Draft2020-12 validators with format assertions and
actual domain/loader/checked-reader/canonical projection consumers as applicable.
Mutate the exact nested reference field, not a similarly typed sibling. Include
valid maxima, empty/minimum/maximum/over-boundary collections, duplicates, absent/
null/type distinctions, variant discriminants, portable path positives/negatives,
source root cardinality and target/provider subject alternatives. Assertions derive
from independently authored expectations plus owning APIs; no generator inspection
alone masquerades as behavior. Report intentional schema/token/semantic differences.
Retain independent literal bytes/digests, v1 controls and graph-bound matching.
Do not certify full coverage with grouped type labels or a bare test count.

### S043-R4 — Actual integration and full original cycle exit

Commit `test: verify field-complete P03 consumer parity`.
Run nested-reference/source-root/other discovered controls through actual asymmetric
preparation and public CLI admission/diagnostics, with valid positives. Retain all
transitive ownership, repeated actor/distinct build, explicit N/N+1, exact revision/
configuration/context and component mutation/literal-oracle obligations. Execute
ALL original S039/S041/S043/P03 exit lanes and affected owning regressions/doctests.
Reconcile implementation reports/current progress before final gates; retain failed
attempts. Extended verification is unrun unless executed, not automatically blocked
by hardware or unspecified prerequisites. Actual live facts/operational authority
remain separate. Pi reports implementation/verification only; independent P03
acceptance before S044 remains Codex-owned. One green commit per checkpoint;
continue all four, not after the first repair. No private tracker/session metadata
belongs in public records and no activation/publication is authorized.

## DEV-020 — Complete bounded schema contracts and independent boundary corpus

**Date/author:** 2026-10-03, Codex under owner delegation.
**Reviewed candidate:** `aa5556336075aae2065862ed65166176717888f6`.
[Independent review](reports/CODEX-aa55563.md) preserves all five returned commits,
accepts S039/RF55/RF60 and allocated RF57 authority repair, and retains 40 prior
numbered acceptances (41 total). S041/S043 and P03 remain changes-required;
RF59/RF61 partial, new RF62/RF63/RF64 require completion of existing DEV-017 criteria.
Original 64 R/AC, 21 RA, Q01–Q15 and S211 remain unchanged. Prior decisions remain
except the current frontier is replaced below. This is the sole current amendment.

Execute **P03-R6 → P03-R7 → S043-R3**, **pfc through EQM-P03**; all three checkpoints,
then independent P03 acceptance before S044. This boundary is the existing hard
integration gate, not a one-commit repair limit. No owner-only decision is pending.

### P03-R6 — Complete the full owning schema audit

Commit `fix: complete bounded v2 identifier and extension schemas`.
Audit every introduced P03 authored and graph/comparison member against its owning
constructor, recording a compact field/rule/layer table in the implementation
report. Fix all schema-expressible omissions, not merely four probe assertions.
RF62: derive each identifier's exact segment count, <=63 ASCII segment bound,
<=255 ID bound and distinct requirement bound from owning types; use valid boundary
controls, not only malformed examples. RF63: enforce extension namespace grammar
and nested object key grammar recursively in authored and protocol shapes; preserve
valid recursive tagged unions and display-only projection exclusion. Inspect
profile defaults keys/values and other map keys, optional fields, reference forms,
positive canonical revisions, sets, roots and text against owning contracts.
RF64: schema must preserve the actual `RepositoryIdentity` contract, including
explicit-port values accepted by the domain. Do not tighten its parser or introduce
new URI/product rules. Express existing ASCII/length/forbidden-character rules
where faithful. Keep exact Unicode byte/NFC/depth/node/token/relationship validation
at owning boundaries when not expressible. Required exactly-one remains enforced.
Use generator source/local references and regenerate atomically; no hand-edited
JSON, fallback loosening, new wire fields/digests or v1 changes. Accepted graph
binding must remain intact. Any discovered mismatches within this audit are
eligible work within this checkpoint, not a reason to return for the same decisions.

### P03-R7 — Execute an independent cross-layer boundary corpus

Commit `test: prove owning v2 schema boundary parity`.
Build independently authored table-driven valid/invalid boundary inputs, covering
all audit rows in P03-R6, against local real Draft 2020-12 validation with format
assertions and actual loader/domain/checked-reader paths. Include 63/64 segment,
255/256 total ID, exact per-type segment counts, namespace and recursive-key
mutants, valid explicit-port repository plus actual parser-invalid variants,
profile default map vocabulary and recursive union unknown members. Keep Unicode
byte-vs-character and raw-token/semantic relationship exceptions explicit; never
assert universal equivalence where distinct layers intentionally apply.
Add valid maximum/range controls so constraints do not silently overreject.
Exercise graph, comparison and authored inputs, with positive controls at the same
ingress. Retain literal bytes/digests and v1 compatibility controls. Run owning
regressions/doctests and declared standard/schema lanes per checkpoint.

### S043-R3 — Finish actual integration and original cycle exit

Commit `test: verify complete P03 boundary integration`.
Integrate the new corpus into real asymmetric preparation and CLI diagnostics,
including positive accepted boundary inputs and one-axis negatives at real ingress.
Preserve transitive source closure, repeated actors/distinct builds, explicit N/N+1
cohorts, typed graph-bound comparison and all original mutations/literal oracles.
Run every original S039/S041/S043/P03 exit obligation and affected owning lane,
not just helpers. Reconcile stale P03-R5 baseline and any intermediate failures
against exact history/evidence; never rewrite historical failures into passes.
Finalize report/status/progress before final gates, one green commit per checkpoint.
Pi records implementation/verification only; Codex owns acceptance. Private coordination
and tracing remain supervisor-owned. No activation or publication authority.

## DEV-017 — Bind consumed cohort authority and complete P03 value parity

**Date/author:** 2026-10-03, Codex under owner delegation.
**Reviewed candidate:** `8e5f21867bdaa7ccf32c4c2d626b2f269184380e`.
[Independent review](reports/CODEX-8e5f218.md) retains all five commits and the
40 prior allocated numbered-step acceptances. RF56/RF58 close for allocated scope;
RF55/RF57/RF59 remain partial, RF60/RF61 are reproduced integration defects.
S039/S041/S043 and the required independent P03 gate remain changes-required.
All original 64 R/AC, 21 RA, Q01–Q15 and full MVP S211 remain unchanged.

Execute **S039-R3 → P03-R5 → S043-R2**, **pfc through EQM-P03**. This is the
complete eligible integration batch: bind the actual application consumer,
audit/fix the owning generated value contracts, then execute original cycle-exit
parity through real preparation and CLI diagnostics. Continue through all three;
one green correction is not an endpoint. S044 cannot precede independent P03
acceptance. All ordinary decisions below are settled; no owner-only blocker.
DEV-015 and retained earlier decisions remain except these consuming-boundary
clarifications. This is the sole current governing amendment.

### S039-R3 — Revalidate retained cohort at consuming boundary (RF60)

Commit `fix: bind consumed cohort to exact graph and scenario authority`.
Retain the exact resolved cohort value privately, in addition to the resolved
scenario ID/revision and expected axes. Before comparison matches candidates,
require that value to equal the consuming immutable graph's exact ID/revision
entry and its resolved scenario to equal the selected exact scenario. Re-resolve
that retained authority against the consuming graph with the owning resolver,
and require its independently derived expected axes to equal the carrier's axes.
Neither occurrence-name equality nor a private constructor proves this binding.

This is value authority, not address identity: independently prepared copies with
identical retained cohort/scenario/configuration values may pass. Do not add a
new wire digest/domain or require unrelated graph content to change comparison
identity. Missing/altered same-ID/revision cohort, wrong scenario ID/revision,
changed target/provider configuration, source/build/artifact substitution and
foreign resolved authority must reject with typed causes before Eligible. Retain
candidate SubjectManifestDigest separately and do not promote matching to trust.
Audit every consuming call, including any standalone matcher: a matcher with no
graph input must be described only as matching its supplied resolved authority,
never as current-graph/scenario validation. Preserve pure engine boundaries.

Tests through the real application API include the two reproduced negatives,
same-ID/revision altered content and graph configuration substitutions, same-name
occurrences with different scenario roles/subjects, and equivalent independently
prepared graph positives. Retain all prior coverage/repeated-actor/distinct-build
controls and execute actual prepared asymmetric fixture counterparts. Owning
application/domain/manifest/protocol/engine lanes and changed doctests.

### P03-R5 — Complete owning v2 value contracts (RF61/RF59)

Commit `fix: align v2 value schemas with owning validation contracts`.
Audit all newly introduced P03 authored provider/cohort and graph/comparison
member forms against existing owning domain and ADR-0013 rules. Implement the
JSON Schema-expressible constraints in DTO/generator source: exact markers,
symbol vocabulary and bounds, positive authority revisions with actual wire
range/representation, SHA/source-commit forms, repository and owner vocabulary,
nonempty/unique sets, nonempty bounded text and finite profile declarations,
absent-or-present optionals, and exactly one target/provider configuration claim.
Do not merely fix the six demonstrated examples. Include provider duplicate owners,
profile title/owner/dimension/value forms and recursive extension union/key rules.
Use correct local schema definitions/references; no hand-edited generated schemas
or broadened `additionalProperties` fallback after generator failures.

Derive constraints from owning types, not new product values. Character-length
constraints cannot prove UTF-8 byte bounds; retain exact NFC/byte/depth/node limits
in owning reader/domain checks. Raw duplicate members, normalized/logical-key
collisions and relationships not faithfully expressible in schemas stay separate
explicit validation layers. Required exact-one field presence is schema-expressible
and must reject both/neither. Do not require development-optional build/source axes
in every structural form; completeness remains the consuming mode's obligation.

Add independent positive and one-axis negative matrices across generated manifest,
graph and comparison schemas using the existing local-only real Draft 2020-12
validator with format assertions, plus actual authored loader/conversion and checked
reader controls. Clearly distinguish schema/token/semantic failures. Regenerate
owning schemas atomically; v1 bytes unchanged, valid approved boundaries preserved.
Owning domain/manifest/protocol/application/xtask lanes and changed doctests.

### S043-R2 — Complete consumer and cycle-exit parity

Commit `test: close graph-bound comparison and v2 value parity`.
Integrate both checkpoints into the real asymmetric prepared fixture, checked
comparison/cohort consumer and public CLI admission/diagnostics. Add cross-session,
cross-scenario, changed exact authority and schema-reader mismatch negatives;
retain repeated actors, distinct builds, separate N/N+1 combinations, transitive
sources and all existing role/source/configuration/subject mutations. Exercise
public usage examples and generated schemas, independently expected literal
bytes/digests and actual consumer paths. Do not describe helper success as whole
acceptance or synthetic material as real-product certification.

Execute original S039/S041/S043 and P03 exit commands plus all affected owning
lanes, doctests and full standard/schema/structure gates after finalizing every
report/status/progress edit. Preserve historical failures honestly. Stop only
after all three checkpoints for required independent P03 review before S044;
full S211, original criteria and external gates remain. Actual unavailable real
facts do not block synthetic/simulator-capable implementation or local automation.

## DEV-015 — Close checked comparison and cohort authority before P04

**Date/author:** 2026-10-03, Codex under owner delegation.
**Reviewed candidate:** `d2c9c57d2e5d3c8f448426619a49c48af45b211a`.
[Independent review](reports/CODEX-d2c9c57.md) retains all seven checkpoints and
accepts S040/S042 for allocated scope, preserving prior acceptance: **40 numbered
steps accepted**. RF51/RF52 close for allocated scope; RF45/RF53 remain partial.
S039/S041/S043 require RF55–RF59 corrections. S043 was reached but its independent
integration gate is not passed. Full original64 R/AC,21 RA,Q01–Q15 and S211 remain.

Execute **S039-R2 → S041-R1 → P03-R4 → S043-R1** with **pfc through EQM-P03**.
This substantial batch completes checked application authority, frozen cohort
representation/preparation, full definition/schema constraints, and the real
asymmetric fixture/CLI integration exit. All four checkpoints must complete;
a green repair alone is not an endpoint. Required independent review before
S044 prohibits a P04 start until these existing P03 criteria are satisfied.
No owner-only decision is needed. DEV-014 and earlier decisions remain binding
except the clarifications below; this is the sole current governing amendment.

### S039-R2 — graph-bound checked comparison inputs (RF55)

Commit `fix: bind comparison eligibility to exact matched scenario coverage`.
The application must derive comparison contract/scenario/profile authority from
one immutable prepared graph and the explicit approved rule reference. Derive
ContractDigest from that graph's frozen S019 projection or validate supplied
authority against it; no arbitrary contract digest accepted as graph-derived.
Bind checked wire context to that independently derived expected context; a
well-formed self-selected context is not independent expected authority.

Expected coverage is the exact occurrence set of the selected graph scenario.
The pure engine must reject missing, extra and duplicate occurrences and retain
distinct repeated same-target actors. Do not infer expected coverage from submitted
participants. Application inputs retain independently matched checked subject
records/results, exact occurrence/kind/configuration/content authority and their
separate SubjectManifestDigest. Construct immutable match results only after
existing bytes-first subject matching against resolved independent authority;
public digest fields alone cannot stand in for matching. If structural engine
inputs remain public, the checked application boundary must still enforce proof
of matching and exact graph context. No serializer/effect back-edge in domain/engine.

Different valid subject source/build/artifact assignments may share comparison
eligibility. Missing/mismatched context or subject coverage remains incomplete or
ineligible, never eligible by a fabricated digest. Preserve v1 equivalence and
later evaluator/trust ownership: no eligibility-to-conformance/trust promotion.
Test actual application calls: subset, extra, duplicate, wrong occurrence/kind,
phone record relabeled as provider, failed matching followed by eligibility,
context/contract substitution, valid repeated occurrences and distinct builds.
Owning lanes: domain/protocol/manifest/engine/application and changed doctests.

### S041-R1 — frozen cohort fields and checked graph authority (RF56/RF57)

Commit `fix: validate exact authored cohorts during session preparation`.
Restore assignment members target_configuration_digest and
provider_configuration_digest across domain/authored/protocol/schema/projection/
fixtures, alongside all existing S038 fields. Exactly one kind must be present.
The authored claim is retained, not substituted; compare it with independently
computed typed target/provider configuration from the immutable graph. Wrong
kind, mismatch, missing/both fields reject. No digest-string hash/type substitution.

Each cohort authority consumed must be the exact id/revision/value retained in
the prepared graph; reject absent or altered caller-created records. Loading and
preparation must validate every authored cohort's scenario resolution, exact
occurrence coverage, complete content axes and configuration. Resolve cohort
checks in the existing acyclic application preparation boundary after the graph's
owned configuration is available; pure engine resolution checks typed references
where appropriate, without acquiring a manifest/protocol dependency. No finalized
PreparedSession success with a dangling/ambiguous/incomplete cohort. Keep detailed
typed causal errors and real source anchors. Holding an authored cohort grants
no acquisition/trust claim; content axes remain independent candidate expectations.

Separate authored N/N+1 cohorts remain separate combinations. Match checked
candidate subjects against the exact retained cohort, not candidate-derived
expectations. Execute real load/resolve/prepare and consumed matching positives
and single-axis negatives, including altered authority with identical ID/revision.
Extend complete graph identity/schema and expected literal bytes atomically,
preserving genuine v1 behavior. Owning lanes: domain/manifest/protocol/engine/
application and changed doctests.

### P03-R4 — full definition and authored schema constraints (RF58/RF59)

Commit `fix: enforce v2 definition and authored schema value contracts`.
Comparison checked ingress must enforce the owning Profile definition invariants:
nonempty valid title/owners/dimensions, duplicate logical keys and finite values,
default and selected-value relationships, optional description and exact recursive
extension rules/bounds. Use shared owning domain validators/conversion rather
than a smaller parallel notion of a valid profile. Validation never grants trust.

Fix owning generated v2 authored provider/cohort value schemas, including exact
schema markers, approved symbol/digest/range/set/omission constraints. Optionals
are absent-or-present, not null; configure DTO ingress consistently. Restore the
negative SHA/null/marker tests described as over-strict instead of narrowing the
criteria. Audit introduced graph/comparison member value constraints against
the same contract, including recursive extension unions and profile definitions.
Keep real local-only Draft 2020-12 validation with format assertions. Raw token,
NFC/duplicate and cross-field constraints remain explicit reader layers; do not
claim JSON Schema proves them or material trust. Regenerate owning schemas;
v1 bytes remain unchanged. Owning lanes: domain/manifest/protocol/application,
schema generation/check and changed doctests.

### S043-R1 — real P03 consumer and asymmetric cycle-exit evidence

Commit `test: close checked asymmetric graph integration parity`.
Update the real S042 asymmetric fixture to the frozen assignment fields and
execute the checked application comparison/cohort path through actual preparation
and public CLI admission/diagnostics. Cover transitive sources, distinct roles,
repeated same-target occurrences, known provider, complete independent combinations
and negative counterpart fixtures. Do not manually discard match results and
reconstruct public digest structs, or claim a helper call proves production use.
Verify public docs/examples/generated schemas/domain/protocol/strict readers/
session and consumer parity for every changed field and scope. Retain independent
literal byte/digest and component mutation controls; filtering grants no authority.
No real product tasks, operational cohort policy, credentials or platform facts
are invented. Synthetic/simulator software remains eligible; Q gates stay at
actual dependent claims. After all four checkpoints, stop for mandatory independent
Codex integration review before S044. Original S043 exit criteria remain.

### Tracking and final-tree evidence (RF54 recurrence)

This amendment supersedes stale current-frontier text below. Reconcile current
STATUS/progress/step/chapter/spec mirrors with actual commits and dispositions;
backfill S043 d2c9c57 SHA. Preserve the historical P03-R2 committed report-marker
failure repaired by P03-R3; do not assert that every intermediate tree was green.
Finalize code/tests/schemas/reports/status/progress before final standard/schema/
structure/diff gates and commit. Use exclusive attempt logs with real exits and
retain failed attempts. Pi records implementation/verification, Codex acceptance.
Do not stop at an estimated budget, commit count or report. Actual runtime/user
limits, required review gates and genuine external blockers remain legitimate.
No publication/deployment/signing/live or optional-native authority is granted.

## Historical DEV-014 — Complete lossless graph and comparison integration through P03

**Date/author:** 2026-10-03, Codex under owner delegation.
**Reviewed candidate:** `667776537709f824efac48f6bf325be2c9ea2913`.
[Independent review](reports/CODEX-6677765.md) retains seven returned checkpoints
and accepts S032/S033/S035/S036/S037/S038 for allocated scope: **38 numbered steps
accepted**, preserving all prior acceptances. RF46–RF50 close for allocated scope;
RF45 remains partial and RF51–RF54 require correction. S039 is retained, not
accepted; S040–S043 are unstarted. All original64 R/AC,21 RA and Q01–Q15 remain.

Execute **P03-R2 → P03-R3 → S039-R1 → every ordered S040–S043**, using
**pfc through EQM-P03**, endpoint **S043 inclusive**. This substantial batch
combines three integrated repairs with four original implementation checkpoints.
The mandatory independent model/schema/identity review before S044 remains;
full MVP ends at S211. Continue after every green checkpoint, using focused reads
and native context management. A commit, report, long conversation, or estimated
context budget is not an endpoint. Actual runtime/user limits, required review
gates and genuine external blockers remain legitimate. No owner-only decision
is needed for this range. DEV-012 and earlier allocations remain unless expressly
clarified below; this is the current amendment in the sole governing plan.

### P03-R2 — lossless graph sum types (RF51 and remaining RF45)

Commit `fix: preserve tagged runner semantics in v2 graph identity`.
Use the existing approved runner v2 representation: argument literal is
`{kind:"literal",value:<exact text>}`; template arguments are `{kind:"target_root"}`,
`{kind:"selector_json"}` or `{kind:"result_path"}`. Cwd is `{kind:"repository",path}`
or the existing target_root/result_path tagged form. Preserve argument order.
The current bare-string graph projection collapses different valid domain inputs.
Reuse these semantic forms at the appropriate acyclic owning boundary; no
manifest→runner or domain/engine serializer dependency. Retain v1 bytes and
the frozen S019 contract projection. Audit every graph sum type for equivalent
lossiness: waiver target/provider/set coordinates must preserve their distinct
subject kind using the existing approved tagged v2 scope forms. Do not invent
other wire semantics or rewrite unrelated component identities.

Execute production prepared-session and component identity controls distinguishing
literal placeholder text and expanded argv, repository placeholder-looking path
and expanded cwd, ordered args, and distinct subject variants. Use independent
literal JCS bytes/digest oracles and compile-time typed-domain controls where
relevant. Update affected v2 goldens with old/new bytes and reason recorded;
do not change v1 goldens or silently regenerate expected values from the function
under test. Owning lanes: domain/manifest/runner/application, changed doctests.

### P03-R3 — real semantic projection schemas (RF52)

Commit `fix: constrain v2 graph and comparison projection schemas`.
Replace newly introduced unconstrained collection/property bags with generated
schemas from owning typed semantic projections. Each graph collection has its
actual exact member shape and nested union/member constraints; extensions use
the approved recursive tagged value representation. Comparison schema has exact
SHA spelling for contract/scenario/rule references and exact selected-profile
definition/revision/known-or-unknown member shapes, not arbitrary items. Preserve
the frozen graph/comparison root membership, typed digest domains, JCS full-member
set rules, decimal-string/numeric bounds, NFC and absent-vs-null decisions.
Do not add unapproved product fields. Keep owning crate dependencies acyclic.

Real Draft 2020-12 local-only validation with format assertions must run on actual
generated projections and independently authored negative mutations at every
collection/union boundary. Include null/numeric/unknown members, missing fields,
wrong kind/marker/digest, profile known/unknown misuse and extension type cases.
Remove manual root-only pseudo-conformance claims. Raw duplicate/token/cross-field
checks remain distinct strict-reader obligations, never claimed from JSON Schema.
Regenerate owning schemas atomically and preserve all v1 schema bytes. Add future
provider/cohort shapes atomically in their owning steps. Owning lanes:
manifest/protocol/application and schema generation/check, changed doctests.

### S039-R1 — checked comparison boundary and real consumer evidence (RF53)

Commit `fix: validate and consume explicit v2 comparison context`.
Retain the approved semantic root exactly contract_digest,scenario_digest,profiles,
comparison_rule_digest; the wire root adds only the exact schema marker. Implement
bounded strict bytes-first protocol ingress with an immutable checked carrier,
mutable DTO revalidation where exposed, typed digest domains and exact semantic
canonical bytes/digest. Share owning profile projection semantics rather than
adding a policy fallback or profile-ID-only reader. Reject duplicate members,
duplicate selected-profile/dimension logical keys, invalid references, wrong
schema, null/omission misuse and malformed known/unknown selections. Validate
selected values against the embedded exact definitions and retained known/unknown
contract; do not mint a trusted definition merely by deserialization.

Complete the allocated explicit v2 comparison eligibility path in the pure
engine/shared application ownership. Shared comparison identity is typed
ComparisonContextDigest; exact participant assignments and SubjectManifestDigest
remain separate. Different independently matched valid source/build/artifact
assignments can share comparison eligibility. Missing or mismatched shared context
or exact subject coverage stays incomplete/unknown, never eligibility by an
unchecked digest cast. Preserve v1 equivalence behavior and later S059–S076
conformance/trust ownership: eligibility alone is not equivalence or trust.
Use the existing immutable session graph and explicit approved rule reference;
no reload/latest lookup/clock capture/policy-digest substitute or invented rule
format/default. Execute genuine distinct checked subject manifests, independent
context matching and one-axis contract/scenario/profile/rule negatives through
the new reader and actual consumer. Owning lanes:
domain/protocol/manifest/engine/application and changed doctests.

### S040–S043 — resolved provider/cohort decisions and original implementation

S040 retains DEV-012's exact provider root id,configuration_digest,owners,extensions
and typed ProviderConfigurationDigest over the whole validated provider definition
under eqm:v2:provider-configuration. The declared configuration_digest is external
content authority, not its own projected identity or proof of verified material.
Extend target providers, component and complete graph projections/schemas together.
Resolve exact declared target/scenario provider references, with missing/ambiguous
references fail-closed at preparation; no universal linkage/guessed endpoints.
Replace the temporary structural provider SHA in domain/protocol expected/assignment
carriers with the typed configuration domain without changing frozen entry names.
Update the temporary unresolved-provider positive preparation test to the actual
known-provider positive and missing-provider negative once this authority exists.

S041 retains exact cohort root id,revision,scenario,assignments,extensions.
ScenarioId must identify exactly one graph scenario definition; reject ambiguous
multiple revisions rather than infer latest or invent a revision selector.
Authored cohort assignments declare one approved exact source/build/artifact
combination using S038 members; validate all and only scenario occurrences/kinds
and target/provider configuration against the immutable graph projections.
The cohort is independently supplied authority when matching a candidate subject
manifest; never derive expected source/build/artifact axes from that candidate.
Graph-backed configuration authority and explicitly authored cohort content axes
are distinct inputs. Match candidates through checked bytes-first subject ingress
against that resolved cohort; undeclared combinations and missing occurrences reject.
Do not claim loading the authored cohort verifies real source/build material;
actual acquisition/trust remains later work. Multiple approved combinations are
separate cohort authorities, not an inferred support window. Resolve discovery
through existing contract_sources and extend graph/schema identity atomically.

S042 executes the genuinely synthetic asymmetric CLI/desktop/mobile/provider
fixture, shared closures, repeated actors, participation and independently released
build combinations through real loading/preparation/identity and cohort matching.
S043 completes public usage/examples and actual schema/model/domain/reader/
application/CLI parity with adversarial one-axis controls, then stops for required
independent review before S044. Synthetic and simulator-capable work is eligible;
Q02/Q06 actual host/cohort facts remain unverified, never fabricated or a blanket
hardware blocker. Preserve all original per-step contracts and verification lanes.

### Current tracking and final-tree evidence (RF54)

This amendment supersedes stale current-frontier text below. Retain historical
receipts honestly; backfill S039's actual 6677765 SHA. Keep one current frontier,
matching step/cycle counts, report dispositions and owning spec/chapter mirrors.
No whole acceptance case closes from references or narrow helper checks. Finalize
code/tests/schemas/public docs/reports/progress/status before each final gate/commit.
Use exclusive per-attempt logs and actual command exits; retain corrected failed
attempts. Pi records implementation/verification; Codex alone records independent
acceptance. No publication/deployment/signing/live authority is granted.

## Historical execution revalidation — 2026-10-03

The source remains at the preceding planning checkpoint with no subsequent product
implementation changes available for review. The DEV-012 eleven-checkpoint batch
is still pending; no additional implementation or acceptance is recorded. Continue
P03-R1 through S043 under the complete decisions below, preserving the required
independent review before S044 and the full S211 MVP scope. The prior 32 allocated
step acceptances and RF45–RF50 findings remain unchanged.


## DEV-012 — Finish production shared-subject integration and the P03 boundary

**Date/author:** 2026-10-03, Codex under owner delegation.
**Reviewed candidate:** `38c125467084f41bb0414484d62f7efc59eb28d0`.
[Independent review](reports/CODEX-38c1254.md) retains all14 returned commits and
accepts S025–S031 and S034 for allocated scope, preserving S001–S024: **32 numbered
steps accepted**. S032/S033/S035/S036/S037/S038 remain retained contributions with
changes required. S039–S043 are unstarted. Original acceptance, RA02/RA13 and
Q01–Q15 remain; no release readiness or human-only testing gate is reached.

Execute **P03-R1 → S032-R1 → S033-R1 → S035-R1 → S037-R1 → S038-R1 → every
ordered S039–S043**, using **pfc through EQM-P03**, endpoint S043 inclusive.
This is six coherent repair/integration checkpoints followed by five original
implementation checkpoints, not a correction-only batch. Independent model/schema/
identity review before S044 remains required; full MVP ends at S211. Do not pause
at a green checkpoint, a report, or an estimated context budget. Use focused
reads and available native context management. Actual user/runtime limits,
original review gates and genuine dependent external blockers remain legitimate.
No owner-only decision remains for this allocated range.

### P03-R1 — explicit complete v2 graph identity (RF45)

Commit `fix: bind new graph semantics to explicit v2 identity`.
Production prepare currently always calls canonicalize_graph (v1), which silently
omits role/source_components/depends_on, component authorities and participation.
Add explicit complete v2 graph projection under **eqm:v2:semantic-graph** with its
own typed digest/carrier; retain the v1 label, schemas and byte oracles. The root
uses the existing graph collection names and exact v2 semantic-graph schema marker,
plus **source_components** and **participation**. Include every currently modeled
normative field in those collections, using approved v2 numeric/NFC/omission/JCS
set/extension-type rules and bounds. Reuse approved component projections where
semantically complete; do not reuse lossy v1 representation bytes or extend frozen
S019 ContractDigest membership. Each later scenario/provider/cohort introduction
extends the v2 graph projection/schema atomically with its own collection.

The v1 projection explicitly rejects graphs containing v2-only metadata/authority;
it never silently omits it. Retain explicit authored version/authority provenance
through loading/preparation and choose the matching graph representation in the
application session. V2 documents/metadata must not be implicitly projected as v1.
A compatibility structural SHA accessor may remain, but expose/retain the chosen
version/domain so it cannot be called a v1 identity. Preserve all genuine v1
session/output behavior. Update actual graph schema generation/parity from owning
types, not placeholder schemas or generic unconstrained property bags. No domain
or engine serialization back-edge.

Test actual prepared sessions: one-axis role, component roots/owners/dependencies/
extensions, target references and participation mutations each affect v2 identity;
formatting/set permutations preserve identity; explicit v1 projection rejects new
metadata. Execute independent literal canonical bytes/digest and real local schema
conformance, including large exact integers and extension type distinctions.
Owning lanes: domain/manifest/protocol/application and their changed doctests.

### S032-R1 and S033-R1 — one resolved ownership boundary (RF47/RF46)

S032-R1 commit `fix: honor transitive declared source ownership`.
The current target/root overlap check accepts only direct component IDs, while
binding conversion computes their transitive closure. Use one deterministic
validated closure for overlap and ownership consumers. Target depends_on does not
implicitly grant another target's roots; only its declared component closure does.
Keep missing/cyclic/duplicate/portable-output-root/escape/ambiguous checks blocking.
A dependency declaration cannot authorize the non-overlapping remainder of an
otherwise unrelated root. Test nested direct/transitive overlap positively and
same paths without the exact closure negatively, with unrelated roots rejected.

S033-R1 commit `fix: route structural checks through resolved ownership`.
The CLI still invokes output-root-only evaluate_structure; its new roots helper
has no production caller. Pass the checked graph's exact target/component closure
through the production structural path, preserving the pure injected view and v1
behavior. Do not trust arbitrary caller root lists as authority or globally widen
confinement. Execute real CLI check (and shared application path where present)
against a declared shared artifact plus missing/undeclared/escaped/symlink controls.
Do not assert overall conformance passes when unrelated evidence obligations fail;
assert exact structural diagnostic/status and exit behavior at the real boundary.
Owning lanes: manifest/engine/application and CLI package **eqm**.

### S035-R1 — reject conflicting participation at production ingress (RF50)

Commit `fix: reject ambiguous participation in prepared graphs`.
WorkspaceGraph::new uses unique_index by rule ID and bypasses
validate_participation_rules, allowing distinct IDs to conflict on the same
(requirement,target). evaluate_participation then chooses the first ID. Route
production graph construction/resolution through checked coordinate uniqueness
with a typed stable error. Complete the already allocated independent ParticipationDigest
projection under eqm:v2:participation over id,requirement,targets,applicability,extensions,
with literal bytes/digest controls, retaining frozen ContractDigest membership.
The public evaluation boundary must reject an ambiguous
supplied registry, not select a winner; it may accept a checked carrier or explicitly
validate before matching. Keep exact profile true/false/unknown behavior, no global
inclusion, exclusion or waiver. Add real authored/load/resolve/prepare conflicting
true/false and identical-value different-ID negatives, authority-order permutations,
disjoint-coordinate positives and existing unknown/profile cases. S034's standalone
validator contribution is retained. Owning lanes: domain/engine/manifest/application.

### S037-R1 — complete authored scenarios (RF48)

Commit `feat: integrate authored scenario occurrences`.
The domain Scenario exists, but no v2 scenario reader/schema/discovery/conversion/
graph authority exists. Implement the already frozen DEV-009 scenario members:
**id, revision, participants, dependencies, extensions**; participant **id, role,
subject** tagged target/provider, dependency **from,to**. Existing contract_sources
routes exact v2 scenario kind, no new source aliases/CLI flags. Index scenarios,
reject duplicate authority/occurrence, missing roles/targets/dependency endpoints,
and preserve repeated target/provider occurrences and canonical full-member sets.
Provider references remain explicit unresolved/non-authorizing until S040 supplies
provider authority; do not synthesize it or skip S040. Occurrence dependencies are
explicit directed relations; do not invent an additional scenario DAG requirement
absent from the governing criteria. Generate actual schema and extend complete
v2 graph projection atomically. Correct S037's report claiming S038 owns this work.
Test real loader/prepare, malformed roles/tags/references and repeated actors.
Owning lanes: domain/engine/manifest/protocol/application.

### S038-R1 — exact subject admission and representation parity (RF49)

Commit `fix: validate exact subject assignments at the protocol boundary`.
The DTO accepts two entries sharing occurrence with conflicting builds, while the
domain rejects them. It has no checked conversion, scenario/context matching or
executed substitution matrix. Domain id is SelectorText but DTO id validates SHA.
Keep the modeled **authored symbolic id** (validated SelectorText), revision and
entries; compute **SubjectManifestDigest separately** over canonical root
**id,revision,entries**, excluding schema. Id is not its own digest/seal. This
resolves the existing carrier inconsistency without inventing an authority claim.
Each entry keeps existing occurrence/repository/source_commit/source_content_digest/
target_configuration_digest/provider_configuration_digest/build_id/artifact_digest
names and omitted absence. Exactly one configuration kind remains required.

Implement bytes-first strict checked conversion and revalidation of mutable DTOs:
reject exact and logical-key duplicates before domain promotion. Match one exact
assignment for every scenario occurrence (no missing/extra/target-provider kind
substitution) against independently supplied immutable expected subject axes.
All supplied expected repository/commit/source/config/build/artifact axes must
match, not a context derived from the same untrusted DTO. Only explicitly declared
development use admits allowed absent axes; completeness-requiring use rejects
absence, without claiming signature/source/build verification. Preserve typed
TargetConfigurationDigest; S040 introduces the allocated provider-config carrier,
then extends this comparison atomically. Existing structural source SHA references
may remain until their owning source acquisition supplies verified material; never
cast/relabel raw hashes or substitute target configuration for source content.

Add canonical projection/digest with independent literal oracle, domain/DTO/schema
parity and real strict bytes ingress negatives for duplicates/null/NFC/schema/bounds.
Execute two occurrences with different valid builds, every-axis substitutions,
missing/extra occurrence and wrong kind, declared-development absence and required
completeness failure. Schema value constraints and cross-field/token checks stay
explicit layers. Owning lanes: domain/protocol/manifest/application.

### Remaining representation mechanics settled before implementation

S039's comparison-context semantic root is exactly the retained four members;
the wire root adds only its exact schema marker. Use ContractDigest, ScenarioDigest
and ComparisonContextDigest typed domains. ScenarioDigest projects exact scenario
id/revision/participants/dependencies/typed semantic extensions. Profiles are the
existing selected-profile definition/value projection, not whole selected policy
or a profile-ID-only list. comparison_rule_digest is an explicitly supplied
structural SHA reference to approved rule authority; no new rule format/domain,
policy-digest substitution or implicit default is invented at S039. Missing rule
material remains missing, while synthetic explicit references permit software
tests. Comparison identity alone establishes neither subject validity nor trust;
keep v1 equivalence behavior and add the explicit v2 path separately.

S040 provider manifest remains id,configuration_digest,owners,extensions. Its
configuration_digest is the declared external configuration content reference;
the separate typed ProviderConfigurationDigest projects that entire validated
provider-definition root under eqm:v2:provider-configuration. Never hash a digest
string as a replacement projection or equate declared content with verified
material. Subject provider_configuration_digest compares against the typed
projected configuration authority; source/configuration verification remains
later acquisition/trust work. Target providers are an explicit ProviderId set,
extending target/v2 graph projections and resolution atomically, never all providers.

S041 cohort root is id,revision,scenario,assignments,extensions; scenario is the
exact ScenarioId reference to the uniquely indexed current scenario definition.
assignments reuse S038's exact occurrence-assignment member names/forms, canonical
full-member sets with unique occurrence keys. Resolve against that scenario and
its exact graph revision/context: all and only required occurrences, correct
subject kind and independently supplied exact configuration/source/build/artifact
assignments. Each authored cohort declares one combination; multiple approved
combinations are separate cohort authorities, never an inferred version range.
Full graph identity binds the referenced scenario definition and cohort together.
Synthetic positive/negative N/N+1 combinations establish this software behavior,
not a real product support promise. Unknown actual provider/cohort material remains
visible/non-authorizing; it does not block these synthetic implementations.

### Continue S039–S043 and reconcile evidence

After the six repairs, implement every original S039–S043 in order under DEV-009's
exact comparison-context/provider/cohort/discovery/domain allocation. S039 uses
canonical scenario and exact selected profiles, with shared comparison identity
separate from participant build/source/artifact identities. S040 adds explicit
provider authority and target providers references and typed provider-config
identity; S041 authors complete declared cohorts; S042 synthetic asymmetric
multi-runtime fixture; S043 real reader/schema/domain/application/CLI/public-example
parity. Each introduced field extends its component and complete v2 graph identity
atomically. Actual Q02/Q06 operational cohort/host facts remain unverified, not a
blanket block on synthetic or simulator-capable software.

Finalize all code/tests/generated schemas/reports/progress/status/mirrors before
final per-checkpoint standard/schema/structure/diff gates. Use exclusive unique
attempt logs, record exact exits/source tree and preserve failed attempts. Never
amend reviewed commits, mask command exits or invent evidence. Backfill S038 SHA
and report correctly that the continuation completed nine steps S030–S038, fourteen
checkpoints total. Reconcile accepted/retained counts and one frontier; historical
checks remain historical. Codex acceptance stays independent. All original64 R/AC,
21 RA and Q gates, dependencies and full S211 scope remain unchanged.


## DEV-011 — Complete reusable lifecycle and provenance integration evidence

**Date/author:** 2026-10-02, Codex under owner delegation.
**Reviewed source:** `c774fe000bdecb50dd1b8ea4b16164900bd6032b`.
[Independent review](reports/CODEX-c774fe0.md) accepts S022 allocated scope;
24 numbered steps S001–S024 are accepted. Retain S025 typed/chronology and S026
acyclic-registry contributions; whole steps await RF42/RF43 closure. All four
returned commits remain, no prior acceptance revoked. Original R/AC/RA/Q gates
remain; no release readiness or new owner authority is implied.

Execute **S025-R2 → S026-R1 → every ordered eligible S027–S043** with
**pfc through EQM-P03**. This is two corrections plus 17 original implementation
steps. S043 inclusive is the required model/schema/identity integration review
before S044; full MVP remains S211. Continue after each green commit; do not
choose an arbitrary commit-count, report or estimated-context stopping boundary.
Use available native context management and focused reads, retaining the range;
actual user stops, enforced runtime limits, original review/external gates apply.
All engineering decisions required for this allocated range are resolved here.

### S025-R2 — real projection-to-admission evidence (RF43)

Commit `test: verify admission with production provenance projections`.
Retain the approved implementation; complete missing DEV-010 evidence. Put a real
loader/prepared-session projection-to-admission integration in the application
owning test lane (application already depends on manifest/protocol/runner).
Obtain contract/binding/evidence-spec/target identities from existing canonical
v2 projection APIs and selected-policy identity from same-session production
prepare_policy_provenance. Wire their exact digests into a genuine v2 assertion,
seal semantic identity, parse exact bytes and admit against the independently
computed typed context. A real semantic mutation through projection/lookup must
reject old evidence; formatting-only equivalent loaded inputs preserve identity.
No test-local substitute projector or hash_content of arbitrary seed strings.
Include present typed runner/adapter expectations where existing APIs/carriers
permit; use existing production runner projection/acquisition for runner evidence.
Do not invent an adapter projection API absent from this step: document its existing
carrier/domain boundary and retain typed-reference comparison tests.

Add compile-fail examples that actually misuse EvidenceAdmissionContext fields
or AdmittedEvidenceV2 access, with real positive counterparts; generic helper
functions prove only the already-known newtypes. Run the sealed temporal matrix
for **StructuralCheck, Test and Snapshot**, with kind/context kept consistent:
sequential, equal-boundary and subsecond positives; reversed, overlapping and
out-of-window negatives. Preserve v1 acceptance, exact raws and RA02/RA13 limits.
Read/mutate the actual domain fixture via real loader and production projection;
no private credentials/live product values. Existing exact-pinned workspace dev
libraries may be used in owning tests if necessary, with justified locked update;
no external dependency upgrade or policy waiver. Run protocol/domain/application
owning tests and actual doctests plus all original standard/schema/structure lanes.

### S026-R1 — repeated acyclic evaluations (RF42)

Commit `fix: allow historical deployment inputs in acyclic record histories`.
The registry is an explicitly supplied **ordered definition history**; Vec order
is caller-declared creation order, not timestamp/path/kind sort or inferred trust.
A checked record references only an earlier definition in that supplied history.
Document this at the public validation boundary and preserve order in checked
history. Do not sort definitions to make future references pass. Maintain the
complete known-ID, duplicate, self, missing and directed-cycle checks; use explicit
typed failure for a forward definition. Keep cycle detection iterative and bounded
by supplied input, no recursion/unchecked public checked construction.

Retain existing kind-direction checks, with exactly these historical exceptions:
**Runtime → DeploymentReceipt** and **FrozenInput → DeploymentReceipt** may
reference an already earlier-defined receipt. They model post-deployment
observations/receipts consumed by a later input/decision, never initial readiness
waiting on its own future deployment. Evidence → Decision remains forbidden;
self/future/cyclic initial-readiness references remain invalid. No signature,
release authority, persistence or wire carrier is created by this domain check.
No new epoch, product identifier, timestamp heuristic or digest domain is needed.

Execute full initial lifecycle then earlier receipt → later runtime/input → later
decision positive controls. Reject same-kind forward definitions, future receipts,
initial readiness/deployment cycles, missing/wrong-kind references, duplicates
and self-reference. Permuting caller history to put a referenced definition later
must reject; permuting reference member supply order stays equivalent. Store
references sorted by complete typed (kind,id), rejecting duplicates, matching
references() canonical-order documentation. The original S026 ID and regression
obligations remain, followed by S027's real corpus integration. Run domain/protocol
owning tests and all original standard/schema/structure lanes.

### Final-tree gates and focused continuation (RF44)

Write/finalize code, tests, schemas, reports, ledger/status and any required mirrors
before the checkpoint's final standard/structure/diff gate and commit. A gate run
before a later report/ledger edit is not proof that the committed tree is green.
Preserve unique attempt logs and real exits; never overwrite or hide failures.
No reviewed commit amendment. Public instruction surfaces now point to DEV-011.

To reach S043 efficiently, read only current amendment + retained DEV-009 P03
allocation, current step object/required predecessor summary and relevant caller
sections. Select progress/steps entries with a small parser; do not repeatedly
load entire multi-thousand-line ledgers or full historical plan. Use compact
external receipts for resume context; they are not a second governing plan/store.
After the repairs continue S027 then all S028–S043 under the exact DEV-009
field/kind/domain/discovery allocation. Synthetic model/schema/fixture work is
eligible; Q02/Q06 real host/cohort facts stay unverified at actual dependencies.
Original hard predecessors, original acceptance and S043 independent review remain.


## DEV-010 — Accept repaired provenance and finish the P03 implementation range

**Date/author:** 2026-10-02, Codex under owner delegation.
**Reviewed source:** `49541e36942bc0649be59a065479e559fc34e1cd`.
[Independent review](reports/CODEX-49541e3.md) accepts S019/S021/S023/S024 for
allocated scope and preserves earlier acceptance: 23 numbered steps accepted.
S022's recursive concealment fix is accepted as a contribution, but RF40 remains;
S025's checked admission is retained with RF38/RF39 corrections required.
All six returned commits remain. S026–S043 were not implemented. No whole
requirement, release readiness or external fact is accepted by association.

Execute **S022-R2 → S025-R1 → every ordered eligible S026–S043** using
**pfc through EQM-P03**, endpoint S043 inclusive. Independent integration review
before S044 remains a hard boundary; full MVP remains S211. This authorizes the
entire 18-step implementation remainder plus two repairs, not a repair-only batch.
Do not stop after a commit or small group of commits. Context growth or an
estimated safe budget is not proof of a runtime limit: use supported native
compaction/context management and retain the batch range. Report an actual
forced limit precisely; never misstate a discretionary stop as observed exhaustion.
Preserve user stops, genuine external blockers and required independent gates.
No unresolved owner-only decision is needed for this allocated batch.

### S022-R2 — confine recursive Gitlink inspection (RF40)

Commit `fix: confine nested source inspection to declared roots`.
Retain all accepted root/nested dirty/concealment/HEAD/ignored/unavailable controls.
Before any child Git invocation, check every path component under the inspected
root, rejecting a symlink/non-directory ancestor or final entry, and require the
canonical child to be a strict descendant of the canonical inspected root.
Keep NUL-safe path parsing, depth/repository limits, visited roots and typed
non-authorizing errors. No network/init. Test normal nested source positively,
final and intermediate symlink escape negatively, and path names with spaces and
newlines at actual source ingress. Add a release-boundary negative control;
a dirty parent is already non-authorizing, so do not claim a clean-release exploit.
R19/R22/R30/R58 and existing S022 acceptance contributions remain unchanged.
Run application/CLI owning tests, all standard/check/clippy/schema/structure/diff
lanes before the separate commit.

### S025-R1 — preserve typed identities and ordered temporal admission (RF38/RF39)

Commit `fix: retain typed evidence context and validate retry chronology`.
Use existing ContractDigest, BindingDigest, EvidenceSpecificationDigest,
PolicySelectionDigest, RunnerDigest and AdapterDigest for their expected context
fields and checked typed access. Also bind the expected TargetConfigurationDigest
explicitly and verify its exact SHA equals the subject configuration field.
Context identities originate from already approved canonical projection/acquisition
APIs. Parse transport strings as structural SHA, compare with each typed expected
identity's as_digest, then retain/expose the respective typed expectation in the
private checked result. No public unchecked generic SHA cast, cross-type
conversion, digest-string rehash, invented digest domain or v1 representation change.
Runtime-facts/release/repository references without an existing distinct wrapper
remain exact structural SHA references and must not be called newly typed domains.
Legacy EvidenceResult generic fields may remain inside the checked wrapper for
v1 compatibility; new v2 typed consumers must use the retained typed access.
Execute positive projection-to-admission integration and compile-fail cross-domain
context/accessor controls, including runner versus adapter and contract versus
binding. Do not merely replace fixture plain hashes with unrelated constants.

For execution-bearing v2 payloads, require each numbered retry start to be at or
after the preceding retry finish; equal boundaries are allowed, comparisons retain
nanoseconds. Validate individual windows and aggregate containment as before.
Put this check at the new admission boundary (with a narrow helper if useful),
preserving established v1 constructors/readers. Add positive sequential/equal/
subsecond controls and reversed, overlapping and out-of-window negatives through
real bytes-first admission after recomputing the semantic seal. Exercise every
execution-bearing kind. Include complete one-axis context mutations: each subject
axis, optional present/absent reference, core digest, profile revision/value,
target-set membership, payload kind, unit/requirement/facet and temporal lineage.
Mutable DTO convenience entry must revalidate; raw bytes and v2 forms remain exact.
No new rule relating observed_at to execution finish is invented here; freshness
and evaluation-time eligibility remain with their owning later steps.
Counts are not retries or case-level mapping. Preserve mixed history and explicit
non-success; checked structure/context does not establish trust or minimum credit.
S059/S062–S066 still own complete case selection/count evaluation, RA02 remains
open globally. S025/S059/S061 together still own full RA13 ingress integration.
Run protocol/domain tests and doctests, all standard/check/clippy/schema/structure/
diff lanes before this separate commit. No dependency/policy waiver is authorized.

### Evidence reconciliation (RF41)

In S022-R2's planning/report portion correct S021's claim of full failed-output
retention: a successful retry overwrote the failed standard-gate destination;
only an excerpt remains in native records. Do not fabricate/relabel an excerpt as
an original full log. Use an exclusive unique attempt-numbered destination for
EVERY command invocation (including generation, failed checks and final checks),
record real process exits, and fail if a destination already exists. Keep every
failure and retry; ordinary passing checks need not be repeated without changes.

### Substantial continuation and single authority

After the repairs continue S026/S027 then S028–S043 without another routine pause.
The exact DEV-009 lifecycle and P03 field/kind/domain/source routing allocations
below remain binding. Read only the active step and its owning source/contracts;
keep compact durable progress and use native context management rather than
re-reading all history. No intermediate arbitrary commit-count endpoint.
Update progress/status/reports and sequence mirrors with exact implemented,
verified and supervisor-accepted states and a single frontier. Backfill preceding
commit SHAs. Historical receipts remain historical; do not mix an older supervisor
revision with new independent claims. Preserve all 64 R/AC and 21 RA obligations,
Q01–Q15, boundaries and original hard predecessors. Q02/Q06 actual host/cohort
facts are still unverified; they do not block synthetic P03 software work.


## DEV-009 — Checked evidence identities and complete shared-subject modeling

**Date/author:** 2026-10-02, Codex under owner delegation.
**Reviewed source:** `d6261c4c0c3fdcc082d8d7902642d364b687d7af`.
[Independent review](reports/CODEX-d6261c4.md) preserves all earlier acceptance
and accepts DEV008-P1/RF31. Retain all six new commits. Their implemented and
verified contributions are not whole-step acceptance. S001–S018 and S020 remain
accepted (19 numbered steps); remaining evidence/defects below must close before
those corrected steps are accepted. No owner-only decision is needed for this batch. Q02/Q06 remain required host/real-cohort evidence gates; synthetic model/schema implementation through S043 is eligible, but those facts are not certified by it.

### Batch and ordered repairs

Execute **S019-R4 → S021-R2 → S022-R1 → S023-R1 → S024-R1 → S025–S043**,
using `pfc through EQM-P03`. EQM-P03 ends at S043. This substantial batch completes
identity admission and the shared-source/participant/subject model, then returns
for independent integration review before S044 obligation routing. It does not
cancel the remaining MVP S044–S211. One green checkpoint is not a stop; preserve
hard predecessors, actual review/external gates, user stops and genuine runtime
limits. Context growth or a chosen 'natural boundary' is not itself a runtime
limit. Use compact step receipts and native context management; preserve the
selected range after compaction. Do not ask Codex again about decisions frozen here.

| Repair | Required result | Commit |
| --- | --- | --- |
| S019-R4 | RF28 remaining literal digest for the heterogeneous rule-order trap; independently compute its constant from frozen complete bytes and assert production output against it, retaining R3's controls. | `test: freeze heterogeneous policy ordering identity` |
| S021-R2 | RF34: execute the actual CLI runner-authority acquisition/resolution path against a temporary repository executable before/after replacement; same definition/context. Also make argv reversal differ only in argv, prove all defined field families with isolated controls and nested extension tags/set-order traps. Retain literal/typed JCS implementation and executed doctests. | `test: verify runner identity at production acquisition` |
| S022-R1 | RF36: inspect populated nested submodule indices/worktrees recursively; concealment, ignored content and unavailable inspection cannot authorize clean source. Keep existing release gate/fixture and narrow output exclusion. | `fix: reject concealed nested source changes` |
| S023-R1 | RF32/RF33: raw bytes must parse to the same record; semantic projection must use explicit existing v2 DTO forms, not v1 serialization. Preserve originals and full numeric identity. | `fix: bind raw evidence to lossless v2 assertions` |
| S024-R1 | RF35: checked lineage-to-execution/expected-plan/campaign validation with mismatch controls; prove every missing field and separate run/case independence. | `fix: validate explicit execution lineage relationships` |

Run each repair's owning tests, fmt, locked workspace all-target check/clippy,
standard, schema, structure and diff lanes before its separate commit. S019-R4
uses S019's tests/doctests; S021-R2 additionally CLI verification tests and runner/
protocol doctests; S022-R1 domain/application/CLI release lanes; S023/S024 domain/
protocol tests. No direct dependency upgrade or additional license/policy waiver.

### Source inspection closure (S022-R1)

The current top-level `ls-files -v -z` rejects concealment only in the parent
index; normal submodule status can hide modified child bytes under a child
assume-unchanged/skip-worktree flag. Codex reproduced a clean result for that
modified tree. Traverse exact Gitlink paths using NUL-safe output and inspect
all populated nested repositories without network/submodule initialization.
Reject concealed child entries and treat tracked/index/untracked/ignored child
changes conservatively. The reserved generated-output exclusion is only the
owning inspected root `.eqm/`; it is not an arbitrary exemption for nested source.
A missing/uninitialized/failed nested inspection is explicitly non-authorizing.
Verify child HEAD against its declared Gitlink and prevent symlink/root escape,
repeated/cyclic roots and unbounded traversal; limits produce typed unavailable
inspection, never clean. No changed-byte or executed-subject universality claim.

Add actual source and release-boundary tests for normal/modified nested source,
both concealment flags, ignored/untracked child source, mismatch/unavailable child
and nested depth. On this supported host submodule fixture construction works;
do not return Ok from a failed setup and count it as a passed regression. Preserve
root clean Pass, reserved output neutrality, dirty non-Pass/non-Conditional and
unconfigured Unknown controls. No blanket golden regeneration or v1 change.

### Raw/semantic identity contract (S023-R1)

The current `EvidenceResultDto::identity_relation(&self, raw_bytes)` accepts
unrelated non-JSON bytes and hashes v1 default serialization. Two legal different
attachment sizes 9007199254740992 and 9007199254740993 collide after JCS numeric
rounding. Do not narrow v1 acceptance or silently convert a v1 assertion into v2.
Remove/replace that newly added v1 relation helper, preserving established v1 APIs.

Protocol owns a bytes-first checked v2 parsing result with private fields,
read-only record/raw/semantic access and retained canonical semantic bytes. Parse
once through the existing strict v2 reader, preserving every input byte separately.
If a receiver-plus-bytes convenience method remains, reparse/validate and require
record equality before constructing the result. Invalid/duplicate/oversized JSON,
wrong exact schema and mismatched raw record reject with typed contextual errors.
A generic public domain pair is non-authorizing data, not a 'validated' admission
capability; prevent unchecked construction of the protocol checked result.

Freeze the semantic root to every existing EvidenceResultV2Dto assertion member
except `schema`, `id` and `result_digest`. This explicitly contains subject, target,
unit, requirements, facets, kind, evidence_spec_digest, contract_digest,
binding_digest, policy_digest, present runner/adapter/runtime/release references,
profile_values, producer, claimed_trust, observed_at, payload and attachments.
Root/schema/self-transport identifiers are excluded, not substantive claims.
Use current explicit v2 member forms recursively: WireUint, CanonicalInstant,
absence by omission, canonical full-member sets and ordered attempts. Revalidate
mutable DTO fields before projection; never use the v1 serializer. No semantic
extension field exists in this DTO yet: do not pretend it was included or add an
unallocated wire field. Later owning additions extend projections atomically.
Hash under existing SemanticEvidenceDigest/RawObjectDigest domains; execution
identity remains a separate required relation, not observation/file-count credit.

Use one explicit published bound per stage: existing evidence JSON ingress is
8 MiB; the generic raw-object carrier's 16 MiB cap is separate, not a claim that
this evidence reader admits 16 MiB. Test both relevant boundaries without changing
v1 ranges. Required regressions: unrelated valid JSON and non-JSON reject; alternate
whitespace/object order preserve raw differences with equal semantic bytes/digest;
large neighboring attachment/count values stay distinct with literal decimal
strings; all bound assertion axes change digest; set permutations agree while
attempt order does not; duplicates/null/noncanonical tokens reject; signed raw
bytes unchanged. Freeze a complete literal v2 semantic byte/digest oracle, computed
independently. This parsed relation itself does not verify producer trust or storage.

### Lineage and checked conversion (S024-R1/S025)

Keep existing typed PlanId/CampaignId/RunId/LogicalCaseId/AttemptId grammars and
complete parts. Add a fallible validation operation matching EvidenceLineage to
an EvidenceExecution plus explicit expected PlanId/CampaignId: exact plan,
campaign, run and logical case must match; attempt must be declared in that
execution. Expected authority is independently supplied context, not established
by these constructors or producer strings. Reject each mismatch specifically.
Missing each of five parts, empty/duplicate attempts, independent runs and cases
are separate executed tests. Never derive lineage from timestamps/directories.

S025 owns one protocol-to-domain checked conversion over the parsed v2 object,
explicit lineage, declared execution and immutable expected context. These are
separate typed inputs; do not invent wire lineage from absent transport fields.
Exact existing v2 component strings become their respective typed digest domains;
subject/scope/target/unit/requirement/facet/profile/reference expectations must
match approved context, payload kind must match the envelope, requirement units
must match, and payload runtime/release references must match the same envelope.
Target scope requires its target; target-set scope requires membership; provider
scope uses the explicitly declared provider-to-target association, never ID equality
or invented universal linkage. Subject repository/commit/build/artifact/configuration
and profile revisions/values are exact axes. Missing required references fail;
allowed absence stays typed/visible. No workspace-digest or zero-value fallback.
Require id == result_digest == recomputed semantic digest for newly checked v2
admission, excluding the two digest members from their own projection. Keep the
older structural v2 parsing API distinct from admission; synthetic representation
vectors do not become accepted evidence. Preserve all v1 bytes/reader behavior.

Validate arithmetic plus contradictions between asserted counts and terminal
outcomes (RA02), temporal ordering/window bounds, exact attempt numbering/declared
membership, and every RA13 one-axis mismatch before constructing a private-field
checked result. Do not equate selected logical observations with retry count.
Mixed retry history remains representable and non-success; incomplete case/outcome
mapping cannot create minimum-count credit. S059/S062–S066 retain full selection/
aggregation integration; S025 implements its boundary contribution without claiming
those are complete. Route actual new v2 admission through this boundary, with a
bytes-first entry and a separately revalidating DTO entry if needed. Old v1 CLI
consumer transition is owned later; do not route legacy fixtures through implicit
conversion. No parsing/lineage/digest creates verification trust. New field/schema
changes happen only in owning steps with explicit version dispatch and parity.

### Acyclic references and complete corpus (S026/S027)

Use typed record-kind/reference distinctions and validated already-defined
references for the lifecycle in EVIDENCE_AND_IDENTITY.md: authority/candidate →
plan/campaign → evidence/worker/runtime → frozen input → decision → statement →
authorization → deployment receipt. Validate known IDs, allowed direction,
self/future/cyclic references against an explicit supplied registry. Evidence
references plan/candidate, not its future decision. Post-deployment evidence
belongs to a later input/decision. Preserve separate digest domains; no timestamp
or directory ordering substitutes for references. Public checked records cannot
be built by unchecked deserialization. No storage/publication is implied here.

S027 includes every repaired axis and literal oracle plus negative bypass tests
in the real domain/protocol/manifest boundaries, not merely a declarative vector
inventory. Actual v2 selector ingress must call its typed helper when introduced.
Keep deterministic fixture generation and original raws; no live credentials.

### EQM-P03 representation and sequencing decisions (S028–S043)

These close engineering allocation within the existing model; they do not define
real product cohorts/business rules. Exact operational values remain explicit gates.
Domain/engine remain pure; manifest owns authored projection/validation, protocol
owns wire carriers, application owns effects. No dependency back-edges.

- S028: RoleId is an authored extensible symbolic identity, not a three-role enum.
  Target retains stable id/root/platform/framework/owners/extensions and adds
  explicit role metadata. Preserve the existing constructor/v1 projection; legacy
  absent role is typed absence, never an invented role. V2 role-bearing metadata
  requires an explicit role; target v2 projection adds `role` when present.
- S029/S030: SourceComponentId, confined canonical root set, owner set, explicit
  dependency-ID set and semantic extensions. Domain/root projection members are
  `id`, `roots`, `owners`, `dependencies`, `extensions`. Target adds explicit
  `source_components` ID set and `depends_on` target ID set, bound in target v2
  identity when modeled. Validate dangling refs/cycles/duplicate authority; compute
  transitive ownership deterministically, not filename guesses. Raw source content
  remains a separate subject axis. No default shared-source authority.
- S031/S032/S033: Explicit workspace v2 reader/DTO uses existing workspace fields
  plus `source_components` keyed by ID, with existing targets keyed by ID carrying
  `role`, `source_components`, `depends_on`. No implicit v1/v2 conversion. New
  manifest families use the existing schemas/v2/manifest base with exact kind
  filenames allocated below; generate actual schemas, not placeholders. Keep v1
  schemas/readers/oracles frozen. Overlaps are permitted only when their complete
  overlap is explained by the same declared source component dependency closure;
  undeclared/ambiguous/escaped ownership remains an error. Distinct target IDs and
  configurations retain distinct shipped output identities. Shared paths never
  imply identical artifact/builds or universal ownership. Test real loader inputs.
- S034/S035/S036: Participation rules bind exact fully-qualified `requirement`,
  explicit `targets` and existing typed `applicability`, with unique `id` and
  semantic `extensions`. Use `participation` v2 manifest kind. Reject duplicate/
  ambiguous authority for the same requirement/target; exact profiles come from
  the immutable selected context. No undeclared target/profile. Evaluate at each
  coordinate; explicit false may exclude, unknown remains visible/non-authorizing,
  and unspecified required context is unknown. Do not alter old v1 evaluation
  implicitly or synthesize a global drop/waiver.
- S037: `scenario` v2 manifest kind, exact id/revision, `participants` with unique
  occurrence `id`, explicit `role`, tagged target/provider `subject`, and explicit
  directed `dependencies` between occurrence IDs, plus semantic extensions.
  Participants/dependencies are canonical sets by complete member bytes; repeated
  target/provider references with different occurrence IDs remain distinct.
  Roles/refs/dependency mappings must validate; no target-ID set collapse.
- S038: Exact subject assignments keyed by occurrence, retaining separate
  repository/source commit and immutable source/component-content identity,
  target/provider configuration, build ID and artifact digest. Use explicit
  `subject-manifest` v2 protocol kind. Absence is omitted wire member/typed absent
  domain state only for declared development context; trusted completeness requires
  every relevant axis and never zero placeholders. One participant's valid build
  does not need to equal another's. Later preparation/acquisition/trust owns actual
  authorized material, not parsing or synthetic fixtures.
- S039: `comparison-context` v2 protocol kind; root `contract_digest`,
  `scenario_digest`, `profiles`, `comparison_rule_digest`. Profiles retain exact
  selected definitions/revisions/logical values under the approved selected-profile
  projection. Distinct participant build/source/artifact assignments do not enter
  this shared identity; subject/evaluation identities remain separate. Use distinct
  `eqm:v2:scenario`, `eqm:v2:subject-manifest`, `eqm:v2:comparison-context` domains
  when introduced, with typed carriers and literal oracles; no aggregate fallback.
- S040: Explicit provider records/configuration and target/scenario dependency IDs;
  attach provider obligations only to declared dependents. Exact provider subjects
  enter the subject manifest. Missing required provider remains non-authorizing;
  an unrelated provider is not attached to every target. Operational endpoints/
  credentials remain explicit unavailable inputs, not guessed defaults.
- S041: `cohort` v2 manifest kind with id/revision/scenario, explicit participant
  occurrence assignments to exact subject/build identities, and semantic extensions.
  Validate required participant coverage and declared combinations. N/N+1 are
  synthetic positive/negative fixture data, not a real supported-version promise.
- S042/S043: synthetic CLI/desktop/mobile/provider fixture, shared components,
  repeated occurrences, role-specific requirements, exact subjects and explicit
  cohorts. Update public usage/schema inventory and real loader/application parity
  controls atomically. New target fields always extend its v2 projection in the
  introducing step; new wire families use explicit local dispatch, bounds, NFC,
  omission/null, exact numeric/time rules and full-JCS sets, without network reads.
  Preserve v1, ordered journeys/argv/extension arrays, and original signed bytes.

Authoring allocation: new participation/scenario/provider/cohort documents use
existing contract_sources discovery with exact v2 kind dispatch; do not invent
new CLI flags or workspace source-list aliases. Source components remain embedded
in workspace v2. Provider manifest kind is `provider`, with id,
configuration_digest, owners and semantic extensions; explicit target `providers`
ID sets are added/bound at S040, scenario occurrences already carry provider
references. New reference fields are canonical sets, with no automatic dependency.
Distinct `eqm:v2:participation`, `eqm:v2:provider-configuration` and
`eqm:v2:source-snapshot` domains are allocated at their owning steps as needed;
actual source/configuration verification remains independently required. Include
participation in its own exact component projection and later authority linkage;
do not silently alter S019's frozen ContractDigest root. For newly modeled graph
semantics, provide explicit v2 semantic graph summary/projections in owning steps
under `eqm:v2:semantic-graph`; old v1 projection must reject a v2-only graph rather
than omit new normative fields or silently change legacy bytes. Actual supported
schema inventory expands only when an owning step implements its reader/DTO;
no placeholder schema or relaxation of exact parity is authorized.

New local helper names and module placement are routine mechanics. Apply the
member/digest/version decisions above and the original read lists/tests. If a
truly new product/compatibility/authority decision is evidenced, return the exact
conflict and smallest alternatives; do not re-ask already resolved allocations.
All original per-step acceptance and RA obligations remain; the table does not
replace them or grant whole-feature acceptance.

### Evidence and cursor discipline

RF37: current scalar next_step/active_step still say S019 while next_checkpoint
says S025; historical supervisor metadata also mixes independent and new self-verification. Reconcile one
implementation frontier plus distinct acceptance status; preserve original commits
and accurately backfill S024. Cycle counts distinguish retained implementation
from accepted work. Never change historical supervisor source/check metadata to
look like new independent verification. Distinguish dedicated commands from checks
inside the standard gate; S023/S024 have schema parity there, not evidence of
a separate schema-check command. Preserve actual native/raw failure evidence.

Use unique attempt log names, exact command, exit and source/tree identity; fail
fast between dependent gates and serialize build-router workloads within a repo.
Never commit from a shell pipeline that masks a failed Git exit. Do not amend
reviewed commits; repair later with a new checkpoint. Reports/progress updates
are not a stop. New code remains supervisor acceptance pending until reviewed.


## DEV-008 — Finish source provenance and continue the eligible MVP

**Date/author:** 2026-10-02, Codex under owner delegation.
**Reviewed source:** `eb2f323e459423fd06d25beffa7f6f85a1077af4`.
[Independent review](reports/CODEX-eb2f323.md) accepts RF27 and S020 for allocated
scope, retains S019-R2 with a small evidence gap, and retains S021/S022 with
changes required. S001–S018 and S020 are accepted (19 numbered steps). S019's
main implementation remains; no history is rewritten. This amendment supersedes
stale cursors and S022's decision block. No owner-only decision is needed now.

### Ordered work and batch boundary

Execute **DEV008-P1 → S019-R3 → S021-R1 → S022 completion → every ordered eligible S023–S211**.
The selected batch is the full remaining approved MVP, ending at S211. Each
checkpoint remains independently green and committed; a commit/report/context
growth is not a stop condition. Continue after the repairs and after each later
green checkpoint. Hard predecessors, explicit independent-review gates, actual
runtime limits, user stops and genuine unavailable inputs/authority remain binding.
Do not skip a blocked predecessor. If another approved slice is independently
eligible under existing dependencies, continue it; do not invent a new order.

Codex owns material requirements, representation decisions and independent
acceptance. Pi owns product code/tests and routine internal mechanics. A new
material decision must return with exact evidence and smallest alternatives;
Pi cannot waive criteria or promote its own verification to Codex acceptance.
S143 is intermediate. Q01–Q15 remain at actual dependent steps, not blanket
hardware blockers. Synthetic inputs prove software behavior only. No push,
publication, deployment, signing, live enforcement or optional native activation.

### DEV008-P1 — RF31: repair the newly yanked transitive dependency

Current cargo deny rejects locked yoke-derive 0.8.3. Official registry metadata
lists 0.8.4 unyanked, MSRV 1.82, compatible with the current toolchain and yoke
0.8 dependency range. Codex selects 0.8.4. Perform a narrow transitive Cargo.lock
update (`cargo update -p yoke-derive --precise 0.8.4`), inspect all resulting
lock changes and preserve exact direct pins. No dependency-policy waiver, broad
upgrade or new license exception. One intentionally unlocked resolution is
authorized; all subsequent tests/checks stay --locked. Verify the resolved graph
with cargo deny check and cargo audit --deny warnings plus workspace standard,
schema and relevant protocol/test-support schema-validator tests. If that precise
resolution is impossible or exposes a new policy failure, return actual evidence
to Codex rather than changing pins/gates. Record the failed baseline and exact
replacement; commit `build: replace yanked schema validator dependency`. This
prerequisite does not renumber the original 211 steps or pretend prior trees
passed today's registry state. Continue the next repair after green verification.

### S019-R3 — RF28's remaining proof (not new product semantics)

Retain all S019-R1/R2 code and regressions. RF27 is closed: shared applicability
projection correctly orders full JCS member bytes and preserves domain sets/v1.
RF28 is largely repaired but these assertions still need real production output:

- `policy_rule_endpoints_use_literal_v2_wire_forms` checks minimum `1` by calling
  boundary_expected(1), not by projecting a minimum-valued rule. Project actual
  minimum and legal maximum rules; use reviewed literal expected JSON tokens for
  both age/count. Replace computed boundary_expected helpers in contract and
  policy revision tests with a literal (input, expected JSON value) table.
- `rule_order_uses_full_member_bytes` compares two outputs but does not assert
  reviewed literal order/digest; the chosen facets follow ordinary enum order.
  Use a heterogeneous two-rule trap where full JCS order differs from domain
  ordering (e.g. optional selector presence versus minimum_age/count ordering),
  assert the exact expected member order and permutation-equal bytes/digests.
- `populated_dimension_description_is_present_and_absent_exactly` asserts only
  presence. Add a same-ingress absent control and require omission, not null.

No whole AC/RA closure follows. Scope is owning manifest tests/reports and any
existing approved-projection defect they expose. Run S019's declared tests and
commands plus structure/schema checks; commit
`test: finish literal policy provenance controls`. No ordinary successor is
folded into this repair. Mark new implementation acceptance pending, then continue.

### S021-R1 — RF30: approved v2 runner identity

S021 successfully binds executable bytes, definition and approved guarantees,
but invents `eqm:v1:runner-identity` with a private binary encoder and returns
untyped SHA-256 while the frozen RunnerDigest domain is `eqm:v2:runner`.
Keep stronger executable binding; close this representation now, before new
subject consumers. Do not publish that invented v1 identity as the new contract.

Choose runner-owned pure JCS projection over the resolved definition/program and
approved enforceable guarantees, using existing dependencies. No manifest-to-
runner/engine/protocol back-edge and no domain serialization dependency. Root is
exactly `definition`, `program`, `approved_guarantees`. `definition` includes all
RunnerDefinition fields: id, revision, owners, backend, declared program, ordered
args, cwd, environment, secret references, timeout_ms, max_output_bytes,
max_concurrency, declared guarantees, semantic extensions. Approved guarantees
are actual independently supplied context; do not hash hypothetical host claims
or invent toolchain versions. This digest proves no downstream wrapper-tool
immutability; owning execution/subject steps must still bind actual inputs.

Use explicit v2 fields, not default Serde/v1 projection rehashing. Definition
program is `{kind:"repository",path}` or `{kind:"locked",resolved,digest}`.
Resolved root program retains that form plus independently verified digest for
repository executables. A resolved repository path must match the declaration;
locked identity/digest must match exactly. Unresolved programs still reject.
Args are tagged records: `{kind:"literal",value}`, `{kind:"target_root"}`,
`{kind:"selector_json"}`, `{kind:"result_path"}`. Cwd is tagged likewise as
`{kind:"target_root"}`, `{kind:"repository",path}`, `{kind:"result_path"}`.
This distinguishes authored literal placeholder text from substitution semantics.
Environment is the existing complete `{name,source,value?}` record set; literal
value present only for literal sources. Secret references are `{name,provider}`
sets, never resolved secret values. Owners, environment, secrets and both guarantee
collections are full JCS member-byte canonical sets; args and extension arrays
retain order. Counts/revisions use ADR-0013 integer/string rules. Extensions use
recursive DEV-006 tags and exclude only the display namespace. Optional absence
is omission; exact validated strings preserved. Bound projection bytes using the
existing component limit, with typed errors, no fallback to a definition hash.

ResolvedRunner stores immutable bytes and RunnerDigest; add canonical_bytes() and
identity_v2() accessors (names frozen here). Existing digest() can remain an
explicit legacy/wire adapter returning identity_v2().as_digest(); document that
boundary. Internal new v2 consumers use RunnerDigest. No existing DTO shape,
schema/v1 bytes or v1 graph/fragment/definition oracles change. Changes to runtime
runner hash values are expressly approved in this scope and must be reported;
there is no silent legacy alias/fallback. Preserve any independently needed old
v1 definition projection as v1; it is not authorization for v2 evidence.

Required tests: reviewed literal complete bytes and independently computed
`SHA256("eqm:v2:runner" || 0x00 || bytes)`; typed positive/cross-domain compile-fail
proof with executed doctests; actual temp executable replacement through the
existing resolution/acquisition path with freshly independently observed digest;
same-definition content change, definition-only change, approved-context change,
repository/locked cases and unresolved rejection. Also full-range quantities,
integer/text extension distinction, placeholder-literal versus template, display
exclusion, set permutations and argv order. Do not claim actual executable
reading from a test that only mutates an authority map. Keep map-boundary tests
as separate controls. Run runner/protocol tests, relevant doctests, standard,
schema, structure and diff checks. Commit
`fix: align resolved runner identity with v2 provenance`.

### Source provenance decision (S022)

Select a conservative repository-wide inspection gate, not application-only
exposure and not source-root narrowing. Clean release provenance requires no
tracked/index changes and no untracked or ignored content outside the reserved
untracked generated `.eqm/` output tree. Tracked changes remain dirty even inside
that tree. `.gitignore` is not permission to conceal source; ignored source or
outputs elsewhere conservatively make this inspection dirty. Source-root-only
inspection could miss build files, EQM authority, lock files and tooling.

Retain normalization and typed SourceCommit. Inspect the explicit repository
identity/root, disable status shortcuts that conceal changes, and handle Git
NUL-delimited paths without whitespace/newline ambiguity. Include staged,
unstaged, deleted, untracked, ignored-outside-output and submodule changes;
`--ignore-submodules=none` (or equivalent). Reject assume-unchanged/skip-worktree
tracked entries as non-authorizing inspection, since normal status may conceal
modified bytes. Missing/failed Git or unsupported identity remains a typed
error, with retained causal errors where possible, never clean/default provenance.
The existing SourceProvenance::clean() means this defined inspection passed,
not universal equality of all bytes nor proof of what executed. Correct its
current broad 'exactly matches HEAD' claims. Races/actual execution binding remain
with immutable prepared source/subject and acquisition steps; no trust promotion.

Wire clean() into release context completeness now. A dirty or failed inspection
cannot produce Pass or Conditional; successful underlying tests do not override
this. Retain structured non-authorizing diagnostics/exit behavior. Local development
may inspect dirty provenance but cannot relabel it as clean HEAD. Attest/verify
must not fabricate a clean executed-subject claim from only an inspected commit;
later subject steps retain explicit dirty/immutable context. Do not add a v1 wire
field or reinterpret signed originals to bypass this gate.

Repair the fixture, not the requirement: materialize generated release/runtime/
evidence records in the reserved `.eqm/` tree, ignored by a fixture `.gitignore`
written before its initial commit. Adjust actual fixture input paths/commands and
all related integration callers atomically. Do not add broad ignore patterns for
source, tests, authored manifests or arbitrary release directories. Do not commit
the generated artifacts as source to hide the issue. Codex authorizes regeneration
of only affected fixture goldens via repository-owned generators after proving the
input change, preserving the original clean-pass/fail/unknown acceptance. Record
old/new hash and exact cause if any oracle changes; no blanket golden refresh.
A changed source commit due to fixture setup is handled by its existing dynamic
subject construction, not hardcoded as production provenance.

Required production-boundary controls: genuinely clean fixture Pass with required
synthetic trust; unconfigured trust Unknown; generated untracked `.eqm/` inputs
remain neutral; same-HEAD tracked source modification, staged change, deletion,
non-output untracked file and ignored non-output source make release non-authorizing.
A tracked modification inside `.eqm/` also fails. Include Git concealment flags,
submodule dirtiness where constructible and newline/path controls for the parser.
Missing origin/invalid repository failures remain explicit. Never create hardware
or credentials as a prerequisite for these software tests. Run the full S022
declared domain/application/CLI lane, owning release/fixture tests and standard,
schema, structure/diff checks. Commit
`fix: bind source provenance beyond clean head claims`.

### Progress, review and continuation

Backfill all five retained implementation SHAs, checkpoint contributions and
current scalar cursors consistently. S019-R3 owns remaining RF28; S021-R1 owns
RF30; S022 is engineering-unblocked by this decision. S019/S021 retain committed
implementation with changes required, not nonexistent work. RF27/S020 acceptance
is independent; later corrections still need Codex acceptance. Keep one active
checkpoint and accurately recompute cycle counts. Reports distinguish implemented,
verified, incomplete/blocked and independently accepted. Preserve all original
R/AC/RA criteria and failed logs. Record actual command, source HEAD and exit for
unique log attempts; do not overwrite failed raw output or infer historical passes.

After green repairs/S022, continue eligible S023–S211 under their original scope,
read lists, tests and dependencies. No extra supervisor pause follows a routine
green commit; genuinely required independent review gates remain binding. Material
new semantic choices belong to Codex; ordinary implementation mechanics belong to
Pi. A missing real product/host credential gates only its dependent claim/work,
not all software by default. No publication or operational activation is granted.


## DEV-007 — Correct applicability sets and close provenance evidence

**Date/author:** 2026-09-28, Codex supervisor under owner delegation.
**Reviewed source:** `1512c9fa7e886781bdf2976dfaabde1d8328ac43`.
See [the independent review](reports/CODEX-1512c9f.md). This amendment supersedes
contradictory DEV-005/DEV-006 all/any classifications and stale completion cursors;
all other approved representation, authority and scope decisions remain binding.
Codex accepts RF22/RF24/RF26 and the actual loader/display-duplicate portion of
RF25. RF25's impossible authored-order demand is replaced by RF27 below. S016 and
S018 are accepted for allocated scope; S017 retains its implementation but needs
RF27; S019-A/B/C are retained with corrections/evidence required. No history rewrite.

### Ordered corrective checkpoints

| Checkpoint | Exact scope | Commit |
| --- | --- | --- |
| S019-R1 | RF27: correct v2 all/any canonical set ordering in both binding and contract projections; genuine permutation/duplicate/order-trap controls; correct source/test/report descriptions. Closes the remaining S017 contribution too. | `fix: canonicalize applicability children as semantic sets` |
| S019-R2 | RF28: finish the concrete S019 boundary matrix below and reconcile each claim to an executed test and log. | `test: complete contract and policy provenance boundaries` |
| S020 | Implement the resolved current-target configuration contract below. | `fix: derive exact target configuration identity` |

Run each checkpoint's fmt, locked workspace all-target check, owning tests,
clippy with warnings denied, doctests, standard gate, schema parity, package
structure validator and diff check before its independent commit. R1/R2 use
S019's domain/manifest/engine/application test and doctest commands. S020 uses its
own declared commands. Preserve v1 bytes, all schemas and Cargo.lock unless a
later owning step explicitly requires change. No new dependency is necessary.
S019 becomes implementation-complete only after R2; S017's remaining contribution
is complete after R1. Record supervisor acceptance as pending for new code, then
continue all ordered dependency-unblocked work through S211 without an extra
approval pause. S143 is intermediate, not a stop target.

### RF27: applicability is a semantic set

The domain already uses BTreeSet for all/any and rejects duplicate children.
Codex's prior ordered-child instruction was inconsistent with this inspected
model. Retain the domain representation and authored rejection behavior. All/any
children are **unordered semantic sets**, like membership values. In BOTH v2
binding/exposure and contract/requirement projections, recursively project each
child, order by its complete JCS member bytes, and reject projected duplicates.
Do not use derived Rust enum ordering as the wire ordering. Do not flatten,
simplify, deduplicate, or otherwise equate structurally distinct expressions;
not remains a single child. Journey surfaces and extension arrays remain ordered.
V1 serialization and domain evaluation do not change. Shared pure projection
helper extraction is permitted to keep the two v2 paths identical.

Required controls through both public canonicalizers: all and any with identical
child objects supplied in opposite input order produce identical bytes/digests;
a real child-value change differs; duplicate identical children reject at the
existing domain boundary with valid controls. Include a heterogeneous ordering
trap: a comparison and a nested all child (and equivalent coverage for any).
Derived enum order puts Comparison before All, but JCS member bytes beginning
`{"all":` precede `{"dimension":`. Assert the reviewed literal child bytes/order,
not merely equality between two identically wrong implementations. Cover nested
all/any/not and membership sets. Keep journey/extension sequence sensitivity.

The old binding test swaps values between fixed region/tier dimensions; this is
a semantic mutation, not a child permutation. Rename/retain it for what it proves
and add real permutations. Correct source comments and S017/S018/S019 reports.
Only affected unpublished v2 composite-expression oracles may change, with old
values/reason retained and independently recomputed expected hashes. Current six
base/boundary component oracles contain no such composite trap and stay unchanged.

### RF28: concrete missing S019 evidence

Retain all existing controls and add the following at owning public boundaries:

1. Contract revisions and policy/profile revisions at 9007199254740991,
   9007199254740992 and u64::MAX; policy maximum_age/minimum_count at their legal
   endpoints. Assert exact integer versus decimal-string payloads with literal
   expected values, not a second call to the production serializer. Exercise
   populated optional requirement risk/provider, fragment-origin prefix, dimension
   description, waiver maximum_days and all five rule-selector axes; mutation or
   exact literal assertions must fail if a field disappears. Preserve absence.
2. Real prepared-session policy/profile requests for missing exact policy revision
   and missing exact profile revision while another revision exists; typed errors
   must survive without latest fallback. Known versus unknown on the SAME profile
   definition must differ in canonical bytes/digest; unknown stays unknown despite
   an available default; omitted dimensions still reject at the carrier boundary.
3. Canonical set ordering using at least two selected profiles, two rules and two
   contract records with an order trap where ID/derived order differs from full
   member bytes. Reverse supplied members and assert reviewed canonical order and
   identical identities. Existing one-profile fixtures do not prove this.
4. Load actual manifest variants through prepare: comment/formatting-only changes
   preserve both identities; unselected policy/profile semantic changes preserve
   both identities; selected-profile same-ID/revision semantic mutation changes
   only policy identity. Assert replacement really changed the fixture text.
5. Prepare once, then mutate/remove the backing authored file: the existing session
   yields the same bundle while a fresh preparation observes the mutation or
   fails. This proves reuse of the immutable prepared graph. Preserve existing
   contract/policy isolation and mode/authority tests; traverse Error::source for
   reachable selection errors. Do not fabricate unreachable carrier errors in
   the application using unchecked constructors.

These are previously required S019 acceptance contributions, not whole AC019 or
RA13 completion. No new product semantics or gate waivers are authorized.

### S020: exact currently available target configuration

Choose a **target-definition component identity**, separate from selected-policy,
source snapshot, build artifact and execution identities. At this step Target
contains exactly id/root/platform/framework/owners/extensions and has no build,
profile or source-component reference fields. Do not invent such fields, infer
values from host tools or arbitrary extensions, or fold the workspace/policy
summary into TargetConfigurationDigest.

In eqm_manifest add pure project_target_configuration_v2 and
canonicalize_target_configuration_v2 over &Target, returning immutable
CanonicalTargetConfigurationV2 bytes plus the existing TargetConfigurationDigest.
Its internal JSON root has exactly `id`, `root`, `platform`, `framework`, `owners`,
`extensions`, without a schema URI or nested target wrapper. Strings retain their
validated exact values; root remains the authored RepoPath (no absolute expansion,
filesystem canonicalization or case folding). Owners are a complete member-byte
canonical set. Extensions use recursive DEV-006 type tags and exclude only the
named display namespace. Enforce the existing projection byte bound and typed
canonicalization errors. No new domain carrier is needed for already-valid Target.

Add a pure application entry accepting &PreparedSession and &TargetId, looking up
that exact target in that same finalized graph and invoking the manifest API.
Return the typed canonical result; distinguish MissingTarget (with requested ID)
from Projection errors with causal Error::source. No graph/file reload, selection,
clock read, tool execution, empty default target or workspace-digest fallback.
An unavailable target is an error, never a hash of empty/missing configuration.

There are ZERO build/profile/source-component references to project from Target
at S020. Their absence is stated in the API documentation, report and provenance
coverage, not encoded as null, invented empty collections, successful verification
or an invented 'none' authority. Existing selected policy/profile definitions stay
bound by PolicySelectionDigest. Source revisions, build IDs/artifact digests and
resolved source inputs remain separate subject axes in S022/S023/S038; obtaining
a target digest does not establish them. Consumers must not use it as full build
or trusted-subject identity. This resolves the reported decision, not a waiver of
later source/build requirements.

Tests: literal target bytes and independently recomputed domain hash; typed digest
API; same-ID root/platform/framework/owner/semantic-extension mutations differ;
display-only and real authored formatting changes do not; owner permutations
canonicalize; large integer/text extensions remain distinct; exact target lookup
and missing-target typed error; changing another target or policy leaves this
identity unchanged; same-session file mutation does not change the prepared target.

**Later closure:** when S028 adds role and S029–S032 add source-component relations,
update the target projection in the SAME owning step to bind all newly available
normative target fields. Role uses its exact validated ID. Source-component ID
references use a canonical set of exact IDs; changes to referenced source content
remain separately bound by exact subject/source identities. Do not silently drop
new fields or replace this identity with a workspace digest. S031/S038/S043 must
prove this linkage together with their existing source-schema/subject/negative
cases. New incompatible published wire shapes remain governed by ADR-0013; these
internal, unshipped v2 construction oracles are updated explicitly with reasons.

### Receipt and continuation discipline

RF29 is reconciled by this supervisor amendment: current counts/cursors and all
six commit SHAs are backfilled, and known log-reference discrepancies are recorded
without fabricating historical exits. Update progress steps, checkpoint SHAs,
cycle counts, local verification, next/active checkpoint, STATUS and reports
consistently after every commit (backfill a commit's own SHA at the next update).
Use distinct attempt-numbered log files; record the exact command, HEAD and exit
at execution time. Do not overwrite failed output or label a later rerun as proof
that an earlier tree passed. Missing historical exit evidence remains unverified.

Pi makes implementation-mechanic choices only. Codex owns new material semantics,
scope, sequencing and acceptance. Continue compact checkpoint execution through
all unblocked S021–S211 after the above work. Newly evidenced decisions go back
to Codex; unavailable product/host/credential/policy facts remain Q01–Q15 at their
first dependent step. No release candidate or human release-testing dependency
exists yet. No push/publication/deployment/signing/live activation or optional
native scope is granted.


## DEV-006 — Close boundary evidence and unblock contract provenance

**Date/author:** 2026-09-28, Codex supervisor under owner delegation.
**Reviewed source:** `3469face3a88d280f2577f4ff936419408c1d248`.
The four implementation commits are retained. RF20/RF21 are accepted; RF22 is
partially repaired, not closed. New RF24/RF25/RF26 concern typed component identities,
missing manifest-boundary proof and extension type collisions. See [the review](reports/CODEX-3469fac.md).
S019's reported representation decision is resolved below. No external product
fact or user approval is required to implement these decisions.

This amendment supersedes obsolete cursors and completion claims. S001–S015
remain accepted; S016 is active for its remaining tests, S017/S018 have retained
implementation with changes required, and S019 is pending implementation with its
decision resolved. Preserve 211 numbered IDs, all original dependencies, v1
behavior and original receipts. Current independent standard gate (393 tests),
dependency policy, live advisory audit and package structure pass. That does not
close missing acceptance evidence.

### Ordered checkpoints before S019

| Checkpoint | Scope | Green / commit |
| --- | --- | --- |
| S016-R3 | Finish RF22's exact matrix below; correct the diagnostic-list claim and retain R1/R2 behavior and evidence. | Complete declared S016 commands plus final structure validation; `test: close collection and diagnostic boundary evidence`. |
| S018-R1 | Close RF24/RF26 for both S017/S018 projections: return existing typed digest wrappers and encode extension type tags to prevent integer/string identity collisions; add literal mutation controls. | Locked domain/manifest all-target tests, manifest doctests including type-boundary proof, fmt/check/clippy, standard gate, schema parity, final structure/diff checks; `fix: preserve extension types in component identities`. |
| S018-R2 | Close RF25: actual loader/comment/formatting, reachable projected-duplicate and ordered-expression controls; correct the projection reports. | Same domain/manifest/standard/schema/structure/diff gates; `test: prove component projections at manifest boundaries`. |

One independently green commit per checkpoint; do not amend reviewed history.
S016-R3 completes S016's implementation contribution. S018-R1/R2 complete the
remaining S017 and S018 contributions together because they share the projection
API boundary. Record their supervisor acceptance as pending after implementation,
then immediately continue S019-A/B/C and unblocked successors through S211.
No extra approval pause is required between those objective gates.

### RF22: concrete remaining regression matrix

Keep all existing successful controls. The required missing cases are:

| Owning root / collection | Add or complete |
| --- | --- |
| EvidenceResult facets | Exact duplicate rejects; retain existing permutation proof. |
| EvidenceResult attachments | Two valid reversed members produce identical canonical bytes; exact duplicate rejects; retain differing-digest same-name rejection and the member-order trap. |
| EvidenceResult profile_values | Exact duplicate rejects; retain permutation, logical-key and conflicting-revision checks. |
| RuntimeFacts profile_values | Exact duplicate and same (profile, dimension) with a different value reject; retain permutation/digest and conflicting-revision checks. |
| Inventory entries, standalone and embedded AdapterResponse | Exact duplicate rejects at both owning paths; retain differing-attribute same-key and permutation controls. |
| Inventory diagnostics and outer AdapterResponse diagnostics | At least two distinct diagnostics swap order and change canonical output; repeated diagnostic records retain multiplicity. Test the diagnostic vector itself, not only its related-location vector. |

Retain target-set, requirements, facts, valid/invalid attempt numbering,
related-location order, authored text and raw-byte controls. Every negative has
a valid control through the same reader. Recompute mutant runtime-facts digests
without hiding serialization failures; remove the test helper's silent
`unwrap_or_default` fallback. Exact-member rejection is distinct from later
logical-key rejection. Compare JCS of returned semantic DTO values for canonical
byte claims; normal Serde output may additionally prove stable serialization.
Do not claim new component/relationship/storage acceptance from these tests.

### RF24/RF25/RF26: typed identities and actual projection boundaries

CanonicalBindingV2 must store/return BindingDigest, and CanonicalEvidenceSpecV2
must store/return EvidenceSpecificationDigest, using those existing wrappers'
hash_content constructors. The wrapper-only change preserves bytes and digest text; RF26 separately requires
the explicit extension representation correction below. Raw
Sha256Digest extraction is explicit through as_digest only at a wire/legacy
boundary, never the generic internal return type. Add typed positive API checks
and a compile-fail doctest showing cross-domain substitution fails; execute the
doctests. Preserve the v1 generic digest APIs and existing domain labels.

**RF26 — extension type distinction:** support::extension_value_v2 currently
encodes a large Integer as a JSON string while String emits that same JSON
string. For example Integer(i64::MIN) and String("-9223372036854775808") yield
identical bytes and digest when all other fields match. Hashing complete
semantics requires retaining this type distinction.

Freeze this v2 extension representation now: every ExtensionValue is an object
with exactly `type` and `value`. Tags are `boolean`, `integer`, `string`, `array`,
and `object`. Boolean/string payloads are their exact validated values; integer
payloads use wire_int; array payloads are ordered arrays of recursively tagged
ExtensionValues; object payloads are JCS-keyed maps of recursively tagged
ExtensionValues. The extensions namespace map itself stays a map, excluding
only dev.equivalencematrix.display. Tag each namespace value recursively; do not
tag only out-of-range integers, rely on schema inference, or parse strings that
look numeric. This also distinguishes an authored object resembling a tag from
the typed value it resembles. No v1 extension representation changes.

Add same-ID integer-versus-text cases for both projections, covering positive
and negative out-of-safe-range values and nested arrays/objects; require different
canonical bytes and typed digests. Include literal base and boundary byte/hash
oracles for the new representation. Codex authorizes updating ONLY the affected
new v2 extension goldens; retain old values in the report with the exact reason.
Empty-extension base bytes/digests and all v1 goldens stay unchanged. Recompute
expected hashes independently from reviewed literal bytes. No digest-domain
label or published schema family changes. This correction is also the extension
representation for S019 and all later v2 component projections.

The current parse-provenance test constructs the same Binding twice; it does
not alter or load a manifest. Add actual temporary authored fixtures through
load_workspace/conversion: a valid original, a comments/whitespace-only change,
and a semantic selector/path change. First two yield identical v2 canonical
bytes/identities; the semantic mutation differs while IDs remain fixed. Keep
real display-extension controls, including an embedded entity level.

Correct the assertion that canonical duplicates are unreachable. Binding::new
rejects equal Exposure values, but two exposures with different display-only
extension values can both pass construction and become identical after display
exclusion. Exercise that public path and assert DuplicateMember. Keep two
semantically different exposure controls accepted, and prove reversing ordered
applicability all/any children changes the projection rather than sorting them.
Do not add unchecked constructors merely to manufacture a negative case.

### S019 decisions: exact contract projection

Choose contract authority **plus normative graph extensions**. WorkspaceGraphInput
already calls these extensions normative; excluding them would drop bound
semantics. The v2 root has exactly `capabilities`, `journeys`, `surfaces`,
`fragments`, and `extensions`. Input is the prepared FinalizedWorkspaceGraph,
including all loaded exact fragment revisions and expanded surface requirements;
do not choose a latest revision or silently narrow it to the current CLI filter.
CanonicalContractV2 owns immutable JCS bytes and ContractDigest. Version/domain
comes from the v2 API and existing digest label; do not invent a published schema
URI for this internal projection or reuse the v1 semantic-graph schema marker.

For each entity include the existing canonical capability/journey/surface/fragment
field inventory and all nested requirement/fragment-origin/transition fields.
Include titles, descriptions, owners, statuses, risk, applicability, providers,
revision/pin metadata and semantic extensions. Only the named display extension
namespace is excluded; titles/descriptions are not newly declared display-only.
The manifest file path, comments, whitespace and source-map spans are not inputs.
Exclude targets, bindings, policies, profiles, runners, waiver records, imports
and adapter lock records as top-level contract components. Their own identities
and later provenance retain those axes; fragment use pins and included fragment
semantics remain in the contract. Graph-level semantic extension changes do
change ContractDigest by this explicit decision.

Build explicit v2 entity projections. Reusing complete v1 JSON unchanged under
a new domain is rejected: large integers, Unicode, escaped text and complete
record ordering are not guaranteed by small ASCII fixtures. Field-safe selector
helpers containing only already validated strings may be reused. Revisions,
counts and extension integers follow ADR-0013 full-range numeric forms. Use the
existing v2 pure support helpers without manifest-to-engine/protocol edges.

Canonical sets: the four entity arrays, owners, requirement records, facets,
fragment-origin records, transition edges and applicability membership values.
Order by complete projected member JCS bytes and reject exact projected duplicates.
Ordered sequences: a journey's declared surfaces (stored as Vec in the domain),
applicability all/any child lists and extension arrays. Object maps use JCS keys.
Retain the existing absence and NFC rules. Do not recursively sort all arrays.

### S019 decisions: policy/profile projection and crate ownership

Choose a narrow, validated **domain projection input carrier**, with canonical
serialization in eqm_manifest and assembly in eqm_application. Do not move the
engine's selection algorithm or introduce serialization/I/O dependencies into it.
Do not move/rename existing SelectedProfile types merely to implement this API.

Add PolicySelectionInputV2 and SelectedProfileInputV2 in eqm_domain (lifetimes or
owned equivalents are an internal choice). Private fields retain the exact
Policy and selected Profile definitions plus fully resolved dimension values.
Fallible construction requires exactly the policy's profile-ID set, no duplicate
profile IDs (including different revisions), every declared dimension exactly
once, no undeclared dimensions and only declared finite known values. Dimension
state is explicit known(SymbolicValueId) or unknown; absence does not become
unknown and constructors do not apply development defaults. Accept Vec inputs
where necessary to reject duplicates before map collection. These carriers prove
structural selection validity, not protected authority or trusted execution.

The v2 root has exactly `policy` and `profiles`. `policy` contains the complete
selected policy definition with v2 forms: id/revision/title/description/owners,
profile IDs, required targets, rule selectors/constraints, waiver policy and
semantic extensions. `profiles` is a canonical set of records with exactly
`definition` and `values`. `definition` contains each exact selected Profile's
complete v2 definition: id/revision/title/description/owners, dimension declarations
and finite values, defaults and semantic extensions. This binds same-ID/revision
semantic edits; IDs/revisions alone are insufficient. Unselected definitions do
not enter this projection.

`values` is a canonical set with one record per declared dimension:
`{"dimension":"locale","state":"known","value":"en"}` or
`{"dimension":"locale","state":"unknown"}`. Unknown has no value member;
never encode null, omit a declared dimension, or equate unknown to a default.
This is the exact projection spelling, not a new transport protocol. Existing
engine Option values map explicitly to these states after engine selection.

Sets: profiles, selected dimension records, profile dimension definitions and
finite values, owners, policy profile/target references, policy rules, all
populated rule-selector axes, required facets and waiver controls. Ordered
extension arrays retain order; defaults remain object maps. Every numeric field,
including policy maximum_age/minimum_count and waiver limits, uses approved v2
forms. Preserve all selector axes and distinction between an absent selector
axis and a populated axis. Exclude only display extensions and parse provenance.
CanonicalPolicySelectionV2 owns JCS bytes and PolicySelectionDigest. Mode, authority
origin, clock, waivers as records, unrelated graph extensions and workspace
summary are separate context axes, not injected into this projection. No trust
claim follows from obtaining its digest.

The application entry point takes one PreparedSession plus the existing mode and
explicit/development-default PolicyProfileRequest inputs. It invokes the existing
selection algorithm against that session's immutable graph, resolves each exact
selected profile definition from the same graph, builds the validated domain
carrier and calls manifest canonicalization. Expose a typed immutable provenance
bundle with contract and selected-policy identities/bytes. Missing exact definitions
or invalid selection return typed errors retaining causes; never fallback to a
workspace digest, reselect latest, reload files, inspect the clock or execute tools.
Keep existing engine mode/default/trusted-origin checks. Do not accept an unrelated
preselected policy/profile graph through a new unchecked public adapter. The
existing workspace_digest remains aggregate context. V1 CLI/runner records are
not silently switched to v2 identities; later owning v2 construction steps consume
this additive application entry point.

### S019 independently green implementation slices

| Checkpoint | Scope | Commit |
| --- | --- | --- |
| S019-A | Domain projection carriers, local invariant/error tests; no engine algorithm change or schema publication. | `feat: validate policy selection provenance inputs` |
| S019-B | Complete separate v2 contract/entity projection, ContractDigest and independent byte/hash/isolation controls. | `feat: project exact v2 contract authority` |
| S019-C | Complete selected-policy/profile projection, typed identity and same-session application construction; integration and isolation controls; mark numbered S019 complete only here. | `fix: separate contract and policy provenance` |

All three slices retain S019's original R19/AC019 mapping and declared gates.
For each run fmt, locked workspace all-target check, owning domain/manifest/engine/
application tests, clippy -D warnings, standard check, schema parity and final
structure/diff checks. Include relevant doctests and preserve all literal v1 and unaffected
S017/S018 oracles; RF26 alone updates the affected v2 extension goldens. No new dependency or license-policy exception is needed.

Required S019 evidence: independent literal canonical bytes/digests, safe-integer
boundary and endpoint values, unknown versus missing versus known/default values,
exact revision failures, duplicate and undeclared profiles/dimensions/values,
contract-only mutation versus selected-policy-only mutation, selected-profile
same-ID/revision definition mutation, unselected policy/profile stability, graph
semantic-extension inclusion and display-only exclusion, real manifest formatting
stability, canonical set permutations, ordered journey-surface/expression changes,
and same-session application integration retaining mode/authority failures.
Do not claim byte equivalence by rebuilding the same object without mutation.

### Continuation decisions and limits

This settles S019's composition, representations, crate placement and input types.
Its earlier decision-required block is removed by Codex, not reclassified as a
session interruption. Ordinary function naming, helper extraction, borrowing and
test layout remain implementation choices. Existing additive v2 representation
rules apply to successors; small currently observed numbers do not authorize
rehashing v1 encodings. New material semantic compositions still return to Codex;
Pi may not invent authority, waive gates or change scope to avoid that boundary.

After objective repair and S019 gates, continue every ordered unblocked successor
toward S211; S143 is intermediate. Preserve Q01–Q15 at their first dependent step,
use compact per-step context and receipts, and distinguish genuine context limits
from missing decisions or external facts. No publication, deployment, signing,
live enforcement or optional-native activation is authorized. Human release
testing is not the next action; component hashes and local fixtures do not prove
end-to-end product conformance or release authority.

## DEV-005 — S016 review closure and binding projection decision

**Date/author:** 2026-09-28, Codex supervisor under owner delegation.
**Reviewed source:** `68378317044d49a121caff07e783573abdf3cd56`.
**Disposition:** S016-P1 (`ed06989`) is accepted; RF19 is closed. S016's
implementation commit is retained, but the numbered step is reopened for RF20–RF22.
S001–S015 remain complete. The next checkpoint is **S016-R1**, then S016-R2,
then dependency-unblocked S017–S211. This amendment supersedes older cursors and
S016 completion claims, not the retained representation or authority contracts.
See [the source and evidence review](reports/CODEX-6837831.md).

### Findings and independently green repairs

| Checkpoint | Required work | Verification and commit |
| --- | --- | --- |
| S016-R1 | Close RF20/RF21: shared structural validation of authored v2 fields using the field/type mapping below; call it from every existing owning `from_json` path, including embedded inventories. Retain S015 bounds, versions, vocabulary and temporal checks. Add valid controls and one-field negative mutations for each mapped field family. | S016's complete declared commands, schema generation/check if affected, actual package structure validation and diff check. Commit `fix: validate authored v2 metadata at ingress`. |
| S016-R2 | Close RF22: execute the missing set/logical-key/sequence/text controls at real v2 roots, add independent literal runtime-facts canonical bytes/digest, and prove raw-byte preservation separately. Reconcile reports, exact SHAs, progress, cycle counts and current status. | S016's complete declared commands plus the evidence discipline below. Commit `test: complete v2 collection boundary regressions`. |

Retain both existing commits; no amend, reset, renumbering, new product subsystem,
weaker assertion, or successful receipt for a failing command. R1 carries its own
behavioral regressions; R2 closes the remaining matrix rather than postponing all
R1 tests. Each checkpoint is independently green. After R2 passes, Pi records
implementation completion with supervisor acceptance pending and continues S017;
there is no additional user-approval gate. S016 is not complete merely because
CanonicalSet or an isolated NFC predicate passes tests.

### Authored field/type mapping (RF20/RF21)

Reuse the existing domain syntax/bounds, adding no v1 reader changes. NFC alone
is insufficient: `café` is valid SelectorText but is not a valid UnitId. Tests
must use valid typed IDs; demonstrate composed Unicode with a text field whose
existing grammar permits it. Validation rejects rather than normalizes.

| v2 field family | Required structural contract |
| --- | --- |
| Evidence target/unit/requirements/facets | TargetId, UnitId, FullRequirementId, Facet respectively, including existing vocabulary/grammar. |
| Shared subject scope | TargetId for target and each target-set member; ProviderId for provider. Retain existing nonempty scope rules where the corresponding domain constructor requires them. |
| Shared subject metadata | RepositoryIdentity, SourceCommit and digest syntax; optional build_id uses SelectorText. Share existing runtime-facts checks across the other owning roots without inventing relationship/trust acceptance. |
| Profile values in evidence and runtime facts | ProfileId, positive Revision, DimensionId, SymbolicValueId; retain unique (profile, dimension) and consistent per-profile revision. |
| Manual review | OwnerRef reviewer and existing outcome vocabulary; do not treat reviewer as an arbitrary NFC string. |
| Attachments | SelectorText name/media_type and existing digest/size forms; preserve name uniqueness. Attachment content is opaque bytes, not either metadata string. |
| Inventory, adapter request and response | AdapterId, TargetId where present, the shared subject checks where present, and shared embedded inventory validation. |
| Inventory entries / fact symbolic metadata | SelectorText kind/key/attribute names; SymbolicValueId for Symbol values; retain SurfaceId/DimensionId/SelectorText rules for runtime-fact keys and existing restrictions on typed Text values. Source path uses its existing RepoPath contract. |
| Selector helper | Existing EvidenceSelector field types (SelectorText, HttpMethod, ReleaseChannel as applicable), not NFC-only checks. No current v2 root owns this helper: mark its consumer integration pending until the first owning v2 selector ingress exists; wire validation there. Do not invent a schema family to manufacture a consumer. |
| Diagnostic unit/target references | Existing UnitId/TargetId syntax when present; diagnostic prose and source-export content do not acquire blanket authored-text NFC rules. |

Do not apply blanket NFC checks to observed text, diagnostic messages, source
exports or attachment bytes. Existing domain fields explicitly modeled as
SelectorText retain their own bounded/NFC rules; they do not redefine arbitrary
observed bytes as authored metadata. Opaque request identifiers, filesystem roots
and location URIs retain their existing contracts, not invented identifier types.
Representation validation remains distinct from S025/S059/S061 relationship,
trust or signature acceptance. A v2 record passing these checks is non-authorizing.

### Regression closure and evidence discipline (RF22/RF23)

Exercise subject target sets, requirements, facets, attachments, profile values,
runtime facts and inventory entries through their actual owning `from_json`
methods. For each applicable family show valid permutations produce identical
canonical bytes and exact duplicates reject. Show differing records with the
same logical key reject for attachment names, profile/dimension, facts and
inventory; separately test conflicting revisions for one profile. Cover embedded
adapter-response inventory as well as standalone inventory. Mutants retain all
unrelated fields and recompute a required digest so an unrelated error cannot
make a negative test pass.

A deserialize-only execution payload with attempt numbers `[2,1,2]` does not
prove valid evidence ingestion. Test valid numbered attempts at EvidenceResult
root; reordered/duplicated indices reject under the existing index rule. Show
order/multiplicity preservation for valid diagnostic and related-location lists,
including different canonical outputs after reordering. Do not require invalid
attempt numbering to pass merely to demonstrate sequence multiplicity.

Keep the escaped-string/object-member ordering traps. Add a reviewed literal
canonical runtime-facts byte string and fixed expected digest; the expected value
must not be produced by a test copy of the production sorting algorithm. Retain
permutation controls for both facts and profile_values. Exercise strict ingestion
on an actual raw JSON input with formatting/escaping and verify the input bytes
and raw identity remain exact while its semantic view canonicalizes separately.
Use RawObjectDigest for literal decomposed UTF-8 and non-UTF-8 opaque vectors;
compare fixed expected bytes/domain hashes. No storage/attachment subsystem is
required now. A frozen-vector byte-literal decoder must reject malformed or
truncated escapes rather than silently discard characters. Passing arbitrary
non-UTF-8 through a JSON parser is not an opaque-byte preservation requirement.

RF23 is a historical receipt discrepancy: the saved S016 package-structure log
says FAIL while the receipt says exit 0; current independent validation passes.
Codex has corrected the current report/status and recorded the distinction.
Retain original logs. For subsequent checkpoints record each command's real exit
status and unique log name, run structure validation after the final ledger/doc
edit and before committing, and do not infer a command's result from the last
command in a shell sequence. STATUS.md, progress.json and checkpoint receipts
must agree. A later green run is new evidence, not retroactive proof of the
historical receipt. Do not claim that every historical committed tree was green.

### S017 decision: separate complete v2 binding projection

The existing `eqm_manifest::canonical::binding` is a useful field inventory,
not a v2 wire implementation. Hashing its unchanged v1 JSON under BindingDigest
would retain numeric and collection encodings inconsistent with ADR-0013.
Implement a separately named v2 binding projection in eqm_manifest, with immutable
canonical bytes and the existing BindingDigest domain. Keep the v1 binding,
fragment and graph APIs/bytes/hashes unchanged. No new digest domain or dependency
from the manifest layer to protocol/transport is approved or needed.

Include all existing binding semantics: id, revision, target, unit, owners,
artifacts (including selectors), exposures (surface/state/applicability/route),
evidence specifications (selectors, requirements, facets, runner references,
counts/freshness) and semantic extensions at every existing level. Apply approved
v2 integer wire rules to revisions/counts/freshness and extension integers.
Apply full canonical-member byte ordering to owners, artifacts, exposures,
evidence specifications, requirements, facets and applicability membership
values. The remaining ordered expression/extension arrays preserve order; do
not recursively sort every array. Reject exact duplicate set members rather than
silently erasing them. Existing typed construction retains logical uniqueness.

Exclude only the already defined `dev.equivalencematrix.display` extension
namespace wherever the existing projection excludes it. Comments, whitespace,
source spans and the manifest file's own location are parse provenance, absent
from Binding. An artifact's declared path is semantics and remains included.
Do not invent a Binding.description/comment field or exclude other extension
namespaces. Test real display-extension changes and comment/formatting changes,
plus same-ID selector/path/constraint/semantic-extension mutations, set
permutations and large integer boundaries with literal byte/digest expectations.

S017 delivers and tests this component projection; the later owning v2
construction/relationship steps integrate it. Do not silently replace v1
transport/runner identities or claim end-to-end evidence binding now. S018 uses
the same decided representation rules for its separate specification identity.
The complete cross-domain corpus remains S027. This resolves the handback's
scope decision; ordinary function/module layout remains Pi's implementation choice.

### Continuation and authority

Proceed in the sole governing sequence through all dependency-unblocked work to
S211; S143 is intermediate. Codex owns new scope, representations, sequencing,
license exceptions and acceptance decisions. Pi may choose internal mechanics
within this contract and maintain factual reports/progress; it may not change
this scope, self-waive a gate or invent missing external facts. Newly evidenced
contract decisions return to Codex. Preserve Q01–Q15 at their first dependent
steps, keep one active step, checkpoint before context exhaustion, and use
available compaction. A real session limit is INTERRUPTED, not task completion.
No publication, deployment, signing, live enforcement or optional-native
activation is authorized. Human release testing is not the next dependency.

## DEV-004 — Accept S015 and resume canonical collection work

**Date/author:** 2026-09-28, Codex supervisor under owner delegation.
**Reviewed source HEAD:** `121622ff61a18efb0afd1dc9a13264fc2a97f259`.
**Disposition:** S015 and R1–R9 are accepted for their allocated implementation
scope. RF01–RF18 are closed for that scope; later relationship/trust/evaluator
acceptance remains with its original owning steps. S001–S015 are complete.
The [review](reports/CODEX-121622f.md) records independent verification and the
remaining dependency-policy finding RF19. No release readiness is implied.

DEV-002/DEV-003 representation, authority and continuation decisions remain
binding except their obsolete active cursors and pending S015 acceptance.
Preserve all 211 numbered step IDs and the sole governing sequence.

### Dependency decision and prerequisite

Codex approves the exact-pinned dev-only `jsonschema` 0.48.5 dependency with
its disabled default retrieval features and the documented Zlib license
allowance for the reviewed dependency graph. Local manifests/license text,
resolved features, live advisory audit and license/source checks were reviewed.
This does not authorize future license-policy exceptions by Pi. The additional
locked packages are test tooling, not an approved new production dependency.

**RF19:** `cargo deny check` fails because eqm_engine's eqm_test_support path
dev-dependency has no version. This originated in S005 and predates R6–R9;
fix it now rather than waive it or wait for the release cycle. The actual
command surface has no `cargo xtask supply-chain` subcommand. Use `cargo audit
--deny warnings` and `cargo deny check`; `cargo xtask verify` owns the combined
extended release lane. Do not add a new command solely to fit an inaccurate
handback command name. Advisory availability was demonstrated in this review.

| Checkpoint | Scope | Definition of green |
| --- | --- | --- |
| S016-P1 | Repair RF19 as a prerequisite to S016: add `version = "=0.1.0"` to the existing eqm_engine path dev-dependency on eqm_test_support. Preserve package identity, path and dependency direction. | Locked workspace check, full `cargo deny check`, `cargo audit --deny warnings`, standard `cargo xtask check` and `git diff --check` pass. No wildcard/duplicate/advisory threshold is weakened. Record any future live-DB outage distinctly; do not convert it into a successful audit. |

One small green commit for S016-P1, using `fix: pin the engine test support
dependency`. No application source changes are needed in this prerequisite.
Record it under S016 without marking the numbered S016 complete. Then execute
S016 itself and every dependency-unblocked successor through S211. An existing
allowed duplicate-version warning is not a waiver or a new failure.

### S016 decisions resolved before delegation

The existing S016 is enforcement work, not merely adding more catalog strings.
Its already approved set/sequence/text rules require real v2 consumers and
executable controls. Preserve v1 schemas, readers, canonical bytes and hashes.
Do not change the existing v1 graph/fragment projection in place. Introduce
reusable v2 canonical collection/text boundaries now; later component-projection
steps consume them without silently reinterpreting v1.

For the five allocated v2 families, use this classification:

| Collection | Semantics |
| --- | --- |
| Subject target sets; evidence requirements, facets and attachments | Unordered sets. Reject duplicate values before any BTreeSet/map collection can erase them; retain existing attachment-name uniqueness. |
| Evidence/runtime profile_values; runtime facts; inventory entries | Unordered semantic records. Canonicalize by each complete member's canonical JSON UTF-8 bytes, preserving existing logical-key uniqueness independently of ordering. Profile/dimension duplicates or conflicting revisions within one profile, duplicate fact (surface, dimension, key), and duplicate inventory (kind, key) do not become multiple valid records. |
| Execution attempts, diagnostic lists and related-location lists | Ordered sequences. Preserve input order and multiplicity; retain existing attempt-index validity checks. Do not sort/deduplicate them through a generic collection helper. |
| Object maps, including inventory attributes | Strict duplicate-member ingestion plus existing JCS object-key rules, not an array-set sorting algorithm. |

Accept permutations of a valid semantic set and emit the same canonical
representation; reject duplicate members rather than silently collapsing them.
Sort by full canonical member bytes, not Rust derived Ord, field insertion order,
a selected key tuple or unescaped string ordering. Separate a semantic record's
uniqueness key from its canonical ordering key. Keep original received bytes for
raw identity/signature use; canonicalization produces a distinct semantic view.
Runtime-facts digest validation must use the approved canonical semantic set
projection while preserving the original bytes and all S015 numeric/time/bound
checks. Do not redefine digest domains or introduce later trust authority here.

Authored metadata follows ADR-0013's NFC rejection rule. Reuse existing typed
validators and apply the rule to v2 equivalents of authored identifiers, selector
metadata, profile/symbolic values and reviewer metadata; preserve their existing
syntax constraints. This does not authorize blanket NFC validation of observed
payload text, source exports, diagnostic text or attachment bytes. Keep opaque
payload bytes exact, including decomposed UTF-8 and arbitrary non-UTF-8 bytes.
Where existing types impose additional restrictions, preserve them rather than
broadening a v1 reader. Document the field classification and test the actual
v2 boundaries, not only a standalone demonstration helper.

Resolve the frozen vector notation explicitly: authored-text vectors carry JSON
string input and canonical JSON output. In RV-TEXT-2, escaped `caf\u00e9` decodes
to accepted NFC text and canonical output contains literal UTF-8 `café` between
JSON quotes. Pi is authorized to correct that expected output and its explanatory
note, retaining the vector ID and recording the change. This changes the
representation of the same accepted text, not Unicode normalization. Opaque-byte
vectors need a distinct execution discipline and literal byte/hash expectations;
they must not be passed through the authored JSON-string validator. Add decomposed
and non-UTF-8 controls. JCS string serialization and original bytes are different
representations; see [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785.html#section-3.2.2.2).

S016 must execute every set/sequence/authored-text/opaque vector by explicit
category/discipline, compare exact outputs, test permutations and duplicates at
real nested v2 ingress, and prove order sensitivity for sequences. Include cases
where escaped characters and object field ordering distinguish canonical-byte
ordering from incidental Rust order. Regenerate/check affected v2 schemas and
keep the 22 v1 documents byte-identical. No new placeholder schema family is
required. S027 retains complete cross-domain hash/provenance corpus closure.

### Continuation and evidence

The next session begins S016-P1, then S016–S211. S143 remains intermediate.
Codex owns changed scope/representation/sequence/license/acceptance decisions;
Pi may choose ordinary internal mechanics within these settled rules. Keep one
active step, small context working sets, available session compaction, and a
verified report/log/commit checkpoint after each slice. Do not stop voluntarily
at the end of S016 or P02 when continuation is possible. An actual context limit
is INTERRUPTED; an unavailable required external fact is a dependency blocker.
Preserve Q01–Q15 gates at their first dependent step and do not invent product,
cohort, policy, credential or host facts. No deployment/publication/signing or
optional-native activation is authorized.

Model reporting must distinguish configured defaults from observed runtime
selection. Record the actual session identity if exposed; otherwise say runtime
identity is unverified. Do not silently switch models. No need to expose unrelated
agent state or credentials to establish this fact.

## DEV-003 — Close version ingress and prove S015 conformance

**Date/author:** 2026-09-27, Codex supervisor under owner delegation.
**Reviewed source:** `7d44724abba2fa90baf806563da0a692c5bb5857`.
**Disposition:** S015 remains in progress, changes required. R1–R5 commits are
retained; the next checkpoint is S015-R6. This amendment takes precedence over
DEV-002 only where it clarifies conformance evidence and continuation status.
All other DEV-002 decisions remain binding. Preserve the 211 numbered steps.

The independent [review](reports/CODEX-7d44724.md) records RF11–RF18.
A green standard gate did not detect v2-labeled workspace/lock input being read
with v1 semantics, missing nested inventory validation, repeated smaller byte
bounds, schema/runtime differences, or tests which bypass their claimed boundary.
These are unfinished S015 obligations, not new product scope.

### Decisions settled by Codex

1. **Identity versus supported reader.** `SchemaUri` may represent a known
   kind/version pair without promising a generated schema or a supported reader.
   Every ingress must enforce its exact supported version and kind. Existing
   workspace, lock and authored-manifest readers accept v1 only until their
   owning integration step. Keep CURRENT at v1. Audit every SchemaUri consumer;
   a kind-only check is insufficient. No placeholder v2 manifests or fallback.
2. **Nested structural validation.** A v2 adapter response must validate the
   nested inventory's v2 marker and all structural checks already required by
   standalone InventoryV2Dto ingestion, including digest syntax and bounds.
   Also validate the response's own digest syntax. Reuse validation without a
   weaker deserialize-only route. This does not confer trust or pull the later
   relationship/verification/evaluator steps into S015.
3. **Bounds are end-to-end.** Keep the existing per-family byte limits at every
   parse, including canonical-digest preparation. v1 and v2 runtime facts retain
   the declared 16 MiB envelope and 100000-fact ceiling. Preserve v1's existing
   subject syntax checks in v2 runtime facts as well: repository identity,
   repository/configuration digests, source commit, optional build ID and artifact
   digest. These are existing structural checks, not later trust semantics.
   Do not globally raise
   the default strict parser's 8 MiB limit. Test actual valid records, including
   computed digests, padded beyond 8 MiB, at 16 MiB and one byte over; test fact
   count at/above its cap. Preserve the completed release bound correction.
4. **Schema assertions.** All five v2 schemas must constrain each instance's
   `schema` member to its exact family/version URI, including embedded inventory.
   `$id` alone is not this constraint. The UTC pattern must reject zero and
   redundant fractional groups (including `.000Z`, `.123000Z`, `.123456000Z`)
   while accepting canonical 0/3/6/9 groups. Use date-time format assertions in
   the conformance validator for calendar validity, in addition to the v2
   canonical pattern. Raw parser/calendar validation remains mandatory.
5. **Three distinct validation layers.** JSON Schema validates JSON values,
   not original number-token spellings: 1.0 and 1e3 are integral values. Raw v2
   parsing must still reject those tokens, negative zero and duplicate members.
   Keep raw-byte/token, schema-value and semantic-validation test expectations
   separate. This corrects DEV-002's overbroad schema/parser agreement wording;
   it does not relax the v2 wire contract. Cross-field digest/trust semantics
   remain runtime responsibilities and must not be attributed to JSON Schema.
   See [Draft 2020-12 validation](https://json-schema.org/draft/2020-12/json-schema-validation#section-6.1.1).
6. **Real schema validation is authorized.** Add an exact-pinned, toolchain-
   compatible Rust `jsonschema` dev dependency, with Draft 2020-12 and explicit
   format assertion support. Select the exact compatible release as an ordinary
   implementation detail, record its version/features/license/lock changes and
   use only local schema resolution with network retrieval disabled. No runtime
   dependency is authorized by this decision. Replace the bespoke pseudo-validator;
   do not reinterpret standard integer semantics to force test agreement.
7. **Evidence must exercise the claimed input and boundary.** Absence-vector
   tests must consume each literal input and compare its declared output; never
   synthesize the test input from expected success/failure. Validate exact
   category/discipline mappings. Test provider over-return through read_evidence
   with a fake reader (exact limit, one over, missing reader, propagated error,
   no runner execution), not only the comparison helper. A reconcile control
   must successfully load a valid locked local inventory; its duplicate-member
   mutant must fail at the same ingress. Another unrelated error is not success.
8. **Continuation is preauthorized, acceptance is separate.** Complete R6–R9
   in order, then continue S016–S211 while dependency-unblocked. Once the
   objective repair checks pass, Pi may mark S015 implementation-complete and
   proceed, recording supervisor acceptance as pending review of the new HEAD.
   Do not stop solely to request that already-authorized transition. Codex
   reviews at each session handback; Pi cannot approve itself, change these
   decisions, waive gates or skip blocked predecessors. S143 is intermediate.
   A new material scope/contract conflict needs a concrete Codex decision;
   routine mechanics and pinned dependency selection within decision 6 do not.

### Remaining corrective commits under S015

Each row is one independently green commit, with report and ledger updates.
Use the existing S015 report with real stable anchors and exact commit receipts.

| Checkpoint | Scope | Required evidence and dominant lane |
| --- | --- | --- |
| S015-R6 | RF11/RF12: enforce reader version/kind and nested inventory structural validation. | Successful v1 workspace/lock controls; changing only the URI to v2/v3/wrong kind rejects through actual preparation/CLI. Valid records for all five v2 readers pass; root and nested version mutations reject; nested invalid digest rejects both standalone and embedded. Manifest/protocol/CLI tests, full standard gate. |
| S015-R7 | RF13/RF18: repair runtime-facts bounds and retain fact-count/subject structural validation. | Actual digest-valid records pass beyond 8 MiB and at 16 MiB; one byte over fails with size error; 100000 facts pass where otherwise valid, 100001 reject. Valid subjects pass; one-field malformed repository/commit/digest/build-ID mutants reject after recomputing the enclosing facts digest. Audit all family parse bounds; protocol tests and full standard gate. |
| S015-R8 | RF14/RF15: correct generated schema constraints and use standards-conforming validation. | Regenerate five v2 schemas atomically; 22 v1 schemas byte-identical. Full valid fixtures and single-field mutants for all families run against committed/generated schemas, including nested markers/null, integer endpoints/string ranges, canonical times and invalid calendars. Separate lexical-only and cross-field expectations. Protocol/domain schema tests, schema parity and full standard gate. |
| S015-R9 | RF16/RF17: replace tautological controls and reconcile evidence. | Literal absence inputs/outputs execute; exact discipline mapping; successful reconcile control and duplicate mutant; fake-reader integration succeeds/fails at real boundary. Run application doctests and all S015 tests; correct report claims, anchors, cursor and per-checkpoint receipts. Package structure and full standard gate. |

Full standard gate is `cargo xtask check` plus `git diff --check`. Each checkpoint
also runs its targeted tests; regenerate/check schemas whenever their source
changes. R9 runs S015's complete declared command set. Retain raw command logs
outside the checkout and report their paths externally. Do not rewrite original
package delivery hashes or amend reviewed commits. Do not write implementation
code through the supervisor: Pi remains the implementation agent.

S016 retains canonical sets/NFC/opaque-byte work; S025/S059/S061 retain relationship
acceptance, S027 hash-corpus closure, S069 freshness arithmetic, and later owning
steps the rest of the v2 consumer migration. External product/host/policy gates
remain effective at their first dependent step. Local tests do not establish
release readiness; no publication, deployment or optional-native activation is
part of this amendment.

## DEV-002 — Supervisor-owned S015 completion and continuation

**Date/author:** 2026-09-26, Codex supervisor. **Status:** owner-delegated
engineering decision; implementation pending. **Reviewed HEAD:**
`3da0b50aeb7cf7f8b23f8f5ebd47f043892ed00b`. **Affected steps:** S015 plus
corrective regression contributions to S006/S011/S012/S014; S016 and all later
steps retain their IDs and order. **Contracts:** R18/R19/R21/R52/R64 and existing
step-linked acceptance/regression clauses. No new product subsystem is added.

**Evidence:** the supervisor report `implementation/reports/CODEX-3da0b50.md`
records RF01–RF10: v1 acceptance changed despite frozen schemas, v2 DTO/schema
integration and absence enforcement are missing, integer endpoints/canonical
forms disagree, release ingress repeats the smaller parser bound, and the
bounded read helper trusts over-returning providers. S015 already requires
representation enforcement and atomic schema updates. Treating those as
unallocated future work is unsafe. Null/absence belongs to S015, not S016.

**Authority and responsibility:** the owner delegates decisions needed between
coding periods to Codex. The supervisor changes scope, sequence and normative
contracts; the implementation agent performs code generation, verification and
evidence/status updates. It may choose ordinary internal mechanics that obey
this contract, but may not change representations, defer required work, invent
an additional step, reorder dependencies, waive gates or claim acceptance to
get past a failure. A newly evidenced conflicting obligation returns to the
supervisor with a concrete decision request. Approved decisions below need no
reconfirmation.

### Binding engineering decisions

1. Preserve the pre-S015 v1 accepted integer ranges, existing DTO behavior,
   schema bytes and signed originals. Revert only S015's new v1 numeric-range
   restrictions. Retain bounded duplicate-rejecting ingestion as the approved
   security correction. Do not add decimal strings to v1 or use v1 URIs for v2.
2. v2 unsigned quantities use `u64` (0 through 18446744073709551615); signed
   facts use `i64` (-9223372036854775808 through 9223372036854775807).
   JSON integer tokens are used exactly within [-9007199254740991,
   9007199254740991], intersected with the semantic type's range. Only values
   outside that interval use decimal strings. Reject small-value strings,
   leading zeros, plus signs, negative zero, fractional/exponent tokens and
   overflow. Every supported value must serialize and deserialize identically.
   Positive counts further require at least one. This policy is not a claim
   that every integer beyond 2^53-1 is individually unrepresentable as a float;
   it fixes an interoperable safe interval.
3. v2 time uses exact nanosecond UTC, a literal Z, four-digit years 0001–9999,
   and the minimal AutoSi group of 0/3/6/9 fractional digits. Reject leap seconds,
   offsets, missing Z, out-of-range years, redundant fractions and unsupported
   precision. A v2 validated wrapper must not silently change the retained v1
   parser. Absence means omitted members; explicit null rejects recursively
   for optional v2 members. Preserve original bytes before interpretation.
4. Establish explicit version-qualified schema/DTO ownership. v1 APIs remain
   unchanged. v2 protocol DTOs live in `eqm_protocol::v2`; schema identity must
   include the explicit version, never a fallback or a changed v1 CURRENT alias.
   `WireUint`/`WireInt` are v2 primitives, not validation adapters for v1.
5. The first concrete v2 schema inventory, owned by S015, is five protocol
   families: `evidence-result`, `runtime-facts`, `inventory`, `adapter-request`,
   `adapter-response`, each at `schemas/v2/protocol/<name>.schema.json` and the
   matching approved GitHub raw URI. Include their nested DTOs and full existing
   fields, with v2 numeric/time/absence validation. Do not invent later campaign,
   relationship, storage or verification semantics in this checkpoint. Raw
   claims cannot become validated authority merely by parsing a DTO.
6. `cargo xtask schemas generate` and `schemas check` own both version inventories
   and deterministic parity. The current 22 v1 schemas remain unchanged.
   Check unique IDs, explicit version/kind dispatch, exact file inventories,
   nested schema references, positive/negative runtime/schema conformance and
   unsupported-version rejection. Do not hardcode a new total that future
   versioned additions must bypass. Schema constraints must enforce numeric
   encoding and representable string ranges; classify cross-field semantic
   validations separately rather than claiming the schema proves them.
7. Do not clone all remaining schemas or add placeholder v2 manifests now.
   The existing step that first changes an authored or protocol shape owns its
   real v2 DTO, generator registration, fixtures and explicit consumer update
   in the same commit. This is a standing requirement for S016–S211, including
   P03/P04 authority, P09 decisions, P10/P11 reports and P12 release records.
   No new wire meaning may ship under v1. Existing CLI/MCP v1 operations remain
   stable until their owning integration steps deliberately select v2. No
   implicit conversion reader or indefinite dual-version support is added.
8. Keep the established 16 MiB release-record boundary at every parse. A bounded
   application read rejects an oversized provider result with `LimitExceeded`;
   it never silently truncates. Future concrete providers must enforce bounds
   during reading/allocation, since checking an already returned Vec is not an
   allocation bound. A read-only trait name cannot establish absence of effects:
   actual implementations and integrated call paths require later audit/spies.

### Corrective checkpoint order before S016

These are supervisor-approved independently green corrections under the
original S015 traceability anchor, not additional renumbered mandatory steps.
Complete each in its own commit and record its SHA/report. Only one is active.

| Checkpoint | Scope | Definition of green and dominant verification |
| --- | --- | --- |
| S015-R1 | Restore v1 acceptance; correct v2 integer endpoints, canonical input and generated primitive constraints (RF01/RF03/RF04/RF05). | Existing v1 accepted endpoint fixtures still pass with identical schemas/bytes; all i64/u64 endpoints, safe-boundary neighbors, negative zero, fractional/exponent, small-string and overflow controls agree with v2 parser/serializer/primitive schema. Protocol/domain/runner tests plus full standard gate. |
| S015-R2 | Implement explicit version identity and multi-inventory generation/parity plumbing (RF02). | Existing v1 generation remains byte-identical; explicit v2 identities parse/render, unsupported versions reject, known inventories remain exact. Do not publish invented empty DTO schemas. Generator/domain/xtask tests plus full standard gate. |
| S015-R3 | Add the five actual v2 protocol families with canonical numeric/time/absence behavior and generated schemas atomically (RF02/RF05/RF06). | Real valid records and one-field negative mutations agree with their generated schema constraints and runtime validation; missing optional is omitted, explicit null rejects, leap second rejects, endpoints round-trip, and no v1 path/schema changes. Schema generate/check and protocol/domain/vector tests plus full standard gate. |
| S015-R4 | Finish release/adapter bounded ingress and application result-bound/integration regressions (RF07/RF08/RF09; CR01/CR02/CR06). | Valid actual command-ingress controls pass; duplicate inventory attributes fail through a locked-adapter reconcile fixture; release records immediately below/at/above 16 MiB behave as specified; provider over-return fails, exact limit succeeds, missing ports fail; meaningful command time-injection evidence; application/CLI/protocol tests and application doctests plus full standard gate. |
| S015-R5 | Execute the whole applicable representation corpus and reconcile evidence/ledger/schema ownership (RF09/RF10). | Dispatch all numeric/time/absence vectors by declared semantic category/type, fail on unhandled applicable vectors (no ID allow-list or minimum-count proxy), validate raw bytes/unsupported versions and five-family schema parity, run application doctests, all S015 commands and package structure checks, review reports and mark S015 complete only with recorded evidence. |

Full standard gate means `cargo xtask check` and `git diff --check`; run each
checkpoint's meaningful targeted tests in addition. Do not add intentionally
red commits, amend the reviewed history or weaken tests. S016 retains its
set/sequence/authored-NFC/opaque-byte responsibilities; S027 retains full
cross-domain hash/provenance corpus closure; S069 retains evaluator freshness
arithmetic. RA13 relationship acceptance remains pending S025/S059/S061.

### Continued execution and external gates

The approved next run executes S015-R1–R5, then S016–S211 in order for as long
as the work is dependency-unblocked. S143 is an intermediate checkpoint, not a
stop instruction. The final definition of done remains the full MVP, not a
successful partial batch. Preserve one active step and the original predecessor
chain; there is no authorization to skip an externally blocked prerequisite.

Missing live configuration does not block generic fail-closed code, synthetic
cryptographic fixtures or validated configurable policies. It does block a
step's required real-product/host/authority evidence when that evidence is
actually necessary. At that first boundary, finish already authorized work in
the current slice, record the exact unavailable fact and return to the
supervisor. Do not assume similarly named local projects are the approved
product, invent business/cohort/retention values, or promote synthetic checks
to live acceptance. Q01–Q15 remain explicit where evidence is missing.

A context limit is an interruption, not a product blocker. Compact and resume
where supported; otherwise provide a precise clean checkpoint handback. The
supervisor reviews between sessions and owns any changed decision. Production
release/enforcement, publication, credentials and optional-native activation
remain outside this authorization.

**Verification of this amendment:** standard source gate and application
doctest passed at the reviewed source HEAD; document structure/diff validation
is required after plan synchronization. No repair implementation is claimed.

## Commit convention — inferred from the existing repository

Use **`type: imperative subject`**, lowercase type, no mandatory scope. The checked history includes build/chore/docs subjects in this form. Use the exact supplied subject unless repository evidence requires a documented adjustment; an optional body explains rationale, validation and breaking compatibility. Do not substitute a different convention based on personal preference.

## Execution rules

This is the complete mandatory ordered plan. Chapters are navigation/checkpoint groupings, **not permission to merge the steps into a phase-sized commit**. Each Sxxx is one independently reviewable, tested, known-good logical change. Use one step per commit. Do not skip, merge, reorder or broaden steps. Only repository evidence proving a step obsolete or unsafe justifies a documented deviation; record the precise replacement/disposition and tests before proceeding. Missing live product/host/credential evidence is a blocker, not proof a requirement is obsolete.

Perform preflight from VERIFICATION.md before S001. Keep the extracted handoff outside the checkout until S001 narrowly admits the versioned documentation subtree; S002 imports it under docs/eqm_product_v1. Never silently remove repository safety gates or overwrite unrelated work. All statuses begin pending.

Read the listed durable contracts before each step. Files/modules are likely locations based on the reviewed repository; inspect actual layout and relevant nested instructions first. New modules/symbols/DTOs are proposals to implement, not claims they already exist. Exact public fields, wire versions and native tasks are closed spec-first, not invented to satisfy a test. Internal additions remain non-public or fail-closed until their validating integration exists.

Each step includes its regression/tests with the implementation: no committed intentionally failing tests. Use additive internal construction and bounded refactors to preserve compilation while replacing old orchestration. Generate schemas from source and update tests/docs in the same semantic change. Standard Rust checks and `cargo xtask check` must pass before committing; record actual results and self-review, update the ledger, commit, and then report the SHA and next-step safety.

A reproduced unchanged pre-existing/out-of-scope failure may be recorded with lack-of-regression evidence and explicit safe-to-proceed rationale. It never counts as a pass or closes a required security/native/product gate. A new regression blocks advancement. If a step is too broad to stay safe, document a repository-evidenced split in DEVIATIONS.md rather than silently combining/reordering work. Do not weaken tests/thresholds or hide unavailable hosts to proceed.

The supplied commands use known package names; `eqm` is the CLI package. `eqm_application` is usable only from its creation step. Exact dependency/feature/native commands needing discovery are called out separately. Intentional dependency resolution updates Cargo.lock in the owning step, then checks run --locked. A command listed here is a required future action, not a claim it ran during handoff preparation.

Optional native embedding is **not** part of this mandatory sequence. It is explicitly deferred unless Q11 is activated; its own ordered plan is OPTIONAL_NATIVE_COMMIT_SEQUENCE.md. Within the mandatory product-integration chapter, actual product repos/tasks/host authority must be known before real adapter changes or acceptance claims.

After each commit emit STEP_REPORT_TEMPLATE.md fields: files, behavior, exact commands/results, self-review, schema/DB/security impact, baseline exceptions, risks, commit SHA and next-step safety. Never place a commit's own SHA inside itself; report it externally or in the next ledger update.

## Approved execution cycles — revision 2

Owner approval on 2026-09-26 accepts the complete reviewed scope and recommended defaults. [ADR-0012](../decisions/ADR-0012-approved-review-closure.md) records its limits; [DEV-001](DEVIATIONS.md) amends S049 explicitly. This document is the single governing sequence; chapter files and machine ledgers are synchronized views, not competing plans.

Keep exactly one step active. All 211 implementation steps remain pending; next is S001. Preparing this revision does not complete S001-S006. Every cycle below requires all constituent steps and its definition of green. Resolve named gates at the first affected mandatory step; missing external evidence never licenses skipping ahead. Recorded host support is a target obligation until native execution evidence exists.

| Cycle | Steps | Definition of green | Admission inputs |
|---|---|---|---|
| EQM-P01 | S001–S012 | Approved package admitted; baseline reconciled; S006 representation vectors frozen; application extraction and typed ports remain acyclic. | Q03, Q05, Q14 |
| EQM-P02 | S013–S027 | Canonical component/subject/raw/semantic/attempt identities and dirty-source provenance pass mutation vectors. | Q03, Q14 |
| EQM-P03 | S028–S043 | Shared-source dependencies, participant roles and exact multi-subject records validate without forced target symmetry. | Q02, Q06 |
| EQM-P04 | S044–S058 | Complete protected universe, exact revision loading, fragment risk, prepared construction and facet constraints satisfy negative tests. | Q05 |
| EQM-P05 | S059–S076 | Reference evaluator implements every evidence kind, count, retry, campaign, freshness and waiver rule; meaningful properties pass. | Q05 |
| EQM-P06 | S077–S096 | Pinned linked SQLite and immutable objects pass concurrency, crash/reopen, migration/restore, receipt and retention tests. | Q02, Q07, Q10 |
| EQM-P07 | S097–S115 | Authorized concrete plans and all ingress/process paths obey actual host capabilities and bounded lifecycle guarantees. | Q02, Q07, Q08, Q13 |
| EQM-P08 | S116–S131 | Independent protected authority and real test cryptography reject every missing/wrong trust dimension; no production test bypass. | Q04 |
| EQM-P09 | S132–S143 | Frozen manifests, immutable decisions, replay/current-acceptance distinction and coherent bounded queries agree. | Q04, Q07 |
| EQM-P10 | S144–S157 | All CLI commands use shared application decisions; lifecycle/output-schema/format/error/consumer-doctor regressions pass. | Q14 |
| EQM-P11 | S158–S164 | MCP tools/resources retain one authority/snapshot with explicit refresh, permissions, pagination and full-envelope bounds. | Q07, Q14 |
| EQM-P12 | S165–S178 | Complete release decisions, statements, signing and exact authorizations pass positive/negative consumption tests; shadow remains default. | Q04, Q06, Q09 |
| EQM-P13 | S179–S190 | Conservative impact and bounded batching/cache paths match the reference evaluator and measured representative budgets. | Q07 |
| EQM-P14 | S191–S201 | Actual authorized Rust/KMP/provider tasks, approved product scenarios and supported cohorts pass; optional embedding disposition recorded. | Q01, Q02, Q06, Q12 |
| EQM-P15 | S202–S211 | Standard/extended/host/product/crypto/compatibility/distribution/shadow gates have evidence; final report separates code/platform/operational status. | Q02, Q04, Q07, Q09, Q15 |

Verification lane for every cycle: each step’s listed owning tests, full `cargo xtask check`, and `git diff --check`; additionally run every declared host/product/security/extended lane needed for that cycle’s claims. Optional N001–N016 remain not activated.

## Ordered chapters

- [S001–S012: Repository admission and application boundary](sequence/01-repository-admission-and-application-boundary.md) — 12 separate commits.
- [S013–S027: Canonical identities and evidence provenance](sequence/02-canonical-identities-and-evidence-provenance.md) — 15 separate commits.
- [S028–S043: Shared sources target roles and exact subjects](sequence/03-shared-sources-target-roles-and-exact-subjects.md) — 16 separate commits.
- [S044–S058: Complete obligation derivation and canonical selection](sequence/04-complete-obligation-derivation-and-canonical-selection.md) — 15 separate commits.
- [S059–S076: Evidence selection outcomes and the reference evaluator](sequence/05-evidence-selection-outcomes-and-the-reference-evaluator.md) — 18 separate commits.
- [S077–S096: Immutable object store and transactional SQLite index](sequence/06-immutable-object-store-and-transactional-sqlite-index.md) — 20 separate commits.
- [S097–S115: Authorized acquisition and host supervision](sequence/07-authorized-acquisition-and-host-supervision.md) — 19 separate commits.
- [S116–S131: Protected authority and real verification](sequence/08-protected-authority-and-real-verification.md) — 16 separate commits.
- [S132–S143: Frozen inputs decisions and coherent query service](sequence/09-frozen-inputs-decisions-and-coherent-query-service.md) — 12 separate commits.
- [S144–S157: CLI integration and removal of placeholder semantics](sequence/10-cli-integration-and-removal-of-placeholder-semantics.md) — 14 separate commits.
- [S158–S164: Read-only MCP and coherent agent access](sequence/11-read-only-mcp-and-coherent-agent-access.md) — 7 separate commits.
- [S165–S178: Release evaluation statements and exact authorization](sequence/12-release-evaluation-statements-and-exact-authorization.md) — 14 separate commits.
- [S179–S190: Conservative impact analysis and measured optimization](sequence/13-conservative-impact-analysis-and-measured-optimization.md) — 12 separate commits.
- [S191–S201: Rust KMP adapters and product acceptance integration](sequence/14-rust-kmp-adapters-and-product-acceptance-integration.md) — 11 separate commits.
- [S202–S211: Verification distribution and operational sign-off](sequence/15-verification-distribution-and-operational-sign-off.md) — 10 separate commits.

---

<a id="s001"></a>
## S001 — Admit the approved spec subtree

**Predecessor:** PREFLIGHT. **Contracts:** R53, R56, R61. **Acceptance:** AC053, AC056, AC061.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md).

### 1. Step title
Admit the approved spec subtree.

### 2. Purpose
Keep the standard repository gate usable without discarding security checks.

### 3. Exact scope of code changes
Narrowly allow docs/eqm_product_v1 content and replace generic-word handoff rejection with scoped meaningful private-data checks; keep unrelated directory/generated-state restrictions.

### 4. Files/modules likely involved
- `tools/xtask/src/main.rs`
- `tools/xtask tests`
- `docs/README.md`

### 5. Unit tests or integration tests required
Add fixtures admitting the versioned spec subtree and rejecting arbitrary top-level/private/generated material; run public-repository checks.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Approved contract docs can be imported while existing meaningful safety rules still fail on forbidden inputs.

### 8. Commit message
```text
build: admit versioned product specifications
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s002"></a>
## S002 — Import durable contracts and the ordered plan

**Predecessor:** S001. **Contracts:** R56, R57, R61, R62. **Acceptance:** AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [repo/BASELINE.md](../repo/BASELINE.md).

### 1. Step title
Import durable contracts and the ordered plan.

### 2. Purpose
Establish versionable approved intent before implementation.

### 3. Exact scope of code changes
Add the handoff under docs/eqm_product_v1, including required contract files, requirements/acceptance catalogs and initial pending progress; preserve existing public docs and mark code/tests unimplemented.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/`
- `docs/README.md`

### 5. Unit tests or integration tests required
Run the package validator at its imported path and verify internal links/IDs; assert no new generated .eqm state or secrets.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Agents can reconstruct intent and resume from an explicit pending step without chat history.

### 8. Commit message
```text
docs: record the eqm product v1 contract
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s003"></a>
## S003 — Merge concise agent guidance

**Predecessor:** S002. **Contracts:** R50, R53, R56, R62. **Acceptance:** AC050, AC053, AC056, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md).

### 1. Step title
Merge concise agent guidance.

### 2. Purpose
Keep repository-local operating rules aligned with the approved plan.

### 3. Exact scope of code changes
Merge repo/AGENTS.md guidance into existing root instructions; point to durable specs and one-step reporting; clarify approved changes versus gated unsafe/version decisions.

### 4. Files/modules likely involved
- `AGENTS.md`
- `docs/agent-context.md`
- `docs/eqm_product_v1/implementation/`

### 5. Unit tests or integration tests required
Validate documentation references and compare instructions for conflicting execution/trust permissions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Root guidance is short, preserves existing protections, and routes an agent to the next exact step.

### 8. Commit message
```text
docs: anchor agent work to the product specification
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s004"></a>
## S004 — Record baseline findings and decision gates

**Predecessor:** S003. **Contracts:** R14, R23, R50, R53, R57, R62. **Acceptance:** AC014, AC023, AC050, AC053, AC057, AC062.

**Read:** [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Record baseline findings and decision gates.

### 2. Purpose
Prevent static observations being mistaken for tested current defects.

### 3. Exact scope of code changes
Reconcile F01-F20 and RA01-RA21 against the actual HEAD, incorporate the recorded full-source review and baseline checks, and preserve unrun implementation acceptance. Revalidate the approved decisions in ADR-0012 and record only remaining external facts; apply DEV-001 to the planning views before its owning implementation step.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/repo/REVIEW_FINDINGS.md`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`

### 5. Unit tests or integration tests required
Verify every F/RA item has source evidence, requirements, acceptance, owning steps and a precise regression; distinguish static review from runtime reproduction and prior baseline checks from new implementation evidence.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
The implementation baseline and unresolved blockers are evidenced without invented passing tests.

### 8. Commit message
```text
docs: record implementation baseline and open gates
```

**Approved review regression contributions:** RA05.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s005"></a>
## S005 — Add deterministic conformance fixture builders

**Predecessor:** S004. **Contracts:** R14, R26, R50, R53. **Acceptance:** AC014, AC026, AC050, AC053.

**Read:** [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Add deterministic conformance fixture builders.

### 2. Purpose
Supply reusable regression inputs without production trust shortcuts.

### 3. Exact scope of code changes
Extend test support with injected time, exact subject/selection builders and synthetic evidence helpers; keep every authority stand-in test-only.

### 4. Files/modules likely involved
- `crates/eqm_test_support/src/`
- `crates/eqm_engine/tests/support/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Builders reject malformed coordinates and produce repeatable IDs; production dependency/feature inspection shows no synthetic trusted factory.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Later steps can add focused regressions without duplicating fragile fixture construction.

### 8. Commit message
```text
test: add deterministic conformance fixtures
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s006"></a>
## S006 — Freeze semantic and version representation decisions

**Predecessor:** S005. **Contracts:** R14, R16, R21, R23, R50, R52, R58. **Acceptance:** AC014, AC016, AC021, AC023, AC050, AC052, AC058.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Freeze semantic and version representation decisions.

### 2. Purpose
Resolve protocol ambiguity before new wire meanings ship.

### 3. Exact scope of code changes
Materialize ADR-0012 approval: explicit new wire namespace, required conjunction/authorized alternatives, per-kind logical count units, retry/conflict/campaign rules and inherited fragment risk. Complete engineering decisions for the exact namespace/URI transition, numeric ranges, timestamp precision, null/absence, sets/sequences, digest domains and minimal public API names. Audit consumers/releases; stop only for a newly evidenced obligation conflicting with the approved direction or a still-missing required contract.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/decisions/`
- `docs/eqm_product_v1/specs/API_CONTRACTS.md`
- `docs/eqm_product_v1/specs/EVALUATION_SEMANTICS.md`

### 5. Unit tests or integration tests required
Add conformance vectors for approved outcome/count/risk rules and all representation boundaries. Cross-check schema generators, URI parsing and consumer expectations; retain originals and reject unsupported versions explicitly. Record fixture policies as synthetic; do not invent production threshold/skew values.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** ADR-0012 records owner approval; do not re-request the settled direction. S006 closes exact engineering representations and any newly evidenced conflicting consumer obligation. Live product/authority values remain governed by Q01-Q15; DEV-001 defines the S049 amendment.

### 7. Expected result
New wire and policy work has explicit normative representations; unresolved external compatibility blocks exposure.

### 8. Commit message
```text
docs: freeze evaluation and protocol representations
```

**Approved review regression contributions:** RA05.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s007"></a>
## S007 — Create the shared application crate

**Predecessor:** S006. **Contracts:** R04, R06, R50. **Acceptance:** AC004, AC006, AC050.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md).

### 1. Step title
Create the shared application crate.

### 2. Purpose
Introduce the approved effectful boundary without moving semantics.

### 3. Exact scope of code changes
Add eqm_application to workspace with exact local dependencies and inherited lint/license metadata; expose only a documented minimal module boundary and retain existing CLI behavior.

### 4. Files/modules likely involved
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_application/Cargo.toml`
- `crates/eqm_application/src/lib.rs`

### 5. Unit tests or integration tests required
Compile the new crate, verify no CLI/MCP dependency cycle, and test its empty/minimal public boundary without success placeholders.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Workspace remains green and the application layer exists without changing release decisions.

### 8. Commit message
```text
refactor: introduce the shared application layer
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s008"></a>
## S008 — Move prepared workspace ownership into the application layer

**Predecessor:** S007. **Contracts:** R04, R06, R22. **Acceptance:** AC004, AC006, AC022.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Move prepared workspace ownership into the application layer.

### 2. Purpose
Share immutable preparation without CLI coupling.

### 3. Exact scope of code changes
Move or extract PreparedSession/SessionRequest loading ownership into eqm_application; leave MCP adaptation in the transport/CLI layer; update call sites without changing evaluation semantics.

### 4. Files/modules likely involved
- `crates/eqm_application/src/session.rs`
- `crates/eqm_cli/src/session.rs`
- `crates/eqm_cli/src/commands/`
- `crates/eqm_mcp/src/session.rs`

### 5. Unit tests or integration tests required
Existing session fixtures retain graph digest/source maps; test immutable borrowing and no reload/cycle across MCP adaptation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_mcp
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI behavior is preserved while other adapters can use the same prepared workspace.

### 8. Commit message
```text
refactor: share prepared workspace sessions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s009"></a>
## S009 — Preserve manifest error causes

**Predecessor:** S008. **Contracts:** R45, R50. **Acceptance:** AC045, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md).

### 1. Step title
Preserve manifest error causes.

### 2. Purpose
Restore actionable parse/load diagnostics.

### 3. Exact scope of code changes
Replace stage-only load/conversion error erasure with typed contextual variants retaining source path, field and causal error where safe; do not alter schema acceptance.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/loader.rs`
- `crates/eqm_manifest/src/validation.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Invalid TOML, conversion, discovery and lock fixtures retain appropriate path/field information without secret leakage.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Manifest failures remain diagnosable at higher boundaries.

### 8. Commit message
```text
fix: preserve manifest loading diagnostics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s010"></a>
## S010 — Preserve session preparation errors

**Predecessor:** S009. **Contracts:** R45, R50. **Acceptance:** AC045, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md).

### 1. Step title
Preserve session preparation errors.

### 2. Purpose
Keep new shared preparation failures typed and contextual.

### 3. Exact scope of code changes
Map resolve/expand/canonicalization failures into application errors with causes; retain stable command behavior until public error-envelope step.

### 4. Files/modules likely involved
- `crates/eqm_application/src/session.rs`
- `crates/eqm_application/src/error.rs`
- `crates/eqm_cli/src/session.rs`

### 5. Unit tests or integration tests required
Malformed graph/fragment pin and unavailable file fixtures retain causes across application/CLI conversion.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Preparation no longer discards useful errors while production commands remain fail-closed.

### 8. Commit message
```text
fix: retain prepared session error context
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s011"></a>
## S011 — Inject evaluation time consistently

**Predecessor:** S010. **Contracts:** R05, R22, R38. **Acceptance:** AC005, AC022, AC038.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Inject evaluation time consistently.

### 2. Purpose
Remove hidden wall-clock reads from shared semantic preparation.

### 3. Exact scope of code changes
Add explicit host-captured evaluation time to application requests and pass it through existing query/evaluation call sites; keep acquisition timing and monotonic process timers separate.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Repeated requests with the same time are deterministic; boundary-time fixtures prove no internal SystemTime drift in evaluation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evaluation has explicit time without changing process timeout mechanisms.

### 8. Commit message
```text
refactor: inject evaluation clock context
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s012"></a>
## S012 — Define narrow application capability interfaces

**Predecessor:** S011. **Contracts:** R04, R05, R06, R64. **Acceptance:** AC004, AC005, AC006, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Define narrow application capability interfaces.

### 2. Purpose
Separate read/evaluate operations from privileged acquisition.

### 3. Exact scope of code changes
Introduce documented ports for authority/evidence access, verification and execution with explicit requests; no default global runtime or permissive fallback implementations.

### 4. Files/modules likely involved
- `crates/eqm_application/src/lib.rs`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/src/acquisition.rs`

### 5. Unit tests or integration tests required
Spy implementations show read/evaluate cannot launch runners or fetch network; missing capabilities return explicit errors.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Application use cases can be tested independently without implicit host privileges.

### 8. Commit message
```text
refactor: separate application capability boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s013"></a>
## S013 — Add domain-specific identity wrappers

**Predecessor:** S012. **Contracts:** R18, R19, R50. **Acceptance:** AC018, AC019, AC050.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Add domain-specific identity wrappers.

### 2. Purpose
Prevent interchange of unrelated digest meanings.

### 3. Exact scope of code changes
Extend digest/ID types with the approved raw-object, semantic-evidence, plan/campaign/attempt and evaluation-context distinctions; use private validated construction and documented domains.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/digest.rs`
- `crates/eqm_domain/src/id.rs`
- `crates/eqm_domain/src/lib.rs`

### 5. Unit tests or integration tests required
Invalid/foreign digest domains reject; round trips and equality remain deterministic; no Default success identity.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Internal APIs cannot accidentally pass a workspace digest where a distinct identity is required.

### 8. Commit message
```text
feat: distinguish conformance identity domains
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s014"></a>
## S014 — Reject duplicate JSON members at ingestion

**Predecessor:** S013. **Contracts:** R18, R19, R21, R42, R64. **Acceptance:** AC018, AC019, AC021, AC042, AC064.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Reject duplicate JSON members at ingestion.

### 2. Purpose
Prevent ambiguous canonical or signed payload interpretation.

### 3. Exact scope of code changes
Add a bounded strict JSON ingestion path for protocol records that detects duplicate object names before conversion/canonicalization; preserve original raw bytes.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_protocol/tests/`

### 5. Unit tests or integration tests required
Duplicate top-level/nested keys reject; valid records retain exact raw bytes; malformed/deep input remains bounded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence is not normalized through an ambiguous last-key-wins parse.

### 8. Commit message
```text
fix: reject duplicate protocol object members
```

**Approved review regression contributions:** RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s015"></a>
## S015 — Enforce approved numeric and temporal wire forms

**Predecessor:** S014. **Contracts:** R18, R19, R21, R52, R64. **Acceptance:** AC018, AC019, AC021, AC052, AC064.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enforce approved numeric and temporal wire forms.

### 2. Purpose
Make cross-runtime digests predictable.

### 3. Exact scope of code changes
Retain DEV-002 S015-R1 through S015-R5 and complete DEV-003 S015-R6 through S015-R9: close exact version/kind ingress and nested inventory validation; repair runtime-facts byte/count bounds and retained subject syntax checks; enforce v2 schema markers and minimal UTC fractions; use actual schema validation with separate raw-token expectations; replace tautological corpus/application/CLI controls and reconcile evidence before S016.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/time.rs`
- `crates/eqm_domain/src/schema.rs`
- `crates/eqm_protocol/src/bin/generate_protocol_schemas.rs`
- `tools/xtask/src/main.rs`
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/`
- `crates/eqm_runner/src/inventory.rs`
- `crates/eqm_test_support/`
- `schemas/`
- `crates/eqm_protocol/tests/`
- `crates/eqm_manifest/src/`
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_protocol/Cargo.toml`

### 5. Unit tests or integration tests required
All DEV-002 and DEV-003 controls pass: full integer endpoints, canonical raw tokens, UTC/calendar/absence rules, exact root/nested versions, v1 preservation, all five schemas validated with Draft 2020-12, actual 16 MiB runtime-facts and fact-count bounds, literal corpus execution, successful reconcile control and fake-reader integration. Application doctests and schema parity pass; JSON-value conformance is distinguished from raw-token and cross-field validation.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_domain -p eqm_manifest -p eqm_application -p eqm -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
cargo test --locked -p eqm_application --doc
git diff --check
```

**Admission gate:** DEV-005: S015 accepted for allocated scope; S016-P1 also accepted. Retain corrective history and proceed to S016-R1. Later semantic/product acceptance remains with its owning steps.

### 7. Expected result
No consumer silently changes a number or time before hashing.

### 8. Commit message
```text
fix: enforce canonical numeric and time representations
```

**Approved review regression contributions:** RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s016"></a>
## S016 — Add canonical set and Unicode vectors

**Predecessor:** S015. **Contracts:** R18, R19, R21, R43, R64. **Acceptance:** AC018, AC019, AC021, AC043, AC064.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../decisions/ADR-0013-wire-namespace-and-representations.md), [implementation/reports/CODEX-121622f.md](../implementation/reports/CODEX-121622f.md), [implementation/reports/CODEX-6837831.md](../implementation/reports/CODEX-6837831.md), [implementation/reports/CODEX-3469fac.md](../implementation/reports/CODEX-3469fac.md), [implementation/reports/CODEX-1512c9f.md](../implementation/reports/CODEX-1512c9f.md).

### 1. Step title
Add canonical set and Unicode vectors.

### 2. Purpose
Distinguish semantic unordered collections from ordered scenarios.

### 3. Exact scope of code changes
S016 implementation and RF22 matrix are accepted by Codex DEV-007. Preserve the real-root canonical collection, NFC, raw-byte, duplicate and diagnostic regressions.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/`
- `crates/eqm_test_support/`
- `crates/eqm_engine/Cargo.toml`
- `schemas/v2/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Retain all executed S016-R1/R2/R3 controls and their independent evidence; RF22 is accepted for allocated scope.

### 6. Verification commands
```sh
cargo audit --deny warnings
cargo deny check
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_protocol -p eqm_domain -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

**Admission gate:** DEV-007 accepts S016; next active repair is S019-R1 affecting retained S017/S019 implementations.

### 7. Expected result
Canonicalization is executable protocol behavior rather than serializer convention.

### 8. Commit message
```text
test: lock canonical collection and string semantics
```

**Approved review regression contributions:** RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s017"></a>
## S017 — Hash complete binding semantics

**Predecessor:** S016. **Contracts:** R19. **Acceptance:** AC019.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../decisions/ADR-0013-wire-namespace-and-representations.md), [implementation/reports/CODEX-6837831.md](../implementation/reports/CODEX-6837831.md), [implementation/reports/CODEX-3469fac.md](../implementation/reports/CODEX-3469fac.md), [implementation/reports/CODEX-1512c9f.md](../implementation/reports/CODEX-1512c9f.md).

### 1. Step title
Hash complete binding semantics.

### 2. Purpose
Correct the ID-only binding digest shortcut.

### 3. Exact scope of code changes
Retain the complete typed v2 binding projection and accepted RF24/RF26 repairs. DEV-007 S019-R1 corrects all/any to canonical semantic sets shared with contract applicability; preserve v1 and unaffected literal oracles.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical.rs`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_domain/src/digest.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Retain typed API, extension type distinction, actual loader formatting/mutation and projected-duplicate controls. Add DEV-007 RF27 genuine all/any permutations, duplicate rejection and heterogeneous JCS-order traps; retain semantic mutation and ordered extension tests.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Binding identity binds actual approved semantics.

### 8. Commit message
```text
fix: bind evidence to canonical binding semantics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s018"></a>
## S018 — Hash complete evidence specifications

**Predecessor:** S017. **Contracts:** R19. **Acceptance:** AC019.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [implementation/reports/CODEX-3469fac.md](../implementation/reports/CODEX-3469fac.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../decisions/ADR-0013-wire-namespace-and-representations.md), [implementation/reports/CODEX-1512c9f.md](../implementation/reports/CODEX-1512c9f.md).

### 1. Step title
Hash complete evidence specifications.

### 2. Purpose
Correct the ID-pair evidence-specification shortcut.

### 3. Exact scope of code changes
Complete v2 evidence-specification projection and typed identity accepted by Codex DEV-007, including recursive extension tags, literal controls and actual manifest-boundary evidence. Retain the original and repair commits. RF25 ordered-child misclassification is superseded by RF27 in S019-R1.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_domain/src/evidence.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Retain selector/kind/count/runner/freshness mutations. Exercise integer-versus-text extension pairs and nested arrays/objects at both component projections, typed return and compile-fail domain substitution, actual loader/display/duplicate/expression controls, unchanged empty-extension and v1 goldens. Execute manifest doctests.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --locked -p eqm_manifest --doc
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence cannot match a changed specification merely by retaining names.

### 8. Commit message
```text
fix: hash exact evidence specification semantics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s019"></a>
## S019 — Separate contract and policy digests

**Predecessor:** S018. **Contracts:** R19, R58. **Acceptance:** AC019, AC058.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [implementation/reports/CODEX-3469fac.md](../implementation/reports/CODEX-3469fac.md), [decisions/ADR-0013-wire-namespace-and-representations.md](../decisions/ADR-0013-wire-namespace-and-representations.md), [implementation/reports/CODEX-1512c9f.md](../implementation/reports/CODEX-1512c9f.md), [implementation/reports/CODEX-eb2f323.md](../implementation/reports/CODEX-eb2f323.md).

### 1. Step title
Separate contract and policy digests.

### 2. Purpose
Stop populating unrelated provenance with one workspace digest.

### 3. Exact scope of code changes
Retain accepted applicability repair and implemented DEV-006/007 provenance assembly. Finish DEV-008 S019-R3: real minimum rule projection, literal endpoint expectations, two-rule JCS order trap and absent dimension-description control. Existing requirements, crate directions and v1 forms remain.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_domain/tests/`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_manifest/src/lib.rs`
- `crates/eqm_manifest/tests/`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_application/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Retain all S019-R1/R2 controls and execute DEV-008 remaining RF28 literal/production-boundary assertions with valid controls; preserve original complete provenance matrix.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_manifest -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --locked -p eqm_domain -p eqm_manifest -p eqm_application --doc
cargo xtask check
cargo xtask schemas check
git diff --check
```

**Admission gate:** DEV-008: complete DEV008-P1, then S019-R3, S021-R1 and S022 completion before ordered successors. No extra approval pause follows objective green checks.

### 7. Expected result
Internal prepared context has distinct accurate contract/policy identities.

### 8. Commit message
```text
fix: separate contract and policy provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s020"></a>
## S020 — Separate target configuration identity

**Predecessor:** S019. **Contracts:** R19, R22. **Acceptance:** AC019, AC022.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [implementation/reports/CODEX-1512c9f.md](../implementation/reports/CODEX-1512c9f.md).

### 1. Step title
Separate target configuration identity.

### 2. Purpose
Bind actual target configuration rather than the full graph summary.

### 3. Exact scope of code changes
Implement DEV-007 exact current-target configuration: pure manifest projection over Target with root exactly id/root/platform/framework/owners/extensions, existing TargetConfigurationDigest, and same-PreparedSession exact TargetId lookup with typed missing/projection errors. No invented build/profile/source fields or workspace fallback; retain separate subject/policy axes and later projection-extension obligations.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Literal bytes and independent domain hash; typed API; same-ID root/platform/framework/owner/semantic-extension changes; owner permutations; integer/text distinction; display and actual formatting stability; unrelated-target/policy isolation; exact missing-target error and same-session reuse.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_domain -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** DEV-007 resolves S020 representation; start after green S019-R1/R2. Model only present Target fields; later role/source owners extend projection atomically.

### 7. Expected result
Target evidence can be checked against a meaningful configuration identity.

### 8. Commit message
```text
fix: derive exact target configuration identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s021"></a>
## S021 — Bind runner identity to approved executable bytes

**Predecessor:** S020. **Contracts:** R19, R30. **Acceptance:** AC019, AC030.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [implementation/reports/CODEX-eb2f323.md](../implementation/reports/CODEX-eb2f323.md).

### 1. Step title
Bind runner identity to approved executable bytes.

### 2. Purpose
Preserve executable provenance in persisted evidence.

### 3. Exact scope of code changes
Retain executable-bound resolution but complete DEV-008 S021-R1: explicit bounded runner-owned v2 JCS projection of full definition, resolved program and approved guarantees; existing RunnerDigest, typed identity_v2/canonical_bytes accessors and explicit digest wire adapter. Preserve v1 shapes/oracles, unresolved rejection and record runtime identity change.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/resolution.rs`
- `crates/eqm_runner/tests/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Execute DEV-008 RF30 literal v2 bytes/domain hash, typed/compile-fail doctests, real executable acquisition/replacement and map-boundary controls, definition/context changes, locked/repository/unresolved cases, full-range quantities, template/literal distinction, set/argv order and extension/display semantics.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --locked -p eqm_runner --doc
cargo xtask check
git diff --check
```

### 7. Expected result
Runner evidence cannot reuse a definition digest after executable substitution.

### 8. Commit message
```text
fix: include executable content in runner identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s022"></a>
## S022 — Normalize repository and source provenance explicitly

**Predecessor:** S021. **Contracts:** R19, R22. **Acceptance:** AC019, AC022.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [implementation/reports/CODEX-eb2f323.md](../implementation/reports/CODEX-eb2f323.md).

### 1. Step title
Normalize repository and source provenance explicitly.

### 2. Purpose
Prevent display remote strings and dirty HEAD from implying exact clean source.

### 3. Exact scope of code changes
Complete DEV-008 conservative source inspection and clean release gate. Inspect repository-wide tracked/index/untracked/ignored state with only reserved untracked .eqm outputs excluded, reject Git concealment flags, preserve typed identity/error handling and causal failures. Move generated fixture inputs to .eqm before enforcing clean provenance; preserve original release outcomes and separate actual execution/subject claims. DEV-010 S022-R2 requires ancestor symlink and canonical descendant confinement before any nested Git invocation, preserving recursive conservative provenance.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_cli/src/commands/attest.rs`

### 5. Unit tests or integration tests required
Execute DEV-008 actual clean Pass/unconfigured Unknown controls, generated-output neutrality, tracked/staged/deleted/untracked/ignored non-output and tracked-output non-authorizing mutants, concealment flags and submodule/path controls. Preserve normalization failures, original release outcomes and golden evidence.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** DEV-008 resolves the representation block. Execute after DEV008-P1, S019-R3 and S021-R1; no requirement is deferred. Continue every ordered eligible S023-S211 after green completion.

### 7. Expected result
Source identity reflects inspected/executed bytes and shared identity logic replaces copies.

### 8. Commit message
```text
fix: bind source provenance beyond clean head claims
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s023"></a>
## S023 — Model raw and semantic evidence identities

**Predecessor:** S022. **Contracts:** R18, R21, R22, R35. **Acceptance:** AC018, AC021, AC022, AC035.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Model raw and semantic evidence identities.

### 2. Purpose
Preserve exact signed bytes independently of normalized assertions.

### 3. Exact scope of code changes
Add validated raw-object/semantic-evidence relation types and conversion results without storage or trust promotion; keep existing records readable only under the version ADR. DEV-009 S023-R1 repairs raw-record binding and lossless explicit v2 semantic projection; newly added v1 identity helper is not a v2 contract.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_protocol/tests/`

### 5. Unit tests or integration tests required
Alternate whitespace transports differ in raw identity but agree semantically; signed originals are returned unchanged; changed assertions differ. Also execute raw mismatch rejection, neighboring large-value separation, all assertion-axis mutations, canonical sets/ordered attempts, bounds and independently computed complete literal v2 bytes/digest.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence normalization no longer conflates raw collision with semantic identity.

### 8. Commit message
```text
feat: separate raw and semantic evidence identity
```

**Approved review regression contributions:** RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s024"></a>
## S024 — Model plans campaigns and attempts

**Predecessor:** S023. **Contracts:** R16, R17, R18. **Acceptance:** AC016, AC017, AC018.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Model plans campaigns and attempts.

### 2. Purpose
Establish exact acquisition lineage before selection/persistence.

### 3. Exact scope of code changes
Add validated plan/campaign/run/logical-case/attempt identifiers and explicit relationships; reject inconsistent or missing required lineage instead of deriving it from timestamps. DEV-009 S024-R1 adds exact lineage validation against declared execution and independently supplied expected plan/campaign context.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_domain/src/id.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Duplicate attempt names under the same execution reject ambiguous construction; distinct runs/cases remain independent. Test every missing and mismatched lineage axis, attempt membership and run/case independence separately.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
The engine can reason about attempts without counting JSON files.

### 8. Commit message
```text
feat: model evidence execution lineage
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s025"></a>
## S025 — Validate evidence identity relationships

**Predecessor:** S024. **Contracts:** R14, R15, R16, R18, R19, R21, R64. **Acceptance:** AC014, AC015, AC016, AC018, AC019, AC021, AC064.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate evidence identity relationships.

### 2. Purpose
Catch contradictions among envelope and payload subjects.

### 3. Exact scope of code changes
Validate matching scope/target/unit/requirements/facets/kind and identity lineage across protocol-to-domain conversion; require explicit typed absence where allowed. DEV-009 freezes checked bytes-first v2 admission with explicit lineage/execution/expected typed context, recomputed semantic identity, local relationship/count/time checks and no trust promotion or implicit v1 conversion. DEV-010 S025-R1 retains existing typed component expectations and checked access, validates ordered retry chronology at v2 admission, with full one-axis real-ingress controls; v1 stays unchanged. DEV-011 S025-R2 adds actual prepared-session/projection-to-admission integration, real context/access compile-fail controls and every execution-bearing temporal matrix.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_protocol/tests/`
- `crates/eqm_application/tests/`
- `crates/eqm_application/Cargo.toml`
- `Cargo.lock`

### 5. Unit tests or integration tests required
Mismatched payload kind, subject, attempt or semantic digest reject before evaluation; valid fixtures round-trip. Use genuine positive fixtures and one-axis RA02/RA13 mutants, including mutation after DTO parsing; no helper-only or self-hash acceptance.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_domain -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Structurally valid JSON cannot create internally contradictory evidence.

### 8. Commit message
```text
fix: validate evidence envelope relationships
```

**Approved review regression contributions:** RA02, RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s026"></a>
## S026 — Model the acyclic record dependency chain

**Predecessor:** S025. **Contracts:** R20, R28. **Acceptance:** AC020, AC028.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Model the acyclic record dependency chain.

### 2. Purpose
Prevent evidence-to-decision digest cycles.

### 3. Exact scope of code changes
Add candidate/plan/input-manifest/decision/reference distinctions and validate directed references before publication; do not require future release decisions in evidence. DEV-009 allocates typed record-kind references to already-defined supplied registry entries along the approved acyclic lifecycle; reject missing/self/future/cyclic references without storage/publication. DEV-011 S026-R1 freezes explicit definition-history order, already-defined historical deployment inputs for Runtime/FrozenInput, forbidden forward definitions and sorted typed references while preserving cycle/direction checks.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_protocol/src/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Candidate-to-evidence-to-decision chains validate; self/future/cyclic record references reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Readiness records can be created in a realizable order.

### 8. Commit message
```text
feat: enforce acyclic evaluation record identities
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s027"></a>
## S027 — Add cross-domain provenance regression corpus

**Predecessor:** S026. **Contracts:** R18, R19, R20, R21, R22. **Acceptance:** AC018, AC019, AC020, AC021, AC022.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Add cross-domain provenance regression corpus.

### 2. Purpose
Lock identity semantics before target and evaluator changes.

### 3. Exact scope of code changes
Consolidate approved raw/semantic/component/source/lineage vectors into shared fixtures and golden identities without live credentials. DEV-009 includes all repaired raw/semantic/nested-source/runner/lineage controls and complete literal oracles in real owning boundaries, not only declarative vector inventory.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `crates/eqm_test_support/src/`
- `crates/eqm_protocol/tests/`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Mutation of every bound axis prevents inappropriate reuse; formatting-only changes behave as declared; fixture generator is deterministic.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support -p eqm_protocol -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
Future layers consume tested identity contracts with stable regression evidence.

### 8. Commit message
```text
test: lock conformance provenance boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s028"></a>
## S028 — Model product roles separately from targets

**Predecessor:** S027. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model product roles separately from targets.

### 2. Purpose
Support asymmetric CLI/control-tower/edge behavior.

### 3. Exact scope of code changes
Add validated role metadata to the logical target model without hardcoding the user's whole product; keep existing target IDs stable. DEV-007: bind the newly modeled exact role ID in the target configuration projection in this same step; preserve v1 forms and prove same-target role mutation changes its v2 identity. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/id.rs`
- `crates/eqm_domain/tests/`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Distinct target outputs may share a role; role changes do not accidentally rename targets or imply participation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Product role is explicit and independent of platform/build identity.

### 8. Commit message
```text
feat: separate product roles from shipped targets
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s029"></a>
## S029 — Model source components

**Predecessor:** S028. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model source components.

### 2. Purpose
Represent shared implementation input without duplicate target roots.

### 3. Exact scope of code changes
Add source-component identities and confined roots with explicit target-to-component relations; retain domain-only validation. DEV-007: bind newly modeled target-to-source-component ID references in the target projection as canonical ID sets in this same step; exact source content remains a separate subject axis. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/graph.rs`
- `crates/eqm_domain/tests/`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Shared components attach to multiple targets; duplicate authority and invalid path identities reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Shared KMP/Rust source ownership is representable without filesystem exceptions.

### 8. Commit message
```text
feat: model shared source components
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s030"></a>
## S030 — Resolve source dependency edges

**Predecessor:** S029. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Resolve source dependency edges.

### 2. Purpose
Make transitive shared-source ownership explicit.

### 3. Exact scope of code changes
Resolve component references, target dependencies and cycles in graph preparation; reject dangling/cyclic unsupported dependencies deterministically. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/resolve.rs`
- `crates/eqm_engine/src/invariants.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Shared dependency DAG resolves; missing nodes/cycles and ambiguous ownership produce stable diagnostics.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Source dependency graph is validated before affected analysis.

### 8. Commit message
```text
feat: resolve shared source dependencies
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s031"></a>
## S031 — Extend authored target and source schemas

**Predecessor:** S030. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Extend authored target and source schemas.

### 2. Purpose
Expose the approved model through generated manifests.

### 3. Exact scope of code changes
Add target role/source-component DTOs using the version decision; regenerate schemas and examples atomically without implicit old aliases. DEV-007: round-trip all new target role/source references into the target identity through authored schemas and the real loader; no new normative target field may be dropped from its projection. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/dto/`
- `crates/eqm_manifest/src/conversion/`
- `schemas/`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Strict decode/encode/schema vectors cover shared components and unknown fields; current explicit version behavior is tested.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Authored shared-source metadata has a versioned strict representation.

### 8. Commit message
```text
feat: author shared source target metadata
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s032"></a>
## S032 — Load overlapping shared-source targets safely

**Predecessor:** S031. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Load overlapping shared-source targets safely.

### 2. Purpose
Remove duplicate-root rejection only where explicit shared ownership permits it.

### 3. Exact scope of code changes
Update loader root handling to distinguish output identity from shared source roots, preserve portable collision checks, and reject ambiguous undeclared overlaps. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF47: S032-R1 uses the same checked transitive component closure for overlap and binding ownership, retaining confinement and unrelated-root rejection.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/loader.rs`
- `crates/eqm_manifest/tests/`

- `crates/eqm_engine/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Two approved outputs share a component; undeclared root collision and path escape reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Natural KMP layouts load without weakening repository containment.

### 8. Commit message
```text
fix: load explicitly shared target source roots
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s033"></a>
## S033 — Validate artifacts against declared source ownership

**Predecessor:** S032. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Validate artifacts against declared source ownership.

### 2. Purpose
Stop requiring every valid shared artifact under one output root.

### 3. Exact scope of code changes
Update binding conversion and structural expectations to use declared source-component ownership; keep wrong-target and undeclared paths blocking. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF46: S033-R1 routes resolved shared ownership into production CLI structural checking and executes declared-shared versus undeclared/escaped controls.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/conversion/`
- `crates/eqm_engine/src/structure.rs`
- `crates/eqm_engine/tests/`

- `crates/eqm_cli/src/commands/check.rs`
- `crates/eqm_application/tests/`
- `crates/eqm_cli/tests/`

### 5. Unit tests or integration tests required
Shared commonMain artifact validates for dependent outputs; unrelated component and escaped paths fail.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_engine -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Shared code is valid only through explicit dependency authority.

### 8. Commit message
```text
feat: validate artifacts through source ownership
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s034"></a>
## S034 — Model capability participation rules

**Predecessor:** S033. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model capability participation rules.

### 2. Purpose
Express intentional role differences independently of waivers.

### 3. Exact scope of code changes
Add validated requirement/target/profile participation selectors aligned with existing applicability vocabulary; reject ambiguous selectors and duplicate authority. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/policy.rs`
- `crates/eqm_domain/tests/`

### 5. Unit tests or integration tests required
Mobile-only capture and desktop-only review fixtures validate; unspecified required context stays representably unknown.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Product asymmetry can be authored without permanent waiver abuse.

### 8. Commit message
```text
feat: model target capability participation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s035"></a>
## S035 — Author and resolve participation metadata

**Predecessor:** S034. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Author and resolve participation metadata.

### 2. Purpose
Make participation part of protected graph semantics.

### 3. Exact scope of code changes
Add strict DTO/conversion/resolution and semantic hashing for approved participation rules; regenerate schemas together. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF50: S035-R1 rejects duplicate requirement/target coordinates at real graph preparation and public participation evaluation, never first-ID wins.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/dto/`
- `crates/eqm_manifest/src/conversion/`
- `crates/eqm_engine/src/resolve.rs`
- `schemas/`

- `crates/eqm_domain/src/graph.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Round-trip participation rules; undeclared targets/profiles and weakening graph mutations are diagnosable.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine -p eqm_manifest -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Participation is versioned semantic authority, not a UI-only filter.

### 8. Commit message
```text
feat: resolve authored capability participation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s036"></a>
## S036 — Evaluate applicability per participant coordinate

**Predecessor:** S035. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Evaluate applicability per participant coordinate.

### 2. Purpose
Preserve target-specific applicability and unknown context.

### 3. Exact scope of code changes
Extend engine context so participation/applicability can be evaluated for exact requirement/participant/profile combinations; keep true/false/unknown distinct. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF50: retain coordinate evaluation contribution but reject ambiguous supplied authorities through S035-R1 before accepting this step.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/applicability.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Same requirement is true for edge, false for explicit exclusion, unknown for missing profile; no early global drop.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Applicability no longer incorrectly applies one pre-fanout conclusion to all targets.

### 8. Commit message
```text
fix: evaluate applicability for exact participants
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s037"></a>
## S037 — Model role-bearing scenario occurrences

**Predecessor:** S036. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model role-bearing scenario occurrences.

### 2. Purpose
Represent multiple actors of one shipped target.

### 3. Exact scope of code changes
Add scenario participant occurrence IDs/roles and exact dependency mappings for end-to-end scope; do not collapse repeated devices into a set. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF48: S037-R1 includes exact v2 scenario reader/schema/discovery/conversion/index/reference validation and complete graph identity, not domain-only modeling.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_domain/tests/`

- `crates/eqm_manifest/src/`
- `crates/eqm_engine/src/`
- `crates/eqm_application/tests/`
- `schemas/v2/manifest/scenario.schema.json`

### 5. Unit tests or integration tests required
Two mobile occurrences remain distinct; duplicate occurrence IDs and missing roles reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine -p eqm_manifest -p eqm_protocol -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Multi-device journeys have unambiguous participant identity.

### 8. Commit message
```text
feat: model scenario participant occurrences
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s038"></a>
## S038 — Model exact multi-subject manifests

**Predecessor:** S037. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model exact multi-subject manifests.

### 2. Purpose
Bind each actor to its own source build artifact and configuration.

### 3. Exact scope of code changes
Add validated subject manifests for target/provider/scenario participants with explicit required identity fields and typed absence for allowed development cases. DEV-007: exact subjects retain separate target-configuration, source snapshot/component content and build artifact identities; a target digest alone cannot satisfy missing source/build inputs. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF49: S038-R1 aligns symbolic id/domain/wire, rejects logical-key duplicates, validates exact occurrence/context assignments with explicit development absence, and projects a separately typed subject identity.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_protocol/tests/`

- `crates/eqm_domain/src/subject_manifest.rs`
- `crates/eqm_application/tests/`
- `crates/eqm_manifest/src/canonical/`

### 5. Unit tests or integration tests required
Different valid mobile/desktop builds coexist; substituted artifact/build/config mismatches reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol -p eqm_manifest -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Cross-target context does not require identical build digests.

### 8. Commit message
```text
feat: bind exact multi-participant subjects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s039"></a>
## S039 — Separate comparison context identity

**Predecessor:** S038. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Separate comparison context identity.

### 2. Purpose
Prevent exact build identity from blocking valid comparability.

### 3. Exact scope of code changes
Define shared contract/scenario/logical-profile/comparison-rule identity separately from subject manifest/evaluation identity and project it canonically. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-014 supplies retained clarification: complete RF51/RF52/RF53 corrections before S040, graph-backed provider/cohort matching and genuine asymmetric parity; preserve all original tests, dependencies and criteria. DEV-015 governs checked comparison coverage, exact retained cohort/configuration authority at preparation, full definition/schema invariants and real asymmetric consumer parity; all original criteria and the independent S043 gate remain. DEV-017 binds consumed resolved cohort values to the exact current graph/scenario and independently rederived axes, and completes owning v2 schema value parity before independent P03 acceptance; no original criterion is relaxed.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/digest.rs`
- `crates/eqm_engine/src/equivalence.rs`
- `crates/eqm_manifest/src/canonical/`

### 5. Unit tests or integration tests required
Approved differing builds share comparison context; different scenario/profile/contract does not compare.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Comparison eligibility and exact subject verification remain independent.

### 8. Commit message
```text
feat: distinguish comparison context from exact subjects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s040"></a>
## S040 — Bind provider obligations explicitly

**Predecessor:** S039. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Bind provider obligations explicitly.

### 2. Purpose
Avoid unscoped provider participation in unrelated targets.

### 3. Exact scope of code changes
Represent provider subject/configuration and target/scenario dependency links; adjust domain/graph checks while deferring final aggregation to evaluator steps. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-014 supplies retained clarification: complete RF51/RF52/RF53 corrections before S040, graph-backed provider/cohort matching and genuine asymmetric parity; preserve all original tests, dependencies and criteria.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_engine/src/resolve.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Missing required provider blocks its dependents; an unrelated provider is not automatically attached to every target.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Shared services have exact subject and dependency semantics.

### 8. Commit message
```text
feat: bind shared provider dependencies
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s041"></a>
## S041 — Author compatibility cohorts

**Predecessor:** S040. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Author compatibility cohorts.

### 2. Purpose
Support approved mixed-version combinations without inventing a support window.

### 3. Exact scope of code changes
Add strict data representation and validation for declared required participant/build cohorts and scenario coverage; leave operational cohort values explicit. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-014 supplies retained clarification: complete RF51/RF52/RF53 corrections before S040, graph-backed provider/cohort matching and genuine asymmetric parity; preserve all original tests, dependencies and criteria. DEV-015 governs checked comparison coverage, exact retained cohort/configuration authority at preparation, full definition/schema invariants and real asymmetric consumer parity; all original criteria and the independent S043 gate remain. DEV-017 binds consumed resolved cohort values to the exact current graph/scenario and independently rederived axes, and completes owning v2 schema value parity before independent P03 acceptance; no original criterion is relaxed.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/policy.rs`
- `crates/eqm_manifest/src/dto/`
- `crates/eqm_manifest/src/conversion/`
- `schemas/`

### 5. Unit tests or integration tests required
Approved N/N+1 fixture cohort works; undeclared combination and absent participant reject.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Independently released targets can be tested against explicit compatibility policy.

### 8. Commit message
```text
feat: define explicit compatibility cohorts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s042"></a>
## S042 — Add the asymmetric multi-runtime graph fixture

**Predecessor:** S041. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Add the asymmetric multi-runtime graph fixture.

### 2. Purpose
Exercise the target/source/participant model as a whole.

### 3. Exact scope of code changes
Add synthetic Rust CLI, desktop, mobile and provider authority with shared source sets, repeated actors and role-specific requirements; no invented real product build tasks. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-014 supplies retained clarification: complete RF51/RF52/RF53 corrections before S040, graph-backed provider/cohort matching and genuine asymmetric parity; preserve all original tests, dependencies and criteria. DEV-015 governs checked comparison coverage, exact retained cohort/configuration authority at preparation, full definition/schema invariants and real asymmetric consumer parity; all original criteria and the independent S043 gate remain.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `crates/eqm_test_support/src/`
- `crates/eqm_manifest/tests/`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Load/resolve/canonicalize fixture and assert stable roles/dependencies/cohorts; intentional exclusions are not waivers.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support -p eqm_manifest -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Later evaluator tests have a representative but honestly synthetic product graph.

### 8. Commit message
```text
test: add asymmetric multi-runtime graph coverage
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s043"></a>
## S043 — Verify target model schema and identity parity

**Predecessor:** S042. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Verify target model schema and identity parity.

### 2. Purpose
Close graph-model changes before obligation routing.

### 3. Exact scope of code changes
Update public usage examples and schema parity assertions for new target/subject/participation records, preserving explicit compatibility decisions. DEV-007: include regressions proving role/source-reference changes invalidate target configuration while separately bound source/build content changes invalidate the appropriate subject; missing axes never become success. DEV-009 closes the exact engineering allocation for this step; apply its EQM-P03 representation table together with this original scope and criteria. DEV-012 RF45-RF50: cycle exit verifies production v2 graph identity/version routing, checked transitive structural ownership, nonambiguous participation and exact scenario/subject/provider/cohort ingress with public parity. DEV-014 supplies retained clarification: complete RF51/RF52/RF53 corrections before S040, graph-backed provider/cohort matching and genuine asymmetric parity; preserve all original tests, dependencies and criteria. DEV-015 governs checked comparison coverage, exact retained cohort/configuration authority at preparation, full definition/schema invariants and real asymmetric consumer parity; all original criteria and the independent S043 gate remain. DEV-017 binds consumed resolved cohort values to the exact current graph/scenario and independently rederived axes, and completes owning v2 schema value parity before independent P03 acceptance; no original criterion is relaxed.

### 4. Files/modules likely involved
- `docs/concepts.md`
- `docs/manifests.md`
- `examples/`
- `crates/eqm_protocol/tests/`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
Full schema regeneration comparison, example validation and component-digest regressions pass.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
Authored/protocol/domain target models agree under the approved version contract.

### 8. Commit message
```text
test: verify shared target model parity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s044"></a>
## S044 — Move exact selection orchestration into application

**Predecessor:** S043. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Move exact selection orchestration into application.

### 2. Purpose
Remove CLI-only development selection assumptions.

### 3. Exact scope of code changes
Extract policy/profile request preparation into application operations with explicit evaluation mode and authority input; keep host verification separate.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_engine/src/selection.rs`

### 5. Unit tests or integration tests required
Development baseline behavior remains explicit and protected requests cannot take an implicit development branch.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Selection is reusable without deriving authority from the calling command name.

### 8. Commit message
```text
refactor: share exact policy selection preparation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s045"></a>
## S045 — Reject duplicate profile requests

**Predecessor:** S044. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Reject duplicate profile requests.

### 2. Purpose
Prevent silent last-write-wins profile selection.

### 3. Exact scope of code changes
Replace collection-overwrite request parsing with duplicate detection for profiles/dimensions/revisions and retain exact diagnostics.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_engine/src/selection.rs`

### 5. Unit tests or integration tests required
Duplicate same/conflicting profile values reject; distinct complete selections work.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Public selection cannot silently discard caller intent.

### 8. Commit message
```text
fix: reject duplicate profile selections
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s046"></a>
## S046 — Select exact policy and profile revisions

**Predecessor:** S045. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Select exact policy and profile revisions.

### 2. Purpose
Remove the one-development-policy restriction without guessing authority.

### 3. Exact scope of code changes
Accept exact selected references according to the approved API decision; resolve multiple revisions deterministically and require explicit selection when ambiguous.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_cli/src/cli.rs`
- `crates/eqm_manifest/src/validation.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Multiple stored policies/revisions no longer force arbitrary selection; ambiguous/missing authority rejects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Explicit policy selection works across development and protected modes.

### 8. Commit message
```text
feat: select exact policy and profile revisions
```

**Approved review regression contributions:** RA16.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s047"></a>
## S047 — Enumerate required evaluation coordinates

**Predecessor:** S046. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59, R60. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059, AC060.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enumerate required evaluation coordinates.

### 2. Purpose
Establish the full denominator before reading evidence.

### 3. Exact scope of code changes
Introduce a pure required-universe result for participating targets/providers/scenarios and profile/facet coordinates using approved authority.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Expected coordinates are complete and deterministic for asymmetric/shared-provider/repeated-actor fixtures.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Later evidence filtering cannot shrink required scope.

### 8. Commit message
```text
feat: enumerate the required evaluation universe
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s048"></a>
## S048 — Retain unknown applicability coordinates

**Predecessor:** S047. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59, R60. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059, AC060.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Retain unknown applicability coordinates.

### 2. Purpose
Repair skipped requirements that can disappear from completeness.

### 3. Exact scope of code changes
Carry explicit unknown and justified not-applicable coordinates alongside derived obligations; preserve requirement references and reasons.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/src/applicability.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Missing profile context remains in scope and cannot yield a vacuous conformant target.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Unknown applicability is visible and non-authorizing.

### 8. Commit message
```text
fix: retain unresolved applicability in evaluation scope
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s049"></a>
## S049 — Validate completeness construction

**Predecessor:** S048. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R23, R24, R26, R50, R58, R59, R60, R63. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC023, AC024, AC026, AC050, AC058, AC059, AC060, AC063.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md), [implementation/DEVIATIONS.md](../implementation/DEVIATIONS.md).

### 1. Step title
Validate completeness construction.

### 2. Purpose
Replace freely asserted completeness with a checked boundary.

### 3. Exact scope of code changes
Apply DEV-001: preserve the stronger fragment/journey inherited risk, reject explicitly lower requirement risk, and revalidate the expanded graph before finalization. Close unchecked finalized-graph construction with a justified acyclic crate boundary. Add a validated prepared-scope constructor accounting for every expected coordinate and participant; restrict unchecked construction and deserialization of completeness.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/tests/`
- `crates/eqm_engine/src/expand.rs`
- `crates/eqm_engine/src/invariants.rs`
- `crates/eqm_domain/src/graph.rs`
- `crates/eqm_manifest/src/canonical.rs`

### 5. Unit tests or integration tests required
Test absent and explicit fragment risks in both lower/higher destination journeys, invalid expanded graphs, public construction bypasses, omitted/duplicate coordinates, and complete failing/unknown scope. Exercise the real manifest-to-expansion-to-preparation path; use compile-fail/API tests where appropriate.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** ADR-0012 records owner approval; do not re-request the settled direction. S006 closes exact engineering representations and any newly evidenced conflicting consumer obligation. Live product/authority values remain governed by Q01-Q15; DEV-001 defines the S049 amendment.

### 7. Expected result
Complete evaluation cannot be forged by a boolean or partial map.

### 8. Commit message
```text
feat: validate complete evaluation scope
```

**Approved review regression contributions:** RA01, RA05, RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s050"></a>
## S050 — Reject accidental empty release scope

**Predecessor:** S049. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59, R60. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059, AC060.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Reject accidental empty release scope.

### 2. Purpose
Avoid vacuous release success and meaningless comparison claims.

### 3. Exact scope of code changes
Add explicit admission semantics for empty required release scope and singleton comparisons according to protected policy; retain local inspection usefulness.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_engine/src/equivalence.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Empty unauthorized release is non-authorizing; singleton is labeled no meaningful comparison; approved explicit exception is not invented by defaults.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Empty work cannot become a release pass.

### 8. Commit message
```text
fix: reject vacuous release assurance
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s051"></a>
## S051 — Compose policy constraints per facet

**Predecessor:** S050. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R23, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC023, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Compose policy constraints per facet.

### 2. Purpose
Prevent unrelated rule strengths from leaking across facets.

### 3. Exact scope of code changes
Derive facet-specific matching rule sets and strength using the approved normative contract. Compare protected obligation-universe inclusion plus per-coordinate constraints when checking policy strengthening; increasing a minimum-level selection floor is not automatically stronger. Preserve all matched-rule reasons.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Test different facets with different trust/count/age constraints and compare derived universes before/after policy changes, including Recommended-to-Required selection-floor changes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Per-facet constraints match authored policy intent.

### 8. Commit message
```text
fix: compose obligation strength per facet
```

**Approved review regression contributions:** RA04, RA07.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s052"></a>
## S052 — Derive provider and end-to-end dependencies

**Predecessor:** S051. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Derive provider and end-to-end dependencies.

### 2. Purpose
Build complete non-target scopes for later aggregation.

### 3. Exact scope of code changes
Emit exact provider/scenario obligations with participant/dependency linkage; do not flatten cross-target coverage into separate target passes.
DEV-030 freezes explicit requirement-to-scenario preparation inputs, exact provider
dependencies and the separate scope-binding projection. Include domain, application,
manifest, CLI, usage docs and their owning test lanes in this allocation.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Missing provider/end-to-end evidence remains independently required; repeated same-target actors are preserved.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
All approved scope kinds are present in the required universe.

### 8. Commit message
```text
feat: derive exact cross-runtime obligations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s053"></a>
## S053 — Bind expected selection manifests

**Predecessor:** S052. **Contracts:** R08, R11, R12, R13, R14, R15, R16, R17, R18, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Bind expected selection manifests.

### 2. Purpose
Make missing test cases observable independently of result files.

### 3. Exact scope of code changes
Represent and derive required logical case/observation selection manifests from approved binding/specification policy; reject ambiguous count units.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence.rs`
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Report with all reported cases passing but an expected case missing remains incomplete; selectors/count units stay exact.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
The evaluator knows what should have run before trusting what was reported.

### 8. Commit message
```text
feat: bind expected evidence selections
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s054"></a>
## S054 — Generate complete public obligation identities

**Predecessor:** S053. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Generate complete public obligation identities.

### 2. Purpose
Fix IDs that omit profiles and release/candidate context.

### 3. Exact scope of code changes
Replace ad hoc string formatting with a versioned complete coordinate identity/projection; update query DTO references coherently.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Profile/revision/subject/context changes cannot collide; deterministic identical coordinates retain identity.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Public references uniquely identify the actual obligation.

### 8. Commit message
```text
fix: include full context in obligation identities
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s055"></a>
## S055 — Prepare structural observations independently

**Predecessor:** S054. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R24, R28, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC024, AC028, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Prepare structural observations independently.

### 2. Purpose
Avoid echoing authored artifact roles as observations.

### 3. Exact scope of code changes
Define a read-only observation boundary that supplies validated file kind/containment and independently supported roles; absent role authority stays unknown.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/structure.rs`
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/check.rs`

### 5. Unit tests or integration tests required
A declared service role on an unverified file does not prove role match; independent adapter roles are checked.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Structural evaluation consumes observations rather than self-confirming declarations.

### 8. Commit message
```text
fix: separate authored roles from structural observations
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s056"></a>
## S056 — Require trusted complete inventory for absence

**Predecessor:** S055. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R24, R28, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC024, AC028, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Require trusted complete inventory for absence.

### 2. Purpose
Preserve three-valued observation semantics.

### 3. Exact scope of code changes
Consolidate inventory completeness/currentness/trust prerequisites in prepared supporting facts; retain provenance and absence/unknown distinction.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/exposure.rs`
- `crates/eqm_runner/src/inventory.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Partial/missing/stale/untrusted inventories cannot prove absence; complete verified inventory can.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Presence/absence conclusions have explicit supporting authority.

### 8. Commit message
```text
fix: preserve inventory absence prerequisites
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s057"></a>
## S057 — Add normative derivation property tests

**Predecessor:** S056. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R23, R50, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC023, AC050, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Add normative derivation property tests.

### 2. Purpose
Test policy rules independently of orchestration shortcuts.

### 3. Exact scope of code changes
Add table/property coverage for ordering, unknown scope, empty scope, monotonic policy composition and protected candidate deletions using reference expected universes.

### 4. Files/modules likely involved
- `crates/eqm_engine/tests/obligations.rs`
- `crates/eqm_engine/tests/properties.rs`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Input permutations preserve identities; stronger requirements never shrink required work; omissions never increase assurance.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Derivation invariants are executable and independently reviewable.

### 8. Commit message
```text
test: lock complete obligation derivation semantics
```

**Approved review regression contributions:** RA04, RA05, RA07.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s058"></a>
## S058 — Expose complete obligation query DTOs

**Predecessor:** S057. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R21, R44, R45, R52, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC021, AC044, AC045, AC052, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Expose complete obligation query DTOs.

### 2. Purpose
Make required/unresolved/non-applicable scope visible to consumers.

### 3. Exact scope of code changes
Map the prepared universe into versioned protocol records with reasons and context without loading/evaluating evidence or inventing final statuses.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/query.rs`
- `crates/eqm_cli/src/commands/obligations.rs`
- `schemas/`

### 5. Unit tests or integration tests required
Query contains every required coordinate and explicit pending/unknown facts; filtering is presentation-only.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Obligation inspection truthfully explains scope before acquisition.

### 8. Commit message
```text
feat: expose complete obligation scope views
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s059"></a>
## S059 — Convert protocol evidence through one validated boundary

**Predecessor:** S058. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R19, R21, R24, R29, R43, R64. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC021, AC024, AC029, AC043, AC064.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Convert protocol evidence through one validated boundary.

### 2. Purpose
Remove divergent domain construction paths.

### 3. Exact scope of code changes
Provide a single checked DTO-to-domain conversion for all currently supported evidence payloads and lineage; no trust conclusion is created by parsing.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Each valid kind converts; inconsistent or unsupported payload/subject/identity fields reject deterministically.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_domain -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Engine inputs share one strict conversion contract.

### 8. Commit message
```text
refactor: centralize validated evidence conversion
```

**Approved review regression contributions:** RA02, RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s060"></a>
## S060 — Classify duplicate transport and immutable conflicts

**Predecessor:** S059. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R22, R24, R29, R35, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC022, AC024, AC029, AC035, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Classify duplicate transport and immutable conflicts.

### 2. Purpose
Correct duplicate-ID poisoning without hiding contradictory attempts.

### 3. Exact scope of code changes
Add pure candidate-set normalization that deduplicates identical semantic delivery and distinguishes same-attempt conflicts; retain all raw/provenance references.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/coverage.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Duplicate transport leaves counts/status unchanged; conflicting authenticated same-attempt candidates remain non-authorizing in affected scope.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Harmless duplicate delivery no longer manufactures unknown coverage.

### 8. Commit message
```text
fix: distinguish duplicate evidence from attempt conflicts
```

**Approved review regression contributions:** RA03, RA08, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s061"></a>
## S061 — Match complete evidence coordinates

**Predecessor:** S060. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R19, R21, R24, R29, R35, R43, R64. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC021, AC024, AC029, AC035, AC043, AC064.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Match complete evidence coordinates.

### 2. Purpose
Bind every required coverage axis before aggregation.

### 3. Exact scope of code changes
Update coverage expectations for exact component digests, subject manifests, profiles, participant roles and campaign/selection lineage; preserve rejected-axis explanations.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/coverage.rs`
- `crates/eqm_engine/tests/coverage.rs`

### 5. Unit tests or integration tests required
One-axis substitution tests reject wrong build/spec/role/profile/runner/context; irrelevant rejected candidates cannot create success.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Only exact compatible evidence reaches outcome evaluation.

### 8. Commit message
```text
fix: match complete conformance evidence coordinates
```

**Approved review regression contributions:** RA03, RA08, RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s062"></a>
## S062 — Validate distinct logical observations

**Predecessor:** S061. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate distinct logical observations.

### 2. Purpose
Prevent file/attempt multiplication from meeting counts.

### 3. Exact scope of code changes
Implement approved logical-case identity and expected-selection completeness checks across normalized evidence; keep distinct case observations separate from retries.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/tests/outcomes.rs`

### 5. Unit tests or integration tests required
Split reports, duplicate imports and ten retries cannot replace missing distinct expected cases.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Count credit reflects the policy-defined logical unit.

### 8. Commit message
```text
fix: count distinct selected observations
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s063"></a>
## S063 — Apply policy-composed count constraints

**Predecessor:** S062. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Apply policy-composed count constraints.

### 2. Purpose
Resolve inconsistency between specification counts and stronger policy minimums.

### 3. Exact scope of code changes
Apply the coordinate's approved count semantics and strongest relevant minimum during evaluation, independently of runner acquisition summaries.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Policy minimum exceeding specification minimum remains missing; exact counts meet thresholds without file-count shortcuts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Policy count constraints cannot be bypassed by lower acquisition thresholds.

### 8. Commit message
```text
fix: enforce policy evidence count constraints
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s064"></a>
## S064 — Preserve retry history within a campaign

**Predecessor:** S063. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Preserve retry history within a campaign.

### 2. Purpose
Keep failure/pass instability visible.

### 3. Exact scope of code changes
Aggregate all immutable attempts for each selected case within the authorized campaign; implement canonical unknown/unstable/failure precedence and retain reasons.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Fail-then-pass is unstable; timeout/cancel remains unknown where normative; order permutations do not erase history.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Retrying never silently converts an unstable campaign into clean success.

### 8. Commit message
```text
fix: retain campaign retry history
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s065"></a>
## S065 — Evaluate manual review payloads explicitly

**Predecessor:** S064. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate manual review payloads explicitly.

### 2. Purpose
Remove the possibility of catch-all success for failed review.

### 3. Exact scope of code changes
Use explicit manual-review outcome/count/authority semantics in the canonical evaluator and reject unsupported interpretation.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/tests/outcomes.rs`

### 5. Unit tests or integration tests required
Passed, failed, skipped, quarantined, canceled and unverified review fixtures follow the normative table.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Manual review cannot become satisfied merely because its payload parsed.

### 8. Commit message
```text
fix: evaluate manual review outcomes explicitly
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s066"></a>
## S066 — Evaluate executable payload selections explicitly

**Predecessor:** S065. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate executable payload selections explicitly.

### 2. Purpose
Preserve skipped filtered and quarantined case semantics.

### 3. Exact scope of code changes
Unify tests/structural-check/snapshot payload aggregation over validated expected selections and attempts; prohibit terminal zero-exit shortcuts.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_runner/src/normalized.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Passed-with-missing/skipped/filtered/quarantined cases, zero selection and malformed counts remain non-success as specified.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Executable evidence uses the same complete case semantics across consumers.

### 8. Commit message
```text
fix: preserve executable evidence selection outcomes
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s067"></a>
## S067 — Evaluate inventory and runtime payloads explicitly

**Predecessor:** S066. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate inventory and runtime payloads explicitly.

### 2. Purpose
Do not treat non-test payload existence as success.

### 3. Exact scope of code changes
Define supported inventory/runtime/release-reference payload evaluation with exact counts, referenced objects, completeness and supporting prerequisites; reject unspecified kinds.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/src/exposure.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Incomplete inventory, failing counts, missing referenced record and wrong subject cannot satisfy an obligation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
All supported payload families have closed tested semantics.

### 8. Commit message
```text
fix: evaluate non-test evidence prerequisites
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s068"></a>
## S068 — Select campaigns under explicit authority

**Predecessor:** S067. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Select campaigns under explicit authority.

### 2. Purpose
Avoid arbitrary timestamp order and cherry-picked passing runs.

### 3. Exact scope of code changes
Implement pure campaign eligibility/ordering/supersession selection from the approved policy; return ambiguous or missing selection explicitly.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_engine/tests/selection.rs`
- `crates/eqm_domain/src/policy.rs`

### 5. Unit tests or integration tests required
Forged newer timestamps do not replace authority; a later authorized campaign can be selected without erasing older failures.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Current evidence selection is reproducible and audit history remains separate.

### 8. Commit message
```text
feat: select authorized evidence campaigns
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s069"></a>
## S069 — Evaluate exact freshness with explicit clock semantics

**Predecessor:** S068. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Evaluate exact freshness with explicit clock semantics.

### 2. Purpose
Align all consumers on temporal validity.

### 3. Exact scope of code changes
Consolidate age/skew and component-key freshness checks using approved precision and boundary rules; distinguish historical context from current checks without cache integration yet.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/freshness.rs`
- `crates/eqm_engine/tests/freshness.rs`

### 5. Unit tests or integration tests required
Exact expiry/skew boundaries, future times and each changed digest axis produce specified fresh/stale/unknown results.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence freshness is deterministic for explicit input time and context.

### 8. Commit message
```text
fix: unify exact evidence freshness semantics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s070"></a>
## S070 — Represent independent verification facts

**Predecessor:** S069. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R26, R29, R43, R50, R63. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC026, AC029, AC043, AC050, AC063.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Represent independent verification facts.

### 2. Purpose
Keep missing verification distinct from low verified trust.

### 3. Exact scope of code changes
Define the pure engine input for checked identity/issuer/subject/producer/temporal dimensions and their required completeness; no real cryptography in the engine.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_domain/src/`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
A missing mandatory dimension remains unknown; known insufficient authority remains non-success; producer claims cannot raise facts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Later verifier adapters must supply complete independent acceptance facts.

### 8. Commit message
```text
feat: model independent evidence verification facts
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s071"></a>
## S071 — Apply only precise contributing waivers

**Predecessor:** S070. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R19, R24, R29, R43, R58. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC024, AC029, AC043, AC058.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Apply only precise contributing waivers.

### 2. Purpose
Prevent unrelated or expired exceptions changing outcomes.

### 3. Exact scope of code changes
Route canonical waiver evaluation to exact scope/profile/subject/context and protected approval inputs; preserve baseline non-waivable unknown/unstable cases.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/waivers.rs`
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/tests/waivers.rs`

### 5. Unit tests or integration tests required
Expired, broad, unrelated, wrong-profile or unapproved waivers reject; only eligible blockers produce visible conditional state.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Waivers never become satisfied evidence or a graph-wide blanket list.

### 8. Commit message
```text
fix: bind waivers to exact contributing obligations
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s072"></a>
## S072 — Derive canonical facet decisions with full reasons

**Predecessor:** S071. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Derive canonical facet decisions with full reasons.

### 2. Purpose
Make one normative status implementation explainable.

### 3. Exact scope of code changes
Combine validated applicability, context, coverage, outcomes, freshness, verification, supporting checks and waivers into a structured facet decision retaining reasons.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/src/diagnostics.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Exhaustive precedence table tests include simultaneous unknown/failure/unstable/stale/missing and applied-waiver cases.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Summary precedence is explicit and no cause is lost through a single color/status.

### 8. Commit message
```text
refactor: derive explainable canonical facet decisions
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s073"></a>
## S073 — Aggregate complete participant conformance

**Predecessor:** S072. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Aggregate complete participant conformance.

### 2. Purpose
Remove target-only partial-map assurance.

### 3. Exact scope of code changes
Aggregate facets through validated universe and provider/scenario dependency participation; replace caller-supplied policy_resolution_complete booleans at internal callers.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/tests/`
- `crates/eqm_application/src/evaluation.rs`

### 5. Unit tests or integration tests required
Missing provider/end-to-end/unknown coordinates block appropriate participants; complete failures remain valid complete decisions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
No target is graded using a silently incomplete obligation subset.

### 8. Commit message
```text
fix: aggregate complete participant conformance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s074"></a>
## S074 — Derive contract-relative comparison conclusions

**Predecessor:** S073. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Derive contract-relative comparison conclusions.

### 2. Purpose
Compare required cohorts without conflating build equality.

### 3. Exact scope of code changes
Update equivalence evaluation to use approved comparison context and complete subject/cohort results; require declared relational evidence rather than inventing a general comparison language.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/equivalence.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Different legitimate builds compare; missing relational scenario or wrong cohort remains blocking; extra targets do not change required scope.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Equivalence claims are scoped and supported by the required relational evidence.

### 8. Commit message
```text
fix: scope equivalence to complete comparison contracts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s075"></a>
## S075 — Assemble a simple reference evaluation path

**Predecessor:** S074. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Assemble a simple reference evaluation path.

### 2. Purpose
Provide an independent semantic oracle for future optimization.

### 3. Exact scope of code changes
Compose the tested pure operations into an unoptimized reference evaluation over frozen fixtures with explicit provenance/reasons; keep no alternative production policy interpretation.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/`
- `crates/eqm_engine/tests/`
- `crates/eqm_test_support/src/`

### 5. Unit tests or integration tests required
Golden lifecycle cases match manually specified outcomes and all non-success states; input permutations give identical decisions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
A readable reference path establishes expected whole-evaluation behavior.

### 8. Commit message
```text
test: establish reference conformance evaluation
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s076"></a>
## S076 — Lock adversarial evidence regression fixtures

**Predecessor:** S075. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R23, R24, R29, R35, R43, R50. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC023, AC024, AC029, AC035, AC043, AC050.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Lock adversarial evidence regression fixtures.

### 2. Purpose
Validate the complete semantic slice before persistence and host work.

### 3. Exact scope of code changes
Add full fixture campaigns covering duplicates/conflicts/retries/counts/unknown scopes/mixed subjects and real non-test failures; update normative docs when representation choices are resolved.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `crates/eqm_engine/tests/`
- `docs/eqm_product_v1/specs/EVALUATION_SEMANTICS.md`

### 5. Unit tests or integration tests required
Reference path passes the adversarial matrix and stronger-policy/deleted-evidence monotonicity checks.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Semantic defects from the review have focused regression coverage before command routing.

### 8. Commit message
```text
test: cover adversarial evidence evaluation
```

**Approved review regression contributions:** RA03, RA05, RA08.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s077"></a>
## S077 — Select and pin the storage implementation

**Predecessor:** S076. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Select and pin the storage implementation.

### 2. Purpose
Establish safe runtime and durability assumptions before persistence code.

### 3. Exact scope of code changes
Resolve Q10 from actual dependencies/hosts; choose an audited exact SQLite wrapper/runtime containing the WAL-reset fix and record linking/features/durability design; update lockfile intentionally.

### 4. Files/modules likely involved
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_application/Cargo.toml`
- `docs/eqm_product_v1/decisions/`

### 5. Unit tests or integration tests required
Inspect linked SQLite runtime/fix and license/advisory results; compile chosen features; no network-filesystem assumption.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Storage uses a justified fixed audited runtime rather than an unchecked system dependency.

### 8. Commit message
```text
build: pin the reviewed local storage runtime
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s078"></a>
## S078 — Define store operations and typed failures

**Predecessor:** S077. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Define store operations and typed failures.

### 2. Purpose
Keep storage contracts independent of SQL and privilege assumptions.

### 3. Exact scope of code changes
Add bounded object/index/read-snapshot interfaces and typed storage errors for unavailable/corrupt/conflict/limit conditions; avoid a generic unrestricted file API.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/mod.rs`
- `crates/eqm_application/src/error.rs`

### 5. Unit tests or integration tests required
In-memory contract tests demonstrate missing objects and conflicts never produce verified/success states.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Application operations can rely on explicit storage semantics without trusting metadata.

### 8. Commit message
```text
refactor: define immutable evidence store boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s079"></a>
## S079 — Initialize protected store roots safely

**Predecessor:** S078. **Contracts:** R18, R22, R26, R33, R35, R36, R37, R38, R42, R50, R52. **Acceptance:** AC018, AC022, AC026, AC033, AC035, AC036, AC037, AC038, AC042, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Initialize protected store roots safely.

### 2. Purpose
Handle concurrent creation and confinement.

### 3. Exact scope of code changes
Create private staging/object/index roots with checked existing-directory behavior, concurrent AlreadyExists revalidation and explicit development/trusted location selection.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_runner/src/persistence.rs`

### 5. Unit tests or integration tests required
Concurrent first initialization succeeds safely; symlink/wrong-type roots and candidate-writable trusted configurations reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Store initialization is idempotent and does not race into unsafe destinations.

### 8. Commit message
```text
fix: initialize evidence storage safely
```

**Approved review regression contributions:** RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s080"></a>
## S080 — Write bounded immutable staging objects

**Predecessor:** S079. **Contracts:** R18, R22, R26, R33, R35, R36, R37, R38, R42, R50, R52. **Acceptance:** AC018, AC022, AC026, AC033, AC035, AC036, AC037, AC038, AC042, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Write bounded immutable staging objects.

### 2. Purpose
Prevent unbounded buffering before persistence validation.

### 3. Exact scope of code changes
Stream or bounded-read staged objects, compute raw digest, validate limits and retain handles; keep new objects invisible to the index.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/objects.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Oversized/changing input and short writes fail without publication; exact bytes/digest are retained for valid input.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Staging cannot allocate or publish unbounded unvalidated evidence.

### 8. Commit message
```text
feat: stage bounded immutable evidence objects
```

**Approved review regression contributions:** RA12, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s081"></a>
## S081 — Install objects durably without clobbering

**Predecessor:** S080. **Contracts:** R18, R22, R26, R33, R35, R36, R37, R38, R42, R50, R52. **Acceptance:** AC018, AC022, AC026, AC033, AC035, AC036, AC037, AC038, AC042, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Install objects durably without clobbering.

### 2. Purpose
Make object publication crash-safe and collision-aware.

### 3. Exact scope of code changes
Implement durable no-clobber install and supported-host file/directory sync behavior; bound existing-file comparisons and retain raw collision distinction.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/objects.rs`
- `crates/eqm_runner/src/persistence.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Identical installs are idempotent; different bytes under same raw identity reject; oversized existing files are not read unbounded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
An installed object is immutable and safely published before any index reference.

### 8. Commit message
```text
feat: publish immutable objects durably
```

**Approved review regression contributions:** RA12, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s082"></a>
## S082 — Create versioned local metadata tables

**Predecessor:** S081. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Create versioned local metadata tables.

### 2. Purpose
Introduce the smallest index schema matching established identities.

### 3. Exact scope of code changes
Add schema version metadata and tables/constraints for raw/semantic objects, execution lineage and committed references; do not store trusted booleans as authority.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/src/storage/migrations/`

### 5. Unit tests or integration tests required
Fresh initialization/version mismatch/constraint tests pass; incompatible writers reject explicitly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Local metadata has explicit versioning and identity constraints.

### 8. Commit message
```text
feat: initialize the versioned evidence index
```

**Approved review regression contributions:** RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s083"></a>
## S083 — Commit import references atomically

**Predecessor:** S082. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Commit import references atomically.

### 2. Purpose
Enforce object-first/index-second publication.

### 3. Exact scope of code changes
Add transactions that reference already validated installed objects and atomically index semantic/execution relationships; no visibility before commit.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Inject rollback before commit and reopen; no completed row references unpublished bytes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Import publication has a clear atomic visibility boundary.

### 8. Commit message
```text
feat: commit evidence references atomically
```

**Approved review regression contributions:** RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s084"></a>
## S084 — Persist idempotent imports and attempt conflicts

**Predecessor:** S083. **Contracts:** R17, R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC017, AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Persist idempotent imports and attempt conflicts.

### 2. Purpose
Make replay safety survive process restarts.

### 3. Exact scope of code changes
Enforce uniqueness by appropriate semantic/attempt identity, retain alternate raw transports, and persist contradictory attempt conflicts instead of run-only rejection.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_runner/src/ci.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Reimport after restart stays idempotent; many results from one CI run work; same-attempt contradictions remain recorded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Replay handling is persistent, precise and not overly coarse.

### 8. Commit message
```text
fix: persist exact evidence replay identities
```

**Approved review regression contributions:** RA08, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s085"></a>
## S085 — Persist campaigns and execution receipts

**Predecessor:** S084. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Persist campaigns and execution receipts.

### 2. Purpose
Preserve history and selection authority across restarts.

### 3. Exact scope of code changes
Store plan/campaign/case/attempt lineage, terminal worker receipts and explicit supersession/selection metadata with validated references.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Interrupted acquisitions remain distinguishable from completed campaigns; old attempts survive new campaign selection.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Execution history supports deterministic current selection and audit.

### 8. Commit message
```text
feat: persist evidence campaign history
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s086"></a>
## S086 — Store runtime facts and release subject records

**Predecessor:** S085. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Store runtime facts and release subject records.

### 2. Purpose
Unify bounded validated supporting inputs.

### 3. Exact scope of code changes
Index immutable runtime observations, candidate/subject manifests and release-related records by exact subjects/context; reject malformed referenced objects.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Wrong-subject, missing-reference and duplicate delivery cases behave consistently with evidence objects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Supporting facts can be loaded coherently rather than enumerating arbitrary directories.

### 8. Commit message
```text
feat: store exact runtime and release inputs
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s087"></a>
## S087 — Persist verification receipts without minting authority

**Predecessor:** S086. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Persist verification receipts without minting authority.

### 2. Purpose
Retain cryptographic context for replay and current checks.

### 3. Exact scope of code changes
Store receipts and supporting material references as records requiring verification on import; expose raw facts through a checked retrieval path.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/src/trust.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Forged serialized verified fields are not accepted; missing trust material remains unavailable; originals remain immutable.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Storage preserves receipts but cannot convert them into live authority.

### 8. Commit message
```text
feat: retain verification receipt provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s088"></a>
## S088 — Read consistent index snapshots

**Predecessor:** S087. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Read consistent index snapshots.

### 2. Purpose
Prevent mixed-generation evaluation inputs.

### 3. Exact scope of code changes
Add bounded read transactions/snapshot generation handles and validate referenced immutable objects before preparing inputs; do not hold transactions during test execution.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/src/evaluation.rs`

### 5. Unit tests or integration tests required
Concurrent import during preparation cannot alter selected input generation; missing/corrupt objects fail explicitly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Evaluation preparation observes one coherent store state.

### 8. Commit message
```text
feat: read consistent evidence snapshots
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s089"></a>
## S089 — Implement indexed coordinate queries

**Predecessor:** S088. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Implement indexed coordinate queries.

### 2. Purpose
Avoid scanning and parsing every evidence file.

### 3. Exact scope of code changes
Query by exact subject/specification/profile/campaign/attempt coordinates using appropriate indexes; preserve deterministic ordering and rejected-record explanations.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Indexed queries match a full-scan reference on synthetic histories, including conflicts and irrelevant records.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Evidence lookup is precise and testably equivalent to the simple reference.

### 8. Commit message
```text
feat: index exact evidence coordinates
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s090"></a>
## S090 — Bound writers and own checkpoints

**Predecessor:** S089. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Bound writers and own checkpoints.

### 2. Purpose
Make SQLite contention behavior explicit.

### 3. Exact scope of code changes
Implement short writer transactions, bounded busy handling and checkpoint ownership/configuration; refuse unsupported local-store configurations as specified.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Concurrent readers/writers/checkpoints have bounded outcomes and no silent partial commits; linked runtime check is exercised.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
WAL operation has explicit contention and durability guarantees.

### 8. Commit message
```text
fix: bound evidence index contention
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s091"></a>
## S091 — Recover staged and orphaned objects

**Predecessor:** S090. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Recover staged and orphaned objects.

### 2. Purpose
Make crash cleanup safe rather than destructive.

### 3. Exact scope of code changes
Implement recovery classification for staged/orphan/committed objects with conservative handling of active operations; never fabricate missing committed objects.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/recovery.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Faults at each publication boundary recover coherently; live staged work is not deleted and missing committed bytes are reported.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Restart recovery does not produce phantom completed evidence.

### 8. Commit message
```text
feat: recover interrupted evidence publication
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s092"></a>
## S092 — Protect live readers and acquisitions with leases

**Predecessor:** S091. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Protect live readers and acquisitions with leases.

### 2. Purpose
Prevent cleanup racing active use.

### 3. Exact scope of code changes
Add explicit bounded lease/reference ownership for ongoing reads/acquisitions and release archives, with tested expiry/recovery semantics from operator policy.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/retention.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Concurrent cleanup cannot remove objects used by readers/runs; stale orphan lease handling follows explicit rules.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Object lifecycle accounts for active consumers and workers.

### 8. Commit message
```text
feat: protect live evidence references
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s093"></a>
## S093 — Implement policy-driven retention and GC

**Predecessor:** S092. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Implement policy-driven retention and GC.

### 2. Purpose
Reclaim only authorized unreachable data.

### 3. Exact scope of code changes
Add retention classes and reachability-based cleanup using configured policy and leases; preserve required signed originals/release archives and sensitive-data boundaries.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/retention.rs`
- `crates/eqm_application/tests/`
- `docs/eqm_product_v1/runbooks/`

### 5. Unit tests or integration tests required
Class/lease/archive tests prevent premature deletion; eligible orphans/history are reclaimed deterministically.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Immutability is compatible with safe explicit retention, not infinite unbounded growth.

### 8. Commit message
```text
feat: reclaim eligible evidence objects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s094"></a>
## S094 — Rebuild indexes from retained manifests

**Predecessor:** S093. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Rebuild indexes from retained manifests.

### 2. Purpose
Make the index replaceable without losing trust/history.

### 3. Exact scope of code changes
Add bounded reconstruction from immutable manifests/receipts with conflict preservation and validation; do not trust filenames or create receipt authority during rebuild.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/recovery.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Delete only a disposable fixture index and rebuild; identities, history, conflicts and decisions match original records.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Recovery can reconstruct discoverability without inventing accepted evidence.

### 8. Commit message
```text
feat: rebuild evidence indexes safely
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s095"></a>
## S095 — Test explicit database migration and restore

**Predecessor:** S094. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Test explicit database migration and restore.

### 2. Purpose
Verify compatibility before schema evolution.

### 3. Exact scope of code changes
Implement approved migration coordination/backup-or-restore mechanics and one controlled fixture transition; retain original immutable/signature objects.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/migrations/`
- `crates/eqm_application/tests/`
- `docs/eqm_product_v1/specs/COMPATIBILITY.md`

### 5. Unit tests or integration tests required
Upgrade, interruption, reopen, incompatible writer and rollback/restore tests pass on declared hosts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Database version transitions are atomic or recoverable and never silent reinterpretations.

### 8. Commit message
```text
test: verify evidence store migration recovery
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s096"></a>
## S096 — Complete storage fault-injection coverage

**Predecessor:** S095. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Complete storage fault-injection coverage.

### 2. Purpose
Close persistence guarantees before production routing.

### 3. Exact scope of code changes
Add publication/transaction/checkpoint/cleanup interruption points and bounded corrupt-file fixtures; document which hosts/filesystems were actually tested.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
AC018/AC033/AC035–AC038/AC052 storage cases pass on supported hosts or remain explicit blocked gates.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Store readiness is supported by failure tests rather than filesystem assumptions.

### 8. Commit message
```text
test: cover evidence storage failure boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s097"></a>
## S097 — Define host execution capabilities

**Predecessor:** S096. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Define host execution capabilities.

### 2. Purpose
Stop treating authored guarantees as operator authority.

### 3. Exact scope of code changes
Add explicit host/backend capability configuration and validation, including paths, permissions, network/secrets and limits; declarations cannot self-authorize capabilities.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/resolution.rs`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/tests/`

### 5. Unit tests or integration tests required
Unsupported guarantees reject before spawn; local declarations cannot claim isolation not supplied by the host.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Resolution is constrained by independent host authority.

### 8. Commit message
```text
fix: resolve runners against actual host capabilities
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s098"></a>
## S098 — Build complete non-executing acquisition plans

**Predecessor:** S097. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Build complete non-executing acquisition plans.

### 2. Purpose
Make dry-run a concrete safety artifact.

### 3. Exact scope of code changes
Resolve executable identity, argv, cwd, selector/expected cases, environment references, limits and outputs into a deterministic plan without launching programs.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/substitution.rs`
- `crates/eqm_cli/src/commands/verify.rs`

### 5. Unit tests or integration tests required
Spawn/network spies stay unused; every required plan field is present; unresolved executables and selectors fail explicitly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Dry-run reveals exact work rather than only specification IDs.

### 8. Commit message
```text
feat: prepare concrete acquisition plans
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s099"></a>
## S099 — Bind explicit approval to exact plans

**Predecessor:** S098. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Bind explicit approval to exact plans.

### 2. Purpose
Prevent execution after meaningful plan substitution.

### 3. Exact scope of code changes
Introduce a checked approval boundary for plan identity/authority; verify approved immutable inputs immediately before execution and reject changed executable/argv/context.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/resolution.rs`
- `crates/eqm_runner/tests/`

### 5. Unit tests or integration tests required
Changing any approved plan axis invalidates approval; read-only callers cannot create execution permission.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Authorized execution is tied to the plan the operator reviewed.

### 8. Commit message
```text
feat: bind execution approval to exact plans
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s100"></a>
## S100 — Centralize bounded confined file access

**Predecessor:** S099. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Centralize bounded confined file access.

### 2. Purpose
Remove validate-then-unbounded-read shortcuts.

### 3. Exact scope of code changes
Add a narrowly scoped safe handle-based filesystem utility for source/result/import reads using reviewed APIs; preserve byte/type/ancestor containment checks and document host differences.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_runner/src/`
- `crates/eqm_manifest/src/`
- `Cargo.toml`
- `Cargo.lock`

### 5. Unit tests or integration tests required
Parent-symlink, replacement, oversized read and invalid-type fixtures reject; any dependency/unsafe exception is reviewed before use.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Callers have one tested confinement and bounded-read boundary.

### 8. Commit message
```text
fix: centralize bounded confined file access
```

**Approved review regression contributions:** RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s101"></a>
## S101 — Validate result files after execution

**Predecessor:** S100. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate result files after execution.

### 2. Purpose
Ensure pre-created temp paths cannot bypass limits.

### 3. Exact scope of code changes
Replace verify fs::read with bounded handle acquisition and post-execution validation; reject replaced/symlink/oversized/malformed result files before normalization.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_runner/src/normalized.rs`
- `crates/eqm_application/src/acquisition.rs`

### 5. Unit tests or integration tests required
Worker replaces output path, emits huge file or inconsistent selector; all fail safely without unbounded allocation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Only safe bounded exact runner results enter evidence normalization.

### 8. Commit message
```text
fix: validate runner output files before loading
```

**Approved review regression contributions:** RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s102"></a>
## S102 — Prepare immutable worker input and output roots

**Predecessor:** S101. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Prepare immutable worker input and output roots.

### 2. Purpose
Separate candidate execution from authoritative state.

### 3. Exact scope of code changes
Materialize or verify approved immutable sources and dedicated staging outputs; expose no evaluator/authority/index/signing write access; record dirty development mode separately.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/src/container.rs`

### 5. Unit tests or integration tests required
Worker attempts writes to protected fixture roots fail or unsupported configuration rejects before launch; input mutation invalidates identity.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Trusted acquisition does not execute against mutable self-authorizing inputs.

### 8. Commit message
```text
feat: isolate worker inputs and output staging
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s103"></a>
## S103 — Resolve explicit host environments and secrets

**Predecessor:** S102. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Resolve explicit host environments and secrets.

### 2. Purpose
Remove hardcoded PATH and empty-secret wiring.

### 3. Exact scope of code changes
Provide operator-approved host environment and secret references through the application boundary; retain env_clear and avoid logging values; no implicit inheritance or retrieval.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_cli/src/commands/verify.rs`

### 5. Unit tests or integration tests required
Environment marker absence, exact allowlist, missing secret failure and host-specific PATH fixtures pass without real credentials.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Native acquisition gets explicit portable inputs without ambient privilege.

### 8. Commit message
```text
feat: resolve authorized runner environment inputs
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s104"></a>
## S104 — Signal output overflow immediately

**Predecessor:** S103. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Signal output overflow immediately.

### 2. Purpose
Stop draining unlimited output until process exit.

### 3. Exact scope of code changes
Use signaled bounded stdout/stderr readers and supervised bounded stdin delivery; activate cancellation/deadline ownership before any potentially blocking request write. Terminate promptly on overflow; every input-write error retains child/task cleanup ownership.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/src/adapter.rs`
- `crates/eqm_runner/tests/security.rs`

### 5. Unit tests or integration tests required
Exercise input beyond pipe capacity, a worker refusing stdin or writing before reading, early stdin closure, output floods and cancellation during request delivery. Assert prompt bounded termination and cleanup, not only retained buffer size.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Output caps constrain execution promptly, not only final classification.

### 8. Commit message
```text
fix: terminate acquisition on output overflow
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s105"></a>
## S105 — Supervise pipes and descendants through completion

**Predecessor:** S104. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Supervise pipes and descendants through completion.

### 2. Purpose
Close the parent-exit hang.

### 3. Exact scope of code changes
Keep deadline/cancellation active through input delivery, descendant ownership, output drainage and cleanup. Implement bounded joins and cleanup after immediate-child exit; handle broken pipes and every supervisor error path without orphaning owned work.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/tests/security.rs`

### 5. Unit tests or integration tests required
Test a child exiting while descendants retain pipes, cancellation/deadline during input/drain, broken-pipe cleanup and bounded shutdown on each supported native host.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Immediate-child exit cannot move blocking reader joins outside supervision.

### 8. Commit message
```text
fix: bound descendant and pipe shutdown
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s106"></a>
## S106 — Wire cooperative and host cancellation

**Predecessor:** S105. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Wire cooperative and host cancellation.

### 2. Purpose
Make cancellation reachable from real callers.

### 3. Exact scope of code changes
Propagate cancellation from application/CLI lifecycle into owned execution and bounded cleanup; specify completion/publication races and retain committed results discoverably.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_cli/src/main.rs`
- `crates/eqm_runner/src/execution.rs`

### 5. Unit tests or integration tests required
Cancel before spawn, during execution/drain and around publication; no detached work or contradictory committed state.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Cancellation is an explicit lifecycle outcome rather than an unused token.

### 8. Commit message
```text
feat: propagate acquisition cancellation safely
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s107"></a>
## S107 — Harden Unix process ownership

**Predecessor:** S106. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Harden Unix process ownership.

### 2. Purpose
Make Unix tree cleanup and errors explicit.

### 3. Exact scope of code changes
Use approved safe OS abstractions for owned process groups/session handling; remove fragile assumptions about external kill invocation where needed and document containment limits.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/tests/security.rs`
- `docs/eqm_product_v1/specs/EXECUTION_SECURITY.md`

### 5. Unit tests or integration tests required
Unix-native child/grandchild/error cleanup tests pass; escape attempts are contained by approved host isolation or reject unsupported guarantees.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Unix supervision claims match tested host capability rather than best-effort direct-child kill.

### 8. Commit message
```text
fix: harden unix worker lifecycle ownership
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s108"></a>
## S108 — Implement supported Windows process ownership

**Predecessor:** S107. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Implement supported Windows process ownership.

### 2. Purpose
Avoid claiming tree termination from Child.kill alone.

### 3. Exact scope of code changes
After Q02/Q13, implement safe reviewed Windows Job Object ownership and kill-on-close/cancellation behavior, with explicit configuration for unsupported nested host cases.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/`
- `crates/eqm_runner/tests/security.rs`
- `Cargo.toml`
- `Cargo.lock`

### 5. Unit tests or integration tests required
Windows-native descendant/pipe/cancel/exit tests and target compilation pass; do not substitute Linux-only tests for this gate.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Windows support is implemented and evidenced, or recorded unsupported by an approved support-matrix decision.

### 8. Commit message
```text
feat: supervise supported windows worker trees
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s109"></a>
## S109 — Dispatch only supported acquisition backends

**Predecessor:** S108. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Dispatch only supported acquisition backends.

### 2. Purpose
Connect host-authorized execution paths coherently.

### 3. Exact scope of code changes
Add application dispatch for the repository's supported local/container/locked program mechanisms; unsupported resolution/execution capabilities return explicit errors without fallback.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/container.rs`
- `crates/eqm_runner/src/resolution.rs`

### 5. Unit tests or integration tests required
Backend selection, immutable locked identities, unavailable runtime and guarantee mismatch fixtures reject or execute exactly as approved.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Backend declarations correspond to real implemented capabilities.

### 8. Commit message
```text
feat: dispatch authorized acquisition backends
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s110"></a>
## S110 — Treat executable discovery as acquisition

**Predecessor:** S109. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Treat executable discovery as acquisition.

### 2. Purpose
Remove any read-only illusion around build-tool discovery.

### 3. Exact scope of code changes
Route discovery adapters that execute programs through approved planning/supervision; keep static bounded inspection separate and read-only.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/discover.rs`
- `crates/eqm_runner/src/adapter.rs`
- `crates/eqm_application/src/acquisition.rs`

### 5. Unit tests or integration tests required
A read-only discover request cannot run Gradle/scripts; explicitly approved discovery uses the same bounded worker path.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Discovery has the same privilege and result-validation rules as other execution.

### 8. Commit message
```text
fix: require authority for executable discovery
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s111"></a>
## S111 — Persist terminal acquisition receipts

**Predecessor:** S110. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Persist terminal acquisition receipts.

### 2. Purpose
Keep failures and canceled work visible in expected selections.

### 3. Exact scope of code changes
Record successful and failed/timeout/cancel/overflow terminal receipts with plan/campaign/case context where safely possible; keep malformed results as failures not absent successes.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_runner/src/normalized.rs`

### 5. Unit tests or integration tests required
Restart after failed acquisition retains expected missing/failed work; zero exit with malformed result never becomes evidence success.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Acquisition history explains terminal non-success across process restarts.

### 8. Commit message
```text
feat: retain terminal acquisition outcomes
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s112"></a>
## S112 — Bound scheduling and device ownership

**Predecessor:** S111. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Bound scheduling and device ownership.

### 2. Purpose
Establish safe concurrency before throughput optimization.

### 3. Exact scope of code changes
Add bounded work queues, concurrency admission and exclusive device/resource leases using explicit plan limits; deterministic result collection and owned shutdown.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Concurrent plans sharing a device do not overlap illegally; queue saturation/cancellation is explicit and no work is silently dropped.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Scheduling respects resource authority and cancellation.

### 8. Commit message
```text
feat: bound acquisition scheduling and device use
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s113"></a>
## S113 — Harden retained-output redaction

**Predecessor:** S112. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Harden retained-output redaction.

### 2. Purpose
Keep sensitive values out of plans reports and stored logs.

### 3. Exact scope of code changes
Centralize bounded secret redaction and safe error formatting, including truncation/multibyte/chunk-boundary cases and declared overlapping secrets; document limits against hostile exfiltration.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_application/src/error.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Test secrets spanning read chunks, truncated outputs, overlapping values and malformed UTF-8; no real secrets in fixtures.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Retained outputs obey the declared redaction contract without claiming sandbox security.

### 8. Commit message
```text
fix: harden bounded acquisition output redaction
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s114"></a>
## S114 — Enforce application-wide resource budgets

**Predecessor:** S113. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R44, R46, R47, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC044, AC046, AC047, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enforce application-wide resource budgets.

### 2. Purpose
Prevent graph/protocol/request workloads bypassing runner limits.

### 3. Exact scope of code changes
Apply configured byte/depth/expansion/profile/candidate/attachment/queue budgets at acquisition/preparation boundaries; no silent truncation of required scope.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/`
- `crates/eqm_runner/src/`

### 5. Unit tests or integration tests required
Oversized/deep/explosive fixtures terminate with explicit limit diagnostics and cannot authorize release.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_protocol -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Bounded behavior covers the whole input pipeline.

### 8. Commit message
```text
feat: enforce conformance resource budgets
```

**Approved review regression contributions:** RA12, RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s115"></a>
## S115 — Complete host adversarial acquisition tests

**Predecessor:** S114. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Complete host adversarial acquisition tests.

### 2. Purpose
Evidence the promises made by each backend.

### 3. Exact scope of code changes
Consolidate environment/confinement/flood/descendant/cancel/source-mutation/protected-write tests for each declared host, updating capability docs honestly.

### 4. Files/modules likely involved
- `crates/eqm_runner/tests/security.rs`
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
Run native host/security lanes and record unavailable/unsupported guarantees; no new blanket capability claims.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask test security
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Acquisition security is backed by explicit host execution evidence.

### 8. Commit message
```text
test: verify adversarial worker boundaries
```

**Approved review regression contributions:** RA11, RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s116"></a>
## S116 — Model protected authority configuration

**Predecessor:** S115. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Model protected authority configuration.

### 2. Purpose
Define authority independently of candidate files.

### 3. Exact scope of code changes
Add strict operator configuration for baseline expectations, required scope, verifier/evaluator identity and authorized trust dimensions; no live values or permissive defaults.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/`

### 5. Unit tests or integration tests required
Missing authority, duplicate identities and candidate-only release origin reject; test-only configs remain labeled.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Protected configuration is explicit validated input.

### 8. Commit message
```text
feat: configure protected evaluation authority
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s117"></a>
## S117 — Load protected baseline expectations

**Predecessor:** S116. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Load protected baseline expectations.

### 2. Purpose
Preserve protected contract participation and selections.

### 3. Exact scope of code changes
Acquire exact protected authority through an authorized loader independent of candidate paths; bind baseline contract/policy/participation/evidence mappings and profile values.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Candidate policy/default/path changes cannot substitute authority; missing exact baseline fails closed.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Protected evaluation receives exact independent expectations.

### 8. Commit message
```text
feat: load exact protected evaluation baselines
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s118"></a>
## S118 — Detect protected scope weakening

**Predecessor:** S117. **Contracts:** R14, R23, R24, R25, R26, R29, R38, R51, R58, R64. **Acceptance:** AC014, AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC058, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Detect protected scope weakening.

### 2. Purpose
Include deletions and reclassification in policy comparison.

### 3. Exact scope of code changes
Extend monotonicity/diff checks to required targets, requirements, participation, critical test selections and evidence mappings; allow changes only through approved authority transition.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/monotonicity.rs`
- `crates/eqm_engine/src/diff.rs`
- `crates/eqm_application/src/authority.rs`

### 5. Unit tests or integration tests required
Deleting/reclassifying a critical requirement/test/target cannot shrink protected work; approved versioned transitions are explicit.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
A candidate cannot pass by deleting what would fail.

### 8. Commit message
```text
fix: detect protected conformance scope weakening
```

**Approved review regression contributions:** RA04.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s119"></a>
## S119 — Bind the trusted evaluator identity

**Predecessor:** S118. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Bind the trusted evaluator identity.

### 2. Purpose
Prevent candidate code from grading itself in release mode.

### 3. Exact scope of code changes
Include approved EQM evaluator identity/provenance in protected preparation and receipts; reject candidate-modified/unapproved grader execution as trusted.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Substituted grader binary/version/config cannot produce accepted trusted preparation merely using the same candidate workspace.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Release assurance is tied to independently approved grading infrastructure.

### 8. Commit message
```text
feat: bind protected evaluator identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s120"></a>
## S120 — Define structured verification adapter results

**Predecessor:** S119. **Contracts:** R11, R23, R24, R25, R26, R29, R38, R50, R51, R63, R64. **Acceptance:** AC011, AC023, AC024, AC025, AC026, AC029, AC038, AC050, AC051, AC063, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Define structured verification adapter results.

### 2. Purpose
Avoid a single trust-level integer standing for all checks.

### 3. Exact scope of code changes
Implement verifier request/result contracts covering raw bytes, identity/issuer, signer authorization, builder, subject, policy and temporal material; construction crosses trusted adapter boundary.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust.rs`
- `crates/eqm_runner/src/ci.rs`
- `crates/eqm_engine/src/conformance.rs`

### 5. Unit tests or integration tests required
Missing required dimension stays explicit; serialized verified flags cannot manufacture accepted facts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Verifiers return specific checked claims rather than unconditional SignedCi.

### 8. Commit message
```text
refactor: expose structured verification conclusions
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s121"></a>
## S121 — Pin a reviewed verification implementation

**Predecessor:** S120. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Pin a reviewed verification implementation.

### 2. Purpose
Connect real cryptography without custom primitives.

### 3. Exact scope of code changes
Inspect current Sigstore bundle support and select an audited exact library or safely supervised verifier tool behind the neutral interface; update pins/lock/ADR and feature implications.

### 4. Files/modules likely involved
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_application/Cargo.toml`
- `crates/eqm_application/src/trust/`
- `docs/eqm_product_v1/decisions/`

### 5. Unit tests or integration tests required
License/advisory/feature checks pass; compile the real implementation; no disabled subject/claim verification flags.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
A production-capable cryptographic path exists without live authority being invented.

### 8. Commit message
```text
build: pin the reviewed evidence verifier
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s122"></a>
## S122 — Verify bundle integrity and exact signed bytes

**Predecessor:** S121. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Verify bundle integrity and exact signed bytes.

### 2. Purpose
Make signature verification real and byte-bound.

### 3. Exact scope of code changes
Implement bounded parsing and cryptographic verification of supported bundle material over the original payload, retaining exact raw/semantic relationships.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Valid test-only signed material verifies; tampering, malformed bundle, wrong payload/digest and unsupported algorithms reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Cryptographic verification runs rather than trusting a fixture boolean.

### 8. Commit message
```text
feat: verify exact evidence bundle integrity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s123"></a>
## S123 — Check signer identity issuer and authorization

**Predecessor:** S122. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Check signer identity issuer and authorization.

### 2. Purpose
Separate valid signatures from authorized evidence.

### 3. Exact scope of code changes
Enforce configured exact identities/issuers/allowed signers and scope authorization using verifier output; reject unrecognized or overbroad authority.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Correct signature from wrong issuer/signer or unauthorized scope rejects; correct configured fixture succeeds.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Signature validity alone cannot establish trusted EQM evidence.

### 8. Commit message
```text
feat: enforce authorized evidence signer identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s124"></a>
## S124 — Check builder source and artifact expectations

**Predecessor:** S123. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Check builder source and artifact expectations.

### 2. Purpose
Bind verified evidence to what was actually intended.

### 3. Exact scope of code changes
Validate producer/builder/repository/source/build/artifact/configuration/selection against protected expected subjects; reject unrecognized security-relevant inputs.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
One-axis subject/build/source substitution and unexpected builder parameters fail without dropping other reasons.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Verified evidence cannot be replayed for a different product subject.

### 8. Commit message
```text
feat: verify exact evidence producer expectations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s125"></a>
## S125 — Check temporal and trust-material eligibility

**Predecessor:** S124. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Check temporal and trust-material eligibility.

### 2. Purpose
Make missing/current verification material explicit.

### 3. Exact scope of code changes
Apply configured temporal/trust-root/revocation requirements and bounded material retrieval only through authorized acquisition; distinguish historical verification from current acceptance.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Expired/missing/wrong-time material and denied network retrieval yield explicit non-acceptance; frozen historical fixtures reproduce.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Trust material has explicit validity and no hidden network fallback.

### 8. Commit message
```text
feat: validate verification material eligibility
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s126"></a>
## S126 — Verify every mandatory input type

**Predecessor:** S125. **Contracts:** R23, R24, R25, R26, R28, R29, R38, R51, R59, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC028, AC029, AC038, AC051, AC059, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Verify every mandatory input type.

### 2. Purpose
Close gaps where absent record/runtime verification disappears.

### 3. Exact scope of code changes
Apply the checked verification path to evidence, candidate/release records and runtime observations; build a complete required verification set without filter_map loss.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Missing record or runtime receipt blocks despite signed evidence; unrelated optional rejected inputs remain distinct from required gaps.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Every required input has an explicit acceptance conclusion.

### 8. Commit message
```text
fix: require complete input verification
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s127"></a>
## S127 — Persist checked verification provenance

**Predecessor:** S126. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Persist checked verification provenance.

### 2. Purpose
Connect actual verifier receipts to durable objects.

### 3. Exact scope of code changes
Atomically associate real checked results/material with exact raw/semantic objects and authority/evaluator context; import still revalidates the needed boundary.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Reopen/reimport preserves verification provenance without trusting caller-set flags; altered receipt/material rejects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Durable verification supports reproducible audit without minting authority.

### 8. Commit message
```text
feat: persist checked verification provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s128"></a>
## S128 — Enforce protected waiver approvals

**Predecessor:** S127. **Contracts:** R19, R23, R24, R25, R26, R29, R38, R51, R58, R64. **Acceptance:** AC019, AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC058, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enforce protected waiver approvals.

### 2. Purpose
Keep waiver authority outside candidate/agent control.

### 3. Exact scope of code changes
Load and validate waiver approval/scope/time provenance through protected configuration; supply only eligible applications to the pure evaluator.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/waivers.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Candidate-created/widened/renewed waivers cannot self-approve; expired/unrelated approvals reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Conditional outcomes require explicit protected exception authority.

### 8. Commit message
```text
feat: verify protected waiver authority
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s129"></a>
## S129 — Disable synthetic trust in production surfaces

**Predecessor:** S128. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Disable synthetic trust in production surfaces.

### 2. Purpose
Ensure test helpers cannot bypass the real pipeline.

### 3. Exact scope of code changes
Remove or isolate production-reachable synthetic SignedCi factories/flags; keep explicit unconfigured development state without representing it as verified release trust.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_runner/src/ci.rs`
- `crates/eqm_application/src/trust/`
- `Cargo.toml`

### 5. Unit tests or integration tests required
Production API/feature tests cannot construct unconditional signed trust; test fixtures remain usable only in test boundaries.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Trusted release facts originate only from the configured checked path.

### 8. Commit message
```text
fix: exclude synthetic trust from production paths
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s130"></a>
## S130 — Add the cryptographic negative acceptance suite

**Predecessor:** S129. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Add the cryptographic negative acceptance suite.

### 2. Purpose
Pressure-test trust across all attack dimensions.

### 3. Exact scope of code changes
Combine exact positive signatures with wrong identity/issuer/builder/subject/attempt/replay/material/config/evaluator fixtures and persistent import cases.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
Actual verifier tests pass; unsupported live identity/network lanes remain explicitly unconfigured, not marked verified.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Trust assurance is backed by cryptographic and authorization regressions.

### 8. Commit message
```text
test: cover conformance verification attack cases
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s131"></a>
## S131 — Document trust activation and rotation

**Predecessor:** S130. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Document trust activation and rotation.

### 2. Purpose
Prepare operators without granting live signer permissions.

### 3. Exact scope of code changes
Add validated config examples using non-production placeholders, key/material rotation and historical/current acceptance procedures; record Q04/Q15 live blockers.

### 4. Files/modules likely involved
- `docs/evidence-and-trust.md`
- `docs/eqm_product_v1/runbooks/TRUST_OPERATIONS.md`
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`

### 5. Unit tests or integration tests required
Fixture config validation and offline rotation/revocation rehearsal pass; no real credentials/signers are embedded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask test security
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Production activation is explicit and operationally reviewable.

### 8. Commit message
```text
docs: define verified trust activation procedures
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s132"></a>
## S132 — Assemble frozen input manifests

**Predecessor:** S131. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Assemble frozen input manifests.

### 2. Purpose
Connect coherent storage authority and subjects to evaluation.

### 3. Exact scope of code changes
Capture exact authority, universe, campaigns, subject manifest, evidence/runtime objects, verification material, evaluator and explicit time from one consistent generation.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Concurrent imports/config changes cannot mix inputs; missing referenced objects prevent successful preparation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
A decision can be reproduced from explicit immutable inputs.

### 8. Commit message
```text
feat: assemble frozen conformance input manifests
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s133"></a>
## S133 — Validate prepared evaluation construction

**Predecessor:** S132. **Contracts:** R04, R05, R11, R22, R24, R26, R38, R44, R45, R46, R50, R63. **Acceptance:** AC004, AC005, AC011, AC022, AC024, AC026, AC038, AC044, AC045, AC046, AC050, AC063.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate prepared evaluation construction.

### 2. Purpose
Prevent partial or unverified maps entering the main evaluator.

### 3. Exact scope of code changes
Combine validated completeness with checked input conversions and mandatory verification facts in a private application preparation boundary; no serde/public boolean bypass.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Missing provider/profile/receipt and forged prepared-input fields cannot produce a trusted complete decision.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Prepared inputs prove accounting completeness without implying success.

### 8. Commit message
```text
feat: validate prepared conformance inputs
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s134"></a>
## S134 — Run the canonical evaluator through application

**Predecessor:** S133. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Run the canonical evaluator through application.

### 2. Purpose
Expose one whole-evaluation operation to adapters.

### 3. Exact scope of code changes
Compose the reference semantics into the application evaluate operation with no new policy logic; return structured results and safe errors for explicit requests.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Valid, missing, stale, failed, unstable, waived and wrong-subject fixtures match reference outcomes; no acquisition spies fire.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
All adapters can invoke one normative evaluation path.

### 8. Commit message
```text
feat: expose shared conformance evaluation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s135"></a>
## S135 — Produce immutable decision snapshots

**Predecessor:** S134. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Produce immutable decision snapshots.

### 2. Purpose
Define stable exact decision identity and complete explanations.

### 3. Exact scope of code changes
Map evaluation outputs to immutable snapshot records with full scope/input/evaluator/time identities, accepted/rejected reasons, applied waivers and aggregate conclusions.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Identical inputs/time reproduce the same semantic decision; changed inputs/time/context are explicit rather than overwritten.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Decisions are replayable immutable records instead of mutable dashboard state.

### 8. Commit message
```text
feat: produce immutable evaluation snapshots
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s136"></a>
## S136 — Separate read-only and writable session access

**Predecessor:** S135. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Separate read-only and writable session access.

### 2. Purpose
Keep inspection from creating generated state or running tests.

### 3. Exact scope of code changes
Provide read-only store/session access that reports absent evidence without initializing DB/directories; explicit acquisition/publication operations retain write permission.

### 4. Files/modules likely involved
- `crates/eqm_application/src/session.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
check/query preparation on a missing .eqm tree does not create it; read-only callers cannot publish or acquire.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Query/evaluation use cases preserve declared side-effect boundaries.

### 8. Commit message
```text
fix: preserve read-only evaluation sessions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s137"></a>
## S137 — Persist decision manifests explicitly

**Predecessor:** S136. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Persist decision manifests explicitly.

### 2. Purpose
Make archived decisions durable without implicit query writes.

### 3. Exact scope of code changes
Add an explicitly authorized publication operation for input/decision manifests using object-first transactions and idempotent IDs; leave ordinary read-only queries non-mutating.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Crash/reopen and duplicate publication retain exact decision references; failed persistence cannot appear as archived success.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Historical decisions are durable only through a visible publication boundary.

### 8. Commit message
```text
feat: archive exact evaluation decisions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s138"></a>
## S138 — Replay historical snapshots

**Predecessor:** S137. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Replay historical snapshots.

### 2. Purpose
Reproduce recorded decisions without present-day reinterpretation.

### 3. Exact scope of code changes
Load exact archived inputs/evaluator/context and reproduce historical results; missing original material/unsupported semantics returns explicit limitations.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
A decision replays after present time/trust changes using original context; unsupported evaluator does not silently migrate acceptance.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Historical audit is reproducible and separate from current permission.

### 8. Commit message
```text
feat: replay historical conformance decisions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s139"></a>
## S139 — Evaluate current acceptance separately

**Predecessor:** S138. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Evaluate current acceptance separately.

### 2. Purpose
Prevent permanent permission from old signed or cached results.

### 3. Exact scope of code changes
Add a pure/current-context check over recorded decisions with present authorized freshness/trust/deployment criteria, without mutating the historic snapshot.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/freshness.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Expiry/revocation/policy changes reject current acceptance while original history remains unchanged.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Current release use must reestablish relevant eligibility.

### 8. Commit message
```text
feat: separate current decision acceptance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s140"></a>
## S140 — Provide shared explanation and query models

**Predecessor:** S139. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Provide shared explanation and query models.

### 2. Purpose
Replace command-specific interpretation with snapshot views.

### 3. Exact scope of code changes
Build query mapping for scope, obligations, reasons, evidence rejection, inventory/exposure, waivers and summaries; retain exact source/provenance information.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Query variants reference the same snapshot and preserve all blockers without implicit recalculation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI/MCP/native consumers get one coherent explanation surface.

### 8. Commit message
```text
feat: expose coherent evaluation explanations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s141"></a>
## S141 — Bind pagination to snapshots and filters

**Predecessor:** S140. **Contracts:** R04, R05, R11, R22, R38, R42, R44, R45, R46, R47, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC042, AC044, AC045, AC046, AC047, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Bind pagination to snapshots and filters.

### 2. Purpose
Keep large views coherent under updates.

### 3. Exact scope of code changes
Add bounded query pagination/cursor validation with snapshot/filter/order identity and explicit truncation metadata; avoid unauthorized cross-context cursor reuse.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Page continuation across changed snapshot/filter rejects; partial display does not alter completeness/authorization.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Large reports remain bounded and truthful.

### 8. Commit message
```text
feat: bind query pagination to evaluation snapshots
```

**Approved review regression contributions:** RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s142"></a>
## S142 — Define safe snapshot change notifications

**Predecessor:** S141. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Define safe snapshot change notifications.

### 2. Purpose
Support incremental consumers without hidden daemons.

### 3. Exact scope of code changes
Expose a bounded in-process/application notification or polling contract with snapshot generation identity and backpressure; no default network server.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_application/tests/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Slow/canceled consumers do not leak tasks or conflate generations; missed deltas trigger explicit resync.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Optional live consumers can refresh coherently without per-row reevaluation.

### 8. Commit message
```text
feat: expose bounded snapshot change signals
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s143"></a>
## S143 — Test end-to-end snapshot coherence

**Predecessor:** S142. **Contracts:** R04, R05, R11, R21, R22, R38, R44, R45, R46, R50, R52. **Acceptance:** AC004, AC005, AC011, AC021, AC022, AC038, AC044, AC045, AC046, AC050, AC052.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Test end-to-end snapshot coherence.

### 2. Purpose
Close application-service wiring before switching public commands.

### 3. Exact scope of code changes
Execute synthetic acquire/import-to-store-to-evaluate-to-query/replay flows using real checked fixture boundaries and expected non-success mutations.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
AC004/AC005/AC011/AC022/AC038/AC044/AC046 cases pass with no side-effect or trust shortcuts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
The shared application pipeline is independently verified before UI/CLI adoption.

### 8. Commit message
```text
test: verify the shared evaluation lifecycle
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s144"></a>
## S144 — Route obligations through shared scope preparation

**Predecessor:** S143. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Route obligations through shared scope preparation.

### 2. Purpose
Remove CLI-specific policy derivation copies.

### 3. Exact scope of code changes
Make obligations use exact application selection/universe/query models, including unknown/not-applicable/provider/scenario coordinates and complete IDs.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/obligations.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
CLI JSON fixtures include complete scope and duplicate-profile/ambiguous-selection errors.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Obligation inspection uses the same authority and identities as evaluation.

### 8. Commit message
```text
refactor: route obligation queries through application
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s145"></a>
## S145 — Route check through stored-evidence evaluation

**Predecessor:** S144. **Contracts:** R04, R11, R12, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC011, AC012, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Route check through stored-evidence evaluation.

### 2. Purpose
Replace unconditional missing findings.

### 3. Exact scope of code changes
Load bounded prepared evidence through read-only application evaluation and render actual facet/structure findings; no runner execution or implicit state creation.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/check.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_cli tests`

### 5. Unit tests or integration tests required
Persist passing evidence then stale/failed/untrusted variants; check reflects each and remains non-executing.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
The intended verify/import-to-check lifecycle becomes functional.

### 8. Commit message
```text
fix: evaluate stored evidence in conformance checks
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s146"></a>
## S146 — Render conformance and evidence matrices from snapshots

**Predecessor:** S145. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Render conformance and evidence matrices from snapshots.

### 2. Purpose
Remove hardcoded missing matrix values.

### 3. Exact scope of code changes
Map complete snapshot decisions to conformance/evidence cells with stable axes, obligation references, reasons and presentation-only filters.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_engine/src/matrix.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
Passing/stale/failed/unknown/mixed cells match check and complete universe; filter does not recompute assurance.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Matrix content reflects evaluated evidence rather than placeholders.

### 8. Commit message
```text
fix: render evaluated conformance matrices
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s147"></a>
## S147 — Render exposure release and comparison matrices

**Predecessor:** S146. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Render exposure release and comparison matrices.

### 2. Purpose
Complete remaining matrix kinds using real evaluated inputs.

### 3. Exact scope of code changes
Populate exposure/release/equivalence views from appropriate snapshot facts and conclusions, with unavailable states where data genuinely lacks verification.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_application/src/query.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Runtime mismatch and missing end-to-end/cohort cases agree with engine; no presence-only exposure match.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Every supported matrix view has a real semantic source.

### 8. Commit message
```text
feat: render complete assurance matrix views
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s148"></a>
## S148 — Route context and explanation through snapshots

**Predecessor:** S147. **Contracts:** R04, R30, R40, R42, R44, R45, R46, R47, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC042, AC044, AC045, AC046, AC047, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Route context and explanation through snapshots.

### 2. Purpose
Replace empty or fragmented evidence/findings context.

### 3. Exact scope of code changes
Populate context/explain with approved authority, exact obligations, evidence acceptance/rejection and contributing waivers; keep product text labeled untrusted and bounded.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/context.rs`
- `crates/eqm_cli/src/commands/explain.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
Context matches check/matrix and respects byte budgets without claiming truncated scope is complete.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Human/agent investigation gets actionable consistent reasons.

### 8. Commit message
```text
feat: expose evaluated context and explanations
```

**Approved review regression contributions:** RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s149"></a>
## S149 — Render concrete verify dry-run plans

**Predecessor:** S148. **Contracts:** R04, R11, R12, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC011, AC012, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Render concrete verify dry-run plans.

### 2. Purpose
Fulfill the execution-inspection contract publicly.

### 3. Exact scope of code changes
Serialize the shared fully resolved plan in human/JSON output with all safe executable/argv/environment-reference/limit/output details and no secret values.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_cli/src/renderer.rs`

### 5. Unit tests or integration tests required
Dry-run includes exact planned work, invokes no programs, rejects unresolved selectors/programs and never outputs secret bytes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Users can inspect actual execution before authorizing it.

### 8. Commit message
```text
fix: render complete verification dry-run plans
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s150"></a>
## S150 — Execute verify through authorized acquisition

**Predecessor:** S149. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Execute verify through authorized acquisition.

### 2. Purpose
Remove ad hoc runner authority and unbounded result reads.

### 3. Exact scope of code changes
Route verify execution to application approval/campaign/supervision/persistence path, retaining explicit cancellation and typed execution outcomes.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/`

### 5. Unit tests or integration tests required
Local fixture execution persists exact receipts/evidence; unsupported guarantees/secret absence fail visibly; canceled work is retained correctly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI execution uses the same bounded independently authorized acquisition boundary.

### 8. Commit message
```text
refactor: route verification through authorized acquisition
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s151"></a>
## S151 — Separate acquisition summaries from conformance

**Predecessor:** S150. **Contracts:** R04, R11, R12, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC011, AC012, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Separate acquisition summaries from conformance.

### 2. Purpose
Fix empty-plan and low-minimum success ambiguity.

### 3. Exact scope of code changes
Report selected/acquired outcome distinctly from evaluated policy result; reject accidental empty selection under documented semantics and never claim policy success from exit zero.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_protocol/src/`
- `docs/cli.md`

### 5. Unit tests or integration tests required
Empty plan, inadequate policy minimum and successful runner with missing expected case remain nonconformant/explicit failure.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
verify output cannot mislead callers about complete product assurance.

### 8. Commit message
```text
fix: distinguish acquisition from policy conformance
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s152"></a>
## S152 — Emit structured CLI failure envelopes

**Predecessor:** S151. **Contracts:** R04, R05, R21, R30, R40, R44, R45, R46, R52, R58, R59, R60. **Acceptance:** AC004, AC005, AC021, AC030, AC040, AC044, AC045, AC046, AC052, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Emit structured CLI failure envelopes.

### 2. Purpose
Preserve requested machine output on command failures.

### 3. Exact scope of code changes
Map application/parser/runner/storage/trust errors to stable safe machine envelopes and documented exit classes. Validate command/output-format compatibility before acquisition/storage; implement renderers for accepted formats or reject them during preflight. Preserve discoverable durable receipts and truthful errors if output fails after commit.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/main.rs`
- `crates/eqm_cli/src/renderer.rs`
- `crates/eqm_protocol/src/`
- `docs/cli.md`

### 5. Unit tests or integration tests required
Validate actual success/error envelopes against generated schemas for every command family; test all accepted format pairs, early usage/format rejection with effect spies, and output failure after durable receipt publication.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Machine consumers no longer receive only generic stderr stage failures.

### 8. Commit message
```text
fix: emit structured command failure envelopes
```

**Approved review regression contributions:** RA14, RA20.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s153"></a>
## S153 — Align discovery and reconciliation output

**Predecessor:** S152. **Contracts:** R04, R24, R28, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC024, AC028, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Align discovery and reconciliation output.

### 2. Purpose
Use independent inventory observations and shared evaluation facts.

### 3. Exact scope of code changes
Route static/executable discovery permission distinctions and reconciliation status into coherent DTOs; do not rewrite authored expectations from observations.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/discover.rs`
- `crates/eqm_cli/src/commands/reconcile.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Partial/untrusted inventory remains unknown, approved executable discovery uses acquisition, and reconciliation is non-mutating.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Observed state and intended authority stay separate and visible.

### 8. Commit message
```text
refactor: align discovery and reconciliation contracts
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s154"></a>
## S154 — Align inspect and diagnostic commands

**Predecessor:** S153. **Contracts:** R04, R05, R30, R40, R44, R45, R46, R48, R53, R58, R59, R60. **Acceptance:** AC004, AC005, AC030, AC040, AC044, AC045, AC046, AC048, AC053, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Align inspect and diagnostic commands.

### 2. Purpose
Preserve accurate provenance and root/schema/capability errors.

### 3. Exact scope of code changes
Update show/locate/doctor/validate to use shared prepared types/errors. Separate consumer prerequisites from EQM contributor toolchain checks and recognize valid generated store/work states. Keep read commands free of accidental acquisition and honor format preflight.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/show.rs`
- `crates/eqm_cli/src/commands/locate.rs`
- `crates/eqm_cli/src/commands/doctor.rs`
- `crates/eqm_cli/src/commands/validate.rs`

### 5. Unit tests or integration tests required
Run doctor on an unmodified non-Rust consumer without copied EQM Cargo/toolchain files, after supported generated-state creation, and with invalid store entries; test accepted output formats and read-only effect boundaries.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Supporting commands remain useful after shared-layer extraction.

### 8. Commit message
```text
refactor: align inspection commands with prepared context
```

**Approved review regression contributions:** RA19, RA20.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s155"></a>
## S155 — Add CLI lifecycle golden tests

**Predecessor:** S154. **Contracts:** R04, R05, R11, R12, R21, R30, R40, R44, R45, R46, R48, R52, R53, R58, R59, R60. **Acceptance:** AC004, AC005, AC011, AC012, AC021, AC030, AC040, AC044, AC045, AC046, AC048, AC052, AC053, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Add CLI lifecycle golden tests.

### 2. Purpose
Replace tests that merely assert missing or nonempty placeholder output.

### 3. Exact scope of code changes
Test actual passing and negative evidence lifecycle across obligations/check/matrix/context/verify and machine errors with deterministic time/snapshot inputs.

### 4. Files/modules likely involved
- `crates/eqm_cli tests`
- `tests/fixtures/`
- `crates/eqm_test_support/src/`

### 5. Unit tests or integration tests required
Views agree for the same snapshot; stale/trust/failed/missing changes are reflected and no .eqm writes occur on read-only paths.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI coverage exercises the promised product workflow rather than placeholder shape.

### 8. Commit message
```text
test: verify the full cli evidence lifecycle
```

**Approved review regression contributions:** RA01, RA14, RA19, RA20.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s156"></a>
## S156 — Remove superseded CLI evaluation helpers

**Predecessor:** S155. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Remove superseded CLI evaluation helpers.

### 2. Purpose
Eliminate duplicate truth calculators after adoption tests.

### 3. Exact scope of code changes
Delete unused status/count/freshness/profile/digest implementations now replaced by application/engine; retain only command orchestration and rendering.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_cli/src/commands/check.rs`
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_cli/src/commands/verify.rs`

### 5. Unit tests or integration tests required
Golden lifecycle tests remain unchanged; dependency/lint scan finds no new divergent evaluator.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI is thin and semantic maintenance has one authoritative implementation.

### 8. Commit message
```text
refactor: remove duplicate cli evaluation logic
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s157"></a>
## S157 — Update CLI usage and safe adoption examples

**Predecessor:** S156. **Contracts:** R04, R30, R40, R44, R45, R46, R48, R53, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC048, AC053, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Update CLI usage and safe adoption examples.

### 2. Purpose
Make docs reflect implemented behavior and exact command grammar.

### 3. Exact scope of code changes
Update docs/cli.md, getting started, evidence/trust and existing examples for actual invocation/exit/dry-run/selection behavior; do not invent product tasks.

### 4. Files/modules likely involved
- `docs/cli.md`
- `docs/getting-started.md`
- `docs/evidence-and-trust.md`
- `examples/`

### 5. Unit tests or integration tests required
Run documented example validation and safe smoke commands discovered from the repo; examples pass without live credentials or implicit network.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
Public instructions no longer promise placeholder or unimplemented behavior.

### 8. Commit message
```text
docs: document the completed evaluation workflow
```

**Approved review regression contributions:** RA19.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s158"></a>
## S158 — Share application snapshots with MCP sessions

**Predecessor:** S157. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Share application snapshots with MCP sessions.

### 2. Purpose
Remove CLI coupling without reloading/reinterpreting evidence.

### 3. Exact scope of code changes
Adapt MCP session ownership to shared application snapshot/query services while preserving transport lifecycle and no dependency cycle.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/session.rs`
- `crates/eqm_cli/src/commands/mcp.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
Session tests borrow/reference one immutable snapshot and do not implicitly reacquire data or create state.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP can expose the same prepared decision as the CLI.

### 8. Commit message
```text
refactor: connect mcp to shared evaluation snapshots
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s159"></a>
## S159 — Populate MCP context and findings resources

**Predecessor:** S158. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Populate MCP context and findings resources.

### 2. Purpose
Replace empty placeholder resource records.

### 3. Exact scope of code changes
Render obligations/evidence/rejections/findings/applied waivers from shared query models with actual completeness/trust/truncation metadata.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/resources.rs`
- `crates/eqm_application/src/query.rs`
- `crates/eqm_mcp tests`

### 5. Unit tests or integration tests required
Resource results agree with check/context for passing and negative fixtures; empty findings do not imply evaluated completeness unless established.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Agent resources contain real evaluated context.

### 8. Commit message
```text
fix: populate evaluated mcp resources
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s160"></a>
## S160 — Route MCP tools through shared read operations

**Predecessor:** S159. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Route MCP tools through shared read operations.

### 2. Purpose
Avoid transport-specific status calculation.

### 3. Exact scope of code changes
Map supported inspection/query tools to shared application use cases with exact request validation; retain read-only defaults.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/tools.rs`
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp tests`

### 5. Unit tests or integration tests required
Tool/resource/CLI outputs reference the same decision and unauthorized execution-shaped requests reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP tools remain thin coherent readers.

### 8. Commit message
```text
refactor: route mcp tools through shared queries
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s161"></a>
## S161 — Enforce MCP permission and untrusted-data boundaries

**Predecessor:** S160. **Contracts:** R04, R05, R44, R45, R46, R47, R50. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md).

### 1. Step title
Enforce MCP permission and untrusted-data boundaries.

### 2. Purpose
Prevent text or read authority from becoming execution permission.

### 3. Exact scope of code changes
Add explicit operation admission checks and safe labeling for product descriptions/logs/attachments; ensure no signer/waiver/acquisition capability is exposed by default.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp/src/tools.rs`
- `crates/eqm_mcp tests`

### 5. Unit tests or integration tests required
Inject untrusted textual instructions and privilege-escalation requests; no execution/signing/mutation occurs.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Read-only agent access cannot grant itself stronger capabilities.

### 8. Commit message
```text
fix: enforce mcp operation authority boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s162"></a>
## S162 — Bound MCP requests pagination and lifecycle

**Predecessor:** S161. **Contracts:** R04, R05, R42, R44, R45, R46, R47, R50. **Acceptance:** AC004, AC005, AC042, AC044, AC045, AC046, AC047, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Bound MCP requests pagination and lifecycle.

### 2. Purpose
Keep agent access resource-safe and generation-correct.

### 3. Exact scope of code changes
Apply input/output/queue budgets, snapshot-bound pagination, cancellation and explicit unsupported transport behavior; do not add a default HTTP daemon.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp/src/resources.rs`
- `crates/eqm_cli/src/commands/mcp.rs`

### 5. Unit tests or integration tests required
Oversized/deep requests, stale cursors, slow clients and shutdown cannot leak work or falsely complete a truncated view.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP lifecycle and views are bounded and truthful.

### 8. Commit message
```text
fix: bound mcp request and snapshot lifecycles
```

**Approved review regression contributions:** RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s163"></a>
## S163 — Map structured errors into MCP responses

**Predecessor:** S162. **Contracts:** R04, R05, R21, R44, R45, R46, R47, R50, R52. **Acceptance:** AC004, AC005, AC021, AC044, AC045, AC046, AC047, AC050, AC052.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Map structured errors into MCP responses.

### 2. Purpose
Preserve causes without transport divergence or secret leakage.

### 3. Exact scope of code changes
Translate shared typed errors into supported protocol error/result shapes and stable diagnostics; retain request identity and context safely.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp/src/tools.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Invalid authority/query/resource and unavailable snapshot fixtures return valid protocol errors matching CLI diagnostic semantics.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP errors are actionable without implementing another evaluator.

### 8. Commit message
```text
fix: preserve structured mcp diagnostics
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s164"></a>
## S164 — Verify and document secure MCP integration

**Predecessor:** S163. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Verify and document secure MCP integration.

### 2. Purpose
Establish parity and least-privilege evidence.

### 3. Exact scope of code changes
Add stdio/restricted-transport smoke/security tests and update documentation with actual supported methods, permissions, limits and snapshot semantics.

### 4. Files/modules likely involved
- `crates/eqm_mcp tests`
- `crates/eqm_cli/src/commands/mcp.rs`
- `docs/mcp.md`
- `docs/agent-context.md`

### 5. Unit tests or integration tests required
Cross-interface fixture suite and unauthorized-operation tests pass; no unverified public server support claim is added.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask test security
```

### 7. Expected result
MCP is usable as a coherent read-only agent interface.

### 8. Commit message
```text
test: verify secure mcp snapshot access
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s165"></a>
## S165 — Route release selection through protected preparation

**Predecessor:** S164. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Route release selection through protected preparation.

### 2. Purpose
Close the development-authority bypass.

### 3. Exact scope of code changes
Make release requests use exact protected authority/profile values and candidate subjects from application preparation, never the development derive helper/defaults.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/selection.rs`

### 5. Unit tests or integration tests required
Candidate-local policy/defaults and absent protected baseline reject; exact authorized fixture selection succeeds.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release evaluation cannot select its own weaker development authority.

### 8. Commit message
```text
fix: use protected authority for release evaluation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s166"></a>
## S166 — Evaluate releases from complete snapshots

**Predecessor:** S165. **Contracts:** R13, R19, R23, R24, R27, R28, R29, R38, R44, R52, R54, R58, R63. **Acceptance:** AC013, AC019, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC058, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate releases from complete snapshots.

### 2. Purpose
Remove target-only and duplicate outcome logic.

### 3. Exact scope of code changes
Route release checking through the shared complete decision including provider/scenario/unknown coordinates, canonical counts/retries/trust and applied waivers.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/release.rs`

### 5. Unit tests or integration tests required
Target passes with missing provider/end-to-end or failed manual review cannot pass; canonical facet results match check/matrix.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release conformance uses the same normative semantics and full denominator.

### 8. Commit message
```text
fix: evaluate complete release conformance snapshots
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s167"></a>
## S167 — Compare real release exposure facts

**Predecessor:** S166. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Compare real release exposure facts.

### 2. Purpose
Replace runtime-document presence with semantic match.

### 3. Exact scope of code changes
Use exact verified runtime observations and authored expectations to derive exposure comparisons for the candidate/deployment context; include mismatch reasons.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/exposure.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Present but wrong/stale/untrusted facts remain mismatch/unknown; exact complete verified facts satisfy only declared expectations.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release exposure reflects observed behavior rather than document existence.

### 8. Commit message
```text
fix: compare actual release exposure facts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s168"></a>
## S168 — Separate readiness and post-deployment records

**Predecessor:** S167. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Separate readiness and post-deployment records.

### 2. Purpose
Make the release lifecycle realizable and non-circular.

### 3. Exact scope of code changes
Expose approved candidate-readiness versus deployed-conformance record/use-case distinctions, with exact prior-record references and explicit time/material prerequisites.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/`
- `schemas/`

### 5. Unit tests or integration tests required
Initial readiness works without impossible future observations; post-deploy evaluation requires exact later receipts and rejects cyclic references.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release context can be evaluated before and after deployment honestly.

### 8. Commit message
```text
feat: separate readiness from deployed conformance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s169"></a>
## S169 — Require actual subject verification in release gates

**Predecessor:** S168. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Require actual subject verification in release gates.

### 2. Purpose
Remove self-confirming expected/observed record shortcuts.

### 3. Exact scope of code changes
Supply expected subjects independently from protected manifests and observed verified subjects from inputs; require every mandatory receipt and current acceptance before eligibility.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Wrong artifact/build/channel, missing record/runtime verification and stale acceptance reject despite otherwise signed evidence.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Gate inputs represent actual checked facts, not copied expected values.

### 8. Commit message
```text
fix: require verified exact release subjects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s170"></a>
## S170 — Emit attestations from evaluated snapshots

**Predecessor:** S169. **Contracts:** R13, R19, R23, R24, R27, R28, R29, R38, R44, R52, R54, R58, R63. **Acceptance:** AC013, AC019, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC058, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Emit attestations from evaluated snapshots.

### 2. Purpose
Replace unknown placeholders and ad hoc conformance calculations.

### 3. Exact scope of code changes
Build statements from one exact decision and its input references, actual contributing waivers and per-participant artifact subjects; preserve multi-target identities rather than requiring all subjects identical.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/attest.rs`
- `crates/eqm_application/src/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Passing/failed/unknown statements reproduce snapshot facts; different legitimate subjects are represented correctly and evidence subsets cannot alter the claim.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Attestation reports actual decision provenance without promoting evidence claims.

### 8. Commit message
```text
fix: attest exact evaluated conformance snapshots
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s171"></a>
## S171 — Add explicit signing through approved authority

**Predecessor:** S170. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Add explicit signing through approved authority.

### 2. Purpose
Support real signing without making it a release decision.

### 3. Exact scope of code changes
Invoke the selected reviewed signing provider only with explicit operator authority, bounded inputs and exact decision bytes; unsigned output remains available and accurately labeled.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_cli/src/commands/attest.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Test-only cryptographic signatures verify; missing authority rejects signing; signed failed/unknown statements remain non-authorizing.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Signing is a separate auditable operation rather than implicit trust promotion.

### 8. Commit message
```text
feat: sign decisions with explicit authority
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s172"></a>
## S172 — Model protected release authorization

**Predecessor:** S171. **Contracts:** R11, R13, R23, R24, R26, R27, R28, R29, R38, R44, R50, R52, R54, R63. **Acceptance:** AC011, AC013, AC023, AC024, AC026, AC027, AC028, AC029, AC038, AC044, AC050, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Model protected release authorization.

### 2. Purpose
Define the promotion boundary without building a deployment platform.

### 3. Exact scope of code changes
Add validated authorization request/record for exact decision/artifacts/participants/destination/channel/validity/allowed deviations under protected authority; finalize wire shape under version ADR.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/`
- `schemas/`

### 5. Unit tests or integration tests required
Arbitrary successful DTOs cannot construct authority; missing exact artifact/destination/policy facts reject.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release permission is a distinct exact protected record.

### 8. Commit message
```text
feat: model exact release authorization
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s173"></a>
## S173 — Authorize only currently acceptable decisions

**Predecessor:** S172. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Authorize only currently acceptable decisions.

### 2. Purpose
Keep unknown and unapproved conditional states from promotion.

### 3. Exact scope of code changes
Implement the protected authorization evaluator using complete current acceptance, verified exact subjects and explicit deviation permissions; do not deploy or publish.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Unknown/failure/stale/wrong-authority decisions reject; conditional requires exact approved deviations; real fixture pass authorizes only its configured subject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Signing or historical success alone never grants promotion.

### 8. Commit message
```text
feat: enforce protected release authorization
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s174"></a>
## S174 — Validate authorization at deployment consumption

**Predecessor:** S173. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Validate authorization at deployment consumption.

### 2. Purpose
Bind what is promoted to what was approved.

### 3. Exact scope of code changes
Add a host-neutral validator/receipt contract consumed by an existing deployment integration for exact artifacts/destination/channel/time; no new infrastructure orchestration.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Artifact/destination substitution, expired interval and replay for another deployment reject; exact permitted consumption produces a receipt.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Deployment consumers can enforce exact authorization rather than trusting a generic pass.

### 8. Commit message
```text
feat: validate exact promotion authorization
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s175"></a>
## S175 — Handle transformed artifacts explicitly

**Predecessor:** S174. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Handle transformed artifacts explicitly.

### 2. Purpose
Avoid accepting new bytes merely because the source commit matches.

### 3. Exact scope of code changes
Validate an approved transformation provenance relationship or require reevaluation when deployment artifacts differ; actual transformation policies remain owner-configured.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Same-commit different artifact without provenance rejects; configured verified transformation fixture follows its explicit policy.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Post-evaluation packaging transformations cannot silently bypass subject binding.

### 8. Commit message
```text
fix: verify transformed release artifact provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s176"></a>
## S176 — Default release enforcement to shadow mode

**Predecessor:** S175. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Default release enforcement to shadow mode.

### 2. Purpose
Prevent accidental production activation during implementation.

### 3. Exact scope of code changes
Add explicit operational configuration/admission separating evaluation/shadow reporting from enforcement; production identities/destinations and independent approval remain required.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/release_check.rs`
- `docs/eqm_product_v1/runbooks/RELEASE_OPERATIONS.md`

### 5. Unit tests or integration tests required
Unconfigured environment evaluates honestly but cannot enforce/promote; activation/reversion requires exact protected configuration.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
The completed gate is not automatically enabled against live releases.

### 8. Commit message
```text
feat: gate release enforcement activation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s177"></a>
## S177 — Test real positive and negative release lifecycle

**Predecessor:** S176. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Test real positive and negative release lifecycle.

### 2. Purpose
Prove trusted functionality rather than only fail-closed placeholders.

### 3. Exact scope of code changes
Execute fixture acquisition/import/verification/snapshot/signing/authorization/consumption plus subject/tamper/waiver/scope/expiry failures using real test-only cryptography.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `crates/eqm_cli tests`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Exact configured positive case succeeds through all boundaries; each mandatory negative case remains non-authorizing.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Production-capable release logic is exercised without live deployment or credentials.

### 8. Commit message
```text
test: verify exact release authorization lifecycle
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s178"></a>
## S178 — Remove duplicate release and attestation evaluators

**Predecessor:** S177. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Remove duplicate release and attestation evaluators.

### 2. Purpose
Leave one semantic implementation after parity tests.

### 3. Exact scope of code changes
Delete old count/freshness/status/trust/digest helpers and constant conclusions replaced by the shared pipeline; preserve only presentation and explicit capability wiring.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_cli/src/commands/attest.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`

### 5. Unit tests or integration tests required
Full CLI/MCP/snapshot/release parity and schema gates pass with no duplicated evaluator paths.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
cargo xtask test security
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
All release and statement conclusions are anchored to the canonical decision.

### 8. Commit message
```text
refactor: remove duplicate release decision logic
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s179"></a>
## S179 — Index source and authority reverse dependencies

**Predecessor:** S178. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Index source and authority reverse dependencies.

### 2. Purpose
Support exact conservative affected selection.

### 3. Exact scope of code changes
Build reverse indexes for shared source components, targets, bindings, contracts, profiles, runners, adapters, trust and evaluator context to affected obligations.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/affected.rs`
- `crates/eqm_application/src/`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Shared commonMain changes reach all dependent outputs; authority changes select their dependent obligations deterministically.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Impact analysis uses the actual dependency model rather than one target root heuristic.

### 8. Commit message
```text
feat: index conformance reverse dependencies
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s180"></a>
## S180 — Handle deleted renamed and unclassified paths

**Predecessor:** S179. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Handle deleted renamed and unclassified paths.

### 2. Purpose
Prevent under-selection when source ownership is uncertain.

### 3. Exact scope of code changes
Extend change ingestion/baseline analysis for deletions/renames and conservative fallback on unknown paths or dependency gaps; keep reasons visible.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/affected.rs`
- `crates/eqm_cli/src/commands/affected.rs`
- `crates/eqm_cli/src/commands/diff.rs`

### 5. Unit tests or integration tests required
Deleted bindings/tests and renamed/shared/unmapped paths never shrink required reevaluation unsafely.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Uncertainty widens impacted work rather than dropping it.

### 8. Commit message
```text
fix: preserve conservative affected scope
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s181"></a>
## S181 — Wire verify affected selection to actual plans

**Predecessor:** S180. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Wire verify affected selection to actual plans.

### 2. Purpose
Repair the parsed-but-discarded baseline path.

### 3. Exact scope of code changes
Use exact baseline/impact results to select acquisition work and record conservative fallback and scope; bind the resulting plan to approval.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_engine/src/affected.rs`

### 5. Unit tests or integration tests required
Targeted baseline changes alter planned executions; unknown changes conservatively include required work; missing baseline rejects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
verify --affected performs tested dependency-based selection.

### 8. Commit message
```text
fix: apply affected analysis to verification plans
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s182"></a>
## S182 — Cache parsing and semantic projections

**Predecessor:** S181. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Cache parsing and semantic projections.

### 2. Purpose
Reuse pure stable work without caching trust decisions implicitly.

### 3. Exact scope of code changes
Add bounded stage cache keyed by exact source/canonicalization/schema identities; preserve component projection semantics and explicit invalidation.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Cached and uncached parse/projection results match; input/schema changes invalidate and limits remain enforced.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Stable preparation work can be reused safely.

### 8. Commit message
```text
perf: cache canonical preparation stages
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s183"></a>
## S183 — Cache prepared graph and obligation derivation

**Predecessor:** S182. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Cache prepared graph and obligation derivation.

### 2. Purpose
Avoid full graph reconstruction for unchanged exact contexts.

### 3. Exact scope of code changes
Cache validated graph/universe results with complete authority/profile/source dependency identities and bounded memory; do not cache caller assertions of completeness.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Changed participation/profile/fragment/policy invalidates correct entries; reference outputs match cached preparation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Warm derivation reuses exact validated work.

### 8. Commit message
```text
perf: reuse validated obligation preparation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s184"></a>
## S184 — Cache verification with trust-context binding

**Predecessor:** S183. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Cache verification with trust-context binding.

### 2. Purpose
Avoid repeated cryptography without perpetual trust.

### 3. Exact scope of code changes
Key reusable checked material by exact bytes/verifier/configuration/subject/material identity and validity, retaining separate historical/current semantics.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Trust-root/verifier/subject/material changes or expiry prevent reuse; immutable historical receipts remain available.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Verification caching cannot survive a relevant authority change silently.

### 8. Commit message
```text
perf: cache exact verification contexts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s185"></a>
## S185 — Expire decision cache entries correctly

**Predecessor:** S184. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Expire decision cache entries correctly.

### 2. Purpose
Prevent stale passes across time and authority transitions.

### 3. Exact scope of code changes
Add decision reuse eligibility with explicit validity boundaries and event invalidation; invalidate on relevant input/campaign/waiver/trust/evaluator changes.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/freshness.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Boundary-time and revocation/waiver/campaign changes reject cached acceptance; exact unchanged eligible input may reuse.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Current acceptance is never extended by cache lifetime.

### 8. Commit message
```text
fix: bound cached decision validity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s186"></a>
## S186 — Batch compatible native test work

**Predecessor:** S185. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Batch compatible native test work.

### 2. Purpose
Reduce process startup without changing observation counts.

### 3. Exact scope of code changes
Group approved cases by compatible runner/target/device/selection context while preserving exact manifest coverage and individual case/attempt lineage.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Batched/unbatched outcomes match; shared executions do not multiply count credit; incompatible devices or contexts remain separated.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Native startup overhead drops without weakening evidence semantics.

### 8. Commit message
```text
perf: batch compatible evidence acquisition
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s187"></a>
## S187 — Avoid redundant snapshot cloning and serialization

**Predecessor:** S186. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Avoid redundant snapshot cloning and serialization.

### 2. Purpose
Improve query efficiency after correctness is stable.

### 3. Exact scope of code changes
Reuse immutable shared graph/decision data and serialize only requested bounded views; avoid new mutable global caches or cross-snapshot aliases.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_engine/src/matrix.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Filtered/paginated output matches full reference slices; concurrent readers remain isolated and bounded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Large report consumers avoid repeated full graph copies and payloads.

### 8. Commit message
```text
perf: streamline immutable snapshot queries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s188"></a>
## S188 — Build representative end-to-end benchmarks

**Predecessor:** S187. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Build representative end-to-end benchmarks.

### 2. Purpose
Replace placeholder warm-context and incomplete cold timings.

### 3. Exact scope of code changes
Extend benchmark fixtures with real targets/profiles/evidence histories/trust/runtime facts/full matrices and measure complete preparation including canonicalization plus stage metrics.

### 4. Files/modules likely involved
- `tools/benchmarks/src/main.rs`
- `crates/eqm_test_support/src/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Benchmark verifies fixture semantics and required counts before timing; populated context and evidence evaluation are actually executed.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Run the owning benchmark/fixture tests and, when the populated benchmark step is reached, the actual cargo xtask benchmark lane on declared hardware; record measurements and host gaps, not thresholds alone.

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Performance reports measure the implemented assurance workload.

### 8. Commit message
```text
test: benchmark the complete conformance pipeline
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s189"></a>
## S189 — Add reference versus optimized differential tests

**Predecessor:** S188. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Add reference versus optimized differential tests.

### 2. Purpose
Protect semantics under caching batching and indexing.

### 3. Exact scope of code changes
Generate bounded randomized and fixed frozen inputs; compare canonical outcomes/reasons/identities for reference versus optimized paths including cache invalidation and reordered inputs.

### 4. Files/modules likely involved
- `crates/eqm_engine/tests/properties.rs`
- `crates/eqm_application/tests/`
- `tools/benchmarks/`

### 5. Unit tests or integration tests required
Any divergence fails; duplicates/missing/unknown/campaign/trust/expiry cases remain identical.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Optimization is continuously checked against the normative reference.

### 8. Commit message
```text
test: compare optimized and reference evaluation
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s190"></a>
## S190 — Establish measured performance budgets

**Predecessor:** S189. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Establish measured performance budgets.

### 2. Purpose
Avoid unsupported super-optimized claims or arbitrary limits.

### 3. Exact scope of code changes
Record actual supported-host workloads, latency distributions, memory/storage growth and contention; set reviewed budgets with Q07 rather than copying baseline constants.

### 4. Files/modules likely involved
- `tools/benchmarks/src/main.rs`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `docs/eqm_product_v1/specs/STORAGE_AND_PERFORMANCE.md`

### 5. Unit tests or integration tests required
Run benchmark lane on declared hardware and publish actual results plus unavailable-host gaps; correctness gates remain independent.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask benchmark
```

**Required command discovery:**
- Run the owning benchmark/fixture tests and, when the populated benchmark step is reached, the actual cargo xtask benchmark lane on declared hardware; record measurements and host gaps, not thresholds alone.

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Performance objectives are measured and reviewable, not inferred from code constants.

### 8. Commit message
```text
perf: establish measured conformance budgets
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s191"></a>
## S191 — Discover real product targets and native commands

**Predecessor:** S190. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Discover real product targets and native commands.

### 2. Purpose
Avoid invented Gradle/Xcode selectors and unsupported platform promises.

### 3. Exact scope of code changes
Resolve Q01/Q02/Q06/Q12 from authorized repos/owners; record product commits, native tasks, selectors, output artifacts, roles and supported cohorts without editing product code yet.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`
- `docs/integrations/`
- `examples/`

### 5. Unit tests or integration tests required
Read manifests/help and verify discovered commands in disposable authorized fixtures; distinguish synthetic versus live lanes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Real integration has exact known inputs or an explicit blocker instead of fabricated tasks.

### 8. Commit message
```text
docs: record native product integration contracts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s192"></a>
## S192 — Define native result adapter contracts

**Predecessor:** S191. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Define native result adapter contracts.

### 2. Purpose
Preserve expected selection and subject identity across test frameworks.

### 3. Exact scope of code changes
Specify and implement minimal adapters' common normalized result/receipt boundary for native selectors, cases, attempts and artifacts; do not build a new test framework.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/normalized.rs`
- `crates/eqm_protocol/src/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Equivalent Rust/Kotlin native fixture results normalize identically; missing cases and wrong subject/attempt reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Native producers can supply exact conformance evidence independently of UI frameworks.

### 8. Commit message
```text
feat: define native evidence adapter contracts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s193"></a>
## S193 — Integrate the Rust product CLI test adapter

**Predecessor:** S192. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Integrate the Rust product CLI test adapter.

### 2. Purpose
Verify the product CLI without confusing it with EQM's binary.

### 3. Exact scope of code changes
Bind discovered Rust product tests/inspection/export scenarios to exact EQM selectors and normalized results; implement an authorized adapter or fixture harness without inventing product business logic.

### 4. Files/modules likely involved
- `examples/`
- `tests/fixtures/`
- `crates/eqm_runner tests`
- `authorized product Rust test adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Actual fixture/test commands produce bounded subject-bound expected-case results; malformed/missing selection fails.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Rust product CLI evidence participates in the common conformance model.

### 8. Commit message
```text
feat: integrate rust product conformance evidence
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s194"></a>
## S194 — Integrate desktop KMP test evidence

**Predecessor:** S193. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Integrate desktop KMP test evidence.

### 2. Purpose
Connect actual control-tower tests to EQM contracts.

### 3. Exact scope of code changes
Bind discovered desktop tasks/selectors/artifacts and normalize account/task projection/review fixtures; keep business UI and source ownership in the product repo.

### 4. Files/modules likely involved
- `docs/integrations/`
- `examples/`
- `tests/fixtures/`
- `authorized desktop KMP test adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Native desktop lane validates current/stale/conflicting projection fixtures and exact subject/selection mapping; no JVM/native assumption is fabricated.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Desktop conformance evidence comes from real native tests where access exists.

### 8. Commit message
```text
feat: integrate desktop kmp conformance evidence
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s195"></a>
## S195 — Integrate mobile capture and lifecycle evidence

**Predecessor:** S194. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Integrate mobile capture and lifecycle evidence.

### 2. Purpose
Verify edge behavior through actual platform tests.

### 3. Exact scope of code changes
Bind discovered mobile photo/video/voice/annotation/save-restart/permission fixtures and native task selectors; keep media handling out of EQM's engine.

### 4. Files/modules likely involved
- `docs/integrations/`
- `examples/`
- `tests/fixtures/`
- `authorized mobile test adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Supported device/simulator lanes verify acknowledged persistence/interruption and correct normalized expected cases; unsupported platforms stay explicit.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Mobile edge evidence is native and lifecycle-aware rather than a parser-only fixture claim.

### 8. Commit message
```text
feat: integrate mobile edge conformance evidence
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s196"></a>
## S196 — Integrate provider and runtime observations

**Predecessor:** S195. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R24, R28, R30, R48, R49, R55, R59. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC024, AC028, AC030, AC048, AC049, AC055, AC059.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Integrate provider and runtime observations.

### 2. Purpose
Bind shared services and actual exposure to exact subjects.

### 3. Exact scope of code changes
Add discovered provider/deployment observation producers and bounded normalization for required sync/exposure facts; distinguish partial from complete trusted inventory.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/inventory.rs`
- `examples/`
- `tests/fixtures/`
- `authorized provider adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Wrong provider/config/build and partial/stale observations cannot satisfy required dependencies.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Shared providers have independent exact evidence rather than inferred client success.

### 8. Commit message
```text
feat: integrate exact provider runtime evidence
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s197"></a>
## S197 — Implement the cross-runtime lifecycle scenario

**Predecessor:** S196. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Implement the cross-runtime lifecycle scenario.

### 2. Purpose
Verify the approved capture-to-review-to-export chain.

### 3. Exact scope of code changes
Orchestrate existing native test operations under one approved scenario/campaign: offline capture, edit, terminate/restart, reconnect/retry, desktop review and CLI inspect/export; use synthetic account/task data.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `examples/`
- `crates/eqm_application tests`
- `authorized native integration test paths from Q01`

### 5. Unit tests or integration tests required
Intended revision appears exactly once under the correct task with no acknowledged-state loss or cross-account disclosure.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
End-to-end evidence is independently produced for the actual participant set.

### 8. Commit message
```text
test: verify the evidence lifecycle across runtimes
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s198"></a>
## S198 — Add product failure and account-boundary scenarios

**Predecessor:** S197. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Add product failure and account-boundary scenarios.

### 2. Purpose
Exercise failure contracts beyond the happy path.

### 3. Exact scope of code changes
Extend the real scenario with approved account switching, permission revocation, concurrent edits, interrupted uploads and stale projections; use owner-defined conflict semantics only.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `authorized product integration test paths from Q01`
- `docs/eqm_product_v1/specs/ACCEPTANCE_CRITERIA.md`

### 5. Unit tests or integration tests required
Each interruption/security case follows explicit product invariants and produces distinct expected outcomes; no invented conflict algorithm.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Product assurance covers the approved lifecycle failure boundaries.

### 8. Commit message
```text
test: cover cross-runtime failure and account boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s199"></a>
## S199 — Verify approved mixed-version cohorts

**Predecessor:** S198. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Verify approved mixed-version cohorts.

### 2. Purpose
Demonstrate independently released target compatibility.

### 3. Exact scope of code changes
Execute the scenario for the owner-approved old/new client/provider combinations and role occurrences; add explicit relational outcome comparison where required.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `authorized product integration test paths from Q01`
- `examples/`

### 5. Unit tests or integration tests required
Approved combinations meet declared outcomes; undeclared or mismatched cohorts cannot borrow passing evidence.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Compatibility assurance is tied to tested exact cohorts, not a global same-version assumption.

### 8. Commit message
```text
test: verify approved runtime compatibility cohorts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s200"></a>
## S200 — Publish a complete consumer integration example

**Predecessor:** S199. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R44, R48, R49, R53, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC044, AC048, AC049, AC053, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Publish a complete consumer integration example.

### 2. Purpose
Make the intended Rust/KMP adoption reconstructable.

### 3. Exact scope of code changes
Add a small copyable synthetic workspace with shared-source roles, bindings, policies, profiles, native adapter contracts and safe dry-run/check instructions; mark real task substitutions explicitly.

### 4. Files/modules likely involved
- `examples/`
- `docs/integrations/`
- `docs/getting-started.md`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
Example validates and safe fixture smoke tests pass; generated state remains ignored and no fake real-product evidence is committed.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Consumers can adopt one real slice without duplicating intent or confusing business/conformance evidence.

### 8. Commit message
```text
docs: publish the multi-runtime integration example
```

**Approved review regression contributions:** RA19.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s201"></a>
## S201 — Record optional embedding disposition

**Predecessor:** S200. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Record optional embedding disposition.

### 2. Purpose
Preserve the approved default without silently starting bridge work.

### 3. Exact scope of code changes
Record that default EQM deployment is developer/CI; evaluate whether an actual local evaluation feature activates the separate optional sequence; do not add bridges/daemons otherwise.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`
- `docs/eqm_product_v1/decisions/`
- `docs/eqm_product_v1/specs/RUNTIME_INTEGRATION.md`

### 5. Unit tests or integration tests required
Package/dependency review confirms no unactivated native/daemon surface; activation decision references exact use case and matrix if granted.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Mandatory v1 scope is closed independently of optional embedding.

### 8. Commit message
```text
docs: record native embedding activation status
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s202"></a>
## S202 — Complete the supported feature and host matrix

**Predecessor:** S201. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Complete the supported feature and host matrix.

### 2. Purpose
Make packaging capability claims explicit.

### 3. Exact scope of code changes
Inspect actual Cargo features and supported targets, add tests for every supported combination and ensure optional capabilities never disable mandatory validation; record unsupported hosts deliberately.

### 4. Files/modules likely involved
- `Cargo.toml`
- `crates/*/Cargo.toml`
- `tools/xtask/src/main.rs`
- `docs/eqm_product_v1/implementation/VERIFICATION.md`

### 5. Unit tests or integration tests required
Discover and run valid no-default/default/feature combinations plus native host lanes; no blanket all-features command for incompatible combinations.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Build and support claims match tested actual configurations.

### 8. Commit message
```text
test: verify supported feature and host combinations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s203"></a>
## S203 — Expand fuzz and mutation coverage to new boundaries

**Predecessor:** S202. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Expand fuzz and mutation coverage to new boundaries.

### 2. Purpose
Test the newly centralized critical paths rather than only old helpers.

### 3. Exact scope of code changes
Extend fuzz corpus/targets and focused mutation coverage for strict parsing, scope completeness, evidence conflicts, trust acceptance, authorization and storage records while retaining bounds.

### 4. Files/modules likely involved
- `tools/fuzz/`
- `tools/xtask/src/main.rs`
- `crates/eqm_engine/tests/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Bounded fuzz and mutation lanes pass with meaningful seeded valid/invalid inputs; surviving critical mutants are investigated, not ignored by threshold changes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Extended verification targets the actual production decision path.

### 8. Commit message
```text
test: harden critical conformance verification lanes
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s204"></a>
## S204 — Verify schema API and migration compatibility

**Predecessor:** S203. **Contracts:** R21, R43, R44, R45, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC021, AC043, AC044, AC045, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Verify schema API and migration compatibility.

### 2. Purpose
Close explicit version-transition obligations.

### 3. Exact scope of code changes
Run all approved wire/ABI-if-activated/DB compatibility fixtures, original-signed-byte preservation, generator parity and unsupported-version errors; update transition docs with actual evidence.

### 4. Files/modules likely involved
- `crates/eqm_protocol/tests/`
- `crates/eqm_application/tests/`
- `tools/xtask/src/main.rs`
- `docs/eqm_product_v1/specs/COMPATIBILITY.md`

### 5. Unit tests or integration tests required
Old/new supported behavior matches ADR, unsupported versions fail visibly, interrupted migrations recover and signed originals remain intact.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_application -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Compatibility is tested rather than inferred from deserialization success.

### 8. Commit message
```text
test: verify explicit compatibility transitions
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s205"></a>
## S205 — Verify reproducible distribution and supply chain

**Predecessor:** S204. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Verify reproducible distribution and supply chain.

### 2. Purpose
Ensure shipped artifacts match declared dependencies and capabilities.

### 3. Exact scope of code changes
Extend existing dist/package checks for new crate/dependencies/runtime libraries, SPDX/SBOM/provenance inputs and checksums; keep outputs unsigned/local unless separately authorized.

### 4. Files/modules likely involved
- `tools/xtask/src/main.rs`
- `Cargo.lock`
- `deny.toml if present`
- `docs/eqm_product_v1/implementation/VERIFICATION.md`

### 5. Unit tests or integration tests required
Package/reproducibility and current dependency audit/policy lanes pass; inspect actual SQLite/verifier/native library versions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Distribution artifacts have reproducible provenance without implicit publication.

### 8. Commit message
```text
build: verify conformance distribution provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s206"></a>
## S206 — Reconcile public docs and remove transitional code

**Predecessor:** S205. **Contracts:** R43, R44, R48, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC044, AC048, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Reconcile public docs and remove transitional code.

### 2. Purpose
Finish code health without broad cosmetic churn.

### 3. Exact scope of code changes
Remove obsolete helpers/features/placeholder fixtures only after tests prove replacement; update README/public docs/error references and generated schemas; retain approved historical records.

### 4. Files/modules likely involved
- `README.md`
- `docs/`
- `crates/`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
Standard gate, doc tests, examples and dependency/lint scans pass; no hardcoded production missing/unknown-as-final placeholders or synthetic trust bypass remains.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
The repository has one documented implemented architecture without abandoned transition paths.

### 8. Commit message
```text
refactor: finalize the shared conformance implementation
```

**Approved review regression contributions:** RA19.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s207"></a>
## S207 — Rehearse storage trust and release incidents

**Predecessor:** S206. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Rehearse storage trust and release incidents.

### 2. Purpose
Make runbooks executable and owners explicit.

### 3. Exact scope of code changes
Assign owner roles using Q15 and run disposable crash/restore, key/material rotation, stale decision, failed authorization and evaluator rollback drills; record results and limitations.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/runbooks/`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Recovery preserves history and trust boundaries; rollback pauses or uses known-good evaluation, never permissive promotion.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Operational procedures are evidenced and responsibility is explicit.

### 8. Commit message
```text
docs: record conformance operational recovery drills
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s208"></a>
## S208 — Run shadow release comparison

**Predecessor:** S207. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Run shadow release comparison.

### 2. Purpose
Validate decisions independently before enforcement.

### 3. Exact scope of code changes
Integrate host-neutral shadow reporting into approved existing CI/deployment environment; compare representative positive/negative decisions with independently reviewed expected outcomes.

### 4. Files/modules likely involved
- `approved CI integration path discovered in Q09`
- `docs/eqm_product_v1/runbooks/RELEASE_OPERATIONS.md`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
Wrong-subject/weakening/missing-scope/expiry fixtures remain blocked; independent review records discrepancies and no live promotion occurs.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover authorized CI/promotion validation commands and exact protected identities/destinations through Q04/Q09/Q15. Do not execute publication/deployment or invent operational approval as part of this code step.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Gate behavior has operational comparison evidence before activation.

### 8. Commit message
```text
test: verify shadow release decision parity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s209"></a>
## S209 — Complete final acceptance and release-candidate gates

**Predecessor:** S208. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Complete final acceptance and release-candidate gates.

### 2. Purpose
Establish readiness from actual evidence rather than approved design.

### 3. Exact scope of code changes
Map every mandatory R/AC/F item to committed implementation/tests, execute standard/extended/native/product/feature lanes and document unresolved blockers; do not mark optional embedding required.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/TRACEABILITY.md`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
cargo xtask check and verify plus all declared supported lanes pass; no new failures are waived and every unrun lane is explicit.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
cargo xtask verify
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Code/platform readiness is accurately established or explicitly blocked.

### 8. Commit message
```text
test: complete product v1 acceptance verification
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s210"></a>
## S210 — Record enforcement approval or explicit non-activation

**Predecessor:** S209. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Record enforcement approval or explicit non-activation.

### 2. Purpose
Keep operational permission separate from code completion.

### 3. Exact scope of code changes
Have the authorized owner review actual identities/destinations/cohorts/host guarantees and shadow evidence; record enforce-or-remain-shadow disposition without deploying from this coding task.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/runbooks/RELEASE_OPERATIONS.md`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `approved operator config path from Q09`

### 5. Unit tests or integration tests required
Activation record binds exact evaluator/authority/configuration; absent approval leaves enforcement disabled; deployment consumer rechecks exact authorization.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover authorized CI/promotion validation commands and exact protected identities/destinations through Q04/Q09/Q15. Do not execute publication/deployment or invent operational approval as part of this code step.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
No one mistakes a green build or signed report for permission to promote.

### 8. Commit message
```text
docs: record release gate activation disposition
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s211"></a>
## S211 — Finalize the implementation continuation record

**Predecessor:** S210. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Finalize the implementation continuation record.

### 2. Purpose
Leave a reconstructable audited handoff after coding.

### 3. Exact scope of code changes
Update progress ledger, completed step reports, deviations, open decisions and final report with actual commit SHAs/tests/risks; no claimed test runs are inferred or fabricated.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/`
- `docs/eqm_product_v1/summary.txt`

### 5. Unit tests or integration tests required
Traceability/link/integrity checks pass for evolved docs; Git history matches reports and final status separates code/platform/operational completion.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
cargo xtask verify
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Another agent or maintainer can reconstruct exactly what shipped and what remains unverified.

### 8. Commit message
```text
docs: finalize the product v1 implementation record
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
