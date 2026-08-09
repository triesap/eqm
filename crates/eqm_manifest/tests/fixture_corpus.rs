//! End-to-end coverage for the checked-in manifest fixture corpus.

use eqm_manifest::{LoadError, load_workspace};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn corpus() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/manifest")
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        if entry.file_name() == "GIT_HEAD.fixture" {
            let git = destination.join(".git");
            fs::create_dir_all(&git)?;
            fs::copy(entry.path(), git.join("HEAD"))?;
            continue;
        }
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn workspace() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let directory = tempfile::tempdir()?;
    copy_tree(&corpus().join("valid/minimal"), directory.path())?;
    Ok(directory)
}

fn negative(name: &str) -> PathBuf {
    corpus().join("negative").join(name)
}

#[test]
fn valid_workspace_is_repeatable_and_ignores_generated_and_nested_state()
-> Result<(), Box<dyn Error>> {
    let repository = workspace()?;
    let first = load_workspace(repository.path(), None)?;
    let second = load_workspace(repository.path(), None)?;
    assert_eq!(first, second);
    assert_eq!(first.graph_input().capabilities.len(), 1);
    Ok(())
}

#[test]
fn schema_field_unicode_and_duplicate_fixtures_fail_closed() -> Result<(), Box<dyn Error>> {
    for name in [
        "wrong-schema.toml",
        "unknown-field.toml",
        "decomposed-unicode.toml",
    ] {
        let repository = workspace()?;
        fs::copy(
            negative(name),
            repository.path().join("eqm/contracts/capability.toml"),
        )?;
        assert!(
            load_workspace(repository.path(), None).is_err(),
            "accepted {name}"
        );
    }

    let repository = workspace()?;
    fs::copy(
        negative("duplicate-authority.toml"),
        repository.path().join("eqm/contracts/duplicate.toml"),
    )?;
    assert!(matches!(
        load_workspace(repository.path(), None),
        Err(LoadError::Validation(Some(_)))
    ));
    Ok(())
}

#[test]
fn path_collision_and_lock_fixtures_fail_at_their_stage() -> Result<(), Box<dyn Error>> {
    let repository = workspace()?;
    fs::copy(
        negative("path-escape-eqm.toml"),
        repository.path().join("eqm.toml"),
    )?;
    assert_eq!(
        load_workspace(repository.path(), None),
        Err(LoadError::WorkspaceTarget)
    );

    let repository = workspace()?;
    fs::copy(
        negative("portable-root-collision-eqm.toml"),
        repository.path().join("eqm.toml"),
    )?;
    assert_eq!(
        load_workspace(repository.path(), None),
        Err(LoadError::DuplicateTargetRoot)
    );

    let repository = workspace()?;
    fs::copy(
        negative("floating-lock.toml"),
        repository.path().join("eqm.lock"),
    )?;
    assert_eq!(
        load_workspace(repository.path(), None),
        Err(LoadError::Lock)
    );
    Ok(())
}

#[test]
fn materialized_limit_collision_and_symlink_cases_fail_closed() -> Result<(), Box<dyn Error>> {
    let repository = workspace()?;
    let oversized = format!(
        "schema = \"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/capability.schema.json\"\nid = \"oversized.value\"\ntitle = \"{}\"\nstatus = \"active\"\nowners = [\"owner://team/test\"]\n",
        "x".repeat(4 * 1024 * 1024)
    );
    fs::write(
        repository.path().join("eqm/contracts/capability.toml"),
        oversized,
    )?;
    assert!(matches!(
        load_workspace(repository.path(), None),
        Err(LoadError::Validation(Some(_)))
    ));

    let repository = workspace()?;
    let original = fs::read(repository.path().join("eqm/contracts/capability.toml"))?;
    fs::write(
        repository.path().join("eqm/contracts/Capability.toml"),
        &original,
    )?;
    if fs::read_dir(repository.path().join("eqm/contracts"))?.count() == 2 {
        assert_eq!(
            load_workspace(repository.path(), None),
            Err(LoadError::Discovery)
        );
    } else {
        assert!(load_workspace(repository.path(), None).is_ok());
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let repository = workspace()?;
        symlink(
            "capability.toml",
            repository.path().join("eqm/contracts/link.toml"),
        )?;
        assert_eq!(
            load_workspace(repository.path(), None),
            Err(LoadError::Discovery)
        );
    }
    Ok(())
}
