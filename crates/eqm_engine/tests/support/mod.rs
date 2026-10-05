//! Shared integration-test materialization for the public example.

use eqm_domain::UtcInstant;
use eqm_manifest::LoadedWorkspace;
use std::error::Error;
use std::fs;
use std::path::Path;

pub(crate) fn loaded_example() -> Result<(tempfile::TempDir, LoadedWorkspace), Box<dyn Error>> {
    loaded_example_with(|_| Ok(()))
}

/// Materializes the example and lets a caller mutate it before loading.
pub(crate) fn loaded_example_with(
    mutate: impl FnOnce(&Path) -> Result<(), Box<dyn Error>>,
) -> Result<(tempfile::TempDir, LoadedWorkspace), Box<dyn Error>> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios");
    let repository = tempfile::tempdir()?;
    copy_directory(&source, repository.path())?;
    fs::create_dir(repository.path().join(".git"))?;
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
