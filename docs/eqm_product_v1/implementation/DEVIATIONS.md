# Deviation register

Revision 2 is the imported planning baseline. DEV-001–DEV-004 record approved amendments; source implementation status is tracked separately in progress.json and STATUS.md.

The commit order and boundaries are mandatory. Do not skip, merge, reorder or broaden a step for convenience. Only inspected repository evidence that a step is obsolete or unsafe may justify a documented deviation; a product-intent/security change additionally needs owner approval.

## Required record

Deviation ID/date/author; affected step and requirement IDs; actual HEAD; precise repository evidence; why the prescribed action is obsolete or unsafe; smallest replacement/disposition and dependency effects; spec/acceptance impact; verification commands/results; owner approval where required; next-step safety. Preserve the original step ID as a traceability reference. A split must define independently green replacement commits, not bury new scope in a single large patch.

Missing product repositories/hosts/credentials are explicit gates, not proof that requirements are obsolete. Optional-native steps remain unactivated outside the mandatory sequence unless approved; that is a declared scope boundary rather than a silent skip.

## DEV-001 — Validate expanded risk before complete preparation

**Date/author:** 2026-09-26, implementation planning agent. **Status:** owner-approved plan amendment; implementation pending. **Affected step:** S049; S004/S006 record/freeze the contract, S057/S076 provide later regression contributions. **Requirements:** R14, R23, R50 plus S049 completeness requirements. **Acceptance:** AC014, AC023, AC050 and mapped completeness cases. **Finding:** RA05/RA06.

**Actual reviewed HEAD:** 7c8293837c09c627489cdf244a2fbd97b60aa8ed. `crates/eqm_engine/src/expand.rs:20` validates only before expansion; lines 132–134 finalize the rebuilt graph without validating expanded requirements; line 158 copies optional explicit risk without preserving the fragment default. `crates/eqm_domain/src/graph.rs:302` exposes unchecked finalized construction. Existing risk invariants inspect fragment and journey defaults separately.

**Why the unamended action is unsafe:** S049 cannot establish a valid complete prepared scope over a graph whose expansion weakened inherited risk or bypassed invariants. Mapping the repair to S030's source DAG or only to later test steps would leave the behavioral repair unspecified.

**Smallest amendment:** extend S049 to preserve the stronger fragment/journey default, reject explicit risk below it, validate after expansion, and close unchecked finalization with an acyclic ownership/API design. Include expansion/invariants/domain graph and affected manifest canonicalization callers. Preserve S049's original completeness construction and its position after S048. No added dependency on later trust/storage code; no merge, skip or reorder. If source evidence later requires a separate independently green split, document it before implementation and retain S049 traceability.

**Approval:** owner approved all recommendations following the 2026-09-26 readiness review; ADR-0012 records the scope. **Spec impact:** inherited-risk and validated-boundary rules are normative; no new product subsystem. **Required verification:** absent/explicit risk across higher/lower source/destination defaults, post-expansion invalidity, public-construction bypass, and real loader/expansion/preparation integration, plus S049 standard commands. **Results:** not_run; only static source review and plan validation are established. **Next-step safety:** S050 may start only after amended S049 is green and its evidence is recorded.

## Supervisor corrections — CR01–CR08 (S001–S014)

The independent supervisor review of the S001–S014 batch (recorded externally)
required eight corrections before S015. Each was verified against current code
and fixed as a separate corrective checkpoint; none renumbers or merges the 211
mandatory steps, and the original step IDs and dependencies are preserved.

| ID | Severity | Disposition | Corrective commit | Behavior change |
| --- | --- | --- | --- | --- |
| CR01 | P1 | Fixed | `ba883c6` | strict ingestion honors each caller's established bound instead of shrinking every record to 8 MiB; adapter responses keep 16 MiB |
| CR02 | P1 | Fixed | `ba883c6` | reconcile inventory and release-record ingress route through the strict duplicate-rejecting boundary; valid controls and escaped-spelling duplicates added |
| CR03 | P1 | Fixed | `54bf325` | ADR/API/vectors synchronized on AutoSi subseconds, signed/unsigned decimal strings and NFC-vs-opaque text, with concrete executable vectors |
| CR04 | P2 | Fixed | `84435fb` | host evaluation time preserves subsecond precision (AutoSi); S011 nanosecond freshness overclaim corrected to S069 |
| CR05 | P2 | Fixed | `84435fb` | `EvidenceReadRequest` is bounded private validated state with an accessor |
| CR06 | P2 | Fixed | `84435fb` | narrow `ReadCapabilities` view plus `read_evidence` use case; S012 privilege-isolation overclaim corrected |
| CR07 | P2 | Fixed | `cdda196` | `LoadError` exposes retained causes through `Error::source`, exercised by chain traversal |
| CR08 | P2 | Fixed | this checkpoint | S014 marked complete with its SHA, resume cursor set to S015, and report/ledger overclaims reconciled |

S014's report is corrected to record that its first strict-parser bound wrongly
shrank the established adapter-response limit (CR01) and that two of its DTO
integration tests used objects the old parser also rejected (CR02). RA13
relationship closure remains with S025/S059/S061. No product/security intent
changed and no step was skipped.

## DEV-002 — Reopen S015 and allocate the v2 completion layer

Date/author: 2026-09-26, Codex supervisor under explicit owner delegation. Reviewed HEAD: `3da0b50aeb7cf7f8b23f8f5ebd47f043892ed00b`. Status: approved engineering amendment; repairs pending.

The [binding decisions and independently green S015-R1–R5 checkpoints](COMMIT_SEQUENCE.md#dev-002--supervisor-owned-s015-completion-and-continuation) are part of the existing governing sequence. [Review RF01–RF10](reports/CODEX-3da0b50.md) supplies the evidence: v1 acceptance regression, missing v2 schemas/null handling, integer canonicality/endpoints, temporal validation, ingress/read bounds and incomplete proof. S015 already owns these representation and schema obligations; no new product subsystem or renumbering is approved.

Preserve all S001–S211 IDs; reopen S015 and block S016 admission until its five corrective checkpoints pass. S015-R4 also closes outstanding contributions from earlier corrections. All original R/AC/RA links remain in force. Generate schemas from source, preserve v1 and original signed bytes, run the declared targeted/standard/doctest/structure checks, and record each actual corrective SHA. No source repair is asserted by this planning amendment. The owner authorizes Codex to make scope decisions; the implementation agent performs the decided work and reports new decisions to the supervisor. Full continuation is authorized through the dependency-unblocked sequence toward S211; concrete external gates remain binding.

## DEV-003 — S015 reader isolation and conformance closure

**Date/author:** 2026-09-27, Codex supervisor under owner delegation.
**Reviewed HEAD:** 7d44724abba2fa90baf806563da0a692c5bb5857.
**Affected steps/contracts:** S015 and retained S006/S011/S012/S014 regression
contributions; R18/R19/R21/R52/R64 and existing acceptance links.
**Evidence:** [review RF11–RF18](reports/CODEX-7d44724.md): actual command ingress
accepts unsupported v2 workspace/lock labels; nested inventory validation,
runtime-facts limits, schema assertions and test evidence remain incomplete.

**Why proceeding unchanged is unsafe:** green tests leave version boundaries
open and overstate conformance. JSON Schema cannot prove original numeric token
spelling, so requiring that would incentivize an incorrect substitute validator.
**Replacement:** four independently green corrective commits S015-R6–R9 in the
existing [governing sequence](COMMIT_SEQUENCE.md), retaining all original step IDs
and R1–R5 history. Exact version checks, existing bounds, actual Draft 2020-12
validation with a dev-only dependency, and real input/boundary tests are approved.
The raw canonical contract is unchanged; value and semantic layers are explicit.
**Continuation:** after objective repair gates, Pi continues dependency-unblocked
S016–S211 with independent supervisor acceptance still pending; no gate waiver.
**Verification:** current-source standard check, application doctest and build
passed; ingress probe demonstrates the defect. Package structure/diff checks
are required on this planning update. Implementation is pending with Pi.

## DEV-004 — S015 acceptance, dependency prerequisite and S016 decisions

**Date/author:** 2026-09-28, Codex supervisor under owner delegation.
**Reviewed HEAD:** 121622ff61a18efb0afd1dc9a13264fc2a97f259.
**Affected scope:** S015 acceptance; S005-origin dependency correction allocated
as S016-P1; S016 collection/text enforcement (R18/R19/R21/R43/R64 and its existing
AC/RA contributions). **Evidence:** [review](reports/CODEX-121622f.md).

R6–R9 close the S015 findings with source and independent standard/CLI/schema
checks. The live advisory audit passes, but full dependency policy fails RF19.
Leaving an unversioned dependency until release would preserve a known broken
gate; one exact-pin prerequisite is the smallest repair. No policy threshold
changes. Codex approves the reviewed validator's Zlib allowance after inspecting
its manifest, license, resolved features and successful license/source checks.

The governing DEV-004 text settles S016 field classification, duplicate-safe
canonical ordering, text-versus-opaque disciplines and RV-TEXT-2's JCS expected
output. Preserve v1 behavior and raw bytes. These are engineering closures of
approved representations, not new product features or permission for trust.

S016-P1 is one green commit; S016 follows separately, then ordered unblocked
S017–S211. No renumbering, merged steps, paused S015 acceptance or human release
approval is needed. Package structure and documentation diff checks validate the
planning update; no source implementation is claimed by this amendment.

## DEV-005 — Reopen S016 and settle S017 representation

**Date/author:** 2026-09-28, Codex supervisor under owner delegation.
**Reviewed source:** 68378317044d49a121caff07e783573abdf3cd56.
**Affected contracts:** S016 R18/R19/R21/R43/R64 and S017 R19, with existing AC/RA links retained.

[Review RF20–RF23](reports/CODEX-6837831.md) finds incomplete authored validation,
NFC-only syntax acceptance, missing boundary controls and a historical receipt
discrepancy. RF19 is accepted as fixed. The [DEV-005 amendment](COMMIT_SEQUENCE.md#dev-005--s016-review-closure-and-binding-projection-decision)
allocates S016-R1/R2 before S017, retains all 211 step IDs, freezes the field/type
mapping and separate v2 binding projection, and preserves v1 behavior. Current
structure validation passes; the earlier failed log is retained as failed evidence.
Codex reconciles status now; Pi implements the source/test repairs and continues
all dependency-unblocked work through S211. No implementation fix, live product
acceptance or release readiness is asserted by this planning amendment.

## DEV-006 — Projection type safety, regression closure and S019 composition

**Date/author:** 2026-09-28, Codex supervisor under owner delegation.
**Reviewed source:** 3469face3a88d280f2577f4ff936419408c1d248.
RF20/RF21 are accepted; RF22 remains partial. [RF24–RF26](reports/CODEX-3469fac.md)
require typed component identities, real manifest-boundary proof and recursive
v2 extension type tags to prevent large-integer/text canonical collisions.
All four implementation commits remain; S016-R3 and S018-R1/R2 close the gaps.
Only the affected new v2 extension goldens change; v1 remains frozen.

The [DEV-006 governing amendment](COMMIT_SEQUENCE.md#dev-006--close-boundary-evidence-and-unblock-contract-provenance)
settles S019: contract entities plus normative graph extensions, explicit v2
forms, narrow validated domain selection carriers, full selected definitions and
known/unknown values, canonicalization in manifest and same-session assembly in
application. S019-A/B/C are independently green slices without renumbering the
211 steps. The decision block is resolved; code/test changes remain Pi-owned.
Continue ordered unblocked work to S211 after the objective gates. No external
authority, release readiness or new operational/native activation is inferred.
