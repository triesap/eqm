//! Deterministic cross-reference resolution into an immutable graph.

use crate::{diagnostic_registry, explain_diagnostic};
use eqm_domain::{
    Diagnostic, DiagnosticBuildError, DiagnosticCode, DiagnosticDescriptor, EvidenceScopeSubject,
    RepoPath, SourceLocation, SourceName, SourcePosition, WorkspaceGraph, WorkspaceGraphBuildError,
    WorkspaceGraphInput,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Returns the stable graph-resolution diagnostic registry.
pub fn resolution_diagnostics() -> Result<[DiagnosticDescriptor; 6], DiagnosticBuildError> {
    diagnostic_registry()
}

/// Resolves every typed cross-reference and constructs deterministic graph indexes.
pub fn resolve_graph(
    input: WorkspaceGraphInput,
    sources: &BTreeMap<Box<str>, RepoPath>,
) -> Result<WorkspaceGraph, ResolutionError> {
    let duplicates = duplicate_findings(&input, sources)?;
    if !duplicates.is_empty() {
        return Err(ResolutionError::Findings(duplicates));
    }
    let graph = WorkspaceGraph::new(input).map_err(ResolutionError::Graph)?;
    let dangling = dangling_findings(&graph, sources)?;
    if !dangling.is_empty() {
        return Err(ResolutionError::Findings(dangling));
    }
    Ok(graph)
}

fn duplicate_findings(
    input: &WorkspaceGraphInput,
    sources: &BTreeMap<Box<str>, RepoPath>,
) -> Result<Vec<Diagnostic>, ResolutionError> {
    let mut entries = Vec::new();
    entries.extend(input.capabilities.iter().map(|value| {
        let id = value.id().as_str();
        (format!("capability:{id}"), format!("capability:{id}"))
    }));
    entries.extend(input.journeys.iter().map(|value| {
        let id = value.id().as_str();
        (format!("journey:{id}"), format!("journey:{id}"))
    }));
    entries.extend(input.surfaces.iter().map(|value| {
        let id = value.id().as_str();
        (format!("surface:{id}"), format!("surface:{id}"))
    }));
    entries.extend(input.fragments.iter().map(|value| {
        let id = format!("{}@{}", value.id(), value.revision().get());
        (format!("fragment:{id}"), format!("fragment:{id}"))
    }));
    entries.extend(input.targets.iter().map(|value| {
        let id = value.id().as_str();
        (format!("target:{id}"), format!("target:{id}"))
    }));
    entries.extend(input.bindings.iter().map(|value| {
        let id = value.id().as_str();
        (format!("binding:{id}"), format!("binding:{id}"))
    }));
    entries.extend(input.policies.iter().map(|value| {
        let id = format!("{}@{}", value.id(), value.revision().get());
        (format!("policy:{id}"), format!("policy:{id}"))
    }));
    entries.extend(input.profiles.iter().map(|value| {
        let id = format!("{}@{}", value.id(), value.revision().get());
        (format!("profile:{id}"), format!("profile:{id}"))
    }));
    entries.extend(input.runners.iter().map(|value| {
        let id = format!("{}@{}", value.id(), value.revision().get());
        (format!("runner:{id}"), format!("runner:{id}"))
    }));
    entries.extend(input.waivers.iter().map(|value| {
        let id = format!("{}@{}", value.id(), value.revision().get());
        (format!("waiver:{id}"), format!("waiver:{id}"))
    }));
    entries.extend(input.imports.iter().map(|value| {
        let id = format!("{}@{}:{}", value.id, value.revision.get(), value.digest);
        (
            format!("import:{id}"),
            format!("import:{}@{}", value.id, value.revision.get()),
        )
    }));
    entries.extend(input.adapter_locks.iter().map(|value| {
        let id = format!("{}@{}:{}", value.id, value.version.as_str(), value.digest);
        (
            format!("adapter_lock:{id}"),
            format!("adapter_lock:{}@{}", value.id, value.version.as_str()),
        )
    }));
    entries.extend(input.adapters.iter().map(|value| {
        let id = format!(
            "{}@{}:{}",
            value.id(),
            value.version().as_str(),
            value.digest()
        );
        (format!("adapter:{id}"), format!("adapter:{id}"))
    }));
    for binding in &input.bindings {
        entries.push((
            format!("binding_coordinate:{}:{}", binding.target(), binding.unit()),
            format!("binding:{}", binding.id()),
        ));
    }

    let mut grouped = BTreeMap::<String, Vec<String>>::new();
    for (identity, authority) in entries {
        grouped.entry(identity).or_default().push(authority);
    }
    let mut findings = Vec::new();
    for (identity, authorities) in grouped {
        if authorities.len() > 1 {
            findings.push(diagnostic(
                300,
                format!("duplicate semantic authority `{identity}`"),
                authorities.first().map(String::as_str),
                authorities.iter().skip(1).map(String::as_str),
                sources,
            )?);
        }
    }
    findings.sort();
    Ok(findings)
}

fn dangling_findings(
    graph: &WorkspaceGraph,
    sources: &BTreeMap<Box<str>, RepoPath>,
) -> Result<Vec<Diagnostic>, ResolutionError> {
    let capability_ids: BTreeSet<_> = graph.capabilities().keys().map(|id| id.as_str()).collect();
    let journey_ids: BTreeSet<_> = graph.journeys().keys().map(|id| id.as_str()).collect();
    let surface_ids: BTreeSet<_> = graph.surfaces().keys().map(|id| id.as_str()).collect();
    let fragment_revisions: BTreeSet<_> = graph
        .fragments()
        .keys()
        .map(|(id, revision)| (id.as_str(), revision.get()))
        .chain(
            graph
                .imports()
                .keys()
                .map(|(id, revision, _)| (id.as_str(), revision.get())),
        )
        .collect();
    let target_ids: BTreeSet<_> = graph.targets().keys().map(|id| id.as_str()).collect();
    let profile_ids: BTreeSet<_> = graph.profiles().keys().map(|(id, _)| id.as_str()).collect();
    let policy_ids: BTreeSet<_> = graph.policies().keys().map(|(id, _)| id.as_str()).collect();
    let runner_ids: BTreeSet<_> = graph.runners().keys().map(|(id, _)| id.as_str()).collect();
    let unit_ids: BTreeSet<_> = capability_ids
        .iter()
        .copied()
        .chain(journey_ids.iter().copied())
        .chain(surface_ids.iter().copied())
        .chain(graph.fragments().keys().map(|(id, _)| id.as_str()))
        .collect();
    let requirement_ids: BTreeSet<String> = graph
        .surfaces()
        .values()
        .flat_map(|surface| {
            surface
                .requirements()
                .keys()
                .map(|requirement| format!("{}#{requirement}", surface.id()))
        })
        .chain(graph.fragments().values().flat_map(|fragment| {
            fragment
                .requirements()
                .keys()
                .map(|requirement| format!("{}#{requirement}", fragment.id()))
        }))
        .collect();

    let mut findings = Vec::new();
    for journey in graph.journeys().values() {
        let owner = format!("journey:{}", journey.id());
        require(
            capability_ids.contains(journey.capability().as_str()),
            &owner,
            "capability",
            journey.capability().as_str(),
            sources,
            &mut findings,
        )?;
        for surface in journey.surfaces() {
            require(
                surface_ids.contains(surface.as_str()),
                &owner,
                "surface",
                surface.as_str(),
                sources,
                &mut findings,
            )?;
        }
        for transition in journey.transitions() {
            for endpoint in [transition.from(), transition.to()] {
                require(
                    surface_ids.contains(endpoint.as_str()),
                    &owner,
                    "transition surface",
                    endpoint.as_str(),
                    sources,
                    &mut findings,
                )?;
            }
        }
    }
    for surface in graph.surfaces().values() {
        let owner = format!("surface:{}", surface.id());
        require(
            journey_ids.contains(surface.journey().as_str()),
            &owner,
            "journey",
            surface.journey().as_str(),
            sources,
            &mut findings,
        )?;
        for fragment in surface.fragments() {
            require(
                fragment_revisions
                    .contains(&(fragment.fragment().as_str(), fragment.revision().get())),
                &owner,
                "fragment revision",
                &format!("{}@{}", fragment.fragment(), fragment.revision().get()),
                sources,
                &mut findings,
            )?;
        }
    }
    for binding in graph.bindings().values() {
        let owner = format!("binding:{}", binding.id());
        require(
            target_ids.contains(binding.target().as_str()),
            &owner,
            "target",
            binding.target().as_str(),
            sources,
            &mut findings,
        )?;
        require(
            unit_ids.contains(binding.unit().as_str()),
            &owner,
            "unit",
            binding.unit().as_str(),
            sources,
            &mut findings,
        )?;
        for artifact in binding.artifacts().values().values() {
            if let Some(surface) = artifact.surface() {
                require(
                    surface_ids.contains(surface.as_str()),
                    &owner,
                    "surface",
                    surface.as_str(),
                    sources,
                    &mut findings,
                )?;
            }
        }
        for exposure in binding.exposures() {
            require(
                surface_ids.contains(exposure.surface().as_str()),
                &owner,
                "surface",
                exposure.surface().as_str(),
                sources,
                &mut findings,
            )?;
        }
        for evidence in binding.evidence().values() {
            for requirement in evidence.requirements() {
                require(
                    requirement_ids.contains(requirement.as_str()),
                    &owner,
                    "requirement",
                    requirement.as_str(),
                    sources,
                    &mut findings,
                )?;
            }
            if let Some(runner) = evidence.runner() {
                require(
                    runner_ids.contains(runner.as_str()),
                    &owner,
                    "runner",
                    runner.as_str(),
                    sources,
                    &mut findings,
                )?;
            }
        }
    }
    for policy in graph.policies().values() {
        let owner = format!("policy:{}@{}", policy.id(), policy.revision().get());
        for profile in policy.profiles() {
            require(
                profile_ids.contains(profile.as_str()),
                &owner,
                "profile",
                profile.as_str(),
                sources,
                &mut findings,
            )?;
        }
        for target in policy.required_targets() {
            require(
                target_ids.contains(target.as_str()),
                &owner,
                "target",
                target.as_str(),
                sources,
                &mut findings,
            )?;
        }
        for rule in policy.rules() {
            if let Some(units) = rule.selector().units() {
                for unit in units {
                    require(
                        unit_ids.contains(unit.as_str()),
                        &owner,
                        "unit",
                        unit.as_str(),
                        sources,
                        &mut findings,
                    )?;
                }
            }
            if let Some(requirements) = rule.selector().requirements() {
                for requirement in requirements {
                    require(
                        requirement_ids.contains(requirement.as_str()),
                        &owner,
                        "requirement",
                        requirement.as_str(),
                        sources,
                        &mut findings,
                    )?;
                }
            }
        }
    }
    for waiver in graph.waivers().values() {
        let owner = format!("waiver:{}@{}", waiver.id(), waiver.revision().get());
        require(
            policy_ids.contains(waiver.policy().as_str()),
            &owner,
            "policy",
            waiver.policy().as_str(),
            sources,
            &mut findings,
        )?;
        require(
            unit_ids.contains(waiver.scope().unit().as_str()),
            &owner,
            "unit",
            waiver.scope().unit().as_str(),
            sources,
            &mut findings,
        )?;
        require(
            requirement_ids.contains(waiver.scope().requirement().as_str()),
            &owner,
            "requirement",
            waiver.scope().requirement().as_str(),
            sources,
            &mut findings,
        )?;
        for profile in waiver.scope().profiles().keys() {
            require(
                profile_ids.contains(profile.as_str()),
                &owner,
                "profile",
                profile.as_str(),
                sources,
                &mut findings,
            )?;
        }
        match waiver.scope().target() {
            EvidenceScopeSubject::Target(target) => require(
                target_ids.contains(target.as_str()),
                &owner,
                "target",
                target.as_str(),
                sources,
                &mut findings,
            )?,
            EvidenceScopeSubject::TargetSet(targets) => {
                for target in targets {
                    require(
                        target_ids.contains(target.as_str()),
                        &owner,
                        "target",
                        target.as_str(),
                        sources,
                        &mut findings,
                    )?;
                }
            }
            EvidenceScopeSubject::Provider(_) => {}
        }
    }
    findings.sort();
    findings.dedup();
    Ok(findings)
}

fn require(
    exists: bool,
    owner: &str,
    kind: &str,
    reference: &str,
    sources: &BTreeMap<Box<str>, RepoPath>,
    findings: &mut Vec<Diagnostic>,
) -> Result<(), ResolutionError> {
    if !exists {
        findings.push(diagnostic(
            301,
            format!("{owner} references missing {kind} `{reference}`"),
            Some(owner),
            std::iter::empty(),
            sources,
        )?);
    }
    Ok(())
}

pub(crate) fn diagnostic<'a>(
    code: u16,
    message: String,
    primary: Option<&str>,
    related: impl Iterator<Item = &'a str>,
    sources: &BTreeMap<Box<str>, RepoPath>,
) -> Result<Diagnostic, ResolutionError> {
    let code = DiagnosticCode::from_number(code)
        .ok_or(DiagnosticBuildError::InvalidCode)
        .map_err(ResolutionError::Diagnostic)?;
    let source = primary
        .and_then(|key| sources.get(key))
        .map(source_location)
        .transpose()
        .map_err(ResolutionError::Diagnostic)?;
    let related = related
        .filter_map(|key| sources.get(key))
        .map(source_location)
        .collect::<Result<Vec<_>, _>>()
        .map_err(ResolutionError::Diagnostic)?;
    let descriptor = explain_diagnostic(code)
        .map_err(ResolutionError::Diagnostic)?
        .ok_or(DiagnosticBuildError::InvalidCode)
        .map_err(ResolutionError::Diagnostic)?;
    Diagnostic::from_descriptor(&descriptor, message, source, related)
        .map_err(ResolutionError::Diagnostic)
}

fn source_location(path: &RepoPath) -> Result<SourceLocation, DiagnosticBuildError> {
    SourceLocation::new(
        SourceName::new(path.as_str())?,
        SourcePosition::new(1, 1)?,
        SourcePosition::new(1, 1)?,
    )
}

/// Graph resolution failure.
#[derive(Debug)]
pub enum ResolutionError {
    /// Stable user-facing resolution diagnostics.
    Findings(Vec<Diagnostic>),
    /// Construction of a diagnostic violated its internal contract.
    Diagnostic(DiagnosticBuildError),
    /// Deterministic graph index construction found an unanticipated conflict.
    Graph(WorkspaceGraphBuildError),
    /// Rebuilding validated entities during expansion violated an internal contract.
    Expansion,
}

impl ResolutionError {
    /// Returns stable findings when this is an authored-input failure.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        match self {
            Self::Findings(values) => values,
            _ => &[],
        }
    }
}

impl Display for ResolutionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Findings(values) => write!(
                formatter,
                "graph resolution produced {} finding(s)",
                values.len()
            ),
            Self::Diagnostic(error) => write!(formatter, "invalid resolution diagnostic: {error}"),
            Self::Graph(error) => write!(formatter, "graph index construction failed: {error}"),
            Self::Expansion => formatter.write_str("fragment expansion reconstruction failed"),
        }
    }
}

impl Error for ResolutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Diagnostic(error) => Some(error),
            Self::Graph(error) => Some(error),
            Self::Findings(_) => None,
            Self::Expansion => None,
        }
    }
}
