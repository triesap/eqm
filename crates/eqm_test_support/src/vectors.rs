//! Normative representation and rule vectors frozen by ADR-0013.
//!
//! Every vector carries a literal input, an exact accept output or rejection,
//! and a note stating the rule. Timestamp and digest vectors are executable
//! against `eqm_domain` today; the remaining vectors are consumed by the owning
//! enforcement steps (S015 for wire numeric/text/set forms, later steps for the
//! outcome/count/risk rules). None of this module implements production policy.

/// The resolved outcome of one representation vector.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VectorOutcome {
    /// The input is accepted and maps to this exact canonical output.
    Accept {
        /// Exact canonical output text for the accepted input.
        output: &'static str,
    },
    /// The input is rejected.
    Reject,
}

/// One frozen conformance vector with literal input and exact expectation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RepresentationVector {
    /// Stable vector identifier.
    pub id: &'static str,
    /// Frozen category this vector covers.
    pub category: &'static str,
    /// Literal wire input (or a scenario identifier for rule vectors).
    pub input: &'static str,
    /// Exact acceptance output or rejection.
    pub outcome: VectorOutcome,
    /// Rule statement the vector pins.
    pub note: &'static str,
    /// Expected canonical digest text when the rule produces one.
    pub expected_digest: Option<&'static str>,
}

const fn accept(output: &'static str) -> VectorOutcome {
    VectorOutcome::Accept { output }
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
    // Namespace (v2).
    RepresentationVector {
        id: "RV-NAMESPACE-1",
        category: "namespace",
        input: "new incompatible contract",
        outcome: accept("eqm:v2 / schemas/v2/"),
        note: "incompatible wire work uses the eqm:v2 namespace and schemas/v2 base",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-NAMESPACE-2",
        category: "namespace",
        input: "{\"schema\":\".../schemas/v3/manifest/capability.schema.json\"}",
        outcome: VectorOutcome::Reject,
        note: "an unsupported wire version fails explicitly; there is no implicit reader",
        expected_digest: None,
    },
    // Unsigned integers within the exact JSON range are plain integers.
    RepresentationVector {
        id: "RV-INTEGER-1",
        category: "integer_range",
        input: "0",
        outcome: accept("0"),
        note: "unsigned counts accept zero as an exact JSON integer",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-INTEGER-2",
        category: "integer_range",
        input: "9007199254740991",
        outcome: accept("9007199254740991"),
        note: "2^53-1 is the largest exact JSON integer",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-INTEGER-3",
        category: "integer_range",
        input: "9007199254740992",
        outcome: VectorOutcome::Reject,
        note: "2^53 and above are not exact JSON integers; use a decimal string",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-INTEGER-4",
        category: "integer_range",
        input: "-1",
        outcome: VectorOutcome::Reject,
        note: "unsigned counts and lengths are never negative",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-INTEGER-5",
        category: "integer_range",
        input: "-9007199254740991",
        outcome: accept("-9007199254740991"),
        note: "signed facts accept the exact negative JSON integer boundary",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-INTEGER-6",
        category: "integer_range",
        input: "1.5",
        outcome: VectorOutcome::Reject,
        note: "counts and integer facts are never fractional",
        expected_digest: None,
    },
    // Decimal strings carry out-of-range exact values.
    RepresentationVector {
        id: "RV-DECIMAL-1",
        category: "decimal_string",
        input: "\"9007199254740992\"",
        outcome: accept("\"9007199254740992\""),
        note: "an unsigned exact value beyond 2^53-1 is a decimal string",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-2",
        category: "decimal_string",
        input: "\"-9007199254740992\"",
        outcome: accept("\"-9007199254740992\""),
        note: "a signed exact value beyond 2^53-1 is a signed decimal string",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-3",
        category: "decimal_string",
        input: "\"01\"",
        outcome: VectorOutcome::Reject,
        note: "decimal strings forbid leading zeros",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-4",
        category: "decimal_string",
        input: "\"+1\"",
        outcome: VectorOutcome::Reject,
        note: "decimal strings forbid an explicit plus sign",
        expected_digest: None,
    },
    // Timestamps: canonical UTC with AutoSi subseconds (0/3/6/9 digits).
    RepresentationVector {
        id: "RV-TIMESTAMP-1",
        category: "timestamp",
        input: "2026-01-02T03:04:05Z",
        outcome: accept("2026-01-02T03:04:05Z"),
        note: "whole seconds carry no fractional component",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-2",
        category: "timestamp",
        input: "2026-01-02T03:04:05.100Z",
        outcome: accept("2026-01-02T03:04:05.100Z"),
        note: "millisecond values use exactly three digits (AutoSi)",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-3",
        category: "timestamp",
        input: "2026-01-02T03:04:05.1Z",
        outcome: VectorOutcome::Reject,
        note: "a shortened fraction is not canonical; AutoSi emits three digits",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-4",
        category: "timestamp",
        input: "2026-01-02T03:04:05.000Z",
        outcome: VectorOutcome::Reject,
        note: "a zero fraction is not canonical; omit the fraction instead",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-5",
        category: "timestamp",
        input: "2026-01-02T03:04:05.000001Z",
        outcome: accept("2026-01-02T03:04:05.000001Z"),
        note: "microsecond values use exactly six digits (AutoSi)",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-6",
        category: "timestamp",
        input: "2026-01-02T03:04:05.000000001Z",
        outcome: accept("2026-01-02T03:04:05.000000001Z"),
        note: "nanosecond values use exactly nine digits (AutoSi)",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-7",
        category: "timestamp",
        input: "2026-01-02T03:04:05+00:00",
        outcome: VectorOutcome::Reject,
        note: "offsets are rejected; the canonical form carries a literal Z",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-8",
        category: "timestamp",
        input: "2026-01-02T03:04:05",
        outcome: VectorOutcome::Reject,
        note: "a missing UTC designator is rejected",
        expected_digest: None,
    },
    // Absence.
    RepresentationVector {
        id: "RV-ABSENCE-1",
        category: "absence",
        input: "{}",
        outcome: accept("{}"),
        note: "an absent optional member is omitted and is not success",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-ABSENCE-2",
        category: "absence",
        input: "{\"optional\":null}",
        outcome: VectorOutcome::Reject,
        note: "an optional member is never encoded as explicit null",
        expected_digest: None,
    },
    // Sets and sequences.
    RepresentationVector {
        id: "RV-SET-1",
        category: "set",
        input: "[\"b\",\"a\"]",
        outcome: accept("[\"a\",\"b\"]"),
        note: "set members are canonically ordered",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-SET-2",
        category: "set",
        input: "[\"a\",\"a\"]",
        outcome: VectorOutcome::Reject,
        note: "duplicate set members are rejected, not collapsed",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-SEQUENCE-1",
        category: "sequence",
        input: "[\"b\",\"a\"]",
        outcome: accept("[\"b\",\"a\"]"),
        note: "an ordered sequence preserves declared order",
        expected_digest: None,
    },
    // Text.
    RepresentationVector {
        id: "RV-TEXT-1",
        category: "text_unicode",
        input: "\"cafe\\u0301\"",
        outcome: VectorOutcome::Reject,
        note: "authored text must be NFC; it is rejected, never normalized",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TEXT-2",
        category: "text_unicode",
        input: "\"caf\\u00e9\"",
        outcome: accept("\"caf\\u00e9\""),
        note: "NFC authored text is preserved and hashed as exact UTF-8 bytes",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TEXT-3",
        category: "text_unicode",
        input: "opaque attachment bytes",
        outcome: accept("opaque attachment bytes"),
        note: "opaque Unicode-bearing/attachment payload bytes are not NFC-validated",
        expected_digest: None,
    },
    // Digest domains.
    RepresentationVector {
        id: "RV-DIGEST-1",
        category: "digest_domain",
        input: "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        outcome: accept("sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        note: "a digest is sha256: plus 64 lowercase hex",
        expected_digest: Some(
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
    },
    RepresentationVector {
        id: "RV-DIGEST-2",
        category: "digest_domain",
        input: "sha256:E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
        outcome: VectorOutcome::Reject,
        note: "uppercase hex is rejected",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DIGEST-3",
        category: "digest_domain",
        input: "sha256:00",
        outcome: VectorOutcome::Reject,
        note: "a short digest is rejected",
        expected_digest: None,
    },
    // Rule vectors (executed by their owning semantic steps).
    RepresentationVector {
        id: "RV-CONJUNCTION-1",
        category: "conjunction",
        input: "required members: one missing, others pass",
        outcome: accept("not satisfied"),
        note: "required selections compose with AND; passes cannot hide a missing member",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-ALTERNATIVE-1",
        category: "alternatives",
        input: "observed pass would select an unauthorized alternative",
        outcome: accept("not satisfied"),
        note: "no OR substitution without an explicit protected authorized group",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-ALTERNATIVE-2",
        category: "alternatives",
        input: "one authorized alternative in an explicit protected group",
        outcome: accept("group composes with AND"),
        note: "an authorized alternative composes with the required conjunction",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-FAILPASS-1",
        category: "distinct_fail_pass",
        input: "distinct required pass and fail cases",
        outcome: accept("Failed"),
        note: "distinct required fail/pass cases aggregate to Failed",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-RETRY-1",
        category: "retry_unstable",
        input: "fail then pass retries of one case",
        outcome: accept("Unstable"),
        note: "fail/pass retries of one logical case aggregate to Unstable",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-CONFLICT-1",
        category: "attempt_conflict",
        input: "contradictory assertions for one immutable attempt",
        outcome: accept("Unknown"),
        note: "same-attempt contradictions aggregate to Unknown conflict",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMEOUT-1",
        category: "timeout_unknown",
        input: "timed out or cancelled execution",
        outcome: accept("Unknown"),
        note: "timeout/cancellation is Unknown and cannot be waived",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-MISSING-1",
        category: "missing_non_success",
        input: "skipped, filtered, quarantined or missing selection",
        outcome: accept("non-success"),
        note: "missing selections remain non-success",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-COUNT-1",
        category: "count_unit",
        input: "one release-record reference against a minimum of two",
        outcome: accept("at most one unit"),
        note: "one reference is one unit; copies and retries add no credit",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-COUNT-2",
        category: "count_unit",
        input: "several evidence kinds with declared units",
        outcome: accept("per-kind logical unit"),
        note: "the minimum uses the declared logical unit for the kind",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-CAMPAIGN-1",
        category: "campaign_selection",
        input: "several campaigns with different timestamps",
        outcome: accept("explicit authorized selection"),
        note: "never the latest passing timestamp",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-CAMPAIGN-2",
        category: "campaign_selection",
        input: "selected campaign plus unrelated superseded history",
        outcome: accept("selected campaign only"),
        note: "unrelated superseded history neither poisons nor substitutes",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-RISK-1",
        category: "fragment_risk",
        input: "expanded fragment with a stronger fragment/journey risk",
        outcome: accept("stronger risk inherited"),
        note: "absence inherits the stronger risk; an explicit lower value rejects",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-RISK-2",
        category: "fragment_risk",
        input: "risk weakened during expansion",
        outcome: VectorOutcome::Reject,
        note: "validation rejects a weakened expanded graph",
        expected_digest: None,
    },
];

/// Returns every vector in one category.
pub fn vectors_for(category: &str) -> impl Iterator<Item = &'static RepresentationVector> {
    REPRESENTATION_VECTORS
        .iter()
        .filter(move |vector| vector.category == category)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{Sha256Digest, UtcInstant};
    use std::collections::BTreeSet;
    use std::str::FromStr;

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
    fn frozen_vectors_have_unique_ids_and_concrete_expectations() {
        let mut ids = BTreeSet::new();
        for vector in REPRESENTATION_VECTORS {
            assert!(ids.insert(vector.id), "duplicate vector id: {}", vector.id);
            assert!(!vector.input.is_empty(), "{}: empty input", vector.id);
            assert!(!vector.note.is_empty(), "{}: empty note", vector.id);
            match vector.outcome {
                VectorOutcome::Accept { output } => {
                    assert!(!output.is_empty(), "{}: empty accept output", vector.id)
                }
                VectorOutcome::Reject => {}
            }
        }
    }

    #[test]
    fn timestamp_vectors_match_the_canonical_utc_rule() {
        let mut checked = 0;
        for vector in vectors_for("timestamp") {
            checked += 1;
            let parsed = UtcInstant::from_str(vector.input);
            match vector.outcome {
                VectorOutcome::Accept { output } => {
                    let rendered = parsed.map(|value| value.to_string()).ok();
                    assert_eq!(rendered.as_deref(), Some(output), "{}", vector.id);
                }
                VectorOutcome::Reject => {
                    assert!(parsed.is_err(), "{}", vector.id);
                }
            }
        }
        assert!(checked >= 8);
    }

    #[test]
    fn digest_vectors_match_the_canonical_digest_rule() {
        let mut checked = 0;
        for vector in vectors_for("digest_domain") {
            checked += 1;
            let parsed = Sha256Digest::from_str(vector.input);
            match vector.outcome {
                VectorOutcome::Accept { output } => {
                    let rendered = parsed.map(|value| value.to_string()).ok();
                    assert_eq!(rendered.as_deref(), Some(output), "{}", vector.id);
                    if let Some(expected) = vector.expected_digest {
                        assert_eq!(rendered.as_deref(), Some(expected), "{}", vector.id);
                    }
                }
                VectorOutcome::Reject => {
                    assert!(parsed.is_err(), "{}", vector.id);
                }
            }
        }
        assert!(checked >= 3);
    }
}
