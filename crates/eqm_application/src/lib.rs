//! Shared effectful use-case orchestration for EquivalenceMatrix.
//!
//! This crate is the single boundary between transports (CLI, MCP, and any
//! future native embedding) and the host-adapting work of loading authority,
//! preparing immutable sessions, acquiring authorized evidence, evaluating a
//! frozen input, and serving stored queries. Transports adapt its results;
//! they never re-derive conformance.
//!
//! # Boundary rules
//!
//! - It may orchestrate `eqm_manifest`, `eqm_protocol`, and `eqm_runner`, but
//!   it never depends on `eqm_cli` or `eqm_mcp`; transport mapping stays in the
//!   transport crates.
//! - `eqm_domain` and `eqm_engine` remain pure. This crate holds the I/O and
//!   privilege decisions, and it must not add implicit network access, an
//!   ambient current directory, inherited environment, or global mutable state.
//! - Evaluation is pure over a frozen input and an explicit injected time;
//!   querying a stored evaluation is not acquisition, and missing inputs stay
//!   explicit non-success rather than triggering a hidden fetch or run.
//! - Capability ports are narrow and explicit. There is no default global
//!   runtime, no permissive fallback, and no synthetic trust factory.
//!
//! The module allocation for later steps (`authority`, `acquisition`,
//! `evaluation`, `query`, `storage`, `trust`) lands as each owning step
//! implements it; this step only establishes the crate and its dependency
//! direction.

#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub mod acquisition;
pub mod authority;
pub mod clock;
pub mod evaluation;
pub mod session;

pub use acquisition::{
    AcquisitionError, AcquisitionOutcome, AcquisitionReceipt, AcquisitionRequest, EvidenceVerifier,
    Executor, VerificationError, VerificationReport, VerificationRequest,
};
pub use authority::{AuthorityError, AuthorityProvider, AuthorityRequest};
pub use clock::{ClockError, host_evaluation_time};
pub use evaluation::{
    EvaluationRequest, EvidenceReadError, EvidenceReadRequest, EvidenceReader, MAX_EVIDENCE_READ,
};
pub use session::{PreparedSession, SessionError, SessionRequest, prepare};

/// A capability an application operation may require.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Capability {
    /// Protected authority access.
    Authority,
    /// Read-only evidence access.
    EvidenceRead,
    /// Privileged evidence verification.
    Verification,
    /// Privileged acquisition execution.
    Execution,
}

/// A required capability was not configured.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityError {
    /// The missing capability.
    pub capability: Capability,
}

impl Display for CapabilityError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "required capability is not configured: {:?}",
            self.capability
        )
    }
}

impl Error for CapabilityError {}

/// Explicit set of capabilities available to one operation.
///
/// Read and evaluation paths request only the capabilities they need, so they
/// structurally cannot reach privileged execution or verification. Absent
/// capabilities return an explicit [`CapabilityError`]; there is no default
/// implementation and no permissive fallback.
#[derive(Default)]
pub struct Capabilities<'a> {
    authority: Option<&'a dyn AuthorityProvider>,
    evidence: Option<&'a dyn EvidenceReader>,
    verifier: Option<&'a dyn EvidenceVerifier>,
    executor: Option<&'a dyn Executor>,
}

impl<'a> Capabilities<'a> {
    /// Creates an empty capability set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a protected authority provider.
    #[must_use]
    pub fn with_authority(mut self, value: &'a dyn AuthorityProvider) -> Self {
        self.authority = Some(value);
        self
    }

    /// Adds a read-only evidence reader.
    #[must_use]
    pub fn with_evidence(mut self, value: &'a dyn EvidenceReader) -> Self {
        self.evidence = Some(value);
        self
    }

    /// Adds a privileged evidence verifier.
    #[must_use]
    pub fn with_verifier(mut self, value: &'a dyn EvidenceVerifier) -> Self {
        self.verifier = Some(value);
        self
    }

    /// Adds a privileged acquisition executor.
    #[must_use]
    pub fn with_executor(mut self, value: &'a dyn Executor) -> Self {
        self.executor = Some(value);
        self
    }

    /// Requires the protected authority provider.
    pub fn authority(&self) -> Result<&'a dyn AuthorityProvider, CapabilityError> {
        self.authority.ok_or(CapabilityError {
            capability: Capability::Authority,
        })
    }

    /// Requires the read-only evidence reader.
    pub fn evidence(&self) -> Result<&'a dyn EvidenceReader, CapabilityError> {
        self.evidence.ok_or(CapabilityError {
            capability: Capability::EvidenceRead,
        })
    }

    /// Requires the privileged evidence verifier.
    pub fn verifier(&self) -> Result<&'a dyn EvidenceVerifier, CapabilityError> {
        self.verifier.ok_or(CapabilityError {
            capability: Capability::Verification,
        })
    }

    /// Requires the privileged acquisition executor.
    pub fn executor(&self) -> Result<&'a dyn Executor, CapabilityError> {
        self.executor.ok_or(CapabilityError {
            capability: Capability::Execution,
        })
    }
}

#[cfg(test)]
mod tests {
    /// The application layer must never depend on a transport crate.
    #[test]
    fn manifest_has_no_transport_dependency() {
        let manifest = include_str!("../Cargo.toml");
        for forbidden in ["eqm_cli", "eqm_mcp"] {
            assert!(
                !manifest.contains(forbidden),
                "application crate must not depend on {forbidden}"
            );
        }
    }

    /// The crate must stay unsafe-free and inherit the workspace lints.
    #[test]
    fn crate_is_unsafe_free_and_inherits_workspace_lints() {
        let source = include_str!("lib.rs");
        assert!(source.contains("#![forbid(unsafe_code)]"));
        let manifest = include_str!("../Cargo.toml");
        assert!(manifest.contains("[lints]"));
        assert!(manifest.contains("workspace = true"));
    }

    use super::*;
    use std::cell::Cell;
    use std::error::Error;

    #[derive(Default)]
    struct SpyReader;

    impl EvidenceReader for SpyReader {
        fn read_evidence(
            &self,
            _request: &EvidenceReadRequest,
        ) -> Result<Vec<eqm_domain::EvidenceResult>, EvidenceReadError> {
            Ok(Vec::new())
        }
    }

    #[derive(Default)]
    struct SpyExecutor {
        calls: Cell<usize>,
    }

    impl Executor for SpyExecutor {
        fn execute(
            &self,
            request: &AcquisitionRequest,
        ) -> Result<AcquisitionReceipt, AcquisitionError> {
            self.calls.set(self.calls.get() + 1);
            Ok(AcquisitionReceipt {
                plan_digest: request.plan_digest,
                outcome: AcquisitionOutcome::Completed,
            })
        }
    }

    #[derive(Default)]
    struct SpyVerifier {
        calls: Cell<usize>,
    }

    impl EvidenceVerifier for SpyVerifier {
        fn verify(
            &self,
            request: &VerificationRequest,
        ) -> Result<VerificationReport, VerificationError> {
            self.calls.set(self.calls.get() + 1);
            Ok(VerificationReport {
                subject_digest: request.subject_digest,
                verified: true,
            })
        }
    }

    #[test]
    fn read_path_never_reaches_execution_or_verification() -> Result<(), Box<dyn Error>> {
        let executor = SpyExecutor::default();
        let verifier = SpyVerifier::default();
        let reader = SpyReader;
        let capabilities = Capabilities::new()
            .with_evidence(&reader)
            .with_executor(&executor)
            .with_verifier(&verifier);
        let request = EvidenceReadRequest::new(16)?;
        let records = capabilities.evidence()?.read_evidence(&request)?;
        assert!(records.is_empty());
        assert_eq!(executor.calls.get(), 0);
        assert_eq!(verifier.calls.get(), 0);
        Ok(())
    }

    #[test]
    fn missing_capabilities_return_explicit_errors() {
        let empty = Capabilities::new();
        assert_eq!(
            empty.executor().err(),
            Some(CapabilityError {
                capability: Capability::Execution,
            })
        );
        assert_eq!(
            empty.verifier().err(),
            Some(CapabilityError {
                capability: Capability::Verification,
            })
        );
        assert_eq!(
            empty.evidence().err(),
            Some(CapabilityError {
                capability: Capability::EvidenceRead,
            })
        );
        assert_eq!(
            empty.authority().err(),
            Some(CapabilityError {
                capability: Capability::Authority,
            })
        );
    }
}
