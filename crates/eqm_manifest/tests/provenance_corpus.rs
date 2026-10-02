//! S027: shared component/source provenance corpus at the real loader boundary.

use eqm_manifest::{
    canonicalize_binding_v2, canonicalize_evidence_spec_v2, canonicalize_target_configuration_v2,
    load_workspace,
};
use std::error::Error;
use std::fs;
use std::path::Path;

fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        if entry.file_name() == "GIT_HEAD.fixture" {
            fs::create_dir_all(destination.join(".git"))?;
            fs::copy(entry.path(), destination.join(".git/HEAD"))?;
        } else if entry.file_name() == "eqm.toml.fixture" {
            fs::copy(entry.path(), destination.join("eqm.toml"))?;
        } else if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &destination.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn fixture() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/signup");
    let root = tempfile::tempdir()?;
    copy_tree(&source, root.path())?;
    Ok(root)
}

fn android_binding_digest(root: &Path) -> Result<eqm_domain::BindingDigest, Box<dyn Error>> {
    let loaded = load_workspace(root, None)?;
    let binding = loaded
        .graph_input()
        .bindings
        .iter()
        .find(|binding| binding.id().as_str() == "binding.android.signup")
        .ok_or("android binding missing")?;
    Ok(canonicalize_binding_v2(binding)?.digest())
}

fn android_spec_digest(
    root: &Path,
) -> Result<eqm_domain::EvidenceSpecificationDigest, Box<dyn Error>> {
    let loaded = load_workspace(root, None)?;
    let binding = loaded
        .graph_input()
        .bindings
        .iter()
        .find(|binding| binding.id().as_str() == "binding.android.signup")
        .ok_or("android binding missing")?;
    let spec = binding
        .evidence()
        .values()
        .next()
        .ok_or("android evidence specification missing")?;
    Ok(canonicalize_evidence_spec_v2(spec)?.digest())
}

fn web_target_digest(root: &Path) -> Result<eqm_domain::TargetConfigurationDigest, Box<dyn Error>> {
    let loaded = load_workspace(root, None)?;
    let target = loaded
        .graph_input()
        .targets
        .iter()
        .find(|target| target.id().as_str() == "web")
        .ok_or("web target missing")?;
    Ok(canonicalize_target_configuration_v2(target)?.digest())
}

#[test]
fn component_corpus_is_deterministic_across_repeated_loads() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    let binding = android_binding_digest(repository.path())?;
    let spec = android_spec_digest(repository.path())?;
    let target = web_target_digest(repository.path())?;
    assert_eq!(binding, android_binding_digest(repository.path())?);
    assert_eq!(spec, android_spec_digest(repository.path())?);
    assert_eq!(target, web_target_digest(repository.path())?);
    // Component identities are distinct domains over distinct bytes.
    assert_ne!(binding.to_string(), spec.to_string());
    assert_ne!(target.to_string(), spec.to_string());
    Ok(())
}

#[test]
fn formatting_only_authored_change_preserves_component_identities() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    let binding = android_binding_digest(repository.path())?;
    let spec = android_spec_digest(repository.path())?;
    let target = web_target_digest(repository.path())?;

    let path = repository.path().join("eqm/bindings/android.toml");
    let original = fs::read_to_string(&path)?;
    fs::write(
        &path,
        format!("# authored formatting comment\n\n{original}\n"),
    )?;

    assert_eq!(binding, android_binding_digest(repository.path())?);
    assert_eq!(spec, android_spec_digest(repository.path())?);
    assert_eq!(target, web_target_digest(repository.path())?);
    Ok(())
}

#[test]
fn semantic_binding_mutation_moves_only_the_binding_identity() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    let binding = android_binding_digest(repository.path())?;
    let spec = android_spec_digest(repository.path())?;
    let target = web_target_digest(repository.path())?;

    let path = repository.path().join("eqm/bindings/android.toml");
    let original = fs::read_to_string(&path)?;
    let mutated = original.replace("SignupScreen.kt", "SignupScreenV2.kt");
    assert_ne!(mutated, original, "the binding mutation must be real");
    fs::write(&path, mutated)?;

    assert_ne!(binding, android_binding_digest(repository.path())?);
    assert_eq!(spec, android_spec_digest(repository.path())?);
    assert_eq!(target, web_target_digest(repository.path())?);
    Ok(())
}

#[test]
fn semantic_target_mutation_moves_only_the_target_configuration_identity()
-> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    let binding = android_binding_digest(repository.path())?;
    let spec = android_spec_digest(repository.path())?;
    let target = web_target_digest(repository.path())?;

    let path = repository.path().join("eqm.toml");
    let config = fs::read_to_string(&path)?;
    let mutated = config.replace("framework = \"sveltekit\"", "framework = \"sveltekit2\"");
    assert_ne!(mutated, config);
    fs::write(&path, mutated)?;

    assert_ne!(
        web_target_digest(repository.path())?,
        target,
        "a semantic target change must move the target identity"
    );
    assert_eq!(
        binding,
        android_binding_digest(repository.path())?,
        "an unrelated target change must not move the binding identity"
    );
    assert_eq!(spec, android_spec_digest(repository.path())?);
    Ok(())
}
