//! Complete pure pre-evidence scope projection, before presentation filtering.
use crate::{ObligationIdentityError, PreparedObligationIdentities};
use eqm_domain::UtcInstantV2;
use eqm_engine::{AuthorityOrigin, EvaluationMode, FinalizedWorkspaceGraph, UnresolvedReason};
use eqm_manifest::{
    CanonicalizationError, canonicalize_graph_v2, canonicalize_obligation_scope_v2,
    canonicalize_scope_bindings_v2, canonicalize_target_configuration_v2,
};
use eqm_protocol::schema_graph_v2::{
    DimensionKnownV2Dto, DimensionUnknownV2Dto, DimensionValueV2Dto, KnownState, UnknownState,
};
use eqm_protocol::*;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// A projection failure keeps the complete query unavailable, never truncated.
#[derive(Debug)]
pub enum QueryPreparationError {
    /// Finalized graph is foreign to the prepared universe.
    ForeignGraph,
    /// Complete visible work exceeds the declared bound.
    Limit,
    /// The retained prepared identities are internally inconsistent.
    MissingIdentity,
    /// Exact scope projection is unavailable.
    Scope(ObligationIdentityError),
    /// Pure canonicalization failed.
    Canonical(CanonicalizationError),
    /// The strict wire projection failed its owning invariants.
    Protocol(ObligationQueryError),
    /// Canonical set contained duplicate members.
    Collection(CanonicalCollectionError),
}
impl Display for QueryPreparationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "complete obligation query failed: {self:?}")
    }
}
impl Error for QueryPreparationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Scope(e) => Some(e),
            Self::Canonical(e) => Some(e),
            Self::Protocol(e) => Some(e),
            Self::Collection(e) => Some(e),
            _ => None,
        }
    }
}
struct Record {
    view: ObligationViewV2Dto,
}
/// Complete immutable pre-evidence work; only checked preparation constructs it.
/// Filtering cannot replace its denominator or underlying prepared universe.
///
/// ```compile_fail
/// let _ = eqm_application::query::PreparedObligationQuery::default();
/// ```
pub struct PreparedObligationQuery {
    identities: PreparedObligationIdentities,
    context: ObligationQueryContextV2,
    records: Vec<Record>,
}
fn reason(value: UnresolvedReason) -> ObligationQueryReason {
    match value {
        UnresolvedReason::NotApplicable => ObligationQueryReason::NotApplicable,
        UnresolvedReason::UnknownApplicability => ObligationQueryReason::UnknownApplicability,
        UnresolvedReason::UnknownParticipation => ObligationQueryReason::UnknownParticipation,
        UnresolvedReason::MissingScopeAuthority => ObligationQueryReason::MissingScopeAuthority,
        UnresolvedReason::UnmatchedPolicy => ObligationQueryReason::UnmatchedPolicy,
    }
}
/// Constructs a complete query from one actual checked graph and identity map.
/// Evaluation time/offline are explicit display inputs. No evidence, filesystem,
/// verifier, execution, clock or storage capability is accepted here.
pub fn prepare_obligation_query(
    finalized: &FinalizedWorkspaceGraph,
    identities: &PreparedObligationIdentities,
    evaluated_at: UtcInstantV2,
    offline: bool,
) -> Result<PreparedObligationQuery, QueryPreparationError> {
    let u = identities.universe();
    if !u.matches_graph(finalized) {
        return Err(QueryPreparationError::ForeignGraph);
    }
    if u.coordinates().len().saturating_add(u.unresolved().len()) > MAX_OBLIGATION_QUERY_RECORDS {
        return Err(QueryPreparationError::Limit);
    }
    if identities.identities().len() != u.coordinates().len() {
        return Err(QueryPreparationError::MissingIdentity);
    }
    let minimum_bytes = identities
        .identities()
        .values()
        .try_fold(0usize, |sum, identity| {
            sum.checked_add(identity.bytes().len())?
                .checked_add(identity.reference().len())
        })
        .ok_or(QueryPreparationError::Limit)?;
    if minimum_bytes > MAX_OBLIGATION_QUERY_BYTES {
        return Err(QueryPreparationError::Limit);
    }
    let profiles = CanonicalSet::from_members(
        u.profiles()
            .values()
            .map(|p| {
                let values = CanonicalSet::from_members(
                    p.values()
                        .iter()
                        .map(|(id, value)| match value {
                            Some(value) => DimensionValueV2Dto::Known(DimensionKnownV2Dto {
                                dimension: id.to_string(),
                                state: KnownState::Known,
                                value: value.to_string(),
                            }),
                            None => DimensionValueV2Dto::Unknown(DimensionUnknownV2Dto {
                                dimension: id.to_string(),
                                state: UnknownState::Unknown,
                            }),
                        })
                        .collect(),
                )
                .map_err(QueryPreparationError::Collection)?;
                Ok(QueryProfileV2 {
                    id: p.id().to_string(),
                    revision: WireUint::new(p.revision().get()),
                    values,
                })
            })
            .collect::<Result<Vec<_>, QueryPreparationError>>()?,
    )
    .map_err(QueryPreparationError::Collection)?;
    let context = ObligationQueryContextV2 {
        semantic_graph_digest: canonicalize_graph_v2(finalized.graph())
            .map_err(QueryPreparationError::Canonical)?
            .digest()
            .to_string(),
        scope_bindings_digest: canonicalize_scope_bindings_v2(u.scope_bindings())
            .map_err(QueryPreparationError::Canonical)?
            .digest()
            .to_string(),
        policy: QueryPolicyReferenceV2 {
            id: u.policy().id().to_string(),
            revision: WireUint::new(u.policy().revision().get()),
        },
        profiles,
        mode: match u.mode() {
            EvaluationMode::Development => QueryModeV2::Development,
            EvaluationMode::PullRequest => QueryModeV2::PullRequest,
            EvaluationMode::Release => QueryModeV2::Release,
        },
        authority_origin: match u.authority_origin() {
            AuthorityOrigin::CandidateLocal => QueryOriginV2::CandidateLocal,
            AuthorityOrigin::TrustedInvocation => QueryOriginV2::TrustedInvocation,
            AuthorityOrigin::ProtectedBaseline => QueryOriginV2::ProtectedBaseline,
        },
        release_context: u.release_context().map(|v| v.to_string()),
        offline,
        evaluated_at: CanonicalInstant::new(evaluated_at),
    };
    let mut records = Vec::new();
    for key in u.coordinates() {
        let identity = identities
            .identities()
            .get(key)
            .ok_or(QueryPreparationError::MissingIdentity)?;
        let coordinate = ObligationCoordinateV2Dto::from_json(identity.bytes())
            .map_err(QueryPreparationError::Protocol)?;
        if coordinate
            .reference()
            .map_err(QueryPreparationError::Protocol)?
            != identity.reference()
        {
            return Err(QueryPreparationError::MissingIdentity);
        }
        records.push(Record {
            view: ObligationViewV2Dto::Required {
                state: ObligationQueryState::Pending,
                reference: identity.reference(),
                coordinate: Box::new(coordinate),
            },
        });
    }
    let known = finalized
        .graph()
        .targets()
        .values()
        .map(|t| {
            Ok((
                t.id().clone(),
                canonicalize_target_configuration_v2(t)?.digest(),
            ))
        })
        .collect::<Result<BTreeMap<_, _>, CanonicalizationError>>()
        .map_err(QueryPreparationError::Canonical)?;
    for value in u.unresolved() {
        if &value.profiles != u.profiles() || value.release_context != u.release_context() {
            return Err(QueryPreparationError::MissingIdentity);
        }
        let subject = value
            .subject
            .as_ref()
            .map(|s| {
                let input = crate::obligation_identity::subject(finalized.graph(), s, &known)
                    .map_err(QueryPreparationError::Scope)?;
                let bytes = canonicalize_obligation_scope_v2(&input)
                    .map_err(QueryPreparationError::Canonical)?;
                ObligationQuerySubjectV2::from_json(&bytes).map_err(QueryPreparationError::Protocol)
            })
            .transpose()?;
        let reason = reason(value.reason);
        records.push(Record {
            view: ObligationViewV2Dto::Unresolved {
                state: if reason == ObligationQueryReason::NotApplicable {
                    ObligationQueryState::NotApplicable
                } else {
                    ObligationQueryState::Unknown
                },
                reason,
                unit: value.unit.to_string(),
                requirement: value.requirement.to_string(),
                profiles: context.profiles.clone(),
                subject: subject.map(Box::new),
                release_context: value.release_context.map(|v| v.to_string()),
            },
        });
    }
    let prepared = PreparedObligationQuery {
        identities: identities.clone(),
        context,
        records,
    };
    // Validate full serialization before any filter can hide malformed work or
    // make a complete envelope fit only by silently omitting required records.
    prepared
        .view(ObligationQueryFiltersV2 {
            unit: None,
            target: None,
            statuses: CanonicalSet::empty(),
        })?
        .to_json()
        .map_err(QueryPreparationError::Protocol)?;
    Ok(prepared)
}
impl PreparedObligationQuery {
    /// Original complete universe remains available independently of display filters.
    #[must_use]
    pub fn universe(&self) -> &eqm_engine::RequiredUniverse {
        self.identities.universe()
    }
    /// Returns a pure presentation view; context and full denominator never shrink.
    pub fn view(
        &self,
        filters: ObligationQueryFiltersV2,
    ) -> Result<ObligationsReportV2Dto, QueryPreparationError> {
        filters
            .validate()
            .map_err(QueryPreparationError::Protocol)?;
        let records = self
            .records
            .iter()
            .filter(|record| filters.matches(&record.view))
            .map(|r| r.view.clone())
            .collect::<Vec<_>>();
        let records =
            CanonicalSet::from_members(records).map_err(QueryPreparationError::Collection)?;
        let returned_count = WireUint::new(records.len() as u64);
        Ok(ObligationsReportV2Dto::Success {
            schema: QuerySchemaV2,
            tool_version: ToolVersionDto,
            command: ObligationQueryCommand::Obligations,
            context: Box::new(self.context.clone()),
            result: ObligationsResultV2Dto {
                filters,
                denominator: ObligationQueryDenominatorV2 {
                    required: WireUint::new(self.universe().coordinates().len() as u64),
                    unresolved: WireUint::new(self.universe().unresolved().len() as u64),
                },
                returned_count,
                presentation_filtered: records.len() != self.records.len(),
                records,
            },
        })
    }
}
