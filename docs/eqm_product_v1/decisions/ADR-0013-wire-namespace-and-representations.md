# ADR-0013: Freeze the v2 wire namespace and canonical representations

**Status:** Accepted (engineering closure under ADR-0012). **Date:** 2026-09-26.
**Requirement links:** R14,R16,R21,R23,R50,R52,R58. **Owning step:** S006.

## Context

ADR-0012 approved an explicit new wire namespace for incompatible changes and
required exact namespace/URI spelling, numeric ranges, timestamp precision,
null/absence, set/sequence and digest-domain representations to be frozen
before new semantics ship. COMPATIBILITY.md deferred the exact transition to
this step after inspecting consumers, generators, URI checks and releases.

Inspected evidence at baseline `7c8293837c09c627489cdf244a2fbd97b60aa8ed`:

- The repository is a standalone public OSS crate with no vendored consumer
  checkout and no published legacy compatibility obligation recorded.
- Schemas are generated from source DTOs and committed under `schemas/v1/`;
  `cargo xtask check` enforces an exact 22-schema inventory, `$id`/URI parity,
  and scans `examples/android-ios` and `tests/fixtures/signup` for
  `.../schemas/v1/...` URIs.
- Authored documents declare exact current schema URIs; the reader uses compiled
  local types and never fetches schemas.
- Existing digest domain labels are `eqm:v1:fragment` and
  `eqm:v1:semantic-graph`. Authored text is already required to be NFC.

No externally committed compatibility obligation conflicts with a monotonic
successor namespace. The repository's current-only rule means the transition is
a coordinated, additive namespace rather than an in-place reinterpretation.

## Decision

1. **Namespace.** Incompatible wire changes use the namespace `eqm:v2` and the
   schema URI base
   `https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/`, with
   `manifest/` and `protocol/` groups. New schemas live under `schemas/v2/`.
   Existing `schemas/v1/**` files, bytes and URIs are preserved unchanged and
   remain the v1 contract; they never acquire incompatible semantics. No
   implicit legacy reader, alias, or fallback bridges v1 and v2: an unsupported
   version is an explicit failure or unavailable view.
2. **Digest domains.** New v2 semantic digests use the label pattern
   `eqm:v2:<kind>` with the existing zero-byte delimiter and SHA-256. Subject,
   binding, evidence-specification, policy, runner, adapter and record kinds
   receive distinct labels when their owning steps introduce them. Existing v1
   labels and any already-signed bytes are never rewritten.
3. **Numbers (DEV-002 closure).** v2 unsigned quantities use the full u64
   range 0..18446744073709551615; signed quantities use the full i64 range
   -9223372036854775808..9223372036854775807. Within the safe interoperable
   interval `|n| <= 9007199254740991`, use a JSON integer token. Only outside
   that interval use a canonical unsigned/signed decimal string. Reject strings
   for in-range values, `+`, leading zeros, negative zero, fractional/exponent
   tokens and overflow. Positive counts additionally require at least one.
   Every supported endpoint must round-trip. Numeric schemas constrain both
   encoding and representable string ranges; regex syntax alone is insufficient.
   These rules apply to v2 only. Preserve v1's pre-S015 acceptance and bytes;
   its old schema identity does not acquire narrower numeric semantics.
4. **Time (DEV-002 closure).** v2 timestamps are canonical UTC RFC 3339,
   literal `Z`, four-digit years 0001–9999, exact nanoseconds, and the minimal
   AutoSi fractional group: absent at zero, otherwise 3, 6 or 9 digits.
   `.1Z`, `.000Z`, redundant groups, offsets, missing Z, leap seconds and
   unsupported precision reject. `.100Z`, `.000001Z`, `.000000001Z` are canonical.
   Use a validated v2 boundary without altering retained v1 parser acceptance.
   Freshness arithmetic must preserve exact nanoseconds at S069; no hidden skew.
5. **Null and absence.** Optional members are omitted when absent; they are
   never encoded as a present `null`. Explicit `null` is rejected for members
   with absence semantics. Distinguishing absent from present-but-null is
   normative, and absence is not success.
6. **Sets and sequences.** Unordered semantic sets are encoded as JSON arrays in
   canonical byte order of each member's canonical encoding; duplicate members
   are rejected. Ordered sequences preserve declared order and treat order as
   significant. Iteration order is never implementation-defined.
7. **Text.** Authored text fields (IDs, titles, statements, reviewer names,
   symbolic values) are exact NFC; non-NFC input is rejected rather than
   silently normalized, and their canonical digest is the exact UTF-8 bytes of
   the accepted NFC form. Opaque payload bytes (attachments, exported source
   content, media) are never NFC-validated or normalized; they are hashed and
   preserved exactly as received.
8. **Count units.** Counts are declared per evidence kind: expected logical test
   cases; independently checked structural/inventory/runtime facts; explicit
   review slots and independently authorized principals where independence is
   required; or an exact eligible independently verified referenced record.
   One release-record reference provides at most one eligible record unit.
   Copies, transports and retries add no credit.
9. **Outcome and risk rules.** Required expected selections compose with AND;
   alternatives require explicit protected authorization bound into the frozen
   selection. Distinct required fail/pass cases aggregate to Failed; fail/pass
   retries of one case aggregate to Unstable; contradictory assertions for one
   immutable attempt aggregate to Unknown. Timeout/cancellation is Unknown;
   missing/skipped/filtered/quarantined is non-success. Expanded fragment
   requirements inherit the stronger fragment/journey risk; absence inherits it
   and an explicit lower value rejects.
10. **Public API names.** Minimal public names are frozen at their owning steps;
    this ADR fixes only the namespace, representation and rule vocabulary. It
    does not invent command flags, JSON member spellings, or Kotlin/RPC names.

## Consequences and rejected shortcuts

Rejected: reinterpreting `schemas/v1` in place; a floating "latest" version;
accepting floats or ambiguous numeric strings; normalizing non-NFC text
silently; encoding absent optionals as `null`; unordered arrays; an implicit
v1↔v2 reader; and counting files, copies or retries as evidence units.

The generator and URI-parity checks must be extended together when the first v2
schema is authored (owning steps S013+). Until then, `cargo xtask check`
continues to enforce exactly the v1 inventory.

## Verification and implementation

`crates/eqm_test_support` exposes the normative frozen vector set
(`representation_vectors`) with a literal input, an exact accept output or
rejection, a rule note, and expected digest text where applicable. Its unit
tests prove the set covers each representation boundary and each approved
outcome/count/risk rule, and execute the timestamp and digest vectors against
`eqm_domain` so those vectors are real controls. S015 consumes the concrete
wire-form vectors; semantic enforcement is implemented by the owning steps
(S013–S076) and tested there. This ADR records the decision, not its
enforcement. Fixture thresholds and instants are synthetic; real policy
minimums, ages and skews remain gated by Q05/Q15.

## Supervisor allocation

DEV-002 in [COMMIT_SEQUENCE](../implementation/COMMIT_SEQUENCE.md) assigns
explicit version identity/generation to S015-R2 and the first five actual v2
protocol families to S015-R3. S015 owns null/absence enforcement. Later steps
own each additional concrete v2 shape atomically with generated schemas and
consumer updates. No placeholder schemas, fallback readers or v1 reinterpretation
are permitted. The supervisor owns changes to these decisions.

## DEV-003 conformance clarification

DEV-003 in the governing COMMIT_SEQUENCE.md closes reader version isolation,
nested structural validation and schema assertions. A known SchemaUri is not
proof that a reader or generated schema exists. Existing authored/workspace/lock
readers reject v2 until their owning integration steps. The five v2 families
must enforce exact root/nested schema markers and uniform declared byte/count
bounds. v1 schemas and accepted numeric/time forms remain unchanged.

Raw canonical JSON token validation, Draft 2020-12 value validation and runtime
semantic validation are separate obligations. JSON Schema integer includes
integral values written as 1.0 or 1e3; raw v2 readers reject those spellings.
Do not claim a schema proves original token form, duplicate-member rejection,
digest consistency or trust. Conformance tests use an actual standards validator
with date-time format assertions and local-only resolution. v2 UTC patterns
still enforce minimal AutoSi groups; runtime validation retains exact calendar
checks. No canonical input rule is relaxed by this clarification.

## DEV-004 collection and text allocation

S015 is accepted. DEV-004 in the governing COMMIT_SEQUENCE.md freezes S016's
field classification and executable controls: semantic sets sort by complete
canonical member JSON bytes, preserve existing logical-key uniqueness, and
reject duplicates before collection; attempts, diagnostics and related locations
retain sequence order. Strict object maps retain JCS key ordering. Preserve
original bytes separately from canonical semantic representations and retain all
S015 numeric/time/absence/bound checks. No v1 canonical projection is redefined.

Authored metadata is NFC-validated without normalization; opaque source/payload
bytes are preserved exactly. The text-vector input is JSON, its output canonical
JSON; RV-TEXT-2's Unicode escape denotes accepted café and canonical output emits
that literal UTF-8 text. A separate opaque-byte discipline includes decomposed
and non-UTF-8 cases. The governing amendment specifies real-ingress integration
and regression evidence; vector inventory alone is not implementation.

## DEV-005 structural field validation clarification

The governing sequence's DEV-005 field/type table is normative for S016. Exact
NFC is necessary but does not replace typed identifier, vocabulary, text-length
or positive-revision constraints. Shared validation covers each existing v2 root
and embedded inventory. Opaque bytes and arbitrary diagnostic prose retain their
separate classification. The unconsumed EvidenceSelectorV2Dto helper does not
prove consumer validation; the first owning v2 selector ingress must call its
typed validation. No extra schema family or relationship/trust acceptance is
introduced to manufacture that integration. S017 uses a separate v2 binding
projection under the existing BindingDigest domain; all v1 bytes stay frozen.
