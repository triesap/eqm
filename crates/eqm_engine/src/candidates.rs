//! Pure, bounded delivery normalization. Structural admission grants no trust.

use crate::{
    CoverageExpectation, CoverageReport, CoverageStatus, EvidenceCandidate,
    evaluate_evidence_coverage,
};
use eqm_domain::{
    EvidenceLineage, EvidenceResult, EvidenceSpecId, EvidenceSubject, RawObjectDigest,
    SelectorText, SemanticEvidenceDigest,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Maximum input deliveries, including repeated transport; overflow is explicit.
pub const MAX_CANDIDATE_DELIVERIES: usize = 10_000;

/// Complete immutable attempt coordinate, without a verification conclusion.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ImmutableEvidenceAttempt {
    subject: EvidenceSubject,
    lineage: EvidenceLineage,
}

impl ImmutableEvidenceAttempt {
    /// Returns the exact repository/build/artifact/configuration/scope subject.
    #[must_use]
    pub const fn subject(&self) -> &EvidenceSubject {
        &self.subject
    }
    /// Returns all five explicit execution lineage identities.
    #[must_use]
    pub const fn lineage(&self) -> &EvidenceLineage {
        &self.lineage
    }
}

/// One admitted delivery's input data. These fields themselves confer no trust.
#[derive(Clone, Copy, Debug)]
pub struct EvidenceNormalizationCandidate<'a> {
    /// The admitted immutable domain record.
    pub result: &'a EvidenceResult,
    /// Complete checked execution lineage.
    pub lineage: &'a EvidenceLineage,
    /// Recomputed semantic assertion identity.
    pub semantic_identity: SemanticEvidenceDigest,
    /// Checked identity of exact original bytes.
    pub raw_identity: RawObjectDigest,
    /// Explicit owning local evidence mapping.
    pub evidence_spec_id: &'a EvidenceSpecId,
    /// Explicit delivery/provenance record reference, never inferred from time.
    pub provenance: &'a SelectorText,
}

/// Retained exact raw, mapping and provenance references for one delivery.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EvidenceDeliveryReference {
    raw_identity: RawObjectDigest,
    evidence_spec_id: EvidenceSpecId,
    provenance: SelectorText,
}

impl EvidenceDeliveryReference {
    /// Returns the exact original-byte identity.
    #[must_use]
    pub const fn raw_identity(&self) -> RawObjectDigest {
        self.raw_identity
    }
    /// Returns the explicit local mapping reference.
    #[must_use]
    pub const fn evidence_spec_id(&self) -> &EvidenceSpecId {
        &self.evidence_spec_id
    }
    /// Returns the explicit delivery/provenance record reference.
    #[must_use]
    pub const fn provenance(&self) -> &SelectorText {
        &self.provenance
    }
}

/// One semantic assertion retaining every distinct delivery reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedEvidenceAssertion<'a> {
    result: &'a EvidenceResult,
    deliveries: BTreeSet<EvidenceDeliveryReference>,
}

impl<'a> NormalizedEvidenceAssertion<'a> {
    /// Returns the admitted domain record; it is not an effective trust fact.
    #[must_use]
    pub const fn result(&self) -> &'a EvidenceResult {
        self.result
    }
    /// Returns all original raw and provenance references without observation credit.
    #[must_use]
    pub const fn deliveries(&self) -> &BTreeSet<EvidenceDeliveryReference> {
        &self.deliveries
    }
}

/// All assertions for one attempt. Contradictions remain visible, never overwritten.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedEvidenceAttempt<'a> {
    assertions: BTreeMap<SemanticEvidenceDigest, NormalizedEvidenceAssertion<'a>>,
}

impl<'a> NormalizedEvidenceAttempt<'a> {
    /// Returns every distinct semantic assertion and its raw/provenance references.
    #[must_use]
    pub const fn assertions(
        &self,
    ) -> &BTreeMap<SemanticEvidenceDigest, NormalizedEvidenceAssertion<'a>> {
        &self.assertions
    }
    /// Returns whether this input contains contradictory assertions for one attempt.
    /// Authentication and exact-coordinate eligibility are separate boundaries.
    #[must_use]
    pub fn has_conflict(&self) -> bool {
        self.assertions.len() > 1
    }
}

/// Deterministically ordered immutable candidate data; no counts or trust are minted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedEvidenceSet<'a> {
    attempts: BTreeMap<ImmutableEvidenceAttempt, NormalizedEvidenceAttempt<'a>>,
}

impl<'a> NormalizedEvidenceSet<'a> {
    /// Returns every exact subject/lineage group. Group count is not observation count.
    #[must_use]
    pub const fn attempts(
        &self,
    ) -> &BTreeMap<ImmutableEvidenceAttempt, NormalizedEvidenceAttempt<'a>> {
        &self.attempts
    }
}

/// Input data cannot be safely normalized.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CandidateNormalizationError {
    /// Complete input exceeds its bound; no truncated result is returned.
    LimitExceeded,
    /// A domain result identity differs from its declared checked semantic identity.
    IdentityMismatch,
    /// Different domain records claim the same semantic identity.
    ConflictingSemanticIdentity,
}

impl Display for CandidateNormalizationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::LimitExceeded => "candidate delivery limit exceeded",
            Self::IdentityMismatch => "candidate semantic identity mismatch",
            Self::ConflictingSemanticIdentity => {
                "candidate semantic identity has conflicting records"
            }
        })
    }
}
impl Error for CandidateNormalizationError {}

/// Normalizes complete admitted input data without selecting campaigns or verifying trust.
pub fn normalize_evidence_candidates<'a>(
    candidates: &[EvidenceNormalizationCandidate<'a>],
) -> Result<NormalizedEvidenceSet<'a>, CandidateNormalizationError> {
    if candidates.len() > MAX_CANDIDATE_DELIVERIES {
        return Err(CandidateNormalizationError::LimitExceeded);
    }
    let mut attempts: BTreeMap<ImmutableEvidenceAttempt, NormalizedEvidenceAttempt<'a>> =
        BTreeMap::new();
    let mut semantics = BTreeMap::new();
    for candidate in candidates {
        if candidate.result.id() != candidate.semantic_identity.as_digest() {
            return Err(CandidateNormalizationError::IdentityMismatch);
        }
        if let Some(previous) = semantics.insert(candidate.semantic_identity, candidate.result)
            && previous != candidate.result
        {
            return Err(CandidateNormalizationError::ConflictingSemanticIdentity);
        }
        let key = ImmutableEvidenceAttempt {
            subject: candidate.result.subject().clone(),
            lineage: candidate.lineage.clone(),
        };
        let attempt = attempts
            .entry(key)
            .or_insert_with(|| NormalizedEvidenceAttempt {
                assertions: BTreeMap::new(),
            });
        let assertion = attempt
            .assertions
            .entry(candidate.semantic_identity)
            .or_insert_with(|| NormalizedEvidenceAssertion {
                result: candidate.result,
                deliveries: BTreeSet::new(),
            });
        assertion.deliveries.insert(EvidenceDeliveryReference {
            raw_identity: candidate.raw_identity,
            evidence_spec_id: candidate.evidence_spec_id.clone(),
            provenance: candidate.provenance.clone(),
        });
    }
    Ok(NormalizedEvidenceSet { attempts })
}

/// Existing structural coverage plus relevant immutable-attempt contradictions.
/// Neither coverage nor normalization authenticates inputs or grants conformance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedCoverageReport {
    /// Existing exact structural coordinate matching and rejection diagnostics.
    pub coverage: CoverageReport,
    /// Attempts having more than one compatible distinct semantic assertion.
    pub conflicting_attempts: BTreeSet<ImmutableEvidenceAttempt>,
}

/// Matches first, then exposes conflicts only among compatible assertions.
#[must_use]
pub fn evaluate_normalized_evidence_coverage(
    expected: &CoverageExpectation,
    normalized: &NormalizedEvidenceSet<'_>,
) -> NormalizedCoverageReport {
    let mut candidates = Vec::new();
    let mut conflicting_attempts = BTreeSet::new();
    let mut conflicting_results = BTreeSet::new();
    for (key, attempt) in &normalized.attempts {
        let mut matching = BTreeSet::new();
        for (identity, assertion) in &attempt.assertions {
            for delivery in &assertion.deliveries {
                let candidate = EvidenceCandidate {
                    evidence_spec_id: &delivery.evidence_spec_id,
                    result: assertion.result,
                };
                if super::coverage::mismatches(expected, &candidate).is_empty() {
                    matching.insert(*identity);
                }
                candidates.push(candidate);
            }
        }
        if matching.len() > 1 {
            conflicting_attempts.insert(key.clone());
            conflicting_results.extend(matching.into_iter().map(SemanticEvidenceDigest::as_digest));
        }
    }
    let mut coverage = evaluate_evidence_coverage(expected, &candidates);
    for identity in conflicting_results {
        coverage.covered.remove(&identity);
    }
    if !conflicting_attempts.is_empty() {
        coverage.status = CoverageStatus::Unknown;
    }
    NormalizedCoverageReport {
        coverage,
        conflicting_attempts,
    }
}
