//! Injected repository-view structural fixtures.

mod support;

use eqm_domain::{ArtifactRole, RepoPath};
use eqm_engine::{
    RepositoryEntry, RepositoryEntryKind, RepositoryView, StructureFindingKind, evaluate_structure,
    resolve_graph,
};
use std::collections::BTreeSet;
use std::error::Error;

#[test]
fn existence_role_collision_and_symlink_resolution_are_fail_closed() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let binding = graph.bindings().values().next().ok_or("binding missing")?;
    let target = graph
        .targets()
        .get(binding.target())
        .ok_or("target missing")?;
    let path = RepoPath::new("apps/android/app/src/main/java/com/example/signup/SignupScreen.kt")?;
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
        RepoPath::new("apps/android/app/src/main/java/com/example/signup/SIGNUPSCREEN.kt")?,
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
