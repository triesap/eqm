//! Shared exact development policy/profile preparation.

use crate::session::PreparedSession;
use eqm_domain::{DimensionId, ProfileId, SymbolicValueId};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, EvaluationMode, Obligation, ObligationDerivation,
    PolicyProfileRequest, PolicyRef, ProfileRequest, ScopeSubject, SelectedPolicyProfiles,
    derive_obligations, select_policy_profiles,
};
use eqm_protocol::{FacetStatusDto, ObligationDto, ProfileValueDto};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::error::Error;

type ProfileValues = Vec<(DimensionId, Option<SymbolicValueId>)>;

/// Selects complete development authority and derives obligations.
pub fn derive<'a>(
    session: &'a PreparedSession,
    requested: &[String],
) -> Result<(SelectedPolicyProfiles<'a>, ObligationDerivation), Box<dyn Error>> {
    let graph = session.finalized().graph();
    let policies = graph.policies().values().collect::<Vec<_>>();
    if policies.len() != 1 {
        return Err("exactly one development policy revision is required".into());
    }
    let policy = policies[0];
    let requested = requested
        .iter()
        .map(|value| parse_profile(value))
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    if requested.keys().any(|id| !policy.profiles().contains(id)) {
        return Err("selected profile is outside the policy profile set".into());
    }
    let profiles = policy
        .profiles()
        .iter()
        .map(|id| {
            let matches = graph
                .profiles()
                .iter()
                .filter(|((candidate, _), _)| candidate == id)
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                return Err("profile authority did not resolve uniquely".into());
            }
            let ((_, revision), _) = matches[0];
            Ok(ProfileRequest::new(
                id.clone(),
                *revision,
                requested.get(id).cloned().unwrap_or_default(),
            )?)
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let request = PolicyProfileRequest::new(
        AuthorityOrigin::CandidateLocal,
        PolicyRef::new(policy.id().clone(), policy.revision()),
        profiles,
    )?;
    let selection =
        select_policy_profiles(graph, EvaluationMode::Development, Some(&request), None)?;
    let applicability = ApplicabilityContext::from_profiles(
        selection
            .profiles()
            .values()
            .map(|selected| {
                graph
                    .profiles()
                    .get(&(selected.id().clone(), selected.revision()))
                    .map(|profile| (profile, selected.values()))
                    .ok_or("selected profile authority")
            })
            .collect::<Result<Vec<_>, _>>()?,
    )?;
    let obligations = derive_obligations(session.finalized(), &selection, &applicability, None)?;
    Ok((selection, obligations))
}

fn parse_profile(value: &str) -> Result<(ProfileId, ProfileValues), Box<dyn Error>> {
    let (id, assignments) = value.split_once('=').unwrap_or((value, ""));
    let values = if assignments.is_empty() {
        Vec::new()
    } else {
        assignments
            .split(',')
            .map(|assignment| {
                let (dimension, value) = assignment.split_once(':').ok_or("profile assignment")?;
                Ok((
                    DimensionId::new(dimension)?,
                    Some(SymbolicValueId::new(value)?),
                ))
            })
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?
    };
    Ok((ProfileId::new(id)?, values))
}

/// Converts one current obligation to its stable missing-evidence query shape.
pub fn obligation_dto(obligation: &Obligation) -> Result<ObligationDto, Box<dyn Error>> {
    let (scope, subject) = match &obligation.key.subject {
        ScopeSubject::Target(value) => ("each_target", format!("target:{value}")),
        ScopeSubject::Provider(value) => ("shared_provider", format!("provider:{value}")),
        ScopeSubject::TargetSet(values) => (
            "end_to_end",
            format!(
                "targets:{}",
                values
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        ),
    };
    let mut profile_values = Vec::new();
    for selected in obligation.key.profiles.values() {
        for (dimension, value) in selected.values() {
            let value = value.as_ref().ok_or("unknown selected profile value")?;
            profile_values.push(ProfileValueDto::from_parts(
                selected.id(),
                selected.revision(),
                dimension,
                value,
            ));
        }
    }
    Ok(ObligationDto {
        id: obligation_id(obligation),
        policy: format!(
            "{}@{}",
            obligation.key.policy, obligation.key.policy_revision
        ),
        profile_values,
        unit: obligation.key.unit.to_string(),
        requirement: obligation.key.requirement.to_string(),
        scope: scope.to_owned(),
        scope_subject: subject,
        facet: obligation.key.facet.to_string(),
        minimum_trust: obligation.strength.minimum_trust.to_string(),
        maximum_age_ms: obligation.strength.maximum_age.get(),
        minimum_count: obligation.strength.minimum_count.get(),
        status: FacetStatusDto::Missing,
        evidence: BTreeSet::new(),
        waiver: None,
    })
}

/// Returns the exact stable obligation coordinate.
pub fn obligation_id(obligation: &Obligation) -> String {
    let subject = match &obligation.key.subject {
        ScopeSubject::Target(value) => format!("target:{value}"),
        ScopeSubject::Provider(value) => format!("provider:{value}"),
        ScopeSubject::TargetSet(values) => format!(
            "targets:{}",
            values
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        ),
    };
    format!(
        "{}@{}:{}:{}:{}:{}",
        obligation.key.policy,
        obligation.key.policy_revision.get(),
        obligation.key.unit,
        obligation.key.requirement,
        subject,
        obligation.key.facet
    )
}
