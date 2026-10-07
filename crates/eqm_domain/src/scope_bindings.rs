//! Explicit requirement-to-scenario selection claims.
//!
//! These values preserve exact identities and reject duplicate authored keys.
//! Construction grants no authority: the engine must validate them against the
//! same finalized graph, and protected application inputs come from authority.

use crate::{FullRequirementId, Revision, ScenarioId};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// One scenario identity with its exact revision; never a floating selection.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScenarioRef {
    id: ScenarioId,
    revision: Revision,
}

impl ScenarioRef {
    /// Creates an exact typed reference, without asserting graph membership.
    #[must_use]
    pub const fn new(id: ScenarioId, revision: Revision) -> Self {
        Self { id, revision }
    }

    /// Returns the exact scenario ID.
    #[must_use]
    pub const fn id(&self) -> &ScenarioId {
        &self.id
    }

    /// Returns the exact scenario revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
}

/// Explicit exact scenario bindings, retaining every distinct requirement key.
///
/// An explicit empty map can be represented, but supplies no scenario authority.
/// There is no implicit default, serde constructor or overwrite-on-duplicate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequirementScenarioBindings {
    bindings: BTreeMap<FullRequirementId, ScenarioRef>,
}

impl RequirementScenarioBindings {
    /// Collects supplied claims, rejecting even identical duplicate keys.
    pub fn new(entries: Vec<(FullRequirementId, ScenarioRef)>) -> Result<Self, ScopeBindingsError> {
        let mut bindings = BTreeMap::new();
        for (requirement, scenario) in entries {
            if bindings.contains_key(&requirement) {
                return Err(ScopeBindingsError::DuplicateRequirement(requirement));
            }
            bindings.insert(requirement, scenario);
        }
        Ok(Self { bindings })
    }

    /// Returns exact bindings in full requirement identity order.
    #[must_use]
    pub const fn bindings(&self) -> &BTreeMap<FullRequirementId, ScenarioRef> {
        &self.bindings
    }
}

/// Invalid local scenario-binding construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScopeBindingsError {
    /// A requirement occurred twice, whether its scenario claims agree or not.
    DuplicateRequirement(FullRequirementId),
}

impl Display for ScopeBindingsError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateRequirement(id) => {
                write!(formatter, "duplicate scenario binding for requirement {id}")
            }
        }
    }
}

impl Error for ScopeBindingsError {}
