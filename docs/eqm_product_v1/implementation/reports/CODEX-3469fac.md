# Supervisor review of 3469fac

Date: 2026-09-28. Reviewer: Codex under owner-delegated engineering authority.
Reviewed branch: master; initial worktree clean. Reviewed source:
`3469face3a88d280f2577f4ff936419408c1d248`.
Prior supervisor baseline: `c14ac20dffac451380b1b2bea771a575b0ba8bd9`.

**Disposition:** RF20/RF21 accepted for allocated scope. RF22 partially repaired,
not closed. S017/S018 projections are implemented but require RF24/RF25/RF26
corrections. Retain all four commits; next S016-R3, S018-R1/R2, then S019-A/B/C.
S019's decision-required block was correctly escalated and is now resolved by
Codex in DEV-006. No application code was generated in this supervisor review.

## Reviewed work and scope

| Checkpoint | Commit | Disposition |
| --- | --- | --- |
| S016-R1 | 49d7177ee68c77e5a43c17ac40de35a39f611c78 | Typed authored ingress accepted; RF20/RF21 closed |
| S016-R2 | 8d872757517bb287907ce1e5e0d6fe9c1d5f9079 | Meaningful improvements; remaining RF22 matrix required |
| S017 | 93a82a141d3ffdb03d62cf3d088d5e14afa898a6 | Retain separate v2 projection; identity/evidence corrections required |
| S018 | 3469face3a88d280f2577f4ff936419408c1d248 | Retain specification projection/shared helpers; same corrections required |

Reviewed the entire new binding/specification/support modules and tests, protocol
validator diff and owning call paths, the new authored-ingress test file, full
collection regression file, vector/decoder changes, domain constructors/types,
selection and session boundaries, relevant v1 projections and identity/spec
contracts, reports/progress/status, checkpoint logs and handback. This is not a
claim to have freshly reread all unchanged code in every later subsystem.
Findings are source-path and test-coverage review; no new Rust implementation or
regression code was written by Codex. Pi must add the specified executable proofs.

## Findings, ordered by impact

### RF26 — P1: different extension types collapse to one component identity

`crates/eqm_manifest/src/canonical/support.rs:81` sends ExtensionValue::Integer
through wire_int. Out-of-safe-range integers become JSON strings. The next branch
at line 82 emits ExtensionValue::String as a JSON string without a type tag.
Consequently Integer(i64::MIN) and String("-9223372036854775808") under the same
semantic extension key produce identical Value, canonical bytes and digest with
all other fields fixed. Both domain values are constructible; Extensions::new
bounds/normalizes strings but does not forbid digit-containing text.

This is a deterministic semantic collision in the projection, not a SHA-256
collision. It affects both newly added component projections and would propagate
into S019 if its shared helper were reused unchanged. No current end-to-end
consumer integration or live trust exploitation is claimed.

**Required repair / Codex decision:** S018-R1 recursively tags all v2 extension
values with `type` and `value` (boolean/integer/string/array/object), retaining
ordered arrays, JCS object maps, approved numeric forms and the precise display
namespace exclusion. Add independent literal and same-ID integer-versus-text
mutation controls, including nested data. V1 and empty-extension base oracles
remain unchanged; only the affected new v2 extension oracles are deliberately
updated under DEV-006. A generic numeric-looking string must not be coerced.

### RF24 — P2: new component APIs erase the domain-specific identity types

`binding_v2.rs:37,49,191` stores/returns Sha256Digest and hashes via DigestDomain.
`evidence_spec_v2.rs:39,51,110` does the same. The labels separate hash content,
but the Rust API still permits a binding digest to be supplied where a specification
digest is expected. This bypasses the S013 typed identity wrappers and DEV-005's
allocation to BindingDigest/EvidenceSpecificationDigest.

**Required repair:** S018-R1 stores/returns the existing wrappers and constructs
them through hash_content. No new domain labels, wrapper types, v1 API changes or
implicit conversions. Typed positive API tests plus a cross-domain compile-fail
doctest must execute. Wrapper-only conversion does not change bytes; RF26's
separate approved representation correction does change affected extension bytes.

### RF22 — P2, still partial: claimed boundary matrix remains incomplete

The test at `canonical_collections.rs:315` creates one diagnostic. The supposed
"diagnostic order" mutation at line 338 changes that diagnostic's related locations,
not the diagnostic list. It therefore cannot detect sorting/deduplication of the
actual diagnostics vector. Its report claims diagnostic-list reordering proof.

Exact-member duplicates are missing for facets, attachments, evidence/runtime
profile_values, and standalone/embedded inventory entries. Existing profile and
inventory cases mostly exercise differing records with one logical key, which
is a different branch. Attachment permutations lack the required equality control.
Runtime profile values also lack the differing-value same-dimension case. The
mutant digest helper still suppresses serialization errors with unwrap_or_default.

**Required repair:** S016-R3 executes the explicit DEV-006 matrix, retaining every
successful R1/R2 control. Test outer inventory/adapter diagnostics with distinct
members and repeated members. Use valid controls, recomputed mutant digests, and
JCS of actual parsed DTOs for canonical byte claims. This review does not allege
that all missing cases currently fail; it identifies required proof not supplied.

The implemented attempt index rule, shared metadata validation, literal facts
oracle, both facts/profile permutations, related-location control, and raw-byte/
domain-hash vectors are useful and retained. RF20/RF21 do not need another repair.

### RF25 — P2: manifest-boundary proof is substituted and duplicate claim is false

`binding_v2` test `projection_excludes_parse_provenance_and_keeps_only_semantics`
at line 315 inspects keys and constructs the same Binding twice. It does not load
an original and a comment/formatting-mutated manifest through real conversion,
as DEV-005 required. Ordered extension arrays are tested, but ordered all/any
expression children are not.

S017/S018 reports assert canonical duplicate rejection cannot be reached through
typed constructors. Binding::new instead checks equality of full Exposure values,
including extensions. Two exposures differing only in their display extension
pass this equality check; excluding those extensions yields equal projected
members. CanonicalSet correctly rejects them, but the advertised impossibility
and omitted boundary test are wrong.

**Required repair:** S018-R2 adds actual load_workspace/conversion controls with
valid unchanged-ID comment/formatting and semantic mutations, nested display
exclusion, publicly constructible projected duplicate rejection, accepted distinct
exposures, and ordered all/any sensitivity. Correct report claims; do not add
unchecked constructors to manufacture test inputs.

## Accepted behavior and evidence limits

S016-R1 now calls shared typed validation from all five owning v2 roots, including
embedded inventory. It validates subject metadata, typed IDs/vocabularies,
reviewer OwnerRef, profile revisions, attachment/entry metadata and diagnostics'
unit/target references. Typed Text restrictions remain distinct from arbitrary
observed prose and opaque bytes. The selector helper's consumer integration is
accurately pending: no current owning v2 selector root exists.

The v2 projections explicitly include full field semantics and use set member
bytes rather than IDs as their order. S018 reuses the specification projection
inside binding; numeric forms and display filtering are additive to v1. These
improvements are retained while the extension collision is corrected.

Saved final targeted/standard/schema/structure/dependency logs are green at each
reported checkpoint. Historical RF23 artifacts were preserved. The R1 raw log
`test-protocol-iter1.log` also records an intermediate targeted failure, in
addition to the handback's mentioned R2 clippy and S017 test failures. Final R1
runs pass; summarize all known failures accurately rather than saying there were
exactly two. A final saved log is not proof that no earlier run failed.

## Independent verification on reviewed source

| Check | Result |
| --- | --- |
| cargo xtask check | Exit 0; 393 passed test results; standard schema/CLI/MCP fixtures pass |
| cargo deny check | Exit 0; advisories/bans/licenses/sources pass; existing allowed duplicate warning remains |
| cargo audit --deny warnings | Exit 0 against live database; 160 dependencies scanned |
| Structure-only package validator | Exit 0; 98 files on reviewed source |
| Independent SHA-256 recomputation | Four existing binding/specification base/boundary literal hashes match label + zero delimiter + literal bytes |
| Diff of schemas/v1, schemas/v2, Cargo.lock across batch | Empty |
| Initial Git status | Clean at 3469fac |

Raw logs are retained externally as `codex-review-3469fac-{check,deny,audit,package,
literal-oracles}.log`. Hash recomputation confirms the stated byte hashes, not
injectivity of the semantic projection; RF26 still exists. Planning-update checks
are recorded separately. Cargo work was routed through the configured build
router after successful diagnostics, serially. No extended verify, Linux,
coverage/mutation/fuzz/bench, live product/crypto/host or release/signing lanes
were run or represented as established by this review.

## Decisions, continuation and release state

[DEV-006](../COMMIT_SEQUENCE.md#dev-006--close-boundary-evidence-and-unblock-contract-provenance)
settles S019's choices: contract entities plus normative graph extensions; complete
v2 entity forms; narrow validated domain selection carriers; manifest-owned
serialization; full selected policy/profile definitions plus known/unknown values;
same-session application assembly using the existing engine selection algorithm.
No manifest-to-engine/protocol dependency, trusted-default invention, latest-revision
selection or blind reuse of v1 bytes is approved. Three independently green S019
slices preserve the original numbered step and its R19/AC019 contribution.

The first next work is S016-R3, S018-R1/R2, then S019-A/B/C and ordered unblocked
S020–S211. S143 remains intermediate. Codex owns changed scope/representation/
sequence/acceptance decisions; Pi implements code and records factual progress.
A fresh Pi session is recommended to load this superseding contract cleanly,
although this handback was a decision block rather than context exhaustion.

Not a release candidate. Human release testing is not next. Real product,
platform and crypto evidence, protected authority and operational acceptance
remain with their allocated later steps/Q01–Q15. When those prerequisites reach
an actual candidate, run the declared extended/platform/package lanes and real
product CLI/MCP/acquisition/negative-state acceptance against a pinned candidate,
record immutable receipts, and return defects before release approval. Local
component hashes do not establish product conformance or publication authority.
