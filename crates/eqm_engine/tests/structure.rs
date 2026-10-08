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
            roles: Some(BTreeSet::from([ArtifactRole::View])),
        },
    )]);
    assert!(evaluate_structure(binding, target, &view, false).satisfied());

    view.get_mut(&path).ok_or("entry missing")?.roles = None;
    let unavailable = evaluate_structure(binding, target, &view, false);
    assert!(!unavailable.satisfied());
    assert!(
        unavailable
            .findings
            .iter()
            .any(|finding| { finding.kind == StructureFindingKind::RoleUnknown })
    );
    assert!(
        !unavailable
            .findings
            .iter()
            .any(|finding| { finding.kind == StructureFindingKind::RoleMismatch })
    );

    view.get_mut(&path).ok_or("entry missing")?.roles = Some(BTreeSet::new());
    assert!(
        evaluate_structure(binding, target, &view, false)
            .findings
            .iter()
            .any(|finding| finding.kind == StructureFindingKind::RoleMismatch)
    );
    view.get_mut(&path).ok_or("entry missing")?.roles = Some(BTreeSet::from([ArtifactRole::View]));
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
            roles: Some(BTreeSet::from([ArtifactRole::View])),
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

#[test]
fn declared_source_component_root_admits_a_shared_artifact() -> Result<(), Box<dyn Error>> {
    use eqm_domain::{
        Artifact, ArtifactId, Artifacts, Binding, BindingId, Extensions, FrameworkId, OwnerRef,
        PlatformId, Revision, SurfaceId, Target, TargetId, UnitId,
    };
    use eqm_engine::evaluate_structure_with_roots;
    use std::str::FromStr;

    let binding = Binding::new(
        BindingId::new("binding.web")?,
        Revision::new(1)?,
        vec![OwnerRef::from_str("owner://team/web")?],
        TargetId::new("web")?,
        UnitId::new("account.create.primary.form")?,
        Artifacts::new(vec![Artifact::new(
            ArtifactId::new("form")?,
            ArtifactRole::View,
            RepoPath::new("shared/kmp/src/form.ts")?,
            Some(SurfaceId::new("account.create.primary.form")?),
            None,
            None,
            Extensions::default(),
        )?])?,
        Vec::new(),
        Vec::new(),
        Extensions::default(),
    )?;
    let target = Target::new(
        TargetId::new("web")?,
        RepoPath::new("apps/web")?,
        PlatformId::new("web")?,
        FrameworkId::new("sveltekit")?,
        vec![OwnerRef::from_str("owner://team/web")?],
        Extensions::default(),
    )?;
    let path = RepoPath::new("shared/kmp/src/form.ts")?;
    let view = RepositoryView::from([(
        path,
        RepositoryEntry {
            kind: RepositoryEntryKind::File,
            resolved: None,
            roles: Some(BTreeSet::from([ArtifactRole::View])),
        },
    )]);

    // The shipped output root alone does not own the shared artifact.
    assert!(
        evaluate_structure(&binding, &target, &view, false)
            .findings
            .iter()
            .any(|finding| finding.kind == StructureFindingKind::OutsideTargetRoot)
    );
    // The declared source component ownership admits it.
    let roots = vec![RepoPath::new("apps/web")?, RepoPath::new("shared/kmp")?];
    assert!(evaluate_structure_with_roots(&binding, &roots, &view, false).satisfied());
    Ok(())
}
