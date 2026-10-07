# Implementation status

Current amendment: DEV-035 in COMMIT_SEQUENCE.md. Codex owns implementation and
planning; implementation and green checkpoint commits are authorized by `pfc all`.
Active checkpoint: P04-R4B2, then P04-R4C. P04-R4B1 semantic evaluated carrier
is verified and independently reviewed for allocated scope; full standard gate
passes with 798 successful test executions. Candidate
b442b806e26a7ddd08ca8543b802c1b3d91251f0 is committed. RF79/P04-R4A independently
accepted for allocated scope; 47 prior numbered acceptances retained. Original
S044/S047/S048/S049 remain partial, S052–S211 unstarted (164 numbered steps remain
unaccepted). Aggregate P04-R4 and whole MVP remain incomplete.

Next: P04-R4B provenance/protected caller, P04-R4C durable controls including RF84
causal dirty-source regression and RF83 v1/v2 CLI coverage, S052–S076, mandatory
independent evaluator acceptance before S077, then eligible original S077–S211.
RF75/RF78/RF81/RF82/RF83 remain open. Existing technical decisions are settled;
no current owner-only software blocker or human release-test milestone.

Independent release tests: 16 passed. Restored unsafe matcher makes three new
real-ingress negatives return Pass and fail (13 others pass). Pi's retained full
gates corroborate local implementation only. RF84 source-clean coverage is masked
by the already non-authorizing full-scope fixture and requires repair.

Original 64 R/AC, 21 RA and Q01–Q15 preserved. Global RA02/RA13, extended verify,
Linux, coverage/mutation/fuzz/bench/package, live product/crypto/host and signing/
operational acceptance remain open. No publication or deployment authorization.
See [review](reports/CODEX-b442b80.md) and [plan](COMMIT_SEQUENCE.md).
