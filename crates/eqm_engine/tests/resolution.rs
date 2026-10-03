//! End-to-end graph-resolution fixtures.

mod support;

use eqm_domain::validate_diagnostic_registry;
use eqm_engine::{ResolutionError, resolution_diagnostics, resolve_graph};
use std::error::Error;

#[test]
fn valid_graph_resolves_deterministically() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let first = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let second = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    assert_eq!(first, second);
    assert_eq!(first.capabilities().len(), 1);
    assert_eq!(first.bindings().len(), 2);
    Ok(())
}

#[test]
fn duplicates_and_dangling_references_are_stable_and_source_linked() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let mut duplicate = loaded.graph_input().clone();
    duplicate
        .capabilities
        .push(duplicate.capabilities[0].clone());
    let error = resolve_graph(duplicate, loaded.source_map())
        .err()
        .ok_or("duplicate accepted")?;
    assert!(matches!(error, ResolutionError::Findings(_)));
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0300");
    assert!(error.diagnostics()[0].source().is_some());

    let mut dangling = loaded.graph_input().clone();
    dangling.capabilities.clear();
    let first = resolve_graph(dangling.clone(), loaded.source_map())
        .err()
        .ok_or("dangling reference accepted")?;
    let second = resolve_graph(dangling, loaded.source_map())
        .err()
        .ok_or("dangling reference accepted")?;
    assert_eq!(first.diagnostics(), second.diagnostics());
    assert_eq!(first.diagnostics()[0].code().to_string(), "EQM-E0301");
    assert!(first.diagnostics()[0].source().is_some());
    Ok(())
}

#[test]
fn resolution_diagnostic_registry_is_complete_and_sorted() -> Result<(), Box<dyn Error>> {
    let registry = resolution_diagnostics()?;
    validate_diagnostic_registry(&registry)?;
    assert_eq!(registry[0].code.to_string(), "EQM-E0300");
    assert_eq!(registry[1].code.to_string(), "EQM-E0301");
    assert_eq!(registry[2].code.to_string(), "EQM-E0302");
    assert_eq!(registry[3].code.to_string(), "EQM-E0303");
    assert_eq!(registry[4].code.to_string(), "EQM-E0304");
    assert_eq!(registry[5].code.to_string(), "EQM-E0305");
    Ok(())
}

use eqm_domain::{
    Applicability, Extensions, FrameworkId, FullRequirementId, OwnerRef, ParticipationRule,
    ParticipationRuleId, PlatformId, Provider, ProviderId, RepoPath, Sha256Digest, SourceComponent,
    SourceComponentId, Target, TargetId, WorkspaceGraphInput,
};
use std::collections::BTreeMap;
use std::str::FromStr;

fn owner() -> Result<OwnerRef, Box<dyn Error>> {
    Ok(OwnerRef::from_str("owner://team/shared")?)
}

fn component(
    id: &str,
    roots: &[&str],
    dependencies: &[&str],
) -> Result<SourceComponent, Box<dyn Error>> {
    Ok(SourceComponent::new(
        SourceComponentId::new(id)?,
        roots
            .iter()
            .map(|root| RepoPath::new(*root))
            .collect::<Result<Vec<_>, _>>()?,
        vec![owner()?],
        dependencies
            .iter()
            .map(|dependency| SourceComponentId::new(*dependency))
            .collect::<Result<Vec<_>, _>>()?,
        Extensions::default(),
    )?)
}

fn target(id: &str, components: &[&str], depends_on: &[&str]) -> Result<Target, Box<dyn Error>> {
    let target = Target::new(
        TargetId::new(id)?,
        RepoPath::new("apps/target")?,
        PlatformId::new("kmp")?,
        FrameworkId::new("compose")?,
        vec![owner()?],
        Extensions::default(),
    )?;
    let target = target.with_source_components(
        components
            .iter()
            .map(|component| SourceComponentId::new(*component))
            .collect::<Result<Vec<_>, _>>()?,
    )?;
    Ok(target.with_depends_on(
        depends_on
            .iter()
            .map(|dependency| TargetId::new(*dependency))
            .collect::<Result<Vec<_>, _>>()?,
    )?)
}

fn input(components: Vec<SourceComponent>, targets: Vec<Target>) -> WorkspaceGraphInput {
    WorkspaceGraphInput {
        source_components: components,
        targets,
        ..WorkspaceGraphInput::default()
    }
}

#[test]
fn shared_source_dependency_dag_resolves() -> Result<(), Box<dyn Error>> {
    let graph = resolve_graph(
        input(
            vec![
                component("shared.base", &["shared/base"], &[])?,
                component("shared.kmp", &["shared/kmp"], &["shared.base"])?,
            ],
            vec![
                target("android", &["shared.kmp"], &[])?,
                target("ios", &["shared.kmp"], &["android"])?,
            ],
        ),
        &BTreeMap::new(),
    )?;
    assert_eq!(graph.source_components().len(), 2);
    assert_eq!(
        graph
            .targets()
            .get(&TargetId::new("android")?)
            .and_then(|target| target.source_components().iter().next())
            .map(SourceComponentId::as_str),
        Some("shared.kmp")
    );
    Ok(())
}

#[test]
fn missing_source_component_is_stable() -> Result<(), Box<dyn Error>> {
    let first = resolve_graph(
        input(vec![], vec![target("android", &["shared.absent"], &[])?]),
        &BTreeMap::new(),
    )
    .err()
    .ok_or("missing component accepted")?;
    assert_eq!(first.diagnostics()[0].code().to_string(), "EQM-E0301");
    let second = resolve_graph(
        input(vec![], vec![target("android", &["shared.absent"], &[])?]),
        &BTreeMap::new(),
    )
    .err()
    .ok_or("missing component accepted")?;
    assert_eq!(first.diagnostics(), second.diagnostics());
    Ok(())
}

#[test]
fn missing_component_dependency_is_stable() -> Result<(), Box<dyn Error>> {
    let error = resolve_graph(
        input(
            vec![component(
                "shared.kmp",
                &["shared/kmp"],
                &["shared.absent"],
            )?],
            vec![],
        ),
        &BTreeMap::new(),
    )
    .err()
    .ok_or("missing dependency accepted")?;
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0301");
    Ok(())
}

#[test]
fn component_dependency_cycle_reports_a_stable_diagnostic() -> Result<(), Box<dyn Error>> {
    let error = resolve_graph(
        input(
            vec![
                component("shared.a", &["shared/a"], &["shared.b"])?,
                component("shared.b", &["shared/b"], &["shared.a"])?,
            ],
            vec![],
        ),
        &BTreeMap::new(),
    )
    .err()
    .ok_or("component cycle accepted")?;
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0302");
    let first = error.diagnostics().to_vec();
    let second = resolve_graph(
        input(
            vec![
                component("shared.a", &["shared/a"], &["shared.b"])?,
                component("shared.b", &["shared/b"], &["shared.a"])?,
            ],
            vec![],
        ),
        &BTreeMap::new(),
    )
    .err()
    .ok_or("component cycle accepted")?
    .diagnostics()
    .to_vec();
    assert_eq!(first, second);
    Ok(())
}

#[test]
fn target_dependency_cycle_reports_a_stable_diagnostic() -> Result<(), Box<dyn Error>> {
    let error = resolve_graph(
        input(
            vec![],
            vec![
                target("android", &[], &["ios"])?,
                target("ios", &[], &["android"])?,
            ],
        ),
        &BTreeMap::new(),
    )
    .err()
    .ok_or("target cycle accepted")?;
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0302");
    Ok(())
}

#[test]
fn duplicate_source_component_authority_rejects() -> Result<(), Box<dyn Error>> {
    let error = resolve_graph(
        input(
            vec![
                component("shared.kmp", &["shared/a"], &[])?,
                component("shared.kmp", &["shared/b"], &[])?,
            ],
            vec![],
        ),
        &BTreeMap::new(),
    )
    .err()
    .ok_or("duplicate component accepted")?;
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0300");
    Ok(())
}

#[test]
fn participation_references_resolve_and_missing_references_reject() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let mut input = loaded.graph_input().clone();
    let target = input.targets.first().ok_or("target missing")?.id().clone();
    let requirement = input
        .surfaces
        .iter()
        .flat_map(|surface| {
            surface
                .requirements()
                .keys()
                .map(move |local| format!("{}#{local}", surface.id()))
        })
        .next()
        .ok_or("requirement missing")?;
    let rule = ParticipationRule::new(
        ParticipationRuleId::new("participation.mobile")?,
        FullRequirementId::new(requirement)?,
        vec![target.clone()],
        Applicability::always(true),
        Extensions::default(),
    )?;
    input.participation.push(rule);
    let graph = resolve_graph(input.clone(), loaded.source_map())?;
    assert_eq!(graph.participation().len(), 1);

    // A participation rule that names an absent requirement rejects as dangling.
    input.participation[0] = ParticipationRule::new(
        ParticipationRuleId::new("participation.mobile")?,
        FullRequirementId::new("absent.unit#rule")?,
        vec![target],
        Applicability::always(true),
        Extensions::default(),
    )?;
    let error = resolve_graph(input, loaded.source_map())
        .err()
        .ok_or("missing requirement accepted")?;
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0301");
    Ok(())
}

#[test]
fn declared_provider_references_resolve_and_missing_provider_is_rejected()
-> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let base = loaded.graph_input().clone();
    let provider_id = ProviderId::new("provider.mailgun")?;
    let provider = Provider::new(
        provider_id.clone(),
        Sha256Digest::hash_content(b"provider configuration"),
        vec!["owner://team/providers".parse()?],
        Extensions::default(),
    )?;

    // A target may reference a declared provider; an unrelated declared provider
    // is not attached to any target automatically.
    let mut declared = base.clone();
    declared.providers.push(provider);
    for target in &mut declared.targets {
        *target = target.clone().with_providers(vec![provider_id.clone()])?;
    }
    let graph = resolve_graph(declared, loaded.source_map())?;
    assert_eq!(graph.providers().len(), 1);
    assert!(
        graph
            .targets()
            .values()
            .all(|target| { target.providers().contains(&provider_id) })
    );

    // A target referencing an undeclared provider fails preparation.
    let mut missing = base.clone();
    for target in &mut missing.targets {
        *target = target
            .clone()
            .with_providers(vec![ProviderId::new("provider.absent")?])?;
    }
    match resolve_graph(missing, loaded.source_map()) {
        Err(ResolutionError::Findings(findings)) => assert!(!findings.is_empty()),
        other => return Err(format!("expected missing-provider findings, got {other:?}").into()),
    }
    Ok(())
}

#[test]
fn scenario_provider_reference_requires_declared_provider() -> Result<(), Box<dyn Error>> {
    use eqm_domain::{
        OccurrenceId, ProviderId, Revision, RoleId, Scenario, ScenarioId, ScenarioParticipant,
        ScenarioSubject,
    };
    let (_repository, loaded) = support::loaded_example()?;
    let scenario = Scenario::new(
        ScenarioId::new("signup.flow")?,
        Revision::new(1)?,
        vec![ScenarioParticipant::new(
            OccurrenceId::new("mailer")?,
            RoleId::new("provider")?,
            ScenarioSubject::Provider(ProviderId::new("provider.mailgun")?),
        )],
        Vec::new(),
        Extensions::default(),
    )?;
    let mut input = loaded.graph_input().clone();
    input.scenarios.push(scenario);
    match resolve_graph(input, loaded.source_map()) {
        Err(ResolutionError::Findings(findings)) => assert!(!findings.is_empty()),
        other => {
            return Err(format!("expected unresolved provider findings, got {other:?}").into());
        }
    }
    Ok(())
}
