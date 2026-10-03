//! Graph-backed compatibility-cohort resolution and candidate matching.
//!
//! S041 resolves an authored cohort against the immutable prepared graph: the
//! scenario must identify exactly one graph definition, every declared
//! occurrence must be covered exactly, and each assignment's target/provider
//! configuration must equal the graph projection. The authored source/build/
//! artifact axes remain the independent authority a candidate checked subject
//! manifest is matched against; expected values are never copied from the
//! candidate. Resolution and matching grant no acquisition or trust.

use eqm_domain::{Cohort, FinalizedWorkspaceGraph, OccurrenceId, ScenarioSubject, SubjectManifest};
use eqm_manifest::{
    CanonicalizationError, canonicalize_provider_configuration_v2,
    canonicalize_target_configuration_v2,
};
use eqm_protocol::v2::{
    CheckedSubjectManifestV2, ExpectedSubjectAxis, ExpectedSubjectConfiguration,
    ExpectedSubjectContext, SubjectMatchError, SubjectMatchMode, match_subjects,
};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Graph-backed cohort resolution failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CohortError {
    /// No graph scenario definition carries the cohort's scenario ID.
    ScenarioNotFound,
    /// More than one graph scenario revision carries the cohort's scenario ID.
    AmbiguousScenario,
    /// An assignment declares an occurrence the scenario does not.
    UndeclaredOccurrence(OccurrenceId),
    /// A scenario occurrence has no cohort assignment.
    MissingOccurrence(OccurrenceId),
    /// The assignment configuration kind disagrees with the occurrence kind.
    ConfigurationKind,
    /// The assignment configuration differs from the immutable graph projection.
    ConfigurationMismatch(OccurrenceId),
    /// The authored combination omits a source, build or artifact axis.
    IncompleteOccurrence(OccurrenceId),
    /// The graph projection could not be computed.
    Canonicalize,
}

impl Display for CohortError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for CohortError {}

impl From<CanonicalizationError> for CohortError {
    fn from(_: CanonicalizationError) -> Self {
        Self::Canonicalize
    }
}

/// One cohort resolved against the immutable graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCohort {
    expected: ExpectedSubjectContext,
    occurrences: usize,
}

impl ResolvedCohort {
    /// Returns the exact independently supplied expected subject context.
    #[must_use]
    pub const fn expected(&self) -> &ExpectedSubjectContext {
        &self.expected
    }

    /// Returns the resolved occurrence count.
    #[must_use]
    pub const fn occurrences(&self) -> usize {
        self.occurrences
    }
}

/// Resolves one authored cohort against the immutable prepared graph.
pub fn resolve_cohort(
    graph: &FinalizedWorkspaceGraph,
    cohort: &Cohort,
) -> Result<ResolvedCohort, CohortError> {
    let mut definitions = graph
        .graph()
        .scenarios()
        .values()
        .filter(|scenario| scenario.id() == cohort.scenario());
    let scenario = definitions.next().ok_or(CohortError::ScenarioNotFound)?;
    if definitions.next().is_some() {
        return Err(CohortError::AmbiguousScenario);
    }

    let participants: BTreeMap<&OccurrenceId, &ScenarioSubject> = scenario
        .participants()
        .iter()
        .map(|participant| (participant.occurrence(), participant.subject()))
        .collect();

    let mut axes = Vec::with_capacity(cohort.assignments().len());
    for assignment in cohort.assignments() {
        let occurrence = assignment.occurrence();
        let subject = participants
            .get(occurrence)
            .ok_or_else(|| CohortError::UndeclaredOccurrence(occurrence.clone()))?;
        if !assignment.is_complete() {
            return Err(CohortError::IncompleteOccurrence(occurrence.clone()));
        }
        let configuration =
            match subject {
                ScenarioSubject::Target(target) => {
                    let target =
                        graph.graph().targets().get(target).ok_or_else(|| {
                            CohortError::ConfigurationMismatch(occurrence.clone())
                        })?;
                    ExpectedSubjectConfiguration::Target(
                        canonicalize_target_configuration_v2(target)?.digest(),
                    )
                }
                ScenarioSubject::Provider(provider) => {
                    let provider =
                        graph.graph().providers().get(provider).ok_or_else(|| {
                            CohortError::ConfigurationMismatch(occurrence.clone())
                        })?;
                    ExpectedSubjectConfiguration::Provider(
                        canonicalize_provider_configuration_v2(provider)?.digest(),
                    )
                }
            };
        axes.push((
            occurrence.clone(),
            ExpectedSubjectAxis {
                repository: assignment.repository().clone(),
                source_commit: assignment.source_commit().clone(),
                source_content_digest: assignment.source_content(),
                configuration,
                build_id: assignment.build_id().cloned(),
                artifact_digest: assignment.artifact_digest(),
            },
        ));
    }
    if axes.len() != participants.len() {
        for occurrence in participants.keys() {
            if !cohort
                .assignments()
                .iter()
                .any(|assignment| assignment.occurrence() == *occurrence)
            {
                return Err(CohortError::MissingOccurrence((*occurrence).clone()));
            }
        }
    }
    let expected = ExpectedSubjectContext::new(axes).map_err(|_| CohortError::ScenarioNotFound)?;
    Ok(ResolvedCohort {
        expected,
        occurrences: participants.len(),
    })
}

/// Matches one checked candidate subject manifest against the resolved cohort.
///
/// The authored cohort axes and the graph-projected configuration are the
/// independent expected authority; the candidate supplies only its own axes and
/// can never seed the expectations.
pub fn match_cohort_subjects(
    checked: &CheckedSubjectManifestV2,
    resolved: &ResolvedCohort,
) -> Result<SubjectManifest, SubjectMatchError> {
    match_subjects(checked, resolved.expected(), SubjectMatchMode::Completeness)
}
