# Codex independent review — eb2f323

Date: 2026-10-02. Reviewed five retained commits after supervisor `83d18df`:
S019-R1 `3f5c96b`, S019-R2 `6459b69`, S020 `a9df2f1`, S021 `0e6584f`, partial
S022 `eb2f323`. EQM master was clean at review start. Pi owns source/tests;
Codex made no product edits. This review inspected changed source/tests and
relevant governing contracts; it does not claim all unchanged software was
re-reviewed or live product acceptance.

## Disposition and findings

RF27 accepted: one shared recursive applicability helper applies complete JCS
member-byte set ordering in both binding/contract projections; true permutations,
duplicate rejection and heterogeneous literal traps execute. Domain/v1 behavior
and ordered journey/extension semantics remain. S017's remaining contribution closes.

S020 accepted for allocated current-target scope: exact six-member projection,
TargetConfigurationDigest, bound bytes, exact same-session graph lookup and typed
missing/projection errors. Semantic field mutations, display/formatting and
unrelated-target/policy controls execute. Source/build axes and future role/source
extension duties remain deferred to owning steps, not implicitly satisfied.

RF28 mostly repaired, still partial: in `tests/policy_v2.rs`, the minimum rule
endpoint assertion invokes boundary_expected(1), not production projection;
rule_order_uses_full_member_bytes compares permutation outputs without a literal
order trap; populated_dimension_description_is_present_and_absent_exactly proves
only presence. Contract/policy revision expected helpers mirror numeric selection
instead of freezing literal expected tokens. S019-R3 finishes these existing
obligations. The real loaded revision/default/isolation/session controls are
accepted as evidence contributions; no observed defect is inferred solely from
missing proof.

RF30 (P2): `eqm_runner/src/resolution.rs::canonical_runner_identity` invents a new
`eqm:v1:runner-identity` binary digest and returns generic Sha256Digest rather
than the approved v2 RunnerDigest/JCS contract. Content/context mutation tests
prove map-boundary sensitivity, not independent executable acquisition. S021-R1
freezes the explicit v2 identity, tagged template semantics, typed API and literal/
real executable controls. Keep stronger binding and record runtime hash change;
preserve v1 DTO schemas/original signed bytes and established v1 oracles.

S022 partial: repository normalization/inspection centralization retained. The
release gate obtains dirty provenance but does not consume it, so same-HEAD
modified content can still meet the old completeness predicate. Its clean() docs
also overclaim exact byte equality from normal Git status. Codex resolves its
three-option request conservatively in DEV-008: repository-wide inspection,
reserved untracked .eqm outputs only, fixture moved to that area, and an enforced
non-authorizing dirty gate. Ignore patterns/concealment flags cannot authorize
ignored source. No scope reduction or new owner approval is required.

Progress has stale next_checkpoint/supervisor-review metadata and missing S021/
S022 SHAs. DEV-008 reconciles retained implementation versus accepted state.

## Verification and limits

Codex independently ran `cargo xtask check` at reviewed source (exit 0), locked
domain/manifest/application doctests (exit 0), and dependency policy check (exit 1: yanked yoke-derive 0.8.3).
The configured build environment was diagnosed before verification. Exact results
and execution counts are in the external supervisor receipt. Target's literal
hash was independently recomputed from reviewed UTF-8 bytes and
`eqm:v2:target-configuration` plus zero delimiter; it matches
`sha256:52a4e99e8a7096ba27535fa7c74fa755534298263b77f8fa5e9a68bda4f3c490`.
All 58 retained checkpoint log files were scanned; one preserves a failure,
and exit ledgers record reruns. S022's receipt claims a dirty-gate failure but its
available test-targeted logs/exit ledger do not preserve that failed run; that
historical exact result remains unverified. Source inspection establishes the
unconsumed dirty gate independently. No retrospective test pass is fabricated.

No schema or Cargo.lock changes occurred in this batch. New planned runner hashes
are explicitly scoped, not evidence that v1 wires acquired v2 fields. Full verify,
Linux, coverage/mutation/fuzz/bench/package release, live product/crypto/host,
signing and operational acceptance are unrun. This is not a release candidate.

## Next

S001–S018 and S020 accepted for allocated scope, 19 numbered steps. S019/S021
retain implementation with corrections; S022 incomplete, decision resolved.
Execute S019-R3, S021-R1, S022 and all ordered eligible S023–S211. Hard dependencies,
actual review gates and Q01–Q15 remain binding; a commit/report is not a stop.
Codex owns scope/acceptance, Pi owns product implementation. Start a fresh Pi
session to focus on the resolved contract and traceable full batch.

RF31 (P2): current dependency policy rejects locked yoke-derive 0.8.3. Official
registry metadata lists compatible 0.8.4 unyanked (MSRV 1.82). DEV008-P1 pins that
transitive replacement narrowly, preserving direct pins and no policy waiver.
Fresh audit/deny and standard/schema lanes are required before further source work.
