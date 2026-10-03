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

fn shared_roots_workspace(extra_target: &str, extra_component: &str) -> String {
    format!(
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
source_components = ["shared.kmp"]
owners = ["owner://team/web"]
{extra_target}
[source_components."shared.kmp"]
roots = ["shared/kmp"]
owners = ["owner://team/shared"]
dependencies = []
{extra_component}
"#
    )
}

#[test]
fn declared_shared_component_root_is_not_an_output_collision() -> Result<(), Box<dyn Error>> {
    let extra_target = r#"
[targets.ios]
root = "apps/ios"
platform = "ios"
framework = "swiftui"
source_components = ["shared.kmp"]
owners = ["owner://team/ios"]
"#;
    let repository = repository(&shared_roots_workspace(extra_target, ""))?;
    let loaded = load_workspace(repository.path(), None)?;
    assert_eq!(loaded.graph_input().targets.len(), 2);
    assert_eq!(loaded.graph_input().source_components.len(), 1);
    Ok(())
}

#[test]
fn ambiguous_overlapping_component_roots_reject() -> Result<(), Box<dyn Error>> {
    let extra_component = r#"
[source_components."shared.other"]
roots = ["shared/kmp/other"]
owners = ["owner://team/other"]
dependencies = []
"#;
    let repository = repository(&shared_roots_workspace("", extra_component))?;
    assert_eq!(
        load_workspace(repository.path(), None),
        Err(eqm_manifest::LoadError::AmbiguousSourceOverlap)
    );
    Ok(())
}

#[test]
fn undeclared_target_root_overlap_rejects() -> Result<(), Box<dyn Error>> {
    let workspace = shared_roots_workspace("", "")
        .replace("root = \"apps/web\"", "root = \"shared/kmp/app\"")
        .replace(
            "source_components = [\"shared.kmp\"]",
            "source_components = []",
        );
    let repository = repository(&workspace)?;
    assert_eq!(
        load_workspace(repository.path(), None),
        Err(eqm_manifest::LoadError::UndeclaredSourceOverlap)
    );
    Ok(())
}

#[test]
fn escaped_component_root_rejects() -> Result<(), Box<dyn Error>> {
    let workspace = shared_roots_workspace("", "")
        .replace("roots = [\"shared/kmp\"]", "roots = [\"../escape\"]");
    let repository = repository(&workspace)?;
    assert_eq!(
        load_workspace(repository.path(), None),
        Err(eqm_manifest::LoadError::WorkspaceTarget)
    );
    Ok(())
}

#[test]
fn component_roots_require_nonempty_duplicate_free_portable_paths() -> Result<(), Box<dyn Error>> {
    // Same-ingress positive: two distinct valid portable roots load unchanged.
    let positive = repository(&shared_roots_workspace("", "").replace(
        "roots = [\"shared/kmp\"]",
        "roots = [\"shared/kmp\", \"shared/nested\"]",
    ))?;
    let loaded = load_workspace(positive.path(), None)?;
    assert_eq!(loaded.graph_input().source_components.len(), 1);
    assert_eq!(loaded.graph_input().source_components[0].roots().len(), 2);

    // One-axis negatives at the same authored ingress.
    for roots in [
        "[]",
        "[\"shared/kmp\", \"shared/kmp\"]",
        "[\"/abs\"]",
        "[\"shared//kmp\"]",
        "[\"CON\"]",
        "[\"shared/kmp.\"]",
    ] {
        let workspace = shared_roots_workspace("", "")
            .replace("roots = [\"shared/kmp\"]", &format!("roots = {roots}"));
        let repository = repository(&workspace)?;
        assert_eq!(
            load_workspace(repository.path(), None),
            Err(eqm_manifest::LoadError::WorkspaceTarget),
            "roots={roots}"
        );
    }
    Ok(())
}

const PARTICIPATION: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/participation.schema.json"
id = "participation.mobile"
requirement = "account.create#create"
targets = ["web"]

[applicability]
always = true
"#;

#[test]
fn v2_participation_rule_loads_through_the_real_loader() -> Result<(), Box<dyn Error>> {
    let repository = repository(&v2_workspace())?;
    fs::write(
        repository
            .path()
            .join("eqm/contracts/participation.mobile.toml"),
        PARTICIPATION,
    )?;
    let loaded = load_workspace(repository.path(), None)?;
    assert_eq!(loaded.graph_input().participation.len(), 1);
    let rule = &loaded.graph_input().participation[0];
    assert_eq!(rule.id().as_str(), "participation.mobile");
    assert_eq!(rule.requirement().as_str(), "account.create#create");
    assert!(rule.targets().contains(&TargetId::new("web")?));
    Ok(())
}

#[test]
fn v2_participation_unknown_field_and_wrong_version_fail_closed() -> Result<(), Box<dyn Error>> {
    let unknown = repository(&v2_workspace())?;
    fs::write(
        unknown
            .path()
            .join("eqm/contracts/participation.mobile.toml"),
        PARTICIPATION.replace("targets = [\"web\"]", "targets = [\"web\"]\nbogus = true"),
    )?;
    assert!(load_workspace(unknown.path(), None).is_err());

    let v1 = repository(&v2_workspace())?;
    fs::write(
        v1.path().join("eqm/contracts/participation.mobile.toml"),
        PARTICIPATION.replace(
            "schemas/v2/manifest/participation",
            "schemas/v1/manifest/participation",
        ),
    )?;
    assert!(load_workspace(v1.path(), None).is_err());
    Ok(())
}

fn closure_workspace(targets: &str, components: &str) -> String {
    format!(
        r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/workspace.schema.json"
contract_sources = ["eqm/contracts/**/*.toml"]
binding_sources = ["eqm/bindings/**/*.toml"]
policy_sources = ["eqm/policies/**/*.toml"]
profile_sources = ["eqm/profiles/**/*.toml"]
runner_sources = ["eqm/runners/**/*.toml"]
waiver_sources = ["eqm/waivers/**/*.toml"]

{targets}
{components}
"#
    )
}

const WEB_TARGET: &str = r#"[targets.web]
root = "apps/web"
platform = "web"
framework = "sveltekit"
source_components = ["shared.a"]
owners = ["owner://team/web"]
"#;

#[test]
fn target_root_overlap_is_allowed_through_the_transitive_component_closure()
-> Result<(), Box<dyn Error>> {
    // shared.b overlaps the web output root but is reachable from shared.a,
    // which the target declares. The same closure owns the shared roots.
    let components = r#"[source_components."shared.a"]
roots = ["shared/a"]
owners = ["owner://team/shared"]
dependencies = ["shared.b"]

[source_components."shared.b"]
roots = ["apps/web/shared"]
owners = ["owner://team/shared"]
dependencies = []
"#;
    let repository = repository(&closure_workspace(WEB_TARGET, components))?;
    let loaded = load_workspace(repository.path(), None)?;
    assert_eq!(loaded.graph_input().targets.len(), 1);
    assert_eq!(loaded.graph_input().source_components.len(), 2);
    Ok(())
}

#[test]
fn same_target_root_overlap_without_the_closure_rejects() -> Result<(), Box<dyn Error>> {
    // Identical paths, but shared.a no longer declares shared.b.
    let components = r#"[source_components."shared.a"]
roots = ["shared/a"]
owners = ["owner://team/shared"]
dependencies = []

[source_components."shared.b"]
roots = ["apps/web/shared"]
owners = ["owner://team/shared"]
dependencies = []
"#;
    let repository = repository(&closure_workspace(WEB_TARGET, components))?;
    assert_eq!(
        load_workspace(repository.path(), None),
        Err(eqm_manifest::LoadError::UndeclaredSourceOverlap)
    );
    Ok(())
}

#[test]
fn unrelated_component_overlapping_the_target_root_rejects() -> Result<(), Box<dyn Error>> {
    let components = r#"[source_components."shared.a"]
roots = ["shared/a"]
owners = ["owner://team/shared"]
dependencies = []

[source_components."shared.unrelated"]
roots = ["apps/web/unrelated"]
owners = ["owner://team/other"]
dependencies = []
"#;
    let repository = repository(&closure_workspace(WEB_TARGET, components))?;
    assert_eq!(
        load_workspace(repository.path(), None),
        Err(eqm_manifest::LoadError::UndeclaredSourceOverlap)
    );
    Ok(())
}

#[test]
fn transitive_component_overlap_is_explained_but_unrelated_overlap_rejects()
-> Result<(), Box<dyn Error>> {
    let transitive = r#"[source_components."shared.a"]
roots = ["shared/a"]
owners = ["owner://team/shared"]
dependencies = ["shared.b"]

[source_components."shared.b"]
roots = ["shared/b"]
owners = ["owner://team/shared"]
dependencies = ["shared.c"]

[source_components."shared.c"]
roots = ["shared/a/nested"]
owners = ["owner://team/shared"]
dependencies = []
"#;
    let transitive_repo = repository(&closure_workspace(WEB_TARGET, transitive))?;
    let loaded = load_workspace(transitive_repo.path(), None)?;
    assert_eq!(loaded.graph_input().source_components.len(), 3);

    let unrelated = transitive.replace("dependencies = [\"shared.b\"]", "dependencies = []");
    let unrelated_repo = repository(&closure_workspace(WEB_TARGET, &unrelated))?;
    assert_eq!(
        load_workspace(unrelated_repo.path(), None),
        Err(eqm_manifest::LoadError::AmbiguousSourceOverlap)
    );
    Ok(())
}
