//! S041/P03-R5: authored v2 cohort reader and conversion value contracts.
//!
//! The generated cohort schema is a token/value surface; these controls prove
//! the real authored loader and domain conversion reject the same owning-value
//! defects (positive revision, identifier/scenario vocabulary, commit form and
//! exactly-one configuration authority) rather than only the JSON schema.

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

const COHORT: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/cohort.schema.json"
id = "cohort.n"
revision = 1
scenario = "signup.flow"

[[assignments]]
occurrence = "mailer"
repository = "https://example.com/team/project"
source_commit = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
source_content_digest = "sha256:1111111111111111111111111111111111111111111111111111111111111111"
provider_configuration_digest = "sha256:2222222222222222222222222222222222222222222222222222222222222222"
build_id = "build.n.mailer"
artifact_digest = "sha256:3333333333333333333333333333333333333333333333333333333333333333"
"#;

fn repository(cohort: &str) -> Result<tempfile::TempDir, Box<dyn Error>> {
    let directory = tempfile::tempdir()?;
    copy_tree(&corpus(), directory.path())?;
    fs::write(directory.path().join("eqm.toml"), WORKSPACE)?;
    fs::write(directory.path().join("eqm/contracts/cohort.n.toml"), cohort)?;
    Ok(directory)
}

#[test]
fn valid_cohort_loads_and_owning_value_defects_reject() -> Result<(), Box<dyn Error>> {
    assert!(load_workspace(repository(COHORT)?.path(), None).is_ok());

    let revision_zero = COHORT.replace("revision = 1", "revision = 0");
    assert!(load_workspace(repository(&revision_zero)?.path(), None).is_err());

    let empty_id = COHORT.replace("id = \"cohort.n\"", "id = \"\"");
    assert!(load_workspace(repository(&empty_id)?.path(), None).is_err());

    let invalid_scenario =
        COHORT.replace("scenario = \"signup.flow\"", "scenario = \"Bad Scenario\"");
    assert!(load_workspace(repository(&invalid_scenario)?.path(), None).is_err());

    let malformed_commit = COHORT.replace("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "xyz");
    assert!(load_workspace(repository(&malformed_commit)?.path(), None).is_err());

    // Neither configuration kind.
    let neither = COHORT.replace(
        "provider_configuration_digest = \"sha256:2222222222222222222222222222222222222222222222222222222222222222\"\n",
        "",
    );
    assert!(load_workspace(repository(&neither)?.path(), None).is_err());

    // Both configuration kinds.
    let both = COHORT.replace(
        "provider_configuration_digest = \"sha256:2222222222222222222222222222222222222222222222222222222222222222\"",
        "provider_configuration_digest = \"sha256:2222222222222222222222222222222222222222222222222222222222222222\"\ntarget_configuration_digest = \"sha256:4444444444444444444444444444444444444444444444444444444444444444\"",
    );
    assert!(load_workspace(repository(&both)?.path(), None).is_err());
    Ok(())
}

#[test]
fn authored_repository_and_extension_boundaries_use_real_loader() -> Result<(), Box<dyn Error>> {
    // The owning parser accepts an explicit port.
    let explicit_port = COHORT.replace(
        "https://example.com/team/project",
        "https://example.com:443/team/repo",
    );
    assert!(load_workspace(repository(&explicit_port)?.path(), None).is_ok());

    // Parser-invalid repository forms reject at the same authored ingress.
    for invalid in [
        "https://Example.com/team/repo",
        "https://example.com/team/repo/",
        "https://example.com/team/repo?x=1",
    ] {
        let mutated = COHORT.replace("https://example.com/team/project", invalid);
        assert!(
            load_workspace(repository(&mutated)?.path(), None).is_err(),
            "repository {invalid} must reject"
        );
    }

    // A valid authored extension namespace with a nested key loads.
    let valid_extension = format!("{COHORT}\n[extensions.\"dev.example.ext\"]\nflag = true\n");
    assert!(load_workspace(repository(&valid_extension)?.path(), None).is_ok());

    // A two-segment namespace rejects.
    let bad_namespace = format!("{COHORT}\n[extensions.\"dev.example\"]\nflag = true\n");
    assert!(load_workspace(repository(&bad_namespace)?.path(), None).is_err());

    // An invalid nested object key rejects.
    let bad_key =
        format!("{COHORT}\n[extensions.\"dev.example.ext\"]\nnested = {{ Bad = true }}\n");
    assert!(load_workspace(repository(&bad_key)?.path(), None).is_err());
    Ok(())
}
