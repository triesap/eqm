//! S044: application-owned selection orchestration with explicit mode/authority.
//!
//! These tests drive the real example workspace through `prepare` and the new
//! application operation, proving that the development baseline is explicit and
//! that protected modes can never take an implicit development branch or a
//! candidate-local authority.

use eqm_application::{
    AuthorityError, AuthorityProvider, AuthorityRequest, ObligationPreparationError,
    PolicyReference, PolicySelectionRequest, ProfileSelection, ProtectedPreparationError,
    SelectionPreparationError, SessionRequest, prepare, prepare_obligations,
    prepare_protected_obligations, resolve_selection_request,
};
use eqm_domain::{DimensionId, PolicyId, ProfileId, Revision, SymbolicValueId, UtcInstant};
use eqm_engine::{
    AuthorityOrigin, EvaluationMode, PolicyProfileRequest, PolicyRef, ProfileRequest,
    SelectionError,
};
use std::cell::Cell;
use std::error::Error;
use std::fs;
use std::path::Path;

struct ProtectedGraph {
    graph: eqm_engine::FinalizedWorkspaceGraph,
    calls: Cell<usize>,
}

impl AuthorityProvider for ProtectedGraph {
    fn load_authority(
        &self,
        request: &AuthorityRequest,
    ) -> Result<&eqm_engine::FinalizedWorkspaceGraph, AuthorityError> {
        assert_eq!(request.mode, EvaluationMode::Release);
        self.calls.set(self.calls.get() + 1);
        Ok(&self.graph)
    }
}

fn exact_selection() -> Result<PolicySelectionRequest, Box<dyn Error>> {
    Ok(typed(
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

#[test]
fn protected_preparation_requires_a_real_capability_and_exact_context() -> Result<(), Box<dyn Error>>
{
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let input = exact_selection()?;
    assert!(matches!(
        prepare_protected_obligations(
            session.finalized(),
            EvaluationMode::Release,
            &input,
            None,
            None
        ),
        Err(ProtectedPreparationError::Authority(
            AuthorityError::MissingProvider
        ))
    ));
    let provider = ProtectedGraph {
        graph: session.finalized().clone(),
        calls: Cell::new(0),
    };
    assert!(matches!(
        prepare_protected_obligations(
            session.finalized(),
            EvaluationMode::Development,
            &input,
            Some(&provider),
            None
        ),
        Err(ProtectedPreparationError::DevelopmentMode)
    ));
    assert_eq!(provider.calls.get(), 0);
    let context = eqm_domain::Sha256Digest::hash_content(b"validated release record");
    let prepared = prepare_protected_obligations(
        session.finalized(),
        EvaluationMode::Release,
        &input,
        Some(&provider),
        Some(context),
    )?;
    assert_eq!(provider.calls.get(), 1);
    assert_eq!(prepared.selection().mode(), EvaluationMode::Release);
    assert_eq!(
        prepared.selection().origin(),
        AuthorityOrigin::ProtectedBaseline
    );
    assert!(!prepared.derivation().obligations.is_empty());
    assert!(
        prepared
            .derivation()
            .obligations
            .keys()
            .all(|key| key.release_context == Some(context))
    );

    // Same visible IDs cannot conceal different independently loaded definitions.
    let foreign = example()?;
    let path = foreign
        .path()
        .join("eqm/contracts/auth.signup.identifier.toml");
    fs::write(
        &path,
        fs::read_to_string(&path)?.replace(
            "Email is selected when the surface is first presented.",
            "Email must be selected when the surface is first presented.",
        ),
    )?;
    let foreign_session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        foreign.path(),
    )?;
    assert_ne!(
        foreign_session.finalized().graph(),
        session.finalized().graph()
    );
    let foreign_provider = ProtectedGraph {
        graph: foreign_session.finalized().clone(),
        calls: Cell::new(0),
    };
    assert!(matches!(
        prepare_protected_obligations(
            session.finalized(),
            EvaluationMode::Release,
            &input,
            Some(&foreign_provider),
            Some(context)
        ),
        Err(ProtectedPreparationError::CandidateMismatch)
    ));
    assert_eq!(foreign_provider.calls.get(), 1);
    Ok(())
}

#[test]
fn protected_omissions_reject_before_loading_authority() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let provider = ProtectedGraph {
        graph: session.finalized().clone(),
        calls: Cell::new(0),
    };
    let policy = Some(PolicyReference::new(
        PolicyId::new("consumer.critical_flow")?,
        Some(Revision::new(1)?),
    ));
    let profile = ProfileSelection::new(
        ProfileId::new("audience.default")?,
        Some(Revision::new(1)?),
        vec![(
            DimensionId::new("region")?,
            Some(SymbolicValueId::new("us")?),
        )],
    );
    for (index, (input, expected)) in [
        (typed(None, vec![profile.clone()]), "policy ID"),
        (
            typed(
                Some(PolicyReference::new(
                    PolicyId::new("consumer.critical_flow")?,
                    None,
                )),
                vec![profile.clone()],
            ),
            "explicit exact revision",
        ),
        (typed(policy.clone(), vec![]), "explicitly selected"),
        (
            typed(
                policy.clone(),
                vec![ProfileSelection::new(
                    ProfileId::new("audience.default")?,
                    None,
                    profile.values().to_vec(),
                )],
            ),
            "explicit exact revision",
        ),
        (
            typed(
                policy,
                vec![ProfileSelection::new(
                    ProfileId::new("audience.default")?,
                    Some(Revision::new(1)?),
                    vec![],
                )],
            ),
            "DimensionSelectionRequired",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let error = prepare_protected_obligations(
            session.finalized(),
            EvaluationMode::Release,
            &input,
            Some(&provider),
            None,
        )
        .err()
        .ok_or("protected omission was admitted")?;
        assert!(
            matches!(error, ProtectedPreparationError::Preparation(_)),
            "{error}"
        );
        assert!(error.to_string().contains(expected), "{expected}: {error}");
        let exact_cause = match (index, &error) {
            (
                0,
                ProtectedPreparationError::Preparation(
                    ObligationPreparationError::SelectionPreparation(
                        SelectionPreparationError::PolicyIdRequired,
                    ),
                ),
            ) => true,
            (
                1,
                ProtectedPreparationError::Preparation(
                    ObligationPreparationError::SelectionPreparation(
                        SelectionPreparationError::PolicyRevisionRequired { id },
                    ),
                ),
            ) => id.as_str() == "consumer.critical_flow",
            (
                2,
                ProtectedPreparationError::Preparation(
                    ObligationPreparationError::SelectionPreparation(
                        SelectionPreparationError::ProfileSelectionRequired { id },
                    ),
                ),
            ) => id.as_str() == "audience.default",
            (
                3,
                ProtectedPreparationError::Preparation(
                    ObligationPreparationError::SelectionPreparation(
                        SelectionPreparationError::ProfileRevisionRequired { id },
                    ),
                ),
            ) => id.as_str() == "audience.default",
            (
                4,
                ProtectedPreparationError::Preparation(ObligationPreparationError::Selection(
                    SelectionError::DimensionSelectionRequired,
                )),
            ) => true,
            _ => false,
        };
        assert!(exact_cause, "{index}: wrong typed cause: {error}");
    }
    assert_eq!(provider.calls.get(), 0);
    // An explicitly unknown value remains unknown rather than using a default.
    let unknown = typed(
        Some(PolicyReference::new(
            PolicyId::new("consumer.critical_flow")?,
            Some(Revision::new(1)?),
        )),
        vec![ProfileSelection::new(
            ProfileId::new("audience.default")?,
            Some(Revision::new(1)?),
            vec![(DimensionId::new("region")?, None)],
        )],
    );
    let prepared = prepare_protected_obligations(
        session.finalized(),
        EvaluationMode::Release,
        &unknown,
        Some(&provider),
        None,
    )?;
    assert_eq!(
        prepared.selection().profiles()[&ProfileId::new("audience.default")?].values()
            [&DimensionId::new("region")?],
        None
    );
    Ok(())
}

fn copy_directory(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn example() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios");
    let repository = tempfile::tempdir()?;
    copy_directory(&source, repository.path())?;
    fs::create_dir(repository.path().join(".git"))?;
    Ok(repository)
}

fn evaluation_time() -> Result<UtcInstant, Box<dyn Error>> {
    Ok("2026-01-02T03:04:05Z".parse()?)
}

fn request(origin: AuthorityOrigin, region: &str) -> Result<PolicyProfileRequest, Box<dyn Error>> {
    Ok(PolicyProfileRequest::new(
        origin,
        PolicyRef::new(PolicyId::new("consumer.critical_flow")?, Revision::new(1)?),
        vec![ProfileRequest::new(
            ProfileId::new("audience.default")?,
            Revision::new(1)?,
            vec![(
                DimensionId::new("region")?,
                Some(SymbolicValueId::new(region)?),
            )],
        )?],
    )?)
}

#[test]
fn development_default_selection_is_explicit_and_derives_obligations() -> Result<(), Box<dyn Error>>
{
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let default = request(AuthorityOrigin::CandidateLocal, "us")?;
    let prepared = prepare_obligations(
        session.finalized(),
        EvaluationMode::Development,
        None,
        Some(&default),
        None,
    )?;
    assert!(
        prepared
            .selection()
            .profiles()
            .contains_key(&ProfileId::new("audience.default")?)
    );
    assert!(
        !prepared.derivation().obligations.is_empty(),
        "development derivation must enumerate obligations"
    );
    Ok(())
}

#[test]
fn protected_modes_never_take_the_development_branch() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let default = request(AuthorityOrigin::CandidateLocal, "us")?;
    for mode in [EvaluationMode::PullRequest, EvaluationMode::Release] {
        let error = prepare_obligations(session.finalized(), mode, None, Some(&default), None);
        assert!(
            matches!(
                error,
                Err(ObligationPreparationError::Selection(
                    SelectionError::ExplicitSelectionRequired
                ))
            ),
            "protected mode {mode:?} must require an explicit selection"
        );
    }
    Ok(())
}

#[test]
fn protected_modes_reject_candidate_local_authority() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let candidate = request(AuthorityOrigin::CandidateLocal, "eu")?;
    let error = prepare_obligations(
        session.finalized(),
        EvaluationMode::Release,
        Some(&candidate),
        None,
        None,
    );
    assert!(
        matches!(
            error,
            Err(ObligationPreparationError::Selection(
                SelectionError::UntrustedSelection
            ))
        ),
        "release mode must reject candidate-local authority"
    );
    Ok(())
}

#[test]
fn trusted_invocation_selection_is_reusable_across_modes() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let trusted = request(AuthorityOrigin::TrustedInvocation, "eu")?;
    let pull_request = prepare_obligations(
        session.finalized(),
        EvaluationMode::PullRequest,
        Some(&trusted),
        None,
        None,
    )?;
    assert!(!pull_request.derivation().obligations.is_empty());
    let release = prepare_obligations(
        session.finalized(),
        EvaluationMode::Release,
        Some(&trusted),
        None,
        None,
    );
    assert!(
        matches!(
            release,
            Err(ObligationPreparationError::Selection(
                SelectionError::UntrustedSelection
            ))
        ),
        "release mode requires protected baseline authority"
    );
    Ok(())
}

fn typed(
    policy: Option<PolicyReference>,
    profiles: Vec<ProfileSelection>,
) -> PolicySelectionRequest {
    PolicySelectionRequest::development(policy, profiles)
}

#[test]
fn typed_request_resolves_the_same_exact_authority_as_the_adapter() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let input = typed(
        Some(PolicyReference::new(
            PolicyId::new("consumer.critical_flow")?,
            None,
        )),
        vec![ProfileSelection::new(
            ProfileId::new("audience.default")?,
            None,
            vec![(
                DimensionId::new("region")?,
                Some(SymbolicValueId::new("us")?),
            )],
        )],
    );
    let resolved =
        resolve_selection_request(session.finalized(), EvaluationMode::Development, &input)?;
    let expected = request(AuthorityOrigin::CandidateLocal, "us")?;
    assert_eq!(resolved, expected);
    let prepared = prepare_obligations(
        session.finalized(),
        EvaluationMode::Development,
        Some(&resolved),
        None,
        None,
    )?;
    assert!(!prepared.derivation().obligations.is_empty());
    Ok(())
}

#[test]
fn duplicate_identical_profile_selection_rejects_before_resolution() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let selection = ProfileSelection::new(ProfileId::new("audience.default")?, None, vec![]);
    let input = PolicySelectionRequest::development(None, vec![selection.clone(), selection]);
    assert!(matches!(
        resolve_selection_request(session.finalized(), EvaluationMode::Development, &input),
        Err(SelectionPreparationError::DuplicateProfile { .. })
    ));
    Ok(())
}

#[test]
fn missing_and_ambiguous_exact_revisions_reject() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let policy_path = repository
        .path()
        .join("eqm/policies/consumer.critical_flow.toml");
    let policy = fs::read_to_string(&policy_path)?;
    fs::write(
        repository
            .path()
            .join("eqm/policies/consumer.critical_flow.r2.toml"),
        policy.replace("revision = 1", "revision = 2"),
    )?;
    let profile_path = repository.path().join("eqm/profiles/audience.default.toml");
    let profile = fs::read_to_string(&profile_path)?;
    fs::write(
        repository
            .path()
            .join("eqm/profiles/audience.default.r2.toml"),
        profile.replace("revision = 1", "revision = 2"),
    )?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    // An omitted revision with more than one stored revision is ambiguous.
    assert!(matches!(
        resolve_selection_request(
            session.finalized(),
            EvaluationMode::Development,
            &typed(
                Some(PolicyReference::new(
                    PolicyId::new("consumer.critical_flow")?,
                    None
                )),
                vec![]
            )
        ),
        Err(SelectionPreparationError::PolicyAmbiguous { .. })
    ));
    // An explicit revision that is not stored is missing, never remapped.
    assert!(matches!(
        resolve_selection_request(
            session.finalized(),
            EvaluationMode::Development,
            &typed(
                Some(PolicyReference::new(
                    PolicyId::new("consumer.critical_flow")?,
                    Some(Revision::new(9)?)
                )),
                vec![]
            )
        ),
        Err(SelectionPreparationError::PolicyNotFound { .. })
    ));
    assert!(matches!(
        resolve_selection_request(
            session.finalized(),
            EvaluationMode::Development,
            &typed(
                Some(PolicyReference::new(
                    PolicyId::new("consumer.critical_flow")?,
                    Some(Revision::new(1)?)
                )),
                vec![ProfileSelection::new(
                    ProfileId::new("audience.default")?,
                    Some(Revision::new(9)?),
                    vec![],
                )]
            )
        ),
        Err(SelectionPreparationError::ProfileNotFound { .. })
    ));
    Ok(())
}
