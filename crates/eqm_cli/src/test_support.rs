//! Test-only materialization of the public Android/iOS example.

use std::error::Error;
use std::fs;
use std::path::Path;

pub(crate) fn example_repository() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios");
    let repository = tempfile::tempdir()?;
    copy_directory(&source, repository.path())?;
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    fs::copy(
        workspace.join("Cargo.toml"),
        repository.path().join("Cargo.toml"),
    )?;
    fs::copy(
        workspace.join("rust-toolchain.toml"),
        repository.path().join("rust-toolchain.toml"),
    )?;
    fs::create_dir(repository.path().join(".git"))?;
    Ok(repository)
}

pub(crate) fn copy_directory(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
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
