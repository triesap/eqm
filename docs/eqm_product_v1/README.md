# eqm_product_v1 — implementation handoff

**Status:** revision 2 owner-approved execution scope; all implementation steps pending.
**Prepared:** 2026-09-26. **Repository baseline:** `triesap/eqm`, `master`, `7c8293837c09c627489cdf244a2fbd97b60aa8ed`.

Start with [PROMPT.md](PROMPT.md). It directs an agent to a safe repository preflight and then the first pending commit. The complete ordered plan is [implementation/COMMIT_SEQUENCE.md](implementation/COMMIT_SEQUENCE.md); smaller chapter copies in `implementation/sequence/` and `implementation/steps.json` support selective loading. Do not run hundreds of commits as an unattended unreviewed batch.

## Contract stack

1. [Product intent](specs/PRODUCT_SPEC.md), [architecture](specs/ARCHITECTURE.md), [semantic rules](specs/EVALUATION_SEMANTICS.md).
2. [Data model](specs/DATA_MODEL.md), [API contracts](specs/API_CONTRACTS.md), [evidence identity](specs/EVIDENCE_AND_IDENTITY.md), [trust/release](specs/TRUST_AND_RELEASE.md).
3. [Execution](specs/EXECUTION_SECURITY.md), [storage/performance](specs/STORAGE_AND_PERFORMANCE.md), [runtime integration](specs/RUNTIME_INTEGRATION.md), [compatibility](specs/COMPATIBILITY.md).
4. [Acceptance criteria](specs/ACCEPTANCE_CRITERIA.md), [test plan](implementation/TEST_PLAN.md), [verification](implementation/VERIFICATION.md), [traceability](implementation/TRACEABILITY.md).
5. [Open decisions](implementation/OPEN_QUESTIONS.md), [ADRs](decisions/README.md), [review findings](repo/REVIEW_FINDINGS.md), [baseline](repo/BASELINE.md), [status](implementation/STATUS.md).

[summary.txt](summary.txt) is a standalone reconstruction record including product decisions, all durable contracts, review findings, decision gates, acceptance criteria, and the implementation inventory. Its duplication is intentional for disaster recovery; it is not the daily agent context.

## Using the package

Keep this extracted folder **outside** the Git repository during preflight. The baseline repository rejects extra documentation paths and the generic word “handoff” in some checks. Step S001 changes those narrow governance conflicts; S002 then imports the package under the approved spec subtree. Do not bypass gates by deleting safety checks or overwriting the repository's AGENTS.md.

`repo/AGENTS.md` is proposed guidance to merge after reading the existing instructions. New module and conceptual API names are planning allocations, not claims that symbols already exist. Values not established in the conversation are explicitly unresolved; no production credential, signer, release destination, OS-support promise, or business workflow is invented.

The mandatory plan implements EQM assurance infrastructure and adapters/fixtures. Optional native embedding has its own gated [extension sequence](implementation/OPTIONAL_NATIVE_COMMIT_SEQUENCE.md). Absence of real product repositories blocks live acceptance, not the ability to build safe generic contracts and fixtures.

## Package verification

Run `python3 tools/validate_package.py` from this folder (standard library only). This validates package integrity, internal references, required step fields, cycle/regression traceability, and pending status. It does **not** compile EQM or run Rust/native tests. `SHA256SUMS` covers every payload file except itself. Revision 2 is a directory reissue; no replacement archive was generated.

See [sources/SOURCES.md](sources/SOURCES.md) for pinned repository references and external primary documentation. The package contains specifications, not a source-code checkout, private credentials, or prebuilt application binaries.

## Approved execution revision 2

The owner approved the complete review recommendations on 2026-09-26. [ADR-0012](decisions/ADR-0012-approved-review-closure.md) records the accepted defaults and remaining concrete inputs. The [governing sequence](implementation/COMMIT_SEQUENCE.md#approved-execution-cycles--revision-2) now names EQM-P01–EQM-P15; all 211 implementation steps remain pending. [RA01–RA21](implementation/review_regressions.json) supply precise regression contracts, and [DEV-001](implementation/DEVIATIONS.md) records the S049 risk/finalization amendment.

This is a clearly identified revised package, not an unchanged original delivery. The revision-1 checksum-list digest is retained in provenance; current SHA256SUMS and PACKAGE_VALIDATION.json describe revision 2. Planning approval does not supply undisclosed operational values, activate native embedding or authorize live enforcement.
