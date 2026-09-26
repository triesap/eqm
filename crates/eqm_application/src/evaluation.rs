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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceReadRequest {
    /// Maximum records to return; zero is rejected.
    pub limit: usize,
}

impl EvidenceReadRequest {
    /// Creates a bounded evidence read request.
    pub fn new(limit: usize) -> Result<Self, EvidenceReadError> {
        if limit == 0 || limit > MAX_EVIDENCE_READ {
            return Err(EvidenceReadError::InvalidLimit);
        }
        Ok(Self { limit })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_read_limits_are_bounded() {
        assert!(EvidenceReadRequest::new(0).is_err());
        assert!(EvidenceReadRequest::new(1).is_ok());
        assert!(EvidenceReadRequest::new(MAX_EVIDENCE_READ).is_ok());
        assert!(EvidenceReadRequest::new(MAX_EVIDENCE_READ + 1).is_err());
    }
}
