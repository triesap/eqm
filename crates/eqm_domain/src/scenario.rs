//! Role-bearing scenario participant occurrences (R07, AC007).
//!
//! A scenario has an exact identity and revision, a set of participant
//! occurrences keyed by a unique occurrence ID, each with an explicit role and a
//! tagged target/provider subject, and explicit directed dependencies between
//! occurrence IDs. Repeated target/provider references with different occurrence
//! IDs remain distinct; the set is never collapsed. Dependencies must reference
//! supplied occurrences and may not be self or duplicate.

use crate::{Extensions, OccurrenceId, ProviderId, Revision, RoleId, ScenarioId, TargetId};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// One tagged scenario participant subject.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ScenarioSubject {
    /// A shipped target occurrence.
    Target(TargetId),
    /// A shared provider occurrence.
    Provider(ProviderId),
}

/// One role-bearing participant occurrence.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScenarioParticipant {
    occurrence: OccurrenceId,
    role: RoleId,
    subject: ScenarioSubject,
}

impl ScenarioParticipant {
    /// Creates one occurrence with an explicit role and tagged subject.
    #[must_use]
    pub const fn new(occurrence: OccurrenceId, role: RoleId, subject: ScenarioSubject) -> Self {
        Self {
            occurrence,
            role,
            subject,
        }
    }

    /// Returns the unique occurrence ID.
    #[must_use]
    pub const fn occurrence(&self) -> &OccurrenceId {
        &self.occurrence
    }

    /// Returns the explicit role.
    #[must_use]
    pub const fn role(&self) -> &RoleId {
        &self.role
    }

    /// Returns the tagged participant subject.
    #[must_use]
    pub const fn subject(&self) -> &ScenarioSubject {
        &self.subject
    }
}

/// One explicit directed dependency between two occurrence IDs.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScenarioDependency {
    from: OccurrenceId,
    to: OccurrenceId,
}

impl ScenarioDependency {
    /// Creates a dependency, rejecting a self edge.
    pub fn new(from: OccurrenceId, to: OccurrenceId) -> Result<Self, ScenarioError> {
        if from == to {
            return Err(ScenarioError::SelfDependency);
        }
        Ok(Self { from, to })
    }

    /// Returns the dependent occurrence.
    #[must_use]
    pub const fn from(&self) -> &OccurrenceId {
        &self.from
    }

    /// Returns the required occurrence.
    #[must_use]
    pub const fn to(&self) -> &OccurrenceId {
        &self.to
    }
}

/// One validated end-to-end scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scenario {
    id: ScenarioId,
    revision: Revision,
    participants: BTreeSet<ScenarioParticipant>,
    dependencies: BTreeSet<ScenarioDependency>,
    extensions: Extensions,
}

impl Scenario {
    /// Creates a scenario with nonempty, distinct participant occurrences and
    /// dependencies that reference them exactly.
    pub fn new(
        id: ScenarioId,
        revision: Revision,
        participants: Vec<ScenarioParticipant>,
        dependencies: Vec<ScenarioDependency>,
        extensions: Extensions,
    ) -> Result<Self, ScenarioError> {
        if participants.is_empty() {
            return Err(ScenarioError::ParticipantsRequired);
        }
        let mut occurrences = BTreeSet::new();
        let mut participants_set = BTreeSet::new();
        for participant in participants {
            if !occurrences.insert(participant.occurrence().clone()) {
                return Err(ScenarioError::DuplicateOccurrence);
            }
            participants_set.insert(participant);
        }
        let mut dependencies_set = BTreeSet::new();
        for dependency in dependencies {
            if !occurrences.contains(dependency.from()) || !occurrences.contains(dependency.to()) {
                return Err(ScenarioError::UnknownOccurrence);
            }
            if !dependencies_set.insert(dependency) {
                return Err(ScenarioError::DuplicateDependency);
            }
        }
        Ok(Self {
            id,
            revision,
            participants: participants_set,
            dependencies: dependencies_set,
            extensions,
        })
    }

    /// Returns the scenario authority ID.
    #[must_use]
    pub const fn id(&self) -> &ScenarioId {
        &self.id
    }

    /// Returns the exact scenario revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns participant occurrences in canonical complete-member order.
    #[must_use]
    pub const fn participants(&self) -> &BTreeSet<ScenarioParticipant> {
        &self.participants
    }

    /// Returns directed dependencies in canonical order.
    #[must_use]
    pub const fn dependencies(&self) -> &BTreeSet<ScenarioDependency> {
        &self.dependencies
    }

    /// Returns normative extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

/// Scenario construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScenarioError {
    /// No participant occurrence was declared.
    ParticipantsRequired,
    /// Two participants shared one occurrence ID.
    DuplicateOccurrence,
    /// A dependency referenced an occurrence absent from the scenario.
    UnknownOccurrence,
    /// A dependency referenced its own occurrence.
    SelfDependency,
    /// The same directed dependency was declared twice.
    DuplicateDependency,
}

impl Display for ScenarioError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ParticipantsRequired => "scenario requires at least one participant occurrence",
            Self::DuplicateOccurrence => "scenario occurrence IDs contain a duplicate",
            Self::UnknownOccurrence => "scenario dependency references a missing occurrence",
            Self::SelfDependency => "scenario occurrence depends on itself",
            Self::DuplicateDependency => "scenario dependencies contain a duplicate",
        })
    }
}

impl Error for ScenarioError {}
