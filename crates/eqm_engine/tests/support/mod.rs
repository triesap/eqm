//! Shared integration-test materialization for the public example.

use eqm_manifest::LoadedWorkspace;
use std::error::Error;
use std::fs;
use std::path::Path;

pub(crate) fn loaded_example() -> Result<(tempfile::TempDir, LoadedWorkspace), Box<dyn Error>> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios");
    let repository = tempfile::tempdir()?;
    copy_directory(&source, repository.path())?;
    fs::create_dir(repository.path().join(".git"))?;
    let loaded = eqm_manifest::load_workspace(repository.path(), None)?;
    Ok((repository, loaded))
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
