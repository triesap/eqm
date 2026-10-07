//! S052 real shared preparation: exact claims versus provider-owned scope.

use eqm_application::{
    AuthorityError, AuthorityProvider, AuthorityRequest, ObligationPreparationError,
    PolicyReference, PolicySelectionRequest, PreparedSession, ProfileSelection, ProtectedAuthority,
    ProtectedPreparationError, SessionRequest, prepare, prepare_obligations_from_request,
    prepare_protected_obligations,
};
use eqm_domain::{
    DimensionId, FullRequirementId, PolicyId, ProfileId, RequirementScenarioBindings, Revision,
    ScenarioId, ScenarioRef, SymbolicValueId,
};
use eqm_engine::{
    EvaluationMode, ObligationError, ScopeBindingFailure, ScopeSubject, UnresolvedReason,
};
use std::cell::Cell;
use std::error::Error;
use std::fs;
use std::path::Path;

const REQUIREMENT: &str = "account.create.signup.otp#valid_code_advances";

fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &destination.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn fixture() -> Result<(tempfile::TempDir, PreparedSession), Box<dyn Error>> {
    let root = tempfile::tempdir()?;
    copy_tree(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios"),
        root.path(),
    )?;
    fs::create_dir(root.path().join(".git"))?;
    for revision in [1, 2] {
        fs::write(
            root.path()
                .join(format!("eqm/contracts/scenario.{revision}.toml")),
            format!(
                r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/scenario.schema.json"
id = "scenario.preparation"
revision = {revision}
[[participants]]
id = "phone_one"
role = "edge"
[participants.subject]
target = "android"
[[participants]]
id = "phone_two"
role = "observer"
[participants.subject]
target = "android"
[[participants]]
id = "ios_actor"
role = "edge"
[participants.subject]
target = "ios"
[[dependencies]]
from = "phone_two"
to = "phone_one"
"#
            ),
        )?;
    }
    let session = prepare(
        &SessionRequest::new(None, "2026-01-02T03:04:05Z".parse()?),
        root.path(),
    )?;
    Ok((root, session))
}

fn map(requirement: &str, revision: u64) -> Result<RequirementScenarioBindings, Box<dyn Error>> {
    Ok(RequirementScenarioBindings::new(vec![(
        FullRequirementId::new(requirement)?,
        ScenarioRef::new(
            ScenarioId::new("scenario.preparation")?,
            Revision::new(revision)?,
        ),
    )])?)
}

fn input() -> Result<PolicySelectionRequest, Box<dyn Error>> {
    Ok(PolicySelectionRequest::development(
        Some(PolicyReference::new(
            PolicyId::new("consumer.critical_flow")?,
            Some(Revision::new(1)?),
        )),
        vec![ProfileSelection::new(
            ProfileId::new("audience.default")?,
            Some(Revision::new(1)?),
            vec![(
                DimensionId::new("region")?,
                Some(SymbolicValueId::new("us")?),
            )],
        )],
    ))
}

// Local scope-boundary probe; this provider is never a production trust adapter.
struct ScopeAuthority {
    graph: eqm_engine::FinalizedWorkspaceGraph,
    bindings: RequirementScenarioBindings,
    calls: Cell<usize>,
}

impl AuthorityProvider for ScopeAuthority {
    fn load_authority(
        &self,
        request: &AuthorityRequest,
    ) -> Result<ProtectedAuthority<'_>, AuthorityError> {
        assert_eq!(request.mode, EvaluationMode::Release);
        self.calls.set(self.calls.get() + 1);
        Ok(ProtectedAuthority::new(&self.graph, &self.bindings))
    }
}

#[test]
fn development_consumes_explicit_exact_scope_without_inferred_defaults()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let request = input()?.with_scope_bindings(map(REQUIREMENT, 1)?);
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request,
        None,
    )?;
    assert_eq!(
        prepared.universe().scope_bindings(),
        request.scope_bindings().ok_or("map missing")?
    );
    let scope = prepared
        .universe()
        .coordinates()
        .iter()
        .find_map(|key| match &key.subject {
            ScopeSubject::Scenario(scope) => Some(scope),
            _ => None,
        })
        .ok_or("scene absent")?;
    assert_eq!(scope.participants().len(), 3);
    assert_eq!(scope.dependencies().len(), 1);
    assert_eq!(scope.scenario().revision().get(), 1);
    let omitted = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &input()?,
        None,
    )?;
    assert!(!omitted.universe().coordinates().iter().any(|key| matches!(
        key.subject,
        ScopeSubject::Scenario(_) | ScopeSubject::TargetSet(_)
    )));
    assert!(
        omitted
            .universe()
            .unresolved()
            .iter()
            .any(|coordinate| coordinate.requirement.as_str() == REQUIREMENT
                && coordinate.reason == UnresolvedReason::MissingScopeAuthority)
    );
    Ok(())
}

#[test]
fn protected_map_is_loaded_once_and_candidate_omission_never_substitutes_scope()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let authority = ScopeAuthority {
        graph: session.finalized().clone(),
        bindings: map(REQUIREMENT, 2)?,
        calls: Cell::new(0),
    };
    let prepared = prepare_protected_obligations(
        session.finalized(),
        EvaluationMode::Release,
        &input()?,
        Some(&authority),
        None,
    )?;
    assert_eq!(authority.calls.get(), 1);
    assert_eq!(prepared.universe().scope_bindings(), &authority.bindings);
    assert!(
        prepared
            .universe()
            .coordinates()
            .iter()
            .any(|key| matches!(&key.subject,
        ScopeSubject::Scenario(scope) if scope.scenario().revision().get() == 2))
    );
    assert!(
        !prepared
            .universe()
            .coordinates()
            .iter()
            .any(|key| matches!(&key.subject,
        ScopeSubject::Scenario(scope) if scope.scenario().revision().get() == 1))
    );
    assert_eq!(
        prepared.selection().origin(),
        eqm_engine::AuthorityOrigin::ProtectedBaseline
    );
    assert_eq!(
        session.evaluation_time().to_string(),
        "2026-01-02T03:04:05Z"
    );
    Ok(())
}

#[test]
fn candidate_map_must_equal_protected_map_even_when_both_are_graph_valid()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let authority = ScopeAuthority {
        graph: session.finalized().clone(),
        bindings: map(REQUIREMENT, 1)?,
        calls: Cell::new(0),
    };
    let equal = input()?.with_scope_bindings(map(REQUIREMENT, 1)?);
    prepare_protected_obligations(
        session.finalized(),
        EvaluationMode::Release,
        &equal,
        Some(&authority),
        None,
    )?;
    assert_eq!(authority.calls.get(), 1);
    let other = input()?.with_scope_bindings(map(REQUIREMENT, 2)?);
    let rejected = prepare_protected_obligations(
        session.finalized(),
        EvaluationMode::Release,
        &other,
        Some(&authority),
        None,
    )
    .err()
    .ok_or("candidate replaced protected scope")?;
    assert!(matches!(
        rejected,
        ProtectedPreparationError::CandidateScopeMismatch
    ));
    assert_eq!(authority.calls.get(), 2);
    let empty = input()?.with_scope_bindings(RequirementScenarioBindings::new(Vec::new())?);
    let rejected = prepare_protected_obligations(
        session.finalized(),
        EvaluationMode::Release,
        &empty,
        Some(&authority),
        None,
    )
    .err()
    .ok_or("empty candidate erased protected scope")?;
    assert!(matches!(
        rejected,
        ProtectedPreparationError::CandidateScopeMismatch
    ));
    assert_eq!(authority.calls.get(), 3);
    Ok(())
}

#[test]
fn invalid_candidate_binding_rejects_before_missing_or_configured_provider()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let authority = ScopeAuthority {
        graph: session.finalized().clone(),
        bindings: map(REQUIREMENT, 1)?,
        calls: Cell::new(0),
    };
    for (requirement, revision, reason) in [
        (REQUIREMENT, 3, ScopeBindingFailure::UnknownScenario),
        (
            "account.create.signup.identifier#email_default",
            1,
            ScopeBindingFailure::NotEndToEnd,
        ),
        (
            "account.create.signup.otp#missing",
            1,
            ScopeBindingFailure::UnknownRequirement,
        ),
    ] {
        let invalid = input()?.with_scope_bindings(map(requirement, revision)?);
        for provider in [None, Some(&authority as &dyn AuthorityProvider)] {
            let rejected = prepare_protected_obligations(
                session.finalized(),
                EvaluationMode::Release,
                &invalid,
                provider,
                None,
            )
            .err()
            .ok_or("bad candidate map admitted")?;
            let ProtectedPreparationError::Preparation(ObligationPreparationError::Obligations(
                ObligationError::InvalidScopeBinding {
                    requirement: actual,
                    scenario,
                    reason: actual_reason,
                },
            )) = rejected
            else {
                return Err("typed binding cause lost".into());
            };
            assert_eq!(actual.as_str(), requirement);
            assert_eq!(scenario.revision().get(), revision);
            assert_eq!(actual_reason, reason);
        }
    }
    assert_eq!(authority.calls.get(), 0);
    Ok(())
}

#[test]
fn invalid_or_empty_protected_map_cannot_create_implicit_scene_authority()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let invalid = ScopeAuthority {
        graph: session.finalized().clone(),
        bindings: map(REQUIREMENT, 3)?,
        calls: Cell::new(0),
    };
    let rejected = prepare_protected_obligations(
        session.finalized(),
        EvaluationMode::Release,
        &input()?,
        Some(&invalid),
        None,
    )
    .err()
    .ok_or("bad authority map admitted")?;
    assert!(matches!(
        rejected,
        ProtectedPreparationError::Preparation(ObligationPreparationError::Obligations(
            ObligationError::InvalidScopeBinding {
                reason: ScopeBindingFailure::UnknownScenario,
                ..
            }
        ))
    ));
    assert_eq!(invalid.calls.get(), 1);
    let empty = ScopeAuthority {
        graph: session.finalized().clone(),
        bindings: RequirementScenarioBindings::new(Vec::new())?,
        calls: Cell::new(0),
    };
    let incomplete = prepare_protected_obligations(
        session.finalized(),
        EvaluationMode::Release,
        &input()?,
        Some(&empty),
        None,
    )?;
    assert_eq!(empty.calls.get(), 1);
    assert!(
        incomplete
            .universe()
            .unresolved()
            .iter()
            .any(|coordinate| coordinate.requirement.as_str() == REQUIREMENT
                && coordinate.reason == UnresolvedReason::MissingScopeAuthority)
    );
    assert!(
        !incomplete
            .universe()
            .coordinates()
            .iter()
            .any(|key| matches!(
                key.subject,
                ScopeSubject::Scenario(_) | ScopeSubject::TargetSet(_)
            ))
    );
    Ok(())
}
