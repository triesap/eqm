//! S037-R1: authored v2 scenario reader, conversion and discovery.

use eqm_domain::{ScenarioId, ScenarioSubject};
use eqm_manifest::load_workspace;
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

const WORKSPACE: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/workspace.schema.json"
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
owners = ["owner://team/web"]
"#;

const SCENARIO: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/scenario.schema.json"
id = "signup.flow"
revision = 1

[[participants]]
id = "phone_one"
role = "edge"
[participants.subject]
target = "web"

[[participants]]
id = "phone_two"
role = "edge"
[participants.subject]
target = "web"

[[participants]]
id = "mailer"
role = "provider"
[participants.subject]
provider = "provider.mailgun"

[[dependencies]]
from = "phone_two"
to = "phone_one"
"#;

fn repository(scenario: &str) -> Result<tempfile::TempDir, Box<dyn Error>> {
    let directory = tempfile::tempdir()?;
    copy_tree(&corpus(), directory.path())?;
    fs::write(directory.path().join("eqm.toml"), WORKSPACE)?;
    fs::write(
        directory.path().join("eqm/contracts/scenario.signup.toml"),
        scenario,
    )?;
    Ok(directory)
}

#[test]
fn valid_scenario_loads_with_repeated_actors_and_tagged_subjects() -> Result<(), Box<dyn Error>> {
    let repository = repository(SCENARIO)?;
    let loaded = load_workspace(repository.path(), None)?;
    assert_eq!(loaded.graph_input().scenarios.len(), 1);
    let scenario = &loaded.graph_input().scenarios[0];
    assert_eq!(scenario.id(), &ScenarioId::new("signup.flow")?);
    assert_eq!(scenario.participants().len(), 3);
    assert_eq!(scenario.dependencies().len(), 1);
    let roles: Vec<&str> = scenario
        .participants()
        .iter()
        .map(|participant| participant.role().as_str())
        .collect();
    assert_eq!(roles, ["provider", "edge", "edge"]);
    assert!(scenario.participants().iter().any(|participant| {
        matches!(participant.subject(), ScenarioSubject::Provider(provider)
            if provider.as_str() == "provider.mailgun")
    }));
    assert!(scenario.participants().iter().any(|participant| {
        matches!(participant.subject(), ScenarioSubject::Target(target)
            if target.as_str() == "web")
    }));
    Ok(())
}

#[test]
fn duplicate_occurrence_malformed_role_and_missing_role_reject() -> Result<(), Box<dyn Error>> {
    let duplicate = SCENARIO.replace("id = \"phone_two\"", "id = \"phone_one\"");
    assert!(load_workspace(repository(&duplicate)?.path(), None).is_err());

    let bad_role = SCENARIO.replace("role = \"edge\"", "role = \"Bad-Role\"");
    assert!(load_workspace(repository(&bad_role)?.path(), None).is_err());

    let missing_role = SCENARIO.replace("role = \"edge\"\n", "");
    assert!(load_workspace(repository(&missing_role)?.path(), None).is_err());
    Ok(())
}

#[test]
fn subject_tag_and_dependency_endpoints_are_strict() -> Result<(), Box<dyn Error>> {
    // Neither tag.
    let no_tag = SCENARIO.replace(
        "[participants.subject]\ntarget = \"web\"",
        "[participants.subject]",
    );
    assert!(load_workspace(repository(&no_tag)?.path(), None).is_err());

    // Both tags.
    let both = SCENARIO.replace(
        "[participants.subject]\ntarget = \"web\"",
        "[participants.subject]\ntarget = \"web\"\nprovider = \"provider.x\"",
    );
    assert!(load_workspace(repository(&both)?.path(), None).is_err());

    // Unknown dependency occurrence.
    let dangling = SCENARIO.replace("from = \"phone_two\"", "from = \"phone_absent\"");
    assert!(load_workspace(repository(&dangling)?.path(), None).is_err());

    // Occurrence endpoints use the owning single-segment grammar.
    let bad_endpoint = SCENARIO.replace("from = \"phone_two\"", "from = \"phone.two\"");
    assert!(load_workspace(repository(&bad_endpoint)?.path(), None).is_err());

    // Self dependency.
    let self_edge = SCENARIO.replace("to = \"phone_one\"", "to = \"phone_two\"");
    assert!(load_workspace(repository(&self_edge)?.path(), None).is_err());
    Ok(())
}

#[test]
fn provider_and_target_subjects_are_distinct_members() -> Result<(), Box<dyn Error>> {
    let repository = repository(SCENARIO)?;
    let loaded = load_workspace(repository.path(), None)?;
    let scenario = &loaded.graph_input().scenarios[0];
    // Two edge occurrences that share a target are distinct members.
    let edges = scenario
        .participants()
        .iter()
        .filter(|participant| participant.role().as_str() == "edge")
        .count();
    assert_eq!(edges, 2);
    Ok(())
}
