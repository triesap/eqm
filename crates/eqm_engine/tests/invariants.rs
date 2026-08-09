//! End-to-end graph-invariant fixtures.

use eqm_engine::{resolve_graph, validate_graph_invariants};
use eqm_manifest::dto::FragmentDto;
use eqm_manifest::load_workspace;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn source_root() -> Result<PathBuf, Box<dyn Error>> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("workspace root unavailable")?
        .join("examples/android-ios"))
}

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

fn workspace() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let root = source_root()?;
    let directory = tempfile::tempdir()?;
    fs::create_dir(directory.path().join(".git"))?;
    for file in ["eqm.toml", "eqm.lock"] {
        fs::copy(root.join(file), directory.path().join(file))?;
    }
    copy_tree(&root.join("eqm"), &directory.path().join("eqm"))?;
    Ok(directory)
}

fn replace(path: PathBuf, before: &str, after: &str) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(&path)?;
    if !source.contains(before) {
        return Err(format!("fixture text not found in {}", path.display()).into());
    }
    fs::write(path, source.replacen(before, after, 1))?;
    Ok(())
}

#[test]
fn valid_graph_satisfies_every_invariant() -> Result<(), Box<dyn Error>> {
    let repository = workspace()?;
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    validate_graph_invariants(&graph, loaded.source_map())?;
    Ok(())
}

#[test]
fn transition_orphan_lifecycle_and_risk_violations_are_source_linked() -> Result<(), Box<dyn Error>>
{
    let repository = workspace()?;
    replace(
        repository.path().join("eqm/contracts/auth.signup.toml"),
        "surfaces = [\"account.create.signup.identifier\", \"account.create.signup.otp\"]",
        "surfaces = [\"account.create.signup.identifier\"]",
    )?;
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let error = validate_graph_invariants(&graph, loaded.source_map())
        .err()
        .ok_or("invalid transition and orphan accepted")?;
    assert!(
        error
            .diagnostics()
            .iter()
            .all(|item| item.source().is_some())
    );
    assert!(
        error
            .diagnostics()
            .iter()
            .all(|item| item.code().to_string() == "EQM-E0302")
    );

    let repository = workspace()?;
    replace(
        repository.path().join("eqm/contracts/account.create.toml"),
        "status = \"active\"",
        "status = \"deprecated\"",
    )?;
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let error = validate_graph_invariants(&graph, loaded.source_map())
        .err()
        .ok_or("inactive-parent lifecycle accepted")?;
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0302");

    let repository = workspace()?;
    replace(
        repository
            .path()
            .join("eqm/contracts/auth.signup.identifier.toml"),
        "facets = [\"behavior\"]",
        "facets = [\"behavior\"]\nrisk_class = \"low\"",
    )?;
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let error = validate_graph_invariants(&graph, loaded.source_map())
        .err()
        .ok_or("lowered requirement risk accepted")?;
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0303");

    let repository = workspace()?;
    replace(
        repository.path().join("eqm/contracts/auth.signup.toml"),
        "id = \"account.create.signup\"",
        "id = \"identity.signup.flow\"",
    )?;
    for file in ["auth.signup.identifier.toml", "auth.signup.otp.toml"] {
        replace(
            repository.path().join("eqm/contracts").join(file),
            "journey = \"account.create.signup\"",
            "journey = \"identity.signup.flow\"",
        )?;
    }
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let error = validate_graph_invariants(&graph, loaded.source_map())
        .err()
        .ok_or("invalid identifier ownership accepted")?;
    assert!(
        error
            .diagnostics()
            .iter()
            .all(|item| item.code().to_string() == "EQM-E0302")
    );
    Ok(())
}

#[test]
fn fragment_nesting_and_cycles_are_unrepresentable_in_v1() {
    let source = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/fragment.schema.json"
id = "common.cycle"
revision = 1
title = "Cycle"
risk_class = "low"
owners = ["owner://team/product"]
fragments = []

[[requirements]]
id = "present"
level = "required"
scope = "each_target"
statement = "A requirement is present."
facets = ["behavior"]
"#;
    assert!(toml::from_str::<FragmentDto>(source).is_err());
}
