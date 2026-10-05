# Verification and implementation status

Latest checkpoints: P04-R1 and S044 implemented and locally verified at this
run's own commits; Codex acceptance pending. P04-R1 replaces the copied numeric
range predicates in the protocol boundary corpus with the actual owning
`PositiveCount`/`Revision`/`DurationMillis`/`RunnerLimits` and checked-`u32`
`PositiveDays` constructors, and expands the normalized-text corpus to every
forbidden C0/C1 point (embedded and trailing), allowed LF/TAB, empty and each
carrier's exact ASCII maximum/over-maximum; RF70 durable coverage is now in the
product test corpus. S044 moves exact selection orchestration into the new
application `prepare_obligations` operation with explicit mode and authority, so
protected modes cannot take an implicit development branch. S045 rejects a
repeated `--profile` selection instead of silently overwriting it. S046 loads
and selects exact policy/profile revisions through the real manifest loader
(revision-aware authority keys) and adds explicit `--policy <ID[@REVISION]>`
selection that never guesses a latest revision. S047 introduces the pure
`RequiredUniverse` denominator derived before any evidence, with a subset check
for later policy-strengthening comparisons. S048 retains unknown and justified
not-applicable coordinates in the visible universe so skipped requirements
cannot disappear from completeness. S049 applies DEV-001: expanded fragment
requirements inherit the stronger fragment/journey risk, explicit lower risk
rejects, the expanded graph is revalidated before finalization, and the release
path uses the checked `CompletePolicyResolution` universe boundary. The two
reviewed CLI goldens changed only their graph digest. S050 makes an empty
required release scope non-authorizing without explicit protected permission and
labels a one-member cohort as no meaningful comparison. S051–S076 remain pending
(26 checkpoints).

Codex independently reviewed clean `1496c28616a769b9c958965c0377cb3b83ded32e`,
matching the actual issued native instruction and completed branch. All four
P03-R14/R15/R16/S043-R6 commits retained; RF68/RF69 allocated fixes and S043/P03
local implementation accepted. Preserve 42 earlier numbered acceptances:43 total.
P01–P03 local implementation complete; real Q02/Q06/platform acceptance separate.
RF45 P03 parity contribution accepted, evidence-boundary contribution S059/S061
pending; whole RA02/RA13 and original 64 R/AC, 21 RA, Q01–Q15S211 remain open.

RF70: the committed numeric corpus copied range predicates. P04-R1 now retains
the durable actual-constructor checks (`PositiveCount`, separate `Revision`,
`DurationMillis`, `RunnerLimits` with valid other limits, checked-`u32`
`PositiveDays`) plus every forbidden C0/C1 point, embedded and trailing, and each
carrier's exact ASCII length endpoints. Complete S044–S076 under current DEV-028,
`pfc through EQM-P05`. P04 alone is not a stop.
Independent complete reference-evaluator review precedes S077 storage integration.
RF71 stale report arithmetic corrected: returned standard counts773/775/777/778/782,
not distinct tests. Failed attempts retained; workload contention is not a pass.

See [review](reports/CODEX-1496c28.md) and sole [plan](COMMIT_SEQUENCE.md).
No Codex production source/test edits. Generated v1 schemas/Cargo.lock unchanged.
No release/human release-testing gate reached. Full verify/Linux/coverage/mutation/
fuzz/benchmark/package/live product/crypto/host/signing/operational acceptance
unverified. Missing local automation is not inherently hardware-blocked. Real
policy values/authority remain explicit; no signing/publication/deployment/native
activation is authorized. P04–P15 remain unfinished.
