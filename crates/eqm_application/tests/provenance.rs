//! S019-C: same-session v2 contract and selected-policy provenance assembly.

use eqm_application::{SessionRequest, prepare, prepare_policy_provenance};
use eqm_domain::{
    ContractDigest, DimensionId, PolicyId, PolicySelectionDigest, ProfileId, Revision,
    SymbolicValueId, UtcInstant,
};
use eqm_engine::{
    AuthorityOrigin, EvaluationMode, PolicyProfileRequest, PolicyRef, ProfileRequest,
    SelectionError,
};
use std::error::Error;
use std::fs;
use std::path::Path;

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
fn development_explicit_selection_builds_typed_bundle() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let explicit = request(AuthorityOrigin::CandidateLocal, "eu")?;
    let bundle =
        prepare_policy_provenance(&session, EvaluationMode::Development, Some(&explicit), None)?;

    let contract: &ContractDigest = &bundle.contract().digest();
    assert_eq!(
        *contract,
        ContractDigest::hash_content(bundle.contract().bytes())
    );
    let selection: &PolicySelectionDigest = &bundle.policy_selection().digest();
    assert_eq!(
        *selection,
        PolicySelectionDigest::hash_content(bundle.policy_selection().bytes())
    );
    assert!(!bundle.contract().bytes().is_empty());
    assert!(!bundle.policy_selection().bytes().is_empty());
    assert_ne!(
        bundle.contract().bytes(),
        bundle.policy_selection().bytes(),
        "contract and selected-policy identities are distinct"
    );
    Ok(())
}

#[test]
fn mode_and_authority_rules_are_retained() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;

    // Release requires protected-baseline authority.
    let candidate = request(AuthorityOrigin::CandidateLocal, "eu")?;
    assert!(matches!(
        prepare_policy_provenance(&session, EvaluationMode::Release, Some(&candidate), None),
        Err(eqm_application::PolicyProvenanceError::Selection(
            SelectionError::UntrustedSelection
        ))
    ));
    let trusted = request(AuthorityOrigin::TrustedInvocation, "eu")?;
    assert!(matches!(
        prepare_policy_provenance(&session, EvaluationMode::Release, Some(&trusted), None),
        Err(eqm_application::PolicyProvenanceError::Selection(
            SelectionError::UntrustedSelection
        ))
    ));
    let protected = request(AuthorityOrigin::ProtectedBaseline, "eu")?;
    assert!(
        prepare_policy_provenance(&session, EvaluationMode::Release, Some(&protected), None)
            .is_ok()
    );
    assert!(
        prepare_policy_provenance(&session, EvaluationMode::PullRequest, Some(&trusted), None)
            .is_ok()
    );

    // Non-local modes require an explicit selection.
    assert!(matches!(
        prepare_policy_provenance(&session, EvaluationMode::Release, None, None),
        Err(eqm_application::PolicyProvenanceError::Selection(
            SelectionError::ExplicitSelectionRequired
        ))
    ));

    // Development requires an explicit or prepared development-default selection.
    assert!(matches!(
        prepare_policy_provenance(&session, EvaluationMode::Development, None, None),
        Err(eqm_application::PolicyProvenanceError::Selection(
            SelectionError::SelectionRequired
        ))
    ));
    let default = request(AuthorityOrigin::CandidateLocal, "us")?;
    assert!(
        prepare_policy_provenance(&session, EvaluationMode::Development, None, Some(&default))
            .is_ok()
    );
    Ok(())
}

#[test]
fn contract_and_policy_identities_move_independently() -> Result<(), Box<dyn Error>> {
    let baseline_repository = example()?;
    let baseline_session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        baseline_repository.path(),
    )?;
    let explicit = request(AuthorityOrigin::CandidateLocal, "eu")?;
    let baseline = prepare_policy_provenance(
        &baseline_session,
        EvaluationMode::Development,
        Some(&explicit),
        None,
    )?;

    // A policy-only semantic edit changes the selected-policy identity but not
    // the contract identity.
    let policy_repository = example()?;
    let policy_path = policy_repository
        .path()
        .join("eqm/policies/consumer.critical_flow.toml");
    let text = fs::read_to_string(&policy_path)?.replace(
        "title = \"Consumer critical flow policy\"",
        "title = \"Consumer critical flow policy v2\"",
    );
    fs::write(&policy_path, text)?;
    let policy_session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        policy_repository.path(),
    )?;
    let policy_changed = prepare_policy_provenance(
        &policy_session,
        EvaluationMode::Development,
        Some(&explicit),
        None,
    )?;
    assert_eq!(
        baseline.contract().digest(),
        policy_changed.contract().digest(),
        "a policy-only edit must not change the contract identity"
    );
    assert_ne!(
        baseline.policy_selection().digest(),
        policy_changed.policy_selection().digest()
    );

    // A contract-only semantic edit changes the contract identity but not the
    // selected-policy identity.
    let contract_repository = example()?;
    let capability_path = contract_repository
        .path()
        .join("eqm/contracts/account.create.toml");
    let text = fs::read_to_string(&capability_path)?.replace(
        "title = \"Create account\"",
        "title = \"Create account v2\"",
    );
    fs::write(&capability_path, text)?;
    let contract_session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        contract_repository.path(),
    )?;
    let contract_changed = prepare_policy_provenance(
        &contract_session,
        EvaluationMode::Development,
        Some(&explicit),
        None,
    )?;
    assert_ne!(
        baseline.contract().digest(),
        contract_changed.contract().digest()
    );
    assert_eq!(
        baseline.policy_selection().digest(),
        contract_changed.policy_selection().digest(),
        "a contract-only edit must not change the selected-policy identity"
    );
    Ok(())
}

#[test]
fn workspace_digest_remains_aggregate_context() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let explicit = request(AuthorityOrigin::CandidateLocal, "eu")?;
    let bundle =
        prepare_policy_provenance(&session, EvaluationMode::Development, Some(&explicit), None)?;
    // The aggregate workspace digest is a separate axis, not injected into the
    // contract or selected-policy projections.
    let workspace = session.workspace_digest().to_string();
    assert!(!std::str::from_utf8(bundle.contract().bytes())?.contains(&workspace));
    assert!(!std::str::from_utf8(bundle.policy_selection().bytes())?.contains(&workspace));
    Ok(())
}
