//! Thin CLI adapter over application-owned policy selection preparation.

use crate::session::PreparedSession;
use eqm_application::{
    PolicyReference, PolicySelectionRequest, ProfileSelection, prepare_obligations_from_request,
};
use eqm_domain::{
    DimensionId, FullRequirementId, PolicyId, ProfileId, RequirementScenarioBindings, Revision,
    ScenarioId, ScenarioRef, SymbolicValueId,
};
use eqm_engine::{
    EvaluationMode, Obligation, ObligationDerivation, ScopeSubject, SelectedPolicyProfiles,
};
use eqm_protocol::{FacetStatusDto, ObligationDto, ProfileValueDto};
use std::collections::BTreeSet;
use std::error::Error;

#[cfg(test)]
use eqm_application::{prepare_obligations, resolve_selection_request};
#[cfg(test)]
use eqm_engine::PolicyProfileRequest;

/// Parses CLI policy/profile arguments and resolves them through the shared
/// application operation.
///
/// This adapter only parses transport syntax into a typed
/// [`PolicySelectionRequest`]; exact revision resolution, ambiguity rejection
/// and policy membership are owned by
/// [`eqm_application::resolve_selection_request`] over the one immutable session.
#[cfg(test)]
pub fn development_request(
    session: &PreparedSession,
    policy: Option<&str>,
    requested: &[String],
) -> Result<PolicyProfileRequest, Box<dyn Error>> {
    let input = selection_request(policy, requested)?;
    Ok(resolve_selection_request(
        session.finalized(),
        EvaluationMode::Development,
        &input,
    )?)
}

/// Parses CLI policy/profile arguments into the shared typed selection request.
pub fn selection_request(
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

/// Parses repeatable exact scope claims without semantic graph lookup.
pub fn selection_request_with_scopes(
    policy: Option<&str>,
    requested: &[String],
    scenarios: &[String],
) -> Result<PolicySelectionRequest, Box<dyn Error>> {
    let input = selection_request(policy, requested)?;
    if scenarios.is_empty() {
        return Ok(input);
    }
    let entries = scenarios
        .iter()
        .map(|value| {
            let (requirement, reference) = value
                .split_once('=')
                .ok_or("scenario selection requires FULL_REQUIREMENT=SCENARIO_ID@REVISION")?;
            let (id, revision) = reference
                .split_once('@')
                .ok_or("scenario selection requires an exact revision")?;
            Ok((
                FullRequirementId::new(requirement)?,
                ScenarioRef::new(ScenarioId::new(id)?, parse_revision(revision)?),
            ))
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    Ok(input.with_scope_bindings(RequirementScenarioBindings::new(entries)?))
}

/// Prepares development authority through the shared exact request operation.
pub fn derive_with_policy<'a>(
    session: &'a PreparedSession,
    policy: Option<&str>,
    requested: &[String],
    scenarios: &[String],
) -> Result<(SelectedPolicyProfiles<'a>, ObligationDerivation), Box<dyn Error>> {
    let input = selection_request_with_scopes(policy, requested, scenarios)?;
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &input,
        None,
    )?;
    let (selection, _applicability, derivation, _universe) = prepared.into_parts();
    Ok((selection, derivation))
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
    let scope = match &obligation.key.subject {
        ScopeSubject::Target(_) => "each_target",
        ScopeSubject::Provider(_) | ScopeSubject::CheckedProvider(_) => "shared_provider",
        ScopeSubject::TargetSet(_) | ScopeSubject::Scenario(_) => "end_to_end",
    };
    let subject = scope_label(&obligation.key.subject);
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
    let subject = scope_label(&obligation.key.subject);
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

/// Descriptive unresolved coordinate reference; it is not an obligation digest.
pub fn unresolved_label(coordinate: &eqm_engine::UnresolvedCoordinate) -> String {
    let subject = match &coordinate.subject {
        None => "absent-scope-authority".to_owned(),
        Some(ScopeSubject::Target(id)) => format!("target:{id}"),
        Some(ScopeSubject::Provider(id)) => format!("provider:{id}"),
        Some(ScopeSubject::TargetSet(_)) => "reference-target-set".to_owned(),
        Some(ScopeSubject::CheckedProvider(scope)) => format!("provider:{}", scope.provider()),
        Some(ScopeSubject::Scenario(scope)) => format!(
            "scenario:{}@{}",
            scope.scenario().id(),
            scope.scenario().revision()
        ),
    };
    format!(
        "unresolved:{}:{}:{subject}",
        coordinate.unit, coordinate.requirement
    )
}

/// Keeps every non-authorizing unresolved coordinate visible to report adapters.
/// No evidence, runner, clock or authority effect is performed here.
pub fn scope_diagnostics(
    derived: &ObligationDerivation,
) -> Result<Vec<eqm_domain::Diagnostic>, Box<dyn Error>> {
    derived
        .unresolved
        .iter()
        .filter(|coordinate| {
            matches!(
                coordinate.reason,
                eqm_engine::UnresolvedReason::UnknownApplicability
                    | eqm_engine::UnresolvedReason::UnknownParticipation
                    | eqm_engine::UnresolvedReason::MissingScopeAuthority
            )
        })
        .map(|coordinate| {
            Ok(eqm_domain::Diagnostic::new(
                eqm_domain::DiagnosticCode::from_number(501).ok_or("diagnostic code")?,
                eqm_domain::Severity::Error,
                format!(
                    "required scope `{}` is unresolved: {:?}",
                    unresolved_label(coordinate),
                    coordinate.reason
                ),
                None,
                Vec::new(),
                Some("Supply complete exact scope, participation and profile authority.".into()),
            )?)
        })
        .collect()
}

/// Descriptive exact scope details; this label is not an authorizing digest.
pub fn scope_label(subject: &ScopeSubject) -> String {
    match subject {
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
        ScopeSubject::CheckedProvider(scope) => format!(
            "provider:{};dependers:[{}];assignments:[{}];dependencies:[{}]",
            scope.provider(),
            scope
                .dependers()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
            scope
                .assignments()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
            scope
                .dependencies()
                .iter()
                .map(|(from, to)| format!("{from}>{to}"))
                .collect::<Vec<_>>()
                .join(",")
        ),
        ScopeSubject::Scenario(scope) => format!(
            "scenario:{}@{};participants:[{}];dependencies:[{}]",
            scope.scenario().id(),
            scope.scenario().revision(),
            scope
                .participants()
                .iter()
                .map(|participant| {
                    let actor = match participant.subject() {
                        eqm_domain::ScenarioSubject::Target(id) => format!("target:{id}"),
                        eqm_domain::ScenarioSubject::Provider(id) => format!("provider:{id}"),
                    };
                    format!(
                        "{}/{}/{actor}",
                        participant.occurrence(),
                        participant.role()
                    )
                })
                .collect::<Vec<_>>()
                .join(","),
            scope
                .dependencies()
                .iter()
                .map(|edge| format!("{}>{}", edge.from(), edge.to()))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
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
            scenarios: Vec::new().into_boxed_slice(),
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
    #[test]
    fn scenario_transport_and_graph_validation_are_exact() -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        std::fs::write(
            repository.path().join("eqm/contracts/scenario.cli.toml"),
            "schema = \"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/scenario.schema.json\"\nid = \"scenario.cli\"\nrevision = 1\n[[participants]]\nid = \"actor\"\nrole = \"edge\"\n[participants.subject]\ntarget = \"android\"\n[[participants]]\nid = \"observer\"\nrole = \"observer\"\n[participants.subject]\ntarget = \"android\"\n[[dependencies]]\nfrom = \"observer\"\nto = \"actor\"\n",
        )?;
        let session = prepare(
            &SessionRequest::at(
                global(),
                CommandName::Context,
                "2026-01-02T03:04:05Z".parse()?,
            ),
            repository.path(),
        )?;
        let valid = "account.create.signup.otp#valid_code_advances=scenario.cli@1".to_owned();
        let (_, positive) = derive_with_policy(&session, None, &[], std::slice::from_ref(&valid))?;
        assert!(positive.obligations.keys().any(|key| matches!(&key.subject,
            ScopeSubject::Scenario(scope) if scope.participants().len() == 2 && scope.dependencies().len() == 1)));
        for invalid in [
            "account.create.signup.otp#valid_code_advances=scenario.cli",
            "account.create.signup.otp#valid_code_advances=scenario.cli@0",
            "account.create.signup.otp#valid_code_advances=scenario.cli@2",
            "account.create.signup.otp#valid_code_advances=scenario.missing@1",
            "account.create.signup.otp#missing=scenario.cli@1",
            "account.create.signup.identifier#email_default=scenario.cli@1",
        ] {
            assert!(
                derive_with_policy(&session, None, &[], &[invalid.to_owned()]).is_err(),
                "{invalid}"
            );
        }
        for second in [
            valid.clone(),
            "account.create.signup.otp#valid_code_advances=scenario.cli@2".to_owned(),
        ] {
            assert!(selection_request_with_scopes(None, &[], &[valid.clone(), second]).is_err());
        }
        let (_, omitted) = derive_with_policy(&session, None, &[], &[])?;
        assert!(
            omitted.unresolved.iter().any(|coordinate| coordinate.reason
                == eqm_engine::UnresolvedReason::MissingScopeAuthority)
        );
        Ok(())
    }
}
