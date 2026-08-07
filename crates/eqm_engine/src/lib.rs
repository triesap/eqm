//! Pure graph resolution and evaluation for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod resolve;

pub use resolve::{ResolutionError, resolution_diagnostics, resolve_graph};
