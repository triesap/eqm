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
}
