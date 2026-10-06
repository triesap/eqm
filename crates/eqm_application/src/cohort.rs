//! Graph-backed compatibility-cohort resolution and candidate matching.
//!
//! S041-R1 consumes only the exact graph-owned cohort: a caller-created record
//! with an absent or altered id/revision/value is rejected. The authored
//! target/provider configuration claim is retained and compared to the
//! independently projected typed graph configuration; the graph projection is
//! never authorized to delete or substitute the authored field, and a raw
//! digest string is never used in place of the typed projection. The authored
//! source/build/artifact axes remain the independent authority a candidate
//! checked subject manifest is matched against. Resolution and matching grant
//! no acquisition or trust.

use eqm_domain::{
    Cohort, OccurrenceId, Revision, ScenarioId, ScenarioSubject, SelectorText, SubjectManifest,
};
use eqm_engine::FinalizedWorkspaceGraph;
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
    /// The supplied cohort is not the exact graph-owned cohort.
    NotInGraph,
    /// No graph scenario definition carries the cohort's scenario ID.
    ScenarioNotFound,
    /// More than one graph scenario revision carries the cohort's scenario ID.
    AmbiguousScenario,
    /// An assignment declares an occurrence the scenario does not.
    UndeclaredOccurrence(OccurrenceId),
    /// A scenario occurrence has no cohort assignment.
    MissingOccurrence(OccurrenceId),
    /// The authored configuration kind disagrees with the occurrence kind.
    ConfigurationKind(OccurrenceId),
    /// The authored configuration differs from the immutable graph projection.
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

/// One cohort resolved against, and retained from, the immutable graph.
///
/// The exact graph-owned [`Cohort`] value is retained alongside its identity
/// and independently derived expected axes, so a consuming boundary can require
/// the retained value to equal the current graph entry and re-derive its axes
/// rather than trusting occurrence-name equality or a private constructor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCohort {
    cohort: Cohort,
    cohort_id: SelectorText,
    cohort_revision: Revision,
    scenario_id: ScenarioId,
    scenario_revision: Revision,
    expected: ExpectedSubjectContext,
    occurrences: usize,
}

impl ResolvedCohort {
    /// Returns the exact graph-owned cohort value retained at resolution.
    #[must_use]
    pub const fn cohort(&self) -> &Cohort {
        &self.cohort
    }

    /// Returns the exact graph-owned cohort identity.
    #[must_use]
    pub const fn cohort_id(&self) -> &SelectorText {
        &self.cohort_id
    }

    /// Returns the exact graph-owned cohort revision.
    #[must_use]
    pub const fn cohort_revision(&self) -> Revision {
        self.cohort_revision
    }

    /// Returns the exact resolved graph scenario identity.
    #[must_use]
    pub const fn scenario_id(&self) -> &ScenarioId {
        &self.scenario_id
    }

    /// Returns the exact resolved graph scenario revision.
    #[must_use]
    pub const fn scenario_revision(&self) -> Revision {
        self.scenario_revision
    }

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
///
/// The cohort must be the exact graph-owned record. Its scenario must resolve
/// to exactly one graph revision, every scenario occurrence must be covered
/// exactly, each assignment must be content-complete, and each authored
/// configuration claim must equal the typed graph projection.
pub fn resolve_cohort(
    graph: &FinalizedWorkspaceGraph,
    cohort: &Cohort,
) -> Result<ResolvedCohort, CohortError> {
    let retained = graph
        .graph()
        .cohorts()
        .get(&(cohort.id().clone(), cohort.revision()));
    if retained != Some(cohort) {
        return Err(CohortError::NotInGraph);
    }

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
        // Derive the typed configuration from the graph and require the
        // retained authored claim to equal it exactly. A target can never be
        // relabeled a provider and a raw digest string is never substituted.
        let configuration =
            match subject {
                ScenarioSubject::Target(target) => {
                    let target =
                        graph.graph().targets().get(target).ok_or_else(|| {
                            CohortError::ConfigurationMismatch(occurrence.clone())
                        })?;
                    let projected = canonicalize_target_configuration_v2(target)?.digest();
                    match (
                        assignment.target_configuration(),
                        assignment.provider_configuration(),
                    ) {
                        (Some(claimed), None) if claimed == projected.as_digest() => {
                            ExpectedSubjectConfiguration::Target(projected)
                        }
                        (Some(_), None) => {
                            return Err(CohortError::ConfigurationMismatch(occurrence.clone()));
                        }
                        _ => return Err(CohortError::ConfigurationKind(occurrence.clone())),
                    }
                }
                ScenarioSubject::Provider(provider) => {
                    let provider =
                        graph.graph().providers().get(provider).ok_or_else(|| {
                            CohortError::ConfigurationMismatch(occurrence.clone())
                        })?;
                    let projected = canonicalize_provider_configuration_v2(provider)?.digest();
                    match (
                        assignment.target_configuration(),
                        assignment.provider_configuration(),
                    ) {
                        (None, Some(claimed)) if claimed == projected.as_digest() => {
                            ExpectedSubjectConfiguration::Provider(projected)
                        }
                        (None, Some(_)) => {
                            return Err(CohortError::ConfigurationMismatch(occurrence.clone()));
                        }
                        _ => return Err(CohortError::ConfigurationKind(occurrence.clone())),
                    }
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
        cohort: cohort.clone(),
        cohort_id: cohort.id().clone(),
        cohort_revision: cohort.revision(),
        scenario_id: scenario.id().clone(),
        scenario_revision: scenario.revision(),
        expected,
        occurrences: participants.len(),
    })
}

/// Matches one checked candidate subject manifest against the resolved cohort.
///
/// The authored cohort axes and the graph-projected configuration are the
/// independent expected authority; the candidate supplies only its own axes and
/// can never seed the expectations. This matcher takes no graph: it establishes
/// matching against the supplied resolved authority only and never claims
/// current-graph or scenario validation. The checked comparison consumer
/// ([`crate::evaluate_checked_comparison`]) performs that binding before it
/// relies on a match.
pub fn match_cohort_subjects(
    checked: &CheckedSubjectManifestV2,
    resolved: &ResolvedCohort,
) -> Result<SubjectManifest, SubjectMatchError> {
    match_subjects(checked, resolved.expected(), SubjectMatchMode::Completeness)
}
