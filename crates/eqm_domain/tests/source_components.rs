//! S029: shared source components and target-to-component relations.

use eqm_domain::{
    EntityBuildError, Extensions, FrameworkId, OwnerRef, PlatformId, RepoPath, SourceComponent,
    SourceComponentError, SourceComponentId, Target, TargetId, validate_source_components,
};
use std::error::Error;
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

fn target(id: &str, components: &[&str]) -> Result<Target, Box<dyn Error>> {
    let target = Target::new(
        TargetId::new(id)?,
        RepoPath::new("apps/target")?,
        PlatformId::new("kmp")?,
        FrameworkId::new("compose")?,
        vec![owner()?],
        Extensions::default(),
    )?;
    Ok(target.with_source_components(
        components
            .iter()
            .map(|component| SourceComponentId::new(*component))
            .collect::<Result<Vec<_>, _>>()?,
    )?)
}

#[test]
fn shared_components_attach_to_multiple_targets() -> Result<(), Box<dyn Error>> {
    let shared = component("shared.kmp", &["shared/src"], &[])?;
    let android = target("android", &["shared.kmp"])?;
    let ios = target("ios", &["shared.kmp"])?;
    assert_eq!(android.source_components().len(), 1);
    assert_eq!(ios.source_components().len(), 1);
    assert_eq!(
        android.source_components(),
        ios.source_components(),
        "distinct targets may share one component authority"
    );
    assert_eq!(shared.id().as_str(), "shared.kmp");
    assert!(
        shared
            .roots()
            .iter()
            .any(|root| root.as_str() == "shared/src")
    );
    Ok(())
}

#[test]
fn target_dependency_ids_are_modeled_as_canonical_sets() -> Result<(), Box<dyn Error>> {
    let target = target("android", &[])?;
    let target = target.with_depends_on(vec![TargetId::new("shared")?])?;
    assert_eq!(target.depends_on().len(), 1);
    assert!(target.depends_on().contains(&TargetId::new("shared")?));
    Ok(())
}

#[test]
fn duplicate_component_authority_rejects() -> Result<(), Box<dyn Error>> {
    let first = component("shared.kmp", &["shared/src"], &[])?;
    let second = component("shared.kmp", &["shared/other"], &[])?;
    assert_eq!(
        validate_source_components(vec![first, second]),
        Err(SourceComponentError::DuplicateAuthority)
    );
    Ok(())
}

#[test]
fn dangling_component_dependency_rejects() -> Result<(), Box<dyn Error>> {
    let first = component("shared.kmp", &["shared/src"], &["shared.absent"])?;
    assert_eq!(
        validate_source_components(vec![first]),
        Err(SourceComponentError::DanglingDependency)
    );
    Ok(())
}

#[test]
fn valid_component_registry_indexes_by_authority() -> Result<(), Box<dyn Error>> {
    let base = component("shared.base", &["shared/base"], &[])?;
    let kmp = component("shared.kmp", &["shared/kmp"], &["shared.base"])?;
    let indexed = validate_source_components(vec![kmp, base])?;
    assert_eq!(indexed.len(), 2);
    assert!(indexed.contains_key(&SourceComponentId::new("shared.kmp")?));
    Ok(())
}

#[test]
fn invalid_root_and_self_or_duplicate_inputs_reject() -> Result<(), Box<dyn Error>> {
    assert!(RepoPath::new("../escape").is_err());
    assert_eq!(
        SourceComponent::new(
            SourceComponentId::new("shared.kmp")?,
            vec![],
            vec![owner()?],
            vec![],
            Extensions::default(),
        ),
        Err(EntityBuildError::SourceRootsRequired)
    );
    assert_eq!(
        SourceComponent::new(
            SourceComponentId::new("shared.kmp")?,
            vec![RepoPath::new("shared/src")?, RepoPath::new("shared/src")?],
            vec![owner()?],
            vec![],
            Extensions::default(),
        ),
        Err(EntityBuildError::DuplicateSourceRoot)
    );
    assert_eq!(
        SourceComponent::new(
            SourceComponentId::new("shared.kmp")?,
            vec![RepoPath::new("shared/src")?],
            vec![owner()?],
            vec![SourceComponentId::new("shared.kmp")?],
            Extensions::default(),
        ),
        Err(EntityBuildError::SourceSelfDependency)
    );
    let base = Target::new(
        TargetId::new("android")?,
        RepoPath::new("apps/target")?,
        PlatformId::new("kmp")?,
        FrameworkId::new("compose")?,
        vec![owner()?],
        Extensions::default(),
    )?;
    assert_eq!(
        base.with_source_components(vec![
            SourceComponentId::new("shared.kmp")?,
            SourceComponentId::new("shared.kmp")?
        ]),
        Err(EntityBuildError::DuplicateTargetSourceComponent)
    );
    let base = Target::new(
        TargetId::new("android")?,
        RepoPath::new("apps/target")?,
        PlatformId::new("kmp")?,
        FrameworkId::new("compose")?,
        vec![owner()?],
        Extensions::default(),
    )?;
    assert_eq!(
        base.with_depends_on(vec![TargetId::new("shared")?, TargetId::new("shared")?]),
        Err(EntityBuildError::DuplicateTargetDependency)
    );
    Ok(())
}
