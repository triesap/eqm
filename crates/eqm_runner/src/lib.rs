//! Approved runner and adapter execution boundaries for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod resolution;

pub use resolution::{
    ResolvedProgram, ResolvedRunner, RunnerResolutionAuthority, RunnerResolutionError,
    resolve_runner,
};
