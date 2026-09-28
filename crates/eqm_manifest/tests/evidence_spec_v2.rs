//! S018: canonical v2 evidence-specification projection and identity.
//!
//! The literal byte/digest constants are reviewed oracles computed outside the
//! projection code. Tests cover the DEV-005 requirements: selector/kind/count
//! changes with the same specification ID alter identity, the projection is
//! repeatable, and the representation rules match the binding projection.

use eqm_domain::{
    DurationMillis, EvidenceKind, EvidenceSelector, EvidenceSpecId, EvidenceSpecification,
    EvidenceSpecificationDigest, ExtensionKey, ExtensionNamespace, ExtensionValue, Extensions,
    Facet, FullRequirementId, PositiveCount, RunnerId, SelectorText,
};
use eqm_manifest::{canonicalize_evidence_spec_v2, project_evidence_spec_v2};
use std::collections::BTreeMap;
use std::error::Error;

const SPEC_BYTES: &str = r#"{"extensions":{},"facets":["behavior"],"freshness":60000,"id":"signup_test","kind":"test","minimum_count":1,"requirements":["account.create.signup.start#visible"],"runner":"runner.cargo","selector":{"framework":"cargo","kind":"test","test_id":"signup"}}"#;

const SPEC_DIGEST: &str = "sha256:68e79e00cc86a9f0c5d5ad1809d790ca52e3a228a6f1d990b2f1838eda1c2195";

const BOUNDARY_BYTES: &str = r#"{"extensions":{"dev.equivalencematrix.extra":{"type":"object","value":{"boundary":{"type":"integer","value":"-9223372036854775808"}}}},"facets":["behavior"],"freshness":"9007199254740992","id":"signup_test","kind":"test","minimum_count":"9007199254740992","requirements":["account.create.signup.start#visible"],"runner":"runner.cargo","selector":{"framework":"cargo","kind":"test","test_id":"signup"}}"#;

const BOUNDARY_DIGEST: &str =
    "sha256:5e475b37a6fecf4ee5ecb174b43b80a01cf14cbe66b6f3272dcbce5d7bebdb9c";

const MAX_SAFE_INTEGER_OVER: u64 = 9_007_199_254_740_992;

fn test_selector() -> Result<EvidenceSelector, Box<dyn Error>> {
    Ok(EvidenceSelector::Test {
        framework: SelectorText::new("cargo")?,
        test_id: SelectorText::new("signup")?,
        suite: None,
    })
}

#[allow(clippy::too_many_arguments)]
fn spec(
    id: &str,
    kind: EvidenceKind,
    selector: Option<EvidenceSelector>,
    runner: Option<&str>,
    minimum_count: Option<u64>,
    freshness: Option<u64>,
    extensions: Extensions,
) -> Result<EvidenceSpecification, Box<dyn Error>> {
    Ok(EvidenceSpecification::new(
        EvidenceSpecId::new(id)?,
        kind,
        vec![FullRequirementId::new(
            "account.create.signup.start#visible",
        )?],
        vec![Facet::Behavior],
        runner.map(RunnerId::new).transpose()?,
        selector,
        minimum_count.map(PositiveCount::new).transpose()?,
        freshness.map(DurationMillis::new).transpose()?,
        extensions,
    )?)
}

fn base_spec() -> Result<EvidenceSpecification, Box<dyn Error>> {
    spec(
        "signup_test",
        EvidenceKind::Test,
        Some(test_selector()?),
        Some("runner.cargo"),
        Some(1),
        Some(60_000),
        Extensions::default(),
    )
}

fn extra_extensions(value: &str) -> Result<Extensions, Box<dyn Error>> {
    Ok(Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.extra")?,
        ExtensionValue::Object(BTreeMap::from([(
            ExtensionKey::new("marker")?,
            ExtensionValue::String(value.into()),
        )])),
    )]))?)
}

#[test]
fn literal_projection_bytes_and_digest_match_independent_oracle() -> Result<(), Box<dyn Error>> {
    let canonical = canonicalize_evidence_spec_v2(&base_spec()?)?;
    assert_eq!(std::str::from_utf8(canonical.bytes())?, SPEC_BYTES);
    assert_eq!(canonical.digest().to_string(), SPEC_DIGEST);
    assert!(!canonical.bytes().ends_with(b"\n"));
    Ok(())
}

#[test]
fn integer_boundaries_use_v2_wire_forms_with_literal_oracle() -> Result<(), Box<dyn Error>> {
    let extensions = Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.extra")?,
        ExtensionValue::Object(BTreeMap::from([(
            ExtensionKey::new("boundary")?,
            ExtensionValue::Integer(i64::MIN),
        )])),
    )]))?;
    let boundary = spec(
        "signup_test",
        EvidenceKind::Test,
        Some(test_selector()?),
        Some("runner.cargo"),
        Some(9_007_199_254_740_992),
        Some(9_007_199_254_740_992),
        extensions,
    )?;
    let canonical = canonicalize_evidence_spec_v2(&boundary)?;
    assert_eq!(std::str::from_utf8(canonical.bytes())?, BOUNDARY_BYTES);
    assert_eq!(canonical.digest().to_string(), BOUNDARY_DIGEST);
    Ok(())
}

#[test]
fn same_id_kind_selector_count_and_runner_changes_alter_identity() -> Result<(), Box<dyn Error>> {
    let base = canonicalize_evidence_spec_v2(&base_spec()?)?.digest();

    let selector_changed = spec(
        "signup_test",
        EvidenceKind::Test,
        Some(EvidenceSelector::Test {
            framework: SelectorText::new("cargo")?,
            test_id: SelectorText::new("login")?,
            suite: None,
        }),
        Some("runner.cargo"),
        Some(1),
        Some(60_000),
        Extensions::default(),
    )?;
    assert_ne!(
        base,
        canonicalize_evidence_spec_v2(&selector_changed)?.digest(),
        "a same-ID selector change must alter identity"
    );

    let kind_changed = spec(
        "signup_test",
        EvidenceKind::ManualReview,
        None,
        None,
        None,
        None,
        Extensions::default(),
    )?;
    assert_ne!(
        base,
        canonicalize_evidence_spec_v2(&kind_changed)?.digest(),
        "a same-ID kind change must alter identity"
    );

    for changed in [
        spec(
            "signup_test",
            EvidenceKind::Test,
            Some(test_selector()?),
            Some("runner.other"),
            Some(1),
            Some(60_000),
            Extensions::default(),
        )?,
        spec(
            "signup_test",
            EvidenceKind::Test,
            Some(test_selector()?),
            Some("runner.cargo"),
            Some(2),
            Some(60_000),
            Extensions::default(),
        )?,
        spec(
            "signup_test",
            EvidenceKind::Test,
            Some(test_selector()?),
            Some("runner.cargo"),
            Some(1),
            Some(120_000),
            Extensions::default(),
        )?,
        spec(
            "signup_test",
            EvidenceKind::Test,
            Some(test_selector()?),
            Some("runner.cargo"),
            Some(1),
            Some(60_000),
            extra_extensions("marker")?,
        )?,
    ] {
        assert_ne!(
            base,
            canonicalize_evidence_spec_v2(&changed)?.digest(),
            "a runner/count/freshness/extension change must alter identity"
        );
    }
    Ok(())
}

#[test]
fn display_extensions_stay_stable_and_projection_is_repeatable() -> Result<(), Box<dyn Error>> {
    let base = canonicalize_evidence_spec_v2(&base_spec()?)?;
    let display = Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.display")?,
        ExtensionValue::String("Signup".into()),
    )]))?;
    let display_spec = spec(
        "signup_test",
        EvidenceKind::Test,
        Some(test_selector()?),
        Some("runner.cargo"),
        Some(1),
        Some(60_000),
        display,
    )?;
    assert_eq!(
        base,
        canonicalize_evidence_spec_v2(&display_spec)?,
        "the display-only namespace must not alter evidence-specification identity"
    );

    // Requirements are a canonical set: input order does not change the bytes.
    let permuted = EvidenceSpecification::new(
        EvidenceSpecId::new("signup_test")?,
        EvidenceKind::Test,
        vec![
            FullRequirementId::new("account.create.signup.start#visible")?,
            FullRequirementId::new("account.create.signup.start#reachable")?,
        ],
        vec![Facet::Behavior],
        Some(RunnerId::new("runner.cargo")?),
        Some(test_selector()?),
        Some(PositiveCount::new(1)?),
        Some(DurationMillis::new(60_000)?),
        Extensions::default(),
    )?;
    let first = project_evidence_spec_v2(&permuted)?;
    assert_eq!(
        first["requirements"],
        serde_json::json!([
            "account.create.signup.start#reachable",
            "account.create.signup.start#visible"
        ])
    );
    assert_eq!(
        canonicalize_evidence_spec_v2(&permuted)?,
        canonicalize_evidence_spec_v2(&permuted)?,
        "stable semantic projections must be repeatable"
    );
    Ok(())
}

#[test]
fn evidence_spec_identity_is_the_typed_domain_wrapper() -> Result<(), Box<dyn Error>> {
    let canonical = canonicalize_evidence_spec_v2(&base_spec()?)?;
    let typed: EvidenceSpecificationDigest = canonical.digest();
    assert_eq!(
        typed,
        EvidenceSpecificationDigest::hash_content(canonical.bytes())
    );
    assert_eq!(typed.as_digest().to_string(), SPEC_DIGEST);
    Ok(())
}

fn spec_with_extension(value: ExtensionValue) -> Result<EvidenceSpecification, Box<dyn Error>> {
    let extensions = Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.extra")?,
        ExtensionValue::Object(BTreeMap::from([(ExtensionKey::new("same_id")?, value)])),
    )]))?;
    spec(
        "signup_test",
        EvidenceKind::Test,
        Some(test_selector()?),
        Some("runner.cargo"),
        Some(1),
        Some(60_000),
        extensions,
    )
}

#[test]
fn evidence_spec_integer_and_text_with_same_digits_never_collide() -> Result<(), Box<dyn Error>> {
    for (integer, text) in [
        (ExtensionValue::Integer(i64::MIN), "-9223372036854775808"),
        (
            ExtensionValue::Integer(MAX_SAFE_INTEGER_OVER as i64),
            "9007199254740992",
        ),
    ] {
        let integer = canonicalize_evidence_spec_v2(&spec_with_extension(integer)?)?;
        let string = canonicalize_evidence_spec_v2(&spec_with_extension(ExtensionValue::String(
            text.into(),
        ))?)?;
        assert_ne!(integer.bytes(), string.bytes());
        assert_ne!(integer.digest(), string.digest());
    }

    let nested_integer = ExtensionValue::Object(BTreeMap::from([(
        ExtensionKey::new("nested")?,
        ExtensionValue::Array(vec![ExtensionValue::Integer(i64::MIN)]),
    )]));
    let nested_text = ExtensionValue::Object(BTreeMap::from([(
        ExtensionKey::new("nested")?,
        ExtensionValue::Array(vec![ExtensionValue::String("-9223372036854775808".into())]),
    )]));
    assert_ne!(
        canonicalize_evidence_spec_v2(&spec_with_extension(nested_integer)?)?.bytes(),
        canonicalize_evidence_spec_v2(&spec_with_extension(nested_text)?)?.bytes()
    );

    let tag_like = ExtensionValue::Object(BTreeMap::from([
        (
            ExtensionKey::new("type")?,
            ExtensionValue::String("integer".into()),
        ),
        (
            ExtensionKey::new("value")?,
            ExtensionValue::String("-9223372036854775808".into()),
        ),
    ]));
    assert_ne!(
        canonicalize_evidence_spec_v2(&spec_with_extension(tag_like)?)?.bytes(),
        canonicalize_evidence_spec_v2(&spec_with_extension(ExtensionValue::Integer(i64::MIN))?)?
            .bytes()
    );
    Ok(())
}
