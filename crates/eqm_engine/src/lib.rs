//! Pure graph resolution and evaluation for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod invariants;
mod resolve;

pub use invariants::validate_graph_invariants;
pub use resolve::{ResolutionError, resolution_diagnostics, resolve_graph};
