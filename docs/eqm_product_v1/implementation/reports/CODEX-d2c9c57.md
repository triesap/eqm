# Independent review — d2c9c57

Reviewed clean `d2c9c57d2e5d3c8f448426619a49c48af45b211a` on 2026-10-03.
All seven requested checkpoints were committed and S043 was reached. Reaching
the checkpoint does not establish its required independent acceptance.

Preserve S001–S038 acceptance and all returned commits. Accept S040 provider
authority and S042 synthetic fixture for allocated scope: **40 numbered steps
accepted**. RF51 lossless runner/scope tags and RF52 typed nested graph/comparison
schema structure close for allocated scope. S039/S041/S043 remain retained with
changes required; RF45/RF53 integration remains partial. Full original criteria,
RA02/RA13 and external gates remain open. No supervisor product changes were made.

## Findings

**RF55, P1 — comparison eligibility does not enforce scenario coverage or matching.**
The engine ignores `ComparisonParticipant.occurrence`, has no expected occurrence
set, and accepts duplicate/undeclared occurrences. Application
`checked_comparison_eligibility` just forwards caller-supplied public context/
subject digests. It neither binds checked context to graph-derived authority nor
requires subject-matching evidence. An isolated engine probe proves two fabricated
identical undeclared occurrences are Eligible. Existing application tests manually
label independently matched phone subjects as different scenario participants;
the match result is discarded. Exact scenario coverage and independently matched
subject-kind/configuration/content authority are required at the real boundary.

**RF56, P1 — cohort assignment forms silently drop frozen configuration axes.**
S041 domain, authored DTO, graph DTO, schema and projection omit
target_configuration_digest/provider_configuration_digest. DEV-012/DEV-014
explicitly reuse S038 assignment forms. Deriving configuration from the graph
does not authorize deleting the authored fields; retain the claims and compare
them to independent graph-projected authority. Current ConfigurationKind error
is unreachable and advertised configuration matching is actually substitution.

**RF57, P1 — cohort validation is detached from preparation and graph membership.**
The resolver never checks cohorts; `prepare` returns a finalized session with a
cohort referencing a nonexistent scenario. An isolated real fixture probe proves
this. `resolve_cohort` accepts an arbitrary Cohort absent from graph.cohorts(); a
second probe proves caller-created undeclared authority becomes expected context.
Validate every loaded cohort before prepared-session success and require exact
graph-owned cohort identity/value at authority consumption. Reject missing,
ambiguous scenario revisions, missing/extra occurrences, incomplete axes and
configuration mismatch. Candidate matching must use retained independent authority.

**RF58, P2 — checked comparison ingress accepts domain-invalid definitions.**
The reader validates profile IDs/revisions and selected dimensions, but does not
enforce full Profile construction invariants. A probe supplies empty owners and
empty title; immutable checked conversion succeeds. Validate exact profile
definition owner/title/dimension/default/description/extension invariants using
owning domain rules, with positive/negative controls and bounded recursive values.
Deserialization remains non-authorizing even after structural checks.

**RF59, P2 — authored provider/cohort schema parity was narrowed instead of fixed.**
New authored schema markers are arbitrary strings, provider SHA values are
unconstrained, owner sets permit empty values, and cohort optionals permit null.
A real Draft 2020-12 probe accepts wrong schema, empty owners and garbage digest.
S043's report describes removing SHA/null negative assertions as an over-strict
test. Restore the approved value contract in owning generated schemas, preserving
separate raw-token/cross-field validation layers. v1 schemas stay unchanged.

**RF54 recurrence — current status and historical green evidence need reconciliation.**
STATUS still says S040–S043 unstarted, although all were committed; old review
and cursor metadata remain. P03-R2 report was added after its cited standard gate
and committed with a forbidden private-tooling marker; P03-R3 fixed it. The
earlier tree was not independently green. Preserve that historical discrepancy
and failed evidence honestly; finalize every report/status before its final gate.

## Verification and limits

Independent standard `cargo xtask check` passed **693 successful test executions**;
changed domain/manifest/protocol/engine/application doctests passed **9**. Package
structure validation passed. Five isolated defect probes passed by asserting
the defective behavior exists: fabricated comparison coverage, invalid checked
profile, permissive provider schema, undeclared cohort authority and successful
preparation with dangling cohort scenario. Copied fixture tests also passed;
those additional successes do not expand acceptance.

Native branch and retained final clean-HEAD logs corroborate reaching S043;
model/thinking and actual working directory were confirmed. Intermediate failures
and workload contention remain historical, not silently counted as passes.
Only a dev dependency on the existing pinned local-only validator was added;
v1 schema diff is empty. No product code/tests changed in the production checkout
by the supervisor; review probes are isolated.

Full verify, current supply-chain, Linux, coverage/mutation/fuzz/benchmark/package,
live product/crypto/host/signing/operational evidence remains unverified. No human
release-testing milestone is reached. DEV-015 completes required P03 integration
before independent acceptance and S044; full MVP S211 remains.
