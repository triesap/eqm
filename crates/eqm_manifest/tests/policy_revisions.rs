//! S046 / RA16: exact policy and profile revision loading through the real
//! manifest loader. Distinct revisions must load and resolve; duplicate
//! authority for the same exact revision must reject. A constructed-graph-only
//! test is insufficient, so this drives the real loader over the example.

use eqm_manifest::load_workspace;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn example() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios");
    let repository = tempfile::tempdir()?;
    copy_tree(&source, repository.path())?;
    fs::create_dir(repository.path().join(".git"))?;
    Ok(repository)
}

#[test]
fn distinct_policy_and_profile_revisions_load_and_duplicate_revision_rejects()
-> Result<(), Box<dyn Error>> {
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

    let loaded = load_workspace(repository.path(), None)?;
    let policies = &loaded.graph_input().policies;
    assert!(
        policies
            .iter()
            .any(|policy| policy.id().as_str() == "consumer.critical_flow"
                && policy.revision().get() == 1)
    );
    assert!(
        policies
            .iter()
            .any(|policy| policy.id().as_str() == "consumer.critical_flow"
                && policy.revision().get() == 2)
    );
    let profiles = &loaded.graph_input().profiles;
    assert!(profiles.iter().any(
        |profile| profile.id().as_str() == "audience.default" && profile.revision().get() == 1
    ));
    assert!(profiles.iter().any(
        |profile| profile.id().as_str() == "audience.default" && profile.revision().get() == 2
    ));

    // Duplicate authority for the exact same revision must still reject.
    fs::copy(
        &policy_path,
        repository
            .path()
            .join("eqm/policies/consumer.critical_flow.copy.toml"),
    )?;
    assert!(load_workspace(repository.path(), None).is_err());
    Ok(())
}
