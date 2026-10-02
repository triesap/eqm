//! S028: product roles are separate from shipped target identity.

use eqm_domain::{
    Extensions, FrameworkId, OwnerRef, PlatformId, RepoPath, RoleId, Target, TargetId,
};
use std::error::Error;
use std::str::FromStr;

fn target(id: &str, platform: &str, role: Option<&str>) -> Result<Target, Box<dyn Error>> {
    let target = Target::new(
        TargetId::new(id)?,
        RepoPath::new("apps/target")?,
        PlatformId::new(platform)?,
        FrameworkId::new("compose")?,
        vec![OwnerRef::from_str("owner://team/product")?],
        Extensions::default(),
    )?;
    Ok(match role {
        Some(role) => target.with_role(RoleId::new(role)?),
        None => target,
    })
}

#[test]
fn absent_role_is_typed_absence() -> Result<(), Box<dyn Error>> {
    let target = target("cli", "cli", None)?;
    assert_eq!(target.role(), None);
    assert_eq!(target.id().as_str(), "cli");
    Ok(())
}

#[test]
fn role_is_extensible_and_preserves_target_identity() -> Result<(), Box<dyn Error>> {
    let cli = target("cli", "cli", Some("control_tower"))?;
    assert_eq!(cli.role().map(RoleId::as_str), Some("control_tower"));
    // The role does not rename the target or move its other fields.
    assert_eq!(cli.id().as_str(), "cli");
    assert_eq!(cli.platform().as_str(), "cli");
    // An arbitrary authored symbolic role is accepted.
    assert!(target("edge", "android", Some("mobile_edge")).is_ok());
    Ok(())
}

#[test]
fn distinct_targets_may_share_one_role() -> Result<(), Box<dyn Error>> {
    let first = target("tower", "desktop", Some("control_tower"))?;
    let second = target("console", "web", Some("control_tower"))?;
    assert_ne!(first.id(), second.id());
    assert_eq!(first.role(), second.role());
    Ok(())
}

#[test]
fn same_target_role_mutation_is_observable() -> Result<(), Box<dyn Error>> {
    let first = target("cli", "cli", Some("control_tower"))?;
    let second = target("cli", "cli", Some("operator"))?;
    assert_ne!(first, second);
    assert_eq!(first.id(), second.id());
    assert_ne!(first.role(), second.role());
    Ok(())
}
