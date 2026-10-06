//! P03-R1: prepared-session graph version routing and v2 identity.
//!
//! A genuine v1 workspace prepares under the frozen `eqm:v1:semantic-graph`
//! domain, while an authored v2 workspace prepares under the complete
//! `eqm:v2:semantic-graph` identity. A one-axis role mutation moves the v2
//! workspace digest, and a v1 workspace carrying v2-only participation
//! authority is rejected rather than projected as v1.

use eqm_application::{SessionError, SessionRequest, prepare};
use eqm_domain::{
    DigestDomain, Extensions, FrameworkId, OwnerRef, PlatformId, Provider, ProviderId, RepoPath,
    RoleId, SchemaVersion, Sha256Digest, SourceComponentId, Target, TargetId, UtcInstant,
};
use serde_json::json;
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
    let expected = eqm_manifest::canonicalize_graph_v2(session.finalized().graph())?
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
    let expected = eqm_manifest::canonicalize_graph(session.finalized().graph())?.digest();
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

#[test]
fn v2_conflicting_participation_is_rejected_at_prepare() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(
        repository.path().join("eqm.toml"),
        v2_config("control_tower"),
    )?;
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
    fs::write(
        repository
            .path()
            .join("eqm/contracts/participation.mobile.alt.toml"),
        PARTICIPATION
            .replace("participation.mobile", "participation.mobile.alt")
            .replace("always = true", "always = false"),
    )?;
    match prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    ) {
        Err(SessionError::Resolve(eqm_engine::ResolutionError::Graph(
            eqm_domain::WorkspaceGraphBuildError::DuplicateParticipationCoordinate,
        ))) => {}
        other => {
            return Err(format!(
                "expected conflicting participation to be rejected at prepare, got {other:?}"
            )
            .into());
        }
    }
    Ok(())
}

const RUNNER: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/runner.schema.json"
id = "runner.tests"
revision = 1
owners = ["owner://team/platform"]
backend = "local"
program = "tools/test-runner"
args = ["{target_root}", "{selector_json}", "{result_path}"]
cwd = "{target_root}"
timeout_ms = 30000
max_output_bytes = 1048576
max_concurrency = 1
"#;

const SCENARIO: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/scenario.schema.json"
id = "signup.flow"
revision = 1

[[participants]]
id = "mailer"
role = "provider"
[participants.subject]
provider = "provider.mailgun"

[[participants]]
id = "phone"
role = "edge"
[participants.subject]
target = "web"
"#;

const PROVIDER: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/provider.schema.json"
id = "provider.mailgun"
configuration_digest = "sha256:071ca2227754705837aa3ef9748ed59e9f8a015fd765c42f391a4cbc271c6d5e"
owners = ["owner://team/providers"]
"#;

#[test]
fn v2_scenario_with_known_provider_prepares() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(
        repository.path().join("eqm.toml"),
        v2_config("control_tower"),
    )?;
    fs::write(
        repository.path().join("eqm/contracts/scenario.signup.toml"),
        SCENARIO,
    )?;
    fs::write(
        repository
            .path()
            .join("eqm/contracts/provider.mailgun.toml"),
        PROVIDER,
    )?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    assert_eq!(session.semantic_graph_version(), SchemaVersion::V2);
    assert_eq!(session.finalized().graph().scenarios().len(), 1);
    assert_eq!(session.finalized().graph().providers().len(), 1);
    Ok(())
}

#[test]
fn v2_scenario_with_missing_provider_is_rejected() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(
        repository.path().join("eqm.toml"),
        v2_config("control_tower"),
    )?;
    fs::write(
        repository.path().join("eqm/contracts/scenario.signup.toml"),
        SCENARIO,
    )?;
    match prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    ) {
        Err(SessionError::Resolve(eqm_engine::ResolutionError::Findings(findings))) => {
            assert!(!findings.is_empty());
        }
        other => {
            return Err(
                format!("expected a missing-provider resolution finding, got {other:?}").into(),
            );
        }
    }
    Ok(())
}

#[test]
fn v2_scenario_missing_target_reference_is_rejected() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(
        repository.path().join("eqm.toml"),
        v2_config("control_tower"),
    )?;
    fs::write(
        repository.path().join("eqm/contracts/scenario.signup.toml"),
        SCENARIO.replace("target = \"web\"", "target = \"absent\""),
    )?;
    match prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    ) {
        Err(SessionError::Resolve(eqm_engine::ResolutionError::Findings(findings))) => {
            assert!(!findings.is_empty());
        }
        other => {
            return Err(
                format!("expected a missing-target resolution finding, got {other:?}").into(),
            );
        }
    }
    Ok(())
}

#[test]
fn prepared_session_runner_templates_project_as_lossless_tagged_records()
-> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(
        repository.path().join("eqm.toml"),
        v2_config("control_tower"),
    )?;
    fs::create_dir_all(repository.path().join("eqm/runners"))?;
    fs::write(
        repository.path().join("eqm/runners/runner.tests.toml"),
        RUNNER,
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
    let projected = eqm_manifest::project_graph_v2(session.finalized().graph())?;
    assert_eq!(
        projected["runners"][0]["args"],
        json!([
            {"kind": "target_root"},
            {"kind": "selector_json"},
            {"kind": "result_path"}
        ])
    );
    assert_eq!(
        projected["runners"][0]["cwd"],
        json!({"kind": "target_root"})
    );
    // The session digest is exactly the domain-separated digest of the v2 bytes.
    let canonical = eqm_manifest::canonicalize_graph_v2(session.finalized().graph())?;
    assert_eq!(session.workspace_digest(), canonical.digest().as_digest());
    Ok(())
}

#[test]
fn prepared_session_runner_argument_mutation_moves_the_workspace_digest()
-> Result<(), Box<dyn Error>> {
    let baseline = fixture()?;
    fs::write(baseline.path().join("eqm.toml"), v2_config("control_tower"))?;
    fs::create_dir_all(baseline.path().join("eqm/runners"))?;
    fs::write(
        baseline.path().join("eqm/runners/runner.tests.toml"),
        RUNNER,
    )?;
    let baseline = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        baseline.path(),
    )?;

    let changed = fixture()?;
    fs::write(changed.path().join("eqm.toml"), v2_config("control_tower"))?;
    fs::create_dir_all(changed.path().join("eqm/runners"))?;
    fs::write(
        changed.path().join("eqm/runners/runner.tests.toml"),
        RUNNER.replace("\"{target_root}\", ", "\"target_root\", "),
    )?;
    let changed = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        changed.path(),
    )?;

    // A literal `target_root` argument is distinct from the substitution
    // template, so one axis moves the v2 workspace identity.
    assert_ne!(baseline.workspace_digest(), changed.workspace_digest());
    Ok(())
}

fn control_tower_target() -> Result<Target, Box<dyn Error>> {
    let owner: OwnerRef = "owner://team/web".parse()?;
    Ok(Target::new(
        TargetId::new("web")?,
        RepoPath::new("apps/web")?,
        PlatformId::new("web")?,
        FrameworkId::new("sveltekit")?,
        vec![owner],
        Extensions::default(),
    )?
    .with_role(RoleId::new("control_tower")?)
    .with_source_components(vec![SourceComponentId::new("shared.kmp")?])?)
}

fn mailgun_provider() -> Result<Provider, Box<dyn Error>> {
    let owner: OwnerRef = "owner://team/providers".parse()?;
    Ok(Provider::new(
        ProviderId::new("provider.mailgun")?,
        "sha256:071ca2227754705837aa3ef9748ed59e9f8a015fd765c42f391a4cbc271c6d5e"
            .parse::<Sha256Digest>()?,
        vec![owner],
        Extensions::default(),
    )?)
}

/// Builds a cohort document whose authored configuration claims equal the
/// graph projections for the `v2_config` target/provider definitions.
fn cohort_document() -> Result<String, Box<dyn Error>> {
    let target = eqm_manifest::canonicalize_target_configuration_v2(&control_tower_target()?)?
        .digest()
        .as_digest();
    let provider = eqm_manifest::canonicalize_provider_configuration_v2(&mailgun_provider()?)?
        .digest()
        .as_digest();
    Ok(format!(
        r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/cohort.schema.json"
id = "cohort.signup"
revision = 1
scenario = "signup.flow"

[[assignments]]
occurrence = "mailer"
repository = "https://example.com/team/project"
source_commit = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
source_content_digest = "sha256:1111111111111111111111111111111111111111111111111111111111111111"
provider_configuration_digest = "{provider}"
build_id = "build.mailer"
artifact_digest = "sha256:2222222222222222222222222222222222222222222222222222222222222222"

[[assignments]]
occurrence = "phone"
repository = "https://example.com/team/project"
source_commit = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
source_content_digest = "sha256:3333333333333333333333333333333333333333333333333333333333333333"
target_configuration_digest = "{target}"
build_id = "build.phone"
artifact_digest = "sha256:4444444444444444444444444444444444444444444444444444444444444444"
"#
    ))
}

#[test]
fn v2_cohort_document_is_discovered_and_indexed() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(
        repository.path().join("eqm.toml"),
        v2_config("control_tower"),
    )?;
    fs::write(
        repository.path().join("eqm/contracts/scenario.signup.toml"),
        SCENARIO,
    )?;
    fs::write(
        repository
            .path()
            .join("eqm/contracts/provider.mailgun.toml"),
        PROVIDER,
    )?;
    fs::write(
        repository.path().join("eqm/contracts/cohort.signup.toml"),
        cohort_document()?,
    )?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    assert_eq!(session.finalized().graph().cohorts().len(), 1);
    let (_, cohort) = session
        .finalized()
        .graph()
        .cohorts()
        .iter()
        .next()
        .ok_or("cohort missing")?;
    assert_eq!(cohort.assignments().len(), 2);
    Ok(())
}

#[test]
fn v2_dangling_cohort_scenario_is_rejected_at_prepare() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(
        repository.path().join("eqm.toml"),
        v2_config("control_tower"),
    )?;
    let dangling =
        cohort_document()?.replace("scenario = \"signup.flow\"", "scenario = \"absent.flow\"");
    fs::write(
        repository.path().join("eqm/contracts/cohort.dangling.toml"),
        dangling,
    )?;
    match prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    ) {
        Err(SessionError::Cohort(eqm_application::CohortError::ScenarioNotFound)) => {}
        other => {
            return Err(
                format!("expected a dangling-cohort rejection at prepare, got {other:?}").into(),
            );
        }
    }
    Ok(())
}

#[test]
fn v2_cohort_configuration_mismatch_is_rejected_at_prepare() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(
        repository.path().join("eqm.toml"),
        v2_config("control_tower"),
    )?;
    fs::write(
        repository.path().join("eqm/contracts/scenario.signup.toml"),
        SCENARIO,
    )?;
    fs::write(
        repository
            .path()
            .join("eqm/contracts/provider.mailgun.toml"),
        PROVIDER,
    )?;
    // A valid-shaped but different provider claim does not equal the graph
    // projection, so preparation fails rather than substituting it.
    let other = Sha256Digest::hash_content(b"other");
    let mismatched = cohort_document()?
        .lines()
        .map(|line| {
            if line.starts_with("provider_configuration_digest") {
                format!("provider_configuration_digest = \"{other}\"")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        repository.path().join("eqm/contracts/cohort.signup.toml"),
        mismatched,
    )?;
    match prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    ) {
        Err(SessionError::Cohort(eqm_application::CohortError::ConfigurationMismatch(_))) => {}
        other => {
            return Err(format!(
                "expected a cohort configuration mismatch at prepare, got {other:?}"
            )
            .into());
        }
    }
    Ok(())
}
