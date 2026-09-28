# Commit-by-commit implementation sequence — eqm_product_v1

## DEV-005 — S016 review closure and binding projection decision

**Date/author:** 2026-09-28, Codex supervisor under owner delegation.
**Reviewed source:** `68378317044d49a121caff07e783573abdf3cd56`.
**Disposition:** S016-P1 (`ed06989`) is accepted; RF19 is closed. S016's
implementation commit is retained, but the numbered step is reopened for RF20–RF22.
S001–S015 remain complete. The next checkpoint is **S016-R1**, then S016-R2,
then dependency-unblocked S017–S211. This amendment supersedes older cursors and
S016 completion claims, not the retained representation or authority contracts.
See [the source and evidence review](reports/CODEX-6837831.md).

### Findings and independently green repairs

| Checkpoint | Required work | Verification and commit |
| --- | --- | --- |
| S016-R1 | Close RF20/RF21: shared structural validation of authored v2 fields using the field/type mapping below; call it from every existing owning `from_json` path, including embedded inventories. Retain S015 bounds, versions, vocabulary and temporal checks. Add valid controls and one-field negative mutations for each mapped field family. | S016's complete declared commands, schema generation/check if affected, actual package structure validation and diff check. Commit `fix: validate authored v2 metadata at ingress`. |
| S016-R2 | Close RF22: execute the missing set/logical-key/sequence/text controls at real v2 roots, add independent literal runtime-facts canonical bytes/digest, and prove raw-byte preservation separately. Reconcile reports, exact SHAs, progress, cycle counts and current status. | S016's complete declared commands plus the evidence discipline below. Commit `test: complete v2 collection boundary regressions`. |

Retain both existing commits; no amend, reset, renumbering, new product subsystem,
weaker assertion, or successful receipt for a failing command. R1 carries its own
behavioral regressions; R2 closes the remaining matrix rather than postponing all
R1 tests. Each checkpoint is independently green. After R2 passes, Pi records
implementation completion with supervisor acceptance pending and continues S017;
there is no additional user-approval gate. S016 is not complete merely because
CanonicalSet or an isolated NFC predicate passes tests.

### Authored field/type mapping (RF20/RF21)

Reuse the existing domain syntax/bounds, adding no v1 reader changes. NFC alone
is insufficient: `café` is valid SelectorText but is not a valid UnitId. Tests
must use valid typed IDs; demonstrate composed Unicode with a text field whose
existing grammar permits it. Validation rejects rather than normalizes.

| v2 field family | Required structural contract |
| --- | --- |
| Evidence target/unit/requirements/facets | TargetId, UnitId, FullRequirementId, Facet respectively, including existing vocabulary/grammar. |
| Shared subject scope | TargetId for target and each target-set member; ProviderId for provider. Retain existing nonempty scope rules where the corresponding domain constructor requires them. |
| Shared subject metadata | RepositoryIdentity, SourceCommit and digest syntax; optional build_id uses SelectorText. Share existing runtime-facts checks across the other owning roots without inventing relationship/trust acceptance. |
| Profile values in evidence and runtime facts | ProfileId, positive Revision, DimensionId, SymbolicValueId; retain unique (profile, dimension) and consistent per-profile revision. |
| Manual review | OwnerRef reviewer and existing outcome vocabulary; do not treat reviewer as an arbitrary NFC string. |
| Attachments | SelectorText name/media_type and existing digest/size forms; preserve name uniqueness. Attachment content is opaque bytes, not either metadata string. |
| Inventory, adapter request and response | AdapterId, TargetId where present, the shared subject checks where present, and shared embedded inventory validation. |
| Inventory entries / fact symbolic metadata | SelectorText kind/key/attribute names; SymbolicValueId for Symbol values; retain SurfaceId/DimensionId/SelectorText rules for runtime-fact keys and existing restrictions on typed Text values. Source path uses its existing RepoPath contract. |
| Selector helper | Existing EvidenceSelector field types (SelectorText, HttpMethod, ReleaseChannel as applicable), not NFC-only checks. No current v2 root owns this helper: mark its consumer integration pending until the first owning v2 selector ingress exists; wire validation there. Do not invent a schema family to manufacture a consumer. |
| Diagnostic unit/target references | Existing UnitId/TargetId syntax when present; diagnostic prose and source-export content do not acquire blanket authored-text NFC rules. |

Do not apply blanket NFC checks to observed text, diagnostic messages, source
exports or attachment bytes. Existing domain fields explicitly modeled as
SelectorText retain their own bounded/NFC rules; they do not redefine arbitrary
observed bytes as authored metadata. Opaque request identifiers, filesystem roots
and location URIs retain their existing contracts, not invented identifier types.
Representation validation remains distinct from S025/S059/S061 relationship,
trust or signature acceptance. A v2 record passing these checks is non-authorizing.

### Regression closure and evidence discipline (RF22/RF23)

Exercise subject target sets, requirements, facets, attachments, profile values,
runtime facts and inventory entries through their actual owning `from_json`
methods. For each applicable family show valid permutations produce identical
canonical bytes and exact duplicates reject. Show differing records with the
same logical key reject for attachment names, profile/dimension, facts and
inventory; separately test conflicting revisions for one profile. Cover embedded
adapter-response inventory as well as standalone inventory. Mutants retain all
unrelated fields and recompute a required digest so an unrelated error cannot
make a negative test pass.

A deserialize-only execution payload with attempt numbers `[2,1,2]` does not
prove valid evidence ingestion. Test valid numbered attempts at EvidenceResult
root; reordered/duplicated indices reject under the existing index rule. Show
order/multiplicity preservation for valid diagnostic and related-location lists,
including different canonical outputs after reordering. Do not require invalid
attempt numbering to pass merely to demonstrate sequence multiplicity.

Keep the escaped-string/object-member ordering traps. Add a reviewed literal
canonical runtime-facts byte string and fixed expected digest; the expected value
must not be produced by a test copy of the production sorting algorithm. Retain
permutation controls for both facts and profile_values. Exercise strict ingestion
on an actual raw JSON input with formatting/escaping and verify the input bytes
and raw identity remain exact while its semantic view canonicalizes separately.
Use RawObjectDigest for literal decomposed UTF-8 and non-UTF-8 opaque vectors;
compare fixed expected bytes/domain hashes. No storage/attachment subsystem is
required now. A frozen-vector byte-literal decoder must reject malformed or
truncated escapes rather than silently discard characters. Passing arbitrary
non-UTF-8 through a JSON parser is not an opaque-byte preservation requirement.

RF23 is a historical receipt discrepancy: the saved S016 package-structure log
says FAIL while the receipt says exit 0; current independent validation passes.
Codex has corrected the current report/status and recorded the distinction.
Retain original logs. For subsequent checkpoints record each command's real exit
status and unique log name, run structure validation after the final ledger/doc
edit and before committing, and do not infer a command's result from the last
command in a shell sequence. STATUS.md, progress.json and checkpoint receipts
must agree. A later green run is new evidence, not retroactive proof of the
historical receipt. Do not claim that every historical committed tree was green.

### S017 decision: separate complete v2 binding projection

The existing `eqm_manifest::canonical::binding` is a useful field inventory,
not a v2 wire implementation. Hashing its unchanged v1 JSON under BindingDigest
would retain numeric and collection encodings inconsistent with ADR-0013.
Implement a separately named v2 binding projection in eqm_manifest, with immutable
canonical bytes and the existing BindingDigest domain. Keep the v1 binding,
fragment and graph APIs/bytes/hashes unchanged. No new digest domain or dependency
from the manifest layer to protocol/transport is approved or needed.

Include all existing binding semantics: id, revision, target, unit, owners,
artifacts (including selectors), exposures (surface/state/applicability/route),
evidence specifications (selectors, requirements, facets, runner references,
counts/freshness) and semantic extensions at every existing level. Apply approved
v2 integer wire rules to revisions/counts/freshness and extension integers.
Apply full canonical-member byte ordering to owners, artifacts, exposures,
evidence specifications, requirements, facets and applicability membership
values. The remaining ordered expression/extension arrays preserve order; do
not recursively sort every array. Reject exact duplicate set members rather than
silently erasing them. Existing typed construction retains logical uniqueness.

Exclude only the already defined `dev.equivalencematrix.display` extension
namespace wherever the existing projection excludes it. Comments, whitespace,
source spans and the manifest file's own location are parse provenance, absent
from Binding. An artifact's declared path is semantics and remains included.
Do not invent a Binding.description/comment field or exclude other extension
namespaces. Test real display-extension changes and comment/formatting changes,
plus same-ID selector/path/constraint/semantic-extension mutations, set
permutations and large integer boundaries with literal byte/digest expectations.

S017 delivers and tests this component projection; the later owning v2
construction/relationship steps integrate it. Do not silently replace v1
transport/runner identities or claim end-to-end evidence binding now. S018 uses
the same decided representation rules for its separate specification identity.
The complete cross-domain corpus remains S027. This resolves the handback's
scope decision; ordinary function/module layout remains Pi's implementation choice.

### Continuation and authority

Proceed in the sole governing sequence through all dependency-unblocked work to
S211; S143 is intermediate. Codex owns new scope, representations, sequencing,
license exceptions and acceptance decisions. Pi may choose internal mechanics
within this contract and maintain factual reports/progress; it may not change
this scope, self-waive a gate or invent missing external facts. Newly evidenced
contract decisions return to Codex. Preserve Q01–Q15 at their first dependent
steps, keep one active step, checkpoint before context exhaustion, and use
available compaction. A real session limit is INTERRUPTED, not task completion.
No publication, deployment, signing, live enforcement or optional-native
activation is authorized. Human release testing is not the next dependency.

## DEV-004 — Accept S015 and resume canonical collection work

**Date/author:** 2026-09-28, Codex supervisor under owner delegation.
**Reviewed source HEAD:** `121622ff61a18efb0afd1dc9a13264fc2a97f259`.
**Disposition:** S015 and R1–R9 are accepted for their allocated implementation
scope. RF01–RF18 are closed for that scope; later relationship/trust/evaluator
acceptance remains with its original owning steps. S001–S015 are complete.
The [review](reports/CODEX-121622f.md) records independent verification and the
remaining dependency-policy finding RF19. No release readiness is implied.

DEV-002/DEV-003 representation, authority and continuation decisions remain
binding except their obsolete active cursors and pending S015 acceptance.
Preserve all 211 numbered step IDs and the sole governing sequence.

### Dependency decision and prerequisite

Codex approves the exact-pinned dev-only `jsonschema` 0.48.5 dependency with
its disabled default retrieval features and the documented Zlib license
allowance for the reviewed dependency graph. Local manifests/license text,
resolved features, live advisory audit and license/source checks were reviewed.
This does not authorize future license-policy exceptions by Pi. The additional
locked packages are test tooling, not an approved new production dependency.

**RF19:** `cargo deny check` fails because eqm_engine's eqm_test_support path
dev-dependency has no version. This originated in S005 and predates R6–R9;
fix it now rather than waive it or wait for the release cycle. The actual
command surface has no `cargo xtask supply-chain` subcommand. Use `cargo audit
--deny warnings` and `cargo deny check`; `cargo xtask verify` owns the combined
extended release lane. Do not add a new command solely to fit an inaccurate
handback command name. Advisory availability was demonstrated in this review.

| Checkpoint | Scope | Definition of green |
| --- | --- | --- |
| S016-P1 | Repair RF19 as a prerequisite to S016: add `version = "=0.1.0"` to the existing eqm_engine path dev-dependency on eqm_test_support. Preserve package identity, path and dependency direction. | Locked workspace check, full `cargo deny check`, `cargo audit --deny warnings`, standard `cargo xtask check` and `git diff --check` pass. No wildcard/duplicate/advisory threshold is weakened. Record any future live-DB outage distinctly; do not convert it into a successful audit. |

One small green commit for S016-P1, using `fix: pin the engine test support
dependency`. No application source changes are needed in this prerequisite.
Record it under S016 without marking the numbered S016 complete. Then execute
S016 itself and every dependency-unblocked successor through S211. An existing
allowed duplicate-version warning is not a waiver or a new failure.

### S016 decisions resolved before delegation

The existing S016 is enforcement work, not merely adding more catalog strings.
Its already approved set/sequence/text rules require real v2 consumers and
executable controls. Preserve v1 schemas, readers, canonical bytes and hashes.
Do not change the existing v1 graph/fragment projection in place. Introduce
reusable v2 canonical collection/text boundaries now; later component-projection
steps consume them without silently reinterpreting v1.

For the five allocated v2 families, use this classification:

| Collection | Semantics |
| --- | --- |
| Subject target sets; evidence requirements, facets and attachments | Unordered sets. Reject duplicate values before any BTreeSet/map collection can erase them; retain existing attachment-name uniqueness. |
| Evidence/runtime profile_values; runtime facts; inventory entries | Unordered semantic records. Canonicalize by each complete member's canonical JSON UTF-8 bytes, preserving existing logical-key uniqueness independently of ordering. Profile/dimension duplicates or conflicting revisions within one profile, duplicate fact (surface, dimension, key), and duplicate inventory (kind, key) do not become multiple valid records. |
| Execution attempts, diagnostic lists and related-location lists | Ordered sequences. Preserve input order and multiplicity; retain existing attempt-index validity checks. Do not sort/deduplicate them through a generic collection helper. |
| Object maps, including inventory attributes | Strict duplicate-member ingestion plus existing JCS object-key rules, not an array-set sorting algorithm. |

Accept permutations of a valid semantic set and emit the same canonical
representation; reject duplicate members rather than silently collapsing them.
Sort by full canonical member bytes, not Rust derived Ord, field insertion order,
a selected key tuple or unescaped string ordering. Separate a semantic record's
uniqueness key from its canonical ordering key. Keep original received bytes for
raw identity/signature use; canonicalization produces a distinct semantic view.
Runtime-facts digest validation must use the approved canonical semantic set
projection while preserving the original bytes and all S015 numeric/time/bound
checks. Do not redefine digest domains or introduce later trust authority here.

Authored metadata follows ADR-0013's NFC rejection rule. Reuse existing typed
validators and apply the rule to v2 equivalents of authored identifiers, selector
metadata, profile/symbolic values and reviewer metadata; preserve their existing
syntax constraints. This does not authorize blanket NFC validation of observed
payload text, source exports, diagnostic text or attachment bytes. Keep opaque
payload bytes exact, including decomposed UTF-8 and arbitrary non-UTF-8 bytes.
Where existing types impose additional restrictions, preserve them rather than
broadening a v1 reader. Document the field classification and test the actual
v2 boundaries, not only a standalone demonstration helper.

Resolve the frozen vector notation explicitly: authored-text vectors carry JSON
string input and canonical JSON output. In RV-TEXT-2, escaped `caf\u00e9` decodes
to accepted NFC text and canonical output contains literal UTF-8 `café` between
JSON quotes. Pi is authorized to correct that expected output and its explanatory
note, retaining the vector ID and recording the change. This changes the
representation of the same accepted text, not Unicode normalization. Opaque-byte
vectors need a distinct execution discipline and literal byte/hash expectations;
they must not be passed through the authored JSON-string validator. Add decomposed
and non-UTF-8 controls. JCS string serialization and original bytes are different
representations; see [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785.html#section-3.2.2.2).

S016 must execute every set/sequence/authored-text/opaque vector by explicit
category/discipline, compare exact outputs, test permutations and duplicates at
real nested v2 ingress, and prove order sensitivity for sequences. Include cases
where escaped characters and object field ordering distinguish canonical-byte
ordering from incidental Rust order. Regenerate/check affected v2 schemas and
keep the 22 v1 documents byte-identical. No new placeholder schema family is
required. S027 retains complete cross-domain hash/provenance corpus closure.

### Continuation and evidence

The next session begins S016-P1, then S016–S211. S143 remains intermediate.
Codex owns changed scope/representation/sequence/license/acceptance decisions;
Pi may choose ordinary internal mechanics within these settled rules. Keep one
active step, small context working sets, available session compaction, and a
verified report/log/commit checkpoint after each slice. Do not stop voluntarily
at the end of S016 or P02 when continuation is possible. An actual context limit
is INTERRUPTED; an unavailable required external fact is a dependency blocker.
Preserve Q01–Q15 gates at their first dependent step and do not invent product,
cohort, policy, credential or host facts. No deployment/publication/signing or
optional-native activation is authorized.

Model reporting must distinguish configured defaults from observed runtime
selection. Record the actual session identity if exposed; otherwise say runtime
identity is unverified. Do not silently switch models. No need to expose unrelated
agent state or credentials to establish this fact.

## DEV-003 — Close version ingress and prove S015 conformance

**Date/author:** 2026-09-27, Codex supervisor under owner delegation.
**Reviewed source:** `7d44724abba2fa90baf806563da0a692c5bb5857`.
**Disposition:** S015 remains in progress, changes required. R1–R5 commits are
retained; the next checkpoint is S015-R6. This amendment takes precedence over
DEV-002 only where it clarifies conformance evidence and continuation status.
All other DEV-002 decisions remain binding. Preserve the 211 numbered steps.

The independent [review](reports/CODEX-7d44724.md) records RF11–RF18.
A green standard gate did not detect v2-labeled workspace/lock input being read
with v1 semantics, missing nested inventory validation, repeated smaller byte
bounds, schema/runtime differences, or tests which bypass their claimed boundary.
These are unfinished S015 obligations, not new product scope.

### Decisions settled by Codex

1. **Identity versus supported reader.** `SchemaUri` may represent a known
   kind/version pair without promising a generated schema or a supported reader.
   Every ingress must enforce its exact supported version and kind. Existing
   workspace, lock and authored-manifest readers accept v1 only until their
   owning integration step. Keep CURRENT at v1. Audit every SchemaUri consumer;
   a kind-only check is insufficient. No placeholder v2 manifests or fallback.
2. **Nested structural validation.** A v2 adapter response must validate the
   nested inventory's v2 marker and all structural checks already required by
   standalone InventoryV2Dto ingestion, including digest syntax and bounds.
   Also validate the response's own digest syntax. Reuse validation without a
   weaker deserialize-only route. This does not confer trust or pull the later
   relationship/verification/evaluator steps into S015.
3. **Bounds are end-to-end.** Keep the existing per-family byte limits at every
   parse, including canonical-digest preparation. v1 and v2 runtime facts retain
   the declared 16 MiB envelope and 100000-fact ceiling. Preserve v1's existing
   subject syntax checks in v2 runtime facts as well: repository identity,
   repository/configuration digests, source commit, optional build ID and artifact
   digest. These are existing structural checks, not later trust semantics.
   Do not globally raise
   the default strict parser's 8 MiB limit. Test actual valid records, including
   computed digests, padded beyond 8 MiB, at 16 MiB and one byte over; test fact
   count at/above its cap. Preserve the completed release bound correction.
4. **Schema assertions.** All five v2 schemas must constrain each instance's
   `schema` member to its exact family/version URI, including embedded inventory.
   `$id` alone is not this constraint. The UTC pattern must reject zero and
   redundant fractional groups (including `.000Z`, `.123000Z`, `.123456000Z`)
   while accepting canonical 0/3/6/9 groups. Use date-time format assertions in
   the conformance validator for calendar validity, in addition to the v2
   canonical pattern. Raw parser/calendar validation remains mandatory.
5. **Three distinct validation layers.** JSON Schema validates JSON values,
   not original number-token spellings: 1.0 and 1e3 are integral values. Raw v2
   parsing must still reject those tokens, negative zero and duplicate members.
   Keep raw-byte/token, schema-value and semantic-validation test expectations
   separate. This corrects DEV-002's overbroad schema/parser agreement wording;
   it does not relax the v2 wire contract. Cross-field digest/trust semantics
   remain runtime responsibilities and must not be attributed to JSON Schema.
   See [Draft 2020-12 validation](https://json-schema.org/draft/2020-12/json-schema-validation#section-6.1.1).
6. **Real schema validation is authorized.** Add an exact-pinned, toolchain-
   compatible Rust `jsonschema` dev dependency, with Draft 2020-12 and explicit
   format assertion support. Select the exact compatible release as an ordinary
   implementation detail, record its version/features/license/lock changes and
   use only local schema resolution with network retrieval disabled. No runtime
   dependency is authorized by this decision. Replace the bespoke pseudo-validator;
   do not reinterpret standard integer semantics to force test agreement.
7. **Evidence must exercise the claimed input and boundary.** Absence-vector
   tests must consume each literal input and compare its declared output; never
   synthesize the test input from expected success/failure. Validate exact
   category/discipline mappings. Test provider over-return through read_evidence
   with a fake reader (exact limit, one over, missing reader, propagated error,
   no runner execution), not only the comparison helper. A reconcile control
   must successfully load a valid locked local inventory; its duplicate-member
   mutant must fail at the same ingress. Another unrelated error is not success.
8. **Continuation is preauthorized, acceptance is separate.** Complete R6–R9
   in order, then continue S016–S211 while dependency-unblocked. Once the
   objective repair checks pass, Pi may mark S015 implementation-complete and
   proceed, recording supervisor acceptance as pending review of the new HEAD.
   Do not stop solely to request that already-authorized transition. Codex
   reviews at each session handback; Pi cannot approve itself, change these
   decisions, waive gates or skip blocked predecessors. S143 is intermediate.
   A new material scope/contract conflict needs a concrete Codex decision;
   routine mechanics and pinned dependency selection within decision 6 do not.

### Remaining corrective commits under S015

Each row is one independently green commit, with report and ledger updates.
Use the existing S015 report with real stable anchors and exact commit receipts.

| Checkpoint | Scope | Required evidence and dominant lane |
| --- | --- | --- |
| S015-R6 | RF11/RF12: enforce reader version/kind and nested inventory structural validation. | Successful v1 workspace/lock controls; changing only the URI to v2/v3/wrong kind rejects through actual preparation/CLI. Valid records for all five v2 readers pass; root and nested version mutations reject; nested invalid digest rejects both standalone and embedded. Manifest/protocol/CLI tests, full standard gate. |
| S015-R7 | RF13/RF18: repair runtime-facts bounds and retain fact-count/subject structural validation. | Actual digest-valid records pass beyond 8 MiB and at 16 MiB; one byte over fails with size error; 100000 facts pass where otherwise valid, 100001 reject. Valid subjects pass; one-field malformed repository/commit/digest/build-ID mutants reject after recomputing the enclosing facts digest. Audit all family parse bounds; protocol tests and full standard gate. |
| S015-R8 | RF14/RF15: correct generated schema constraints and use standards-conforming validation. | Regenerate five v2 schemas atomically; 22 v1 schemas byte-identical. Full valid fixtures and single-field mutants for all families run against committed/generated schemas, including nested markers/null, integer endpoints/string ranges, canonical times and invalid calendars. Separate lexical-only and cross-field expectations. Protocol/domain schema tests, schema parity and full standard gate. |
| S015-R9 | RF16/RF17: replace tautological controls and reconcile evidence. | Literal absence inputs/outputs execute; exact discipline mapping; successful reconcile control and duplicate mutant; fake-reader integration succeeds/fails at real boundary. Run application doctests and all S015 tests; correct report claims, anchors, cursor and per-checkpoint receipts. Package structure and full standard gate. |

Full standard gate is `cargo xtask check` plus `git diff --check`. Each checkpoint
also runs its targeted tests; regenerate/check schemas whenever their source
changes. R9 runs S015's complete declared command set. Retain raw command logs
outside the checkout and report their paths externally. Do not rewrite original
package delivery hashes or amend reviewed commits. Do not write implementation
code through the supervisor: Pi remains the implementation agent.

S016 retains canonical sets/NFC/opaque-byte work; S025/S059/S061 retain relationship
acceptance, S027 hash-corpus closure, S069 freshness arithmetic, and later owning
steps the rest of the v2 consumer migration. External product/host/policy gates
remain effective at their first dependent step. Local tests do not establish
release readiness; no publication, deployment or optional-native activation is
part of this amendment.

## DEV-002 — Supervisor-owned S015 completion and continuation

**Date/author:** 2026-09-26, Codex supervisor. **Status:** owner-delegated
engineering decision; implementation pending. **Reviewed HEAD:**
`3da0b50aeb7cf7f8b23f8f5ebd47f043892ed00b`. **Affected steps:** S015 plus
corrective regression contributions to S006/S011/S012/S014; S016 and all later
steps retain their IDs and order. **Contracts:** R18/R19/R21/R52/R64 and existing
step-linked acceptance/regression clauses. No new product subsystem is added.

**Evidence:** the supervisor report `implementation/reports/CODEX-3da0b50.md`
records RF01–RF10: v1 acceptance changed despite frozen schemas, v2 DTO/schema
integration and absence enforcement are missing, integer endpoints/canonical
forms disagree, release ingress repeats the smaller parser bound, and the
bounded read helper trusts over-returning providers. S015 already requires
representation enforcement and atomic schema updates. Treating those as
unallocated future work is unsafe. Null/absence belongs to S015, not S016.

**Authority and responsibility:** the owner delegates decisions needed between
coding periods to Codex. The supervisor changes scope, sequence and normative
contracts; the implementation agent performs code generation, verification and
evidence/status updates. It may choose ordinary internal mechanics that obey
this contract, but may not change representations, defer required work, invent
an additional step, reorder dependencies, waive gates or claim acceptance to
get past a failure. A newly evidenced conflicting obligation returns to the
supervisor with a concrete decision request. Approved decisions below need no
reconfirmation.

### Binding engineering decisions

1. Preserve the pre-S015 v1 accepted integer ranges, existing DTO behavior,
   schema bytes and signed originals. Revert only S015's new v1 numeric-range
   restrictions. Retain bounded duplicate-rejecting ingestion as the approved
   security correction. Do not add decimal strings to v1 or use v1 URIs for v2.
2. v2 unsigned quantities use `u64` (0 through 18446744073709551615); signed
   facts use `i64` (-9223372036854775808 through 9223372036854775807).
   JSON integer tokens are used exactly within [-9007199254740991,
   9007199254740991], intersected with the semantic type's range. Only values
   outside that interval use decimal strings. Reject small-value strings,
   leading zeros, plus signs, negative zero, fractional/exponent tokens and
   overflow. Every supported value must serialize and deserialize identically.
   Positive counts further require at least one. This policy is not a claim
   that every integer beyond 2^53-1 is individually unrepresentable as a float;
   it fixes an interoperable safe interval.
3. v2 time uses exact nanosecond UTC, a literal Z, four-digit years 0001–9999,
   and the minimal AutoSi group of 0/3/6/9 fractional digits. Reject leap seconds,
   offsets, missing Z, out-of-range years, redundant fractions and unsupported
   precision. A v2 validated wrapper must not silently change the retained v1
   parser. Absence means omitted members; explicit null rejects recursively
   for optional v2 members. Preserve original bytes before interpretation.
4. Establish explicit version-qualified schema/DTO ownership. v1 APIs remain
   unchanged. v2 protocol DTOs live in `eqm_protocol::v2`; schema identity must
   include the explicit version, never a fallback or a changed v1 CURRENT alias.
   `WireUint`/`WireInt` are v2 primitives, not validation adapters for v1.
5. The first concrete v2 schema inventory, owned by S015, is five protocol
   families: `evidence-result`, `runtime-facts`, `inventory`, `adapter-request`,
   `adapter-response`, each at `schemas/v2/protocol/<name>.schema.json` and the
   matching approved GitHub raw URI. Include their nested DTOs and full existing
   fields, with v2 numeric/time/absence validation. Do not invent later campaign,
   relationship, storage or verification semantics in this checkpoint. Raw
   claims cannot become validated authority merely by parsing a DTO.
6. `cargo xtask schemas generate` and `schemas check` own both version inventories
   and deterministic parity. The current 22 v1 schemas remain unchanged.
   Check unique IDs, explicit version/kind dispatch, exact file inventories,
   nested schema references, positive/negative runtime/schema conformance and
   unsupported-version rejection. Do not hardcode a new total that future
   versioned additions must bypass. Schema constraints must enforce numeric
   encoding and representable string ranges; classify cross-field semantic
   validations separately rather than claiming the schema proves them.
7. Do not clone all remaining schemas or add placeholder v2 manifests now.
   The existing step that first changes an authored or protocol shape owns its
   real v2 DTO, generator registration, fixtures and explicit consumer update
   in the same commit. This is a standing requirement for S016–S211, including
   P03/P04 authority, P09 decisions, P10/P11 reports and P12 release records.
   No new wire meaning may ship under v1. Existing CLI/MCP v1 operations remain
   stable until their owning integration steps deliberately select v2. No
   implicit conversion reader or indefinite dual-version support is added.
8. Keep the established 16 MiB release-record boundary at every parse. A bounded
   application read rejects an oversized provider result with `LimitExceeded`;
   it never silently truncates. Future concrete providers must enforce bounds
   during reading/allocation, since checking an already returned Vec is not an
   allocation bound. A read-only trait name cannot establish absence of effects:
   actual implementations and integrated call paths require later audit/spies.

### Corrective checkpoint order before S016

These are supervisor-approved independently green corrections under the
original S015 traceability anchor, not additional renumbered mandatory steps.
Complete each in its own commit and record its SHA/report. Only one is active.

| Checkpoint | Scope | Definition of green and dominant verification |
| --- | --- | --- |
| S015-R1 | Restore v1 acceptance; correct v2 integer endpoints, canonical input and generated primitive constraints (RF01/RF03/RF04/RF05). | Existing v1 accepted endpoint fixtures still pass with identical schemas/bytes; all i64/u64 endpoints, safe-boundary neighbors, negative zero, fractional/exponent, small-string and overflow controls agree with v2 parser/serializer/primitive schema. Protocol/domain/runner tests plus full standard gate. |
| S015-R2 | Implement explicit version identity and multi-inventory generation/parity plumbing (RF02). | Existing v1 generation remains byte-identical; explicit v2 identities parse/render, unsupported versions reject, known inventories remain exact. Do not publish invented empty DTO schemas. Generator/domain/xtask tests plus full standard gate. |
| S015-R3 | Add the five actual v2 protocol families with canonical numeric/time/absence behavior and generated schemas atomically (RF02/RF05/RF06). | Real valid records and one-field negative mutations agree with their generated schema constraints and runtime validation; missing optional is omitted, explicit null rejects, leap second rejects, endpoints round-trip, and no v1 path/schema changes. Schema generate/check and protocol/domain/vector tests plus full standard gate. |
| S015-R4 | Finish release/adapter bounded ingress and application result-bound/integration regressions (RF07/RF08/RF09; CR01/CR02/CR06). | Valid actual command-ingress controls pass; duplicate inventory attributes fail through a locked-adapter reconcile fixture; release records immediately below/at/above 16 MiB behave as specified; provider over-return fails, exact limit succeeds, missing ports fail; meaningful command time-injection evidence; application/CLI/protocol tests and application doctests plus full standard gate. |
| S015-R5 | Execute the whole applicable representation corpus and reconcile evidence/ledger/schema ownership (RF09/RF10). | Dispatch all numeric/time/absence vectors by declared semantic category/type, fail on unhandled applicable vectors (no ID allow-list or minimum-count proxy), validate raw bytes/unsupported versions and five-family schema parity, run application doctests, all S015 commands and package structure checks, review reports and mark S015 complete only with recorded evidence. |

Full standard gate means `cargo xtask check` and `git diff --check`; run each
checkpoint's meaningful targeted tests in addition. Do not add intentionally
red commits, amend the reviewed history or weaken tests. S016 retains its
set/sequence/authored-NFC/opaque-byte responsibilities; S027 retains full
cross-domain hash/provenance corpus closure; S069 retains evaluator freshness
arithmetic. RA13 relationship acceptance remains pending S025/S059/S061.

### Continued execution and external gates

The approved next run executes S015-R1–R5, then S016–S211 in order for as long
as the work is dependency-unblocked. S143 is an intermediate checkpoint, not a
stop instruction. The final definition of done remains the full MVP, not a
successful partial batch. Preserve one active step and the original predecessor
chain; there is no authorization to skip an externally blocked prerequisite.

Missing live configuration does not block generic fail-closed code, synthetic
cryptographic fixtures or validated configurable policies. It does block a
step's required real-product/host/authority evidence when that evidence is
actually necessary. At that first boundary, finish already authorized work in
the current slice, record the exact unavailable fact and return to the
supervisor. Do not assume similarly named local projects are the approved
product, invent business/cohort/retention values, or promote synthetic checks
to live acceptance. Q01–Q15 remain explicit where evidence is missing.

A context limit is an interruption, not a product blocker. Compact and resume
where supported; otherwise provide a precise clean checkpoint handback. The
supervisor reviews between sessions and owns any changed decision. Production
release/enforcement, publication, credentials and optional-native activation
remain outside this authorization.

**Verification of this amendment:** standard source gate and application
doctest passed at the reviewed source HEAD; document structure/diff validation
is required after plan synchronization. No repair implementation is claimed.

## Commit convention — inferred from the existing repository

Use **`type: imperative subject`**, lowercase type, no mandatory scope. The checked history includes build/chore/docs subjects in this form. Use the exact supplied subject unless repository evidence requires a documented adjustment; an optional body explains rationale, validation and breaking compatibility. Do not substitute a different convention based on personal preference.

## Execution rules

This is the complete mandatory ordered plan. Chapters are navigation/checkpoint groupings, **not permission to merge the steps into a phase-sized commit**. Each Sxxx is one independently reviewable, tested, known-good logical change. Use one step per commit. Do not skip, merge, reorder or broaden steps. Only repository evidence proving a step obsolete or unsafe justifies a documented deviation; record the precise replacement/disposition and tests before proceeding. Missing live product/host/credential evidence is a blocker, not proof a requirement is obsolete.

Perform preflight from VERIFICATION.md before S001. Keep the extracted handoff outside the checkout until S001 narrowly admits the versioned documentation subtree; S002 imports it under docs/eqm_product_v1. Never silently remove repository safety gates or overwrite unrelated work. All statuses begin pending.

Read the listed durable contracts before each step. Files/modules are likely locations based on the reviewed repository; inspect actual layout and relevant nested instructions first. New modules/symbols/DTOs are proposals to implement, not claims they already exist. Exact public fields, wire versions and native tasks are closed spec-first, not invented to satisfy a test. Internal additions remain non-public or fail-closed until their validating integration exists.

Each step includes its regression/tests with the implementation: no committed intentionally failing tests. Use additive internal construction and bounded refactors to preserve compilation while replacing old orchestration. Generate schemas from source and update tests/docs in the same semantic change. Standard Rust checks and `cargo xtask check` must pass before committing; record actual results and self-review, update the ledger, commit, and then report the SHA and next-step safety.

A reproduced unchanged pre-existing/out-of-scope failure may be recorded with lack-of-regression evidence and explicit safe-to-proceed rationale. It never counts as a pass or closes a required security/native/product gate. A new regression blocks advancement. If a step is too broad to stay safe, document a repository-evidenced split in DEVIATIONS.md rather than silently combining/reordering work. Do not weaken tests/thresholds or hide unavailable hosts to proceed.

The supplied commands use known package names; `eqm` is the CLI package. `eqm_application` is usable only from its creation step. Exact dependency/feature/native commands needing discovery are called out separately. Intentional dependency resolution updates Cargo.lock in the owning step, then checks run --locked. A command listed here is a required future action, not a claim it ran during handoff preparation.

Optional native embedding is **not** part of this mandatory sequence. It is explicitly deferred unless Q11 is activated; its own ordered plan is OPTIONAL_NATIVE_COMMIT_SEQUENCE.md. Within the mandatory product-integration chapter, actual product repos/tasks/host authority must be known before real adapter changes or acceptance claims.

After each commit emit STEP_REPORT_TEMPLATE.md fields: files, behavior, exact commands/results, self-review, schema/DB/security impact, baseline exceptions, risks, commit SHA and next-step safety. Never place a commit's own SHA inside itself; report it externally or in the next ledger update.

## Approved execution cycles — revision 2

Owner approval on 2026-09-26 accepts the complete reviewed scope and recommended defaults. [ADR-0012](../decisions/ADR-0012-approved-review-closure.md) records its limits; [DEV-001](DEVIATIONS.md) amends S049 explicitly. This document is the single governing sequence; chapter files and machine ledgers are synchronized views, not competing plans.

Keep exactly one step active. All 211 implementation steps remain pending; next is S001. Preparing this revision does not complete S001-S006. Every cycle below requires all constituent steps and its definition of green. Resolve named gates at the first affected mandatory step; missing external evidence never licenses skipping ahead. Recorded host support is a target obligation until native execution evidence exists.

| Cycle | Steps | Definition of green | Admission inputs |
|---|---|---|---|
| EQM-P01 | S001–S012 | Approved package admitted; baseline reconciled; S006 representation vectors frozen; application extraction and typed ports remain acyclic. | Q03, Q05, Q14 |
| EQM-P02 | S013–S027 | Canonical component/subject/raw/semantic/attempt identities and dirty-source provenance pass mutation vectors. | Q03, Q14 |
| EQM-P03 | S028–S043 | Shared-source dependencies, participant roles and exact multi-subject records validate without forced target symmetry. | Q02, Q06 |
| EQM-P04 | S044–S058 | Complete protected universe, exact revision loading, fragment risk, prepared construction and facet constraints satisfy negative tests. | Q05 |
| EQM-P05 | S059–S076 | Reference evaluator implements every evidence kind, count, retry, campaign, freshness and waiver rule; meaningful properties pass. | Q05 |
| EQM-P06 | S077–S096 | Pinned linked SQLite and immutable objects pass concurrency, crash/reopen, migration/restore, receipt and retention tests. | Q02, Q07, Q10 |
| EQM-P07 | S097–S115 | Authorized concrete plans and all ingress/process paths obey actual host capabilities and bounded lifecycle guarantees. | Q02, Q07, Q08, Q13 |
| EQM-P08 | S116–S131 | Independent protected authority and real test cryptography reject every missing/wrong trust dimension; no production test bypass. | Q04 |
| EQM-P09 | S132–S143 | Frozen manifests, immutable decisions, replay/current-acceptance distinction and coherent bounded queries agree. | Q04, Q07 |
| EQM-P10 | S144–S157 | All CLI commands use shared application decisions; lifecycle/output-schema/format/error/consumer-doctor regressions pass. | Q14 |
| EQM-P11 | S158–S164 | MCP tools/resources retain one authority/snapshot with explicit refresh, permissions, pagination and full-envelope bounds. | Q07, Q14 |
| EQM-P12 | S165–S178 | Complete release decisions, statements, signing and exact authorizations pass positive/negative consumption tests; shadow remains default. | Q04, Q06, Q09 |
| EQM-P13 | S179–S190 | Conservative impact and bounded batching/cache paths match the reference evaluator and measured representative budgets. | Q07 |
| EQM-P14 | S191–S201 | Actual authorized Rust/KMP/provider tasks, approved product scenarios and supported cohorts pass; optional embedding disposition recorded. | Q01, Q02, Q06, Q12 |
| EQM-P15 | S202–S211 | Standard/extended/host/product/crypto/compatibility/distribution/shadow gates have evidence; final report separates code/platform/operational status. | Q02, Q04, Q07, Q09, Q15 |

Verification lane for every cycle: each step’s listed owning tests, full `cargo xtask check`, and `git diff --check`; additionally run every declared host/product/security/extended lane needed for that cycle’s claims. Optional N001–N016 remain not activated.

## Ordered chapters

- [S001–S012: Repository admission and application boundary](sequence/01-repository-admission-and-application-boundary.md) — 12 separate commits.
- [S013–S027: Canonical identities and evidence provenance](sequence/02-canonical-identities-and-evidence-provenance.md) — 15 separate commits.
- [S028–S043: Shared sources target roles and exact subjects](sequence/03-shared-sources-target-roles-and-exact-subjects.md) — 16 separate commits.
- [S044–S058: Complete obligation derivation and canonical selection](sequence/04-complete-obligation-derivation-and-canonical-selection.md) — 15 separate commits.
- [S059–S076: Evidence selection outcomes and the reference evaluator](sequence/05-evidence-selection-outcomes-and-the-reference-evaluator.md) — 18 separate commits.
- [S077–S096: Immutable object store and transactional SQLite index](sequence/06-immutable-object-store-and-transactional-sqlite-index.md) — 20 separate commits.
- [S097–S115: Authorized acquisition and host supervision](sequence/07-authorized-acquisition-and-host-supervision.md) — 19 separate commits.
- [S116–S131: Protected authority and real verification](sequence/08-protected-authority-and-real-verification.md) — 16 separate commits.
- [S132–S143: Frozen inputs decisions and coherent query service](sequence/09-frozen-inputs-decisions-and-coherent-query-service.md) — 12 separate commits.
- [S144–S157: CLI integration and removal of placeholder semantics](sequence/10-cli-integration-and-removal-of-placeholder-semantics.md) — 14 separate commits.
- [S158–S164: Read-only MCP and coherent agent access](sequence/11-read-only-mcp-and-coherent-agent-access.md) — 7 separate commits.
- [S165–S178: Release evaluation statements and exact authorization](sequence/12-release-evaluation-statements-and-exact-authorization.md) — 14 separate commits.
- [S179–S190: Conservative impact analysis and measured optimization](sequence/13-conservative-impact-analysis-and-measured-optimization.md) — 12 separate commits.
- [S191–S201: Rust KMP adapters and product acceptance integration](sequence/14-rust-kmp-adapters-and-product-acceptance-integration.md) — 11 separate commits.
- [S202–S211: Verification distribution and operational sign-off](sequence/15-verification-distribution-and-operational-sign-off.md) — 10 separate commits.

---

<a id="s001"></a>
## S001 — Admit the approved spec subtree

**Predecessor:** PREFLIGHT. **Contracts:** R53, R56, R61. **Acceptance:** AC053, AC056, AC061.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md).

### 1. Step title
Admit the approved spec subtree.

### 2. Purpose
Keep the standard repository gate usable without discarding security checks.

### 3. Exact scope of code changes
Narrowly allow docs/eqm_product_v1 content and replace generic-word handoff rejection with scoped meaningful private-data checks; keep unrelated directory/generated-state restrictions.

### 4. Files/modules likely involved
- `tools/xtask/src/main.rs`
- `tools/xtask tests`
- `docs/README.md`

### 5. Unit tests or integration tests required
Add fixtures admitting the versioned spec subtree and rejecting arbitrary top-level/private/generated material; run public-repository checks.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Approved contract docs can be imported while existing meaningful safety rules still fail on forbidden inputs.

### 8. Commit message
```text
build: admit versioned product specifications
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s002"></a>
## S002 — Import durable contracts and the ordered plan

**Predecessor:** S001. **Contracts:** R56, R57, R61, R62. **Acceptance:** AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [repo/BASELINE.md](../repo/BASELINE.md).

### 1. Step title
Import durable contracts and the ordered plan.

### 2. Purpose
Establish versionable approved intent before implementation.

### 3. Exact scope of code changes
Add the handoff under docs/eqm_product_v1, including required contract files, requirements/acceptance catalogs and initial pending progress; preserve existing public docs and mark code/tests unimplemented.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/`
- `docs/README.md`

### 5. Unit tests or integration tests required
Run the package validator at its imported path and verify internal links/IDs; assert no new generated .eqm state or secrets.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Agents can reconstruct intent and resume from an explicit pending step without chat history.

### 8. Commit message
```text
docs: record the eqm product v1 contract
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s003"></a>
## S003 — Merge concise agent guidance

**Predecessor:** S002. **Contracts:** R50, R53, R56, R62. **Acceptance:** AC050, AC053, AC056, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md).

### 1. Step title
Merge concise agent guidance.

### 2. Purpose
Keep repository-local operating rules aligned with the approved plan.

### 3. Exact scope of code changes
Merge repo/AGENTS.md guidance into existing root instructions; point to durable specs and one-step reporting; clarify approved changes versus gated unsafe/version decisions.

### 4. Files/modules likely involved
- `AGENTS.md`
- `docs/agent-context.md`
- `docs/eqm_product_v1/implementation/`

### 5. Unit tests or integration tests required
Validate documentation references and compare instructions for conflicting execution/trust permissions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Root guidance is short, preserves existing protections, and routes an agent to the next exact step.

### 8. Commit message
```text
docs: anchor agent work to the product specification
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s004"></a>
## S004 — Record baseline findings and decision gates

**Predecessor:** S003. **Contracts:** R14, R23, R50, R53, R57, R62. **Acceptance:** AC014, AC023, AC050, AC053, AC057, AC062.

**Read:** [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Record baseline findings and decision gates.

### 2. Purpose
Prevent static observations being mistaken for tested current defects.

### 3. Exact scope of code changes
Reconcile F01-F20 and RA01-RA21 against the actual HEAD, incorporate the recorded full-source review and baseline checks, and preserve unrun implementation acceptance. Revalidate the approved decisions in ADR-0012 and record only remaining external facts; apply DEV-001 to the planning views before its owning implementation step.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/repo/REVIEW_FINDINGS.md`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`

### 5. Unit tests or integration tests required
Verify every F/RA item has source evidence, requirements, acceptance, owning steps and a precise regression; distinguish static review from runtime reproduction and prior baseline checks from new implementation evidence.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
The implementation baseline and unresolved blockers are evidenced without invented passing tests.

### 8. Commit message
```text
docs: record implementation baseline and open gates
```

**Approved review regression contributions:** RA05.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s005"></a>
## S005 — Add deterministic conformance fixture builders

**Predecessor:** S004. **Contracts:** R14, R26, R50, R53. **Acceptance:** AC014, AC026, AC050, AC053.

**Read:** [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Add deterministic conformance fixture builders.

### 2. Purpose
Supply reusable regression inputs without production trust shortcuts.

### 3. Exact scope of code changes
Extend test support with injected time, exact subject/selection builders and synthetic evidence helpers; keep every authority stand-in test-only.

### 4. Files/modules likely involved
- `crates/eqm_test_support/src/`
- `crates/eqm_engine/tests/support/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Builders reject malformed coordinates and produce repeatable IDs; production dependency/feature inspection shows no synthetic trusted factory.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Later steps can add focused regressions without duplicating fragile fixture construction.

### 8. Commit message
```text
test: add deterministic conformance fixtures
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s006"></a>
## S006 — Freeze semantic and version representation decisions

**Predecessor:** S005. **Contracts:** R14, R16, R21, R23, R50, R52, R58. **Acceptance:** AC014, AC016, AC021, AC023, AC050, AC052, AC058.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Freeze semantic and version representation decisions.

### 2. Purpose
Resolve protocol ambiguity before new wire meanings ship.

### 3. Exact scope of code changes
Materialize ADR-0012 approval: explicit new wire namespace, required conjunction/authorized alternatives, per-kind logical count units, retry/conflict/campaign rules and inherited fragment risk. Complete engineering decisions for the exact namespace/URI transition, numeric ranges, timestamp precision, null/absence, sets/sequences, digest domains and minimal public API names. Audit consumers/releases; stop only for a newly evidenced obligation conflicting with the approved direction or a still-missing required contract.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/decisions/`
- `docs/eqm_product_v1/specs/API_CONTRACTS.md`
- `docs/eqm_product_v1/specs/EVALUATION_SEMANTICS.md`

### 5. Unit tests or integration tests required
Add conformance vectors for approved outcome/count/risk rules and all representation boundaries. Cross-check schema generators, URI parsing and consumer expectations; retain originals and reject unsupported versions explicitly. Record fixture policies as synthetic; do not invent production threshold/skew values.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** ADR-0012 records owner approval; do not re-request the settled direction. S006 closes exact engineering representations and any newly evidenced conflicting consumer obligation. Live product/authority values remain governed by Q01-Q15; DEV-001 defines the S049 amendment.

### 7. Expected result
New wire and policy work has explicit normative representations; unresolved external compatibility blocks exposure.

### 8. Commit message
```text
docs: freeze evaluation and protocol representations
```

**Approved review regression contributions:** RA05.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s007"></a>
## S007 — Create the shared application crate

**Predecessor:** S006. **Contracts:** R04, R06, R50. **Acceptance:** AC004, AC006, AC050.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md).

### 1. Step title
Create the shared application crate.

### 2. Purpose
Introduce the approved effectful boundary without moving semantics.

### 3. Exact scope of code changes
Add eqm_application to workspace with exact local dependencies and inherited lint/license metadata; expose only a documented minimal module boundary and retain existing CLI behavior.

### 4. Files/modules likely involved
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_application/Cargo.toml`
- `crates/eqm_application/src/lib.rs`

### 5. Unit tests or integration tests required
Compile the new crate, verify no CLI/MCP dependency cycle, and test its empty/minimal public boundary without success placeholders.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Workspace remains green and the application layer exists without changing release decisions.

### 8. Commit message
```text
refactor: introduce the shared application layer
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s008"></a>
## S008 — Move prepared workspace ownership into the application layer

**Predecessor:** S007. **Contracts:** R04, R06, R22. **Acceptance:** AC004, AC006, AC022.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Move prepared workspace ownership into the application layer.

### 2. Purpose
Share immutable preparation without CLI coupling.

### 3. Exact scope of code changes
Move or extract PreparedSession/SessionRequest loading ownership into eqm_application; leave MCP adaptation in the transport/CLI layer; update call sites without changing evaluation semantics.

### 4. Files/modules likely involved
- `crates/eqm_application/src/session.rs`
- `crates/eqm_cli/src/session.rs`
- `crates/eqm_cli/src/commands/`
- `crates/eqm_mcp/src/session.rs`

### 5. Unit tests or integration tests required
Existing session fixtures retain graph digest/source maps; test immutable borrowing and no reload/cycle across MCP adaptation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_mcp
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI behavior is preserved while other adapters can use the same prepared workspace.

### 8. Commit message
```text
refactor: share prepared workspace sessions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s009"></a>
## S009 — Preserve manifest error causes

**Predecessor:** S008. **Contracts:** R45, R50. **Acceptance:** AC045, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md).

### 1. Step title
Preserve manifest error causes.

### 2. Purpose
Restore actionable parse/load diagnostics.

### 3. Exact scope of code changes
Replace stage-only load/conversion error erasure with typed contextual variants retaining source path, field and causal error where safe; do not alter schema acceptance.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/loader.rs`
- `crates/eqm_manifest/src/validation.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Invalid TOML, conversion, discovery and lock fixtures retain appropriate path/field information without secret leakage.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Manifest failures remain diagnosable at higher boundaries.

### 8. Commit message
```text
fix: preserve manifest loading diagnostics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s010"></a>
## S010 — Preserve session preparation errors

**Predecessor:** S009. **Contracts:** R45, R50. **Acceptance:** AC045, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md).

### 1. Step title
Preserve session preparation errors.

### 2. Purpose
Keep new shared preparation failures typed and contextual.

### 3. Exact scope of code changes
Map resolve/expand/canonicalization failures into application errors with causes; retain stable command behavior until public error-envelope step.

### 4. Files/modules likely involved
- `crates/eqm_application/src/session.rs`
- `crates/eqm_application/src/error.rs`
- `crates/eqm_cli/src/session.rs`

### 5. Unit tests or integration tests required
Malformed graph/fragment pin and unavailable file fixtures retain causes across application/CLI conversion.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Preparation no longer discards useful errors while production commands remain fail-closed.

### 8. Commit message
```text
fix: retain prepared session error context
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s011"></a>
## S011 — Inject evaluation time consistently

**Predecessor:** S010. **Contracts:** R05, R22, R38. **Acceptance:** AC005, AC022, AC038.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Inject evaluation time consistently.

### 2. Purpose
Remove hidden wall-clock reads from shared semantic preparation.

### 3. Exact scope of code changes
Add explicit host-captured evaluation time to application requests and pass it through existing query/evaluation call sites; keep acquisition timing and monotonic process timers separate.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Repeated requests with the same time are deterministic; boundary-time fixtures prove no internal SystemTime drift in evaluation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evaluation has explicit time without changing process timeout mechanisms.

### 8. Commit message
```text
refactor: inject evaluation clock context
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s012"></a>
## S012 — Define narrow application capability interfaces

**Predecessor:** S011. **Contracts:** R04, R05, R06, R64. **Acceptance:** AC004, AC005, AC006, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Define narrow application capability interfaces.

### 2. Purpose
Separate read/evaluate operations from privileged acquisition.

### 3. Exact scope of code changes
Introduce documented ports for authority/evidence access, verification and execution with explicit requests; no default global runtime or permissive fallback implementations.

### 4. Files/modules likely involved
- `crates/eqm_application/src/lib.rs`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/src/acquisition.rs`

### 5. Unit tests or integration tests required
Spy implementations show read/evaluate cannot launch runners or fetch network; missing capabilities return explicit errors.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Application use cases can be tested independently without implicit host privileges.

### 8. Commit message
```text
refactor: separate application capability boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s013"></a>
## S013 — Add domain-specific identity wrappers

**Predecessor:** S012. **Contracts:** R18, R19, R50. **Acceptance:** AC018, AC019, AC050.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Add domain-specific identity wrappers.

### 2. Purpose
Prevent interchange of unrelated digest meanings.

### 3. Exact scope of code changes
Extend digest/ID types with the approved raw-object, semantic-evidence, plan/campaign/attempt and evaluation-context distinctions; use private validated construction and documented domains.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/digest.rs`
- `crates/eqm_domain/src/id.rs`
- `crates/eqm_domain/src/lib.rs`

### 5. Unit tests or integration tests required
Invalid/foreign digest domains reject; round trips and equality remain deterministic; no Default success identity.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Internal APIs cannot accidentally pass a workspace digest where a distinct identity is required.

### 8. Commit message
```text
feat: distinguish conformance identity domains
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s014"></a>
## S014 — Reject duplicate JSON members at ingestion

**Predecessor:** S013. **Contracts:** R18, R19, R21, R42, R64. **Acceptance:** AC018, AC019, AC021, AC042, AC064.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Reject duplicate JSON members at ingestion.

### 2. Purpose
Prevent ambiguous canonical or signed payload interpretation.

### 3. Exact scope of code changes
Add a bounded strict JSON ingestion path for protocol records that detects duplicate object names before conversion/canonicalization; preserve original raw bytes.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_protocol/tests/`

### 5. Unit tests or integration tests required
Duplicate top-level/nested keys reject; valid records retain exact raw bytes; malformed/deep input remains bounded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence is not normalized through an ambiguous last-key-wins parse.

### 8. Commit message
```text
fix: reject duplicate protocol object members
```

**Approved review regression contributions:** RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s015"></a>
## S015 — Enforce approved numeric and temporal wire forms

**Predecessor:** S014. **Contracts:** R18, R19, R21, R52, R64. **Acceptance:** AC018, AC019, AC021, AC052, AC064.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enforce approved numeric and temporal wire forms.

### 2. Purpose
Make cross-runtime digests predictable.

### 3. Exact scope of code changes
Retain DEV-002 S015-R1 through S015-R5 and complete DEV-003 S015-R6 through S015-R9: close exact version/kind ingress and nested inventory validation; repair runtime-facts byte/count bounds and retained subject syntax checks; enforce v2 schema markers and minimal UTC fractions; use actual schema validation with separate raw-token expectations; replace tautological corpus/application/CLI controls and reconcile evidence before S016.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/time.rs`
- `crates/eqm_domain/src/schema.rs`
- `crates/eqm_protocol/src/bin/generate_protocol_schemas.rs`
- `tools/xtask/src/main.rs`
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/`
- `crates/eqm_runner/src/inventory.rs`
- `crates/eqm_test_support/`
- `schemas/`
- `crates/eqm_protocol/tests/`
- `crates/eqm_manifest/src/`
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_protocol/Cargo.toml`

### 5. Unit tests or integration tests required
All DEV-002 and DEV-003 controls pass: full integer endpoints, canonical raw tokens, UTC/calendar/absence rules, exact root/nested versions, v1 preservation, all five schemas validated with Draft 2020-12, actual 16 MiB runtime-facts and fact-count bounds, literal corpus execution, successful reconcile control and fake-reader integration. Application doctests and schema parity pass; JSON-value conformance is distinguished from raw-token and cross-field validation.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_domain -p eqm_manifest -p eqm_application -p eqm -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
cargo test --locked -p eqm_application --doc
git diff --check
```

**Admission gate:** DEV-005: S015 accepted for allocated scope; S016-P1 also accepted. Retain corrective history and proceed to S016-R1. Later semantic/product acceptance remains with its owning steps.

### 7. Expected result
No consumer silently changes a number or time before hashing.

### 8. Commit message
```text
fix: enforce canonical numeric and time representations
```

**Approved review regression contributions:** RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s016"></a>
## S016 — Add canonical set and Unicode vectors

**Predecessor:** S015. **Contracts:** R18, R19, R21, R43, R64. **Acceptance:** AC018, AC019, AC021, AC043, AC064.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Add canonical set and Unicode vectors.

### 2. Purpose
Distinguish semantic unordered collections from ordered scenarios.

### 3. Exact scope of code changes
Retain accepted S016-P1 and implementation 6837831; complete DEV-005 S016-R1 and S016-R2 to close authored v2 ingress/type validation and real-boundary collection/sequence/raw-byte regressions. Follow the explicit field mapping, independent literal canonical bytes/digest expectations and evidence discipline in DEV-005; preserve all retained DEV-004 rules and v1 bytes.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/`
- `crates/eqm_test_support/`
- `crates/eqm_engine/Cargo.toml`
- `schemas/v2/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Execute every DEV-005 mapped authored field family with valid typed controls and one-field mutants at owning roots; all set permutations/exact and logical-key duplicates, profile revision conflicts, valid attempt-index rules, diagnostic/related-location sequence order and multiplicity, independent literal runtime-facts bytes/hash, and strict/raw opaque identity controls. Preserve S015 regressions and v1 schema/hash bytes. Final ledger and current status must pass structure validation after edits.

### 6. Verification commands
```sh
cargo audit --deny warnings
cargo deny check
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_protocol -p eqm_domain -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

**Admission gate:** DEV-005: S016-P1 accepted, RF19 closed. Execute independently green S016-R1 then S016-R2 before S017. After their objective gates, record supervisor acceptance pending and continue ordered unblocked work through S211.

### 7. Expected result
Canonicalization is executable protocol behavior rather than serializer convention.

### 8. Commit message
```text
test: lock canonical collection and string semantics
```

**Approved review regression contributions:** RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s017"></a>
## S017 — Hash complete binding semantics

**Predecessor:** S016. **Contracts:** R19. **Acceptance:** AC019.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Hash complete binding semantics.

### 2. Purpose
Correct the ID-only binding digest shortcut.

### 3. Exact scope of code changes
Implement a separate v2 complete binding projection and existing BindingDigest using DEV-005 numeric/set rules. Reuse the existing binding projection as a field inventory, not its v1 bytes. Include all binding/artifact/exposure/evidence semantics and semantic extensions; exclude only dev.equivalencematrix.display and absent parse provenance. Preserve v1 APIs/hashes and existing crate directions; later owning steps integrate v2 evidence construction.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical.rs`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_domain/src/digest.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Same-ID selector/artifact path/constraint/semantic-extension changes alter identity; actual display-extension and comment/formatting changes remain stable. Set permutations agree, ordered extension/expression arrays retain order, and integer boundaries have literal canonical-byte/digest controls. All v1 projection/hash fixtures remain unchanged.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Binding identity binds actual approved semantics.

### 8. Commit message
```text
fix: bind evidence to canonical binding semantics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s018"></a>
## S018 — Hash complete evidence specifications

**Predecessor:** S017. **Contracts:** R19. **Acceptance:** AC019.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Hash complete evidence specifications.

### 2. Purpose
Correct the ID-pair evidence-specification shortcut.

### 3. Exact scope of code changes
Add a canonical specification projection covering kind, requirements, facets, selector, runner reference and relevant constraints; document input-domain relationships.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_domain/src/evidence.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Selector/kind/count changes with the same spec ID change identity; stable semantic projections remain repeatable.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence cannot match a changed specification merely by retaining names.

### 8. Commit message
```text
fix: hash exact evidence specification semantics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s019"></a>
## S019 — Separate contract and policy digests

**Predecessor:** S018. **Contracts:** R19, R58. **Acceptance:** AC019, AC058.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Separate contract and policy digests.

### 2. Purpose
Stop populating unrelated provenance with one workspace digest.

### 3. Exact scope of code changes
Expose explicit contract and selected-policy/profile semantic projections; retain workspace digest only as aggregate context and update internal construction helpers.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Contract-only and policy-only changes affect the intended domains; unchanged unrelated projections remain stable.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Internal prepared context has distinct accurate contract/policy identities.

### 8. Commit message
```text
fix: separate contract and policy provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s020"></a>
## S020 — Separate target configuration identity

**Predecessor:** S019. **Contracts:** R19, R22. **Acceptance:** AC019, AC022.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Separate target configuration identity.

### 2. Purpose
Bind actual target configuration rather than the full graph summary.

### 3. Exact scope of code changes
Define target configuration projection with relevant build/profile/source-component references from available data; preserve explicit absence rather than inventing tool metadata.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Target configuration changes invalidate its identity; unrelated display formatting does not; missing required config is explicit.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_domain -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Target evidence can be checked against a meaningful configuration identity.

### 8. Commit message
```text
fix: derive exact target configuration identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s021"></a>
## S021 — Bind runner identity to approved executable bytes

**Predecessor:** S020. **Contracts:** R19, R30. **Acceptance:** AC019, AC030.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md).

### 1. Step title
Bind runner identity to approved executable bytes.

### 2. Purpose
Preserve executable provenance in persisted evidence.

### 3. Exact scope of code changes
Extend resolved runner identity to include independently verified executable digest plus semantic definition and relevant approved toolchain context; keep unresolved programs non-executable.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/resolution.rs`
- `crates/eqm_runner/tests/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Replacing executable bytes at the same path changes the recorded runner identity; definition-only changes also invalidate it.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Runner evidence cannot reuse a definition digest after executable substitution.

### 8. Commit message
```text
fix: include executable content in runner identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s022"></a>
## S022 — Normalize repository and source provenance explicitly

**Predecessor:** S021. **Contracts:** R19, R22. **Acceptance:** AC019, AC022.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Normalize repository and source provenance explicitly.

### 2. Purpose
Prevent display remote strings and dirty HEAD from implying exact clean source.

### 3. Exact scope of code changes
Centralize repository identity acquisition/normalization and exact source-tree provenance in application code; retain unsupported/missing cases as errors; do not silently trust arbitrary remotes.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_cli/src/commands/attest.rs`

### 5. Unit tests or integration tests required
Equivalent supported remote encodings map deliberately; modified/untracked fixture content is labeled dirty and cannot satisfy clean release provenance.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Source identity reflects inspected/executed bytes and shared identity logic replaces copies.

### 8. Commit message
```text
fix: bind source provenance beyond clean head claims
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s023"></a>
## S023 — Model raw and semantic evidence identities

**Predecessor:** S022. **Contracts:** R18, R21, R22, R35. **Acceptance:** AC018, AC021, AC022, AC035.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Model raw and semantic evidence identities.

### 2. Purpose
Preserve exact signed bytes independently of normalized assertions.

### 3. Exact scope of code changes
Add validated raw-object/semantic-evidence relation types and conversion results without storage or trust promotion; keep existing records readable only under the version ADR.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_protocol/tests/`

### 5. Unit tests or integration tests required
Alternate whitespace transports differ in raw identity but agree semantically; signed originals are returned unchanged; changed assertions differ.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence normalization no longer conflates raw collision with semantic identity.

### 8. Commit message
```text
feat: separate raw and semantic evidence identity
```

**Approved review regression contributions:** RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s024"></a>
## S024 — Model plans campaigns and attempts

**Predecessor:** S023. **Contracts:** R16, R17, R18. **Acceptance:** AC016, AC017, AC018.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Model plans campaigns and attempts.

### 2. Purpose
Establish exact acquisition lineage before selection/persistence.

### 3. Exact scope of code changes
Add validated plan/campaign/run/logical-case/attempt identifiers and explicit relationships; reject inconsistent or missing required lineage instead of deriving it from timestamps.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_domain/src/id.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Duplicate attempt names under the same execution reject ambiguous construction; distinct runs/cases remain independent.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
The engine can reason about attempts without counting JSON files.

### 8. Commit message
```text
feat: model evidence execution lineage
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s025"></a>
## S025 — Validate evidence identity relationships

**Predecessor:** S024. **Contracts:** R14, R15, R16, R18, R19, R21, R64. **Acceptance:** AC014, AC015, AC016, AC018, AC019, AC021, AC064.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate evidence identity relationships.

### 2. Purpose
Catch contradictions among envelope and payload subjects.

### 3. Exact scope of code changes
Validate matching scope/target/unit/requirements/facets/kind and identity lineage across protocol-to-domain conversion; require explicit typed absence where allowed.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_protocol/tests/`

### 5. Unit tests or integration tests required
Mismatched payload kind, subject, attempt or semantic digest reject before evaluation; valid fixtures round-trip.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Structurally valid JSON cannot create internally contradictory evidence.

### 8. Commit message
```text
fix: validate evidence envelope relationships
```

**Approved review regression contributions:** RA02, RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s026"></a>
## S026 — Model the acyclic record dependency chain

**Predecessor:** S025. **Contracts:** R20, R28. **Acceptance:** AC020, AC028.

**Read:** [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Model the acyclic record dependency chain.

### 2. Purpose
Prevent evidence-to-decision digest cycles.

### 3. Exact scope of code changes
Add candidate/plan/input-manifest/decision/reference distinctions and validate directed references before publication; do not require future release decisions in evidence.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_protocol/src/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Candidate-to-evidence-to-decision chains validate; self/future/cyclic record references reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Readiness records can be created in a realizable order.

### 8. Commit message
```text
feat: enforce acyclic evaluation record identities
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s027"></a>
## S027 — Add cross-domain provenance regression corpus

**Predecessor:** S026. **Contracts:** R18, R19, R20, R21, R22. **Acceptance:** AC018, AC019, AC020, AC021, AC022.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Add cross-domain provenance regression corpus.

### 2. Purpose
Lock identity semantics before target and evaluator changes.

### 3. Exact scope of code changes
Consolidate approved raw/semantic/component/source/lineage vectors into shared fixtures and golden identities without live credentials.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `crates/eqm_test_support/src/`
- `crates/eqm_protocol/tests/`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Mutation of every bound axis prevents inappropriate reuse; formatting-only changes behave as declared; fixture generator is deterministic.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support -p eqm_protocol -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
Future layers consume tested identity contracts with stable regression evidence.

### 8. Commit message
```text
test: lock conformance provenance boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s028"></a>
## S028 — Model product roles separately from targets

**Predecessor:** S027. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model product roles separately from targets.

### 2. Purpose
Support asymmetric CLI/control-tower/edge behavior.

### 3. Exact scope of code changes
Add validated role metadata to the logical target model without hardcoding the user's whole product; keep existing target IDs stable.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/id.rs`
- `crates/eqm_domain/tests/`

### 5. Unit tests or integration tests required
Distinct target outputs may share a role; role changes do not accidentally rename targets or imply participation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Product role is explicit and independent of platform/build identity.

### 8. Commit message
```text
feat: separate product roles from shipped targets
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s029"></a>
## S029 — Model source components

**Predecessor:** S028. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model source components.

### 2. Purpose
Represent shared implementation input without duplicate target roots.

### 3. Exact scope of code changes
Add source-component identities and confined roots with explicit target-to-component relations; retain domain-only validation.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/graph.rs`
- `crates/eqm_domain/tests/`

### 5. Unit tests or integration tests required
Shared components attach to multiple targets; duplicate authority and invalid path identities reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Shared KMP/Rust source ownership is representable without filesystem exceptions.

### 8. Commit message
```text
feat: model shared source components
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s030"></a>
## S030 — Resolve source dependency edges

**Predecessor:** S029. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Resolve source dependency edges.

### 2. Purpose
Make transitive shared-source ownership explicit.

### 3. Exact scope of code changes
Resolve component references, target dependencies and cycles in graph preparation; reject dangling/cyclic unsupported dependencies deterministically.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/resolve.rs`
- `crates/eqm_engine/src/invariants.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Shared dependency DAG resolves; missing nodes/cycles and ambiguous ownership produce stable diagnostics.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Source dependency graph is validated before affected analysis.

### 8. Commit message
```text
feat: resolve shared source dependencies
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s031"></a>
## S031 — Extend authored target and source schemas

**Predecessor:** S030. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Extend authored target and source schemas.

### 2. Purpose
Expose the approved model through generated manifests.

### 3. Exact scope of code changes
Add target role/source-component DTOs using the version decision; regenerate schemas and examples atomically without implicit old aliases.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/dto/`
- `crates/eqm_manifest/src/conversion/`
- `schemas/`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Strict decode/encode/schema vectors cover shared components and unknown fields; current explicit version behavior is tested.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Authored shared-source metadata has a versioned strict representation.

### 8. Commit message
```text
feat: author shared source target metadata
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s032"></a>
## S032 — Load overlapping shared-source targets safely

**Predecessor:** S031. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Load overlapping shared-source targets safely.

### 2. Purpose
Remove duplicate-root rejection only where explicit shared ownership permits it.

### 3. Exact scope of code changes
Update loader root handling to distinguish output identity from shared source roots, preserve portable collision checks, and reject ambiguous undeclared overlaps.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/loader.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Two approved outputs share a component; undeclared root collision and path escape reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Natural KMP layouts load without weakening repository containment.

### 8. Commit message
```text
fix: load explicitly shared target source roots
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s033"></a>
## S033 — Validate artifacts against declared source ownership

**Predecessor:** S032. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Validate artifacts against declared source ownership.

### 2. Purpose
Stop requiring every valid shared artifact under one output root.

### 3. Exact scope of code changes
Update binding conversion and structural expectations to use declared source-component ownership; keep wrong-target and undeclared paths blocking.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/conversion/`
- `crates/eqm_engine/src/structure.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Shared commonMain artifact validates for dependent outputs; unrelated component and escaped paths fail.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Shared code is valid only through explicit dependency authority.

### 8. Commit message
```text
feat: validate artifacts through source ownership
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s034"></a>
## S034 — Model capability participation rules

**Predecessor:** S033. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model capability participation rules.

### 2. Purpose
Express intentional role differences independently of waivers.

### 3. Exact scope of code changes
Add validated requirement/target/profile participation selectors aligned with existing applicability vocabulary; reject ambiguous selectors and duplicate authority.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/policy.rs`
- `crates/eqm_domain/tests/`

### 5. Unit tests or integration tests required
Mobile-only capture and desktop-only review fixtures validate; unspecified required context stays representably unknown.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Product asymmetry can be authored without permanent waiver abuse.

### 8. Commit message
```text
feat: model target capability participation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s035"></a>
## S035 — Author and resolve participation metadata

**Predecessor:** S034. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Author and resolve participation metadata.

### 2. Purpose
Make participation part of protected graph semantics.

### 3. Exact scope of code changes
Add strict DTO/conversion/resolution and semantic hashing for approved participation rules; regenerate schemas together.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/dto/`
- `crates/eqm_manifest/src/conversion/`
- `crates/eqm_engine/src/resolve.rs`
- `schemas/`

### 5. Unit tests or integration tests required
Round-trip participation rules; undeclared targets/profiles and weakening graph mutations are diagnosable.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Participation is versioned semantic authority, not a UI-only filter.

### 8. Commit message
```text
feat: resolve authored capability participation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s036"></a>
## S036 — Evaluate applicability per participant coordinate

**Predecessor:** S035. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Evaluate applicability per participant coordinate.

### 2. Purpose
Preserve target-specific applicability and unknown context.

### 3. Exact scope of code changes
Extend engine context so participation/applicability can be evaluated for exact requirement/participant/profile combinations; keep true/false/unknown distinct.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/applicability.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Same requirement is true for edge, false for explicit exclusion, unknown for missing profile; no early global drop.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Applicability no longer incorrectly applies one pre-fanout conclusion to all targets.

### 8. Commit message
```text
fix: evaluate applicability for exact participants
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s037"></a>
## S037 — Model role-bearing scenario occurrences

**Predecessor:** S036. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model role-bearing scenario occurrences.

### 2. Purpose
Represent multiple actors of one shipped target.

### 3. Exact scope of code changes
Add scenario participant occurrence IDs/roles and exact dependency mappings for end-to-end scope; do not collapse repeated devices into a set.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/entity.rs`
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_domain/tests/`

### 5. Unit tests or integration tests required
Two mobile occurrences remain distinct; duplicate occurrence IDs and missing roles reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Multi-device journeys have unambiguous participant identity.

### 8. Commit message
```text
feat: model scenario participant occurrences
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s038"></a>
## S038 — Model exact multi-subject manifests

**Predecessor:** S037. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Model exact multi-subject manifests.

### 2. Purpose
Bind each actor to its own source build artifact and configuration.

### 3. Exact scope of code changes
Add validated subject manifests for target/provider/scenario participants with explicit required identity fields and typed absence for allowed development cases.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_protocol/tests/`

### 5. Unit tests or integration tests required
Different valid mobile/desktop builds coexist; substituted artifact/build/config mismatches reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Cross-target context does not require identical build digests.

### 8. Commit message
```text
feat: bind exact multi-participant subjects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s039"></a>
## S039 — Separate comparison context identity

**Predecessor:** S038. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Separate comparison context identity.

### 2. Purpose
Prevent exact build identity from blocking valid comparability.

### 3. Exact scope of code changes
Define shared contract/scenario/logical-profile/comparison-rule identity separately from subject manifest/evaluation identity and project it canonically.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/digest.rs`
- `crates/eqm_engine/src/equivalence.rs`
- `crates/eqm_manifest/src/canonical/`

### 5. Unit tests or integration tests required
Approved differing builds share comparison context; different scenario/profile/contract does not compare.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Comparison eligibility and exact subject verification remain independent.

### 8. Commit message
```text
feat: distinguish comparison context from exact subjects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s040"></a>
## S040 — Bind provider obligations explicitly

**Predecessor:** S039. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Bind provider obligations explicitly.

### 2. Purpose
Avoid unscoped provider participation in unrelated targets.

### 3. Exact scope of code changes
Represent provider subject/configuration and target/scenario dependency links; adjust domain/graph checks while deferring final aggregation to evaluator steps.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_engine/src/resolve.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Missing required provider blocks its dependents; an unrelated provider is not automatically attached to every target.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Shared services have exact subject and dependency semantics.

### 8. Commit message
```text
feat: bind shared provider dependencies
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s041"></a>
## S041 — Author compatibility cohorts

**Predecessor:** S040. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Author compatibility cohorts.

### 2. Purpose
Support approved mixed-version combinations without inventing a support window.

### 3. Exact scope of code changes
Add strict data representation and validation for declared required participant/build cohorts and scenario coverage; leave operational cohort values explicit.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/policy.rs`
- `crates/eqm_manifest/src/dto/`
- `crates/eqm_manifest/src/conversion/`
- `schemas/`

### 5. Unit tests or integration tests required
Approved N/N+1 fixture cohort works; undeclared combination and absent participant reject.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Independently released targets can be tested against explicit compatibility policy.

### 8. Commit message
```text
feat: define explicit compatibility cohorts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s042"></a>
## S042 — Add the asymmetric multi-runtime graph fixture

**Predecessor:** S041. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Add the asymmetric multi-runtime graph fixture.

### 2. Purpose
Exercise the target/source/participant model as a whole.

### 3. Exact scope of code changes
Add synthetic Rust CLI, desktop, mobile and provider authority with shared source sets, repeated actors and role-specific requirements; no invented real product build tasks.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `crates/eqm_test_support/src/`
- `crates/eqm_manifest/tests/`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Load/resolve/canonicalize fixture and assert stable roles/dependencies/cohorts; intentional exclusions are not waivers.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support -p eqm_manifest -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Later evaluator tests have a representative but honestly synthetic product graph.

### 8. Commit message
```text
test: add asymmetric multi-runtime graph coverage
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s043"></a>
## S043 — Verify target model schema and identity parity

**Predecessor:** S042. **Contracts:** R07, R08, R09, R13, R19, R22, R48, R52. **Acceptance:** AC007, AC008, AC009, AC013, AC019, AC022, AC048, AC052.

**Read:** [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Verify target model schema and identity parity.

### 2. Purpose
Close graph-model changes before obligation routing.

### 3. Exact scope of code changes
Update public usage examples and schema parity assertions for new target/subject/participation records, preserving explicit compatibility decisions.

### 4. Files/modules likely involved
- `docs/concepts.md`
- `docs/manifests.md`
- `examples/`
- `crates/eqm_protocol/tests/`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
Full schema regeneration comparison, example validation and component-digest regressions pass.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
Authored/protocol/domain target models agree under the approved version contract.

### 8. Commit message
```text
test: verify shared target model parity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s044"></a>
## S044 — Move exact selection orchestration into application

**Predecessor:** S043. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Move exact selection orchestration into application.

### 2. Purpose
Remove CLI-only development selection assumptions.

### 3. Exact scope of code changes
Extract policy/profile request preparation into application operations with explicit evaluation mode and authority input; keep host verification separate.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_engine/src/selection.rs`

### 5. Unit tests or integration tests required
Development baseline behavior remains explicit and protected requests cannot take an implicit development branch.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Selection is reusable without deriving authority from the calling command name.

### 8. Commit message
```text
refactor: share exact policy selection preparation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s045"></a>
## S045 — Reject duplicate profile requests

**Predecessor:** S044. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Reject duplicate profile requests.

### 2. Purpose
Prevent silent last-write-wins profile selection.

### 3. Exact scope of code changes
Replace collection-overwrite request parsing with duplicate detection for profiles/dimensions/revisions and retain exact diagnostics.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_engine/src/selection.rs`

### 5. Unit tests or integration tests required
Duplicate same/conflicting profile values reject; distinct complete selections work.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Public selection cannot silently discard caller intent.

### 8. Commit message
```text
fix: reject duplicate profile selections
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s046"></a>
## S046 — Select exact policy and profile revisions

**Predecessor:** S045. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Select exact policy and profile revisions.

### 2. Purpose
Remove the one-development-policy restriction without guessing authority.

### 3. Exact scope of code changes
Accept exact selected references according to the approved API decision; resolve multiple revisions deterministically and require explicit selection when ambiguous.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_cli/src/cli.rs`
- `crates/eqm_manifest/src/validation.rs`
- `crates/eqm_manifest/tests/`

### 5. Unit tests or integration tests required
Multiple stored policies/revisions no longer force arbitrary selection; ambiguous/missing authority rejects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Explicit policy selection works across development and protected modes.

### 8. Commit message
```text
feat: select exact policy and profile revisions
```

**Approved review regression contributions:** RA16.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s047"></a>
## S047 — Enumerate required evaluation coordinates

**Predecessor:** S046. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59, R60. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059, AC060.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enumerate required evaluation coordinates.

### 2. Purpose
Establish the full denominator before reading evidence.

### 3. Exact scope of code changes
Introduce a pure required-universe result for participating targets/providers/scenarios and profile/facet coordinates using approved authority.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Expected coordinates are complete and deterministic for asymmetric/shared-provider/repeated-actor fixtures.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Later evidence filtering cannot shrink required scope.

### 8. Commit message
```text
feat: enumerate the required evaluation universe
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s048"></a>
## S048 — Retain unknown applicability coordinates

**Predecessor:** S047. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59, R60. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059, AC060.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Retain unknown applicability coordinates.

### 2. Purpose
Repair skipped requirements that can disappear from completeness.

### 3. Exact scope of code changes
Carry explicit unknown and justified not-applicable coordinates alongside derived obligations; preserve requirement references and reasons.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/src/applicability.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Missing profile context remains in scope and cannot yield a vacuous conformant target.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Unknown applicability is visible and non-authorizing.

### 8. Commit message
```text
fix: retain unresolved applicability in evaluation scope
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s049"></a>
## S049 — Validate completeness construction

**Predecessor:** S048. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R23, R24, R26, R50, R58, R59, R60, R63. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC023, AC024, AC026, AC050, AC058, AC059, AC060, AC063.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md), [implementation/DEVIATIONS.md](../implementation/DEVIATIONS.md).

### 1. Step title
Validate completeness construction.

### 2. Purpose
Replace freely asserted completeness with a checked boundary.

### 3. Exact scope of code changes
Apply DEV-001: preserve the stronger fragment/journey inherited risk, reject explicitly lower requirement risk, and revalidate the expanded graph before finalization. Close unchecked finalized-graph construction with a justified acyclic crate boundary. Add a validated prepared-scope constructor accounting for every expected coordinate and participant; restrict unchecked construction and deserialization of completeness.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/tests/`
- `crates/eqm_engine/src/expand.rs`
- `crates/eqm_engine/src/invariants.rs`
- `crates/eqm_domain/src/graph.rs`
- `crates/eqm_manifest/src/canonical.rs`

### 5. Unit tests or integration tests required
Test absent and explicit fragment risks in both lower/higher destination journeys, invalid expanded graphs, public construction bypasses, omitted/duplicate coordinates, and complete failing/unknown scope. Exercise the real manifest-to-expansion-to-preparation path; use compile-fail/API tests where appropriate.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** ADR-0012 records owner approval; do not re-request the settled direction. S006 closes exact engineering representations and any newly evidenced conflicting consumer obligation. Live product/authority values remain governed by Q01-Q15; DEV-001 defines the S049 amendment.

### 7. Expected result
Complete evaluation cannot be forged by a boolean or partial map.

### 8. Commit message
```text
feat: validate complete evaluation scope
```

**Approved review regression contributions:** RA01, RA05, RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s050"></a>
## S050 — Reject accidental empty release scope

**Predecessor:** S049. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59, R60. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059, AC060.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Reject accidental empty release scope.

### 2. Purpose
Avoid vacuous release success and meaningless comparison claims.

### 3. Exact scope of code changes
Add explicit admission semantics for empty required release scope and singleton comparisons according to protected policy; retain local inspection usefulness.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_engine/src/equivalence.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Empty unauthorized release is non-authorizing; singleton is labeled no meaningful comparison; approved explicit exception is not invented by defaults.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Empty work cannot become a release pass.

### 8. Commit message
```text
fix: reject vacuous release assurance
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s051"></a>
## S051 — Compose policy constraints per facet

**Predecessor:** S050. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R23, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC023, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Compose policy constraints per facet.

### 2. Purpose
Prevent unrelated rule strengths from leaking across facets.

### 3. Exact scope of code changes
Derive facet-specific matching rule sets and strength using the approved normative contract. Compare protected obligation-universe inclusion plus per-coordinate constraints when checking policy strengthening; increasing a minimum-level selection floor is not automatically stronger. Preserve all matched-rule reasons.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Test different facets with different trust/count/age constraints and compare derived universes before/after policy changes, including Recommended-to-Required selection-floor changes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Per-facet constraints match authored policy intent.

### 8. Commit message
```text
fix: compose obligation strength per facet
```

**Approved review regression contributions:** RA04, RA07.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s052"></a>
## S052 — Derive provider and end-to-end dependencies

**Predecessor:** S051. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Derive provider and end-to-end dependencies.

### 2. Purpose
Build complete non-target scopes for later aggregation.

### 3. Exact scope of code changes
Emit exact provider/scenario obligations with participant/dependency linkage; do not flatten cross-target coverage into separate target passes.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Missing provider/end-to-end evidence remains independently required; repeated same-target actors are preserved.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
All approved scope kinds are present in the required universe.

### 8. Commit message
```text
feat: derive exact cross-runtime obligations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s053"></a>
## S053 — Bind expected selection manifests

**Predecessor:** S052. **Contracts:** R08, R11, R12, R13, R14, R15, R16, R17, R18, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Bind expected selection manifests.

### 2. Purpose
Make missing test cases observable independently of result files.

### 3. Exact scope of code changes
Represent and derive required logical case/observation selection manifests from approved binding/specification policy; reject ambiguous count units.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence.rs`
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Report with all reported cases passing but an expected case missing remains incomplete; selectors/count units stay exact.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
The evaluator knows what should have run before trusting what was reported.

### 8. Commit message
```text
feat: bind expected evidence selections
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s054"></a>
## S054 — Generate complete public obligation identities

**Predecessor:** S053. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md).

### 1. Step title
Generate complete public obligation identities.

### 2. Purpose
Fix IDs that omit profiles and release/candidate context.

### 3. Exact scope of code changes
Replace ad hoc string formatting with a versioned complete coordinate identity/projection; update query DTO references coherently.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Profile/revision/subject/context changes cannot collide; deterministic identical coordinates retain identity.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Public references uniquely identify the actual obligation.

### 8. Commit message
```text
fix: include full context in obligation identities
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s055"></a>
## S055 — Prepare structural observations independently

**Predecessor:** S054. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R24, R28, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC024, AC028, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Prepare structural observations independently.

### 2. Purpose
Avoid echoing authored artifact roles as observations.

### 3. Exact scope of code changes
Define a read-only observation boundary that supplies validated file kind/containment and independently supported roles; absent role authority stays unknown.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/structure.rs`
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/check.rs`

### 5. Unit tests or integration tests required
A declared service role on an unverified file does not prove role match; independent adapter roles are checked.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Structural evaluation consumes observations rather than self-confirming declarations.

### 8. Commit message
```text
fix: separate authored roles from structural observations
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s056"></a>
## S056 — Require trusted complete inventory for absence

**Predecessor:** S055. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R24, R28, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC024, AC028, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Require trusted complete inventory for absence.

### 2. Purpose
Preserve three-valued observation semantics.

### 3. Exact scope of code changes
Consolidate inventory completeness/currentness/trust prerequisites in prepared supporting facts; retain provenance and absence/unknown distinction.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/exposure.rs`
- `crates/eqm_runner/src/inventory.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Partial/missing/stale/untrusted inventories cannot prove absence; complete verified inventory can.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Presence/absence conclusions have explicit supporting authority.

### 8. Commit message
```text
fix: preserve inventory absence prerequisites
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s057"></a>
## S057 — Add normative derivation property tests

**Predecessor:** S056. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R23, R50, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC023, AC050, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Add normative derivation property tests.

### 2. Purpose
Test policy rules independently of orchestration shortcuts.

### 3. Exact scope of code changes
Add table/property coverage for ordering, unknown scope, empty scope, monotonic policy composition and protected candidate deletions using reference expected universes.

### 4. Files/modules likely involved
- `crates/eqm_engine/tests/obligations.rs`
- `crates/eqm_engine/tests/properties.rs`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Input permutations preserve identities; stronger requirements never shrink required work; omissions never increase assurance.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Derivation invariants are executable and independently reviewable.

### 8. Commit message
```text
test: lock complete obligation derivation semantics
```

**Approved review regression contributions:** RA04, RA05, RA07.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s058"></a>
## S058 — Expose complete obligation query DTOs

**Predecessor:** S057. **Contracts:** R08, R11, R12, R13, R14, R16, R19, R21, R44, R45, R52, R58, R59. **Acceptance:** AC008, AC011, AC012, AC013, AC014, AC016, AC019, AC021, AC044, AC045, AC052, AC058, AC059.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Expose complete obligation query DTOs.

### 2. Purpose
Make required/unresolved/non-applicable scope visible to consumers.

### 3. Exact scope of code changes
Map the prepared universe into versioned protocol records with reasons and context without loading/evaluating evidence or inventing final statuses.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/query.rs`
- `crates/eqm_cli/src/commands/obligations.rs`
- `schemas/`

### 5. Unit tests or integration tests required
Query contains every required coordinate and explicit pending/unknown facts; filtering is presentation-only.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

### 7. Expected result
Obligation inspection truthfully explains scope before acquisition.

### 8. Commit message
```text
feat: expose complete obligation scope views
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s059"></a>
## S059 — Convert protocol evidence through one validated boundary

**Predecessor:** S058. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R19, R21, R24, R29, R43, R64. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC021, AC024, AC029, AC043, AC064.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Convert protocol evidence through one validated boundary.

### 2. Purpose
Remove divergent domain construction paths.

### 3. Exact scope of code changes
Provide a single checked DTO-to-domain conversion for all currently supported evidence payloads and lineage; no trust conclusion is created by parsing.

### 4. Files/modules likely involved
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Each valid kind converts; inconsistent or unsupported payload/subject/identity fields reject deterministically.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_domain -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Engine inputs share one strict conversion contract.

### 8. Commit message
```text
refactor: centralize validated evidence conversion
```

**Approved review regression contributions:** RA02, RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s060"></a>
## S060 — Classify duplicate transport and immutable conflicts

**Predecessor:** S059. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R22, R24, R29, R35, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC022, AC024, AC029, AC035, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Classify duplicate transport and immutable conflicts.

### 2. Purpose
Correct duplicate-ID poisoning without hiding contradictory attempts.

### 3. Exact scope of code changes
Add pure candidate-set normalization that deduplicates identical semantic delivery and distinguishes same-attempt conflicts; retain all raw/provenance references.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/coverage.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Duplicate transport leaves counts/status unchanged; conflicting authenticated same-attempt candidates remain non-authorizing in affected scope.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Harmless duplicate delivery no longer manufactures unknown coverage.

### 8. Commit message
```text
fix: distinguish duplicate evidence from attempt conflicts
```

**Approved review regression contributions:** RA03, RA08, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s061"></a>
## S061 — Match complete evidence coordinates

**Predecessor:** S060. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R19, R21, R24, R29, R35, R43, R64. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC021, AC024, AC029, AC035, AC043, AC064.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Match complete evidence coordinates.

### 2. Purpose
Bind every required coverage axis before aggregation.

### 3. Exact scope of code changes
Update coverage expectations for exact component digests, subject manifests, profiles, participant roles and campaign/selection lineage; preserve rejected-axis explanations.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/coverage.rs`
- `crates/eqm_engine/tests/coverage.rs`

### 5. Unit tests or integration tests required
One-axis substitution tests reject wrong build/spec/role/profile/runner/context; irrelevant rejected candidates cannot create success.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Only exact compatible evidence reaches outcome evaluation.

### 8. Commit message
```text
fix: match complete conformance evidence coordinates
```

**Approved review regression contributions:** RA03, RA08, RA13.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s062"></a>
## S062 — Validate distinct logical observations

**Predecessor:** S061. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate distinct logical observations.

### 2. Purpose
Prevent file/attempt multiplication from meeting counts.

### 3. Exact scope of code changes
Implement approved logical-case identity and expected-selection completeness checks across normalized evidence; keep distinct case observations separate from retries.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/tests/outcomes.rs`

### 5. Unit tests or integration tests required
Split reports, duplicate imports and ten retries cannot replace missing distinct expected cases.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Count credit reflects the policy-defined logical unit.

### 8. Commit message
```text
fix: count distinct selected observations
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s063"></a>
## S063 — Apply policy-composed count constraints

**Predecessor:** S062. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Apply policy-composed count constraints.

### 2. Purpose
Resolve inconsistency between specification counts and stronger policy minimums.

### 3. Exact scope of code changes
Apply the coordinate's approved count semantics and strongest relevant minimum during evaluation, independently of runner acquisition summaries.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Policy minimum exceeding specification minimum remains missing; exact counts meet thresholds without file-count shortcuts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Policy count constraints cannot be bypassed by lower acquisition thresholds.

### 8. Commit message
```text
fix: enforce policy evidence count constraints
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s064"></a>
## S064 — Preserve retry history within a campaign

**Predecessor:** S063. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Preserve retry history within a campaign.

### 2. Purpose
Keep failure/pass instability visible.

### 3. Exact scope of code changes
Aggregate all immutable attempts for each selected case within the authorized campaign; implement canonical unknown/unstable/failure precedence and retain reasons.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/evidence_result.rs`
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Fail-then-pass is unstable; timeout/cancel remains unknown where normative; order permutations do not erase history.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Retrying never silently converts an unstable campaign into clean success.

### 8. Commit message
```text
fix: retain campaign retry history
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s065"></a>
## S065 — Evaluate manual review payloads explicitly

**Predecessor:** S064. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate manual review payloads explicitly.

### 2. Purpose
Remove the possibility of catch-all success for failed review.

### 3. Exact scope of code changes
Use explicit manual-review outcome/count/authority semantics in the canonical evaluator and reject unsupported interpretation.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/tests/outcomes.rs`

### 5. Unit tests or integration tests required
Passed, failed, skipped, quarantined, canceled and unverified review fixtures follow the normative table.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Manual review cannot become satisfied merely because its payload parsed.

### 8. Commit message
```text
fix: evaluate manual review outcomes explicitly
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s066"></a>
## S066 — Evaluate executable payload selections explicitly

**Predecessor:** S065. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate executable payload selections explicitly.

### 2. Purpose
Preserve skipped filtered and quarantined case semantics.

### 3. Exact scope of code changes
Unify tests/structural-check/snapshot payload aggregation over validated expected selections and attempts; prohibit terminal zero-exit shortcuts.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_runner/src/normalized.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Passed-with-missing/skipped/filtered/quarantined cases, zero selection and malformed counts remain non-success as specified.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Executable evidence uses the same complete case semantics across consumers.

### 8. Commit message
```text
fix: preserve executable evidence selection outcomes
```

**Approved review regression contributions:** RA02, RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s067"></a>
## S067 — Evaluate inventory and runtime payloads explicitly

**Predecessor:** S066. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate inventory and runtime payloads explicitly.

### 2. Purpose
Do not treat non-test payload existence as success.

### 3. Exact scope of code changes
Define supported inventory/runtime/release-reference payload evaluation with exact counts, referenced objects, completeness and supporting prerequisites; reject unspecified kinds.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/outcomes.rs`
- `crates/eqm_engine/src/exposure.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Incomplete inventory, failing counts, missing referenced record and wrong subject cannot satisfy an obligation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
All supported payload families have closed tested semantics.

### 8. Commit message
```text
fix: evaluate non-test evidence prerequisites
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s068"></a>
## S068 — Select campaigns under explicit authority

**Predecessor:** S067. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Select campaigns under explicit authority.

### 2. Purpose
Avoid arbitrary timestamp order and cherry-picked passing runs.

### 3. Exact scope of code changes
Implement pure campaign eligibility/ordering/supersession selection from the approved policy; return ambiguous or missing selection explicitly.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_engine/tests/selection.rs`
- `crates/eqm_domain/src/policy.rs`

### 5. Unit tests or integration tests required
Forged newer timestamps do not replace authority; a later authorized campaign can be selected without erasing older failures.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Current evidence selection is reproducible and audit history remains separate.

### 8. Commit message
```text
feat: select authorized evidence campaigns
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s069"></a>
## S069 — Evaluate exact freshness with explicit clock semantics

**Predecessor:** S068. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Evaluate exact freshness with explicit clock semantics.

### 2. Purpose
Align all consumers on temporal validity.

### 3. Exact scope of code changes
Consolidate age/skew and component-key freshness checks using approved precision and boundary rules; distinguish historical context from current checks without cache integration yet.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/freshness.rs`
- `crates/eqm_engine/tests/freshness.rs`

### 5. Unit tests or integration tests required
Exact expiry/skew boundaries, future times and each changed digest axis produce specified fresh/stale/unknown results.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Evidence freshness is deterministic for explicit input time and context.

### 8. Commit message
```text
fix: unify exact evidence freshness semantics
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s070"></a>
## S070 — Represent independent verification facts

**Predecessor:** S069. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R26, R29, R43, R50, R63. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC026, AC029, AC043, AC050, AC063.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Represent independent verification facts.

### 2. Purpose
Keep missing verification distinct from low verified trust.

### 3. Exact scope of code changes
Define the pure engine input for checked identity/issuer/subject/producer/temporal dimensions and their required completeness; no real cryptography in the engine.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_domain/src/`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
A missing mandatory dimension remains unknown; known insufficient authority remains non-success; producer claims cannot raise facts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Later verifier adapters must supply complete independent acceptance facts.

### 8. Commit message
```text
feat: model independent evidence verification facts
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s071"></a>
## S071 — Apply only precise contributing waivers

**Predecessor:** S070. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R19, R24, R29, R43, R58. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC019, AC024, AC029, AC043, AC058.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Apply only precise contributing waivers.

### 2. Purpose
Prevent unrelated or expired exceptions changing outcomes.

### 3. Exact scope of code changes
Route canonical waiver evaluation to exact scope/profile/subject/context and protected approval inputs; preserve baseline non-waivable unknown/unstable cases.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/waivers.rs`
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/tests/waivers.rs`

### 5. Unit tests or integration tests required
Expired, broad, unrelated, wrong-profile or unapproved waivers reject; only eligible blockers produce visible conditional state.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Waivers never become satisfied evidence or a graph-wide blanket list.

### 8. Commit message
```text
fix: bind waivers to exact contributing obligations
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s072"></a>
## S072 — Derive canonical facet decisions with full reasons

**Predecessor:** S071. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Derive canonical facet decisions with full reasons.

### 2. Purpose
Make one normative status implementation explainable.

### 3. Exact scope of code changes
Combine validated applicability, context, coverage, outcomes, freshness, verification, supporting checks and waivers into a structured facet decision retaining reasons.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/src/diagnostics.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Exhaustive precedence table tests include simultaneous unknown/failure/unstable/stale/missing and applied-waiver cases.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Summary precedence is explicit and no cause is lost through a single color/status.

### 8. Commit message
```text
refactor: derive explainable canonical facet decisions
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s073"></a>
## S073 — Aggregate complete participant conformance

**Predecessor:** S072. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Aggregate complete participant conformance.

### 2. Purpose
Remove target-only partial-map assurance.

### 3. Exact scope of code changes
Aggregate facets through validated universe and provider/scenario dependency participation; replace caller-supplied policy_resolution_complete booleans at internal callers.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/conformance.rs`
- `crates/eqm_engine/tests/`
- `crates/eqm_application/src/evaluation.rs`

### 5. Unit tests or integration tests required
Missing provider/end-to-end/unknown coordinates block appropriate participants; complete failures remain valid complete decisions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
No target is graded using a silently incomplete obligation subset.

### 8. Commit message
```text
fix: aggregate complete participant conformance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s074"></a>
## S074 — Derive contract-relative comparison conclusions

**Predecessor:** S073. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Derive contract-relative comparison conclusions.

### 2. Purpose
Compare required cohorts without conflating build equality.

### 3. Exact scope of code changes
Update equivalence evaluation to use approved comparison context and complete subject/cohort results; require declared relational evidence rather than inventing a general comparison language.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/equivalence.rs`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Different legitimate builds compare; missing relational scenario or wrong cohort remains blocking; extra targets do not change required scope.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Equivalence claims are scoped and supported by the required relational evidence.

### 8. Commit message
```text
fix: scope equivalence to complete comparison contracts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s075"></a>
## S075 — Assemble a simple reference evaluation path

**Predecessor:** S074. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R24, R29, R43. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC024, AC029, AC043.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Assemble a simple reference evaluation path.

### 2. Purpose
Provide an independent semantic oracle for future optimization.

### 3. Exact scope of code changes
Compose the tested pure operations into an unoptimized reference evaluation over frozen fixtures with explicit provenance/reasons; keep no alternative production policy interpretation.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/`
- `crates/eqm_engine/tests/`
- `crates/eqm_test_support/src/`

### 5. Unit tests or integration tests required
Golden lifecycle cases match manually specified outcomes and all non-success states; input permutations give identical decisions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
A readable reference path establishes expected whole-evaluation behavior.

### 8. Commit message
```text
test: establish reference conformance evaluation
```

**Approved review regression contributions:** RA03.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s076"></a>
## S076 — Lock adversarial evidence regression fixtures

**Predecessor:** S075. **Contracts:** R11, R13, R14, R15, R16, R17, R18, R23, R24, R29, R35, R43, R50. **Acceptance:** AC011, AC013, AC014, AC015, AC016, AC017, AC018, AC023, AC024, AC029, AC035, AC043, AC050.

**Read:** [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Lock adversarial evidence regression fixtures.

### 2. Purpose
Validate the complete semantic slice before persistence and host work.

### 3. Exact scope of code changes
Add full fixture campaigns covering duplicates/conflicts/retries/counts/unknown scopes/mixed subjects and real non-test failures; update normative docs when representation choices are resolved.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `crates/eqm_engine/tests/`
- `docs/eqm_product_v1/specs/EVALUATION_SEMANTICS.md`

### 5. Unit tests or integration tests required
Reference path passes the adversarial matrix and stronger-policy/deleted-evidence monotonicity checks.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Semantic defects from the review have focused regression coverage before command routing.

### 8. Commit message
```text
test: cover adversarial evidence evaluation
```

**Approved review regression contributions:** RA03, RA05, RA08.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s077"></a>
## S077 — Select and pin the storage implementation

**Predecessor:** S076. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Select and pin the storage implementation.

### 2. Purpose
Establish safe runtime and durability assumptions before persistence code.

### 3. Exact scope of code changes
Resolve Q10 from actual dependencies/hosts; choose an audited exact SQLite wrapper/runtime containing the WAL-reset fix and record linking/features/durability design; update lockfile intentionally.

### 4. Files/modules likely involved
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_application/Cargo.toml`
- `docs/eqm_product_v1/decisions/`

### 5. Unit tests or integration tests required
Inspect linked SQLite runtime/fix and license/advisory results; compile chosen features; no network-filesystem assumption.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Storage uses a justified fixed audited runtime rather than an unchecked system dependency.

### 8. Commit message
```text
build: pin the reviewed local storage runtime
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s078"></a>
## S078 — Define store operations and typed failures

**Predecessor:** S077. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Define store operations and typed failures.

### 2. Purpose
Keep storage contracts independent of SQL and privilege assumptions.

### 3. Exact scope of code changes
Add bounded object/index/read-snapshot interfaces and typed storage errors for unavailable/corrupt/conflict/limit conditions; avoid a generic unrestricted file API.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/mod.rs`
- `crates/eqm_application/src/error.rs`

### 5. Unit tests or integration tests required
In-memory contract tests demonstrate missing objects and conflicts never produce verified/success states.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Application operations can rely on explicit storage semantics without trusting metadata.

### 8. Commit message
```text
refactor: define immutable evidence store boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s079"></a>
## S079 — Initialize protected store roots safely

**Predecessor:** S078. **Contracts:** R18, R22, R26, R33, R35, R36, R37, R38, R42, R50, R52. **Acceptance:** AC018, AC022, AC026, AC033, AC035, AC036, AC037, AC038, AC042, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Initialize protected store roots safely.

### 2. Purpose
Handle concurrent creation and confinement.

### 3. Exact scope of code changes
Create private staging/object/index roots with checked existing-directory behavior, concurrent AlreadyExists revalidation and explicit development/trusted location selection.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_runner/src/persistence.rs`

### 5. Unit tests or integration tests required
Concurrent first initialization succeeds safely; symlink/wrong-type roots and candidate-writable trusted configurations reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Store initialization is idempotent and does not race into unsafe destinations.

### 8. Commit message
```text
fix: initialize evidence storage safely
```

**Approved review regression contributions:** RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s080"></a>
## S080 — Write bounded immutable staging objects

**Predecessor:** S079. **Contracts:** R18, R22, R26, R33, R35, R36, R37, R38, R42, R50, R52. **Acceptance:** AC018, AC022, AC026, AC033, AC035, AC036, AC037, AC038, AC042, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Write bounded immutable staging objects.

### 2. Purpose
Prevent unbounded buffering before persistence validation.

### 3. Exact scope of code changes
Stream or bounded-read staged objects, compute raw digest, validate limits and retain handles; keep new objects invisible to the index.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/objects.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Oversized/changing input and short writes fail without publication; exact bytes/digest are retained for valid input.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Staging cannot allocate or publish unbounded unvalidated evidence.

### 8. Commit message
```text
feat: stage bounded immutable evidence objects
```

**Approved review regression contributions:** RA12, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s081"></a>
## S081 — Install objects durably without clobbering

**Predecessor:** S080. **Contracts:** R18, R22, R26, R33, R35, R36, R37, R38, R42, R50, R52. **Acceptance:** AC018, AC022, AC026, AC033, AC035, AC036, AC037, AC038, AC042, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Install objects durably without clobbering.

### 2. Purpose
Make object publication crash-safe and collision-aware.

### 3. Exact scope of code changes
Implement durable no-clobber install and supported-host file/directory sync behavior; bound existing-file comparisons and retain raw collision distinction.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/objects.rs`
- `crates/eqm_runner/src/persistence.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Identical installs are idempotent; different bytes under same raw identity reject; oversized existing files are not read unbounded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
An installed object is immutable and safely published before any index reference.

### 8. Commit message
```text
feat: publish immutable objects durably
```

**Approved review regression contributions:** RA12, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s082"></a>
## S082 — Create versioned local metadata tables

**Predecessor:** S081. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Create versioned local metadata tables.

### 2. Purpose
Introduce the smallest index schema matching established identities.

### 3. Exact scope of code changes
Add schema version metadata and tables/constraints for raw/semantic objects, execution lineage and committed references; do not store trusted booleans as authority.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/src/storage/migrations/`

### 5. Unit tests or integration tests required
Fresh initialization/version mismatch/constraint tests pass; incompatible writers reject explicitly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Local metadata has explicit versioning and identity constraints.

### 8. Commit message
```text
feat: initialize the versioned evidence index
```

**Approved review regression contributions:** RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s083"></a>
## S083 — Commit import references atomically

**Predecessor:** S082. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Commit import references atomically.

### 2. Purpose
Enforce object-first/index-second publication.

### 3. Exact scope of code changes
Add transactions that reference already validated installed objects and atomically index semantic/execution relationships; no visibility before commit.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Inject rollback before commit and reopen; no completed row references unpublished bytes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Import publication has a clear atomic visibility boundary.

### 8. Commit message
```text
feat: commit evidence references atomically
```

**Approved review regression contributions:** RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s084"></a>
## S084 — Persist idempotent imports and attempt conflicts

**Predecessor:** S083. **Contracts:** R17, R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC017, AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Persist idempotent imports and attempt conflicts.

### 2. Purpose
Make replay safety survive process restarts.

### 3. Exact scope of code changes
Enforce uniqueness by appropriate semantic/attempt identity, retain alternate raw transports, and persist contradictory attempt conflicts instead of run-only rejection.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_runner/src/ci.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Reimport after restart stays idempotent; many results from one CI run work; same-attempt contradictions remain recorded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Replay handling is persistent, precise and not overly coarse.

### 8. Commit message
```text
fix: persist exact evidence replay identities
```

**Approved review regression contributions:** RA08, RA15.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s085"></a>
## S085 — Persist campaigns and execution receipts

**Predecessor:** S084. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Persist campaigns and execution receipts.

### 2. Purpose
Preserve history and selection authority across restarts.

### 3. Exact scope of code changes
Store plan/campaign/case/attempt lineage, terminal worker receipts and explicit supersession/selection metadata with validated references.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Interrupted acquisitions remain distinguishable from completed campaigns; old attempts survive new campaign selection.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Execution history supports deterministic current selection and audit.

### 8. Commit message
```text
feat: persist evidence campaign history
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s086"></a>
## S086 — Store runtime facts and release subject records

**Predecessor:** S085. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Store runtime facts and release subject records.

### 2. Purpose
Unify bounded validated supporting inputs.

### 3. Exact scope of code changes
Index immutable runtime observations, candidate/subject manifests and release-related records by exact subjects/context; reject malformed referenced objects.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Wrong-subject, missing-reference and duplicate delivery cases behave consistently with evidence objects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Supporting facts can be loaded coherently rather than enumerating arbitrary directories.

### 8. Commit message
```text
feat: store exact runtime and release inputs
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s087"></a>
## S087 — Persist verification receipts without minting authority

**Predecessor:** S086. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Persist verification receipts without minting authority.

### 2. Purpose
Retain cryptographic context for replay and current checks.

### 3. Exact scope of code changes
Store receipts and supporting material references as records requiring verification on import; expose raw facts through a checked retrieval path.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/src/trust.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Forged serialized verified fields are not accepted; missing trust material remains unavailable; originals remain immutable.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Storage preserves receipts but cannot convert them into live authority.

### 8. Commit message
```text
feat: retain verification receipt provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s088"></a>
## S088 — Read consistent index snapshots

**Predecessor:** S087. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Read consistent index snapshots.

### 2. Purpose
Prevent mixed-generation evaluation inputs.

### 3. Exact scope of code changes
Add bounded read transactions/snapshot generation handles and validate referenced immutable objects before preparing inputs; do not hold transactions during test execution.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/src/evaluation.rs`

### 5. Unit tests or integration tests required
Concurrent import during preparation cannot alter selected input generation; missing/corrupt objects fail explicitly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Evaluation preparation observes one coherent store state.

### 8. Commit message
```text
feat: read consistent evidence snapshots
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s089"></a>
## S089 — Implement indexed coordinate queries

**Predecessor:** S088. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Implement indexed coordinate queries.

### 2. Purpose
Avoid scanning and parsing every evidence file.

### 3. Exact scope of code changes
Query by exact subject/specification/profile/campaign/attempt coordinates using appropriate indexes; preserve deterministic ordering and rejected-record explanations.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Indexed queries match a full-scan reference on synthetic histories, including conflicts and irrelevant records.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Evidence lookup is precise and testably equivalent to the simple reference.

### 8. Commit message
```text
feat: index exact evidence coordinates
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s090"></a>
## S090 — Bound writers and own checkpoints

**Predecessor:** S089. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Bound writers and own checkpoints.

### 2. Purpose
Make SQLite contention behavior explicit.

### 3. Exact scope of code changes
Implement short writer transactions, bounded busy handling and checkpoint ownership/configuration; refuse unsupported local-store configurations as specified.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/sqlite.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Concurrent readers/writers/checkpoints have bounded outcomes and no silent partial commits; linked runtime check is exercised.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
WAL operation has explicit contention and durability guarantees.

### 8. Commit message
```text
fix: bound evidence index contention
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s091"></a>
## S091 — Recover staged and orphaned objects

**Predecessor:** S090. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Recover staged and orphaned objects.

### 2. Purpose
Make crash cleanup safe rather than destructive.

### 3. Exact scope of code changes
Implement recovery classification for staged/orphan/committed objects with conservative handling of active operations; never fabricate missing committed objects.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/recovery.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Faults at each publication boundary recover coherently; live staged work is not deleted and missing committed bytes are reported.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Restart recovery does not produce phantom completed evidence.

### 8. Commit message
```text
feat: recover interrupted evidence publication
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s092"></a>
## S092 — Protect live readers and acquisitions with leases

**Predecessor:** S091. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Protect live readers and acquisitions with leases.

### 2. Purpose
Prevent cleanup racing active use.

### 3. Exact scope of code changes
Add explicit bounded lease/reference ownership for ongoing reads/acquisitions and release archives, with tested expiry/recovery semantics from operator policy.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/retention.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Concurrent cleanup cannot remove objects used by readers/runs; stale orphan lease handling follows explicit rules.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Object lifecycle accounts for active consumers and workers.

### 8. Commit message
```text
feat: protect live evidence references
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s093"></a>
## S093 — Implement policy-driven retention and GC

**Predecessor:** S092. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Implement policy-driven retention and GC.

### 2. Purpose
Reclaim only authorized unreachable data.

### 3. Exact scope of code changes
Add retention classes and reachability-based cleanup using configured policy and leases; preserve required signed originals/release archives and sensitive-data boundaries.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/retention.rs`
- `crates/eqm_application/tests/`
- `docs/eqm_product_v1/runbooks/`

### 5. Unit tests or integration tests required
Class/lease/archive tests prevent premature deletion; eligible orphans/history are reclaimed deterministically.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Immutability is compatible with safe explicit retention, not infinite unbounded growth.

### 8. Commit message
```text
feat: reclaim eligible evidence objects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s094"></a>
## S094 — Rebuild indexes from retained manifests

**Predecessor:** S093. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Rebuild indexes from retained manifests.

### 2. Purpose
Make the index replaceable without losing trust/history.

### 3. Exact scope of code changes
Add bounded reconstruction from immutable manifests/receipts with conflict preservation and validation; do not trust filenames or create receipt authority during rebuild.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/recovery.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Delete only a disposable fixture index and rebuild; identities, history, conflicts and decisions match original records.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Recovery can reconstruct discoverability without inventing accepted evidence.

### 8. Commit message
```text
feat: rebuild evidence indexes safely
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s095"></a>
## S095 — Test explicit database migration and restore

**Predecessor:** S094. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Test explicit database migration and restore.

### 2. Purpose
Verify compatibility before schema evolution.

### 3. Exact scope of code changes
Implement approved migration coordination/backup-or-restore mechanics and one controlled fixture transition; retain original immutable/signature objects.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/migrations/`
- `crates/eqm_application/tests/`
- `docs/eqm_product_v1/specs/COMPATIBILITY.md`

### 5. Unit tests or integration tests required
Upgrade, interruption, reopen, incompatible writer and rollback/restore tests pass on declared hosts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Database version transitions are atomic or recoverable and never silent reinterpretations.

### 8. Commit message
```text
test: verify evidence store migration recovery
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s096"></a>
## S096 — Complete storage fault-injection coverage

**Predecessor:** S095. **Contracts:** R18, R22, R26, R35, R36, R37, R38, R50, R52. **Acceptance:** AC018, AC022, AC026, AC035, AC036, AC037, AC038, AC050, AC052.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Complete storage fault-injection coverage.

### 2. Purpose
Close persistence guarantees before production routing.

### 3. Exact scope of code changes
Add publication/transaction/checkpoint/cleanup interruption points and bounded corrupt-file fixtures; document which hosts/filesystems were actually tested.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
AC018/AC033/AC035–AC038/AC052 storage cases pass on supported hosts or remain explicit blocked gates.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Resolve Q10 before the first dependency change. Q02/Q07 determine host guarantees and retention values; missing required host execution evidence blocks that readiness claim.

### 7. Expected result
Store readiness is supported by failure tests rather than filesystem assumptions.

### 8. Commit message
```text
test: cover evidence storage failure boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s097"></a>
## S097 — Define host execution capabilities

**Predecessor:** S096. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Define host execution capabilities.

### 2. Purpose
Stop treating authored guarantees as operator authority.

### 3. Exact scope of code changes
Add explicit host/backend capability configuration and validation, including paths, permissions, network/secrets and limits; declarations cannot self-authorize capabilities.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/resolution.rs`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/tests/`

### 5. Unit tests or integration tests required
Unsupported guarantees reject before spawn; local declarations cannot claim isolation not supplied by the host.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Resolution is constrained by independent host authority.

### 8. Commit message
```text
fix: resolve runners against actual host capabilities
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s098"></a>
## S098 — Build complete non-executing acquisition plans

**Predecessor:** S097. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Build complete non-executing acquisition plans.

### 2. Purpose
Make dry-run a concrete safety artifact.

### 3. Exact scope of code changes
Resolve executable identity, argv, cwd, selector/expected cases, environment references, limits and outputs into a deterministic plan without launching programs.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/substitution.rs`
- `crates/eqm_cli/src/commands/verify.rs`

### 5. Unit tests or integration tests required
Spawn/network spies stay unused; every required plan field is present; unresolved executables and selectors fail explicitly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Dry-run reveals exact work rather than only specification IDs.

### 8. Commit message
```text
feat: prepare concrete acquisition plans
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s099"></a>
## S099 — Bind explicit approval to exact plans

**Predecessor:** S098. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Bind explicit approval to exact plans.

### 2. Purpose
Prevent execution after meaningful plan substitution.

### 3. Exact scope of code changes
Introduce a checked approval boundary for plan identity/authority; verify approved immutable inputs immediately before execution and reject changed executable/argv/context.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/resolution.rs`
- `crates/eqm_runner/tests/`

### 5. Unit tests or integration tests required
Changing any approved plan axis invalidates approval; read-only callers cannot create execution permission.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Authorized execution is tied to the plan the operator reviewed.

### 8. Commit message
```text
feat: bind execution approval to exact plans
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s100"></a>
## S100 — Centralize bounded confined file access

**Predecessor:** S099. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Centralize bounded confined file access.

### 2. Purpose
Remove validate-then-unbounded-read shortcuts.

### 3. Exact scope of code changes
Add a narrowly scoped safe handle-based filesystem utility for source/result/import reads using reviewed APIs; preserve byte/type/ancestor containment checks and document host differences.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_runner/src/`
- `crates/eqm_manifest/src/`
- `Cargo.toml`
- `Cargo.lock`

### 5. Unit tests or integration tests required
Parent-symlink, replacement, oversized read and invalid-type fixtures reject; any dependency/unsafe exception is reviewed before use.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Callers have one tested confinement and bounded-read boundary.

### 8. Commit message
```text
fix: centralize bounded confined file access
```

**Approved review regression contributions:** RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s101"></a>
## S101 — Validate result files after execution

**Predecessor:** S100. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate result files after execution.

### 2. Purpose
Ensure pre-created temp paths cannot bypass limits.

### 3. Exact scope of code changes
Replace verify fs::read with bounded handle acquisition and post-execution validation; reject replaced/symlink/oversized/malformed result files before normalization.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_runner/src/normalized.rs`
- `crates/eqm_application/src/acquisition.rs`

### 5. Unit tests or integration tests required
Worker replaces output path, emits huge file or inconsistent selector; all fail safely without unbounded allocation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Only safe bounded exact runner results enter evidence normalization.

### 8. Commit message
```text
fix: validate runner output files before loading
```

**Approved review regression contributions:** RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s102"></a>
## S102 — Prepare immutable worker input and output roots

**Predecessor:** S101. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Prepare immutable worker input and output roots.

### 2. Purpose
Separate candidate execution from authoritative state.

### 3. Exact scope of code changes
Materialize or verify approved immutable sources and dedicated staging outputs; expose no evaluator/authority/index/signing write access; record dirty development mode separately.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/src/container.rs`

### 5. Unit tests or integration tests required
Worker attempts writes to protected fixture roots fail or unsupported configuration rejects before launch; input mutation invalidates identity.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Trusted acquisition does not execute against mutable self-authorizing inputs.

### 8. Commit message
```text
feat: isolate worker inputs and output staging
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s103"></a>
## S103 — Resolve explicit host environments and secrets

**Predecessor:** S102. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Resolve explicit host environments and secrets.

### 2. Purpose
Remove hardcoded PATH and empty-secret wiring.

### 3. Exact scope of code changes
Provide operator-approved host environment and secret references through the application boundary; retain env_clear and avoid logging values; no implicit inheritance or retrieval.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_cli/src/commands/verify.rs`

### 5. Unit tests or integration tests required
Environment marker absence, exact allowlist, missing secret failure and host-specific PATH fixtures pass without real credentials.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Native acquisition gets explicit portable inputs without ambient privilege.

### 8. Commit message
```text
feat: resolve authorized runner environment inputs
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s104"></a>
## S104 — Signal output overflow immediately

**Predecessor:** S103. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Signal output overflow immediately.

### 2. Purpose
Stop draining unlimited output until process exit.

### 3. Exact scope of code changes
Use signaled bounded stdout/stderr readers and supervised bounded stdin delivery; activate cancellation/deadline ownership before any potentially blocking request write. Terminate promptly on overflow; every input-write error retains child/task cleanup ownership.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/src/adapter.rs`
- `crates/eqm_runner/tests/security.rs`

### 5. Unit tests or integration tests required
Exercise input beyond pipe capacity, a worker refusing stdin or writing before reading, early stdin closure, output floods and cancellation during request delivery. Assert prompt bounded termination and cleanup, not only retained buffer size.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Output caps constrain execution promptly, not only final classification.

### 8. Commit message
```text
fix: terminate acquisition on output overflow
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s105"></a>
## S105 — Supervise pipes and descendants through completion

**Predecessor:** S104. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Supervise pipes and descendants through completion.

### 2. Purpose
Close the parent-exit hang.

### 3. Exact scope of code changes
Keep deadline/cancellation active through input delivery, descendant ownership, output drainage and cleanup. Implement bounded joins and cleanup after immediate-child exit; handle broken pipes and every supervisor error path without orphaning owned work.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/tests/security.rs`

### 5. Unit tests or integration tests required
Test a child exiting while descendants retain pipes, cancellation/deadline during input/drain, broken-pipe cleanup and bounded shutdown on each supported native host.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Immediate-child exit cannot move blocking reader joins outside supervision.

### 8. Commit message
```text
fix: bound descendant and pipe shutdown
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s106"></a>
## S106 — Wire cooperative and host cancellation

**Predecessor:** S105. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Wire cooperative and host cancellation.

### 2. Purpose
Make cancellation reachable from real callers.

### 3. Exact scope of code changes
Propagate cancellation from application/CLI lifecycle into owned execution and bounded cleanup; specify completion/publication races and retain committed results discoverably.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_cli/src/main.rs`
- `crates/eqm_runner/src/execution.rs`

### 5. Unit tests or integration tests required
Cancel before spawn, during execution/drain and around publication; no detached work or contradictory committed state.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Cancellation is an explicit lifecycle outcome rather than an unused token.

### 8. Commit message
```text
feat: propagate acquisition cancellation safely
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s107"></a>
## S107 — Harden Unix process ownership

**Predecessor:** S106. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Harden Unix process ownership.

### 2. Purpose
Make Unix tree cleanup and errors explicit.

### 3. Exact scope of code changes
Use approved safe OS abstractions for owned process groups/session handling; remove fragile assumptions about external kill invocation where needed and document containment limits.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_runner/tests/security.rs`
- `docs/eqm_product_v1/specs/EXECUTION_SECURITY.md`

### 5. Unit tests or integration tests required
Unix-native child/grandchild/error cleanup tests pass; escape attempts are contained by approved host isolation or reject unsupported guarantees.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Unix supervision claims match tested host capability rather than best-effort direct-child kill.

### 8. Commit message
```text
fix: harden unix worker lifecycle ownership
```

**Approved review regression contributions:** RA11.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s108"></a>
## S108 — Implement supported Windows process ownership

**Predecessor:** S107. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Implement supported Windows process ownership.

### 2. Purpose
Avoid claiming tree termination from Child.kill alone.

### 3. Exact scope of code changes
After Q02/Q13, implement safe reviewed Windows Job Object ownership and kill-on-close/cancellation behavior, with explicit configuration for unsupported nested host cases.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/`
- `crates/eqm_runner/tests/security.rs`
- `Cargo.toml`
- `Cargo.lock`

### 5. Unit tests or integration tests required
Windows-native descendant/pipe/cancel/exit tests and target compilation pass; do not substitute Linux-only tests for this gate.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Windows support is implemented and evidenced, or recorded unsupported by an approved support-matrix decision.

### 8. Commit message
```text
feat: supervise supported windows worker trees
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s109"></a>
## S109 — Dispatch only supported acquisition backends

**Predecessor:** S108. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Dispatch only supported acquisition backends.

### 2. Purpose
Connect host-authorized execution paths coherently.

### 3. Exact scope of code changes
Add application dispatch for the repository's supported local/container/locked program mechanisms; unsupported resolution/execution capabilities return explicit errors without fallback.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/container.rs`
- `crates/eqm_runner/src/resolution.rs`

### 5. Unit tests or integration tests required
Backend selection, immutable locked identities, unavailable runtime and guarantee mismatch fixtures reject or execute exactly as approved.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Backend declarations correspond to real implemented capabilities.

### 8. Commit message
```text
feat: dispatch authorized acquisition backends
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s110"></a>
## S110 — Treat executable discovery as acquisition

**Predecessor:** S109. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Treat executable discovery as acquisition.

### 2. Purpose
Remove any read-only illusion around build-tool discovery.

### 3. Exact scope of code changes
Route discovery adapters that execute programs through approved planning/supervision; keep static bounded inspection separate and read-only.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/discover.rs`
- `crates/eqm_runner/src/adapter.rs`
- `crates/eqm_application/src/acquisition.rs`

### 5. Unit tests or integration tests required
A read-only discover request cannot run Gradle/scripts; explicitly approved discovery uses the same bounded worker path.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Discovery has the same privilege and result-validation rules as other execution.

### 8. Commit message
```text
fix: require authority for executable discovery
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s111"></a>
## S111 — Persist terminal acquisition receipts

**Predecessor:** S110. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Persist terminal acquisition receipts.

### 2. Purpose
Keep failures and canceled work visible in expected selections.

### 3. Exact scope of code changes
Record successful and failed/timeout/cancel/overflow terminal receipts with plan/campaign/case context where safely possible; keep malformed results as failures not absent successes.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_runner/src/normalized.rs`

### 5. Unit tests or integration tests required
Restart after failed acquisition retains expected missing/failed work; zero exit with malformed result never becomes evidence success.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Acquisition history explains terminal non-success across process restarts.

### 8. Commit message
```text
feat: retain terminal acquisition outcomes
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s112"></a>
## S112 — Bound scheduling and device ownership

**Predecessor:** S111. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Bound scheduling and device ownership.

### 2. Purpose
Establish safe concurrency before throughput optimization.

### 3. Exact scope of code changes
Add bounded work queues, concurrency admission and exclusive device/resource leases using explicit plan limits; deterministic result collection and owned shutdown.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Concurrent plans sharing a device do not overlap illegally; queue saturation/cancellation is explicit and no work is silently dropped.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Scheduling respects resource authority and cancellation.

### 8. Commit message
```text
feat: bound acquisition scheduling and device use
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s113"></a>
## S113 — Harden retained-output redaction

**Predecessor:** S112. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Harden retained-output redaction.

### 2. Purpose
Keep sensitive values out of plans reports and stored logs.

### 3. Exact scope of code changes
Centralize bounded secret redaction and safe error formatting, including truncation/multibyte/chunk-boundary cases and declared overlapping secrets; document limits against hostile exfiltration.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/execution.rs`
- `crates/eqm_application/src/error.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Test secrets spanning read chunks, truncated outputs, overlapping values and malformed UTF-8; no real secrets in fixtures.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Retained outputs obey the declared redaction contract without claiming sandbox security.

### 8. Commit message
```text
fix: harden bounded acquisition output redaction
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s114"></a>
## S114 — Enforce application-wide resource budgets

**Predecessor:** S113. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R44, R46, R47, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC044, AC046, AC047, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enforce application-wide resource budgets.

### 2. Purpose
Prevent graph/protocol/request workloads bypassing runner limits.

### 3. Exact scope of code changes
Apply configured byte/depth/expansion/profile/candidate/attachment/queue budgets at acquisition/preparation boundaries; no silent truncation of required scope.

### 4. Files/modules likely involved
- `crates/eqm_manifest/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/`
- `crates/eqm_runner/src/`

### 5. Unit tests or integration tests required
Oversized/deep/explosive fixtures terminate with explicit limit diagnostics and cannot authorize release.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_manifest -p eqm_protocol -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Bounded behavior covers the whole input pipeline.

### 8. Commit message
```text
feat: enforce conformance resource budgets
```

**Approved review regression contributions:** RA12, RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s115"></a>
## S115 — Complete host adversarial acquisition tests

**Predecessor:** S114. **Contracts:** R05, R19, R22, R30, R31, R32, R33, R34, R35, R41, R42, R51, R60. **Acceptance:** AC005, AC019, AC022, AC030, AC031, AC032, AC033, AC034, AC035, AC041, AC042, AC051, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Complete host adversarial acquisition tests.

### 2. Purpose
Evidence the promises made by each backend.

### 3. Exact scope of code changes
Consolidate environment/confinement/flood/descendant/cancel/source-mutation/protected-write tests for each declared host, updating capability docs honestly.

### 4. Files/modules likely involved
- `crates/eqm_runner/tests/security.rs`
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
Run native host/security lanes and record unavailable/unsupported guarantees; no new blanket capability claims.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask test security
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q02/Q08/Q13 must define the supported host guarantees. Run the native host lane for host-specific code; an approved unsupported-host disposition must be documented, not silently skipped.

### 7. Expected result
Acquisition security is backed by explicit host execution evidence.

### 8. Commit message
```text
test: verify adversarial worker boundaries
```

**Approved review regression contributions:** RA11, RA12.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s116"></a>
## S116 — Model protected authority configuration

**Predecessor:** S115. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Model protected authority configuration.

### 2. Purpose
Define authority independently of candidate files.

### 3. Exact scope of code changes
Add strict operator configuration for baseline expectations, required scope, verifier/evaluator identity and authorized trust dimensions; no live values or permissive defaults.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_domain/src/`

### 5. Unit tests or integration tests required
Missing authority, duplicate identities and candidate-only release origin reject; test-only configs remain labeled.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol -p eqm_domain
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Protected configuration is explicit validated input.

### 8. Commit message
```text
feat: configure protected evaluation authority
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s117"></a>
## S117 — Load protected baseline expectations

**Predecessor:** S116. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Load protected baseline expectations.

### 2. Purpose
Preserve protected contract participation and selections.

### 3. Exact scope of code changes
Acquire exact protected authority through an authorized loader independent of candidate paths; bind baseline contract/policy/participation/evidence mappings and profile values.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/selection.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Candidate policy/default/path changes cannot substitute authority; missing exact baseline fails closed.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Protected evaluation receives exact independent expectations.

### 8. Commit message
```text
feat: load exact protected evaluation baselines
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s118"></a>
## S118 — Detect protected scope weakening

**Predecessor:** S117. **Contracts:** R14, R23, R24, R25, R26, R29, R38, R51, R58, R64. **Acceptance:** AC014, AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC058, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Detect protected scope weakening.

### 2. Purpose
Include deletions and reclassification in policy comparison.

### 3. Exact scope of code changes
Extend monotonicity/diff checks to required targets, requirements, participation, critical test selections and evidence mappings; allow changes only through approved authority transition.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/monotonicity.rs`
- `crates/eqm_engine/src/diff.rs`
- `crates/eqm_application/src/authority.rs`

### 5. Unit tests or integration tests required
Deleting/reclassifying a critical requirement/test/target cannot shrink protected work; approved versioned transitions are explicit.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
A candidate cannot pass by deleting what would fail.

### 8. Commit message
```text
fix: detect protected conformance scope weakening
```

**Approved review regression contributions:** RA04.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s119"></a>
## S119 — Bind the trusted evaluator identity

**Predecessor:** S118. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Bind the trusted evaluator identity.

### 2. Purpose
Prevent candidate code from grading itself in release mode.

### 3. Exact scope of code changes
Include approved EQM evaluator identity/provenance in protected preparation and receipts; reject candidate-modified/unapproved grader execution as trusted.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Substituted grader binary/version/config cannot produce accepted trusted preparation merely using the same candidate workspace.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Release assurance is tied to independently approved grading infrastructure.

### 8. Commit message
```text
feat: bind protected evaluator identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s120"></a>
## S120 — Define structured verification adapter results

**Predecessor:** S119. **Contracts:** R11, R23, R24, R25, R26, R29, R38, R50, R51, R63, R64. **Acceptance:** AC011, AC023, AC024, AC025, AC026, AC029, AC038, AC050, AC051, AC063, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Define structured verification adapter results.

### 2. Purpose
Avoid a single trust-level integer standing for all checks.

### 3. Exact scope of code changes
Implement verifier request/result contracts covering raw bytes, identity/issuer, signer authorization, builder, subject, policy and temporal material; construction crosses trusted adapter boundary.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust.rs`
- `crates/eqm_runner/src/ci.rs`
- `crates/eqm_engine/src/conformance.rs`

### 5. Unit tests or integration tests required
Missing required dimension stays explicit; serialized verified flags cannot manufacture accepted facts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Verifiers return specific checked claims rather than unconditional SignedCi.

### 8. Commit message
```text
refactor: expose structured verification conclusions
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s121"></a>
## S121 — Pin a reviewed verification implementation

**Predecessor:** S120. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Pin a reviewed verification implementation.

### 2. Purpose
Connect real cryptography without custom primitives.

### 3. Exact scope of code changes
Inspect current Sigstore bundle support and select an audited exact library or safely supervised verifier tool behind the neutral interface; update pins/lock/ADR and feature implications.

### 4. Files/modules likely involved
- `Cargo.toml`
- `Cargo.lock`
- `crates/eqm_application/Cargo.toml`
- `crates/eqm_application/src/trust/`
- `docs/eqm_product_v1/decisions/`

### 5. Unit tests or integration tests required
License/advisory/feature checks pass; compile the real implementation; no disabled subject/claim verification flags.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
A production-capable cryptographic path exists without live authority being invented.

### 8. Commit message
```text
build: pin the reviewed evidence verifier
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s122"></a>
## S122 — Verify bundle integrity and exact signed bytes

**Predecessor:** S121. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Verify bundle integrity and exact signed bytes.

### 2. Purpose
Make signature verification real and byte-bound.

### 3. Exact scope of code changes
Implement bounded parsing and cryptographic verification of supported bundle material over the original payload, retaining exact raw/semantic relationships.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Valid test-only signed material verifies; tampering, malformed bundle, wrong payload/digest and unsupported algorithms reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Cryptographic verification runs rather than trusting a fixture boolean.

### 8. Commit message
```text
feat: verify exact evidence bundle integrity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s123"></a>
## S123 — Check signer identity issuer and authorization

**Predecessor:** S122. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Check signer identity issuer and authorization.

### 2. Purpose
Separate valid signatures from authorized evidence.

### 3. Exact scope of code changes
Enforce configured exact identities/issuers/allowed signers and scope authorization using verifier output; reject unrecognized or overbroad authority.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Correct signature from wrong issuer/signer or unauthorized scope rejects; correct configured fixture succeeds.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Signature validity alone cannot establish trusted EQM evidence.

### 8. Commit message
```text
feat: enforce authorized evidence signer identity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s124"></a>
## S124 — Check builder source and artifact expectations

**Predecessor:** S123. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Check builder source and artifact expectations.

### 2. Purpose
Bind verified evidence to what was actually intended.

### 3. Exact scope of code changes
Validate producer/builder/repository/source/build/artifact/configuration/selection against protected expected subjects; reject unrecognized security-relevant inputs.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
One-axis subject/build/source substitution and unexpected builder parameters fail without dropping other reasons.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Verified evidence cannot be replayed for a different product subject.

### 8. Commit message
```text
feat: verify exact evidence producer expectations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s125"></a>
## S125 — Check temporal and trust-material eligibility

**Predecessor:** S124. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Check temporal and trust-material eligibility.

### 2. Purpose
Make missing/current verification material explicit.

### 3. Exact scope of code changes
Apply configured temporal/trust-root/revocation requirements and bounded material retrieval only through authorized acquisition; distinguish historical verification from current acceptance.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Expired/missing/wrong-time material and denied network retrieval yield explicit non-acceptance; frozen historical fixtures reproduce.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Trust material has explicit validity and no hidden network fallback.

### 8. Commit message
```text
feat: validate verification material eligibility
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s126"></a>
## S126 — Verify every mandatory input type

**Predecessor:** S125. **Contracts:** R23, R24, R25, R26, R28, R29, R38, R51, R59, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC028, AC029, AC038, AC051, AC059, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Verify every mandatory input type.

### 2. Purpose
Close gaps where absent record/runtime verification disappears.

### 3. Exact scope of code changes
Apply the checked verification path to evidence, candidate/release records and runtime observations; build a complete required verification set without filter_map loss.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Missing record or runtime receipt blocks despite signed evidence; unrelated optional rejected inputs remain distinct from required gaps.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Every required input has an explicit acceptance conclusion.

### 8. Commit message
```text
fix: require complete input verification
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s127"></a>
## S127 — Persist checked verification provenance

**Predecessor:** S126. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Persist checked verification provenance.

### 2. Purpose
Connect actual verifier receipts to durable objects.

### 3. Exact scope of code changes
Atomically associate real checked results/material with exact raw/semantic objects and authority/evaluator context; import still revalidates the needed boundary.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Reopen/reimport preserves verification provenance without trusting caller-set flags; altered receipt/material rejects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Durable verification supports reproducible audit without minting authority.

### 8. Commit message
```text
feat: persist checked verification provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s128"></a>
## S128 — Enforce protected waiver approvals

**Predecessor:** S127. **Contracts:** R19, R23, R24, R25, R26, R29, R38, R51, R58, R64. **Acceptance:** AC019, AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC058, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Enforce protected waiver approvals.

### 2. Purpose
Keep waiver authority outside candidate/agent control.

### 3. Exact scope of code changes
Load and validate waiver approval/scope/time provenance through protected configuration; supply only eligible applications to the pure evaluator.

### 4. Files/modules likely involved
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/waivers.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Candidate-created/widened/renewed waivers cannot self-approve; expired/unrelated approvals reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Conditional outcomes require explicit protected exception authority.

### 8. Commit message
```text
feat: verify protected waiver authority
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s129"></a>
## S129 — Disable synthetic trust in production surfaces

**Predecessor:** S128. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Disable synthetic trust in production surfaces.

### 2. Purpose
Ensure test helpers cannot bypass the real pipeline.

### 3. Exact scope of code changes
Remove or isolate production-reachable synthetic SignedCi factories/flags; keep explicit unconfigured development state without representing it as verified release trust.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_runner/src/ci.rs`
- `crates/eqm_application/src/trust/`
- `Cargo.toml`

### 5. Unit tests or integration tests required
Production API/feature tests cannot construct unconditional signed trust; test fixtures remain usable only in test boundaries.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_runner -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Trusted release facts originate only from the configured checked path.

### 8. Commit message
```text
fix: exclude synthetic trust from production paths
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s130"></a>
## S130 — Add the cryptographic negative acceptance suite

**Predecessor:** S129. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Add the cryptographic negative acceptance suite.

### 2. Purpose
Pressure-test trust across all attack dimensions.

### 3. Exact scope of code changes
Combine exact positive signatures with wrong identity/issuer/builder/subject/attempt/replay/material/config/evaluator fixtures and persistent import cases.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
Actual verifier tests pass; unsupported live identity/network lanes remain explicitly unconfigured, not marked verified.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Trust assurance is backed by cryptographic and authorization regressions.

### 8. Commit message
```text
test: cover conformance verification attack cases
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s131"></a>
## S131 — Document trust activation and rotation

**Predecessor:** S130. **Contracts:** R23, R24, R25, R26, R29, R38, R51, R64. **Acceptance:** AC023, AC024, AC025, AC026, AC029, AC038, AC051, AC064.

**Read:** [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Document trust activation and rotation.

### 2. Purpose
Prepare operators without granting live signer permissions.

### 3. Exact scope of code changes
Add validated config examples using non-production placeholders, key/material rotation and historical/current acceptance procedures; record Q04/Q15 live blockers.

### 4. Files/modules likely involved
- `docs/evidence-and-trust.md`
- `docs/eqm_product_v1/runbooks/TRUST_OPERATIONS.md`
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`

### 5. Unit tests or integration tests required
Fixture config validation and offline rotation/revocation rehearsal pass; no real credentials/signers are embedded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask test security
```

**Admission gate:** Q04 supplies live authority only through authorized operators. Tests use cryptographically real but non-production fixture material; live release readiness cannot be claimed from them.

### 7. Expected result
Production activation is explicit and operationally reviewable.

### 8. Commit message
```text
docs: define verified trust activation procedures
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s132"></a>
## S132 — Assemble frozen input manifests

**Predecessor:** S131. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Assemble frozen input manifests.

### 2. Purpose
Connect coherent storage authority and subjects to evaluation.

### 3. Exact scope of code changes
Capture exact authority, universe, campaigns, subject manifest, evidence/runtime objects, verification material, evaluator and explicit time from one consistent generation.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Concurrent imports/config changes cannot mix inputs; missing referenced objects prevent successful preparation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
A decision can be reproduced from explicit immutable inputs.

### 8. Commit message
```text
feat: assemble frozen conformance input manifests
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s133"></a>
## S133 — Validate prepared evaluation construction

**Predecessor:** S132. **Contracts:** R04, R05, R11, R22, R24, R26, R38, R44, R45, R46, R50, R63. **Acceptance:** AC004, AC005, AC011, AC022, AC024, AC026, AC038, AC044, AC045, AC046, AC050, AC063.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Validate prepared evaluation construction.

### 2. Purpose
Prevent partial or unverified maps entering the main evaluator.

### 3. Exact scope of code changes
Combine validated completeness with checked input conversions and mandatory verification facts in a private application preparation boundary; no serde/public boolean bypass.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Missing provider/profile/receipt and forged prepared-input fields cannot produce a trusted complete decision.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Prepared inputs prove accounting completeness without implying success.

### 8. Commit message
```text
feat: validate prepared conformance inputs
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s134"></a>
## S134 — Run the canonical evaluator through application

**Predecessor:** S133. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Run the canonical evaluator through application.

### 2. Purpose
Expose one whole-evaluation operation to adapters.

### 3. Exact scope of code changes
Compose the reference semantics into the application evaluate operation with no new policy logic; return structured results and safe errors for explicit requests.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Valid, missing, stale, failed, unstable, waived and wrong-subject fixtures match reference outcomes; no acquisition spies fire.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
All adapters can invoke one normative evaluation path.

### 8. Commit message
```text
feat: expose shared conformance evaluation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s135"></a>
## S135 — Produce immutable decision snapshots

**Predecessor:** S134. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Produce immutable decision snapshots.

### 2. Purpose
Define stable exact decision identity and complete explanations.

### 3. Exact scope of code changes
Map evaluation outputs to immutable snapshot records with full scope/input/evaluator/time identities, accepted/rejected reasons, applied waivers and aggregate conclusions.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Identical inputs/time reproduce the same semantic decision; changed inputs/time/context are explicit rather than overwritten.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Decisions are replayable immutable records instead of mutable dashboard state.

### 8. Commit message
```text
feat: produce immutable evaluation snapshots
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s136"></a>
## S136 — Separate read-only and writable session access

**Predecessor:** S135. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Separate read-only and writable session access.

### 2. Purpose
Keep inspection from creating generated state or running tests.

### 3. Exact scope of code changes
Provide read-only store/session access that reports absent evidence without initializing DB/directories; explicit acquisition/publication operations retain write permission.

### 4. Files/modules likely involved
- `crates/eqm_application/src/session.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
check/query preparation on a missing .eqm tree does not create it; read-only callers cannot publish or acquire.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Query/evaluation use cases preserve declared side-effect boundaries.

### 8. Commit message
```text
fix: preserve read-only evaluation sessions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s137"></a>
## S137 — Persist decision manifests explicitly

**Predecessor:** S136. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Persist decision manifests explicitly.

### 2. Purpose
Make archived decisions durable without implicit query writes.

### 3. Exact scope of code changes
Add an explicitly authorized publication operation for input/decision manifests using object-first transactions and idempotent IDs; leave ordinary read-only queries non-mutating.

### 4. Files/modules likely involved
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Crash/reopen and duplicate publication retain exact decision references; failed persistence cannot appear as archived success.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Historical decisions are durable only through a visible publication boundary.

### 8. Commit message
```text
feat: archive exact evaluation decisions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s138"></a>
## S138 — Replay historical snapshots

**Predecessor:** S137. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Replay historical snapshots.

### 2. Purpose
Reproduce recorded decisions without present-day reinterpretation.

### 3. Exact scope of code changes
Load exact archived inputs/evaluator/context and reproduce historical results; missing original material/unsupported semantics returns explicit limitations.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
A decision replays after present time/trust changes using original context; unsupported evaluator does not silently migrate acceptance.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Historical audit is reproducible and separate from current permission.

### 8. Commit message
```text
feat: replay historical conformance decisions
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s139"></a>
## S139 — Evaluate current acceptance separately

**Predecessor:** S138. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Evaluate current acceptance separately.

### 2. Purpose
Prevent permanent permission from old signed or cached results.

### 3. Exact scope of code changes
Add a pure/current-context check over recorded decisions with present authorized freshness/trust/deployment criteria, without mutating the historic snapshot.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/freshness.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Expiry/revocation/policy changes reject current acceptance while original history remains unchanged.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Current release use must reestablish relevant eligibility.

### 8. Commit message
```text
feat: separate current decision acceptance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s140"></a>
## S140 — Provide shared explanation and query models

**Predecessor:** S139. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Provide shared explanation and query models.

### 2. Purpose
Replace command-specific interpretation with snapshot views.

### 3. Exact scope of code changes
Build query mapping for scope, obligations, reasons, evidence rejection, inventory/exposure, waivers and summaries; retain exact source/provenance information.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Query variants reference the same snapshot and preserve all blockers without implicit recalculation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI/MCP/native consumers get one coherent explanation surface.

### 8. Commit message
```text
feat: expose coherent evaluation explanations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s141"></a>
## S141 — Bind pagination to snapshots and filters

**Predecessor:** S140. **Contracts:** R04, R05, R11, R22, R38, R42, R44, R45, R46, R47, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC042, AC044, AC045, AC046, AC047, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Bind pagination to snapshots and filters.

### 2. Purpose
Keep large views coherent under updates.

### 3. Exact scope of code changes
Add bounded query pagination/cursor validation with snapshot/filter/order identity and explicit truncation metadata; avoid unauthorized cross-context cursor reuse.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Page continuation across changed snapshot/filter rejects; partial display does not alter completeness/authorization.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Large reports remain bounded and truthful.

### 8. Commit message
```text
feat: bind query pagination to evaluation snapshots
```

**Approved review regression contributions:** RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s142"></a>
## S142 — Define safe snapshot change notifications

**Predecessor:** S141. **Contracts:** R04, R05, R11, R22, R38, R44, R45, R46, R50. **Acceptance:** AC004, AC005, AC011, AC022, AC038, AC044, AC045, AC046, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Define safe snapshot change notifications.

### 2. Purpose
Support incremental consumers without hidden daemons.

### 3. Exact scope of code changes
Expose a bounded in-process/application notification or polling contract with snapshot generation identity and backpressure; no default network server.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_application/tests/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Slow/canceled consumers do not leak tasks or conflate generations; missed deltas trigger explicit resync.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Optional live consumers can refresh coherently without per-row reevaluation.

### 8. Commit message
```text
feat: expose bounded snapshot change signals
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s143"></a>
## S143 — Test end-to-end snapshot coherence

**Predecessor:** S142. **Contracts:** R04, R05, R11, R21, R22, R38, R44, R45, R46, R50, R52. **Acceptance:** AC004, AC005, AC011, AC021, AC022, AC038, AC044, AC045, AC046, AC050, AC052.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Test end-to-end snapshot coherence.

### 2. Purpose
Close application-service wiring before switching public commands.

### 3. Exact scope of code changes
Execute synthetic acquire/import-to-store-to-evaluate-to-query/replay flows using real checked fixture boundaries and expected non-success mutations.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `tests/fixtures/`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
AC004/AC005/AC011/AC022/AC038/AC044/AC046 cases pass with no side-effect or trust shortcuts.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
The shared application pipeline is independently verified before UI/CLI adoption.

### 8. Commit message
```text
test: verify the shared evaluation lifecycle
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s144"></a>
## S144 — Route obligations through shared scope preparation

**Predecessor:** S143. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Route obligations through shared scope preparation.

### 2. Purpose
Remove CLI-specific policy derivation copies.

### 3. Exact scope of code changes
Make obligations use exact application selection/universe/query models, including unknown/not-applicable/provider/scenario coordinates and complete IDs.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/obligations.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
CLI JSON fixtures include complete scope and duplicate-profile/ambiguous-selection errors.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Obligation inspection uses the same authority and identities as evaluation.

### 8. Commit message
```text
refactor: route obligation queries through application
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s145"></a>
## S145 — Route check through stored-evidence evaluation

**Predecessor:** S144. **Contracts:** R04, R11, R12, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC011, AC012, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Route check through stored-evidence evaluation.

### 2. Purpose
Replace unconditional missing findings.

### 3. Exact scope of code changes
Load bounded prepared evidence through read-only application evaluation and render actual facet/structure findings; no runner execution or implicit state creation.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/check.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_cli tests`

### 5. Unit tests or integration tests required
Persist passing evidence then stale/failed/untrusted variants; check reflects each and remains non-executing.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
The intended verify/import-to-check lifecycle becomes functional.

### 8. Commit message
```text
fix: evaluate stored evidence in conformance checks
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s146"></a>
## S146 — Render conformance and evidence matrices from snapshots

**Predecessor:** S145. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Render conformance and evidence matrices from snapshots.

### 2. Purpose
Remove hardcoded missing matrix values.

### 3. Exact scope of code changes
Map complete snapshot decisions to conformance/evidence cells with stable axes, obligation references, reasons and presentation-only filters.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_engine/src/matrix.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
Passing/stale/failed/unknown/mixed cells match check and complete universe; filter does not recompute assurance.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Matrix content reflects evaluated evidence rather than placeholders.

### 8. Commit message
```text
fix: render evaluated conformance matrices
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s147"></a>
## S147 — Render exposure release and comparison matrices

**Predecessor:** S146. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Render exposure release and comparison matrices.

### 2. Purpose
Complete remaining matrix kinds using real evaluated inputs.

### 3. Exact scope of code changes
Populate exposure/release/equivalence views from appropriate snapshot facts and conclusions, with unavailable states where data genuinely lacks verification.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_application/src/query.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Runtime mismatch and missing end-to-end/cohort cases agree with engine; no presence-only exposure match.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Every supported matrix view has a real semantic source.

### 8. Commit message
```text
feat: render complete assurance matrix views
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s148"></a>
## S148 — Route context and explanation through snapshots

**Predecessor:** S147. **Contracts:** R04, R30, R40, R42, R44, R45, R46, R47, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC042, AC044, AC045, AC046, AC047, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Route context and explanation through snapshots.

### 2. Purpose
Replace empty or fragmented evidence/findings context.

### 3. Exact scope of code changes
Populate context/explain with approved authority, exact obligations, evidence acceptance/rejection and contributing waivers; keep product text labeled untrusted and bounded.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/context.rs`
- `crates/eqm_cli/src/commands/explain.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
Context matches check/matrix and respects byte budgets without claiming truncated scope is complete.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Human/agent investigation gets actionable consistent reasons.

### 8. Commit message
```text
feat: expose evaluated context and explanations
```

**Approved review regression contributions:** RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s149"></a>
## S149 — Render concrete verify dry-run plans

**Predecessor:** S148. **Contracts:** R04, R11, R12, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC011, AC012, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Render concrete verify dry-run plans.

### 2. Purpose
Fulfill the execution-inspection contract publicly.

### 3. Exact scope of code changes
Serialize the shared fully resolved plan in human/JSON output with all safe executable/argv/environment-reference/limit/output details and no secret values.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_protocol/src/`
- `crates/eqm_cli/src/renderer.rs`

### 5. Unit tests or integration tests required
Dry-run includes exact planned work, invokes no programs, rejects unresolved selectors/programs and never outputs secret bytes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Users can inspect actual execution before authorizing it.

### 8. Commit message
```text
fix: render complete verification dry-run plans
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s150"></a>
## S150 — Execute verify through authorized acquisition

**Predecessor:** S149. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Execute verify through authorized acquisition.

### 2. Purpose
Remove ad hoc runner authority and unbounded result reads.

### 3. Exact scope of code changes
Route verify execution to application approval/campaign/supervision/persistence path, retaining explicit cancellation and typed execution outcomes.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/`

### 5. Unit tests or integration tests required
Local fixture execution persists exact receipts/evidence; unsupported guarantees/secret absence fail visibly; canceled work is retained correctly.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI execution uses the same bounded independently authorized acquisition boundary.

### 8. Commit message
```text
refactor: route verification through authorized acquisition
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s151"></a>
## S151 — Separate acquisition summaries from conformance

**Predecessor:** S150. **Contracts:** R04, R11, R12, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC011, AC012, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Separate acquisition summaries from conformance.

### 2. Purpose
Fix empty-plan and low-minimum success ambiguity.

### 3. Exact scope of code changes
Report selected/acquired outcome distinctly from evaluated policy result; reject accidental empty selection under documented semantics and never claim policy success from exit zero.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_protocol/src/`
- `docs/cli.md`

### 5. Unit tests or integration tests required
Empty plan, inadequate policy minimum and successful runner with missing expected case remain nonconformant/explicit failure.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
verify output cannot mislead callers about complete product assurance.

### 8. Commit message
```text
fix: distinguish acquisition from policy conformance
```

**Approved review regression contributions:** RA01.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s152"></a>
## S152 — Emit structured CLI failure envelopes

**Predecessor:** S151. **Contracts:** R04, R05, R21, R30, R40, R44, R45, R46, R52, R58, R59, R60. **Acceptance:** AC004, AC005, AC021, AC030, AC040, AC044, AC045, AC046, AC052, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Emit structured CLI failure envelopes.

### 2. Purpose
Preserve requested machine output on command failures.

### 3. Exact scope of code changes
Map application/parser/runner/storage/trust errors to stable safe machine envelopes and documented exit classes. Validate command/output-format compatibility before acquisition/storage; implement renderers for accepted formats or reject them during preflight. Preserve discoverable durable receipts and truthful errors if output fails after commit.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/main.rs`
- `crates/eqm_cli/src/renderer.rs`
- `crates/eqm_protocol/src/`
- `docs/cli.md`

### 5. Unit tests or integration tests required
Validate actual success/error envelopes against generated schemas for every command family; test all accepted format pairs, early usage/format rejection with effect spies, and output failure after durable receipt publication.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Machine consumers no longer receive only generic stderr stage failures.

### 8. Commit message
```text
fix: emit structured command failure envelopes
```

**Approved review regression contributions:** RA14, RA20.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s153"></a>
## S153 — Align discovery and reconciliation output

**Predecessor:** S152. **Contracts:** R04, R24, R28, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC024, AC028, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Align discovery and reconciliation output.

### 2. Purpose
Use independent inventory observations and shared evaluation facts.

### 3. Exact scope of code changes
Route static/executable discovery permission distinctions and reconciliation status into coherent DTOs; do not rewrite authored expectations from observations.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/discover.rs`
- `crates/eqm_cli/src/commands/reconcile.rs`
- `crates/eqm_application/src/`

### 5. Unit tests or integration tests required
Partial/untrusted inventory remains unknown, approved executable discovery uses acquisition, and reconciliation is non-mutating.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Observed state and intended authority stay separate and visible.

### 8. Commit message
```text
refactor: align discovery and reconciliation contracts
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s154"></a>
## S154 — Align inspect and diagnostic commands

**Predecessor:** S153. **Contracts:** R04, R05, R30, R40, R44, R45, R46, R48, R53, R58, R59, R60. **Acceptance:** AC004, AC005, AC030, AC040, AC044, AC045, AC046, AC048, AC053, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Align inspect and diagnostic commands.

### 2. Purpose
Preserve accurate provenance and root/schema/capability errors.

### 3. Exact scope of code changes
Update show/locate/doctor/validate to use shared prepared types/errors. Separate consumer prerequisites from EQM contributor toolchain checks and recognize valid generated store/work states. Keep read commands free of accidental acquisition and honor format preflight.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/show.rs`
- `crates/eqm_cli/src/commands/locate.rs`
- `crates/eqm_cli/src/commands/doctor.rs`
- `crates/eqm_cli/src/commands/validate.rs`

### 5. Unit tests or integration tests required
Run doctor on an unmodified non-Rust consumer without copied EQM Cargo/toolchain files, after supported generated-state creation, and with invalid store entries; test accepted output formats and read-only effect boundaries.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Supporting commands remain useful after shared-layer extraction.

### 8. Commit message
```text
refactor: align inspection commands with prepared context
```

**Approved review regression contributions:** RA19, RA20.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s155"></a>
## S155 — Add CLI lifecycle golden tests

**Predecessor:** S154. **Contracts:** R04, R05, R11, R12, R21, R30, R40, R44, R45, R46, R48, R52, R53, R58, R59, R60. **Acceptance:** AC004, AC005, AC011, AC012, AC021, AC030, AC040, AC044, AC045, AC046, AC048, AC052, AC053, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Add CLI lifecycle golden tests.

### 2. Purpose
Replace tests that merely assert missing or nonempty placeholder output.

### 3. Exact scope of code changes
Test actual passing and negative evidence lifecycle across obligations/check/matrix/context/verify and machine errors with deterministic time/snapshot inputs.

### 4. Files/modules likely involved
- `crates/eqm_cli tests`
- `tests/fixtures/`
- `crates/eqm_test_support/src/`

### 5. Unit tests or integration tests required
Views agree for the same snapshot; stale/trust/failed/missing changes are reflected and no .eqm writes occur on read-only paths.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI coverage exercises the promised product workflow rather than placeholder shape.

### 8. Commit message
```text
test: verify the full cli evidence lifecycle
```

**Approved review regression contributions:** RA01, RA14, RA19, RA20.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s156"></a>
## S156 — Remove superseded CLI evaluation helpers

**Predecessor:** S155. **Contracts:** R04, R30, R40, R44, R45, R46, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Remove superseded CLI evaluation helpers.

### 2. Purpose
Eliminate duplicate truth calculators after adoption tests.

### 3. Exact scope of code changes
Delete unused status/count/freshness/profile/digest implementations now replaced by application/engine; retain only command orchestration and rendering.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/evaluation.rs`
- `crates/eqm_cli/src/commands/check.rs`
- `crates/eqm_cli/src/commands/matrix.rs`
- `crates/eqm_cli/src/commands/verify.rs`

### 5. Unit tests or integration tests required
Golden lifecycle tests remain unchanged; dependency/lint scan finds no new divergent evaluator.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
CLI is thin and semantic maintenance has one authoritative implementation.

### 8. Commit message
```text
refactor: remove duplicate cli evaluation logic
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s157"></a>
## S157 — Update CLI usage and safe adoption examples

**Predecessor:** S156. **Contracts:** R04, R30, R40, R44, R45, R46, R48, R53, R58, R59, R60. **Acceptance:** AC004, AC030, AC040, AC044, AC045, AC046, AC048, AC053, AC058, AC059, AC060.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Update CLI usage and safe adoption examples.

### 2. Purpose
Make docs reflect implemented behavior and exact command grammar.

### 3. Exact scope of code changes
Update docs/cli.md, getting started, evidence/trust and existing examples for actual invocation/exit/dry-run/selection behavior; do not invent product tasks.

### 4. Files/modules likely involved
- `docs/cli.md`
- `docs/getting-started.md`
- `docs/evidence-and-trust.md`
- `examples/`

### 5. Unit tests or integration tests required
Run documented example validation and safe smoke commands discovered from the repo; examples pass without live credentials or implicit network.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

### 7. Expected result
Public instructions no longer promise placeholder or unimplemented behavior.

### 8. Commit message
```text
docs: document the completed evaluation workflow
```

**Approved review regression contributions:** RA19.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s158"></a>
## S158 — Share application snapshots with MCP sessions

**Predecessor:** S157. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Share application snapshots with MCP sessions.

### 2. Purpose
Remove CLI coupling without reloading/reinterpreting evidence.

### 3. Exact scope of code changes
Adapt MCP session ownership to shared application snapshot/query services while preserving transport lifecycle and no dependency cycle.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/session.rs`
- `crates/eqm_cli/src/commands/mcp.rs`
- `crates/eqm_application/src/query.rs`

### 5. Unit tests or integration tests required
Session tests borrow/reference one immutable snapshot and do not implicitly reacquire data or create state.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP can expose the same prepared decision as the CLI.

### 8. Commit message
```text
refactor: connect mcp to shared evaluation snapshots
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s159"></a>
## S159 — Populate MCP context and findings resources

**Predecessor:** S158. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Populate MCP context and findings resources.

### 2. Purpose
Replace empty placeholder resource records.

### 3. Exact scope of code changes
Render obligations/evidence/rejections/findings/applied waivers from shared query models with actual completeness/trust/truncation metadata.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/resources.rs`
- `crates/eqm_application/src/query.rs`
- `crates/eqm_mcp tests`

### 5. Unit tests or integration tests required
Resource results agree with check/context for passing and negative fixtures; empty findings do not imply evaluated completeness unless established.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Agent resources contain real evaluated context.

### 8. Commit message
```text
fix: populate evaluated mcp resources
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s160"></a>
## S160 — Route MCP tools through shared read operations

**Predecessor:** S159. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Route MCP tools through shared read operations.

### 2. Purpose
Avoid transport-specific status calculation.

### 3. Exact scope of code changes
Map supported inspection/query tools to shared application use cases with exact request validation; retain read-only defaults.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/tools.rs`
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp tests`

### 5. Unit tests or integration tests required
Tool/resource/CLI outputs reference the same decision and unauthorized execution-shaped requests reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP tools remain thin coherent readers.

### 8. Commit message
```text
refactor: route mcp tools through shared queries
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s161"></a>
## S161 — Enforce MCP permission and untrusted-data boundaries

**Predecessor:** S160. **Contracts:** R04, R05, R44, R45, R46, R47, R50. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md).

### 1. Step title
Enforce MCP permission and untrusted-data boundaries.

### 2. Purpose
Prevent text or read authority from becoming execution permission.

### 3. Exact scope of code changes
Add explicit operation admission checks and safe labeling for product descriptions/logs/attachments; ensure no signer/waiver/acquisition capability is exposed by default.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp/src/tools.rs`
- `crates/eqm_mcp tests`

### 5. Unit tests or integration tests required
Inject untrusted textual instructions and privilege-escalation requests; no execution/signing/mutation occurs.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
Read-only agent access cannot grant itself stronger capabilities.

### 8. Commit message
```text
fix: enforce mcp operation authority boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s162"></a>
## S162 — Bound MCP requests pagination and lifecycle

**Predecessor:** S161. **Contracts:** R04, R05, R42, R44, R45, R46, R47, R50. **Acceptance:** AC004, AC005, AC042, AC044, AC045, AC046, AC047, AC050.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Bound MCP requests pagination and lifecycle.

### 2. Purpose
Keep agent access resource-safe and generation-correct.

### 3. Exact scope of code changes
Apply input/output/queue budgets, snapshot-bound pagination, cancellation and explicit unsupported transport behavior; do not add a default HTTP daemon.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp/src/resources.rs`
- `crates/eqm_cli/src/commands/mcp.rs`

### 5. Unit tests or integration tests required
Oversized/deep requests, stale cursors, slow clients and shutdown cannot leak work or falsely complete a truncated view.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP lifecycle and views are bounded and truthful.

### 8. Commit message
```text
fix: bound mcp request and snapshot lifecycles
```

**Approved review regression contributions:** RA18.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s163"></a>
## S163 — Map structured errors into MCP responses

**Predecessor:** S162. **Contracts:** R04, R05, R21, R44, R45, R46, R47, R50, R52. **Acceptance:** AC004, AC005, AC021, AC044, AC045, AC046, AC047, AC050, AC052.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Map structured errors into MCP responses.

### 2. Purpose
Preserve causes without transport divergence or secret leakage.

### 3. Exact scope of code changes
Translate shared typed errors into supported protocol error/result shapes and stable diagnostics; retain request identity and context safely.

### 4. Files/modules likely involved
- `crates/eqm_mcp/src/server.rs`
- `crates/eqm_mcp/src/tools.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Invalid authority/query/resource and unavailable snapshot fixtures return valid protocol errors matching CLI diagnostic semantics.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

### 7. Expected result
MCP errors are actionable without implementing another evaluator.

### 8. Commit message
```text
fix: preserve structured mcp diagnostics
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s164"></a>
## S164 — Verify and document secure MCP integration

**Predecessor:** S163. **Contracts:** R04, R05, R44, R45, R46, R47, R50, R58. **Acceptance:** AC004, AC005, AC044, AC045, AC046, AC047, AC050, AC058.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Verify and document secure MCP integration.

### 2. Purpose
Establish parity and least-privilege evidence.

### 3. Exact scope of code changes
Add stdio/restricted-transport smoke/security tests and update documentation with actual supported methods, permissions, limits and snapshot semantics.

### 4. Files/modules likely involved
- `crates/eqm_mcp tests`
- `crates/eqm_cli/src/commands/mcp.rs`
- `docs/mcp.md`
- `docs/agent-context.md`

### 5. Unit tests or integration tests required
Cross-interface fixture suite and unauthorized-operation tests pass; no unverified public server support claim is added.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_mcp -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask test security
```

### 7. Expected result
MCP is usable as a coherent read-only agent interface.

### 8. Commit message
```text
test: verify secure mcp snapshot access
```

**Approved review regression contributions:** RA17.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s165"></a>
## S165 — Route release selection through protected preparation

**Predecessor:** S164. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Route release selection through protected preparation.

### 2. Purpose
Close the development-authority bypass.

### 3. Exact scope of code changes
Make release requests use exact protected authority/profile values and candidate subjects from application preparation, never the development derive helper/defaults.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_application/src/authority.rs`
- `crates/eqm_engine/src/selection.rs`

### 5. Unit tests or integration tests required
Candidate-local policy/defaults and absent protected baseline reject; exact authorized fixture selection succeeds.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release evaluation cannot select its own weaker development authority.

### 8. Commit message
```text
fix: use protected authority for release evaluation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s166"></a>
## S166 — Evaluate releases from complete snapshots

**Predecessor:** S165. **Contracts:** R13, R19, R23, R24, R27, R28, R29, R38, R44, R52, R54, R58, R63. **Acceptance:** AC013, AC019, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC058, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Evaluate releases from complete snapshots.

### 2. Purpose
Remove target-only and duplicate outcome logic.

### 3. Exact scope of code changes
Route release checking through the shared complete decision including provider/scenario/unknown coordinates, canonical counts/retries/trust and applied waivers.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/release.rs`

### 5. Unit tests or integration tests required
Target passes with missing provider/end-to-end or failed manual review cannot pass; canonical facet results match check/matrix.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release conformance uses the same normative semantics and full denominator.

### 8. Commit message
```text
fix: evaluate complete release conformance snapshots
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s167"></a>
## S167 — Compare real release exposure facts

**Predecessor:** S166. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Compare real release exposure facts.

### 2. Purpose
Replace runtime-document presence with semantic match.

### 3. Exact scope of code changes
Use exact verified runtime observations and authored expectations to derive exposure comparisons for the candidate/deployment context; include mismatch reasons.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/exposure.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Present but wrong/stale/untrusted facts remain mismatch/unknown; exact complete verified facts satisfy only declared expectations.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release exposure reflects observed behavior rather than document existence.

### 8. Commit message
```text
fix: compare actual release exposure facts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s168"></a>
## S168 — Separate readiness and post-deployment records

**Predecessor:** S167. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Separate readiness and post-deployment records.

### 2. Purpose
Make the release lifecycle realizable and non-circular.

### 3. Exact scope of code changes
Expose approved candidate-readiness versus deployed-conformance record/use-case distinctions, with exact prior-record references and explicit time/material prerequisites.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/`
- `schemas/`

### 5. Unit tests or integration tests required
Initial readiness works without impossible future observations; post-deploy evaluation requires exact later receipts and rejects cyclic references.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release context can be evaluated before and after deployment honestly.

### 8. Commit message
```text
feat: separate readiness from deployed conformance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s169"></a>
## S169 — Require actual subject verification in release gates

**Predecessor:** S168. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Require actual subject verification in release gates.

### 2. Purpose
Remove self-confirming expected/observed record shortcuts.

### 3. Exact scope of code changes
Supply expected subjects independently from protected manifests and observed verified subjects from inputs; require every mandatory receipt and current acceptance before eligibility.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_cli/src/commands/release_check.rs`

### 5. Unit tests or integration tests required
Wrong artifact/build/channel, missing record/runtime verification and stale acceptance reject despite otherwise signed evidence.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Gate inputs represent actual checked facts, not copied expected values.

### 8. Commit message
```text
fix: require verified exact release subjects
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s170"></a>
## S170 — Emit attestations from evaluated snapshots

**Predecessor:** S169. **Contracts:** R13, R19, R23, R24, R27, R28, R29, R38, R44, R52, R54, R58, R63. **Acceptance:** AC013, AC019, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC058, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Emit attestations from evaluated snapshots.

### 2. Purpose
Replace unknown placeholders and ad hoc conformance calculations.

### 3. Exact scope of code changes
Build statements from one exact decision and its input references, actual contributing waivers and per-participant artifact subjects; preserve multi-target identities rather than requiring all subjects identical.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/attest.rs`
- `crates/eqm_application/src/`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Passing/failed/unknown statements reproduce snapshot facts; different legitimate subjects are represented correctly and evidence subsets cannot alter the claim.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Attestation reports actual decision provenance without promoting evidence claims.

### 8. Commit message
```text
fix: attest exact evaluated conformance snapshots
```

**Approved review regression contributions:** RA09.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s171"></a>
## S171 — Add explicit signing through approved authority

**Predecessor:** S170. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Add explicit signing through approved authority.

### 2. Purpose
Support real signing without making it a release decision.

### 3. Exact scope of code changes
Invoke the selected reviewed signing provider only with explicit operator authority, bounded inputs and exact decision bytes; unsigned output remains available and accurately labeled.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_cli/src/commands/attest.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Test-only cryptographic signatures verify; missing authority rejects signing; signed failed/unknown statements remain non-authorizing.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Signing is a separate auditable operation rather than implicit trust promotion.

### 8. Commit message
```text
feat: sign decisions with explicit authority
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s172"></a>
## S172 — Model protected release authorization

**Predecessor:** S171. **Contracts:** R11, R13, R23, R24, R26, R27, R28, R29, R38, R44, R50, R52, R54, R63. **Acceptance:** AC011, AC013, AC023, AC024, AC026, AC027, AC028, AC029, AC038, AC044, AC050, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Model protected release authorization.

### 2. Purpose
Define the promotion boundary without building a deployment platform.

### 3. Exact scope of code changes
Add validated authorization request/record for exact decision/artifacts/participants/destination/channel/validity/allowed deviations under protected authority; finalize wire shape under version ADR.

### 4. Files/modules likely involved
- `crates/eqm_domain/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/src/`
- `schemas/`

### 5. Unit tests or integration tests required
Arbitrary successful DTOs cannot construct authority; missing exact artifact/destination/policy facts reject.

### 6. Verification commands
```sh
cargo xtask schemas generate
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_domain -p eqm_protocol -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
cargo xtask schemas check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Release permission is a distinct exact protected record.

### 8. Commit message
```text
feat: model exact release authorization
```

**Approved review regression contributions:** RA06.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s173"></a>
## S173 — Authorize only currently acceptable decisions

**Predecessor:** S172. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Authorize only currently acceptable decisions.

### 2. Purpose
Keep unknown and unapproved conditional states from promotion.

### 3. Exact scope of code changes
Implement the protected authorization evaluator using complete current acceptance, verified exact subjects and explicit deviation permissions; do not deploy or publish.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Unknown/failure/stale/wrong-authority decisions reject; conditional requires exact approved deviations; real fixture pass authorizes only its configured subject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Signing or historical success alone never grants promotion.

### 8. Commit message
```text
feat: enforce protected release authorization
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s174"></a>
## S174 — Validate authorization at deployment consumption

**Predecessor:** S173. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Validate authorization at deployment consumption.

### 2. Purpose
Bind what is promoted to what was approved.

### 3. Exact scope of code changes
Add a host-neutral validator/receipt contract consumed by an existing deployment integration for exact artifacts/destination/channel/time; no new infrastructure orchestration.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_protocol/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Artifact/destination substitution, expired interval and replay for another deployment reject; exact permitted consumption produces a receipt.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Deployment consumers can enforce exact authorization rather than trusting a generic pass.

### 8. Commit message
```text
feat: validate exact promotion authorization
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s175"></a>
## S175 — Handle transformed artifacts explicitly

**Predecessor:** S174. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Handle transformed artifacts explicitly.

### 2. Purpose
Avoid accepting new bytes merely because the source commit matches.

### 3. Exact scope of code changes
Validate an approved transformation provenance relationship or require reevaluation when deployment artifacts differ; actual transformation policies remain owner-configured.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_engine/src/release.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Same-commit different artifact without provenance rejects; configured verified transformation fixture follows its explicit policy.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Post-evaluation packaging transformations cannot silently bypass subject binding.

### 8. Commit message
```text
fix: verify transformed release artifact provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s176"></a>
## S176 — Default release enforcement to shadow mode

**Predecessor:** S175. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Default release enforcement to shadow mode.

### 2. Purpose
Prevent accidental production activation during implementation.

### 3. Exact scope of code changes
Add explicit operational configuration/admission separating evaluation/shadow reporting from enforcement; production identities/destinations and independent approval remain required.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_cli/src/commands/release_check.rs`
- `docs/eqm_product_v1/runbooks/RELEASE_OPERATIONS.md`

### 5. Unit tests or integration tests required
Unconfigured environment evaluates honestly but cannot enforce/promote; activation/reversion requires exact protected configuration.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
The completed gate is not automatically enabled against live releases.

### 8. Commit message
```text
feat: gate release enforcement activation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s177"></a>
## S177 — Test real positive and negative release lifecycle

**Predecessor:** S176. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Test real positive and negative release lifecycle.

### 2. Purpose
Prove trusted functionality rather than only fail-closed placeholders.

### 3. Exact scope of code changes
Execute fixture acquisition/import/verification/snapshot/signing/authorization/consumption plus subject/tamper/waiver/scope/expiry failures using real test-only cryptography.

### 4. Files/modules likely involved
- `crates/eqm_application/tests/`
- `crates/eqm_cli tests`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Exact configured positive case succeeds through all boundaries; each mandatory negative case remains non-authorizing.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
Production-capable release logic is exercised without live deployment or credentials.

### 8. Commit message
```text
test: verify exact release authorization lifecycle
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s178"></a>
## S178 — Remove duplicate release and attestation evaluators

**Predecessor:** S177. **Contracts:** R13, R23, R24, R27, R28, R29, R38, R44, R52, R54, R63. **Acceptance:** AC013, AC023, AC024, AC027, AC028, AC029, AC038, AC044, AC052, AC054, AC063.

**Read:** [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [specs/TRUST_AND_RELEASE.md](../specs/TRUST_AND_RELEASE.md).

### 1. Step title
Remove duplicate release and attestation evaluators.

### 2. Purpose
Leave one semantic implementation after parity tests.

### 3. Exact scope of code changes
Delete old count/freshness/status/trust/digest helpers and constant conclusions replaced by the shared pipeline; preserve only presentation and explicit capability wiring.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/release_check.rs`
- `crates/eqm_cli/src/commands/attest.rs`
- `crates/eqm_cli/src/commands/evaluation.rs`

### 5. Unit tests or integration tests required
Full CLI/MCP/snapshot/release parity and schema gates pass with no duplicated evaluator paths.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
cargo xtask test security
```

**Admission gate:** Q03 governs wire changes; Q04/Q06/Q09/Q15 gate live authority, compatibility and operational enforcement. Fixture success is not live activation.

### 7. Expected result
All release and statement conclusions are anchored to the canonical decision.

### 8. Commit message
```text
refactor: remove duplicate release decision logic
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s179"></a>
## S179 — Index source and authority reverse dependencies

**Predecessor:** S178. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Index source and authority reverse dependencies.

### 2. Purpose
Support exact conservative affected selection.

### 3. Exact scope of code changes
Build reverse indexes for shared source components, targets, bindings, contracts, profiles, runners, adapters, trust and evaluator context to affected obligations.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/affected.rs`
- `crates/eqm_application/src/`
- `crates/eqm_engine/tests/`

### 5. Unit tests or integration tests required
Shared commonMain changes reach all dependent outputs; authority changes select their dependent obligations deterministically.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Impact analysis uses the actual dependency model rather than one target root heuristic.

### 8. Commit message
```text
feat: index conformance reverse dependencies
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s180"></a>
## S180 — Handle deleted renamed and unclassified paths

**Predecessor:** S179. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Handle deleted renamed and unclassified paths.

### 2. Purpose
Prevent under-selection when source ownership is uncertain.

### 3. Exact scope of code changes
Extend change ingestion/baseline analysis for deletions/renames and conservative fallback on unknown paths or dependency gaps; keep reasons visible.

### 4. Files/modules likely involved
- `crates/eqm_engine/src/affected.rs`
- `crates/eqm_cli/src/commands/affected.rs`
- `crates/eqm_cli/src/commands/diff.rs`

### 5. Unit tests or integration tests required
Deleted bindings/tests and renamed/shared/unmapped paths never shrink required reevaluation unsafely.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Uncertainty widens impacted work rather than dropping it.

### 8. Commit message
```text
fix: preserve conservative affected scope
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s181"></a>
## S181 — Wire verify affected selection to actual plans

**Predecessor:** S180. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Wire verify affected selection to actual plans.

### 2. Purpose
Repair the parsed-but-discarded baseline path.

### 3. Exact scope of code changes
Use exact baseline/impact results to select acquisition work and record conservative fallback and scope; bind the resulting plan to approval.

### 4. Files/modules likely involved
- `crates/eqm_cli/src/commands/verify.rs`
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_engine/src/affected.rs`

### 5. Unit tests or integration tests required
Targeted baseline changes alter planned executions; unknown changes conservatively include required work; missing baseline rejects.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
verify --affected performs tested dependency-based selection.

### 8. Commit message
```text
fix: apply affected analysis to verification plans
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s182"></a>
## S182 — Cache parsing and semantic projections

**Predecessor:** S181. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Cache parsing and semantic projections.

### 2. Purpose
Reuse pure stable work without caching trust decisions implicitly.

### 3. Exact scope of code changes
Add bounded stage cache keyed by exact source/canonicalization/schema identities; preserve component projection semantics and explicit invalidation.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_manifest/src/canonical/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Cached and uncached parse/projection results match; input/schema changes invalidate and limits remain enforced.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_manifest
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Stable preparation work can be reused safely.

### 8. Commit message
```text
perf: cache canonical preparation stages
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s183"></a>
## S183 — Cache prepared graph and obligation derivation

**Predecessor:** S182. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Cache prepared graph and obligation derivation.

### 2. Purpose
Avoid full graph reconstruction for unchanged exact contexts.

### 3. Exact scope of code changes
Cache validated graph/universe results with complete authority/profile/source dependency identities and bounded memory; do not cache caller assertions of completeness.

### 4. Files/modules likely involved
- `crates/eqm_application/src/`
- `crates/eqm_engine/src/obligations.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Changed participation/profile/fragment/policy invalidates correct entries; reference outputs match cached preparation.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Warm derivation reuses exact validated work.

### 8. Commit message
```text
perf: reuse validated obligation preparation
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s184"></a>
## S184 — Cache verification with trust-context binding

**Predecessor:** S183. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Cache verification with trust-context binding.

### 2. Purpose
Avoid repeated cryptography without perpetual trust.

### 3. Exact scope of code changes
Key reusable checked material by exact bytes/verifier/configuration/subject/material identity and validity, retaining separate historical/current semantics.

### 4. Files/modules likely involved
- `crates/eqm_application/src/trust/`
- `crates/eqm_application/src/storage/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Trust-root/verifier/subject/material changes or expiry prevent reuse; immutable historical receipts remain available.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Verification caching cannot survive a relevant authority change silently.

### 8. Commit message
```text
perf: cache exact verification contexts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s185"></a>
## S185 — Expire decision cache entries correctly

**Predecessor:** S184. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Expire decision cache entries correctly.

### 2. Purpose
Prevent stale passes across time and authority transitions.

### 3. Exact scope of code changes
Add decision reuse eligibility with explicit validity boundaries and event invalidation; invalidate on relevant input/campaign/waiver/trust/evaluator changes.

### 4. Files/modules likely involved
- `crates/eqm_application/src/evaluation.rs`
- `crates/eqm_engine/src/freshness.rs`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Boundary-time and revocation/waiver/campaign changes reject cached acceptance; exact unchanged eligible input may reuse.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Current acceptance is never extended by cache lifetime.

### 8. Commit message
```text
fix: bound cached decision validity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s186"></a>
## S186 — Batch compatible native test work

**Predecessor:** S185. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Batch compatible native test work.

### 2. Purpose
Reduce process startup without changing observation counts.

### 3. Exact scope of code changes
Group approved cases by compatible runner/target/device/selection context while preserving exact manifest coverage and individual case/attempt lineage.

### 4. Files/modules likely involved
- `crates/eqm_application/src/acquisition.rs`
- `crates/eqm_runner/src/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Batched/unbatched outcomes match; shared executions do not multiply count credit; incompatible devices or contexts remain separated.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Native startup overhead drops without weakening evidence semantics.

### 8. Commit message
```text
perf: batch compatible evidence acquisition
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s187"></a>
## S187 — Avoid redundant snapshot cloning and serialization

**Predecessor:** S186. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Avoid redundant snapshot cloning and serialization.

### 2. Purpose
Improve query efficiency after correctness is stable.

### 3. Exact scope of code changes
Reuse immutable shared graph/decision data and serialize only requested bounded views; avoid new mutable global caches or cross-snapshot aliases.

### 4. Files/modules likely involved
- `crates/eqm_application/src/query.rs`
- `crates/eqm_engine/src/matrix.rs`
- `crates/eqm_protocol/src/`

### 5. Unit tests or integration tests required
Filtered/paginated output matches full reference slices; concurrent readers remain isolated and bounded.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application -p eqm_engine -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Large report consumers avoid repeated full graph copies and payloads.

### 8. Commit message
```text
perf: streamline immutable snapshot queries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s188"></a>
## S188 — Build representative end-to-end benchmarks

**Predecessor:** S187. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Build representative end-to-end benchmarks.

### 2. Purpose
Replace placeholder warm-context and incomplete cold timings.

### 3. Exact scope of code changes
Extend benchmark fixtures with real targets/profiles/evidence histories/trust/runtime facts/full matrices and measure complete preparation including canonicalization plus stage metrics.

### 4. Files/modules likely involved
- `tools/benchmarks/src/main.rs`
- `crates/eqm_test_support/src/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Benchmark verifies fixture semantics and required counts before timing; populated context and evidence evaluation are actually executed.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_test_support
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Run the owning benchmark/fixture tests and, when the populated benchmark step is reached, the actual cargo xtask benchmark lane on declared hardware; record measurements and host gaps, not thresholds alone.

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Performance reports measure the implemented assurance workload.

### 8. Commit message
```text
test: benchmark the complete conformance pipeline
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s189"></a>
## S189 — Add reference versus optimized differential tests

**Predecessor:** S188. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Add reference versus optimized differential tests.

### 2. Purpose
Protect semantics under caching batching and indexing.

### 3. Exact scope of code changes
Generate bounded randomized and fixed frozen inputs; compare canonical outcomes/reasons/identities for reference versus optimized paths including cache invalidation and reordered inputs.

### 4. Files/modules likely involved
- `crates/eqm_engine/tests/properties.rs`
- `crates/eqm_application/tests/`
- `tools/benchmarks/`

### 5. Unit tests or integration tests required
Any divergence fails; duplicates/missing/unknown/campaign/trust/expiry cases remain identical.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Optimization is continuously checked against the normative reference.

### 8. Commit message
```text
test: compare optimized and reference evaluation
```

**Approved review regression contributions:** RA10.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s190"></a>
## S190 — Establish measured performance budgets

**Predecessor:** S189. **Contracts:** R07, R19, R22, R38, R39, R40, R41, R42, R43, R46. **Acceptance:** AC007, AC019, AC022, AC038, AC039, AC040, AC041, AC042, AC043, AC046.

**Read:** [specs/API_CONTRACTS.md](../specs/API_CONTRACTS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Establish measured performance budgets.

### 2. Purpose
Avoid unsupported super-optimized claims or arbitrary limits.

### 3. Exact scope of code changes
Record actual supported-host workloads, latency distributions, memory/storage growth and contention; set reviewed budgets with Q07 rather than copying baseline constants.

### 4. Files/modules likely involved
- `tools/benchmarks/src/main.rs`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `docs/eqm_product_v1/specs/STORAGE_AND_PERFORMANCE.md`

### 5. Unit tests or integration tests required
Run benchmark lane on declared hardware and publish actual results plus unavailable-host gaps; correctness gates remain independent.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask benchmark
```

**Required command discovery:**
- Run the owning benchmark/fixture tests and, when the populated benchmark step is reached, the actual cargo xtask benchmark lane on declared hardware; record measurements and host gaps, not thresholds alone.

**Admission gate:** Run benchmark commands only once their fixtures remain meaningful and valid; before that step use owning test lanes. Q07 sets supported workloads and budgets; no numerical SLO is invented.

### 7. Expected result
Performance objectives are measured and reviewable, not inferred from code constants.

### 8. Commit message
```text
perf: establish measured conformance budgets
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s191"></a>
## S191 — Discover real product targets and native commands

**Predecessor:** S190. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Discover real product targets and native commands.

### 2. Purpose
Avoid invented Gradle/Xcode selectors and unsupported platform promises.

### 3. Exact scope of code changes
Resolve Q01/Q02/Q06/Q12 from authorized repos/owners; record product commits, native tasks, selectors, output artifacts, roles and supported cohorts without editing product code yet.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`
- `docs/integrations/`
- `examples/`

### 5. Unit tests or integration tests required
Read manifests/help and verify discovered commands in disposable authorized fixtures; distinguish synthetic versus live lanes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Real integration has exact known inputs or an explicit blocker instead of fabricated tasks.

### 8. Commit message
```text
docs: record native product integration contracts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s192"></a>
## S192 — Define native result adapter contracts

**Predecessor:** S191. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Define native result adapter contracts.

### 2. Purpose
Preserve expected selection and subject identity across test frameworks.

### 3. Exact scope of code changes
Specify and implement minimal adapters' common normalized result/receipt boundary for native selectors, cases, attempts and artifacts; do not build a new test framework.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/normalized.rs`
- `crates/eqm_protocol/src/`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Equivalent Rust/Kotlin native fixture results normalize identically; missing cases and wrong subject/attempt reject.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner -p eqm_protocol
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Native producers can supply exact conformance evidence independently of UI frameworks.

### 8. Commit message
```text
feat: define native evidence adapter contracts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s193"></a>
## S193 — Integrate the Rust product CLI test adapter

**Predecessor:** S192. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Integrate the Rust product CLI test adapter.

### 2. Purpose
Verify the product CLI without confusing it with EQM's binary.

### 3. Exact scope of code changes
Bind discovered Rust product tests/inspection/export scenarios to exact EQM selectors and normalized results; implement an authorized adapter or fixture harness without inventing product business logic.

### 4. Files/modules likely involved
- `examples/`
- `tests/fixtures/`
- `crates/eqm_runner tests`
- `authorized product Rust test adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Actual fixture/test commands produce bounded subject-bound expected-case results; malformed/missing selection fails.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Rust product CLI evidence participates in the common conformance model.

### 8. Commit message
```text
feat: integrate rust product conformance evidence
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s194"></a>
## S194 — Integrate desktop KMP test evidence

**Predecessor:** S193. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Integrate desktop KMP test evidence.

### 2. Purpose
Connect actual control-tower tests to EQM contracts.

### 3. Exact scope of code changes
Bind discovered desktop tasks/selectors/artifacts and normalize account/task projection/review fixtures; keep business UI and source ownership in the product repo.

### 4. Files/modules likely involved
- `docs/integrations/`
- `examples/`
- `tests/fixtures/`
- `authorized desktop KMP test adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Native desktop lane validates current/stale/conflicting projection fixtures and exact subject/selection mapping; no JVM/native assumption is fabricated.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Desktop conformance evidence comes from real native tests where access exists.

### 8. Commit message
```text
feat: integrate desktop kmp conformance evidence
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s195"></a>
## S195 — Integrate mobile capture and lifecycle evidence

**Predecessor:** S194. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Integrate mobile capture and lifecycle evidence.

### 2. Purpose
Verify edge behavior through actual platform tests.

### 3. Exact scope of code changes
Bind discovered mobile photo/video/voice/annotation/save-restart/permission fixtures and native task selectors; keep media handling out of EQM's engine.

### 4. Files/modules likely involved
- `docs/integrations/`
- `examples/`
- `tests/fixtures/`
- `authorized mobile test adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Supported device/simulator lanes verify acknowledged persistence/interruption and correct normalized expected cases; unsupported platforms stay explicit.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Mobile edge evidence is native and lifecycle-aware rather than a parser-only fixture claim.

### 8. Commit message
```text
feat: integrate mobile edge conformance evidence
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s196"></a>
## S196 — Integrate provider and runtime observations

**Predecessor:** S195. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R24, R28, R30, R48, R49, R55, R59. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC024, AC028, AC030, AC048, AC049, AC055, AC059.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Integrate provider and runtime observations.

### 2. Purpose
Bind shared services and actual exposure to exact subjects.

### 3. Exact scope of code changes
Add discovered provider/deployment observation producers and bounded normalization for required sync/exposure facts; distinguish partial from complete trusted inventory.

### 4. Files/modules likely involved
- `crates/eqm_runner/src/inventory.rs`
- `examples/`
- `tests/fixtures/`
- `authorized provider adapter path discovered in Q01`

### 5. Unit tests or integration tests required
Wrong provider/config/build and partial/stale observations cannot satisfy required dependencies.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_runner
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Shared providers have independent exact evidence rather than inferred client success.

### 8. Commit message
```text
feat: integrate exact provider runtime evidence
```

**Approved review regression contributions:** RA21.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s197"></a>
## S197 — Implement the cross-runtime lifecycle scenario

**Predecessor:** S196. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Implement the cross-runtime lifecycle scenario.

### 2. Purpose
Verify the approved capture-to-review-to-export chain.

### 3. Exact scope of code changes
Orchestrate existing native test operations under one approved scenario/campaign: offline capture, edit, terminate/restart, reconnect/retry, desktop review and CLI inspect/export; use synthetic account/task data.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `examples/`
- `crates/eqm_application tests`
- `authorized native integration test paths from Q01`

### 5. Unit tests or integration tests required
Intended revision appears exactly once under the correct task with no acknowledged-state loss or cross-account disclosure.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
End-to-end evidence is independently produced for the actual participant set.

### 8. Commit message
```text
test: verify the evidence lifecycle across runtimes
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s198"></a>
## S198 — Add product failure and account-boundary scenarios

**Predecessor:** S197. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Add product failure and account-boundary scenarios.

### 2. Purpose
Exercise failure contracts beyond the happy path.

### 3. Exact scope of code changes
Extend the real scenario with approved account switching, permission revocation, concurrent edits, interrupted uploads and stale projections; use owner-defined conflict semantics only.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `authorized product integration test paths from Q01`
- `docs/eqm_product_v1/specs/ACCEPTANCE_CRITERIA.md`

### 5. Unit tests or integration tests required
Each interruption/security case follows explicit product invariants and produces distinct expected outcomes; no invented conflict algorithm.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Product assurance covers the approved lifecycle failure boundaries.

### 8. Commit message
```text
test: cover cross-runtime failure and account boundaries
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s199"></a>
## S199 — Verify approved mixed-version cohorts

**Predecessor:** S198. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Verify approved mixed-version cohorts.

### 2. Purpose
Demonstrate independently released target compatibility.

### 3. Exact scope of code changes
Execute the scenario for the owner-approved old/new client/provider combinations and role occurrences; add explicit relational outcome comparison where required.

### 4. Files/modules likely involved
- `tests/fixtures/`
- `authorized product integration test paths from Q01`
- `examples/`

### 5. Unit tests or integration tests required
Approved combinations meet declared outcomes; undeclared or mismatched cohorts cannot borrow passing evidence.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Compatibility assurance is tied to tested exact cohorts, not a global same-version assumption.

### 8. Commit message
```text
test: verify approved runtime compatibility cohorts
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s200"></a>
## S200 — Publish a complete consumer integration example

**Predecessor:** S199. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R44, R48, R49, R53, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC044, AC048, AC049, AC053, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Publish a complete consumer integration example.

### 2. Purpose
Make the intended Rust/KMP adoption reconstructable.

### 3. Exact scope of code changes
Add a small copyable synthetic workspace with shared-source roles, bindings, policies, profiles, native adapter contracts and safe dry-run/check instructions; mark real task substitutions explicitly.

### 4. Files/modules likely involved
- `examples/`
- `docs/integrations/`
- `docs/getting-started.md`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
Example validates and safe fixture smoke tests pass; generated state remains ignored and no fake real-product evidence is committed.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Consumers can adopt one real slice without duplicating intent or confusing business/conformance evidence.

### 8. Commit message
```text
docs: publish the multi-runtime integration example
```

**Approved review regression contributions:** RA19.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s201"></a>
## S201 — Record optional embedding disposition

**Predecessor:** S200. **Contracts:** R01, R02, R03, R07, R09, R10, R16, R30, R48, R49, R55. **Acceptance:** AC001, AC002, AC003, AC007, AC009, AC010, AC016, AC030, AC048, AC049, AC055.

**Read:** [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [specs/DATA_MODEL.md](../specs/DATA_MODEL.md), [specs/EVALUATION_SEMANTICS.md](../specs/EVALUATION_SEMANTICS.md), [specs/EVIDENCE_AND_IDENTITY.md](../specs/EVIDENCE_AND_IDENTITY.md), [specs/EXECUTION_SECURITY.md](../specs/EXECUTION_SECURITY.md), [specs/PRODUCT_SPEC.md](../specs/PRODUCT_SPEC.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md).

### 1. Step title
Record optional embedding disposition.

### 2. Purpose
Preserve the approved default without silently starting bridge work.

### 3. Exact scope of code changes
Record that default EQM deployment is developer/CI; evaluate whether an actual local evaluation feature activates the separate optional sequence; do not add bridges/daemons otherwise.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/OPEN_QUESTIONS.md`
- `docs/eqm_product_v1/decisions/`
- `docs/eqm_product_v1/specs/RUNTIME_INTEGRATION.md`

### 5. Unit tests or integration tests required
Package/dependency review confirms no unactivated native/daemon surface; activation decision references exact use case and matrix if granted.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.

**Admission gate:** Q01/Q02/Q06/Q12 are required for real product tasks and acceptance. Missing product repos are explicit blockers for those steps; synthetic EQM fixtures alone do not close live product acceptance. Discover native commands before changing adapter code.

### 7. Expected result
Mandatory v1 scope is closed independently of optional embedding.

### 8. Commit message
```text
docs: record native embedding activation status
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s202"></a>
## S202 — Complete the supported feature and host matrix

**Predecessor:** S201. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Complete the supported feature and host matrix.

### 2. Purpose
Make packaging capability claims explicit.

### 3. Exact scope of code changes
Inspect actual Cargo features and supported targets, add tests for every supported combination and ensure optional capabilities never disable mandatory validation; record unsupported hosts deliberately.

### 4. Files/modules likely involved
- `Cargo.toml`
- `crates/*/Cargo.toml`
- `tools/xtask/src/main.rs`
- `docs/eqm_product_v1/implementation/VERIFICATION.md`

### 5. Unit tests or integration tests required
Discover and run valid no-default/default/feature combinations plus native host lanes; no blanket all-features command for incompatible combinations.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover and record the exact native host/product test and smoke commands for Q01/Q02 before this step. Run required native execution tests on the declared host; cross-compilation or synthetic EQM fixtures alone are not that evidence.
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Build and support claims match tested actual configurations.

### 8. Commit message
```text
test: verify supported feature and host combinations
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s203"></a>
## S203 — Expand fuzz and mutation coverage to new boundaries

**Predecessor:** S202. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Expand fuzz and mutation coverage to new boundaries.

### 2. Purpose
Test the newly centralized critical paths rather than only old helpers.

### 3. Exact scope of code changes
Extend fuzz corpus/targets and focused mutation coverage for strict parsing, scope completeness, evidence conflicts, trust acceptance, authorization and storage records while retaining bounds.

### 4. Files/modules likely involved
- `tools/fuzz/`
- `tools/xtask/src/main.rs`
- `crates/eqm_engine/tests/`
- `crates/eqm_application/tests/`

### 5. Unit tests or integration tests required
Bounded fuzz and mutation lanes pass with meaningful seeded valid/invalid inputs; surviving critical mutants are investigated, not ignored by threshold changes.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask -p eqm_engine -p eqm_application
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Extended verification targets the actual production decision path.

### 8. Commit message
```text
test: harden critical conformance verification lanes
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s204"></a>
## S204 — Verify schema API and migration compatibility

**Predecessor:** S203. **Contracts:** R21, R43, R44, R45, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC021, AC043, AC044, AC045, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Verify schema API and migration compatibility.

### 2. Purpose
Close explicit version-transition obligations.

### 3. Exact scope of code changes
Run all approved wire/ABI-if-activated/DB compatibility fixtures, original-signed-byte preservation, generator parity and unsupported-version errors; update transition docs with actual evidence.

### 4. Files/modules likely involved
- `crates/eqm_protocol/tests/`
- `crates/eqm_application/tests/`
- `tools/xtask/src/main.rs`
- `docs/eqm_product_v1/specs/COMPATIBILITY.md`

### 5. Unit tests or integration tests required
Old/new supported behavior matches ADR, unsupported versions fail visibly, interrupted migrations recover and signed originals remain intact.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p eqm_protocol -p eqm_application -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo xtask schemas check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Compatibility is tested rather than inferred from deserialization success.

### 8. Commit message
```text
test: verify explicit compatibility transitions
```

**Approved review regression contributions:** RA14.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s205"></a>
## S205 — Verify reproducible distribution and supply chain

**Predecessor:** S204. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Verify reproducible distribution and supply chain.

### 2. Purpose
Ensure shipped artifacts match declared dependencies and capabilities.

### 3. Exact scope of code changes
Extend existing dist/package checks for new crate/dependencies/runtime libraries, SPDX/SBOM/provenance inputs and checksums; keep outputs unsigned/local unless separately authorized.

### 4. Files/modules likely involved
- `tools/xtask/src/main.rs`
- `Cargo.lock`
- `deny.toml if present`
- `docs/eqm_product_v1/implementation/VERIFICATION.md`

### 5. Unit tests or integration tests required
Package/reproducibility and current dependency audit/policy lanes pass; inspect actual SQLite/verifier/native library versions.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Distribution artifacts have reproducible provenance without implicit publication.

### 8. Commit message
```text
build: verify conformance distribution provenance
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s206"></a>
## S206 — Reconcile public docs and remove transitional code

**Predecessor:** S205. **Contracts:** R43, R44, R48, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC044, AC048, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md), [repo/REVIEW_FINDINGS.md](../repo/REVIEW_FINDINGS.md), [implementation/review_regressions.json](../implementation/review_regressions.json), [decisions/ADR-0012-approved-review-closure.md](../decisions/ADR-0012-approved-review-closure.md).

### 1. Step title
Reconcile public docs and remove transitional code.

### 2. Purpose
Finish code health without broad cosmetic churn.

### 3. Exact scope of code changes
Remove obsolete helpers/features/placeholder fixtures only after tests prove replacement; update README/public docs/error references and generated schemas; retain approved historical records.

### 4. Files/modules likely involved
- `README.md`
- `docs/`
- `crates/`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
Standard gate, doc tests, examples and dependency/lint scans pass; no hardcoded production missing/unknown-as-final placeholders or synthetic trust bypass remains.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Inspect Cargo metadata/manifests for the actual package/features and intentional lockfile update command. Verify the linked dependency/runtime and audited exact pins; enumerate valid feature combinations before running feature checks.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
The repository has one documented implemented architecture without abandoned transition paths.

### 8. Commit message
```text
refactor: finalize the shared conformance implementation
```

**Approved review regression contributions:** RA19.
Read their Given/When/Expected contracts in the referenced regression catalog. This step implements/tests its contribution using already available dependencies; later integration contributions remain pending until their owning steps. A reference here is not closure evidence.

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s207"></a>
## S207 — Rehearse storage trust and release incidents

**Predecessor:** S206. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Rehearse storage trust and release incidents.

### 2. Purpose
Make runbooks executable and owners explicit.

### 3. Exact scope of code changes
Assign owner roles using Q15 and run disposable crash/restore, key/material rotation, stale decision, failed authorization and evaluator rollback drills; record results and limitations.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/runbooks/`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `tests/fixtures/`

### 5. Unit tests or integration tests required
Recovery preserves history and trust boundaries; rollback pauses or uses known-good evaluation, never permissive promotion.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Operational procedures are evidenced and responsibility is explicit.

### 8. Commit message
```text
docs: record conformance operational recovery drills
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s208"></a>
## S208 — Run shadow release comparison

**Predecessor:** S207. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Run shadow release comparison.

### 2. Purpose
Validate decisions independently before enforcement.

### 3. Exact scope of code changes
Integrate host-neutral shadow reporting into approved existing CI/deployment environment; compare representative positive/negative decisions with independently reviewed expected outcomes.

### 4. Files/modules likely involved
- `approved CI integration path discovered in Q09`
- `docs/eqm_product_v1/runbooks/RELEASE_OPERATIONS.md`
- `docs/eqm_product_v1/implementation/STATUS.md`

### 5. Unit tests or integration tests required
Wrong-subject/weakening/missing-scope/expiry fixtures remain blocked; independent review records discrepancies and no live promotion occurs.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover authorized CI/promotion validation commands and exact protected identities/destinations through Q04/Q09/Q15. Do not execute publication/deployment or invent operational approval as part of this code step.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Gate behavior has operational comparison evidence before activation.

### 8. Commit message
```text
test: verify shadow release decision parity
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s209"></a>
## S209 — Complete final acceptance and release-candidate gates

**Predecessor:** S208. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Complete final acceptance and release-candidate gates.

### 2. Purpose
Establish readiness from actual evidence rather than approved design.

### 3. Exact scope of code changes
Map every mandatory R/AC/F item to committed implementation/tests, execute standard/extended/native/product/feature lanes and document unresolved blockers; do not mark optional embedding required.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/TRACEABILITY.md`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `tools/xtask/src/main.rs`

### 5. Unit tests or integration tests required
cargo xtask check and verify plus all declared supported lanes pass; no new failures are waived and every unrun lane is explicit.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked -p xtask
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
cargo xtask verify
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Code/platform readiness is accurately established or explicitly blocked.

### 8. Commit message
```text
test: complete product v1 acceptance verification
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s210"></a>
## S210 — Record enforcement approval or explicit non-activation

**Predecessor:** S209. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Record enforcement approval or explicit non-activation.

### 2. Purpose
Keep operational permission separate from code completion.

### 3. Exact scope of code changes
Have the authorized owner review actual identities/destinations/cohorts/host guarantees and shadow evidence; record enforce-or-remain-shadow disposition without deploying from this coding task.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/runbooks/RELEASE_OPERATIONS.md`
- `docs/eqm_product_v1/implementation/STATUS.md`
- `approved operator config path from Q09`

### 5. Unit tests or integration tests required
Activation record binds exact evaluator/authority/configuration; absent approval leaves enforcement disabled; deployment consumer rechecks exact authorization.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
```

**Required command discovery:**
- Discover authorized CI/promotion validation commands and exact protected identities/destinations through Q04/Q09/Q15. Do not execute publication/deployment or invent operational approval as part of this code step.

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
No one mistakes a green build or signed report for permission to promote.

### 8. Commit message
```text
docs: record release gate activation disposition
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.

<a id="s211"></a>
## S211 — Finalize the implementation continuation record

**Predecessor:** S210. **Contracts:** R43, R49, R51, R52, R53, R54, R55, R56, R57, R61, R62. **Acceptance:** AC043, AC049, AC051, AC052, AC053, AC054, AC055, AC056, AC057, AC061, AC062.

**Read:** [PROMPT.md](../PROMPT.md), [implementation/STATUS.md](../implementation/STATUS.md), [implementation/STEP_REPORT_TEMPLATE.md](../implementation/STEP_REPORT_TEMPLATE.md), [implementation/VERIFICATION.md](../implementation/VERIFICATION.md), [repo/BASELINE.md](../repo/BASELINE.md), [runbooks/OPERATIONS.md](../runbooks/OPERATIONS.md), [runbooks/RELEASE_OPERATIONS.md](../runbooks/RELEASE_OPERATIONS.md), [specs/ARCHITECTURE.md](../specs/ARCHITECTURE.md), [specs/COMPATIBILITY.md](../specs/COMPATIBILITY.md), [specs/RUNTIME_INTEGRATION.md](../specs/RUNTIME_INTEGRATION.md), [specs/STORAGE_AND_PERFORMANCE.md](../specs/STORAGE_AND_PERFORMANCE.md).

### 1. Step title
Finalize the implementation continuation record.

### 2. Purpose
Leave a reconstructable audited handoff after coding.

### 3. Exact scope of code changes
Update progress ledger, completed step reports, deviations, open decisions and final report with actual commit SHAs/tests/risks; no claimed test runs are inferred or fabricated.

### 4. Files/modules likely involved
- `docs/eqm_product_v1/implementation/`
- `docs/eqm_product_v1/summary.txt`

### 5. Unit tests or integration tests required
Traceability/link/integrity checks pass for evolved docs; Git history matches reports and final status separates code/platform/operational completion.

### 6. Verification commands
```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --all-targets --locked --workspace
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask check
git diff --check
cargo test --workspace --doc --locked
cargo xtask verify
```

**Admission gate:** Q02/Q04/Q07/Q09/Q15 govern supported-platform and operational sign-off. Extended commands are required at their relevant validation steps, not claimed from package generation. Do not publish/deploy as a side effect.

### 7. Expected result
Another agent or maintainer can reconstruct exactly what shipped and what remains unverified.

### 8. Commit message
```text
docs: finalize the product v1 implementation record
```

**Before advancing:** relevant checks pass (or an unchanged baseline exception is precisely recorded), the diff is self-reviewed, progress and step report are updated, and this step is committed. No later-step code is included. Newly introduced failures block the next step.
