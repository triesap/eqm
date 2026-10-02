//! S019-C: v2 selected-policy/profile projection and `PolicySelectionDigest`.
//!
//! Literal byte/digest oracles are reviewed constants computed independently from
//! the projection code. Controls prove known/unknown state, same-ID/revision
//! definition binding, display-only exclusion, canonical sets, and the recursive
//! extension type distinction.

use eqm_domain::{
    Description, DimensionId, DurationMillis, Extensions, Facet, FullRequirementId, OwnerRef,
    Policy, PolicyId, PolicyRule, PolicySelectionInputV2, PolicySelector, PositiveCount,
    PositiveDays, Profile, ProfileDimension, ProfileId, RequirementLevel, RequirementScope,
    Revision, RiskClass, SelectedDimensionStateV2, SelectedProfileInputV2, SymbolicValueId,
    TargetId, Title, TrustLevel, UnitId, WaiverPolicy,
};
use eqm_manifest::{canonicalize_policy_selection_v2, project_policy_selection_v2};
use std::error::Error;

const POLICY_BYTES: &str = r#"{"policy":{"description":"Baseline policy","extensions":{},"id":"policy.default","owners":["owner://team/security"],"profiles":["audience.default"],"required_targets":["web"],"revision":1,"rules":[{"facets":["behavior"],"maximum_age":86400000,"minimum_count":1,"minimum_level":"recommended","minimum_trust":"trusted_ci","selector":{"units":["account.create"]}}],"title":"Default policy","waivers":{"allowed":false,"minimum_approvers":1,"required_controls":[]}},"profiles":[{"definition":{"defaults":{"locale":"en"},"description":"Default audience","dimensions":[{"id":"locale","values":["en","fr"]},{"id":"region","values":["eu","us"]}],"extensions":{},"id":"audience.default","owners":["owner://team/product"],"revision":1,"title":"Audience"},"values":[{"dimension":"locale","state":"known","value":"en"},{"dimension":"region","state":"unknown"}]}]}"#;
const POLICY_DIGEST: &str =
    "sha256:0ba1fc91cb154a5b5b6d57d80e379c55f6bf0588d6cd2dfbaefcab2611fde60f";

// Reviewed literal complete bytes for the heterogeneous ordering trap used by
// `rule_order_uses_full_member_bytes`: a maximum-valued rule (all five selector
// axes, decimal-string `maximum_age`/`minimum_count`) followed by a
// minimum-valued rule (narrow selector, numeric `maximum_age`/`minimum_count`).
// The digest was independently recomputed as
// `sha256("eqm:v2:policy-selection" || 0x00 || bytes)` from these exact bytes.
const RULE_ORDER_TRAP_BYTES: &str = r#"{"policy":{"extensions":{},"id":"policy.default","owners":["owner://team/security"],"profiles":["audience.default"],"required_targets":["web"],"revision":1,"rules":[{"facets":["behavior"],"maximum_age":"9223372036854775807","minimum_count":"18446744073709551615","minimum_level":"recommended","minimum_trust":"trusted_ci","selector":{"facets":["behavior"],"requirements":["account.create.signup#visible"],"risk_classes":["high"],"scopes":["each_target"],"units":["account.create"]}},{"facets":["behavior"],"maximum_age":0,"minimum_count":1,"minimum_level":"recommended","minimum_trust":"trusted_ci","selector":{"units":["account.create"]}}],"title":"Default policy","waivers":{"allowed":false,"minimum_approvers":1,"required_controls":[]}},"profiles":[{"definition":{"defaults":{"locale":"en"},"description":"Default audience","dimensions":[{"id":"locale","values":["en","fr"]},{"id":"region","values":["eu","us"]}],"extensions":{},"id":"audience.default","owners":["owner://team/product"],"revision":1,"title":"Audience"},"values":[{"dimension":"locale","state":"known","value":"en"},{"dimension":"region","state":"unknown"}]}]}"#;
const RULE_ORDER_TRAP_DIGEST: &str =
    "sha256:99030afca7ed3a3ae1752fc4350a6752cc6bf6f43feb3c1cb37d9a03c9e6f8fc";

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

fn selected(profile: &Profile) -> Result<SelectedProfileInputV2<'_>, Box<dyn Error>> {
    Ok(SelectedProfileInputV2::new(
        profile,
        vec![
            (
                DimensionId::new("locale")?,
                SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
            ),
            (
                DimensionId::new("region")?,
                SelectedDimensionStateV2::Unknown,
            ),
        ],
    )?)
}

#[test]
fn policy_and_profile_revisions_use_literal_v2_wire_forms() -> Result<(), Box<dyn Error>> {
    // Reviewed literal (input revision, expected v2 JSON token) table. The value
    // at the IEEE-754 safe integer stays an exact integer; the two above it are
    // decimal strings.
    let cases: [(u64, serde_json::Value); 3] = [
        (
            9_007_199_254_740_991,
            serde_json::json!(9_007_199_254_740_991_u64),
        ),
        (9_007_199_254_740_992, serde_json::json!("9007199254740992")),
        (u64::MAX, serde_json::json!("18446744073709551615")),
    ];
    for (revision, expected) in cases {
        let profile = profile(revision, "Audience")?;
        let policy = Policy::new(
            PolicyId::new("policy.default")?,
            Revision::new(revision)?,
            Title::new("Default policy")?,
            vec![owner("owner://team/security")?],
            vec![profile.id().clone()],
            vec![TargetId::new("web")?],
            vec![PolicyRule::new(
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
                None,
            )?],
            WaiverPolicy::deny(),
            None,
            Extensions::default(),
        )?;
        let input = PolicySelectionInputV2::new(&policy, vec![selected(&profile)?])?;
        let projected = project_policy_selection_v2(&input)?;
        assert_eq!(projected["policy"]["revision"], expected);
        assert_eq!(projected["profiles"][0]["definition"]["revision"], expected);
    }
    Ok(())
}

#[test]
fn policy_rule_endpoints_use_literal_v2_wire_forms() -> Result<(), Box<dyn Error>> {
    let profile = profile(1, "Audience")?;
    // Project actual minimum and legal maximum rules through the production
    // projection, then assert reviewed literal expected JSON tokens for both the
    // age and count axes rather than recomputing them with a helper.
    let project = |maximum_age: u64, minimum_count: u64| -> Result<_, Box<dyn Error>> {
        let policy = Policy::new(
            PolicyId::new("policy.default")?,
            Revision::new(1)?,
            Title::new("Default policy")?,
            vec![owner("owner://team/security")?],
            vec![profile.id().clone()],
            vec![TargetId::new("web")?],
            vec![PolicyRule::new(
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
                DurationMillis::new(maximum_age)?,
                Some(PositiveCount::new(minimum_count)?),
            )?],
            WaiverPolicy::new(
                true,
                Some(PositiveDays::new(30)?),
                Some(PositiveCount::new(2)?),
                vec![Facet::Behavior],
            )?,
            None,
            Extensions::default(),
        )?;
        let input = PolicySelectionInputV2::new(&policy, vec![selected(&profile)?])?;
        Ok(project_policy_selection_v2(&input)?)
    };

    // Minimum legal endpoints: age `0` and count `1` are exact integers.
    let minimum = project(0, 1)?;
    let minimum_rule = &minimum["policy"]["rules"][0];
    assert_eq!(minimum_rule["maximum_age"], serde_json::json!(0));
    assert_eq!(minimum_rule["minimum_count"], serde_json::json!(1));

    // Maximum legal endpoints: both exceed the IEEE-754 safe integer and are
    // emitted as decimal strings.
    let maximum = project(DurationMillis::MAX, u64::MAX)?;
    let maximum_rule = &maximum["policy"]["rules"][0];
    assert_eq!(
        maximum_rule["maximum_age"],
        serde_json::json!("9223372036854775807")
    );
    assert_eq!(
        maximum_rule["minimum_count"],
        serde_json::json!("18446744073709551615")
    );
    assert_eq!(
        maximum["policy"]["waivers"]["maximum_days"],
        serde_json::json!(30)
    );
    Ok(())
}

fn full_selector() -> Result<PolicySelector, Box<dyn Error>> {
    Ok(PolicySelector::new(
        Some(vec![UnitId::new("account.create")?]),
        Some(vec![FullRequirementId::new(
            "account.create.signup#visible",
        )?]),
        Some(vec![RiskClass::High]),
        Some(vec![Facet::Behavior]),
        Some(vec![RequirementScope::EachTarget]),
    )?)
}

#[test]
fn populated_selector_axes_and_waiver_fields_are_fully_inventoried() -> Result<(), Box<dyn Error>> {
    let profile = profile(1, "Audience")?;
    let policy = Policy::new(
        PolicyId::new("policy.default")?,
        Revision::new(1)?,
        Title::new("Default policy")?,
        vec![owner("owner://team/security")?],
        vec![profile.id().clone()],
        vec![TargetId::new("web")?],
        vec![PolicyRule::new(
            full_selector()?,
            RequirementLevel::Recommended,
            vec![Facet::Behavior],
            TrustLevel::TrustedCi,
            DurationMillis::new(86_400_000)?,
            None,
        )?],
        WaiverPolicy::new(
            true,
            Some(PositiveDays::new(30)?),
            Some(PositiveCount::new(2)?),
            vec![Facet::Behavior],
        )?,
        None,
        Extensions::default(),
    )?;
    let input = PolicySelectionInputV2::new(&policy, vec![selected(&profile)?])?;
    let projected = project_policy_selection_v2(&input)?;
    assert_eq!(
        projected["policy"]["rules"][0]["selector"],
        serde_json::json!({
            "units": ["account.create"],
            "requirements": ["account.create.signup#visible"],
            "risk_classes": ["high"],
            "facets": ["behavior"],
            "scopes": ["each_target"]
        })
    );
    assert_eq!(
        projected["policy"]["waivers"],
        serde_json::json!({
            "allowed": true,
            "minimum_approvers": 2,
            "required_controls": ["behavior"],
            "maximum_days": 30
        })
    );

    // A narrow selector and denied waivers keep the unpopulated axes absent.
    let narrow = Policy::new(
        PolicyId::new("policy.default")?,
        Revision::new(1)?,
        Title::new("Default policy")?,
        vec![owner("owner://team/security")?],
        vec![profile.id().clone()],
        vec![TargetId::new("web")?],
        vec![PolicyRule::new(
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
            DurationMillis::new(1)?,
            None,
        )?],
        WaiverPolicy::deny(),
        None,
        Extensions::default(),
    )?;
    let narrow_input = PolicySelectionInputV2::new(&narrow, vec![selected(&profile)?])?;
    let projected = project_policy_selection_v2(&narrow_input)?;
    let selector = projected["policy"]["rules"][0]["selector"]
        .as_object()
        .ok_or("selector object")?;
    assert_eq!(selector.keys().collect::<Vec<_>>(), vec!["units"]);
    assert!(
        projected["policy"]["waivers"]
            .as_object()
            .ok_or("waivers object")?
            .get("maximum_days")
            .is_none()
    );
    Ok(())
}

#[test]
fn populated_dimension_description_is_present_and_absent_exactly() -> Result<(), Box<dyn Error>> {
    // Same-ingress builder so the populated and absent controls differ only in the
    // optional dimension description.
    let build = |description: Option<Description>| -> Result<_, Box<dyn Error>> {
        let profile = Profile::new(
            ProfileId::new("audience.default")?,
            Revision::new(1)?,
            Title::new("Audience")?,
            vec![owner("owner://team/product")?],
            vec![ProfileDimension::new(
                DimensionId::new("region")?,
                vec![SymbolicValueId::new("eu")?, SymbolicValueId::new("us")?],
                description,
            )?],
            vec![(DimensionId::new("region")?, SymbolicValueId::new("us")?)],
            None,
            Extensions::default(),
        )?;
        let selected_policy = policy(vec![profile.id().clone()])?;
        let input = PolicySelectionInputV2::new(
            &selected_policy,
            vec![SelectedProfileInputV2::new(
                &profile,
                vec![(
                    DimensionId::new("region")?,
                    SelectedDimensionStateV2::Known(SymbolicValueId::new("us")?),
                )],
            )?],
        )?;
        Ok(project_policy_selection_v2(&input)?)
    };

    let present = build(Some(Description::new("Deployment region")?))?;
    let dimension = &present["profiles"][0]["definition"]["dimensions"][0];
    assert_eq!(dimension["description"], "Deployment region");

    // A same-ingress absent description is omitted, not serialized as null.
    let absent = build(None)?;
    let dimension = absent["profiles"][0]["definition"]["dimensions"][0]
        .as_object()
        .ok_or("dimension object")?;
    assert!(
        !dimension.contains_key("description"),
        "an absent dimension description must be omitted, not null"
    );
    Ok(())
}

fn profile_with(id: &str, description: &str) -> Result<Profile, Box<dyn Error>> {
    Ok(Profile::new(
        ProfileId::new(id)?,
        Revision::new(1)?,
        Title::new("Audience")?,
        vec![owner("owner://team/product")?],
        vec![
            ProfileDimension::new(
                DimensionId::new("locale")?,
                vec![SymbolicValueId::new("en")?],
                None,
            )?,
            ProfileDimension::new(
                DimensionId::new("region")?,
                vec![SymbolicValueId::new("eu")?, SymbolicValueId::new("us")?],
                None,
            )?,
        ],
        vec![(DimensionId::new("locale")?, SymbolicValueId::new("en")?)],
        Some(Description::new(description)?),
        Extensions::default(),
    )?)
}

fn profile_values(profile: &Profile) -> Result<SelectedProfileInputV2<'_>, Box<dyn Error>> {
    Ok(SelectedProfileInputV2::new(
        profile,
        vec![
            (
                DimensionId::new("locale")?,
                SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
            ),
            (
                DimensionId::new("region")?,
                SelectedDimensionStateV2::Unknown,
            ),
        ],
    )?)
}

#[test]
fn profile_set_order_uses_full_member_bytes_not_ids() -> Result<(), Box<dyn Error>> {
    // `audience.beta` has description "aaa" and `audience.alpha` has "zzz", so
    // full member-byte order (beta, alpha) differs from the ID order (alpha, beta).
    let alpha = profile_with("audience.alpha", "zzz")?;
    let beta = profile_with("audience.beta", "aaa")?;
    let build = |first: &Profile, second: &Profile| -> Result<_, Box<dyn Error>> {
        let policy = policy(vec![alpha.id().clone(), beta.id().clone()])?;
        let input = PolicySelectionInputV2::new(
            &policy,
            vec![profile_values(first)?, profile_values(second)?],
        )?;
        Ok(canonicalize_policy_selection_v2(&input)?)
    };
    let forward = build(&alpha, &beta)?;
    let reversed = build(&beta, &alpha)?;
    assert_eq!(forward.bytes(), reversed.bytes());
    assert_eq!(forward.digest(), reversed.digest());
    let projected = project_policy_selection_v2(&PolicySelectionInputV2::new(
        &policy(vec![alpha.id().clone(), beta.id().clone()])?,
        vec![profile_values(&alpha)?, profile_values(&beta)?],
    )?)?;
    assert_eq!(
        projected["profiles"][0]["definition"]["id"], "audience.beta",
        "profiles must be ordered by complete member bytes"
    );
    assert_eq!(
        projected["profiles"][1]["definition"]["id"],
        "audience.alpha"
    );
    Ok(())
}

#[test]
fn rule_order_uses_full_member_bytes() -> Result<(), Box<dyn Error>> {
    // Heterogeneous two-rule trap. The rules differ both in optional selector
    // presence and in the `maximum_age`/`minimum_count` wire forms. `maximum_age`
    // is the first differing member byte after the shared `facets`, and the
    // maximum rule's decimal string `"9223…"` sorts before the minimum rule's
    // numeric `0` (`0x22 < 0x30`). Full JCS member-byte order (maximum first)
    // therefore differs from numeric domain order (minimum first).
    let maximum_rule = PolicyRule::new(
        full_selector()?,
        RequirementLevel::Recommended,
        vec![Facet::Behavior],
        TrustLevel::TrustedCi,
        DurationMillis::new(DurationMillis::MAX)?,
        Some(PositiveCount::new(u64::MAX)?),
    )?;
    let minimum_rule = PolicyRule::new(
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
        DurationMillis::new(0)?,
        Some(PositiveCount::new(1)?),
    )?;
    let profile = profile(1, "Audience")?;
    let build_policy = |first: PolicyRule, second: PolicyRule| -> Result<_, Box<dyn Error>> {
        Ok(Policy::new(
            PolicyId::new("policy.default")?,
            Revision::new(1)?,
            Title::new("Default policy")?,
            vec![owner("owner://team/security")?],
            vec![profile.id().clone()],
            vec![TargetId::new("web")?],
            vec![first, second],
            WaiverPolicy::deny(),
            None,
            Extensions::default(),
        )?)
    };
    let forward = build_policy(maximum_rule.clone(), minimum_rule.clone())?;
    let reversed = build_policy(minimum_rule, maximum_rule)?;
    let forward_input = PolicySelectionInputV2::new(&forward, vec![selected(&profile)?])?;
    let reversed_input = PolicySelectionInputV2::new(&reversed, vec![selected(&profile)?])?;
    let forward_bytes = canonicalize_policy_selection_v2(&forward_input)?;
    let reversed_bytes = canonicalize_policy_selection_v2(&reversed_input)?;
    assert_eq!(forward_bytes.bytes(), reversed_bytes.bytes());
    assert_eq!(forward_bytes.digest(), reversed_bytes.digest());

    // Reviewed literal complete bytes and independently recomputed digest. This
    // freezes the exact ordering trap identity rather than only comparing two
    // production outputs to each other.
    assert_eq!(
        std::str::from_utf8(forward_bytes.bytes())?,
        RULE_ORDER_TRAP_BYTES
    );
    assert_eq!(forward_bytes.digest().to_string(), RULE_ORDER_TRAP_DIGEST);

    // Reviewed literal member order: the maximum-valued rule precedes the
    // minimum-valued rule under full JCS member-byte ordering.
    let projected = project_policy_selection_v2(&forward_input)?;
    let rules = projected["policy"]["rules"]
        .as_array()
        .ok_or("rules array")?;
    assert_eq!(rules.len(), 2);
    assert_eq!(
        rules[0]["maximum_age"],
        serde_json::json!("9223372036854775807")
    );
    assert_eq!(
        rules[0]["minimum_count"],
        serde_json::json!("18446744073709551615")
    );
    assert_eq!(
        rules[0]["selector"].as_object().map(|item| item.len()),
        Some(5)
    );
    assert_eq!(rules[1]["maximum_age"], serde_json::json!(0));
    assert_eq!(rules[1]["minimum_count"], serde_json::json!(1));
    assert_eq!(
        rules[1]["selector"].as_object().map(|item| item.len()),
        Some(1)
    );
    Ok(())
}
