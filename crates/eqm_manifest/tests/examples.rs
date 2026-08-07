//! Validation of the repository's complete authored example workspace.

use eqm_domain::Sha256Digest;
use eqm_manifest::load_workspace;
use std::error::Error;
use std::fs;
use std::path::Path;

#[test]
fn repository_examples_load_through_the_real_loader() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("workspace root unavailable")?;
    let loaded = load_workspace(root, None)?;
    let graph = loaded.graph_input();
    assert_eq!(graph.capabilities.len(), 1);
    assert_eq!(graph.journeys.len(), 1);
    assert_eq!(graph.surfaces.len(), 2);
    assert_eq!(graph.fragments.len(), 1);
    assert_eq!(graph.bindings.len(), 1);
    assert_eq!(graph.policies.len(), 1);
    assert_eq!(graph.profiles.len(), 1);
    assert_eq!(graph.runners.len(), 1);
    assert_eq!(graph.waivers.len(), 1);
    Ok(())
}

#[test]
fn example_fragment_pin_is_the_exact_source_content_digest() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("workspace root unavailable")?;
    let fragment = fs::read(root.join("eqm/contracts/auth.otp_entry.toml"))?;
    let surface = fs::read_to_string(root.join("eqm/contracts/auth.signup.otp.toml"))?;
    let digest = Sha256Digest::hash_content(&fragment).to_string();
    assert!(surface.contains(&format!("digest = \"{digest}\"")));
    assert!(!surface.to_ascii_lowercase().contains("placeholder"));
    Ok(())
}

#[test]
fn positive_examples_are_current_toml_and_contain_no_placeholder_digest()
-> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("workspace root unavailable")?;
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
            assert!(source.contains("https://schemas.equivalencematrix.dev/v1/"));
            assert!(!source.to_ascii_lowercase().contains("placeholder"));
        }
    }
    Ok(())
}
