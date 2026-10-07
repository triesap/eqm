//! Narrow access to protected evaluation authority.
//!
//! The authority port is the only way an application use case obtains
//! protected graph authority. There is no default implementation and no
//! permissive fallback: a use case that requires authority and does not have
//! the capability receives an explicit error.

use eqm_domain::RequirementScenarioBindings;
use eqm_engine::{EvaluationMode, FinalizedWorkspaceGraph};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Explicit request for protected evaluation authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthorityRequest {
    /// Evaluation mode whose protected authority is required.
    pub mode: EvaluationMode,
}

impl AuthorityRequest {
    /// Creates an explicit authority request.
    #[must_use]
    pub const fn new(mode: EvaluationMode) -> Self {
        Self { mode }
    }
}

/// Protected authority could not be supplied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthorityError {
    /// No protected authority provider was configured.
    MissingProvider,
    /// The provider could not read the protected authority.
    Unavailable,
    /// The supplied authority was invalid for the request.
    Invalid,
}

impl Display for AuthorityError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingProvider => "protected authority provider is not configured",
            Self::Unavailable => "protected authority is unavailable",
            Self::Invalid => "protected authority is invalid for the request",
        };
        formatter.write_str(message)
    }
}

impl Error for AuthorityError {}

/// One immutable provider-owned graph and explicit scope-binding selection.
///
/// Construction grants no verification result. The application validates the
/// exact map against this graph, and only the explicitly configured protected
/// capability supplies this bundle for protected evaluation.
pub struct ProtectedAuthority<'a> {
    graph: &'a FinalizedWorkspaceGraph,
    scope_bindings: &'a RequirementScenarioBindings,
}

impl<'a> ProtectedAuthority<'a> {
    /// Supplies both authority inputs in the one provider response.
    #[must_use]
    pub const fn new(
        graph: &'a FinalizedWorkspaceGraph,
        scope_bindings: &'a RequirementScenarioBindings,
    ) -> Self {
        Self {
            graph,
            scope_bindings,
        }
    }

    /// Returns the immutable finalized graph.
    #[must_use]
    pub const fn graph(&self) -> &'a FinalizedWorkspaceGraph {
        self.graph
    }

    /// Returns the provider's explicit scenario selection.
    #[must_use]
    pub const fn scope_bindings(&self) -> &'a RequirementScenarioBindings {
        self.scope_bindings
    }
}

/// Narrow provider of protected, already-finalized graph authority.
///
/// Implementations read protected authority configuration; they never execute
/// runners, fetch network content, or mutate authored state.
pub trait AuthorityProvider {
    /// Loads the immutable protected graph and explicit scope map once.
    fn load_authority(
        &self,
        request: &AuthorityRequest,
    ) -> Result<ProtectedAuthority<'_>, AuthorityError>;
}
