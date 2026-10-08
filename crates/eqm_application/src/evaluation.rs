//! Read-only evidence access and pure evaluation requests.
//!
//! Reading evidence is a bounded read; it never executes a runner and never
//! fetches network content. Evaluation is pure over already-read evidence and
//! an explicitly supplied evaluation instant.

use eqm_domain::{EvidenceResult, EvidenceSpecId, SelectorText, UtcInstant};
use eqm_engine::{
    CandidateNormalizationError, CompleteCoverageError, CompleteCoverageExpectation,
    CompleteCoverageReport, CompleteEvidenceCandidate, CoverageSubjectManifest,
    EvidenceNormalizationCandidate, MAX_CANDIDATE_DELIVERIES, NormalizedEvidenceSet, ObligationKey,
    evaluate_complete_evidence_coverage, normalize_evidence_candidates,
};
use eqm_protocol::AdmittedEvidenceV2;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// The maximum number of evidence records one read may return.
pub const MAX_EVIDENCE_READ: usize = 10_000;

/// One actual checked admission and explicit mapping/provenance references.
/// Structural admission and normalization confer no authentication or trust.
#[derive(Clone, Copy, Debug)]
pub struct AdmittedEvidenceDelivery<'a> {
    /// Opaque result of the shared checked conversion boundary.
    pub admitted: &'a AdmittedEvidenceV2,
    /// Explicit local evidence mapping; not inferred from a filename.
    pub evidence_spec_id: &'a EvidenceSpecId,
    /// Explicit immutable delivery/provenance reference.
    pub provenance: &'a SelectorText,
}

/// Actual checked admission with explicit complete context claims.
/// Protected consumers independently bind these claims to their frozen authority.
#[derive(Clone, Copy, Debug)]
pub struct CompleteEvidenceDelivery<'a> {
    /// Shared actual S059 admission and original mapping/provenance references.
    pub delivery: AdmittedEvidenceDelivery<'a>,
    /// Explicit candidate context; absent remains rejected.
    pub coordinate: Option<&'a ObligationKey>,
    /// Explicit claimed pre-result semantic identity, never independent trust.
    pub identity: Option<eqm_domain::ObligationDigest>,
    /// Explicit complete role-bearing subject claims for grouped scopes.
    pub subjects: Option<&'a CoverageSubjectManifest>,
}

/// Matches actual opaque admissions against independent frozen expectations.
/// Pure matching creates no verification, count, conformance or release authority.
pub fn match_admitted_evidence<'a>(
    expected: &CompleteCoverageExpectation,
    deliveries: &[CompleteEvidenceDelivery<'a>],
) -> Result<CompleteCoverageReport<'a>, CompleteCoverageError> {
    if deliveries.len() > MAX_CANDIDATE_DELIVERIES {
        return Err(CompleteCoverageError::Normalization(
            CandidateNormalizationError::LimitExceeded,
        ));
    }
    evaluate_complete_evidence_coverage(expected, &complete_candidates(deliveries))
}

fn complete_candidates<'a>(
    deliveries: &[CompleteEvidenceDelivery<'a>],
) -> Vec<CompleteEvidenceCandidate<'a>> {
    deliveries
        .iter()
        .map(|claim| CompleteEvidenceCandidate {
            delivery: EvidenceNormalizationCandidate {
                result: claim.delivery.admitted.result(),
                lineage: claim.delivery.admitted.lineage(),
                semantic_identity: claim.delivery.admitted.semantic_identity(),
                raw_identity: claim.delivery.admitted.raw_identity(),
                evidence_spec_id: claim.delivery.evidence_spec_id,
                provenance: claim.delivery.provenance,
            },
            coordinate: claim.coordinate,
            identity: claim.identity,
            subjects: claim.subjects,
        })
        .collect()
}

/// Derives distinct structural selections from actual opaque admissions. Counts
/// and prerequisite labels are never successful verification or conformance.
pub fn collect_admitted_observations<'a>(
    expected: &[CompleteCoverageExpectation],
    deliveries: &[CompleteEvidenceDelivery<'a>],
) -> Result<eqm_engine::LogicalObservationReport<'a>, eqm_engine::LogicalObservationError> {
    if deliveries.len() > MAX_CANDIDATE_DELIVERIES {
        return Err(eqm_engine::LogicalObservationError::Matching(
            CompleteCoverageError::Normalization(CandidateNormalizationError::LimitExceeded),
        ));
    }
    eqm_engine::collect_logical_observations(expected, &complete_candidates(deliveries))
}

/// Normalizes admitted candidate data, retaining every raw and provenance reference.
///
/// This pure boundary invokes no reader, runner, verifier, clock or store. It
/// mints no observation count or authentication. Complete-coordinate eligibility,
/// authorized campaign selection and protected verification remain distinct.
pub fn normalize_admitted_evidence<'a>(
    deliveries: &[AdmittedEvidenceDelivery<'a>],
) -> Result<NormalizedEvidenceSet<'a>, CandidateNormalizationError> {
    if deliveries.len() > MAX_CANDIDATE_DELIVERIES {
        return Err(CandidateNormalizationError::LimitExceeded);
    }
    let candidates = deliveries
        .iter()
        .map(|delivery| EvidenceNormalizationCandidate {
            result: delivery.admitted.result(),
            lineage: delivery.admitted.lineage(),
            semantic_identity: delivery.admitted.semantic_identity(),
            raw_identity: delivery.admitted.raw_identity(),
            evidence_spec_id: delivery.evidence_spec_id,
            provenance: delivery.provenance,
        })
        .collect::<Vec<_>>();
    normalize_evidence_candidates(&candidates)
}

/// Explicit bounded request to read already-acquired evidence.
///
/// The bound is private validated state: callers construct through [`new`],
/// which rejects zero and over-bound limits, so no public field can bypass it.
///
/// [`new`]: EvidenceReadRequest::new
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceReadRequest {
    limit: usize,
}

impl EvidenceReadRequest {
    /// Creates a bounded evidence read request.
    pub fn new(limit: usize) -> Result<Self, EvidenceReadError> {
        if limit == 0 || limit > MAX_EVIDENCE_READ {
            return Err(EvidenceReadError::InvalidLimit);
        }
        Ok(Self { limit })
    }

    /// Returns the validated read bound.
    #[must_use]
    pub const fn limit(&self) -> usize {
        self.limit
    }
}

/// Evidence could not be read.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceReadError {
    /// No evidence reader was configured.
    MissingReader,
    /// The evidence store was unavailable.
    Unavailable,
    /// The requested limit was zero or exceeded the bound.
    InvalidLimit,
    /// The read exceeded its declared bound.
    LimitExceeded,
}

impl Display for EvidenceReadError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingReader => "evidence reader is not configured",
            Self::Unavailable => "evidence is unavailable",
            Self::InvalidLimit => "evidence read limit is invalid",
            Self::LimitExceeded => "evidence read exceeded its declared bound",
        };
        formatter.write_str(message)
    }
}

impl Error for EvidenceReadError {}

/// Narrow bounded reader of already-acquired immutable evidence.
///
/// A reader never launches a runner and never fetches network content; it only
/// returns stored, validated records.
pub trait EvidenceReader {
    /// Reads at most `request.limit` stored evidence records.
    fn read_evidence(
        &self,
        request: &EvidenceReadRequest,
    ) -> Result<Vec<EvidenceResult>, EvidenceReadError>;
}

/// One pure evaluation request over already-frozen inputs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvaluationRequest {
    /// Explicit evaluation instant; evaluation never reads the wall clock.
    pub evaluation_time: UtcInstant,
}

impl EvaluationRequest {
    /// Creates a pure evaluation request with an explicit instant.
    #[must_use]
    pub const fn new(evaluation_time: UtcInstant) -> Self {
        Self { evaluation_time }
    }
}

/// Reads bounded evidence through the read-facing view.
///
/// The view exposes only authority and evidence access, so this path cannot
/// launch a runner, verify evidence, or fetch network content.
pub fn read_evidence(
    view: &crate::ReadCapabilities<'_>,
    request: &EvidenceReadRequest,
) -> Result<Vec<EvidenceResult>, EvidenceReadError> {
    let records = view.evidence()?.read_evidence(request)?;
    // RF08: an application read is bounded by its declared limit, so an
    // over-returning provider cannot exceed the advertised result bound.
    if !within_read_limit(records.len(), request.limit()) {
        return Err(EvidenceReadError::LimitExceeded);
    }
    Ok(records)
}

/// Returns whether a provider result of `count` records is within `limit`.
///
/// The application bound is a length check; a concrete store additionally owns
/// its allocation bound while reading (DEV-002 decision 8).
fn within_read_limit(count: usize, limit: usize) -> bool {
    count <= limit
}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{
        AttemptOutcome, DimensionId, EvidenceKind, EvidencePayload, EvidenceResult,
        EvidenceScopeSubject, EvidenceSubject, Extensions, Facet, FullRequirementId, ProfileId,
        ProfileSelection, Revision, SelectorText, Sha256Digest, SymbolicValueId, TargetId,
        TrustLevel, UnitId,
    };
    use std::cell::Cell;

    #[test]
    fn evidence_read_limits_are_bounded() {
        assert!(EvidenceReadRequest::new(0).is_err());
        assert_eq!(EvidenceReadRequest::new(1).map(|r| r.limit()), Ok(1));
        assert_eq!(
            EvidenceReadRequest::new(MAX_EVIDENCE_READ).map(|r| r.limit()),
            Ok(MAX_EVIDENCE_READ)
        );
        assert!(EvidenceReadRequest::new(MAX_EVIDENCE_READ + 1).is_err());
    }

    #[test]
    fn provider_over_return_exceeds_the_declared_bound() {
        // RF08: exact limit succeeds; one past the limit fails without truncation.
        assert!(within_read_limit(0, 1));
        assert!(within_read_limit(1, 1));
        assert!(!within_read_limit(2, 1));
        assert!(within_read_limit(MAX_EVIDENCE_READ, MAX_EVIDENCE_READ));
        assert!(!within_read_limit(MAX_EVIDENCE_READ + 1, MAX_EVIDENCE_READ));
    }

    fn digest(value: &[u8]) -> Sha256Digest {
        Sha256Digest::hash_content(value)
    }

    fn fixture_result() -> Result<EvidenceResult, Box<dyn Error>> {
        let id = digest(b"result");
        Ok(EvidenceResult::new(
            id,
            EvidenceSubject::new(
                "https://github.com/example/project".parse()?,
                digest(b"repository"),
                EvidenceScopeSubject::Target(TargetId::new("web")?),
                "a".repeat(40).parse()?,
                None,
                None,
                digest(b"target-config"),
            ),
            TargetId::new("web")?,
            UnitId::new("account.create.signup.identifier")?,
            vec![FullRequirementId::new(
                "account.create.signup.identifier#email_default",
            )?],
            vec![Facet::Behavior],
            EvidenceKind::ManualReview,
            digest(b"spec"),
            digest(b"contract"),
            digest(b"binding"),
            digest(b"policy"),
            None,
            None,
            None,
            None,
            vec![ProfileSelection::new(
                ProfileId::new("audience.default")?,
                Revision::new(1)?,
                vec![(DimensionId::new("region")?, SymbolicValueId::new("us")?)],
            )?],
            "producer://human/review/reviewer-1".parse()?,
            TrustLevel::TrustedCi,
            "2026-08-07T12:00:00Z".parse()?,
            EvidencePayload::manual_review(
                AttemptOutcome::Passed,
                "owner://team/reviewers".parse()?,
                Some(SelectorText::new("Approved")?),
            )?,
            Vec::new(),
            id,
            Extensions::default(),
        )?)
    }

    /// A read-only fake provider that can under-, exactly-, or over-return and
    /// can propagate a store error. It records how often it was invoked.
    #[derive(Default)]
    struct FakeReader {
        records: Vec<EvidenceResult>,
        error: Option<EvidenceReadError>,
        calls: Cell<usize>,
    }

    impl EvidenceReader for FakeReader {
        fn read_evidence(
            &self,
            _request: &EvidenceReadRequest,
        ) -> Result<Vec<EvidenceResult>, EvidenceReadError> {
            self.calls.set(self.calls.get() + 1);
            match self.error {
                Some(error) => Err(error),
                None => Ok(self.records.clone()),
            }
        }
    }

    #[test]
    fn read_evidence_boundary_covers_exact_over_return_error_and_missing()
    -> Result<(), Box<dyn Error>> {
        let record = fixture_result()?;

        // Exact limit succeeds with the real record count and one read call.
        let reader = FakeReader {
            records: vec![record.clone(), record.clone()],
            ..FakeReader::default()
        };
        let view = crate::ReadCapabilities::new().with_evidence(&reader);
        let request = EvidenceReadRequest::new(2)?;
        let records = read_evidence(&view, &request)?;
        assert_eq!(records.len(), 2);
        assert_eq!(reader.calls.get(), 1);

        // One over the declared limit fails with the exact bound error and no
        // silent truncation. The read view has no execution or verification
        // port, so this path cannot launch a runner.
        let reader = FakeReader {
            records: vec![record.clone(), record.clone(), record],
            ..FakeReader::default()
        };
        let view = crate::ReadCapabilities::new().with_evidence(&reader);
        let request = EvidenceReadRequest::new(2)?;
        assert_eq!(
            read_evidence(&view, &request),
            Err(EvidenceReadError::LimitExceeded)
        );
        assert_eq!(reader.calls.get(), 1);

        // A provider error is propagated unchanged, never masked as success.
        let reader = FakeReader {
            error: Some(EvidenceReadError::Unavailable),
            ..FakeReader::default()
        };
        let view = crate::ReadCapabilities::new().with_evidence(&reader);
        assert_eq!(
            read_evidence(&view, &EvidenceReadRequest::new(1)?),
            Err(EvidenceReadError::Unavailable)
        );

        // A missing reader is an explicit error, not an empty success.
        let empty = crate::ReadCapabilities::new();
        assert_eq!(
            read_evidence(&empty, &EvidenceReadRequest::new(1)?),
            Err(EvidenceReadError::MissingReader)
        );
        Ok(())
    }
}
