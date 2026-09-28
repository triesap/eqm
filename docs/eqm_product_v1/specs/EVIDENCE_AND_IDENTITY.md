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
