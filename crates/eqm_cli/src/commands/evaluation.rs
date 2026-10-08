//! Thin CLI adapter over application-owned policy selection preparation.

use crate::session::PreparedSession;
use eqm_application::{
    PolicyReference, PolicySelectionRequest, PreparedObligationIdentities, ProfileSelection,
    prepare_expected_selections, prepare_obligation_identities, prepare_obligations_from_request,
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
use std::ops::Deref;

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

/// Immutable full derivation plus complete shared identity map, before filtering.
pub struct PreparedQueryDerivation {
    derivation: ObligationDerivation,
    identities: PreparedObligationIdentities,
}
impl Deref for PreparedQueryDerivation {
    type Target = ObligationDerivation;
    fn deref(&self) -> &Self::Target {
        &self.derivation
    }
}

/// Prepares development authority through the shared exact request operation.
pub fn derive_with_policy<'a>(
    session: &'a PreparedSession,
    policy: Option<&str>,
    requested: &[String],
    scenarios: &[String],
) -> Result<(SelectedPolicyProfiles<'a>, PreparedQueryDerivation), Box<dyn Error>> {
    let input = selection_request_with_scopes(policy, requested, scenarios)?;
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &input,
        None,
    )?;
    let expected = prepare_expected_selections(session.finalized(), &prepared, None, None)?;
    let identities = prepare_obligation_identities(session.finalized(), &expected)?;
    let (selection, _applicability, derivation, _universe) = prepared.into_parts();
    Ok((
        selection,
        PreparedQueryDerivation {
            derivation,
            identities,
        },
    ))
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
pub fn obligation_dto(
    prepared: &PreparedQueryDerivation,
    obligation: &Obligation,
) -> Result<ObligationDto, Box<dyn Error>> {
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
        id: obligation_id(prepared, obligation)?,
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

/// Returns the complete versioned reference from this immutable prepared query.
/// A foreign strength/coordinate or absent entry rejects, never a label fallback.
pub fn obligation_id(
    prepared: &PreparedQueryDerivation,
    obligation: &Obligation,
) -> Result<String, Box<dyn Error>> {
    if prepared.derivation.obligations.get(&obligation.key) != Some(obligation) {
        return Err("obligation does not belong to the prepared query".into());
    }
    Ok(prepared
        .identities
        .identities()
        .get(&obligation.key)
        .ok_or("prepared obligation identity is missing")?
        .reference())
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

    fn query(
        root: &std::path::Path,
        arguments: &[&str],
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        let crate::cli::ParseOutcome::Run(parsed) = crate::cli::parse(arguments.iter().copied())?
        else {
            return Err("query did not parse as a command".into());
        };
        Ok(crate::commands::execute(parsed, root)?.payload.json)
    }

    fn references(
        value: &serde_json::Value,
        field: &str,
    ) -> Result<BTreeSet<String>, Box<dyn Error>> {
        let values = value.as_array().ok_or("expected reference array")?;
        values
            .iter()
            .map(|value| {
                let value = if field.is_empty() {
                    value
                } else {
                    &value[field]
                };
                Ok(value
                    .as_str()
                    .ok_or("expected reference string")?
                    .to_owned())
            })
            .collect()
    }

    #[test]
    fn actual_queries_share_complete_references_before_presentation_filters()
    -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let instant: UtcInstant = "2026-01-02T03:04:05Z".parse()?;
        let session = prepare(
            &SessionRequest::at(global(), CommandName::Obligations, instant),
            repository.path(),
        )?;
        let (_, prepared) = derive_with_policy(&session, None, &[], &[])?;
        let all = query(repository.path(), &["obligations"])?;
        let all_refs = references(&all["result"]["obligations"], "id")?;
        assert!(!all_refs.is_empty());
        let prepared_refs = prepared
            .obligations
            .values()
            .map(|value| obligation_id(&prepared, value))
            .collect::<Result<BTreeSet<_>, _>>()?;
        assert_eq!(all_refs, prepared_refs);
        assert!(
            all_refs
                .iter()
                .all(|value| value.starts_with("eqm:v2:obligation:sha256:"))
        );

        let first = prepared
            .obligations
            .values()
            .next()
            .ok_or("positive coordinate absent")?;
        let unit = first.key.unit.as_str();
        let ScopeSubject::Target(target) = &first.key.subject else {
            return Err("example must have a target coordinate".into());
        };
        let subset = prepared
            .obligations
            .values()
            .filter(|value| {
                value.key.unit.as_str() == unit && value.key.subject == first.key.subject
            })
            .map(|value| obligation_id(&prepared, value))
            .collect::<Result<BTreeSet<_>, _>>()?;
        assert!(!subset.is_empty());
        assert!(subset.len() < all_refs.len());
        let filtered = query(
            repository.path(),
            &["obligations", "--unit", unit, "--target", target.as_str()],
        )?;
        assert_eq!(
            references(&filtered["result"]["obligations"], "id")?,
            subset
        );
        let context = query(
            repository.path(),
            &[
                "context",
                unit,
                "--target",
                target.as_str(),
                "--max-bytes",
                "1000000",
            ],
        )?;
        assert_eq!(references(&context["result"]["obligations"], "id")?, subset);

        let check = query(repository.path(), &["check"])?;
        let finding_refs = check["result"]["findings"]
            .as_array()
            .ok_or("findings")?
            .iter()
            .filter(|value| value["diagnostic_code"] == "EQM-E0500")
            .map(|value| {
                Ok(value["obligation"]
                    .as_str()
                    .ok_or("finding reference")?
                    .to_owned())
            })
            .collect::<Result<BTreeSet<_>, Box<dyn Error>>>()?;
        assert_eq!(finding_refs, all_refs);
        for args in [
            vec!["matrix", "conformance"],
            vec![
                "matrix",
                "conformance",
                "--unit",
                unit,
                "--target",
                target.as_str(),
            ],
        ] {
            let matrix = query(repository.path(), &args)?;
            let refs = matrix["result"]["cells"]
                .as_array()
                .ok_or("matrix cells")?
                .iter()
                .map(|cell| references(&cell["obligations"], ""))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect::<BTreeSet<_>>();
            assert_eq!(
                refs,
                if args.len() == 2 {
                    all_refs.clone()
                } else {
                    subset.clone()
                }
            );
        }

        let binding = session
            .finalized()
            .graph()
            .bindings()
            .values()
            .find(|binding| binding.unit().as_str() == unit && binding.target() == target)
            .ok_or("example binding")?;
        let artifact = binding
            .artifacts()
            .values()
            .values()
            .next()
            .ok_or("example artifact")?;
        let baseline = session.workspace_digest().to_string();
        let affected = query(
            repository.path(),
            &[
                "--baseline",
                &baseline,
                "affected",
                "--path",
                artifact.path().as_str(),
            ],
        )?;
        let affected_refs = references(&affected["result"]["obligations"], "")?;
        assert!(!affected_refs.is_empty());
        assert!(affected_refs.is_subset(&all_refs));
        assert!(affected_refs.contains(&obligation_id(&prepared, first)?));

        // Neither hiding missing rows nor rejecting foreign strengths mutates the universe.
        let hidden = query(repository.path(), &["obligations", "--status", "satisfied"])?;
        assert!(references(&hidden["result"]["obligations"], "id")?.is_empty());
        assert_eq!(prepared.obligations.len(), all_refs.len());
        let mut foreign = first.clone();
        foreign.strength.minimum_count =
            eqm_domain::PositiveCount::new(first.strength.minimum_count.get() + 1)?;
        assert!(obligation_id(&prepared, &foreign).is_err());
        Ok(())
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
