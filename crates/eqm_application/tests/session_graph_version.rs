//! P03-R1: prepared-session graph version routing and v2 identity.
//!
//! A genuine v1 workspace prepares under the frozen `eqm:v1:semantic-graph`
//! domain, while an authored v2 workspace prepares under the complete
//! `eqm:v2:semantic-graph` identity. A one-axis role mutation moves the v2
//! workspace digest, and a v1 workspace carrying v2-only participation
//! authority is rejected rather than projected as v1.

use eqm_application::{SessionError, SessionRequest, prepare};
use eqm_domain::{DigestDomain, SchemaVersion, UtcInstant};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const V1_CONFIG: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/workspace.schema.json"
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

fn v2_config(role: &str) -> String {
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
role = "{role}"
source_components = ["shared.kmp"]
owners = ["owner://team/web"]

[source_components."shared.kmp"]
roots = ["shared/kmp"]
owners = ["owner://team/shared"]
dependencies = []
"#
    )
}

const PARTICIPATION: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/participation.schema.json"
id = "participation.mobile"
requirement = "account.create.signup.identifier#create"
targets = ["web"]

[applicability]
always = true
"#;

const JOURNEY: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/journey.schema.json"
id = "account.create.signup"
revision = 1
title = "Signup"
capability = "account.create"
status = "active"
risk_class = "high"
owners = ["owner://team/accounts"]
surfaces = ["account.create.signup.identifier"]
"#;

const SURFACE: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/surface.schema.json"
id = "account.create.signup.identifier"
revision = 1
title = "Create account"
journey = "account.create.signup"
status = "active"
owners = ["owner://team/accounts"]

[[requirements]]
id = "create"
level = "required"
scope = "each_target"
statement = "An account can be created."
facets = ["behavior"]
"#;

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

fn fixture() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let corpus = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../eqm_manifest/tests/fixtures/manifest/valid/minimal");
    let directory = tempfile::tempdir()?;
    copy_tree(&corpus, directory.path())?;
    Ok(directory)
}

fn evaluation_time() -> Result<UtcInstant, Box<dyn Error>> {
    Ok("2026-01-02T03:04:05Z".parse()?)
}

#[test]
fn v2_workspace_prepares_under_the_complete_v2_graph_domain() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(
        repository.path().join("eqm.toml"),
        v2_config("control_tower"),
    )?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    assert_eq!(session.semantic_graph_version(), SchemaVersion::V2);
    assert_eq!(
        session.semantic_graph_domain(),
        DigestDomain::SemanticGraphV2
    );
    let expected = eqm_manifest::canonicalize_graph_v2(session.finalized())?
        .digest()
        .as_digest();
    assert_eq!(session.workspace_digest(), expected);
    Ok(())
}

#[test]
fn one_axis_role_mutation_moves_the_v2_workspace_digest() -> Result<(), Box<dyn Error>> {
    let baseline = fixture()?;
    fs::write(baseline.path().join("eqm.toml"), v2_config("control_tower"))?;
    let baseline = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        baseline.path(),
    )?;

    let changed = fixture()?;
    fs::write(changed.path().join("eqm.toml"), v2_config("operator"))?;
    let changed = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        changed.path(),
    )?;

    assert_eq!(baseline.semantic_graph_version(), SchemaVersion::V2);
    assert_ne!(baseline.workspace_digest(), changed.workspace_digest());
    Ok(())
}

#[test]
fn v1_workspace_prepares_under_the_frozen_v1_graph_domain() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(repository.path().join("eqm.toml"), V1_CONFIG)?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    assert_eq!(session.semantic_graph_version(), SchemaVersion::V1);
    assert_eq!(session.semantic_graph_domain(), DigestDomain::SemanticGraph);
    let expected = eqm_manifest::canonicalize_graph(session.finalized())?.digest();
    assert_eq!(session.workspace_digest(), expected);
    Ok(())
}

#[test]
fn v1_workspace_with_v2_participation_authority_is_rejected_not_projected()
-> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(repository.path().join("eqm.toml"), V1_CONFIG)?;
    fs::write(
        repository.path().join("eqm/contracts/journey.toml"),
        JOURNEY,
    )?;
    fs::write(
        repository.path().join("eqm/contracts/surface.toml"),
        SURFACE,
    )?;
    fs::write(
        repository
            .path()
            .join("eqm/contracts/participation.mobile.toml"),
        PARTICIPATION,
    )?;
    match prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    ) {
        Err(SessionError::Canonicalize(eqm_manifest::CanonicalizationError::V2OnlyMetadata)) => {}
        other => {
            return Err(format!(
                "expected v1 projection to reject v2-only metadata, got {other:?}"
            )
            .into());
        }
    }
    Ok(())
}
