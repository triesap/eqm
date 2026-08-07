//! Approved runner and adapter execution boundaries for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod ci;
mod container;
mod execution;
mod normalized;
mod persistence;
mod resolution;
mod substitution;

pub use ci::{
    CiDelegatedImport, CiImportAuthority, CiImportError, CiReplayGuard, CiSignature,
    CiSignatureVerifier, VerifiedCiSignature, import_ci_delegated_result,
};
pub use container::{
    ContainerAuthority, ContainerError, ContainerPlan, prepare_container_execution,
};
pub use execution::{
    CancellationToken, ExecutionOutcome, ExecutionReport, LocalExecutionContext,
    LocalExecutionError, execute_local_process,
};
pub use normalized::{NormalizedTestResult, TestResultReadError, read_test_result};
pub use persistence::{EvidenceWriteError, EvidenceWriteOutcome, persist_evidence_result};
pub use resolution::{
    ResolvedProgram, ResolvedRunner, RunnerResolutionAuthority, RunnerResolutionError,
    resolve_runner,
};
pub use substitution::{InvocationBindings, SubstitutionError, substitute_argv};
