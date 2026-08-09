//! Validation of the repository's complete authored example workspace.

use eqm_manifest::{canonicalize_fragment, load_workspace};
use std::error::Error;
use std::fs;
use std::path::Path;

fn example_source() -> Result<std::path::PathBuf, Box<dyn Error>> {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("workspace root unavailable")?;
    Ok(workspace.join("examples/android-ios"))
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

fn standalone_example() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let repository = tempfile::tempdir()?;
    copy_directory(&example_source()?, repository.path())?;
    fs::create_dir(repository.path().join(".git"))?;
    Ok(repository)
}

#[test]
fn repository_examples_load_through_the_real_loader() -> Result<(), Box<dyn Error>> {
    let repository = standalone_example()?;
    let loaded = load_workspace(repository.path(), None)?;
    let graph = loaded.graph_input();
    assert_eq!(graph.capabilities.len(), 1);
    assert_eq!(graph.journeys.len(), 1);
    assert_eq!(graph.surfaces.len(), 2);
    assert_eq!(graph.fragments.len(), 1);
    assert_eq!(graph.bindings.len(), 2);
    assert_eq!(graph.policies.len(), 1);
    assert_eq!(graph.profiles.len(), 1);
    assert_eq!(graph.runners.len(), 2);
    assert_eq!(graph.waivers.len(), 1);
    Ok(())
}

#[test]
fn example_fragment_pin_is_the_exact_semantic_digest() -> Result<(), Box<dyn Error>> {
    let repository = standalone_example()?;
    let root = repository.path();
    let loaded = load_workspace(root, None)?;
    let fragment = loaded
        .graph_input()
        .fragments
        .first()
        .ok_or("fragment unavailable")?;
    let surface = fs::read_to_string(root.join("eqm/contracts/auth.signup.otp.toml"))?;
    let digest = canonicalize_fragment(fragment)?.digest().to_string();
    assert!(surface.contains(&format!("digest = \"{digest}\"")));
    assert!(!surface.to_ascii_lowercase().contains("placeholder"));
    Ok(())
}

#[test]
fn positive_examples_are_current_toml_and_contain_no_placeholder_digest()
-> Result<(), Box<dyn Error>> {
    let root = example_source()?;
    for directory in [
        "eqm/contracts",
        "eqm/bindings",
        "eqm/policies",
        "eqm/profiles",
        "eqm/runners",
        "eqm/waivers",
    ] {
        for entry in fs::read_dir(root.join(directory))? {
            let entry = entry?;
            if entry.path().extension().and_then(|value| value.to_str()) != Some("toml") {
                continue;
            }
            let source = fs::read_to_string(entry.path())?;
            let _: toml::Table = toml::from_str(&source)?;
            assert!(source.contains(
                "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/"
            ));
            assert!(!source.to_ascii_lowercase().contains("placeholder"));
        }
    }
    Ok(())
}
