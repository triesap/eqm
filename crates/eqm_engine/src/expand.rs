//! Exact-pinned, collision-free fragment expansion.

use crate::resolve::diagnostic;
use crate::{ResolutionError, validate_graph_invariants};
use eqm_domain::{
    FinalizedWorkspaceGraph, FragmentId, LocalRequirementId, RepoPath, Requirement, Revision,
    Sha256Digest, Surface, WorkspaceGraph,
};
use std::collections::{BTreeMap, BTreeSet};

/// Prepared canonical digest for each available fragment revision.
pub type FragmentDigestMap = BTreeMap<(FragmentId, Revision), Sha256Digest>;

/// Expands all exact fragment pins and returns the only graph type accepted by canonicalization.
pub fn expand_fragments(
    graph: WorkspaceGraph,
    digests: &FragmentDigestMap,
    sources: &BTreeMap<Box<str>, RepoPath>,
) -> Result<FinalizedWorkspaceGraph, ResolutionError> {
    validate_graph_invariants(&graph, sources)?;
    let mut findings = Vec::new();
    let mut input = graph.into_input();
    let fragments: BTreeMap<_, _> = input
        .fragments
        .iter()
        .map(|fragment| ((fragment.id().clone(), fragment.revision()), fragment))
        .collect();
    let mut expanded_surfaces = Vec::with_capacity(input.surfaces.len());
    for surface in &input.surfaces {
        let owner = format!("surface:{}", surface.id());
        let mut identities: BTreeSet<_> = surface.requirements().keys().cloned().collect();
        let mut requirements: Vec<_> = surface.requirements().values().cloned().collect();
        for pin in surface.fragments() {
            let key = (pin.fragment().clone(), pin.revision());
            let Some(fragment) = fragments.get(&key).copied() else {
                findings.push(diagnostic(
                    304,
                    format!(
                        "fragment pin `{}@{}` has no available semantic content",
                        pin.fragment(),
                        pin.revision().get()
                    ),
                    Some(&owner),
                    std::iter::empty(),
                    sources,
                )?);
                continue;
            };
            let Some(digest) = digests.get(&key) else {
                findings.push(diagnostic(
                    304,
                    format!(
                        "fragment pin `{}@{}` has no prepared canonical digest",
                        pin.fragment(),
                        pin.revision().get()
                    ),
                    Some(&owner),
                    std::iter::empty(),
                    sources,
                )?);
                continue;
            };
            if pin.digest() != digest {
                findings.push(diagnostic(
                    304,
                    format!(
                        "fragment pin `{}@{}` digest `{}` does not match `{digest}`",
                        pin.fragment(),
                        pin.revision().get(),
                        pin.digest()
                    ),
                    Some(&owner),
                    std::iter::empty(),
                    sources,
                )?);
                continue;
            }
            for requirement in fragment.requirements().values() {
                let final_id = expanded_id(pin.prefix(), requirement.id());
                let Ok(final_id) = final_id else {
                    findings.push(diagnostic(
                        305,
                        format!(
                            "fragment pin `{}@{}` produces an invalid requirement identity",
                            pin.fragment(),
                            pin.revision().get()
                        ),
                        Some(&owner),
                        std::iter::empty(),
                        sources,
                    )?);
                    continue;
                };
                if !identities.insert(final_id.clone()) {
                    findings.push(diagnostic(
                        305,
                        format!(
                            "fragment pin `{}@{}` collides at requirement `{final_id}`",
                            pin.fragment(),
                            pin.revision().get()
                        ),
                        Some(&owner),
                        std::iter::empty(),
                        sources,
                    )?);
                    continue;
                }
                requirements.push(rename_requirement(requirement, final_id)?);
            }
        }
        expanded_surfaces.push(
            Surface::new(
                surface.id().clone(),
                surface.revision(),
                surface.title().clone(),
                surface.journey().clone(),
                surface.status(),
                surface.owners().iter().cloned().collect(),
                requirements,
                surface.fragments().iter().cloned().collect(),
                surface.description().cloned(),
                surface.extensions().clone(),
            )
            .map_err(|_| ResolutionError::Expansion)?,
        );
    }
    if !findings.is_empty() {
        findings.sort();
        findings.dedup();
        return Err(ResolutionError::Findings(findings));
    }
    input.surfaces = expanded_surfaces;
    let graph = WorkspaceGraph::new(input).map_err(ResolutionError::Graph)?;
    Ok(FinalizedWorkspaceGraph::from_engine(graph))
}

fn expanded_id(
    prefix: Option<&LocalRequirementId>,
    local: &LocalRequirementId,
) -> Result<LocalRequirementId, eqm_domain::IdParseError> {
    match prefix {
        Some(prefix) => LocalRequirementId::new(format!("{}_{local}", prefix.as_str())),
        None => Ok(local.clone()),
    }
}

fn rename_requirement(
    requirement: &Requirement,
    id: LocalRequirementId,
) -> Result<Requirement, ResolutionError> {
    Requirement::new(
        id,
        requirement.level(),
        requirement.scope(),
        requirement.statement().clone(),
        requirement.facets().iter().copied().collect(),
        requirement.applicability().clone(),
        requirement.risk_class(),
        requirement.provider().cloned(),
        requirement.extensions().clone(),
    )
    .map_err(|_| ResolutionError::Expansion)
}
