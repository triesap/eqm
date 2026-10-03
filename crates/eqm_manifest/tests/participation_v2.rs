//! S035-R1: independent v2 participation projection and `ParticipationDigest`.
//!
//! Literal byte/digest oracles are reviewed constants computed independently from
//! the projection code. The controls prove the exact five-member root, canonical
//! target-set ordering, semantic-extension inclusion with display-only
//! exclusion, and the recursive extension type distinction.

use eqm_domain::{
    Applicability, ExtensionNamespace, ExtensionValue, Extensions, FullRequirementId,
    ParticipationRule, ParticipationRuleId, TargetId,
};
use eqm_manifest::{canonicalize_participation_v2, project_participation_v2};
use std::collections::BTreeMap;
use std::error::Error;

const PARTICIPATION_BYTES: &str = r#"{"applicability":{"always":true},"extensions":{},"id":"participation.mobile","requirement":"capture.photo#shutter","targets":["android","ios"]}"#;
const PARTICIPATION_DIGEST: &str =
    "sha256:8586c60c0b30692fa6bbc8e125f02dd9527957bef960aafd5b9e6de2283dc4e0";

fn rule(
    id: &str,
    requirement: &str,
    targets: &[&str],
    applicability: Applicability,
) -> Result<ParticipationRule, Box<dyn Error>> {
    Ok(ParticipationRule::new(
        ParticipationRuleId::new(id)?,
        FullRequirementId::new(requirement)?,
        targets
            .iter()
            .map(|target| TargetId::new(*target))
            .collect::<Result<Vec<_>, _>>()?,
        applicability,
        Extensions::default(),
    )?)
}

fn base() -> Result<ParticipationRule, Box<dyn Error>> {
    rule(
        "participation.mobile",
        "capture.photo#shutter",
        &["ios", "android"],
        Applicability::always(true),
    )
}

#[test]
fn literal_participation_bytes_and_digest_match_independent_oracle() -> Result<(), Box<dyn Error>> {
    let canonical = canonicalize_participation_v2(&base()?)?;
    assert_eq!(std::str::from_utf8(canonical.bytes())?, PARTICIPATION_BYTES);
    assert_eq!(canonical.digest().to_string(), PARTICIPATION_DIGEST);
    assert!(!canonical.bytes().ends_with(b"\n"));
    Ok(())
}

#[test]
fn projection_root_has_exactly_the_allocated_members() -> Result<(), Box<dyn Error>> {
    let projected = project_participation_v2(&base()?)?;
    let object = projected.as_object().ok_or("projected")?;
    let keys: Vec<&str> = object.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        vec![
            "applicability",
            "extensions",
            "id",
            "requirement",
            "targets"
        ]
    );
    Ok(())
}

#[test]
fn target_set_order_is_canonical_and_content_sensitive() -> Result<(), Box<dyn Error>> {
    let forward = canonicalize_participation_v2(&rule(
        "participation.mobile",
        "capture.photo#shutter",
        &["android", "ios"],
        Applicability::always(true),
    )?)?;
    let reversed = canonicalize_participation_v2(&base()?)?;
    assert_eq!(forward, reversed);
    let projected = project_participation_v2(&base()?)?;
    assert_eq!(projected["targets"], serde_json::json!(["android", "ios"]));
    let changed = canonicalize_participation_v2(&rule(
        "participation.mobile",
        "capture.photo#shutter",
        &["android"],
        Applicability::always(true),
    )?)?;
    assert_ne!(forward.digest(), changed.digest());
    Ok(())
}

#[test]
fn one_axis_id_requirement_and_applicability_mutations_move_identity() -> Result<(), Box<dyn Error>>
{
    let base = canonicalize_participation_v2(&base()?)?;
    for changed in [
        rule(
            "participation.other",
            "capture.photo#shutter",
            &["android", "ios"],
            Applicability::always(true),
        )?,
        rule(
            "participation.mobile",
            "capture.photo#other",
            &["android", "ios"],
            Applicability::always(true),
        )?,
        rule(
            "participation.mobile",
            "capture.photo#shutter",
            &["android", "ios"],
            Applicability::always(false),
        )?,
    ] {
        assert_ne!(
            base.digest(),
            canonicalize_participation_v2(&changed)?.digest()
        );
    }
    Ok(())
}

#[test]
fn extension_integer_and_text_never_collide_and_display_is_excluded() -> Result<(), Box<dyn Error>>
{
    let with_extension = |value: ExtensionValue| -> Result<_, Box<dyn Error>> {
        Ok(canonicalize_participation_v2(&ParticipationRule::new(
            ParticipationRuleId::new("participation.mobile")?,
            FullRequirementId::new("capture.photo#shutter")?,
            vec![TargetId::new("android")?, TargetId::new("ios")?],
            Applicability::always(true),
            Extensions::new(BTreeMap::from([(
                ExtensionNamespace::new("dev.equivalencematrix.extra")?,
                value,
            )]))?,
        )?)?)
    };
    let integer = with_extension(ExtensionValue::Integer(9_007_199_254_740_992))?;
    let text = with_extension(ExtensionValue::String("9007199254740992".into()))?;
    assert_ne!(integer.bytes(), text.bytes());

    let display = canonicalize_participation_v2(&ParticipationRule::new(
        ParticipationRuleId::new("participation.mobile")?,
        FullRequirementId::new("capture.photo#shutter")?,
        vec![TargetId::new("android")?, TargetId::new("ios")?],
        Applicability::always(true),
        Extensions::new(BTreeMap::from([(
            ExtensionNamespace::new("dev.equivalencematrix.display")?,
            ExtensionValue::String("hidden".into()),
        )]))?,
    )?)?;
    assert_eq!(display, canonicalize_participation_v2(&base()?)?);
    Ok(())
}
