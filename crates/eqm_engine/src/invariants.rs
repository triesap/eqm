//! Post-resolution graph relationship and inheritance invariants.

use crate::ResolutionError;
use crate::resolve::diagnostic;
use eqm_domain::{Diagnostic, RepoPath, WorkspaceGraph};
use std::collections::{BTreeMap, BTreeSet};

/// Validates hierarchy, membership, lifecycle, and risk invariants.
///
/// Fragment nesting and capability/journey/surface parent cycles are
/// unrepresentable in the v1 typed model: fragments contain requirements only,
/// and every parent edge has a distinct source and destination ID type.
pub fn validate_graph_invariants(
    graph: &WorkspaceGraph,
    sources: &BTreeMap<Box<str>, RepoPath>,
) -> Result<(), ResolutionError> {
    let mut findings = Vec::new();
    validate_journeys(graph, sources, &mut findings)?;
    validate_surfaces(graph, sources, &mut findings)?;
    validate_fragments(graph, sources, &mut findings)?;
    findings.sort();
    findings.dedup();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(ResolutionError::Findings(findings))
    }
}

fn validate_journeys(
    graph: &WorkspaceGraph,
    sources: &BTreeMap<Box<str>, RepoPath>,
    findings: &mut Vec<Diagnostic>,
) -> Result<(), ResolutionError> {
    for journey in graph.journeys().values() {
        let owner = format!("journey:{}", journey.id());
        let expected_prefix = format!("{}.", journey.capability());
        finding(
            journey.id().as_str().starts_with(&expected_prefix),
            302,
            format!(
                "journey `{}` is outside capability `{}` identifier authority",
                journey.id(),
                journey.capability()
            ),
            &owner,
            sources,
            findings,
        )?;
        if let Some(capability) = graph.capabilities().get(journey.capability()) {
            finding(
                capability.status().allows_child(journey.status()),
                302,
                format!(
                    "journey `{}` lifecycle `{}` is invalid beneath capability lifecycle `{}`",
                    journey.id(),
                    journey.status(),
                    capability.status()
                ),
                &owner,
                sources,
                findings,
            )?;
        }
        let members: BTreeSet<_> = journey.surfaces().iter().collect();
        for transition in journey.transitions() {
            for endpoint in [transition.from(), transition.to()] {
                finding(
                    members.contains(endpoint),
                    302,
                    format!(
                        "journey `{}` transition endpoint `{endpoint}` is not a declared member",
                        journey.id()
                    ),
                    &owner,
                    sources,
                    findings,
                )?;
            }
        }
        for surface_id in journey.surfaces() {
            if let Some(surface) = graph.surfaces().get(surface_id) {
                finding(
                    surface.journey() == journey.id(),
                    302,
                    format!(
                        "journey `{}` lists surface `{surface_id}` owned by `{}`",
                        journey.id(),
                        surface.journey()
                    ),
                    &owner,
                    sources,
                    findings,
                )?;
            }
        }
    }
    Ok(())
}

fn validate_surfaces(
    graph: &WorkspaceGraph,
    sources: &BTreeMap<Box<str>, RepoPath>,
    findings: &mut Vec<Diagnostic>,
) -> Result<(), ResolutionError> {
    for surface in graph.surfaces().values() {
        let owner = format!("surface:{}", surface.id());
        let expected_prefix = format!("{}.", surface.journey());
        finding(
            surface.id().as_str().starts_with(&expected_prefix),
            302,
            format!(
                "surface `{}` is outside journey `{}` identifier authority",
                surface.id(),
                surface.journey()
            ),
            &owner,
            sources,
            findings,
        )?;
        if let Some(journey) = graph.journeys().get(surface.journey()) {
            finding(
                journey.status().allows_child(surface.status()),
                302,
                format!(
                    "surface `{}` lifecycle `{}` is invalid beneath journey lifecycle `{}`",
                    surface.id(),
                    surface.status(),
                    journey.status()
                ),
                &owner,
                sources,
                findings,
            )?;
            finding(
                surface.status().as_str() != "active" || journey.surfaces().contains(surface.id()),
                302,
                format!(
                    "active surface `{}` is orphaned from its journey",
                    surface.id()
                ),
                &owner,
                sources,
                findings,
            )?;
            for requirement in surface.requirements().values() {
                if let Some(risk) = requirement.risk_class() {
                    finding(
                        journey.risk_class().allows_child(risk),
                        303,
                        format!(
                            "surface requirement `{}#{}` lowers inherited risk `{}` to `{risk}`",
                            surface.id(),
                            requirement.id(),
                            journey.risk_class()
                        ),
                        &owner,
                        sources,
                        findings,
                    )?;
                }
            }
        }
    }
    Ok(())
}

fn validate_fragments(
    graph: &WorkspaceGraph,
    sources: &BTreeMap<Box<str>, RepoPath>,
    findings: &mut Vec<Diagnostic>,
) -> Result<(), ResolutionError> {
    for fragment in graph.fragments().values() {
        let owner = format!("fragment:{}@{}", fragment.id(), fragment.revision().get());
        for requirement in fragment.requirements().values() {
            if let Some(risk) = requirement.risk_class() {
                finding(
                    fragment.risk_class().allows_child(risk),
                    303,
                    format!(
                        "fragment requirement `{}#{}` lowers inherited risk `{}` to `{risk}`",
                        fragment.id(),
                        requirement.id(),
                        fragment.risk_class()
                    ),
                    &owner,
                    sources,
                    findings,
                )?;
            }
        }
    }
    Ok(())
}

fn finding(
    valid: bool,
    code: u16,
    message: String,
    owner: &str,
    sources: &BTreeMap<Box<str>, RepoPath>,
    findings: &mut Vec<Diagnostic>,
) -> Result<(), ResolutionError> {
    if !valid {
        findings.push(diagnostic(
            code,
            message,
            Some(owner),
            std::iter::empty(),
            sources,
        )?);
    }
    Ok(())
}
