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

/// The declared wire discipline a representation vector exercises.
///
/// Every vector names the exact control that must execute it, so no test needs
/// an ID allow-list or minimum-count proxy to decide what is applicable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VectorDiscipline {
    /// Canonical unsigned v2 integer (`WireUint`).
    UnsignedInteger,
    /// Canonical signed v2 integer (`WireInt`).
    SignedInteger,
    /// Canonical v2 UTC instant (`UtcInstantV2`).
    Timestamp,
    /// v2 optional-member absence.
    Absence,
    /// Canonical digest text.
    Digest,
    /// A semantic rule whose owning enforcement step is later than S015.
    SemanticRule,
}

impl VectorDiscipline {
    /// Every discipline, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UnsignedInteger,
        Self::SignedInteger,
        Self::Timestamp,
        Self::Absence,
        Self::Digest,
        Self::SemanticRule,
    ];
}

/// One frozen conformance vector with literal input and exact expectation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RepresentationVector {
    /// Stable vector identifier.
    pub id: &'static str,
    /// Frozen category this vector covers.
    pub category: &'static str,
    /// Exact control that must execute this vector.
    pub discipline: VectorDiscipline,
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
        discipline: VectorDiscipline::SemanticRule,
        input: "new incompatible contract",
        outcome: accept("eqm:v2 / schemas/v2/"),
        note: "incompatible wire work uses the eqm:v2 namespace and schemas/v2 base",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-NAMESPACE-2",
        category: "namespace",
        discipline: VectorDiscipline::SemanticRule,
        input: "{\"schema\":\".../schemas/v3/manifest/capability.schema.json\"}",
        outcome: VectorOutcome::Reject,
        note: "an unsupported wire version fails explicitly; there is no implicit reader",
        expected_digest: None,
    },
    // Unsigned integers within the exact JSON range are plain integers.
    RepresentationVector {
        id: "RV-INTEGER-1",
        category: "integer_range",
        discipline: VectorDiscipline::UnsignedInteger,
        input: "0",
        outcome: accept("0"),
        note: "unsigned counts accept zero as an exact JSON integer",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-INTEGER-2",
        category: "integer_range",
        discipline: VectorDiscipline::UnsignedInteger,
        input: "9007199254740991",
        outcome: accept("9007199254740991"),
        note: "2^53-1 is the largest exact JSON integer",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-INTEGER-3",
        category: "integer_range",
        discipline: VectorDiscipline::UnsignedInteger,
        input: "9007199254740992",
        outcome: VectorOutcome::Reject,
        note: "2^53 and above are not exact JSON integers; use a decimal string",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-INTEGER-4",
        category: "integer_range",
        discipline: VectorDiscipline::UnsignedInteger,
        input: "-1",
        outcome: VectorOutcome::Reject,
        note: "unsigned counts and lengths are never negative",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-INTEGER-5",
        category: "integer_range",
        discipline: VectorDiscipline::SignedInteger,
        input: "-9007199254740991",
        outcome: accept("-9007199254740991"),
        note: "signed facts accept the exact negative JSON integer boundary",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-INTEGER-6",
        category: "integer_range",
        discipline: VectorDiscipline::UnsignedInteger,
        input: "1.5",
        outcome: VectorOutcome::Reject,
        note: "counts and integer facts are never fractional",
        expected_digest: None,
    },
    // Decimal strings carry out-of-range exact values.
    RepresentationVector {
        id: "RV-DECIMAL-1",
        category: "decimal_string",
        discipline: VectorDiscipline::UnsignedInteger,
        input: "\"9007199254740992\"",
        outcome: accept("\"9007199254740992\""),
        note: "an unsigned exact value beyond 2^53-1 is a decimal string",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-2",
        category: "decimal_string",
        discipline: VectorDiscipline::SignedInteger,
        input: "\"-9007199254740992\"",
        outcome: accept("\"-9007199254740992\""),
        note: "a signed exact value beyond 2^53-1 is a signed decimal string",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-3",
        category: "decimal_string",
        discipline: VectorDiscipline::UnsignedInteger,
        input: "\"01\"",
        outcome: VectorOutcome::Reject,
        note: "decimal strings forbid leading zeros",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-4",
        category: "decimal_string",
        discipline: VectorDiscipline::UnsignedInteger,
        input: "\"+1\"",
        outcome: VectorOutcome::Reject,
        note: "decimal strings forbid an explicit plus sign",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-5",
        category: "decimal_string",
        discipline: VectorDiscipline::UnsignedInteger,
        input: "\"5\"",
        outcome: VectorOutcome::Reject,
        note: "an in-range value is never encoded as a decimal string",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-6",
        category: "decimal_string",
        discipline: VectorDiscipline::SignedInteger,
        input: "\"-5\"",
        outcome: VectorOutcome::Reject,
        note: "an in-range signed value is never encoded as a decimal string",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-7",
        category: "decimal_string",
        discipline: VectorDiscipline::UnsignedInteger,
        input: "\"18446744073709551616\"",
        outcome: VectorOutcome::Reject,
        note: "an unsigned decimal string never exceeds the u64 domain",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-8",
        category: "decimal_string",
        discipline: VectorDiscipline::SignedInteger,
        input: "\"-9223372036854775809\"",
        outcome: VectorOutcome::Reject,
        note: "a signed decimal string never exceeds the i64 domain",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-9",
        category: "decimal_string",
        discipline: VectorDiscipline::UnsignedInteger,
        input: "\"18446744073709551615\"",
        outcome: accept("\"18446744073709551615\""),
        note: "the unsigned u64 endpoint round-trips as a decimal string",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DECIMAL-10",
        category: "decimal_string",
        discipline: VectorDiscipline::SignedInteger,
        input: "\"-9223372036854775808\"",
        outcome: accept("\"-9223372036854775808\""),
        note: "the signed i64 minimum endpoint round-trips as a decimal string",
        expected_digest: None,
    },
    // Timestamps: canonical UTC with AutoSi subseconds (0/3/6/9 digits).
    RepresentationVector {
        id: "RV-TIMESTAMP-1",
        category: "timestamp",
        discipline: VectorDiscipline::Timestamp,
        input: "2026-01-02T03:04:05Z",
        outcome: accept("2026-01-02T03:04:05Z"),
        note: "whole seconds carry no fractional component",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-2",
        category: "timestamp",
        discipline: VectorDiscipline::Timestamp,
        input: "2026-01-02T03:04:05.100Z",
        outcome: accept("2026-01-02T03:04:05.100Z"),
        note: "millisecond values use exactly three digits (AutoSi)",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-3",
        category: "timestamp",
        discipline: VectorDiscipline::Timestamp,
        input: "2026-01-02T03:04:05.1Z",
        outcome: VectorOutcome::Reject,
        note: "a shortened fraction is not canonical; AutoSi emits three digits",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-4",
        category: "timestamp",
        discipline: VectorDiscipline::Timestamp,
        input: "2026-01-02T03:04:05.000Z",
        outcome: VectorOutcome::Reject,
        note: "a zero fraction is not canonical; omit the fraction instead",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-5",
        category: "timestamp",
        discipline: VectorDiscipline::Timestamp,
        input: "2026-01-02T03:04:05.000001Z",
        outcome: accept("2026-01-02T03:04:05.000001Z"),
        note: "microsecond values use exactly six digits (AutoSi)",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-6",
        category: "timestamp",
        discipline: VectorDiscipline::Timestamp,
        input: "2026-01-02T03:04:05.000000001Z",
        outcome: accept("2026-01-02T03:04:05.000000001Z"),
        note: "nanosecond values use exactly nine digits (AutoSi)",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-7",
        category: "timestamp",
        discipline: VectorDiscipline::Timestamp,
        input: "2026-01-02T03:04:05+00:00",
        outcome: VectorOutcome::Reject,
        note: "offsets are rejected; the canonical form carries a literal Z",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-8",
        category: "timestamp",
        discipline: VectorDiscipline::Timestamp,
        input: "2026-01-02T03:04:05",
        outcome: VectorOutcome::Reject,
        note: "a missing UTC designator is rejected",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-9",
        category: "timestamp",
        discipline: VectorDiscipline::Timestamp,
        input: "2016-12-31T23:59:60Z",
        outcome: VectorOutcome::Reject,
        note: "leap-second notation is rejected by the v2 UTC boundary",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMESTAMP-10",
        category: "timestamp",
        discipline: VectorDiscipline::Timestamp,
        input: "0000-01-01T00:00:00Z",
        outcome: VectorOutcome::Reject,
        note: "the v2 UTC boundary rejects the unsupported year zero",
        expected_digest: None,
    },
    // Absence.
    RepresentationVector {
        id: "RV-ABSENCE-1",
        category: "absence",
        discipline: VectorDiscipline::Absence,
        input: "{}",
        outcome: accept("{}"),
        note: "an absent optional member is omitted and is not success",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-ABSENCE-2",
        category: "absence",
        discipline: VectorDiscipline::Absence,
        input: "{\"optional\":null}",
        outcome: VectorOutcome::Reject,
        note: "an optional member is never encoded as explicit null",
        expected_digest: None,
    },
    // Sets and sequences.
    RepresentationVector {
        id: "RV-SET-1",
        category: "set",
        discipline: VectorDiscipline::SemanticRule,
        input: "[\"b\",\"a\"]",
        outcome: accept("[\"a\",\"b\"]"),
        note: "set members are canonically ordered",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-SET-2",
        category: "set",
        discipline: VectorDiscipline::SemanticRule,
        input: "[\"a\",\"a\"]",
        outcome: VectorOutcome::Reject,
        note: "duplicate set members are rejected, not collapsed",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-SEQUENCE-1",
        category: "sequence",
        discipline: VectorDiscipline::SemanticRule,
        input: "[\"b\",\"a\"]",
        outcome: accept("[\"b\",\"a\"]"),
        note: "an ordered sequence preserves declared order",
        expected_digest: None,
    },
    // Text.
    RepresentationVector {
        id: "RV-TEXT-1",
        category: "text_unicode",
        discipline: VectorDiscipline::SemanticRule,
        input: "\"cafe\\u0301\"",
        outcome: VectorOutcome::Reject,
        note: "authored text must be NFC; it is rejected, never normalized",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TEXT-2",
        category: "text_unicode",
        discipline: VectorDiscipline::SemanticRule,
        input: "\"caf\\u00e9\"",
        outcome: accept("\"caf\\u00e9\""),
        note: "NFC authored text is preserved and hashed as exact UTF-8 bytes",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TEXT-3",
        category: "text_unicode",
        discipline: VectorDiscipline::SemanticRule,
        input: "opaque attachment bytes",
        outcome: accept("opaque attachment bytes"),
        note: "opaque Unicode-bearing/attachment payload bytes are not NFC-validated",
        expected_digest: None,
    },
    // Digest domains.
    RepresentationVector {
        id: "RV-DIGEST-1",
        category: "digest_domain",
        discipline: VectorDiscipline::Digest,
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
        discipline: VectorDiscipline::Digest,
        input: "sha256:E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
        outcome: VectorOutcome::Reject,
        note: "uppercase hex is rejected",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-DIGEST-3",
        category: "digest_domain",
        discipline: VectorDiscipline::Digest,
        input: "sha256:00",
        outcome: VectorOutcome::Reject,
        note: "a short digest is rejected",
        expected_digest: None,
    },
    // Rule vectors (executed by their owning semantic steps).
    RepresentationVector {
        id: "RV-CONJUNCTION-1",
        category: "conjunction",
        discipline: VectorDiscipline::SemanticRule,
        input: "required members: one missing, others pass",
        outcome: accept("not satisfied"),
        note: "required selections compose with AND; passes cannot hide a missing member",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-ALTERNATIVE-1",
        category: "alternatives",
        discipline: VectorDiscipline::SemanticRule,
        input: "observed pass would select an unauthorized alternative",
        outcome: accept("not satisfied"),
        note: "no OR substitution without an explicit protected authorized group",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-ALTERNATIVE-2",
        category: "alternatives",
        discipline: VectorDiscipline::SemanticRule,
        input: "one authorized alternative in an explicit protected group",
        outcome: accept("group composes with AND"),
        note: "an authorized alternative composes with the required conjunction",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-FAILPASS-1",
        category: "distinct_fail_pass",
        discipline: VectorDiscipline::SemanticRule,
        input: "distinct required pass and fail cases",
        outcome: accept("Failed"),
        note: "distinct required fail/pass cases aggregate to Failed",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-RETRY-1",
        category: "retry_unstable",
        discipline: VectorDiscipline::SemanticRule,
        input: "fail then pass retries of one case",
        outcome: accept("Unstable"),
        note: "fail/pass retries of one logical case aggregate to Unstable",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-CONFLICT-1",
        category: "attempt_conflict",
        discipline: VectorDiscipline::SemanticRule,
        input: "contradictory assertions for one immutable attempt",
        outcome: accept("Unknown"),
        note: "same-attempt contradictions aggregate to Unknown conflict",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-TIMEOUT-1",
        category: "timeout_unknown",
        discipline: VectorDiscipline::SemanticRule,
        input: "timed out or cancelled execution",
        outcome: accept("Unknown"),
        note: "timeout/cancellation is Unknown and cannot be waived",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-MISSING-1",
        category: "missing_non_success",
        discipline: VectorDiscipline::SemanticRule,
        input: "skipped, filtered, quarantined or missing selection",
        outcome: accept("non-success"),
        note: "missing selections remain non-success",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-COUNT-1",
        category: "count_unit",
        discipline: VectorDiscipline::SemanticRule,
        input: "one release-record reference against a minimum of two",
        outcome: accept("at most one unit"),
        note: "one reference is one unit; copies and retries add no credit",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-COUNT-2",
        category: "count_unit",
        discipline: VectorDiscipline::SemanticRule,
        input: "several evidence kinds with declared units",
        outcome: accept("per-kind logical unit"),
        note: "the minimum uses the declared logical unit for the kind",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-CAMPAIGN-1",
        category: "campaign_selection",
        discipline: VectorDiscipline::SemanticRule,
        input: "several campaigns with different timestamps",
        outcome: accept("explicit authorized selection"),
        note: "never the latest passing timestamp",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-CAMPAIGN-2",
        category: "campaign_selection",
        discipline: VectorDiscipline::SemanticRule,
        input: "selected campaign plus unrelated superseded history",
        outcome: accept("selected campaign only"),
        note: "unrelated superseded history neither poisons nor substitutes",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-RISK-1",
        category: "fragment_risk",
        discipline: VectorDiscipline::SemanticRule,
        input: "expanded fragment with a stronger fragment/journey risk",
        outcome: accept("stronger risk inherited"),
        note: "absence inherits the stronger risk; an explicit lower value rejects",
        expected_digest: None,
    },
    RepresentationVector {
        id: "RV-RISK-2",
        category: "fragment_risk",
        discipline: VectorDiscipline::SemanticRule,
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
    use eqm_domain::{Sha256Digest, UtcInstantV2};
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
    fn every_vector_declares_a_discipline_consistent_with_its_category() {
        // Exact category-to-discipline mapping. The two integer categories are
        // the only ones that legitimately use both the unsigned and signed
        // controls (an unsigned control may still test a negative input that it
        // must reject); every other category names exactly one control.
        for vector in REPRESENTATION_VECTORS {
            let allowed: &[VectorDiscipline] = match vector.category {
                "integer_range" | "decimal_string" => &[
                    VectorDiscipline::UnsignedInteger,
                    VectorDiscipline::SignedInteger,
                ],
                "timestamp" => &[VectorDiscipline::Timestamp],
                "absence" => &[VectorDiscipline::Absence],
                "digest_domain" => &[VectorDiscipline::Digest],
                "namespace"
                | "set"
                | "sequence"
                | "text_unicode"
                | "conjunction"
                | "alternatives"
                | "distinct_fail_pass"
                | "retry_unstable"
                | "attempt_conflict"
                | "timeout_unknown"
                | "missing_non_success"
                | "count_unit"
                | "campaign_selection"
                | "fragment_risk" => &[VectorDiscipline::SemanticRule],
                _ => &[],
            };
            assert!(
                !allowed.is_empty(),
                "{}: category `{}` has no exact discipline mapping",
                vector.id,
                vector.category
            );
            assert!(
                allowed.contains(&vector.discipline),
                "{}: discipline {:?} is not the exact control for category `{}`",
                vector.id,
                vector.discipline,
                vector.category
            );
        }
        // Every required category must be reachable by this exact mapping.
        for category in REQUIRED_VECTOR_CATEGORIES {
            let mapped = REPRESENTATION_VECTORS
                .iter()
                .any(|vector| vector.category == *category);
            assert!(mapped, "required category `{category}` has no vector");
        }
    }

    #[test]
    fn timestamp_vectors_match_the_canonical_v2_utc_rule() {
        let expected = vectors_for("timestamp").count();
        let mut checked = 0;
        for vector in vectors_for("timestamp") {
            checked += 1;
            assert_eq!(
                vector.discipline,
                VectorDiscipline::Timestamp,
                "{}",
                vector.id
            );
            let parsed = UtcInstantV2::from_str(vector.input);
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
        assert_eq!(checked, expected, "not every timestamp vector executed");
    }

    #[test]
    fn digest_vectors_match_the_canonical_digest_rule() {
        let expected = vectors_for("digest_domain").count();
        let mut checked = 0;
        for vector in vectors_for("digest_domain") {
            checked += 1;
            assert_eq!(vector.discipline, VectorDiscipline::Digest, "{}", vector.id);
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
        assert_eq!(checked, expected, "not every digest vector executed");
    }
}
