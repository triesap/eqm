//! Same-session preparation of independently declared expected selections.
use crate::{AuthorityError, AuthorityRequest, PreparedObligations};
use eqm_domain::ExpectedSelectionManifestDigest;
use eqm_engine::{
    EvaluationMode, ExpectedSelectionBindingError, ExpectedSelectionMap, FinalizedWorkspaceGraph,
    ObligationKey, RequiredUniverse, bind_expected_selections,
};
use eqm_manifest::{
    CanonicalizationError, canonicalize_binding_v2, canonicalize_evidence_spec_v2,
    canonicalize_expected_selection_v2,
};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Explicit protected expected-selection capability; no ambient/default provider.
/// The implementing host must authorize this frozen response before results are read.
pub trait ExpectedSelectionProvider {
    /// Loads one immutable protected selection bound to the requested mode.
    fn load_expected_selections(
        &self,
        request: &AuthorityRequest,
    ) -> Result<ExpectedSelectionMap, AuthorityError>;
}
/// Checked pre-result universe and exact per-manifest semantic identities.
/// This is not evidence verification, outcome evaluation or release authorization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedExpectedSelections {
    universe: RequiredUniverse,
    digests: BTreeMap<ObligationKey, Vec<ExpectedSelectionManifestDigest>>,
}
impl PreparedExpectedSelections {
    /// Exact universe with expectations bound into private context.
    #[must_use]
    pub const fn universe(&self) -> &RequiredUniverse {
        &self.universe
    }
    /// Independent manifest identities, ordered by canonical semantic digest.
    #[must_use]
    pub const fn digests(&self) -> &BTreeMap<ObligationKey, Vec<ExpectedSelectionManifestDigest>> {
        &self.digests
    }
}
/// Preparation fails without using observed results as a fallback.
#[derive(Debug)]
pub enum ExpectedSelectionPreparationError {
    /// A graph other than the prepared immutable graph was supplied.
    Graph,
    /// A development claim attempted a protected-only choice of alternative.
    UnprotectedAlternative,
    /// Obligations did not pass the configured protected graph provider.
    ProtectedGraph,
    /// Protected response unavailable or absent.
    Authority(AuthorityError),
    /// Candidate expectations differ from protected frozen authority.
    CandidateMismatch,
    /// Local binding/specification semantic identity differs from actual graph.
    Component,
    /// Pure coordinate/specification association failed.
    Binding(ExpectedSelectionBindingError),
    /// Semantic projection failed.
    Canonicalization(CanonicalizationError),
}
impl Display for ExpectedSelectionPreparationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "expected selection preparation failed: {self:?}")
    }
}
impl Error for ExpectedSelectionPreparationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Authority(e) => Some(e),
            Self::Binding(e) => Some(e),
            Self::Canonicalization(e) => Some(e),
            Self::Graph
            | Self::UnprotectedAlternative
            | Self::ProtectedGraph
            | Self::CandidateMismatch
            | Self::Component => None,
        }
    }
}
/// Freezes expected cases independently of results using the same prepared graph.
/// Protected modes load a configured provider once; candidate claims are checked
/// but cannot replace it. Missing coordinate expectations remain visibly unresolved
/// in expected-aware evaluation. No file reload, clock capture or execution occurs.
pub fn prepare_expected_selections(
    graph: &FinalizedWorkspaceGraph,
    prepared: &PreparedObligations<'_>,
    claims: Option<ExpectedSelectionMap>,
    provider: Option<&dyn ExpectedSelectionProvider>,
) -> Result<PreparedExpectedSelections, ExpectedSelectionPreparationError> {
    if !prepared.universe().matches_graph(graph) {
        return Err(ExpectedSelectionPreparationError::Graph);
    }
    if prepared.selection().mode() != EvaluationMode::Development
        && !prepared.protected_authority_verified
    {
        return Err(ExpectedSelectionPreparationError::ProtectedGraph);
    }
    if prepared.selection().mode() == EvaluationMode::Development
        && claims.as_ref().is_some_and(|map| {
            map.values()
                .flatten()
                .any(|manifest| !manifest.alternatives().is_empty())
        })
    {
        return Err(ExpectedSelectionPreparationError::UnprotectedAlternative);
    }
    // Validate supplied claims before capability lookup; validation grants no authority.
    let claims = claims
        .map(|claims| checked(graph, prepared.universe(), claims))
        .transpose()?;
    let (universe, digests) = if prepared.selection().mode() == EvaluationMode::Development {
        match claims {
            Some(value) => value,
            None => checked(graph, prepared.universe(), BTreeMap::new())?,
        }
    } else {
        let authority = provider
            .ok_or(ExpectedSelectionPreparationError::Authority(
                AuthorityError::MissingProvider,
            ))?
            .load_expected_selections(&AuthorityRequest::new(prepared.selection().mode()))
            .map_err(ExpectedSelectionPreparationError::Authority)?;
        let protected = checked(graph, prepared.universe(), authority)?;
        if claims.is_some_and(|claims| claims != protected) {
            return Err(ExpectedSelectionPreparationError::CandidateMismatch);
        }
        protected
    };
    Ok(PreparedExpectedSelections { universe, digests })
}
type CheckedExpectedSelection = (
    RequiredUniverse,
    BTreeMap<ObligationKey, Vec<ExpectedSelectionManifestDigest>>,
);
fn checked(
    graph: &FinalizedWorkspaceGraph,
    universe: &RequiredUniverse,
    mut selections: ExpectedSelectionMap,
) -> Result<CheckedExpectedSelection, ExpectedSelectionPreparationError> {
    // Association validation precedes component identity; pure binding returns no trust.
    bind_expected_selections(universe.clone(), selections.clone())
        .map_err(ExpectedSelectionPreparationError::Binding)?;
    let mut digests = BTreeMap::new();
    for (key, manifests) in &mut selections {
        let mut ordered = Vec::new();
        for manifest in manifests.drain(..) {
            let source = manifest.source();
            let binding = graph
                .graph()
                .bindings()
                .get(&source.binding)
                .ok_or(ExpectedSelectionPreparationError::Component)?;
            let specification = binding
                .evidence()
                .get(&source.specification)
                .ok_or(ExpectedSelectionPreparationError::Component)?;
            let binding_digest = canonicalize_binding_v2(binding)
                .map_err(ExpectedSelectionPreparationError::Canonicalization)?
                .digest();
            let spec_digest = canonicalize_evidence_spec_v2(specification)
                .map_err(ExpectedSelectionPreparationError::Canonicalization)?
                .digest();
            if source.binding_digest != binding_digest || source.specification_digest != spec_digest
            {
                return Err(ExpectedSelectionPreparationError::Component);
            }
            let digest = canonicalize_expected_selection_v2(&manifest)
                .map_err(ExpectedSelectionPreparationError::Canonicalization)?
                .digest();
            ordered.push((digest, manifest));
        }
        ordered.sort_by_key(|(digest, _)| *digest);
        digests.insert(
            key.clone(),
            ordered.iter().map(|(digest, _)| *digest).collect(),
        );
        *manifests = ordered.into_iter().map(|(_, manifest)| manifest).collect();
    }
    let universe = bind_expected_selections(universe.clone(), selections)
        .map_err(ExpectedSelectionPreparationError::Binding)?;
    Ok((universe, digests))
}
