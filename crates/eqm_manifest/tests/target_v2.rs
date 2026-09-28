//! S020: exact v2 target-configuration projection and `TargetConfigurationDigest`.
//!
//! Literal byte/digest oracles are reviewed constants computed independently
//! from the projection code. The controls prove the exact six-member root,
//! owner-set canonicalization, semantic-extension inclusion with display-only
//! exclusion, and the recursive extension type distinction.

use eqm_domain::{
    ExtensionKey, ExtensionNamespace, ExtensionValue, Extensions, FrameworkId, OwnerRef,
    PlatformId, RepoPath, Target, TargetConfigurationDigest, TargetId,
};
use eqm_manifest::{canonicalize_target_configuration_v2, project_target_configuration_v2};
use std::collections::BTreeMap;
use std::error::Error;
use std::str::FromStr;

const BASE_BYTES: &str = r#"{"extensions":{},"framework":"sveltekit","id":"web","owners":["owner://team/web"],"platform":"web","root":"apps/web"}"#;
const BASE_DIGEST: &str = "sha256:52a4e99e8a7096ba27535fa7c74fa755534298263b77f8fa5e9a68bda4f3c490";

fn target_with(
    id: &str,
    root: &str,
    platform: &str,
    framework: &str,
    owners: &[&str],
    extensions: Extensions,
) -> Result<Target, Box<dyn Error>> {
    Ok(Target::new(
        TargetId::new(id)?,
        RepoPath::new(root)?,
        PlatformId::new(platform)?,
        FrameworkId::new(framework)?,
        owners
            .iter()
            .map(|owner| OwnerRef::from_str(owner))
            .collect::<Result<Vec<_>, _>>()?,
        extensions,
    )?)
}

fn base_target() -> Result<Target, Box<dyn Error>> {
    target_with(
        "web",
        "apps/web",
        "web",
        "sveltekit",
        &["owner://team/web"],
        Extensions::default(),
    )
}

fn display_extensions() -> Result<Extensions, Box<dyn Error>> {
    Ok(Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.display")?,
        ExtensionValue::String("hidden".into()),
    )]))?)
}

fn extra_extensions(value: ExtensionValue) -> Result<Extensions, Box<dyn Error>> {
    Ok(Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.extra")?,
        value,
    )]))?)
}

#[test]
fn literal_target_bytes_and_digest_match_independent_oracle() -> Result<(), Box<dyn Error>> {
    let canonical = canonicalize_target_configuration_v2(&base_target()?)?;
    eprintln!("TARGET_BYTES={}", std::str::from_utf8(canonical.bytes())?);
    eprintln!("TARGET_DIGEST={}", canonical.digest());
    assert_eq!(std::str::from_utf8(canonical.bytes())?, BASE_BYTES);
    assert_eq!(canonical.digest().to_string(), BASE_DIGEST);
    assert!(!canonical.bytes().ends_with(b"\n"));
    Ok(())
}

#[test]
fn projection_root_has_exactly_the_current_target_fields() -> Result<(), Box<dyn Error>> {
    let projected = project_target_configuration_v2(&base_target()?)?;
    let object = projected.as_object().ok_or("projected target")?;
    let keys: Vec<&str> = object.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        vec![
            "extensions",
            "framework",
            "id",
            "owners",
            "platform",
            "root"
        ]
    );
    for absent in [
        "schema",
        "build",
        "build_id",
        "artifact",
        "artifact_digest",
        "profile",
        "profiles",
        "source",
        "source_component",
        "source_revision",
        "workspace_digest",
        "target",
    ] {
        assert!(
            !object.contains_key(absent),
            "the target root must not invent a {absent} field"
        );
    }
    Ok(())
}

#[test]
fn target_identity_is_the_typed_domain_wrapper() -> Result<(), Box<dyn Error>> {
    let canonical = canonicalize_target_configuration_v2(&base_target()?)?;
    let typed: TargetConfigurationDigest = canonical.digest();
    assert_eq!(
        typed,
        TargetConfigurationDigest::hash_content(canonical.bytes())
    );
    Ok(())
}

#[test]
fn same_id_field_and_semantic_extension_mutations_change_identity() -> Result<(), Box<dyn Error>> {
    let base = canonicalize_target_configuration_v2(&base_target()?)?;
    for changed in [
        target_with(
            "web",
            "apps/other",
            "web",
            "sveltekit",
            &["owner://team/web"],
            Extensions::default(),
        )?,
        target_with(
            "web",
            "apps/web",
            "android",
            "sveltekit",
            &["owner://team/web"],
            Extensions::default(),
        )?,
        target_with(
            "web",
            "apps/web",
            "web",
            "compose",
            &["owner://team/web"],
            Extensions::default(),
        )?,
        target_with(
            "web",
            "apps/web",
            "web",
            "sveltekit",
            &["owner://team/other"],
            Extensions::default(),
        )?,
        target_with(
            "web",
            "apps/web",
            "web",
            "sveltekit",
            &["owner://team/web"],
            extra_extensions(ExtensionValue::String("x".into()))?,
        )?,
    ] {
        assert_ne!(
            base.digest(),
            canonicalize_target_configuration_v2(&changed)?.digest(),
            "a same-ID target field or semantic-extension change must alter identity"
        );
    }
    Ok(())
}

#[test]
fn display_extensions_do_not_change_identity() -> Result<(), Box<dyn Error>> {
    let base = canonicalize_target_configuration_v2(&base_target()?)?;
    let decorated = target_with(
        "web",
        "apps/web",
        "web",
        "sveltekit",
        &["owner://team/web"],
        display_extensions()?,
    )?;
    assert_eq!(base, canonicalize_target_configuration_v2(&decorated)?);
    Ok(())
}

#[test]
fn owner_permutations_canonicalize() -> Result<(), Box<dyn Error>> {
    let forward = target_with(
        "web",
        "apps/web",
        "web",
        "sveltekit",
        &["owner://team/web", "owner://team/accounts"],
        Extensions::default(),
    )?;
    let reversed = target_with(
        "web",
        "apps/web",
        "web",
        "sveltekit",
        &["owner://team/accounts", "owner://team/web"],
        Extensions::default(),
    )?;
    let projected = project_target_configuration_v2(&forward)?;
    assert_eq!(
        projected["owners"],
        serde_json::json!(["owner://team/accounts", "owner://team/web"])
    );
    assert_eq!(
        canonicalize_target_configuration_v2(&forward)?,
        canonicalize_target_configuration_v2(&reversed)?
    );
    Ok(())
}

#[test]
fn extension_integer_and_text_with_same_digits_never_collide() -> Result<(), Box<dyn Error>> {
    let build = |value: ExtensionValue| -> Result<_, Box<dyn Error>> {
        Ok(canonicalize_target_configuration_v2(&target_with(
            "web",
            "apps/web",
            "web",
            "sveltekit",
            &["owner://team/web"],
            extra_extensions(value)?,
        )?)?)
    };
    for (integer, text) in [
        (ExtensionValue::Integer(i64::MIN), "-9223372036854775808"),
        (
            ExtensionValue::Integer(9_007_199_254_740_992),
            "9007199254740992",
        ),
    ] {
        let integer = build(integer)?;
        let string = build(ExtensionValue::String(text.into()))?;
        assert_ne!(integer.bytes(), string.bytes());
        assert_ne!(integer.digest(), string.digest());
    }
    Ok(())
}

fn extra_extensions_named(value: ExtensionValue) -> Result<Extensions, Box<dyn Error>> {
    Ok(Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.extra")?,
        ExtensionValue::Object(BTreeMap::from([(ExtensionKey::new("marker")?, value)])),
    )]))?)
}

#[test]
fn nested_large_extension_values_remain_distinct() -> Result<(), Box<dyn Error>> {
    let base = canonicalize_target_configuration_v2(&base_target()?)?;
    let large_integer = canonicalize_target_configuration_v2(&target_with(
        "web",
        "apps/web",
        "web",
        "sveltekit",
        &["owner://team/web"],
        extra_extensions_named(ExtensionValue::Integer(i64::MAX))?,
    )?)?;
    let large_text = canonicalize_target_configuration_v2(&target_with(
        "web",
        "apps/web",
        "web",
        "sveltekit",
        &["owner://team/web"],
        extra_extensions_named(ExtensionValue::String("9223372036854775807".into()))?,
    )?)?;
    assert_ne!(base.digest(), large_integer.digest());
    assert_ne!(large_integer.bytes(), large_text.bytes());
    Ok(())
}
