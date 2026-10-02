# Codex independent review — d6261c4

2026-10-02; six retained commits after planning `29ed78e`:
DEV008-P1 `ba04463`, S019-R3 `9fd4e9e`, S021-R1 `5e42c46`, S022 `af5aab6`,
S023 `9f000b6`, S024 `d6261c4`. Product checkout clean at review start.
Codex inspected changed sources, production callers, tests, original criteria,
previous findings, checkpoint logs and the matched native instruction/activity.
Source/test probes were isolated from the product checkout. No product edits.

## Accepted contributions and remaining findings

DEV008-P1/RF31 accepted: only yoke-derive version/checksum changed to 0.8.4;
exact direct pins, dependency policy and schemas preserved. Fresh independent
deny and advisory audit pass. Prior S001–S018/S020 acceptance is preserved.

RF28 largely repaired: real minimum/max rule projection, literal revision tokens,
heterogeneous JCS ordering trap, permutation byte/digest equality and absent
Description omission controls execute. The trap still has no independently fixed
literal complete digest; S019-R4 completes that existing DEV-008 demand.

RF30's explicit bounded JCS/RunnerDigest implementation is accepted as a
contribution: complete definition/program/context root, typed accessors and wire
adapter, tagged placeholders, quantities, extensions and literal byte/digest
oracle. Runner positive/compile-fail doctests pass independently. RF34 (P2): the
'actual acquisition' test reads fs::read in a test closure mirroring the CLI,
not the production runner_authority path. The argv reversal control also changes
owners/environment, so it does not isolate sequence behavior. S021-R2 tests the
actual caller and isolates field/collection controls without replacing sound code.

S022 now consumes clean() in release completeness and preserves real fixture
Pass/Fail/Unknown, generated output neutrality and top-level dirty/concealment
controls. RF36 (P1): concealment rejection checks only the parent index. An
isolated nested Git fixture with child assume-unchanged plus modified bytes
returns clean(). Submodule status cannot replace child index/worktree inspection.
S022-R1 traverses exact nested Gitlinks, rejects concealment/unavailable inspection,
retains confinement/limits and adds actual release-boundary controls. Existing
fixture/gate improvements remain; they do not establish universal executed bytes.

RF32 (P1), S023: identity_relation accepts arbitrary non-JSON raw bytes unrelated
to self and constructs a purported validated relation. An isolated executed
probe passed such unrelated bytes through the production API. RF33 (P1): the
same method hashes v1 default serialization under the v2 semantic domain;
attachment sizes 9007199254740992 and 9007199254740993 collapse to the same
SemanticEvidenceDigest after JCS numeric rounding. The independently executed
probe observed `sha256:c22ed62b3d2e9dc3366c84cd0df20fe7a06868088f4186df870b58f44de36196`
for both. V1 null/time/set forms also do not establish ADR-0013 v2 semantics.
S023-R1 freezes a bytes-first explicit v2 result and projection, preserving v1
and originals. Domain raw-byte/typed digest carriers are useful retained work;
no storage/idempotence/conflict or admission acceptance follows from them.

RF35 (P2), S024: typed IDs/presence/duplicate attempt construction are retained,
but no validation matches a lineage's plan/campaign/run/case/attempt against a
declared execution and expected context. Only MissingPlan is tested; run/case
'independence' changes both axes simultaneously. S024-R1 adds checked relationships
and independent missing/mismatch controls. These are internal non-authorizing
records; later acquisition/selection/storage semantics remain unfinished.

RF37 (P2): active_step/next_step S019 conflict with next_checkpoint S025;
supervisor metadata still identifies eb2f323 while some values were replaced by
new self-verification. The generic final command list implies separate schema
checks per step, while S023/S024 evidence establishes schema parity inside the
standard gate and no separate command. Expanding shell helper arguments found
no reused raw-log destinations; repeated helper definitions alone are not
overwrite evidence. Native/raw failures remain retained. DEV-009 corrects
frontiers and evidence attribution without inventing historical checks.

## Independent verification and limits

At reviewed source cargo xtask check exit 0 (489 successful test executions),
runner positive/compile-fail doctests exit 0, cargo deny check and live cargo
audit --deny warnings exit 0. All builds were routed after environment diagnosis.
Three isolated regression probes executed and confirmed the defects above;
a first probe location was rejected by the build router's source-root rule,
then the worktree was moved to an allowed source root and diagnosis/checks passed.
Those probe passes prove reproduced defects, not acceptance of candidate behavior.
Runner literal domain hash was independently recomputed from frozen bytes.
All 93 retained batch files were read and indexed; native calls/results and
parent-linked branch were examined. Reports remain distinct from actual checks.

The matched native record contains the issued handoff as an actual user command,
not quoted output. Launch directory differs from the instructed product root;
actual build/edit/Git calls use the product root. One instruction/branch was
matched; no subsequent continuation/fork/retry session was found. Six final
commits, two documentation amendments and intermediate failures are retained in
history/evidence. Configured/observed model selection and high thinking appear
in native metadata. The final message reports a context-budget interruption;
the log ends normally, so a hard runtime/context exhaustion event is not proven.
It did not complete S211, and S025 was not started. No Pi process was observed
continuing work at inspection. Native identifiers/private paths remain external.

Full verify/Linux/coverage/mutation/fuzz/benchmark/release packaging, live product/
crypto/host/signing/operational acceptance unrun. No human release test is the
next gate. Synthetic EQM-P03 work is eligible; Q02/Q06 real evidence remains.

## Next decision

DEV-009 orders five repairs then every eligible S025–S043, `pfc through EQM-P03`.
S043 is the schema/model/subject parity boundary before S044 obligation routing;
full MVP S044–S211 remains pending. Engineering representations are frozen before
handoff. Codex owns independent acceptance, Pi product code/tests. No prior
acceptance is revoked without new evidence; no whole R/AC/RA is closed here.
