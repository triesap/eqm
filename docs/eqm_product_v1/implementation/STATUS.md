# Verification and implementation status

Current binding amendment: DEV-007, 2026-09-28. Reviewed implementation HEAD:
`1512c9fa7e886781bdf2976dfaabde1d8328ac43`. Six Pi checkpoints are retained and
backfilled in progress.json; [Codex review](reports/CODEX-1512c9f.md) records
source findings and evidence limits.

S001–S016 and S018 are accepted for allocated scope (17 numbered steps). S017
retains implemented binding semantics; its remaining applicability contribution is
implemented in S019-R1. S019-A/B/C retain their implementation and the DEV-007
RF27/RF28 corrections are implemented in S019-R1/R2, all pending Codex review.
P01 is complete (12/12); P02 has five accepted numbered steps (S013–S016, S018),
with S017/S019/S020/S021 implemented pending review and S022 in progress and
blocked on a Codex representation decision (its centralization is committed; its
release-provenance gate is not). S023–S027 remain. All later cycles are pending.
No committed code is discarded.

Next: **S022 (BLOCKED on a Codex representation decision), then S023 → S211**.
S019-R1, S019-R2, S020 and S021 are implementation-complete pending Codex review.
S022 centralized repository-identity normalization and source provenance (with a
dirty/clean flag) in `eqm_application`, but wiring dirty content into clean
release provenance conflicts with the approved signup release fixture, which
writes untracked generated release records; the exact dirty representation/gate
requires a Codex decision (see [S022](reports/S022.md)). S023 and later steps are
blocked until that is resolved. Later S028/S029/S031/S038/S043 retain the approved
obligation to bind newly modeled target role and source-component fields.

RF22/RF24/RF26 are accepted. RF25's loader/display/duplicate controls are accepted;
its ordered-child obligation is superseded by RF27's set semantics. RF27, RF28 and
S020's target-definition identity are implemented pending Codex review. S021's
executable-bound runner identity is implemented pending Codex review. RF29's
status/receipt discrepancies are reconciled, with missing historical exits
explicitly unverified. Prior RF23 history remains intact.

Codex independently ran the standard gate at reviewed HEAD. Current standard gate
exits 0 with 463 successful test executions through S021. Six component literal
hashes recompute exactly.
Schemas (22 v1, 5 v2) and Cargo.lock are unchanged across the batch. Current
structure/doc checks are recorded in the supervisor receipt. These checks do not
close whole acceptance or relationship cases: S025/S059/S061 retain relationship
work, S027 provenance corpus, S069 precise freshness, later steps v2 consumers.

Full xtask verify, Linux, coverage/mutation/fuzz/bench/release package, live product,
crypto/host and operational authorization remain unverified. This is not a release
candidate; human release testing is not the next dependency.

Codex owns material decisions and acceptance; Pi generates implementation. Preserve
Q01–Q15 at their first dependent step. No push, publication, deployment, signing,
live enforcement or optional-native activation is authorized.
