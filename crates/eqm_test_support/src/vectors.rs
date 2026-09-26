//! Normative representation and rule vectors frozen by ADR-0013.
//!
//! These vectors are declarative acceptance points, not an implementation of
//! the semantics. Owning steps S013–S076 enforce and expand them; the module
//! test proves the frozen set covers every representation boundary and every
//! approved outcome/count/risk rule with a nonempty clause.

/// One frozen conformance vector with an exact clause.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RepresentationVector {
    /// Stable vector identifier.
    pub id: &'static str,
    /// Frozen category this vector covers.
    pub category: &'static str,
    /// Precondition.
    pub given: &'static str,
    /// Action under evaluation.
    pub when: &'static str,
    /// Required result.
    pub expected: &'static str,
}

/// Every category the frozen vector set must cover.
pub const REQUIRED_VECTOR_CATEGORIES: &[&str] = &[
    "namespace",
    "integer_range",
    "decimal_string",
    "timestamp",
    "absence",
    "set",
    "sequence",
    "text_unicode",
    "digest_domain",
    "conjunction",
    "alternatives",
    "distinct_fail_pass",
    "retry_unstable",
    "attempt_conflict",
    "timeout_unknown",
    "missing_non_success",
    "count_unit",
    "campaign_selection",
    "fragment_risk",
];

/// The normative frozen vector set for ADR-0013.
pub const REPRESENTATION_VECTORS: &[RepresentationVector] = &[
    RepresentationVector {
        id: "RV-NAMESPACE-1",
        category: "namespace",
        given: "an incompatible wire shape",
        when: "a new contract is authored",
        expected: "it uses eqm:v2 and schemas/v2 while schemas/v1 bytes and URIs stay unchanged",
    },
    RepresentationVector {
        id: "RV-NAMESPACE-2",
        category: "namespace",
        given: "a document declaring an unsupported wire version",
        when: "the reader parses it",
        expected: "it fails explicitly or reports unavailable; no implicit v1/v2 reader or alias applies",
    },
    RepresentationVector {
        id: "RV-INTEGER-1",
        category: "integer_range",
        given: "a non-negative count or a signed integer within 2^53-1",
        when: "canonical JSON is produced",
        expected: "the value is an exact JSON integer",
    },
    RepresentationVector {
        id: "RV-INTEGER-2",
        category: "integer_range",
        given: "a fractional, negative count, NaN or infinite number",
        when: "the wire value is parsed",
        expected: "it is rejected",
    },
    RepresentationVector {
        id: "RV-DECIMAL-1",
        category: "decimal_string",
        given: "an integral quantity outside the exact IEEE-754 range",
        when: "it is encoded",
        expected: "it uses a decimal string matching ^(0|[1-9][0-9]*)$",
    },
    RepresentationVector {
        id: "RV-DECIMAL-2",
        category: "decimal_string",
        given: "a decimal string with a sign, leading zero or fraction",
        when: "it is parsed as an exact quantity",
        expected: "it is rejected",
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-1",
        category: "timestamp",
        given: "a canonical UTC instant",
        when: "it is encoded",
        expected: "it uses RFC 3339 with a literal Z and AutoSi subseconds",
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-2",
        category: "timestamp",
        given: "an instant with an offset, a missing Z or a leap second, or a freshness comparison needing sub-millisecond precision",
        when: "it is parsed or compared",
        expected: "the form is rejected and arithmetic stays exact in nanoseconds without flooring",
    },
    RepresentationVector {
        id: "RV-ABSENCE-1",
        category: "absence",
        given: "an optional member that is absent",
        when: "the document is authored",
        expected: "the member is omitted and absence is not success",
    },
    RepresentationVector {
        id: "RV-ABSENCE-2",
        category: "absence",
        given: "an optional member encoded as explicit null",
        when: "the document is parsed",
        expected: "it is rejected",
    },
    RepresentationVector {
        id: "RV-SET-1",
        category: "set",
        given: "an unordered semantic set",
        when: "its members include a duplicate",
        expected: "the duplicate is rejected",
    },
    RepresentationVector {
        id: "RV-SET-2",
        category: "set",
        given: "a set of distinct members delivered out of order",
        when: "it is encoded and compared",
        expected: "members are canonically ordered and results are stable under permutation",
    },
    RepresentationVector {
        id: "RV-SEQUENCE-1",
        category: "sequence",
        given: "an ordered sequence",
        when: "members are permuted",
        expected: "the change is significant and the sequence is not reordered",
    },
    RepresentationVector {
        id: "RV-TEXT-1",
        category: "text_unicode",
        given: "authored text that is not NFC",
        when: "it is validated",
        expected: "it is rejected rather than normalized",
    },
    RepresentationVector {
        id: "RV-TEXT-2",
        category: "text_unicode",
        given: "NFC text containing non-ASCII code points",
        when: "it is canonicalized and digested",
        expected: "code points are preserved exactly and the exact UTF-8 bytes are hashed",
    },
    RepresentationVector {
        id: "RV-DIGEST-1",
        category: "digest_domain",
        given: "a v2 semantic digest",
        when: "it is produced",
        expected: "it uses sha256:<64 lowercase hex> over the eqm:v2:<kind> label, zero delimiter and content",
    },
    RepresentationVector {
        id: "RV-CONJUNCTION-1",
        category: "conjunction",
        given: "a required expected selection with several members",
        when: "some members pass and a required member fails or is missing",
        expected: "the coordinate is not satisfied; enough passes cannot hide the required gap",
    },
    RepresentationVector {
        id: "RV-ALTERNATIVE-1",
        category: "alternatives",
        given: "no explicit protected alternative group is authorized",
        when: "observed results would let an alternative pass",
        expected: "no OR substitution is applied",
    },
    RepresentationVector {
        id: "RV-ALTERNATIVE-2",
        category: "alternatives",
        given: "an explicit protected alternative group frozen into the selection",
        when: "exactly one authorized alternative is satisfied",
        expected: "the group composes with the required conjunction rather than replacing it",
    },
    RepresentationVector {
        id: "RV-FAILPASS-1",
        category: "distinct_fail_pass",
        given: "distinct required pass and fail cases at one coordinate",
        when: "the coordinate aggregates",
        expected: "it aggregates to Failed",
    },
    RepresentationVector {
        id: "RV-RETRY-1",
        category: "retry_unstable",
        given: "fail then pass retries of the same logical case in the selected campaign",
        when: "the coordinate aggregates",
        expected: "it aggregates to Unstable and the earlier failure stays visible",
    },
    RepresentationVector {
        id: "RV-CONFLICT-1",
        category: "attempt_conflict",
        given: "contradictory assertions for the same immutable attempt",
        when: "the coordinate aggregates",
        expected: "it aggregates to Unknown conflict",
    },
    RepresentationVector {
        id: "RV-TIMEOUT-1",
        category: "timeout_unknown",
        given: "a timed-out or cancelled execution",
        when: "the outcome is classified",
        expected: "it is Unknown and cannot be waived",
    },
    RepresentationVector {
        id: "RV-MISSING-1",
        category: "missing_non_success",
        given: "a skipped, filtered, quarantined or missing required selection",
        when: "the coordinate aggregates",
        expected: "it remains non-success",
    },
    RepresentationVector {
        id: "RV-COUNT-1",
        category: "count_unit",
        given: "one release-record reference",
        when: "an arbitrary observation minimum is applied",
        expected: "at most one eligible record unit counts; copies, transports and retries add no credit",
    },
    RepresentationVector {
        id: "RV-COUNT-2",
        category: "count_unit",
        given: "several evidence kinds",
        when: "a minimum is composed",
        expected: "the minimum uses the declared logical unit for that kind",
    },
    RepresentationVector {
        id: "RV-CAMPAIGN-1",
        category: "campaign_selection",
        given: "several campaigns with different timestamps",
        when: "current acceptance is evaluated",
        expected: "the explicit authorized selection decides, never the latest passing timestamp",
    },
    RepresentationVector {
        id: "RV-CAMPAIGN-2",
        category: "campaign_selection",
        given: "a selected campaign plus unrelated superseded history",
        when: "the campaign is evaluated",
        expected: "unrelated superseded history neither poisons nor substitutes for it",
    },
    RepresentationVector {
        id: "RV-RISK-1",
        category: "fragment_risk",
        given: "an expanded fragment requirement with a stronger fragment or destination-journey risk",
        when: "the expanded graph is validated",
        expected: "the stronger risk is inherited when absent and an explicit lower value rejects",
    },
    RepresentationVector {
        id: "RV-RISK-2",
        category: "fragment_risk",
        given: "a weakened risk introduced during expansion",
        when: "finalized or prepared construction is attempted",
        expected: "validation rejects the weakened graph",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn frozen_vectors_cover_every_required_category() {
        let covered: BTreeSet<&str> = REPRESENTATION_VECTORS
            .iter()
            .map(|vector| vector.category)
            .collect();
        for category in REQUIRED_VECTOR_CATEGORIES {
            assert!(
                covered.contains(category),
                "missing frozen vector category: {category}"
            );
        }
    }

    #[test]
    fn frozen_vectors_have_unique_ids_and_nonempty_clauses() {
        let mut ids = BTreeSet::new();
        for vector in REPRESENTATION_VECTORS {
            assert!(ids.insert(vector.id), "duplicate vector id: {}", vector.id);
            assert!(!vector.category.is_empty(), "{}: empty category", vector.id);
            assert!(!vector.given.is_empty(), "{}: empty given", vector.id);
            assert!(!vector.when.is_empty(), "{}: empty when", vector.id);
            assert!(!vector.expected.is_empty(), "{}: empty expected", vector.id);
        }
    }
}
