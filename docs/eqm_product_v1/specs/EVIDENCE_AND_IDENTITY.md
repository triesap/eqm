# Evidence, identity and reproducibility

## Binding semantics (R18–R22)

Use separate domain-separated semantic digest projections for contracts, bindings, evidence specifications, target configuration, runners, adapters, policy/profile selection, authority and trust configuration. Hashing only a binding/specification ID pair is insufficient. The workspace digest remains a useful summary, not a replacement for each component's meaning.

A runner identity used in evidence must include the approved executable content as well as its definition and relevant execution/toolchain context. A wrapper script hash alone does not establish the identity of mutable code or toolchains it invokes. Record the inputs necessary for the claim's threat model rather than asserting universal hermeticity. Immutable trusted source snapshots differ from local dirty-tree development provenance.

Three identities must coexist: raw object bytes, normalized semantic evidence, and authorized execution/attempt. Original signed bytes remain available for verification. Transport formatting changes can yield a different raw object without a different semantic assertion. Reimporting an identical assertion/attempt is idempotent and cannot increase counts. Two conflicting accepted assertions for one immutable attempt form a persistent conflict, not last-writer-wins.

Canonical serialization is a protocol specification, not the default serializer output. RFC 8785 is the approved direction: reject duplicate JSON names, preserve Unicode string data, distinguish semantic sets from ordered sequences, and specify numeric/time/null behavior. Exact field ranges/representations must be frozen with cross-runtime test vectors before schema publication. Do not silently normalize signed bytes. Source path portability normalization is a separate concern from JCS string preservation.

## Campaign and selection contract (R16–R17)

An authorized plan names exact selectors and an expected inventory of logical cases/observations. A campaign groups executions for that subject/selection. Each logical case has explicit attempts, terminal outcomes, and provenance. Missing selections remain missing even when reported cases all pass. File count and retry count are not observation count.

Selection of a current campaign is an explicit policy choice with approved ordering/supersession authority. It is not arbitrary timestamp max, directory sort order, or the one run the caller happens to supply. Local exploration can display selected subsets but cannot claim complete release assurance from them. Maintain audit/reliability history without allowing irrelevant past failures to permanently poison future authorized campaigns.

## Acyclic lifecycle (R20)

```text
protected authority + candidate subject manifest
                       |
             approved plan / campaign
                       |
  evidence + worker receipts + available runtime observations
                       |
              frozen input manifest
                       |
               evaluation decision
                       |
            signed statement (optional)
                       |
         protected release authorization
                       |
           deployment receipt / observations
```

References point to records already defined. Evidence binds a plan/candidate subject, not the future decision that includes the evidence. Post-deployment observations feed a later evaluation, not a circular initial-readiness prerequisite. Do not reuse the old release-record field mechanically where it introduces such a cycle.

## Input freezing

Use one consistent authority/evidence/index generation when creating an input manifest. Capture clock and trust context explicitly. A snapshot stores required scope, selected campaigns, subject mappings, exact referenced objects and verification material. Missing objects invalidate preparation; concurrent imports cannot silently change an in-progress decision.

For local development, expose dirty/untracked source identity and confidence. For trusted evaluation, bind what actually executed to the authorized immutable source/build inputs. Never label execution of modified bytes as clean HEAD.

## Import and collision handling

Validate byte bounds and syntax before normalization. Verify semantic digests and identity/subject relationships. Classify duplicate delivery, semantically identical alternate transport, and contradictory immutable attempt independently. Persist receipt/provenance relationships transactionally with the evidence index. A malicious unrelated file should be rejected without silently replacing a valid result; a conflict relevant to required scope must remain visible and non-authorizing.

The database/index is not a source of trust. Imported statements that claim verified state cross the verification boundary again. Rebuilt indexes preserve identities and conflicts; they must not fabricate new successful verification receipts.

## DEV-005 binding projection allocation

S017's v2 projection includes every current binding, artifact, exposure and
evidence-specification semantic field and extension, using ADR-0013 numeric
forms and the explicit set/sequence classification in DEV-005 of the governing
sequence. The existing v1 projection is a field inventory, not reusable v2 bytes.
Only `dev.equivalencematrix.display` is an excluded extension namespace; declared
artifact paths remain semantic. Comments, formatting and source spans do not
enter Binding. Use the existing BindingDigest domain and preserve v1 projection
APIs/hashes. Later owning v2 construction/relationship steps integrate the new
component identity; S017 does not silently reinterpret existing v1 transports.

## DEV-006 contract and selected-policy identities

ContractDigest binds the explicit v2 capabilities/journeys/surfaces/fragments plus
normative graph extensions from one finalized prepared graph. All exact loaded
revisions and expanded surface requirements remain; unrelated target/binding/
policy/profile/runner/waiver/import/adapter collections are separate identities.
Entity titles/descriptions are retained; only the display extension namespace and
parse provenance are excluded. Journey surface order is semantic.

PolicySelectionDigest binds the complete selected Policy and selected Profile
definitions plus each declared dimension's known or explicit unknown value. IDs
and revisions alone do not suffice. A narrow validated domain input carries exact
definitions/values; manifest owns pure v2 canonicalization; application selects
and resolves definitions from one immutable PreparedSession graph without I/O,
default invention, latest-revision fallback or authority promotion. Existing
engine selection/mode/origin rules stay in force.

The governing DEV-006 amendment freezes exact JSON member names, array semantics,
carrier invariants and S019-A/B/C allocation. Recursive extension type tags retain
integer-versus-text distinctions; v1 and unaffected component oracles remain exact.
The workspace digest remains aggregate context, never a substitute identity.

## DEV-007 target configuration and applicability

Applicability all/any children are unordered semantic sets: recursively project
and sort complete member JCS bytes, rejecting duplicates. Domain enum iteration
is not the wire order. Preserve v1 and domain evaluation; journey surfaces and
extension arrays remain ordered. DEV-007 supersedes earlier ordered-child claims.

S020 hashes the complete current Target definition (id/root/platform/framework/
owners/extensions), with typed TargetConfigurationDigest and exact same-session
target lookup. Missing targets fail explicitly. No build/profile/source reference
fields exist in Target at this stage; do not invent placeholders. Policy selection,
source snapshot/component content and build artifact identities remain separate
required subject axes. Later role/source modeling steps extend the target
projection atomically with new normative fields; S031/S038/S043 prove linkage.
Exact spellings, APIs, failures and regression controls are normative in DEV-007
of the governing sequence. This component digest grants no trust or release authority.

## DEV-008 provenance completion

Repository-wide clean inspection permits only reserved untracked .eqm outputs.
Ignored non-output content and Git concealment flags cannot authorize clean source.
Dirty inspection cannot satisfy clean release provenance; inspection alone is not
immutable execution proof. DEV-008 freezes conservative gate/fixture integration.

## DEV-009 checked identity and model allocation

DEV-009 in the sole governing COMMIT_SEQUENCE closes bytes-first lossless v2
semantic evidence, explicit lineage/admission context, recursive source inspection
and the exact shared-source/participant/subject/cohort engineering allocation.
Use its member/version/domain decisions at each owning step; preserve original
criteria, v1 contracts and signed bytes. Parsed identifiers/digests are not trust,
synthetic combinations are not real cohort certification, and new target fields
extend their canonical projection atomically. S043 is the next independent
integration review boundary before routing; full MVP remains S211.

## DEV-010 checked v2 admission closure

Expected component context and checked access retain existing distinct digest
types; exact structural wire SHA comparison binds independently computed typed
expectations without an unchecked cast or digest-string rehash. Preserve legacy
v1 domain carriers. V2 numbered retries have non-overlapping chronological
windows (next start >= previous finish), equality allowed and nanos preserved,
checked before admission. No added observed_at/freshness rule. Recursive Gitlink
inspection rejects symlinked ancestors and canonical-root escape before child
Git. Exact controls and two repair checkpoints live in the sole governing plan.

## DEV-011 repeated evaluation history

Record validation consumes explicit caller-supplied definition order, not time/path
sorting or inferred trust. References resolve only to earlier definitions. Preserve
kind-direction and cycle checks while allowing already-defined deployment receipts
as inputs to later Runtime/FrozenInput records. Initial evidence cannot require
its future decision/deployment. Canonical typed reference sets remain separate
from ordered definition history. This domain structure confers no authority, storage
or wire publication. Exact repair/evidence scope lives in the governing sequence.
