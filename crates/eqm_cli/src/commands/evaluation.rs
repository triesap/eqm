//! Thin CLI adapter over application-owned policy selection preparation.

use crate::session::PreparedSession;
use eqm_application::prepare_obligations;
use eqm_domain::{DimensionId, ProfileId, SymbolicValueId};
use eqm_engine::{
    AuthorityOrigin, EvaluationMode, Obligation, ObligationDerivation, PolicyProfileRequest,
    PolicyRef, ProfileRequest, ScopeSubject, SelectedPolicyProfiles,
};
use eqm_protocol::{FacetStatusDto, ObligationDto, ProfileValueDto};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::error::Error;

type ProfileValues = Vec<(DimensionId, Option<SymbolicValueId>)>;

/// Builds the explicit development request from CLI profile arguments.
///
/// Reusing the application selection operation keeps authority preparation out
/// of the calling command; this adapter only parses transport arguments and
/// resolves the one development policy revision.
pub fn development_request(
    session: &PreparedSession,
    requested: &[String],
) -> Result<PolicyProfileRequest, Box<dyn Error>> {
    let graph = session.finalized().graph();
    let policies = graph.policies().values().collect::<Vec<_>>();
    if policies.len() != 1 {
        return Err("exactly one development policy revision is required".into());
    }
    let policy = policies[0];
    let mut selected: BTreeMap<ProfileId, ProfileValues> = BTreeMap::new();
    for value in requested {
        let (id, values) = parse_profile(value)?;
        if selected.insert(id.clone(), values).is_some() {
            return Err(format!("profile `{id}` was selected more than once").into());
        }
    }
    if selected.keys().any(|id| !policy.profiles().contains(id)) {
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
                selected.get(id).cloned().unwrap_or_default(),
            )?)
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    Ok(PolicyProfileRequest::new(
        AuthorityOrigin::CandidateLocal,
        PolicyRef::new(policy.id().clone(), policy.revision()),
        profiles,
    )?)
}

/// Selects complete development authority and derives obligations through the
/// application operation with an explicit development mode and authority.
pub fn derive<'a>(
    session: &'a PreparedSession,
    requested: &[String],
) -> Result<(SelectedPolicyProfiles<'a>, ObligationDerivation), Box<dyn Error>> {
    let request = development_request(session, requested)?;
    let prepared = prepare_obligations(
        session.finalized(),
        EvaluationMode::Development,
        Some(&request),
        None,
        None,
    )?;
    let (selection, _applicability, derivation) = prepared.into_parts();
    Ok((selection, derivation))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{ColorWhen, CommandName, GlobalOptions, OutputFormat};
    use crate::session::{SessionRequest, prepare};
    use eqm_domain::UtcInstant;
    use std::error::Error;

    fn global() -> GlobalOptions {
        GlobalOptions {
            config: None,
            profiles: Vec::new(),
            format: OutputFormat::Json,
            offline: true,
            no_progress: true,
            color: ColorWhen::Never,
            baseline: None,
            output: None,
        }
    }

    #[test]
    fn duplicate_profile_selection_rejects_at_the_transport_adapter() -> Result<(), Box<dyn Error>>
    {
        let repository = crate::test_support::example_repository()?;
        let instant: UtcInstant = "2026-01-02T03:04:05Z".parse()?;
        let session = prepare(
            &SessionRequest::at(global(), CommandName::Context, instant),
            repository.path(),
        )?;
        let error = development_request(
            &session,
            &[
                "audience.default=region:eu".to_owned(),
                "audience.default=region:us".to_owned(),
            ],
        )
        .err()
        .ok_or("duplicate profile selection was admitted")?;
        assert!(error.to_string().contains("more than once"), "{error}");

        // A distinct complete selection still resolves and derives obligations.
        let single = development_request(&session, &["audience.default=region:eu".to_owned()])?;
        let prepared = prepare_obligations(
            session.finalized(),
            EvaluationMode::Development,
            Some(&single),
            None,
            None,
        )?;
        assert!(!prepared.derivation().obligations.is_empty());
        Ok(())
    }

    #[test]
    fn duplicate_profile_dimensions_reject_before_selection() -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let instant: UtcInstant = "2026-01-02T03:04:05Z".parse()?;
        let session = prepare(
            &SessionRequest::at(global(), CommandName::Context, instant),
            repository.path(),
        )?;
        let error = development_request(
            &session,
            &["audience.default=region:eu,region:us".to_owned()],
        )
        .err()
        .ok_or("duplicate profile dimension was admitted")?;
        assert!(error.to_string().contains("DuplicateDimension"), "{error}");
        Ok(())
    }
}
