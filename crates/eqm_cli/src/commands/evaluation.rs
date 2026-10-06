//! Thin CLI adapter over application-owned policy selection preparation.

use crate::session::PreparedSession;
use eqm_application::{
    PolicyReference, PolicySelectionRequest, ProfileSelection, prepare_obligations,
    resolve_selection_request,
};
use eqm_domain::{DimensionId, PolicyId, ProfileId, Revision, SymbolicValueId};
use eqm_engine::{
    EvaluationMode, Obligation, ObligationDerivation, PolicyProfileRequest, RequiredUniverse,
    ScopeSubject, SelectedPolicyProfiles,
};
use eqm_protocol::{FacetStatusDto, ObligationDto, ProfileValueDto};
use std::collections::BTreeSet;
use std::error::Error;

/// Parses CLI policy/profile arguments and resolves them through the shared
/// application operation.
///
/// This adapter only parses transport syntax into a typed
/// [`PolicySelectionRequest`]; exact revision resolution, ambiguity rejection
/// and policy membership are owned by
/// [`eqm_application::resolve_selection_request`] over the one immutable session.
pub fn development_request(
    session: &PreparedSession,
    policy: Option<&str>,
    requested: &[String],
) -> Result<PolicyProfileRequest, Box<dyn Error>> {
    let input = development_selection(policy, requested)?;
    Ok(resolve_selection_request(session.finalized(), &input)?)
}

/// Parses CLI policy/profile arguments into the shared typed selection request.
pub fn development_selection(
    policy: Option<&str>,
    requested: &[String],
) -> Result<PolicySelectionRequest, Box<dyn Error>> {
    let policy = policy.map(parse_policy).transpose()?;
    let profiles = requested
        .iter()
        .map(|value| parse_profile(value))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    Ok(PolicySelectionRequest::development(policy, profiles))
}

/// Selects development authority for an optional explicit policy reference.
pub fn derive_with_policy<'a>(
    session: &'a PreparedSession,
    policy: Option<&str>,
    requested: &[String],
) -> Result<(SelectedPolicyProfiles<'a>, ObligationDerivation), Box<dyn Error>> {
    let request = development_request(session, policy, requested)?;
    let prepared = prepare_obligations(
        session.finalized(),
        EvaluationMode::Development,
        Some(&request),
        None,
        None,
    )?;
    let (selection, _applicability, derivation, _universe) = prepared.into_parts();
    Ok((selection, derivation))
}

/// Selects development authority and also returns the checked required
/// universe for callers that must validate completeness before a decision.
pub fn derive_with_policy_universe<'a>(
    session: &'a PreparedSession,
    policy: Option<&str>,
    requested: &[String],
) -> Result<
    (
        SelectedPolicyProfiles<'a>,
        ObligationDerivation,
        RequiredUniverse,
    ),
    Box<dyn Error>,
> {
    let request = development_request(session, policy, requested)?;
    let prepared = prepare_obligations(
        session.finalized(),
        EvaluationMode::Development,
        Some(&request),
        None,
        None,
    )?;
    let (selection, _applicability, derivation, universe) = prepared.into_parts();
    Ok((selection, derivation, universe))
}

fn parse_policy(value: &str) -> Result<PolicyReference, Box<dyn Error>> {
    let (id, revision) = match value.split_once('@') {
        Some((id, revision)) => (id, Some(parse_revision(revision)?)),
        None => (value, None),
    };
    Ok(PolicyReference::new(PolicyId::new(id)?, revision))
}

fn parse_profile(value: &str) -> Result<ProfileSelection, Box<dyn Error>> {
    let (head, assignments) = value.split_once('=').unwrap_or((value, ""));
    let (id, revision) = match head.split_once('@') {
        Some((id, revision)) => (id, Some(parse_revision(revision)?)),
        None => (head, None),
    };
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
    Ok(ProfileSelection::new(ProfileId::new(id)?, revision, values))
}

fn parse_revision(value: &str) -> Result<Revision, Box<dyn Error>> {
    let revision = value
        .parse::<u64>()
        .map_err(|_| "revision must be a positive integer")?;
    Ok(Revision::new(revision)?)
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
            policy: None,
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
            None,
            &[
                "audience.default=region:eu".to_owned(),
                "audience.default=region:us".to_owned(),
            ],
        )
        .err()
        .ok_or("duplicate profile selection was admitted")?;
        assert!(error.to_string().contains("more than once"), "{error}");

        // A distinct complete selection still resolves and derives obligations.
        let single =
            development_request(&session, None, &["audience.default=region:eu".to_owned()])?;
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
            None,
            &["audience.default=region:eu,region:us".to_owned()],
        )
        .err()
        .ok_or("duplicate profile dimension was admitted")?;
        assert!(error.to_string().contains("DuplicateDimension"), "{error}");
        Ok(())
    }

    #[test]
    fn explicit_policy_and_profile_revisions_resolve_exactly() -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let instant: UtcInstant = "2026-01-02T03:04:05Z".parse()?;
        let session = prepare(
            &SessionRequest::at(global(), CommandName::Context, instant),
            repository.path(),
        )?;
        // The exact revision resolves through the explicit reference.
        let request = development_request(
            &session,
            Some("consumer.critical_flow@1"),
            &["audience.default@1=region:eu".to_owned()],
        )?;
        let prepared = prepare_obligations(
            session.finalized(),
            EvaluationMode::Development,
            Some(&request),
            None,
            None,
        )?;
        assert!(!prepared.derivation().obligations.is_empty());

        // A missing exact revision is not silently replaced by a latest one.
        let policy_error = development_request(&session, Some("consumer.critical_flow@2"), &[])
            .err()
            .ok_or("missing policy revision was admitted")?;
        assert!(
            policy_error.to_string().contains("not found"),
            "{policy_error}"
        );
        let profile_error =
            development_request(&session, None, &["audience.default@9=region:eu".to_owned()])
                .err()
                .ok_or("missing profile revision was admitted")?;
        assert!(
            profile_error.to_string().contains("not found"),
            "{profile_error}"
        );
        Ok(())
    }
}
