//! P03-R1: complete v2 semantic-graph identity (`eqm:v2:semantic-graph`).
//!
//! These controls exercise the exact v2 graph projection and the version
//! routing boundary: a one-axis role/source-component/participation mutation
//! changes the v2 identity, canonical set ordering is permutation-invariant,
//! the v1 projection rejects a v2-only graph instead of omitting it, extension
//! integer/type distinctions survive, and the projection conforms to the
//! committed local v2 semantic-graph schema.

use eqm_domain::{
    Applicability, ArgumentTemplate, CalendarDate, Capability, CapabilityId, DimensionId,
    DurationMillis, EvidenceScopeSubject, ExtensionNamespace, ExtensionValue, Extensions, Facet,
    FinalizedWorkspaceGraph, FrameworkId, FullRequirementId, LifecycleStatus, OwnerRef,
    ParticipationRule, ParticipationRuleId, PlatformId, PolicyId, PositiveCount, ProfileId,
    ProviderId, RepoPath, Revision, RoleId, RunnerBackend, RunnerDefinition, RunnerId,
    RunnerLimits, RunnerProgram, SelectorText, SemanticGraphV2Digest, SourceComponent,
    SourceComponentId, SymbolicValueId, Target, TargetId, Title, UnitId, Waiver, WaiverId,
    WaiverProfileScope, WaiverReason, WaiverScope, WorkingDirectoryTemplate, WorkspaceGraph,
    WorkspaceGraphInput,
};
use eqm_manifest::{canonicalize_graph, canonicalize_graph_v2, project_graph_v2};
use jsonschema::{Draft, Validator};
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

const V2_GRAPH_BYTES: &str = r#"{"adapters":[],"bindings":[],"capabilities":[{"extensions":{},"id":"account.create","owners":["owner://team/accounts"],"status":"active","title":"Account creation"}],"extensions":{},"fragments":[],"imports":[],"journeys":[],"participation":[{"applicability":{"always":true},"extensions":{},"id":"participation.mobile","requirement":"account.create#create","targets":["web"]}],"policies":[],"profiles":[],"runners":[],"scenarios":[],"schema":"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/protocol/semantic-graph.schema.json","source_components":[{"dependencies":[],"extensions":{},"id":"shared.kmp","owners":["owner://team/shared"],"roots":["shared/kmp"]}],"surfaces":[],"targets":[{"extensions":{},"framework":"sveltekit","id":"web","owners":["owner://team/web"],"platform":"web","role":"control_tower","root":"apps/web","source_components":["shared.kmp"]}],"waivers":[]}"#;
const V2_GRAPH_DIGEST: &str =
    "sha256:2f86564cb2f6894b0221c6912ec15dd2e491698b3bf3e3a02ba1f191a5ffc4a6";

#[test]
fn literal_v2_graph_bytes_and_digest_match_independent_oracle() -> Result<(), Box<dyn Error>> {
    let canonical =
        canonicalize_graph_v2(&graph("control_tower", None, Extensions::default(), true)?)?;
    assert_eq!(std::str::from_utf8(canonical.bytes())?, V2_GRAPH_BYTES);
    assert_eq!(canonical.digest().to_string(), V2_GRAPH_DIGEST);
    assert!(!canonical.bytes().ends_with(b"\n"));
    Ok(())
}

fn semantic_graph_validator() -> Result<Validator, Box<dyn Error>> {
    let schema_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/v2/protocol/semantic-graph.schema.json");
    let schema: serde_json::Value = serde_json::from_slice(&std::fs::read(schema_path)?)?;
    Ok(jsonschema::options()
        .with_draft(Draft::Draft202012)
        .should_validate_formats(true)
        .build(&schema)?)
}

#[test]
fn v2_graph_projection_conforms_to_real_draft_2020_12_schema() -> Result<(), Box<dyn Error>> {
    let validator = semantic_graph_validator()?;
    let runner_literal = runner_graph(
        vec![ArgumentTemplate::Literal(SelectorText::new(
            "{target_root}",
        )?)],
        Some(WorkingDirectoryTemplate::Repository(RepoPath::new(
            "{target_root}",
        )?)),
    )?;
    let runner_template = runner_graph(
        vec![ArgumentTemplate::TargetRoot],
        Some(WorkingDirectoryTemplate::TargetRoot),
    )?;
    let target_waiver = waiver_graph(EvidenceScopeSubject::Target(TargetId::new("web")?))?;
    let provider_waiver = waiver_graph(EvidenceScopeSubject::Provider(ProviderId::new(
        "provider.mailgun",
    )?))?;
    let set_waiver = waiver_graph(EvidenceScopeSubject::target_set(vec![
        TargetId::new("web")?,
        TargetId::new("ios")?,
    ])?)?;
    for finalized in [
        graph("control_tower", None, Extensions::default(), true)?,
        runner_literal,
        runner_template,
        target_waiver,
        provider_waiver,
        set_waiver,
    ] {
        let projected = project_graph_v2(&finalized)?;
        assert!(
            validator.is_valid(&projected),
            "actual projection failed the committed schema: {projected}"
        );
    }
    Ok(())
}

#[test]
fn v2_graph_collections_reject_null_numeric_unknown_and_missing_members()
-> Result<(), Box<dyn Error>> {
    let validator = semantic_graph_validator()?;
    let base = project_graph_v2(&graph("control_tower", None, Extensions::default(), true)?)?;
    let collections = [
        "adapters",
        "bindings",
        "capabilities",
        "fragments",
        "imports",
        "journeys",
        "participation",
        "policies",
        "profiles",
        "runners",
        "scenarios",
        "source_components",
        "surfaces",
        "targets",
        "waivers",
    ];
    for name in collections {
        for bad in [
            serde_json::json!([null]),
            serde_json::json!([123]),
            serde_json::json!([{}]),
            serde_json::json!([{"unknown": true}]),
        ] {
            let mut mutated = base.clone();
            mutated[name] = bad.clone();
            assert!(!validator.is_valid(&mutated), "{name} accepted {bad}");
        }
    }
    for bad in [
        serde_json::json!({"dev.equivalencematrix.extra": null}),
        serde_json::json!({"dev.equivalencematrix.extra": 5}),
        serde_json::json!({"dev.equivalencematrix.extra": {"type": "bogus", "value": 1}}),
    ] {
        let mut mutated = base.clone();
        mutated["extensions"] = bad.clone();
        assert!(!validator.is_valid(&mutated), "extensions accepted {bad}");
    }
    let mut mutated = base.clone();
    mutated
        .as_object_mut()
        .ok_or("projection is not an object")?
        .remove("targets");
    assert!(
        !validator.is_valid(&mutated),
        "missing root member accepted"
    );
    Ok(())
}

#[test]
fn v2_graph_unions_reject_wrong_kind_marker_and_payload() -> Result<(), Box<dyn Error>> {
    let validator = semantic_graph_validator()?;
    let runner_base = project_graph_v2(&runner_graph(
        vec![ArgumentTemplate::Literal(SelectorText::new("x")?)],
        Some(WorkingDirectoryTemplate::TargetRoot),
    )?)?;
    let bad_args = [
        serde_json::json!([{"kind": "bogus"}]),
        serde_json::json!([{"kind": "literal"}]),
        serde_json::json!([{"kind": "target_root", "value": "x"}]),
        serde_json::json!([{"kind": "literal", "value": 42}]),
    ];
    for bad in bad_args {
        let mut mutated = runner_base.clone();
        mutated["runners"][0]["args"] = bad.clone();
        assert!(!validator.is_valid(&mutated), "args accepted {bad}");
    }
    let bad_cwd = [
        serde_json::json!({"kind": "bogus"}),
        serde_json::json!({"kind": "repository"}),
        serde_json::json!({"kind": "repository", "path": 5}),
        serde_json::json!({"kind": "target_root", "path": "x"}),
    ];
    for bad in bad_cwd {
        let mut mutated = runner_base.clone();
        mutated["runners"][0]["cwd"] = bad.clone();
        assert!(!validator.is_valid(&mutated), "cwd accepted {bad}");
    }
    for bad in [
        serde_json::json!({"kind": "bogus"}),
        serde_json::json!({"kind": "repository"}),
        serde_json::json!({"kind": "locked", "resolved": "x", "digest": "not-a-digest"}),
    ] {
        let mut mutated = runner_base.clone();
        mutated["runners"][0]["program"] = bad.clone();
        assert!(!validator.is_valid(&mutated), "program accepted {bad}");
    }
    for bad in [
        serde_json::json!({"type": "bogus", "value": 1}),
        serde_json::json!({"type": "integer"}),
        serde_json::json!({"type": "integer", "value": {}}),
        serde_json::json!({"type": "integer", "value": "1"}),
        serde_json::json!({"type": "boolean", "value": 1}),
    ] {
        let mut mutated = runner_base.clone();
        mutated["extensions"]["dev.equivalencematrix.extra"] = bad.clone();
        assert!(!validator.is_valid(&mutated), "extension accepted {bad}");
    }

    let waiver_base = project_graph_v2(&waiver_graph(EvidenceScopeSubject::Target(
        TargetId::new("web")?,
    ))?)?;
    for bad in [
        serde_json::json!({"kind": "bogus"}),
        serde_json::json!({"kind": "target"}),
        serde_json::json!({"kind": "target", "provider": "x"}),
        serde_json::json!({"kind": "target_set", "targets": null}),
    ] {
        let mut mutated = waiver_base.clone();
        mutated["waivers"][0]["scope"]["target"] = bad.clone();
        assert!(
            !validator.is_valid(&mutated),
            "waiver subject accepted {bad}"
        );
    }

    let participation_base =
        project_graph_v2(&graph("control_tower", None, Extensions::default(), true)?)?;
    for bad in [
        serde_json::json!({"bogus": true}),
        serde_json::json!({"always": "yes"}),
        serde_json::json!({"all": [{"bogus": true}]}),
        serde_json::json!({"not": {"bogus": true}}),
        serde_json::json!({"dimension": "locale", "operator": "equals"}),
    ] {
        let mut mutated = participation_base.clone();
        mutated["participation"][0]["applicability"] = bad.clone();
        assert!(
            !validator.is_valid(&mutated),
            "applicability accepted {bad}"
        );
    }
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

#[test]
fn scenario_collection_participates_in_v2_identity_and_v1_rejects() -> Result<(), Box<dyn Error>> {
    use eqm_domain::{
        OccurrenceId, ProviderId, Revision, Scenario, ScenarioDependency, ScenarioId,
        ScenarioParticipant, ScenarioSubject,
    };
    let scenario = Scenario::new(
        ScenarioId::new("signup.flow")?,
        Revision::new(1)?,
        vec![
            ScenarioParticipant::new(
                OccurrenceId::new("phone_one")?,
                RoleId::new("edge")?,
                ScenarioSubject::Target(TargetId::new("web")?),
            ),
            ScenarioParticipant::new(
                OccurrenceId::new("phone_two")?,
                RoleId::new("edge")?,
                ScenarioSubject::Target(TargetId::new("web")?),
            ),
            ScenarioParticipant::new(
                OccurrenceId::new("mailer")?,
                RoleId::new("provider")?,
                ScenarioSubject::Provider(ProviderId::new("provider.mailgun")?),
            ),
        ],
        vec![ScenarioDependency::new(
            OccurrenceId::new("phone_two")?,
            OccurrenceId::new("phone_one")?,
        )?],
        Extensions::default(),
    )?;
    let plain = FinalizedWorkspaceGraph::from_engine(WorkspaceGraph::new(WorkspaceGraphInput {
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
    let with_scenario =
        FinalizedWorkspaceGraph::from_engine(WorkspaceGraph::new(WorkspaceGraphInput {
            targets: vec![Target::new(
                TargetId::new("web")?,
                RepoPath::new("apps/web")?,
                PlatformId::new("web")?,
                FrameworkId::new("sveltekit")?,
                vec![owner("owner://team/web")?],
                Extensions::default(),
            )?],
            scenarios: vec![scenario],
            ..WorkspaceGraphInput::default()
        })?);
    assert_ne!(
        canonicalize_graph_v2(&plain)?.digest(),
        canonicalize_graph_v2(&with_scenario)?.digest()
    );
    let projected = project_graph_v2(&with_scenario)?;
    assert!(
        projected["scenarios"]
            .as_array()
            .is_some_and(|v| v.len() == 1)
    );
    assert_eq!(
        canonicalize_graph(&with_scenario).map(|_| ()),
        Err(eqm_manifest::CanonicalizationError::V2OnlyMetadata)
    );
    Ok(())
}

fn runner(
    args: Vec<ArgumentTemplate>,
    cwd: Option<WorkingDirectoryTemplate>,
) -> Result<RunnerDefinition, Box<dyn Error>> {
    Ok(RunnerDefinition::new(
        RunnerId::new("runner.tests")?,
        Revision::new(1)?,
        vec![owner("owner://team/platform")?],
        RunnerBackend::Local,
        RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
        args,
        cwd,
        Vec::new(),
        Vec::new(),
        RunnerLimits::new(
            DurationMillis::new(30_000)?,
            PositiveCount::new(1_024)?,
            None,
        )?,
        Vec::new(),
        Extensions::default(),
    )?)
}

fn runner_graph(
    args: Vec<ArgumentTemplate>,
    cwd: Option<WorkingDirectoryTemplate>,
) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    Ok(FinalizedWorkspaceGraph::from_engine(WorkspaceGraph::new(
        WorkspaceGraphInput {
            runners: vec![runner(args, cwd)?],
            ..WorkspaceGraphInput::default()
        },
    )?))
}

/// The literal `{target_root}` argument text and repository-path placeholder
/// lookalikes are independent graph identity members.
const RUNNER_LITERAL_BYTES: &str = r#"{"adapters":[],"bindings":[],"capabilities":[],"extensions":{},"fragments":[],"imports":[],"journeys":[],"participation":[],"policies":[],"profiles":[],"runners":[{"args":[{"kind":"literal","value":"{target_root}"}],"backend":"local","cwd":{"kind":"repository","path":"{target_root}"},"environment":[],"extensions":{},"guarantees":[],"id":"runner.tests","max_concurrency":1,"max_output_bytes":1024,"owners":["owner://team/platform"],"program":{"kind":"repository","path":"tools/test-runner"},"revision":1,"secrets":[],"timeout_ms":30000}],"scenarios":[],"schema":"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/protocol/semantic-graph.schema.json","source_components":[],"surfaces":[],"targets":[],"waivers":[]}"#;
const RUNNER_LITERAL_DIGEST: &str =
    "sha256:9338d3e2e0240c640e628f5551480d3dd1425a80af3d309533f1bbb7bcbe82af";
const RUNNER_TEMPLATE_BYTES: &str = r#"{"adapters":[],"bindings":[],"capabilities":[],"extensions":{},"fragments":[],"imports":[],"journeys":[],"participation":[],"policies":[],"profiles":[],"runners":[{"args":[{"kind":"target_root"}],"backend":"local","cwd":{"kind":"target_root"},"environment":[],"extensions":{},"guarantees":[],"id":"runner.tests","max_concurrency":1,"max_output_bytes":1024,"owners":["owner://team/platform"],"program":{"kind":"repository","path":"tools/test-runner"},"revision":1,"secrets":[],"timeout_ms":30000}],"scenarios":[],"schema":"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/protocol/semantic-graph.schema.json","source_components":[],"surfaces":[],"targets":[],"waivers":[]}"#;
const RUNNER_TEMPLATE_DIGEST: &str =
    "sha256:fe3ad9d1da58566bb886a04070d7392e71ed254957981369927bb3852bbbfdbf";

#[test]
fn literal_placeholder_text_and_expanded_runner_arguments_stay_distinct()
-> Result<(), Box<dyn Error>> {
    let literal = canonicalize_graph_v2(&runner_graph(
        vec![ArgumentTemplate::Literal(SelectorText::new(
            "{target_root}",
        )?)],
        Some(WorkingDirectoryTemplate::Repository(RepoPath::new(
            "{target_root}",
        )?)),
    )?)?;
    let template = canonicalize_graph_v2(&runner_graph(
        vec![ArgumentTemplate::TargetRoot],
        Some(WorkingDirectoryTemplate::TargetRoot),
    )?)?;
    assert_eq!(std::str::from_utf8(literal.bytes())?, RUNNER_LITERAL_BYTES);
    assert_eq!(literal.digest().to_string(), RUNNER_LITERAL_DIGEST);
    assert_eq!(
        std::str::from_utf8(template.bytes())?,
        RUNNER_TEMPLATE_BYTES
    );
    assert_eq!(template.digest().to_string(), RUNNER_TEMPLATE_DIGEST);
    assert_ne!(literal.bytes(), template.bytes());
    assert_ne!(literal.digest(), template.digest());
    Ok(())
}

#[test]
fn runner_argument_order_and_cwd_variants_are_bound() -> Result<(), Box<dyn Error>> {
    let forward = canonicalize_graph_v2(&runner_graph(
        vec![ArgumentTemplate::TargetRoot, ArgumentTemplate::SelectorJson],
        None,
    )?)?;
    let reversed = canonicalize_graph_v2(&runner_graph(
        vec![ArgumentTemplate::SelectorJson, ArgumentTemplate::TargetRoot],
        None,
    )?)?;
    assert_ne!(forward.bytes(), reversed.bytes());
    assert_ne!(forward.digest(), reversed.digest());

    // A repository path that merely looks like a placeholder is distinct from an
    // actual result-path expansion.
    let lookalike = canonicalize_graph_v2(&runner_graph(
        vec![ArgumentTemplate::SelectorJson],
        Some(WorkingDirectoryTemplate::Repository(RepoPath::new(
            "{result_path}",
        )?)),
    )?)?;
    let expanded = canonicalize_graph_v2(&runner_graph(
        vec![ArgumentTemplate::SelectorJson],
        Some(WorkingDirectoryTemplate::ResultPath),
    )?)?;
    assert_ne!(lookalike.bytes(), expanded.bytes());
    assert_ne!(lookalike.digest(), expanded.digest());
    Ok(())
}

fn waiver(subject: EvidenceScopeSubject) -> Result<Waiver, Box<dyn Error>> {
    Ok(Waiver::new(
        WaiverId::new("waiver.signup")?,
        Revision::new(1)?,
        vec![owner("owner://team/product")?],
        PolicyId::new("release.default")?,
        WaiverScope::new(
            subject,
            UnitId::new("account.create.signup.start")?,
            FullRequirementId::new("account.create.signup.start#reachable")?,
            vec![Facet::Behavior],
            vec![WaiverProfileScope::new(
                ProfileId::new("audience.default")?,
                vec![(DimensionId::new("region")?, SymbolicValueId::new("eu")?)],
            )?],
        )?,
        WaiverReason::new("Temporary remediation")?,
        "issue://PRODUCT-42".parse()?,
        vec![owner("owner://role/reviewer")?],
        "2026-08-01".parse::<CalendarDate>()?,
        "2026-08-08".parse::<CalendarDate>()?,
        vec![Facet::Accessibility],
        Extensions::default(),
    )?)
}

fn waiver_graph(subject: EvidenceScopeSubject) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    Ok(FinalizedWorkspaceGraph::from_engine(WorkspaceGraph::new(
        WorkspaceGraphInput {
            waivers: vec![waiver(subject)?],
            ..WorkspaceGraphInput::default()
        },
    )?))
}

#[test]
fn waiver_subject_kind_survives_v2_graph_projection() -> Result<(), Box<dyn Error>> {
    let target = canonicalize_graph_v2(&waiver_graph(EvidenceScopeSubject::Target(
        TargetId::new("web")?,
    ))?)?;
    let provider = canonicalize_graph_v2(&waiver_graph(EvidenceScopeSubject::Provider(
        ProviderId::new("provider.mailgun")?,
    ))?)?;
    let set = canonicalize_graph_v2(&waiver_graph(EvidenceScopeSubject::target_set(vec![
        TargetId::new("web")?,
        TargetId::new("ios")?,
    ])?)?)?;
    assert_ne!(target.digest(), provider.digest());
    assert_ne!(target.digest(), set.digest());
    assert_ne!(provider.digest(), set.digest());

    let target_projection = project_graph_v2(&waiver_graph(EvidenceScopeSubject::Target(
        TargetId::new("web")?,
    ))?)?;
    assert_eq!(
        target_projection["waivers"][0]["scope"]["target"],
        serde_json::json!({"kind": "target", "target": "web"})
    );
    let provider_projection = project_graph_v2(&waiver_graph(EvidenceScopeSubject::Provider(
        ProviderId::new("provider.mailgun")?,
    ))?)?;
    assert_eq!(
        provider_projection["waivers"][0]["scope"]["target"],
        serde_json::json!({"kind": "provider", "provider": "provider.mailgun"})
    );
    let set_projection =
        project_graph_v2(&waiver_graph(EvidenceScopeSubject::target_set(vec![
            TargetId::new("web")?,
            TargetId::new("ios")?,
        ])?)?)?;
    assert_eq!(
        set_projection["waivers"][0]["scope"]["target"],
        serde_json::json!({"kind": "target_set", "targets": ["ios", "web"]})
    );
    Ok(())
}
