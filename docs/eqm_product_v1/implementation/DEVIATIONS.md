# Deviation register

Revision 2 records one owner-approved planning amendment. No EQM source code has been changed by this package.

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
