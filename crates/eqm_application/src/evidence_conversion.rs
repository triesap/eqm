//! Pure conversion of exact evidence bytes through the shared v2 admission boundary.

use eqm_domain::{EvidenceExecution, EvidenceLineage};
use eqm_protocol::{
    AdmittedEvidenceV2, CheckedEvidenceResultV2, EvidenceAdmissionContext, EvidenceAdmissionError,
    V2DtoError, admit_evidence,
};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Strict evidence decoding or checked relationship admission failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceConversionError {
    /// The exact bytes failed the explicit bounded v2 reader.
    Decode(V2DtoError),
    /// A parsed record failed the shared DTO-to-domain admission contract.
    Admission(EvidenceAdmissionError),
}

impl Display for EvidenceConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(error) => write!(formatter, "evidence decoding failed: {error}"),
            Self::Admission(error) => write!(formatter, "evidence admission failed: {error}"),
        }
    }
}

impl Error for EvidenceConversionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Decode(error) => Some(error),
            Self::Admission(error) => Some(error),
        }
    }
}

/// Converts exact v2 bytes against explicit independent expectations and lineage.
///
/// All supported payloads use the protocol's single checked converter. The
/// returned record retains the original bytes; it carries no independent trust
/// conclusion. This pure operation neither reads a store nor verifies or executes
/// evidence. Unsupported versions are rejected without a legacy fallback.
pub fn admit_evidence_bytes(
    bytes: &[u8],
    lineage: &EvidenceLineage,
    execution: &EvidenceExecution,
    context: &EvidenceAdmissionContext,
) -> Result<AdmittedEvidenceV2, EvidenceConversionError> {
    let checked =
        CheckedEvidenceResultV2::from_bytes(bytes).map_err(EvidenceConversionError::Decode)?;
    admit_evidence(&checked, lineage, execution, context)
        .map_err(EvidenceConversionError::Admission)
}
