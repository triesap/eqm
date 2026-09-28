//! S019-C: v2 selected-policy/profile projection and `PolicySelectionDigest`.
//!
//! Literal byte/digest oracles are reviewed constants computed independently from
//! the projection code. Controls prove known/unknown state, same-ID/revision
//! definition binding, display-only exclusion, canonical sets, and the recursive
//! extension type distinction.

use eqm_domain::{
    Description, DurationMillis, Extensions, Facet, OwnerRef, Policy, PolicyId, PolicyRule,
    PolicySelectionInputV2, PolicySelector, PositiveCount, Profile, ProfileDimension, ProfileId,
    RequirementLevel, Revision, SelectedDimensionStateV2, SelectedProfileInputV2, SymbolicValueId,
    TargetId, Title, TrustLevel, UnitId, WaiverPolicy,
};
use eqm_manifest::{canonicalize_policy_selection_v2, project_policy_selection_v2};
use std::error::Error;

const POLICY_BYTES: &str = r#"{"policy":{"description":"Baseline policy","extensions":{},"id":"policy.default","owners":["owner://team/security"],"profiles":["audience.default"],"required_targets":["web"],"revision":1,"rules":[{"facets":["behavior"],"maximum_age":86400000,"minimum_count":1,"minimum_level":"recommended","minimum_trust":"trusted_ci","selector":{"units":["account.create"]}}],"title":"Default policy","waivers":{"allowed":false,"minimum_approvers":1,"required_controls":[]}},"profiles":[{"definition":{"defaults":{"locale":"en"},"description":"Default audience","dimensions":[{"id":"locale","values":["en","fr"]},{"id":"region","values":["eu","us"]}],"extensions":{},"id":"audience.default","owners":["owner://team/product"],"revision":1,"title":"Audience"},"values":[{"dimension":"locale","state":"known","value":"en"},{"dimension":"region","state":"unknown"}]}]}"#;
const POLICY_DIGEST: &str =
    "sha256:0ba1fc91cb154a5b5b6d57d80e379c55f6bf0588d6cd2dfbaefcab2611fde60f";

fn owner(value: &str) -> Result<OwnerRef, Box<dyn Error>> {
    Ok(value.parse()?)
}

fn profile(revision: u64, title: &str) -> Result<Profile, Box<dyn Error>> {
    Ok(Profile::new(
        ProfileId::new("audience.default")?,
        Revision::new(revision)?,
        Title::new(title)?,
        vec![owner("owner://team/product")?],
        vec![
            ProfileDimension::new(
                eqm_domain::DimensionId::new("locale")?,
                vec![SymbolicValueId::new("en")?, SymbolicValueId::new("fr")?],
                None,
            )?,
            ProfileDimension::new(
                eqm_domain::DimensionId::new("region")?,
                vec![SymbolicValueId::new("eu")?, SymbolicValueId::new("us")?],
                None,
            )?,
        ],
        vec![(
            eqm_domain::DimensionId::new("locale")?,
            SymbolicValueId::new("en")?,
        )],
        Some(Description::new("Default audience")?),
        Extensions::default(),
    )?)
}

fn policy(profiles: Vec<ProfileId>) -> Result<Policy, Box<dyn Error>> {
    let rule = PolicyRule::new(
        PolicySelector::new(
            Some(vec![UnitId::new("account.create")?]),
            None,
            None,
            None,
            None,
        )?,
        RequirementLevel::Recommended,
        vec![Facet::Behavior],
        TrustLevel::TrustedCi,
        DurationMillis::new(86_400_000)?,
        Some(PositiveCount::ONE),
    )?;
    Ok(Policy::new(
        PolicyId::new("policy.default")?,
        Revision::new(1)?,
        Title::new("Default policy")?,
        vec![owner("owner://team/security")?],
        profiles,
        vec![TargetId::new("web")?],
        vec![rule],
        WaiverPolicy::deny(),
        Some(Description::new("Baseline policy")?),
        Extensions::default(),
    )?)
}

#[test]
fn literal_policy_selection_bytes_and_digest_match_independent_oracle() -> Result<(), Box<dyn Error>>
{
    let profile = profile(1, "Audience")?;
    let selected = SelectedProfileInputV2::new(
        &profile,
        vec![
            (
                eqm_domain::DimensionId::new("locale")?,
                SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
            ),
            (
                eqm_domain::DimensionId::new("region")?,
                SelectedDimensionStateV2::Unknown,
            ),
        ],
    )?;
    let policy = policy(vec![profile.id().clone()])?;
    let input = PolicySelectionInputV2::new(&policy, vec![selected])?;
    let canonical = canonicalize_policy_selection_v2(&input)?;
    eprintln!("POLICY_BYTES={}", std::str::from_utf8(canonical.bytes())?);
    eprintln!("POLICY_DIGEST={}", canonical.digest());
    assert_eq!(std::str::from_utf8(canonical.bytes())?, POLICY_BYTES);
    assert_eq!(canonical.digest().to_string(), POLICY_DIGEST);
    Ok(())
}

#[test]
fn known_unknown_and_default_are_distinct() -> Result<(), Box<dyn Error>> {
    let profile = profile(1, "Audience")?;
    let selected = SelectedProfileInputV2::new(
        &profile,
        vec![
            (
                eqm_domain::DimensionId::new("locale")?,
                SelectedDimensionStateV2::Unknown,
            ),
            (
                eqm_domain::DimensionId::new("region")?,
                SelectedDimensionStateV2::Unknown,
            ),
        ],
    )?;
    let policy = policy(vec![profile.id().clone()])?;
    let input = PolicySelectionInputV2::new(&policy, vec![selected])?;
    let projected = project_policy_selection_v2(&input)?;
    let values = projected["profiles"][0]["values"]
        .as_array()
        .ok_or("values array")?;
    assert_eq!(values.len(), 2);
    for value in values {
        assert_eq!(value["state"], "unknown");
        assert!(value.get("value").is_none(), "unknown must carry no value");
    }
    // The definition still carries its declared default, but it is not applied.
    assert_eq!(
        projected["profiles"][0]["definition"]["defaults"]["locale"],
        "en"
    );
    Ok(())
}

#[test]
fn same_id_revision_definition_semantics_change_identity() -> Result<(), Box<dyn Error>> {
    let build = |title: &str| -> Result<_, Box<dyn Error>> {
        let profile = profile(1, title)?;
        let selected = SelectedProfileInputV2::new(
            &profile,
            vec![
                (
                    eqm_domain::DimensionId::new("locale")?,
                    SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
                ),
                (
                    eqm_domain::DimensionId::new("region")?,
                    SelectedDimensionStateV2::Unknown,
                ),
            ],
        )?;
        let policy = policy(vec![profile.id().clone()])?;
        let input = PolicySelectionInputV2::new(&policy, vec![selected])?;
        Ok(canonicalize_policy_selection_v2(&input)?)
    };
    assert_ne!(build("Audience")?.digest(), build("Audience v2")?.digest());
    Ok(())
}

#[test]
fn display_extensions_are_excluded_and_semantic_extensions_change_identity()
-> Result<(), Box<dyn Error>> {
    let build = |extensions: Extensions| -> Result<_, Box<dyn Error>> {
        let base = profile(1, "Audience")?;
        let profile = Profile::new(
            base.id().clone(),
            base.revision(),
            base.title().clone(),
            base.owners().iter().cloned().collect(),
            base.dimensions().values().cloned().collect(),
            base.defaults()
                .iter()
                .map(|(dimension, value)| (dimension.clone(), value.clone()))
                .collect(),
            base.description().cloned(),
            extensions,
        )?;
        let selected = SelectedProfileInputV2::new(
            &profile,
            vec![
                (
                    eqm_domain::DimensionId::new("locale")?,
                    SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
                ),
                (
                    eqm_domain::DimensionId::new("region")?,
                    SelectedDimensionStateV2::Unknown,
                ),
            ],
        )?;
        let policy = policy(vec![profile.id().clone()])?;
        let input = PolicySelectionInputV2::new(&policy, vec![selected])?;
        Ok(canonicalize_policy_selection_v2(&input)?)
    };
    let base = build(Extensions::default())?;
    let display = Extensions::new(std::collections::BTreeMap::from([(
        eqm_domain::ExtensionNamespace::new("dev.equivalencematrix.display")?,
        eqm_domain::ExtensionValue::String("hidden".into()),
    )]))?;
    assert_eq!(base, build(display)?);
    let semantic = Extensions::new(std::collections::BTreeMap::from([(
        eqm_domain::ExtensionNamespace::new("dev.equivalencematrix.extra")?,
        eqm_domain::ExtensionValue::Integer(i64::MIN),
    )]))?;
    assert_ne!(base.digest(), build(semantic)?.digest());
    Ok(())
}

#[test]
fn policy_selection_extension_integer_and_text_never_collide() -> Result<(), Box<dyn Error>> {
    let build = |value: eqm_domain::ExtensionValue| -> Result<_, Box<dyn Error>> {
        let base = profile(1, "Audience")?;
        let profile = Profile::new(
            base.id().clone(),
            base.revision(),
            base.title().clone(),
            base.owners().iter().cloned().collect(),
            base.dimensions().values().cloned().collect(),
            base.defaults()
                .iter()
                .map(|(dimension, value)| (dimension.clone(), value.clone()))
                .collect(),
            base.description().cloned(),
            Extensions::new(std::collections::BTreeMap::from([(
                eqm_domain::ExtensionNamespace::new("dev.equivalencematrix.extra")?,
                value,
            )]))?,
        )?;
        let selected = SelectedProfileInputV2::new(
            &profile,
            vec![
                (
                    eqm_domain::DimensionId::new("locale")?,
                    SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
                ),
                (
                    eqm_domain::DimensionId::new("region")?,
                    SelectedDimensionStateV2::Unknown,
                ),
            ],
        )?;
        let policy = policy(vec![profile.id().clone()])?;
        let input = PolicySelectionInputV2::new(&policy, vec![selected])?;
        Ok(canonicalize_policy_selection_v2(&input)?)
    };
    for (integer, text) in [
        (
            eqm_domain::ExtensionValue::Integer(i64::MIN),
            "-9223372036854775808",
        ),
        (
            eqm_domain::ExtensionValue::Integer(9_007_199_254_740_992),
            "9007199254740992",
        ),
    ] {
        let integer = build(integer)?;
        let string = build(eqm_domain::ExtensionValue::String(text.into()))?;
        assert_ne!(integer.bytes(), string.bytes());
        assert_ne!(integer.digest(), string.digest());
    }
    Ok(())
}
