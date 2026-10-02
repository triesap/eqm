//! S031: the additive v2 workspace family round-trips through the real loader.

use eqm_domain::{RoleId, SourceComponentId, TargetId};
use eqm_manifest::{canonicalize_target_configuration_v2, load_workspace};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn corpus() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/manifest/valid/minimal")
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

fn v2_workspace() -> String {
    r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/workspace.schema.json"
contract_sources = ["eqm/contracts/**/*.toml"]
binding_sources = ["eqm/bindings/**/*.toml"]
policy_sources = ["eqm/policies/**/*.toml"]
profile_sources = ["eqm/profiles/**/*.toml"]
runner_sources = ["eqm/runners/**/*.toml"]
waiver_sources = ["eqm/waivers/**/*.toml"]

[targets.web]
root = "apps/web"
platform = "web"
framework = "sveltekit"
role = "control_tower"
source_components = ["shared.kmp"]
owners = ["owner://team/web"]

[source_components."shared.kmp"]
roots = ["shared/kmp"]
owners = ["owner://team/shared"]
dependencies = []
"#
    .to_owned()
}

fn repository(workspace: &str) -> Result<tempfile::TempDir, Box<dyn Error>> {
    let directory = tempfile::tempdir()?;
    copy_tree(&corpus(), directory.path())?;
    fs::write(directory.path().join("eqm.toml"), workspace)?;
    Ok(directory)
}

#[test]
fn v2_workspace_loads_role_and_source_components_through_the_real_loader()
-> Result<(), Box<dyn Error>> {
    let repository = repository(&v2_workspace())?;
    let loaded = load_workspace(repository.path(), None)?;
    assert_eq!(loaded.graph_input().targets.len(), 1);
    let target = loaded
        .graph_input()
        .targets
        .iter()
        .find(|target| target.id().as_str() == "web")
        .ok_or("web target missing")?;
    assert_eq!(target.role().map(RoleId::as_str), Some("control_tower"));
    assert!(
        target
            .source_components()
            .contains(&SourceComponentId::new("shared.kmp")?)
    );
    assert_eq!(loaded.graph_input().source_components.len(), 1);
    assert_eq!(
        loaded.graph_input().source_components[0].id().as_str(),
        "shared.kmp"
    );
    // The loaded role/source references reach the target v2 identity.
    let canonical = canonicalize_target_configuration_v2(target)?;
    let text = std::str::from_utf8(canonical.bytes())?;
    assert!(text.contains(r#""role":"control_tower""#));
    assert!(text.contains(r#""source_components":["shared.kmp"]"#));
    assert_eq!(
        canonical.digest(),
        eqm_domain::TargetConfigurationDigest::hash_content(canonical.bytes())
    );
    Ok(())
}

#[test]
fn v2_role_and_source_references_move_the_target_identity() -> Result<(), Box<dyn Error>> {
    let baseline = repository(&v2_workspace())?;
    let baseline_loaded = load_workspace(baseline.path(), None)?;
    let baseline_target = baseline_loaded
        .graph_input()
        .targets
        .iter()
        .find(|target| target.id().as_str() == "web")
        .ok_or("web target missing")?;
    let baseline_digest = canonicalize_target_configuration_v2(baseline_target)?.digest();

    let changed = repository(&v2_workspace().replace("control_tower", "operator"))?;
    let changed_loaded = load_workspace(changed.path(), None)?;
    let changed_target = changed_loaded
        .graph_input()
        .targets
        .iter()
        .find(|target| target.id().as_str() == "web")
        .ok_or("web target missing")?;
    assert_ne!(
        baseline_digest,
        canonicalize_target_configuration_v2(changed_target)?.digest()
    );
    assert_eq!(changed_target.id(), &TargetId::new("web")?);
    Ok(())
}

#[test]
fn v2_unknown_field_fails_closed_and_authored_dangling_reference_loads()
-> Result<(), Box<dyn Error>> {
    let unknown = repository(&v2_workspace().replace(
        "framework = \"sveltekit\"",
        "framework = \"sveltekit\"\nbogus = true",
    ))?;
    assert!(load_workspace(unknown.path(), None).is_err());

    // A dangling component reference is an authored document that the loader
    // accepts; the engine's `resolve_graph` rejects it (engine test lane).
    let dangling = repository(&v2_workspace().replace(
        "source_components = [\"shared.kmp\"]",
        "source_components = [\"shared.absent\"]",
    ))?;
    let loaded = load_workspace(dangling.path(), None)?;
    assert_eq!(loaded.graph_input().targets.len(), 1);
    assert_eq!(loaded.graph_input().source_components.len(), 1);
    Ok(())
}
