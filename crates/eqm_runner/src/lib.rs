//! Approved runner and adapter execution boundaries for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod resolution;
mod substitution;

pub use resolution::{
    ResolvedProgram, ResolvedRunner, RunnerResolutionAuthority, RunnerResolutionError,
    resolve_runner,
};
pub use substitution::{InvocationBindings, SubstitutionError, substitute_argv};
