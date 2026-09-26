//! Explicit, separately-authorized acquisition and verification ports.
//!
//! Execution and verification are privileged capabilities. They are never
//! reachable from a read or evaluation path, have no default implementation,
//! and return explicit errors when the capability is missing or the request is
//! not authorized.

use eqm_domain::Sha256Digest;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Explicit request to execute one already-approved acquisition plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcquisitionRequest {
    /// Exact digest of the approved plan this request executes.
    pub plan_digest: Sha256Digest,
}

impl AcquisitionRequest {
    /// Creates an explicit acquisition request bound to a plan digest.
    #[must_use]
    pub const fn new(plan_digest: Sha256Digest) -> Self {
        Self { plan_digest }
    }
}

/// Terminal classification of one acquisition attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcquisitionOutcome {
    /// The plan completed successfully.
    Completed,
    /// The plan failed.
    Failed,
    /// The plan was cancelled before completion.
    Cancelled,
}

/// Receipt binding a terminal outcome to its exact plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcquisitionReceipt {
    /// Exact digest of the plan that was executed.
    pub plan_digest: Sha256Digest,
    /// Terminal outcome of the attempt.
    pub outcome: AcquisitionOutcome,
}

impl AcquisitionReceipt {
    /// Returns whether the receipt records successful completion.
    #[must_use]
    pub const fn is_completed(self) -> bool {
        matches!(self.outcome, AcquisitionOutcome::Completed)
    }
}

/// Acquisition could not be performed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcquisitionError {
    /// No executor was configured.
    MissingExecutor,
    /// The request was not authorized for execution.
    Unauthorized,
    /// The executor failed before producing a receipt.
    Failed,
    /// A receipt did not match the executed plan.
    ReceiptMismatch,
}

impl Display for AcquisitionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingExecutor => "acquisition executor is not configured",
            Self::Unauthorized => "acquisition is not authorized",
            Self::Failed => "acquisition execution failed",
            Self::ReceiptMismatch => "acquisition receipt does not match the executed plan",
        };
        formatter.write_str(message)
    }
}

impl Error for AcquisitionError {}

/// Narrow executor for separately-authorized acquisition plans.
///
/// There is no default implementation and no permissive fallback. The executor
/// only runs an already-approved plan and never decides conformance.
pub trait Executor {
    /// Executes one approved plan and returns a terminal receipt.
    fn execute(&self, request: &AcquisitionRequest)
    -> Result<AcquisitionReceipt, AcquisitionError>;
}

/// Explicit request to verify one immutable evidence subject.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VerificationRequest {
    /// Exact digest of the subject bytes to verify.
    pub subject_digest: Sha256Digest,
}

impl VerificationRequest {
    /// Creates an explicit verification request.
    #[must_use]
    pub const fn new(subject_digest: Sha256Digest) -> Self {
        Self { subject_digest }
    }
}

/// Structured, non-authorizing verification report.
///
/// This is not a trust decision and never raises effective trust by itself;
/// the protected verification step consumes it under explicit authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VerificationReport {
    /// Exact digest of the verified subject.
    pub subject_digest: Sha256Digest,
    /// Whether every checked input verified.
    pub verified: bool,
}

/// Verification could not be performed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationError {
    /// No verifier was configured.
    MissingVerifier,
    /// Verification material was unavailable or ineligible.
    Unavailable,
    /// Verification rejected the subject.
    Rejected,
}

impl Display for VerificationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingVerifier => "evidence verifier is not configured",
            Self::Unavailable => "verification material is unavailable",
            Self::Rejected => "verification rejected the subject",
        };
        formatter.write_str(message)
    }
}

impl Error for VerificationError {}

/// Narrow evidence verifier used only by privileged verification paths.
pub trait EvidenceVerifier {
    /// Verifies one immutable subject under explicit authority.
    fn verify(
        &self,
        request: &VerificationRequest,
    ) -> Result<VerificationReport, VerificationError>;
}
