//! P03-R1: complete v2 semantic-graph identity (`eqm:v2:semantic-graph`).
//!
//! These controls exercise the exact v2 graph projection and the version
//! routing boundary: a one-axis role/source-component/participation mutation
//! changes the v2 identity, canonical set ordering is permutation-invariant,
//! the v1 projection rejects a v2-only graph instead of omitting it, extension
//! integer/type distinctions survive, and the projection conforms to the
//! committed local v2 semantic-graph schema.

use eqm_domain::{
    Applicability, Capability, CapabilityId, ExtensionNamespace, ExtensionValue, Extensions,
    FinalizedWorkspaceGraph, FrameworkId, FullRequirementId, LifecycleStatus, OwnerRef,
    ParticipationRule, ParticipationRuleId, PlatformId, RepoPath, RoleId, SemanticGraphV2Digest,
    SourceComponent, SourceComponentId, Target, TargetId, Title, WorkspaceGraph,
    WorkspaceGraphInput,
};
use eqm_manifest::{
    canonicalize_graph, canonicalize_graph_v2, project_graph_v2, semantic_graph_v2_schema_uri,
};
use std::collections::BTreeMap;
use std::error::Error;

fn owner(value: &str) -> Result<OwnerRef, Box<dyn Error>> {
    Ok(value.parse()?)
}

fn capability() -> Result<Capability, Box<dyn Error>> {
    Ok(Capability::new(
        CapabilityId::new("account.create")?,
        Title::new("Account creation")?,
        LifecycleStatus::Active,
        vec![owner("owner://team/accounts")?],
        None,
        Extensions::default(),
    )?)
}

fn web_target(role: &str) -> Result<Target, Box<dyn Error>> {
    Ok(Target::new(
        TargetId::new("web")?,
        RepoPath::new("apps/web")?,
        PlatformId::new("web")?,
        FrameworkId::new("sveltekit")?,
        vec![owner("owner://team/web")?],
        Extensions::default(),
    )?
    .with_role(RoleId::new(role)?)
    .with_source_components(vec![SourceComponentId::new("shared.kmp")?])?)
}

fn source_component(dependency: Option<&str>) -> Result<SourceComponent, Box<dyn Error>> {
    Ok(SourceComponent::new(
        SourceComponentId::new("shared.kmp")?,
        vec![RepoPath::new("shared/kmp")?],
        vec![owner("owner://team/shared")?],
        dependency
            .map(SourceComponentId::new)
            .transpose()?
            .into_iter()
            .collect(),
        Extensions::default(),
    )?)
}

fn participation() -> Result<ParticipationRule, Box<dyn Error>> {
    Ok(ParticipationRule::new(
        ParticipationRuleId::new("participation.mobile")?,
        FullRequirementId::new("account.create#create")?,
        vec![TargetId::new("web")?],
        Applicability::always(true),
        Extensions::default(),
    )?)
}

fn graph(
    role: &str,
    dependency: Option<&str>,
    component_extensions: Extensions,
    include_participation: bool,
) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    let mut component = source_component(dependency)?;
    if !component_extensions.values().is_empty() {
        component = SourceComponent::new(
            SourceComponentId::new("shared.kmp")?,
            vec![RepoPath::new("shared/kmp")?],
            vec![owner("owner://team/shared")?],
            dependency
                .map(SourceComponentId::new)
                .transpose()?
                .into_iter()
                .collect(),
            component_extensions,
        )?;
    }
    Ok(FinalizedWorkspaceGraph::from_engine(WorkspaceGraph::new(
        WorkspaceGraphInput {
            capabilities: vec![capability()?],
            targets: vec![web_target(role)?],
            source_components: vec![component],
            participation: if include_participation {
                vec![participation()?]
            } else {
                Vec::new()
            },
            ..WorkspaceGraphInput::default()
        },
    )?))
}

fn extra_extensions(value: ExtensionValue) -> Result<Extensions, Box<dyn Error>> {
    Ok(Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.extra")?,
        value,
    )]))?)
}

const V2_GRAPH_BYTES: &str = r#"{"adapters":[],"bindings":[],"capabilities":[{"extensions":{},"id":"account.create","owners":["owner://team/accounts"],"status":"active","title":"Account creation"}],"extensions":{},"fragments":[],"imports":[],"journeys":[],"participation":[{"applicability":{"always":true},"extensions":{},"id":"participation.mobile","requirement":"account.create#create","targets":["web"]}],"policies":[],"profiles":[],"runners":[],"schema":"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/protocol/semantic-graph.schema.json","source_components":[{"dependencies":[],"extensions":{},"id":"shared.kmp","owners":["owner://team/shared"],"roots":["shared/kmp"]}],"surfaces":[],"targets":[{"extensions":{},"framework":"sveltekit","id":"web","owners":["owner://team/web"],"platform":"web","role":"control_tower","root":"apps/web","source_components":["shared.kmp"]}],"waivers":[]}"#;
const V2_GRAPH_DIGEST: &str =
    "sha256:2a7eea14d8a5a532296905aa51110be89a7536896810cb33d3cd86b525ffb6df";

#[test]
fn literal_v2_graph_bytes_and_digest_match_independent_oracle() -> Result<(), Box<dyn Error>> {
    let canonical =
        canonicalize_graph_v2(&graph("control_tower", None, Extensions::default(), true)?)?;
    assert_eq!(std::str::from_utf8(canonical.bytes())?, V2_GRAPH_BYTES);
    assert_eq!(canonical.digest().to_string(), V2_GRAPH_DIGEST);
    assert!(!canonical.bytes().ends_with(b"\n"));
    Ok(())
}

#[test]
fn v2_graph_projection_conforms_to_the_committed_local_schema() -> Result<(), Box<dyn Error>> {
    let projected = project_graph_v2(&graph("control_tower", None, Extensions::default(), true)?)?;
    let schema_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/v2/protocol/semantic-graph.schema.json");
    let schema: serde_json::Value = serde_json::from_slice(&std::fs::read(schema_path)?)?;
    let object = projected
        .as_object()
        .ok_or("projected graph is not an object")?;
    let required = schema["required"]
        .as_array()
        .ok_or("schema required")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<Vec<_>>();
    let mut keys = object.keys().map(String::as_str).collect::<Vec<_>>();
    keys.sort_unstable();
    let mut required_sorted = required.clone();
    required_sorted.sort_unstable();
    assert_eq!(keys, required_sorted, "projection root membership");
    assert_eq!(
        schema["additionalProperties"],
        serde_json::Value::Bool(false)
    );
    for name in required {
        let property = &schema["properties"][name];
        if let Some(constant) = property.get("const").and_then(serde_json::Value::as_str) {
            assert_eq!(object[name], serde_json::json!(constant), "{name} const");
            continue;
        }
        let kind = property["type"].as_str().ok_or("property type")?;
        match kind {
            "array" => assert!(object[name].is_array(), "{name} must be an array"),
            "object" => assert!(object[name].is_object(), "{name} must be an object"),
            "string" => assert!(object[name].is_string(), "{name} must be a string"),
            other => return Err(format!("unexpected schema type {other}").into()),
        }
    }
    assert_eq!(
        object["schema"],
        serde_json::json!(semantic_graph_v2_schema_uri())
    );
    Ok(())
}

#[test]
fn one_axis_role_component_dependency_and_participation_mutations_move_v2_identity()
-> Result<(), Box<dyn Error>> {
    let base = canonicalize_graph_v2(&graph("control_tower", None, Extensions::default(), true)?)?;
    assert_eq!(
        base.digest(),
        SemanticGraphV2Digest::hash_content(base.bytes())
    );
    assert_eq!(
        base.digest(),
        SemanticGraphV2Digest::hash_content(&{
            let value: serde_json::Value = serde_json::from_slice(base.bytes())?;
            serde_json_canonicalizer::to_vec(&value)?
        })
    );

    // One-axis role mutation.
    let role = canonicalize_graph_v2(&graph("operator", None, Extensions::default(), true)?)?;
    assert_ne!(base.digest(), role.digest());

    // One-axis dependency mutation on an otherwise identical component.
    let dependency = canonicalize_graph_v2(&graph(
        "control_tower",
        Some("shared.auth"),
        Extensions::default(),
        true,
    )?)?;
    assert_ne!(base.digest(), dependency.digest());

    // One-axis component extension mutation.
    let extension = canonicalize_graph_v2(&graph(
        "control_tower",
        None,
        extra_extensions(ExtensionValue::String("x".into()))?,
        true,
    )?)?;
    assert_ne!(base.digest(), extension.digest());

    // One-axis participation-presence mutation.
    let without =
        canonicalize_graph_v2(&graph("control_tower", None, Extensions::default(), false)?)?;
    assert_ne!(base.digest(), without.digest());
    Ok(())
}

#[test]
fn v1_graph_projection_rejects_v2_only_metadata() -> Result<(), Box<dyn Error>> {
    let v2_graph = graph("control_tower", None, Extensions::default(), true)?;
    assert_eq!(
        canonicalize_graph(&v2_graph).map(|_| ()),
        Err(eqm_manifest::CanonicalizationError::V2OnlyMetadata)
    );
    // A v2 graph with participation removed still carries v2-only target/role
    // and source-component authority.
    assert_eq!(
        canonicalize_graph(&graph("control_tower", None, Extensions::default(), false)?)
            .map(|_| ()),
        Err(eqm_manifest::CanonicalizationError::V2OnlyMetadata)
    );
    // A genuinely v1 graph still projects under the frozen v1 domain.
    let v1 = FinalizedWorkspaceGraph::from_engine(WorkspaceGraph::new(WorkspaceGraphInput {
        capabilities: vec![capability()?],
        targets: vec![Target::new(
            TargetId::new("web")?,
            RepoPath::new("apps/web")?,
            PlatformId::new("web")?,
            FrameworkId::new("sveltekit")?,
            vec![owner("owner://team/web")?],
            Extensions::default(),
        )?],
        ..WorkspaceGraphInput::default()
    })?);
    let canonical = canonicalize_graph(&v1)?;
    assert_eq!(
        canonical.digest(),
        eqm_domain::Sha256Digest::hash_domain(
            eqm_domain::DigestDomain::SemanticGraph,
            canonical.bytes()
        )
    );
    Ok(())
}

#[test]
fn component_set_order_is_permutation_invariant_but_content_sensitive() -> Result<(), Box<dyn Error>>
{
    let forward = FinalizedWorkspaceGraph::from_engine(WorkspaceGraph::new(WorkspaceGraphInput {
        targets: vec![web_target("control_tower")?],
        source_components: vec![
            SourceComponent::new(
                SourceComponentId::new("shared.auth")?,
                vec![RepoPath::new("shared/auth")?],
                vec![owner("owner://team/shared")?],
                vec![],
                Extensions::default(),
            )?,
            source_component(None)?,
        ],
        ..WorkspaceGraphInput::default()
    })?);
    let reversed =
        FinalizedWorkspaceGraph::from_engine(WorkspaceGraph::new(WorkspaceGraphInput {
            targets: vec![web_target("control_tower")?],
            source_components: vec![
                source_component(None)?,
                SourceComponent::new(
                    SourceComponentId::new("shared.auth")?,
                    vec![RepoPath::new("shared/auth")?],
                    vec![owner("owner://team/shared")?],
                    vec![],
                    Extensions::default(),
                )?,
            ],
            ..WorkspaceGraphInput::default()
        })?);
    assert_eq!(
        canonicalize_graph_v2(&forward)?,
        canonicalize_graph_v2(&reversed)?
    );
    Ok(())
}

#[test]
fn v1_and_v2_graph_identities_stay_domain_separated() -> Result<(), Box<dyn Error>> {
    // Same semantic content is not projected as v1 while being reported as v2.
    let v2 = canonicalize_graph_v2(&graph("control_tower", None, Extensions::default(), true)?)?;
    let v1 = canonicalize_graph(&FinalizedWorkspaceGraph::from_engine(WorkspaceGraph::new(
        WorkspaceGraphInput {
            capabilities: vec![capability()?],
            targets: vec![Target::new(
                TargetId::new("web")?,
                RepoPath::new("apps/web")?,
                PlatformId::new("web")?,
                FrameworkId::new("sveltekit")?,
                vec![owner("owner://team/web")?],
                Extensions::default(),
            )?],
            ..WorkspaceGraphInput::default()
        },
    )?))?;
    assert_ne!(v1.digest().as_bytes(), v2.digest().as_digest().as_bytes());
    Ok(())
}

#[test]
fn display_only_extension_does_not_change_v2_graph_identity() -> Result<(), Box<dyn Error>> {
    let base = canonicalize_graph_v2(&graph("control_tower", None, Extensions::default(), true)?)?;
    let display = Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.display")?,
        ExtensionValue::String("hidden".into()),
    )]))?;
    let decorated = canonicalize_graph_v2(&graph("control_tower", None, display, true)?)?;
    assert_eq!(base, decorated);
    Ok(())
}

#[test]
fn large_component_extension_integer_and_text_never_collide() -> Result<(), Box<dyn Error>> {
    let integer = canonicalize_graph_v2(&graph(
        "control_tower",
        None,
        extra_extensions(ExtensionValue::Integer(9_007_199_254_740_992))?,
        true,
    )?)?;
    let text = canonicalize_graph_v2(&graph(
        "control_tower",
        None,
        extra_extensions(ExtensionValue::String("9007199254740992".into()))?,
        true,
    )?)?;
    assert_ne!(integer.bytes(), text.bytes());
    assert_ne!(integer.digest(), text.digest());
    Ok(())
}
