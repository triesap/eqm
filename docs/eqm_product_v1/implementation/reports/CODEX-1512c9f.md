# Codex review — source 1512c9f

Date: 2026-09-28. Reviewed range: `8bc0fdc..1512c9f` (six Pi checkpoints,
23 changed files). Working tree was clean. Review covered every changed Rust
module/test, related domain constructors/v1 field inventory, the governing plan,
progress/reports, handback and available checkpoint logs. This is a review of the
current batch and its relevant dependencies, not a claim to re-review every
unchanged source file or establish end-to-end product acceptance.

## Findings

| ID | Priority | Evidence and disposition |
| --- | --- | --- |
| RF27 | P2 | `crates/eqm_domain/src/entity.rs:581` stores all/any as BTreeSet; constructors reject duplicates. DEV-006's sequence instruction was Codex's inconsistent decision. Both `canonical/binding_v2.rs:52` and `canonical/contract_v2.rs:122` emit enum-ordered children, not complete JCS member-byte order. `tests/binding_v2.rs:869` swaps values across fixed dimensions, not child positions. Correct the decision and both v2 projections in S019-R1; preserve domain/v1 behavior. |
| RF28 | P2 | The five policy projection tests and nine contract tests lack the required numeric boundary/endpoint, multi-profile/rule member-order, exact-revision failure, unselected-definition and actual formatting controls. Application tests use separately prepared graphs but do not prove immutable same-session reuse after disk mutation. S019-R2 defines the missing concrete matrix. No observed production defect is claimed merely from missing tests. |
| RF29 | P2 | STATUS simultaneously calls S019 complete and not started, retains four P02 completions while progress records five, and claims ordered all/any proof. S019 and multiple checkpoint SHAs are null. S019 tables cite absent package-structure.log paths while final-package.log exists; exits.txt does not record all claimed package/diff exits. Supervisor reconciles current status/SHAs and reports; missing historical exits remain unverified. Future receipts must be exact and append-only. |

Ordering counterexample follows directly from inspected source: derived enum Ord
places Comparison before All; complete JCS bytes for a nested `{"all":...}` child
precede a `{"dimension":...}` comparison. The Python byte-order comparison was
independently checked; a new Rust regression was not written by Codex. Pi owns
that implementation/test change. No flattening or algebraic simplification is
approved.

## Accepted repairs and retained implementation

- RF22 accepted: real exact-duplicate controls, attachment permutations/JCS,
  runtime same-key conflicting value with recomputed digest, standalone/embedded
  inventory paths and diagnostic-vector order/multiplicity now execute. Silent
  serialization fallback removed.
- RF24 accepted: binding and evidence-specification APIs return existing typed
  wrappers; positive type checks and cross-domain compile-fail doctest execute.
- RF26 accepted: recursively tagged extension values distinguish integer/text,
  nested data and tag-like authored objects. Two approved boundary hashes changed;
  base hashes and v1 forms stay unchanged.
- RF25 actual loader comments/formatting/semantic mutation, nested display-only
  exclusion, reachable post-exclusion DuplicateMember and distinct-exposure
  controls accepted. Its impossible ordered-child obligation is superseded by
  RF27, not represented as executed ordering proof.
- S019-A validated carriers correctly retain exact definitions and explicit
  known/unknown values, reject missing/duplicate/undeclared inputs, and confer
  no authority. S019-B/C projections preserve the reviewed v1 field inventory in
  explicit v2 forms. Pure manifest/application ownership and one prepared graph
  are retained; no latest lookup, I/O, clock, execution or workspace-hash fallback
  was introduced. Final S019 acceptance awaits RF27/RF28.

| Checkpoint | Retained commit |
| --- | --- |
| S016-R3 | `1f57d23ee470615d87ff91b3781b2d777ccf64af` |
| S018-R1 | `19064c48ce20c867abb595adf83dfd6bced91b6a` |
| S018-R2 | `a642dfcd97efae729fb618a73c5bd7aedca7b9cd` |
| S019-A | `5b1a12d87aed0ac56b3389fbc16612c6d9646dd0` |
| S019-B | `64e113d2b34e85f29bd761badeb02b211910acff` |
| S019-C | `1512c9fa7e886781bdf2976dfaabde1d8328ac43` |

## Independent verification

The configured build environment was checked before verification. At reviewed HEAD, Codex ran
`cargo xtask check` (exit 0; 428 successful test executions, including repeated
lanes rather than 428 asserted unique cases), locked domain/manifest/application
doctests (exit 0, including the compile-fail type boundary), live
`cargo audit --deny warnings` (exit 0), and `cargo deny check` (exit 0).
Six literal canonical hashes were independently recomputed with Python SHA-256
and the inspected domain labels/zero delimiter: binding base/boundary, evidence
specification base/boundary, contract and policy selection. All matched. Git
comparison confirms schemas and Cargo.lock unchanged across the six commits.
Standard gate includes schema parity; package and updated-document checks are
recorded in the supervisor receipt after this planning amendment.

Available final Pi standard/test/schema logs are green. S018-R2 and S019-A exit
ledgers explicitly preserve failed runs followed by green reruns. Several named
log files contain only the final output, so the handback's broad claim that all
failed raw output was retained cannot be independently established. Audit/deny
logs are present for S016-R3, not every later checkpoint; current independent
reruns establish the reviewed HEAD only. Preserve the original receipts and
historical discrepancies rather than rewriting them into passes.

Not run: full xtask verify, Linux, coverage/mutation/fuzz/bench/package release
lanes, real product/crypto/host/signing and operational authorization. Fixture
success is not product conformance. This is not a release candidate and human
release testing is not the next dependency.

## Decisions and next work

DEV-007 settles all/any as canonical sets, S019-R1/R2, and S020's exact current
Target projection/application lookup. Target has no modeled build/profile/source
references at this stage; do not invent them or hash aggregate context. Future
role/source owners must extend the projection and subject linkage atomically.
S001–S016 and S018 are accepted for allocated scope; S017/S019 retain committed
implementation with corrections required. Next S019-R1, S019-R2, then S020 and
all ordered unblocked successors through S211. Q01–Q15 remain binding. Start a
fresh Pi session because the previous session ended for context/runtime budget.
