# Independent review — candidate 7a5b246

Reviewed candidate `7a5b2465d293f482c22f2e8e4271a6b1507915c2` against original
step/spec/acceptance/regression contracts and outstanding review findings.
All ten returned commits retained. This is partial implementation progress;
P04/P05 and full MVP acceptance are not complete. The stop before S052 was a
real unmade representation decision, now resolved in governing DEV-030.

## Accepted and retained progress

- P04-R1 (`e8c62b8`): RF70 durable actual owning count/revision/duration/day/
  runner-limit constructors and text control/length endpoints accepted. Existing
  literal corpus and 196 combinations retained.
- S045 (`ab47f21`): duplicate identical/conflicting profile/dimension requests
  reject; no silent overwrite. Allocated local scope accepted.
- S046 (`cce23e5`): policy/profile authority is indexed by exact revision in real
  manifest loading and explicit selection; missing/ambiguous revisions do not
  become latest. Allocated local scope accepted; shared preparation/docs followup
  remains under S044/RF74.
- S050 (`e9db2ad`): empty release scope lacks implicit authorization, one member
  yields NoMeaningfulComparison. Allocated local scope accepted, protected live
  authority and occurrence-level integration remain later work.
- S051 (`1258d06`): constraints compose only from rules naming the facet. Allocated
  local scope accepted; full strengthening properties remain S057.
- S044 (`d72d87b`), S047 (`57b9170`), S048 (`9c0ab9a`), S049 (`8f01be0`): partial
  contributions retained; original steps remain changes-required. S049's stronger
  inherited fragment/journey risk and post-expansion validation are accepted
  contributions; completeness and construction safety are not closed.
- Final `7a5b246` only corrects the resume frontier; no extra feature acceptance.

Preserve all 43 earlier numbered acceptances; four new numbered steps make47.
No whole AC/RA/Q gate is closed merely by these local subsets or references.

## Material findings

**RF72 — P1, incomplete denominator may authorize.**
`eqm_engine::conformance::CompletePolicyResolution::new` rejects unexpected
coordinates but allows missing ones. `evaluate_complete_target_conformance`
checks omissions only for a matching Target; required Provider/TargetSet and
`RequiredUniverse::unresolved` are not checked. `RequiredUniverse` also exposes
public Default, allowing a forged empty denominator. Isolated actual-engine
probes loaded/resolved/expanded the real example, derived its universe, removed
all TargetSet statuses and still obtained Conformant. A second probe introduced
unknown requirement applicability, supplied satisfied remaining statuses and
still obtained Conformant. Empty Default plus empty statuses also Conformant.
Two passing reproduction tests establish these defects, not feature acceptance.
The report's deferral to S073 cannot close current S049 omission safety.
P04-R2 must enforce complete relevant scope and unresolved non-authorization.

**RF73 — P1, unchecked finalization remains publicly constructible.**
`eqm_domain::graph::FinalizedWorkspaceGraph::from_engine` is public; doc(hidden)
only hides documentation. S049 validates expansion but leaves this bypass intact.
Any caller can promote a locally indexed raw graph without resolution/invariant
validation. Engine-owned private checked construction, raw manifest projection
inputs and actual authorizing caller migration are required by P04-R2.

**RF74 — P2, semantic request preparation remains CLI-owned.**
Application preparation accepts an already resolved PolicyProfileRequest, while
CLI `development_request`, `resolve_policy` and profile resolution still choose
exact authority/membership and ambiguity behavior. This only partially fulfills
S044's reusable request preparation. P04-R3 centralizes it in application and
keeps adapters as syntax parsing; actual usage docs must cover new policy/profile
revision syntax with positive/negative integration controls.

**RF75 — P1, declared coordinate participation is not consumed.**
Required derivation evaluates requirement applicability before fanout and uses
all policy required targets without calling existing `evaluate_participation`.
Its established missing-rule→Unknown and ambiguous-rule→error contracts are
therefore bypassed. Requirement-only unresolved markers also lose exact subject/
profile scope. P04-R2 integrates three-valued coordinate participation; S052 adds
exact provider/scenario dependencies. Explicit false stays visible N/A and unknown
stays non-authorizing. No implicit global inclusion or guessed scenario mapping.

**RF76 — P2, progress/evidence reconciliation.**
Candidate original step entries S044–S051 had null commit SHAs and the supervisor
review still referred to the previous candidate while the active frontier claimed
S052. Counts and partial completion claims are now corrected with actual commits
and independent dispositions. Preserve historical reports, including errors.
Native command results contain failed intermediate compile/test/format/Clippy
attempts (not successful verification). Some repeated output filenames retained
only the final attempt; raw command evidence, where available, remains separate.
Do not assert an overwritten failed log still exists. Future attempts use distinct
filenames and exact commands/exits. No historical red attempt is called a pass.

**RF77 — P3, existing rustdoc link warnings.**
The current documentation build emits a private-item link warning in protocol
`positive_unsigned_schema` and an unresolved `clean` link in application source
observations. Standard check does not treat these as failures. P04-R3 repairs
these documentation links only; do not report a warning-free documentation build
until verified. They predate this implementation batch.

## Decisions and next work

DEV-030 is the sole execution amendment: P04-R2 checked graph/universe/
participation repair, P04-R3 shared selection/docs, then all S052–S076 (27
checkpoints). Exact requirement→ScenarioRef bindings are explicit immutable
preparation authority; provider dependers follow declared providers/dependency
closure, repeated role-bearing actors stay distinct. Frozen expected logical
manifests precede observed results. New map digest is separate from existing
contract/policy/graph roots. Independent full evaluator review precedes S077;
there is no new stop between S058 and S059. Original P05 decisions remain binding.

## Independent verification

Routed independent `cargo xtask check`: exit0,792 successful test executions plus
CLI/MCP/release/three-target integration. Routed owning domain/manifest/protocol/
engine/application doctests: exit0,9 passed. Isolated engine defect probes:
2 passed, reproducing omitted target-set, unresolved applicability and forged
empty-universe authorization. Repository-owned planning admission, structure and
diff gates verify the final planning snapshot; all evidence is retained externally.
No product source/test changes were made in the owning checkout by the supervisor.

No full verify/Linux/coverage/mutation/fuzz/benchmark/package/live product/crypto/
host/signing/operational acceptance is established. Schema bytes and Cargo.lock
unchanged. Q05 unavailable live values do not prevent pure configurable semantics
or synthetic regression implementation. There is no human release-testing gate
now; remaining software and full acceptance gates precede release readiness.
All original 64 R/AC,21 RA,Q01–Q15,S211 remain binding; whole RA02/RA13 stay open.
