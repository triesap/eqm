//! S020: exact same-session target-configuration lookup in the application layer.

use eqm_application::{
    SessionRequest, TargetConfigurationError, prepare, prepare_target_configuration,
};
use eqm_domain::{TargetConfigurationDigest, TargetId, UtcInstant};
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

fn target_id(value: &str) -> Result<TargetId, Box<dyn Error>> {
    Ok(TargetId::new(value)?)
}

#[test]
fn exact_target_lookup_returns_typed_identity() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let android = target_id("android")?;
    let canonical = prepare_target_configuration(&session, &android)?;
    let typed: TargetConfigurationDigest = canonical.digest();
    assert_eq!(
        typed,
        TargetConfigurationDigest::hash_content(canonical.bytes())
    );
    // The android target really is the requested one.
    let text = std::str::from_utf8(canonical.bytes())?;
    assert!(text.contains(r#""id":"android""#));
    assert!(text.contains(r#""platform":"android""#));
    assert!(text.contains(r#""framework":"compose""#));

    // The two distinct targets have distinct identities.
    let ios = prepare_target_configuration(&session, &target_id("ios")?)?;
    assert_ne!(canonical.bytes(), ios.bytes());
    Ok(())
}

#[test]
fn missing_target_returns_typed_error_with_requested_id() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let missing = target_id("missing")?;
    let Err(error) = prepare_target_configuration(&session, &missing) else {
        return Err("a missing target must be an error".into());
    };
    match &error {
        TargetConfigurationError::MissingTarget { target } => assert_eq!(target, &missing),
        other => return Err(format!("expected MissingTarget, got {other:?}").into()),
    }
    assert!(
        std::error::Error::source(&error).is_none(),
        "a missing target has no projection cause"
    );
    Ok(())
}

#[test]
fn unrelated_target_and_policy_changes_leave_this_identity_unchanged() -> Result<(), Box<dyn Error>>
{
    let repository = example()?;
    let baseline_session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let android = target_id("android")?;
    let baseline = prepare_target_configuration(&baseline_session, &android)?;

    // Changing the unrelated ios target's framework leaves android unchanged
    // while changing ios.
    let config_path = repository.path().join("eqm.toml");
    let config = fs::read_to_string(&config_path)?;
    let mutated = config.replace("framework = \"swiftui\"", "framework = \"swiftui2\"");
    assert_ne!(mutated, config, "the ios target must really change");
    fs::write(&config_path, mutated)?;
    let changed_session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    assert_eq!(
        baseline.bytes(),
        prepare_target_configuration(&changed_session, &android)?.bytes(),
        "an unrelated target change must not move the android identity"
    );
    assert_ne!(
        prepare_target_configuration(&changed_session, &target_id("ios")?)?.bytes(),
        prepare_target_configuration(&baseline_session, &target_id("ios")?)?.bytes()
    );

    // A policy-only change leaves the target identity unchanged.
    let policy_path = repository
        .path()
        .join("eqm/policies/consumer.critical_flow.toml");
    let policy = fs::read_to_string(&policy_path)?;
    let mutated = policy.replace(
        "title = \"Consumer critical flow policy\"",
        "title = \"Consumer critical flow policy v2\"",
    );
    assert_ne!(mutated, policy);
    fs::write(&policy_path, mutated)?;
    let policy_session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    assert_eq!(
        baseline.bytes(),
        prepare_target_configuration(&policy_session, &android)?.bytes()
    );
    Ok(())
}

#[test]
fn authored_formatting_change_does_not_move_target_identity() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let baseline_session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let android = target_id("android")?;
    let baseline = prepare_target_configuration(&baseline_session, &android)?;

    let config_path = repository.path().join("eqm.toml");
    let config = fs::read_to_string(&config_path)?;
    let reformatted = format!("# authored comment, no semantics\n\n{config}\n");
    assert_ne!(reformatted, config);
    fs::write(&config_path, reformatted)?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    assert_eq!(baseline, prepare_target_configuration(&session, &android)?);
    Ok(())
}

#[test]
fn same_session_lookup_survives_authored_file_mutation() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let android = target_id("android")?;
    let first = prepare_target_configuration(&session, &android)?;

    let config_path = repository.path().join("eqm.toml");
    let config = fs::read_to_string(&config_path)?;
    fs::write(
        &config_path,
        config.replace("framework = \"compose\"", "framework = \"compose2\""),
    )?;

    // The already-prepared immutable graph is unchanged...
    assert_eq!(first, prepare_target_configuration(&session, &android)?);
    // ...while a fresh preparation observes the mutation.
    let fresh = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    assert_ne!(
        first.bytes(),
        prepare_target_configuration(&fresh, &android)?.bytes()
    );
    Ok(())
}
