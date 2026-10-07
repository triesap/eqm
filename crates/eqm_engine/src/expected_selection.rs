//! Expected-set completeness independently of reported outcome/count claims.
use crate::{CoverageStatus, FacetEvaluationInput, ObligationKey, RequiredUniverse};
use eqm_domain::{
    ExpectedSelectionManifest, ExpectedSelectionSource, LogicalCaseId, PositiveCount,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Pre-evidence claims indexed by the complete pre-identity obligation coordinate.
/// Manifests under a coordinate compose with AND, never an observed-pass OR.
pub type ExpectedSelectionMap = BTreeMap<ObligationKey, Vec<ExpectedSelectionManifest>>;

/// One reported logical observation claim. Admission/trust/outcome validation is
/// separately required; this type cannot construct protected or evaluated authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionObservation {
    /// Exact binding/local-specification/plan/campaign lineage.
    pub source: ExpectedSelectionSource,
    /// Declared selected logical unit; attempts and transports repeat this ID.
    pub case: LogicalCaseId,
}
/// Binding errors leave the expected set non-authorizing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedSelectionBindingError {
    /// The coordinate is not in this immutable universe.
    Coordinate,
    /// Binding/revision/local specification is absent or incompatible with scope.
    Specification,
    /// A coordinate mixes count units or selected plan/campaign identities.
    AmbiguousCountUnit,
    /// No manifest under a supplied coordinate.
    Empty,
    /// The same binding/specification was declared twice for one coordinate.
    Duplicate,
}
impl Display for ExpectedSelectionBindingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "expected selection binding failed: {self:?}")
    }
}
impl Error for ExpectedSelectionBindingError {}

/// Attaches explicit pre-result expectations to the private semantic context.
/// The application must verify supplied semantic digests and protected origin;
/// this pure operation checks graph association and retains all definitions.
pub fn bind_expected_selections(
    mut universe: RequiredUniverse,
    selections: ExpectedSelectionMap,
) -> Result<RequiredUniverse, ExpectedSelectionBindingError> {
    for (key, manifests) in &selections {
        if !universe.contains(key) {
            return Err(ExpectedSelectionBindingError::Coordinate);
        }
        if manifests.is_empty() {
            return Err(ExpectedSelectionBindingError::Empty);
        }
        let mut seen = BTreeSet::new();
        let reference = &manifests[0];
        if manifests.iter().any(|value| {
            value.count_unit() != reference.count_unit()
                || value.source().plan != reference.source().plan
                || value.source().campaign != reference.source().campaign
        }) {
            return Err(ExpectedSelectionBindingError::AmbiguousCountUnit);
        }
        for manifest in manifests {
            let source = manifest.source();
            if !seen.insert((source.binding.clone(), source.specification.clone())) {
                return Err(ExpectedSelectionBindingError::Duplicate);
            }
            let binding = universe
                .context
                .graph
                .bindings()
                .get(&source.binding)
                .ok_or(ExpectedSelectionBindingError::Specification)?;
            let specification = binding
                .evidence()
                .get(&source.specification)
                .ok_or(ExpectedSelectionBindingError::Specification)?;
            if binding.revision() != source.binding_revision
                || binding.unit() != &key.unit
                || !key.subject.participates(binding.target())
                || !specification.requirements().contains(&key.requirement)
                || !specification.facets().contains(&key.facet)
                || specification.kind() != manifest.kind()
            {
                return Err(ExpectedSelectionBindingError::Specification);
            }
        }
    }
    universe.context.expected_selections = Some(selections);
    Ok(universe)
}
impl RequiredUniverse {
    /// Returns explicit expectations, including a deliberately empty unresolved map.
    #[must_use]
    pub const fn expected_selections(&self) -> Option<&ExpectedSelectionMap> {
        self.context.expected_selections.as_ref()
    }
    /// Same finalized semantic graph check for application component preparation.
    #[must_use]
    pub fn matches_graph(&self, graph: &crate::FinalizedWorkspaceGraph) -> bool {
        self.context.graph == *graph.graph()
    }
}
/// Counts complete expected logical selections; duplicates never add units.
/// Foreign/unselected claims are Unknown. Missing any AND member is Missing,
/// even if enough other reported observations passed or were copied repeatedly.
#[must_use]
pub fn expected_selection_coverage(
    manifests: &[ExpectedSelectionManifest],
    observations: &[SelectionObservation],
    minimum: PositiveCount,
) -> CoverageStatus {
    if manifests.is_empty() {
        return CoverageStatus::Unknown;
    }
    let reference = &manifests[0];
    if manifests.iter().any(|value| {
        value.count_unit() != reference.count_unit()
            || value.source().plan != reference.source().plan
            || value.source().campaign != reference.source().campaign
    }) {
        return CoverageStatus::Unknown;
    }
    let mut expected = BTreeSet::new();
    for manifest in manifests {
        for case in manifest.selected() {
            expected.insert((manifest.source().clone(), case.clone()));
        }
    }
    let mut observed = BTreeSet::new();
    for observation in observations {
        let key = (observation.source.clone(), observation.case.clone());
        if !expected.contains(&key) {
            return CoverageStatus::Unknown;
        }
        observed.insert(key);
    }
    let logical_units: BTreeSet<_> = observed.iter().map(|(_, case)| case).collect();
    if expected != observed || (logical_units.len() as u64) < minimum.get() {
        CoverageStatus::Missing
    } else {
        CoverageStatus::Covered
    }
}
/// Guards prepared facet facts using frozen expected selection completeness.
/// Facts and observations remain claims until the owning admission/trust layers
/// verify them. This cannot turn a failing/unknown outcome into success.
pub fn evaluate_required_facets_with_expected_selections(
    universe: &RequiredUniverse,
    mut inputs: BTreeMap<ObligationKey, FacetEvaluationInput>,
    observations: &BTreeMap<ObligationKey, Vec<SelectionObservation>>,
) -> Result<crate::EvaluatedFacets, crate::ConformanceError> {
    if observations.keys().any(|key| !universe.contains(key))
        || inputs.keys().any(|key| !universe.contains(key))
    {
        return Err(crate::ConformanceError::UnexpectedCoordinate);
    }
    for (key, input) in &mut inputs {
        let coverage = universe
            .expected_selections()
            .and_then(|map| map.get(key))
            .map_or(CoverageStatus::Unknown, |manifests| {
                let minimum = universe.context.obligations[key].strength.minimum_count;
                expected_selection_coverage(
                    manifests,
                    observations.get(key).map_or(&[], Vec::as_slice),
                    minimum,
                )
            });
        // Never upgrade prior coverage or permit a waiver to erase unknown scope.
        if coverage != CoverageStatus::Covered {
            input.coverage = if input.coverage == CoverageStatus::Unknown {
                CoverageStatus::Unknown
            } else {
                coverage
            };
            if input.coverage == CoverageStatus::Unknown {
                input.context_valid = false;
            }
        }
    }
    crate::conformance::evaluate_prepared_facets(universe, inputs)
}
