# Independent review — 8e5f218

Reviewed clean `8e5f21867bdaa7ccf32c4c2d626b2f269184380e` on 2026-10-03.
All four requested checkpoints and a ledger backfill were committed. The
implementation agent correctly stopped at the required independent P03 gate.
Preserve all five commits and 40 prior numbered-step acceptances. The gate is
**changes required**, not passed; S039/S041/S043 remain retained implementation.

## Accepted progress

RF56 closes for allocated scope: authored configuration claims are restored in
domain/manifest/protocol/projection/fixture forms, exactly one kind is enforced
by domain construction, and resolution compares claims to typed graph projections.
RF58 closes for allocated scope: checked comparison ingress constructs the owning
Profile, validating owner/title/dimension/default/description/extension invariants.
RF55 occurrence coverage and bytes-first matching improve substantially; RF57
exact membership at resolution and validation of every cohort during preparation
are implemented and verified. Both remain partial because consuming comparison
can substitute independently resolved authority. RF59 exact markers, SHA patterns,
nonempty collections and absence constraints improve, but value parity remains
partial. RF45/RF53 and whole RA02/RA13 remain open. No whole acceptance case closes
from these allocated contributions.

## RF60 — P1: resolved cohort is not bound at comparison consumption

`crates/eqm_application/src/comparison.rs:158` accepts a private ResolvedCohort,
but only compares the matched occurrence names to the selected scenario. It does
not require the carrier's scenario ID/revision to equal that scenario, look up the
carrier's exact cohort in the consuming graph, or revalidate its expected axes
against that graph. `crates/eqm_application/src/cohort.rs:68` retains identities and
expected axes but not the exact cohort value used at resolution. Private fields
prevent mutation, not reuse of independently resolved authority in another context.

Two isolated tests through the real application API reproduce Eligible for:
- a cohort resolved from a foreign graph while the consuming graph has no cohorts;
- a same-graph cohort assigned to `other.flow` consumed as `signup.flow`, whose
  occurrence names happen to match.

These violate existing graph/scenario-bound independent authority criteria.
DEV-017 S039-R3 settles exact value/context revalidation without a new wire digest,
pointer identity requirement or trust promotion.

## RF61 — P2: generated value schemas still admit domain-invalid inputs

Real local Draft 2020-12 validation with format assertions accepts six invalid
cohort variants: revision 0, empty ID, invalid scenario ID, malformed source commit,
and assignments with neither or both configuration kinds. The manifest generator
still uses unconstrained strings/u64 and emits neither exact-one constraint.

The comparison schema accepts empty title, revision 0, empty dimensions and invalid
owner, while its checked reader rejects each. The restored schema tests cover only
a subset of the settled DEV-015 value contract. RF59 is therefore partial, not
closed. DEV-017 P03-R5 requires the full owning vocabulary/range/set/absence audit,
including exact-one configuration constraints and clear separate token/logical-key/
semantic validation layers. Preserve valid inputs and v1 bytes.

## Tracking and verification

The returned plan's execution-revalidation paragraph still says no run/delta, while
Pi's STATUS reports completion. progress retains first_unfinished_original_step
S032 and supervisor_amendment DEV-012. Current frontiers are reconciled by Codex;
prior receipts remain historical. Future reports/status precede their final gates.

Independent `cargo xtask check` passed **704 successful test executions**, not
704 distinct tests. Changed domain/manifest/protocol/engine/application doctests
passed **9**. Package structure and diff checks passed. Four isolated tests prove
defective acceptance, rather than product correctness; an initial comparison-probe
compile failure used a BTreeSet as a Vec and was repaired only in the isolated probe.
No production code/tests were edited by the supervisor. v1 schema and Cargo.lock
diffs are empty. Native branch confirms actual EQM commands, configured model and
legitimate required-review stop; failures/repairs remain retained evidence.

No release or human-testing milestone is reached. Full verify, Linux, coverage,
mutation, fuzz, benchmark/package and live product/crypto/host/signing/operational
acceptance remain unverified. DEV-017 completes all three integrated checkpoints
before another independent P03 review; S044 and full original MVP S211 remain.
