//! Read-only evidence access and pure evaluation requests.
//!
//! Reading evidence is a bounded read; it never executes a runner and never
//! fetches network content. Evaluation is pure over already-read evidence and
//! an explicitly supplied evaluation instant.

use eqm_domain::{EvidenceResult, UtcInstant};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// The maximum number of evidence records one read may return.
pub const MAX_EVIDENCE_READ: usize = 10_000;

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
}
