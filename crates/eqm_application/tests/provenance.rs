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

fn request_at(
    origin: AuthorityOrigin,
    policy_revision: u64,
    profile_revision: u64,
    region: Option<&str>,
) -> Result<PolicyProfileRequest, Box<dyn Error>> {
    Ok(PolicyProfileRequest::new(
        origin,
        PolicyRef::new(
            PolicyId::new("consumer.critical_flow")?,
            Revision::new(policy_revision)?,
        ),
        vec![ProfileRequest::new(
            ProfileId::new("audience.default")?,
            Revision::new(profile_revision)?,
            vec![(
                DimensionId::new("region")?,
                region.map(SymbolicValueId::new).transpose()?,
            )],
        )?],
    )?)
}

#[test]
fn missing_exact_revisions_return_typed_errors_without_latest_fallback()
-> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;

    // A present revision is accepted.
    let present = request_at(AuthorityOrigin::CandidateLocal, 1, 1, Some("eu"))?;
    assert!(
        prepare_policy_provenance(&session, EvaluationMode::Development, Some(&present), None)
            .is_ok()
    );

    // Missing exact policy revision: another revision (1) exists, but there is
    // no latest fallback.
    let missing_policy = request_at(AuthorityOrigin::CandidateLocal, 2, 1, Some("eu"))?;
    let Err(error) = prepare_policy_provenance(
        &session,
        EvaluationMode::Development,
        Some(&missing_policy),
        None,
    ) else {
        return Err("missing policy revision must fail".into());
    };
    assert!(matches!(
        &error,
        eqm_application::PolicyProvenanceError::Selection(SelectionError::PolicyNotFound)
    ));
    assert_eq!(
        std::error::Error::source(&error)
            .and_then(|source| source.downcast_ref::<SelectionError>()),
        Some(&SelectionError::PolicyNotFound),
        "the typed selection cause must survive"
    );

    // Missing exact profile revision while policy revision 1 exists.
    let missing_profile = request_at(AuthorityOrigin::CandidateLocal, 1, 2, Some("eu"))?;
    let Err(error) = prepare_policy_provenance(
        &session,
        EvaluationMode::Development,
        Some(&missing_profile),
        None,
    ) else {
        return Err("missing profile revision must fail".into());
    };
    assert!(matches!(
        &error,
        eqm_application::PolicyProvenanceError::Selection(SelectionError::ProfileNotFound)
    ));
    assert_eq!(
        std::error::Error::source(&error)
            .and_then(|source| source.downcast_ref::<SelectionError>()),
        Some(&SelectionError::ProfileNotFound)
    );
    Ok(())
}

#[test]
fn known_and_unknown_values_on_the_same_definition_change_identity() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    // Region is `known("us")` in one selection and explicitly `unknown` in the
    // other; the profile definition itself is identical.
    let known_request = request_at(AuthorityOrigin::CandidateLocal, 1, 1, Some("us"))?;
    let unknown_request = request_at(AuthorityOrigin::CandidateLocal, 1, 1, None)?;
    let known = prepare_policy_provenance(
        &session,
        EvaluationMode::Development,
        Some(&known_request),
        None,
    )?;
    let unknown = prepare_policy_provenance(
        &session,
        EvaluationMode::Development,
        Some(&unknown_request),
        None,
    )?;
    assert_eq!(
        known.contract().digest(),
        unknown.contract().digest(),
        "the contract axis is unchanged by a value-only selection change"
    );
    assert_ne!(
        known.policy_selection().bytes(),
        unknown.policy_selection().bytes()
    );
    assert_ne!(
        known.policy_selection().digest(),
        unknown.policy_selection().digest()
    );
    let known_text = std::str::from_utf8(known.policy_selection().bytes())?;
    let unknown_text = std::str::from_utf8(unknown.policy_selection().bytes())?;
    assert!(known_text.contains(r#""state":"known","value":"us""#));
    assert!(unknown_text.contains(r#""region","state":"unknown""#));
    assert!(
        !unknown_text.contains(r#""state":"known""#),
        "an explicit unknown must never become the declared default"
    );
    assert!(
        unknown_text.contains(r#""defaults":{"region":"us"}"#),
        "the default stays declared in the definition but is not applied"
    );
    Ok(())
}

const EXTRA_POLICY: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/policy.schema.json"
id = "other.critical_flow"
revision = 1
title = "Other critical flow policy"
owners = ["owner://team/product"]
profiles = ["audience.other"]
required_targets = ["ios"]

[[rules]]
minimum_level = "required"
facets = ["behavior"]
minimum_trust = "trusted_ci"
maximum_age = 86400000
minimum_count = 1

[rules.selector]
risk_classes = ["high", "critical"]

[waivers]
allowed = true
maximum_days = 30
minimum_approvers = 1
required_controls = ["behavior"]
"#;

const EXTRA_PROFILE: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/profile.schema.json"
id = "audience.other"
revision = 1
title = "Other audience"
owners = ["owner://team/product"]

[[dimensions]]
id = "region"
values = ["eu", "us"]

[defaults]
region = "eu"
"#;

fn example_with_extra_definitions() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let repository = example()?;
    fs::write(
        repository
            .path()
            .join("eqm/policies/other.critical_flow.toml"),
        EXTRA_POLICY,
    )?;
    fs::write(
        repository.path().join("eqm/profiles/audience.other.toml"),
        EXTRA_PROFILE,
    )?;
    Ok(repository)
}

fn selected_bundle(
    repository: &Path,
) -> Result<
    (
        eqm_application::PolicyProvenanceV2,
        (ContractDigest, PolicySelectionDigest),
    ),
    Box<dyn Error>,
> {
    let session = prepare(&SessionRequest::new(None, evaluation_time()?), repository)?;
    let explicit = request_at(AuthorityOrigin::CandidateLocal, 1, 1, Some("eu"))?;
    let bundle =
        prepare_policy_provenance(&session, EvaluationMode::Development, Some(&explicit), None)?;
    let digests = (
        bundle.contract().digest(),
        bundle.policy_selection().digest(),
    );
    Ok((bundle, digests))
}

#[test]
fn unselected_definitions_do_not_affect_selected_identities() -> Result<(), Box<dyn Error>> {
    let policy_repository = example_with_extra_definitions()?;
    let (_, baseline) = selected_bundle(policy_repository.path())?;
    let policy_path = policy_repository
        .path()
        .join("eqm/policies/other.critical_flow.toml");
    let original = fs::read_to_string(&policy_path)?;
    let mutated = original.replace(
        "title = \"Other critical flow policy\"",
        "title = \"Other critical flow policy v2\"",
    );
    assert_ne!(
        mutated, original,
        "the unselected policy must really change"
    );
    fs::write(&policy_path, mutated)?;
    let (_, after_policy) = selected_bundle(policy_repository.path())?;
    assert_eq!(baseline, after_policy);

    let profile_repository = example_with_extra_definitions()?;
    let (_, baseline) = selected_bundle(profile_repository.path())?;
    let profile_path = profile_repository
        .path()
        .join("eqm/profiles/audience.other.toml");
    let original = fs::read_to_string(&profile_path)?;
    let mutated = original.replace(
        "title = \"Other audience\"",
        "title = \"Other audience v2\"",
    );
    assert_ne!(
        mutated, original,
        "the unselected profile must really change"
    );
    fs::write(&profile_path, mutated)?;
    let (_, after_profile) = selected_bundle(profile_repository.path())?;
    assert_eq!(baseline, after_profile);
    Ok(())
}

#[test]
fn selected_profile_definition_mutation_changes_only_policy_identity() -> Result<(), Box<dyn Error>>
{
    let repository = example()?;
    let (_, baseline) = selected_bundle(repository.path())?;
    let profile_path = repository.path().join("eqm/profiles/audience.default.toml");
    let original = fs::read_to_string(&profile_path)?;
    let mutated = original.replace(
        "title = \"Default audience\"",
        "title = \"Default audience v2\"",
    );
    assert_ne!(mutated, original, "the selected profile must really change");
    fs::write(&profile_path, mutated)?;
    let (_, after) = selected_bundle(repository.path())?;
    assert_eq!(
        baseline.0, after.0,
        "a selected-profile-only edit must not change the contract identity"
    );
    assert_ne!(
        baseline.1, after.1,
        "same-ID/revision profile definition semantics must change the policy identity"
    );
    Ok(())
}

#[test]
fn formatting_only_manifest_change_preserves_both_identities() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let (_, baseline) = selected_bundle(repository.path())?;
    let policy_path = repository
        .path()
        .join("eqm/policies/consumer.critical_flow.toml");
    let original = fs::read_to_string(&policy_path)?;
    let reformatted = format!(
        "# authored comment with no semantic meaning\n\n{}\n   \n",
        original.replace("revision = 1", "revision = 1  # inline revision comment")
    );
    assert_ne!(reformatted, original, "the formatting change must be real");
    fs::write(&policy_path, reformatted)?;
    let (_, after) = selected_bundle(repository.path())?;
    assert_eq!(baseline, after);
    Ok(())
}

#[test]
fn prepared_session_reuses_immutable_graph_across_file_mutation_and_removal()
-> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let explicit = request_at(AuthorityOrigin::CandidateLocal, 1, 1, Some("eu"))?;
    let first =
        prepare_policy_provenance(&session, EvaluationMode::Development, Some(&explicit), None)?;

    // A semantic mutation of the backing file does not change the already
    // prepared immutable graph.
    let policy_path = repository
        .path()
        .join("eqm/policies/consumer.critical_flow.toml");
    let original = fs::read_to_string(&policy_path)?;
    fs::write(
        &policy_path,
        original.replace(
            "title = \"Consumer critical flow policy\"",
            "title = \"Consumer critical flow policy v2\"",
        ),
    )?;
    let reused =
        prepare_policy_provenance(&session, EvaluationMode::Development, Some(&explicit), None)?;
    assert_eq!(first.contract().bytes(), reused.contract().bytes());
    assert_eq!(
        first.policy_selection().bytes(),
        reused.policy_selection().bytes()
    );

    // A fresh preparation observes the mutation.
    let fresh_session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let fresh = prepare_policy_provenance(
        &fresh_session,
        EvaluationMode::Development,
        Some(&explicit),
        None,
    )?;
    assert_ne!(
        first.policy_selection().bytes(),
        fresh.policy_selection().bytes()
    );

    // Removing the backing profile file changes nothing for the existing
    // session but makes a fresh preparation fail.
    let profile_path = repository.path().join("eqm/profiles/audience.default.toml");
    fs::remove_file(&profile_path)?;
    let after_removal =
        prepare_policy_provenance(&session, EvaluationMode::Development, Some(&explicit), None)?;
    assert_eq!(
        first.policy_selection().bytes(),
        after_removal.policy_selection().bytes()
    );
    assert!(
        prepare(
            &SessionRequest::new(None, evaluation_time()?),
            repository.path()
        )
        .is_err(),
        "a fresh preparation must observe the removed authored file"
    );
    Ok(())
}
