//! Injected repository-view structural fixtures.

use eqm_domain::{ArtifactRole, RepoPath};
use eqm_engine::{
    RepositoryEntry, RepositoryEntryKind, RepositoryView, StructureFindingKind, evaluate_structure,
    resolve_graph,
};
use eqm_manifest::load_workspace;
use std::collections::BTreeSet;
use std::error::Error;
use std::path::Path;

fn repository_root() -> Result<&'static Path, Box<dyn Error>> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "workspace root unavailable".into())
}

#[test]
fn existence_role_collision_and_symlink_resolution_are_fail_closed() -> Result<(), Box<dyn Error>> {
    let loaded = load_workspace(repository_root()?, None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let binding = graph.bindings().values().next().ok_or("binding missing")?;
    let target = graph
        .targets()
        .get(binding.target())
        .ok_or("target missing")?;
    let path = RepoPath::new("apps/web/src/signup/identifier.svelte")?;
    let mut view = RepositoryView::from([(
        path.clone(),
        RepositoryEntry {
            kind: RepositoryEntryKind::File,
            resolved: None,
            roles: BTreeSet::from([ArtifactRole::View]),
        },
    )]);
    assert!(evaluate_structure(binding, target, &view, false).satisfied());

    view.get_mut(&path).ok_or("entry missing")?.roles.clear();
    assert!(
        evaluate_structure(binding, target, &view, false)
            .findings
            .iter()
            .any(|finding| finding.kind == StructureFindingKind::RoleMismatch)
    );
    view.get_mut(&path)
        .ok_or("entry missing")?
        .roles
        .insert(ArtifactRole::View);
    view.get_mut(&path).ok_or("entry missing")?.kind = RepositoryEntryKind::Symlink;
    view.get_mut(&path).ok_or("entry missing")?.resolved =
        Some(RepoPath::new("other/identifier.svelte")?);
    assert!(
        evaluate_structure(binding, target, &view, true)
            .findings
            .iter()
            .any(|finding| finding.kind == StructureFindingKind::InvalidSymlink)
    );
    view.insert(
        RepoPath::new("apps/web/src/signup/IDENTIFIER.svelte")?,
        RepositoryEntry {
            kind: RepositoryEntryKind::File,
            resolved: None,
            roles: BTreeSet::from([ArtifactRole::View]),
        },
    );
    assert!(
        evaluate_structure(binding, target, &view, true)
            .findings
            .iter()
            .any(|finding| finding.kind == StructureFindingKind::PortableCollision)
    );
    Ok(())
}
