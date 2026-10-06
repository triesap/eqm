//! Engine-owned opaque authority token for a completely finalized graph.
//!
//! [`FinalizedWorkspaceGraph`] proves that graph resolution, fragment
//! expansion, and post-expansion invariant validation all completed inside the
//! engine. Its construction is private to this crate: there is no public
//! constructor and no compatibility alias, so an arbitrary raw
//! [`WorkspaceGraph`] cannot be promoted to authority by a caller. Pure
//! projection layers receive the raw graph through [`Self::graph`] as data.

use eqm_domain::WorkspaceGraph;

/// Immutable graph proven to have completed resolution, expansion, and
/// invariant validation.
///
/// No public constructor or compatibility alias promotes a raw graph, so a
/// caller cannot manufacture authority from locally indexed input:
///
/// ```compile_fail
/// use eqm_domain::{WorkspaceGraph, WorkspaceGraphInput};
/// use eqm_engine::FinalizedWorkspaceGraph;
/// let graph = WorkspaceGraph::new(WorkspaceGraphInput::default()).unwrap();
/// let _ = FinalizedWorkspaceGraph::new(graph);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinalizedWorkspaceGraph(WorkspaceGraph);

impl FinalizedWorkspaceGraph {
    /// Wraps a graph after the pure engine has completed all finalization
    /// stages and revalidated the expanded result.
    ///
    /// This is reachable only from inside the engine; no public constructor or
    /// alias promotes a caller-supplied raw graph.
    #[must_use]
    pub(crate) const fn new(graph: WorkspaceGraph) -> Self {
        Self(graph)
    }

    /// Returns the finalized immutable graph as projection data.
    #[must_use]
    pub const fn graph(&self) -> &WorkspaceGraph {
        &self.0
    }
}
