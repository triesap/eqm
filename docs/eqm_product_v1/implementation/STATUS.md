# Verification and implementation status

Imported plan revision 2; current amendment DEV-003, 2026-09-27.
Reviewed source HEAD: `7d44724abba2fa90baf806563da0a692c5bb5857`.

## Current implementation

S001–S014 are recorded complete. S015 is **implementation-complete, pending
supervisor acceptance** of the new HEAD: corrective commits R1–R9 exist and
R6–R9 closed RF11–RF18 with literal-input, fake-provider and real-validator
controls. S016 is the next step and is dependency-unblocked. R1–R9 receipts are
retained; no history is rewritten and no accepted repair is discarded.

v1 integer acceptance is restored; 22 v1 schemas remain unchanged; five v2
protocol families, version identity, exact integer endpoints, canonical runtime
time/null handling, exact version/kind reader isolation, nested inventory
validation and end-to-end runtime-facts 16 MiB/100000-fact bounds with retained
v2 subject checks are present, and the five v2 schemas constrain exact markers
with a minimal AutoSi pattern verified by a real Draft 2020-12 validator.
Remaining defects involve regression controls that bypass their claimed
boundary and the evidence/ledger reconciliation. See the
[review](reports/CODEX-7d44724.md) for all findings and earlier dispositions.

EQM-P01 has 12 numbered steps recorded complete. EQM-P02 has S013–S014 complete
and S015 implementation-complete pending acceptance; P03–P15 are pending.
Corrective checkpoints retain S015 traceability and do not increase the 211
numbered steps.

Codex owns scope, contract, sequence and acceptance decisions; Pi implements.
S015's objective R6–R9 gates passed, so S015 is recorded implementation-complete
and dependency-unblocked work continues from S016 while supervisor acceptance of
the new HEAD remains separately pending. No stop at S143 or invented approval
gate. Existing external-evidence prerequisites and optional-native boundaries
remain.

## Independent verification and limits

At the reviewed HEAD, `cargo xtask check`, application doctests and the CLI build
passed. The actual-ingress CLI probe nevertheless accepted unsupported v2
workspace/lock markers. Original v1 schemas are byte-identical. Pi's historical
per-checkpoint command claims were reviewed; separate raw R1–R5 logs were not
located, so they are not independently attested. Current-HEAD logs are retained
externally. Planning verification is recorded with the supervisor receipt.

AC001–AC064 and RA01–RA21 are not closed merely by test counts or references.
Extended verification, Linux x86_64 execution, real product acceptance and
operational authorization remain unverified. Initial execution/storage targets
remain macOS arm64 and Linux x86_64. This is not a release candidate; human
release testing is not the next action.

## Governing artifacts

COMMIT_SEQUENCE.md is the sole governing sequence; steps.json and chapter files
are synchronized views. progress.json records live state; DEVIATIONS.md records
DEV-001–DEV-003. Keep original delivery hashes unchanged. Use the structure-only
package validator for intentional implementation/planning updates.
