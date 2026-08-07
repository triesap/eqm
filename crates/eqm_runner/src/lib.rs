//! Approved runner and adapter execution boundaries for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod execution;
mod resolution;
mod substitution;

pub use execution::{
    CancellationToken, ExecutionOutcome, ExecutionReport, LocalExecutionContext,
    LocalExecutionError, execute_local_process,
};
pub use resolution::{
    ResolvedProgram, ResolvedRunner, RunnerResolutionAuthority, RunnerResolutionError,
    resolve_runner,
};
pub use substitution::{InvocationBindings, SubstitutionError, substitute_argv};
