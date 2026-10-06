# Independent review — candidate 3265bbe

Reviewed both repair commits, source callers/tests, original requirements and
outstanding DEV-030 findings. P04-R2 (`4d7db37`) and P04-R3 (`3265bbe`) are
retained but partial/changes-required; S052–S076 were not started. The handback reports a
practical runtime/context limit; the native branch ends in a normal assistant stop,
not an observed runtime-overflow error. The limit is not independently established
and the batch endpoint was not reached. No product/hardware blocker was evidenced. No full
MVP, whole AC/RA or cycle acceptance is inferred. Preserve47 earlier numbered
acceptances and all original64R/AC21RAQ01–Q15S211.

## Accepted allocated contributions

RF73: FinalizedWorkspaceGraph now belongs to engine, private construction is
used after invariant validation; raw manifest projection inputs are data and
no manifest→engine production dependency was added. Public from_engine bypass
is removed, with an external construction compile-fail regression. The public
Default universe bypass is removed as well; full denominator binding remains
incomplete under RF78. Fixture builders use genuine engine validation.

RF77: the two rustdoc links were repaired without numeric/source semantic
changes. Native evidence includes a warning-denying documentation lane. Shared
request carriers/resolution and CLI syntax-only delegation are useful retained
RF74 contributions, but the new protected entry path remains incomplete (RF81).

Required target-set omissions and unresolved coordinate handling improve RF72;
no global closure is granted because derivation and scope matching remain unsafe.

## Material defects and decisions

**RF75 remains open — empty registry means implicit inclusion.**
`obligations::derive_derivation` computes participation_active from registry
nonemptiness. `resolved_subjects` then treats None as true. It cannot identify
v1 from emptiness and contradicts DEV-030's no-rule→Unknown in every mode.
A real loaded/checked graph with empty registry and all reported facets satisfied
still returns Conformant. Isolated actual-engine reproduction passed. This was
an unauthorized interpretation; remove the fallback, do not weaken acceptance.

**RF78 — P1, foreign selection shrinks checked graph scope.**
Public derive_required_universe takes SelectedPolicyProfiles from any graph and
uses its borrowed policy without validating the same checked graph. An isolated
probe loads the real two-target example and a foreign same-ID/revision policy
with ios removed; derivation against the original checked graph omits ios, and
Conformant is returned for ios. This is not merely differing release_context;
that existing test does not establish same-graph authority. Applicability context
is independently supplied too. Checked preparation must bind exact graph,
policy/profile definitions/values and authority context before deriving, then
bind that semantic identity to status admission. No engine serializer is needed.

**RF79 — P1, production matcher promotes target evidence.**
`release_check::evidence_coordinate_matches` permits any TargetSet member and
unrestricted bindings for Provider; no complete participant/provider subject
comparison occurs. The new positive fixture adds a web test to satisfy a legacy
end-to-end requirement, not a complete scenario. An isolated direct production-
matcher probe uses genuine fixture evidence and changes its queried key to an
explicit ios+web scope, then Provider: both match the same web target evidence.
This probe establishes a matcher defect; those mutated keys are not claimed to
be a complete accepted graph or a whole release-path test. Source inspection
confirms production callers use this matcher. Unsupported/incomplete non-target
scope stays Missing/Unknown now; S052/S054/S059/S061/S073 integrate full authority.
Do not defer present false-success safety or replace cross-runtime proof with
multiple ordinary target passes.

**RF80 — P2, final required gate fails.**
Supervisor `cargo xtask check` exits1 at rustfmt on
selection_preparation.rs:224 (wrapped PolicySelectionRequest assignment).
Schema parity passed first:22v1/13v2. P04-R2 native standard check passed796
successful executions including end-to-end; P04-R3 expressly skipped the
combined standard gate. Individual components cannot replace the mandated
full gate after final edits. Retain both historical facts; no green-final-tree
claim. Formatting is repaired by the implementation agent, not the supervisor.

**RF81 — P1, new protected entry infers omitted exact authority.**
resolve_selection_request fills absent policy and revisions from unique stored
candidate definitions, regardless of mode. A real application probe with
ProtectedBaseline origin, no policy, profile ID without revision and known
region calls prepare_obligations_from_request in Release successfully. The
older protected tests only exercise prepare_obligations, not this new path.
Development alone may use unique/default lookup; protected unresolved requests
must name explicit exact complete permitted authority. Parsed origin tags do
not prove live trust. Also distinguish NotFound from Ambiguous for zero versus
multiple revisions; current Option-based unique lookup conflates their causes.

RF76 ledger stale next_checkpoint/batch status/formatting and report arithmetic
are reconciled with actual SHAs/dispositions; normalize cosmetic indentation.
New application tests are3, not a claim of4 unique tests. Preserve actual failed
native attempts and unique future logs; no overwritten output is called retained.

## Independent verification and evidence limits

Current standard check: exit1, formatting failure; no full standard pass claimed.
Isolated source probes:2 engine,1 application,1 release matcher passed, exposing
four current defects. Release probe first failed compilation because the review
used a nonexistent ReleaseRecordDto convenience method; corrected to the actual
reader mechanism and passed, with failed attempt retained separately. These are
review-only probes, not shipped product changes or acceptance regressions.
Current routed engine/application/CLI all-target tests: exit0,246 successful
test executions. Engine/application boundary doctests: exit0,3 passed. Package structure/public planning admission/diff
checks validate planning changes only, not the missing product gate.
Native evidence: P04-R2 standard796 plus integration; full-workspace test logs
789 then792 successful executions; workspace doctests13 each. Command/result
records retain intermediate failures, including compilation/format/Clippy and
host temporary-volume pressure. They are not relabeled passes or hardware gates.

## Next substantial batch

DEV-032 governs P04-R4 then every S052–S076 (26 green checkpoints). Close explicit
same-graph scope/request admission and formatting first, then exact scenario and
expected selection decisions already frozen under DEV-030. Continue across P04/
P05; independent full reference evaluator review precedes S077. No new owner-only
blocker. Full verify/Linux/coverage/mutation/fuzz/bench/package/live product/crypto/
host/signing/operational acceptance remains unverified. No human release-testing
milestone yet. No supervisor product code/tests in owning checkout; public/private
boundaries and unrelated work preserved. Planning remains uncommitted while the
candidate is red; the first genuinely green Pi repair includes these approved
planning updates without changing the criteria.
