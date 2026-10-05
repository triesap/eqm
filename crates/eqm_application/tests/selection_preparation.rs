//! S044: application-owned selection orchestration with explicit mode/authority.
//!
//! These tests drive the real example workspace through `prepare` and the new
//! application operation, proving that the development baseline is explicit and
//! that protected modes can never take an implicit development branch or a
//! candidate-local authority.

use eqm_application::{ObligationPreparationError, SessionRequest, prepare, prepare_obligations};
use eqm_domain::{DimensionId, PolicyId, ProfileId, Revision, SymbolicValueId, UtcInstant};
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
