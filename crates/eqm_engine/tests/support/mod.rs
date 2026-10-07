//! Shared integration-test materialization for the public example.

use eqm_domain::UtcInstant;
use eqm_manifest::LoadedWorkspace;
use std::error::Error;
use std::fs;
use std::path::Path;

#[allow(dead_code)] // Integration crates use different parts of this shared fixture.
pub(crate) fn loaded_example() -> Result<(tempfile::TempDir, LoadedWorkspace), Box<dyn Error>> {
    loaded_example_with(|_| Ok(()))
}

/// Materializes the example and lets a caller mutate it before loading.
#[allow(dead_code)] // Integration crates use different parts of this shared fixture.
pub(crate) fn loaded_example_with(
    mutate: impl FnOnce(&Path) -> Result<(), Box<dyn Error>>,
) -> Result<(tempfile::TempDir, LoadedWorkspace), Box<dyn Error>> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios");
    let repository = tempfile::tempdir()?;
    copy_directory(&source, repository.path())?;
    fs::create_dir(repository.path().join(".git"))?;
    write_scenario(repository.path(), &["android", "ios"])?;
    mutate(repository.path())?;
    let loaded = eqm_manifest::load_workspace(repository.path(), None)?;
    Ok((repository, loaded))
}

/// Returns the shared deterministic evaluation instant for fixtures.
///
/// Fixtures must never read the wall clock. This delegates to the shared test
/// support builder so engine regressions and other crates agree on one instant
/// instead of re-declaring private constants.
pub(crate) fn evaluation_time() -> Result<UtcInstant, Box<dyn Error>> {
    Ok(eqm_test_support::evaluation_time()?)
}

#[allow(dead_code)] // Integration crates use different parts of this shared fixture.
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

/// Synthetic authored scenario; its exact selected map is separate and explicit.
pub(crate) fn write_scenario(root: &Path, targets: &[&str]) -> Result<(), Box<dyn Error>> {
    let mut text = String::from(
        "schema = \"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/scenario.schema.json\"\nid = \"scenario.fixture_signup\"\nrevision = 1\n",
    );
    for (index, target) in targets.iter().enumerate() {
        text.push_str(&format!("\n[[participants]]\nid = \"actor_{index}\"\nrole = \"edge\"\n[participants.subject]\ntarget = \"{target}\"\n"));
    }
    // A second occurrence of the first target proves actor multiplicity does
    // not collapse to a set of target IDs.
    let target = targets.first().ok_or("fixture target missing")?;
    text.push_str(&format!("\n[[participants]]\nid = \"repeat_actor\"\nrole = \"observer\"\n[participants.subject]\ntarget = \"{target}\"\n\n[[dependencies]]\nfrom = \"repeat_actor\"\nto = \"actor_0\"\n"));
    fs::write(
        root.join("eqm/contracts/scenario.fixture_signup.toml"),
        text,
    )?;
    Ok(())
}

/// Explicit fixture policy input, never a production inferred/default mapping.
/// This known fixture requirement selects exactly the authored fixture scene;
/// transformed non-EndToEnd/minimal fixtures explicitly select no scenario.
#[allow(dead_code)] // Integration crates use different parts of this shared fixture.
pub(crate) fn fixture_bindings(
    graph: &eqm_domain::WorkspaceGraph,
) -> Result<eqm_domain::RequirementScenarioBindings, Box<dyn Error>> {
    let requirement =
        eqm_domain::FullRequirementId::new("account.create.signup.otp#valid_code_advances")?;
    let is_end_to_end = graph
        .surfaces()
        .values()
        .filter(|surface| surface.id().as_str() == requirement.owner())
        .flat_map(|surface| surface.requirements().values())
        .any(|definition| {
            definition.id().as_str() == requirement.local()
                && definition.scope() == eqm_domain::RequirementScope::EndToEnd
        });
    let entries = if is_end_to_end {
        vec![(
            requirement,
            eqm_domain::ScenarioRef::new(
                eqm_domain::ScenarioId::new("scenario.fixture_signup")?,
                eqm_domain::Revision::new(1)?,
            ),
        )]
    } else {
        Vec::new()
    };
    Ok(eqm_domain::RequirementScenarioBindings::new(entries)?)
}

#[allow(dead_code)] // Integration crates use different parts of this shared fixture.
pub(crate) fn derive_fixture_universe(
    finalized: &eqm_engine::FinalizedWorkspaceGraph,
    selection: &eqm_engine::SelectedPolicyProfiles<'_>,
    applicability: &eqm_engine::ApplicabilityContext,
    release_context: Option<eqm_domain::Sha256Digest>,
) -> Result<eqm_engine::RequiredUniverse, eqm_engine::ObligationError> {
    let bindings = fixture_bindings(finalized.graph())
        .map_err(|_| eqm_engine::ObligationError::InvalidFinalizedGraph)?;
    eqm_engine::derive_required_universe_with_scope_bindings(
        finalized,
        selection,
        applicability,
        release_context,
        &bindings,
    )
}

#[allow(dead_code)] // Integration crates use different parts of this shared fixture.
pub(crate) fn derive_fixture_obligations(
    finalized: &eqm_engine::FinalizedWorkspaceGraph,
    selection: &eqm_engine::SelectedPolicyProfiles<'_>,
    applicability: &eqm_engine::ApplicabilityContext,
    release_context: Option<eqm_domain::Sha256Digest>,
) -> Result<eqm_engine::ObligationDerivation, eqm_engine::ObligationError> {
    let bindings = fixture_bindings(finalized.graph())
        .map_err(|_| eqm_engine::ObligationError::InvalidFinalizedGraph)?;
    eqm_engine::derive_obligations_with_scope_bindings(
        finalized,
        selection,
        applicability,
        release_context,
        &bindings,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_evaluation_time_is_injected_and_repeatable() -> Result<(), Box<dyn Error>> {
        let first = evaluation_time()?;
        assert_eq!(first, evaluation_time()?);
        assert_eq!(first.to_string(), eqm_test_support::FIXED_EVALUATION_TIME);
        Ok(())
    }
}
