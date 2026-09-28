//! S017: complete v2 binding projection and `BindingDigest` identity.
//!
//! The independent literal byte/digest constants are reviewed oracles computed
//! outside the projection code. The tests cover the DEV-005 requirements:
//! same-ID selector/path/constraint/semantic-extension changes alter identity,
//! display-extension and parse-provenance changes stay stable, set members are
//! ordered by complete canonical bytes, ordered extension arrays keep order, and
//! integer boundaries use the v2 wire forms.

use eqm_domain::{
    Applicability, Artifact, ArtifactId, ArtifactRole, ArtifactSelector, Artifacts, Binding,
    BindingId, ComparisonOperator, DimensionId, DurationMillis, EvidenceKind, EvidenceSelector,
    EvidenceSpecId, EvidenceSpecification, Exposure, ExtensionKey, ExtensionNamespace,
    ExtensionValue, Extensions, Facet, FullRequirementId, HttpMethod, IntendedExposureState,
    MembershipOperator, OwnerRef, PositiveCount, RepoPath, Revision, RunnerId, SelectorText,
    SurfaceId, SymbolicValueId, TargetId, UnitId,
};
use eqm_manifest::{canonicalize_binding_v2, project_binding_v2};
use std::collections::BTreeMap;
use std::error::Error;
use std::str::FromStr;

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
const MAX_SAFE_INTEGER_OVER: u64 = MAX_SAFE_INTEGER + 1;

const BASE_BYTES: &str = r#"{"artifacts":[{"extensions":{},"id":"signup","path":"src/signup.rs","role":"route","selector":{"kind":"route","method":"get","path":"/signup"},"surface":"account.create.signup.start","symbol":"signup_handler"}],"evidence":[{"extensions":{},"facets":["behavior"],"freshness":60000,"id":"signup_test","kind":"test","minimum_count":1,"requirements":["account.create.signup.start#visible"],"runner":"runner.cargo","selector":{"framework":"cargo","kind":"test","test_id":"signup"}}],"exposures":[{"applicability":{"always":true},"extensions":{},"state":"required","surface":"account.create.signup.start"}],"extensions":{},"id":"web.account","owners":["owner://team/web"],"revision":1,"target":"web","unit":"account.create"}"#;

const BASE_DIGEST: &str = "sha256:b8154689fb2253814a3184f5e19436c6aaddfc4ecae0618a5c5a44b5a74f8c87";

const BOUNDARY_BYTES: &str = r#"{"artifacts":[{"extensions":{},"id":"signup","path":"src/signup.rs","role":"route","selector":{"kind":"route","method":"get","path":"/signup"},"surface":"account.create.signup.start","symbol":"signup_handler"}],"evidence":[{"extensions":{},"facets":["behavior"],"freshness":"9007199254740992","id":"signup_test","kind":"test","minimum_count":"9007199254740992","requirements":["account.create.signup.start#visible"],"runner":"runner.cargo","selector":{"framework":"cargo","kind":"test","test_id":"signup"}}],"exposures":[{"applicability":{"always":true},"extensions":{},"state":"required","surface":"account.create.signup.start"}],"extensions":{"dev.equivalencematrix.extra":{"boundary":"-9223372036854775808"}},"id":"web.account","owners":["owner://team/web"],"revision":"9007199254740992","target":"web","unit":"account.create"}"#;

const BOUNDARY_DIGEST: &str =
    "sha256:d1ed8eacb6f2d61ddc847ae12bf6f7cd509d12071d36891dd307e8a1f626a619";

fn route_selector(path: &str) -> Result<ArtifactSelector, Box<dyn Error>> {
    Ok(ArtifactSelector::Route {
        path: SelectorText::new(path)?,
        method: Some(HttpMethod::Get),
    })
}

fn artifact(
    id: &str,
    path: &str,
    selector: Option<ArtifactSelector>,
    extensions: Extensions,
) -> Result<Artifact, Box<dyn Error>> {
    Ok(Artifact::new(
        ArtifactId::new(id)?,
        ArtifactRole::Route,
        RepoPath::new(path)?,
        Some(SurfaceId::new("account.create.signup.start")?),
        Some(SelectorText::new("signup_handler")?),
        selector,
        extensions,
    )?)
}

fn evidence(
    minimum_count: Option<u64>,
    freshness: Option<u64>,
    extensions: Extensions,
) -> Result<EvidenceSpecification, Box<dyn Error>> {
    Ok(EvidenceSpecification::new(
        EvidenceSpecId::new("signup_test")?,
        EvidenceKind::Test,
        vec![FullRequirementId::new(
            "account.create.signup.start#visible",
        )?],
        vec![Facet::Behavior],
        Some(RunnerId::new("runner.cargo")?),
        Some(EvidenceSelector::Test {
            framework: SelectorText::new("cargo")?,
            test_id: SelectorText::new("signup")?,
            suite: None,
        }),
        minimum_count.map(PositiveCount::new).transpose()?,
        freshness.map(DurationMillis::new).transpose()?,
        extensions,
    )?)
}

fn exposure(applicability: Applicability) -> Result<Exposure, Box<dyn Error>> {
    Ok(Exposure::new(
        SurfaceId::new("account.create.signup.start")?,
        IntendedExposureState::Required,
        applicability,
        None,
        Extensions::default(),
    ))
}

#[allow(clippy::too_many_arguments)]
fn binding(
    revision: u64,
    owners: &[&str],
    artifacts: Vec<Artifact>,
    exposures: Vec<Exposure>,
    evidence: Vec<EvidenceSpecification>,
    extensions: Extensions,
) -> Result<Binding, Box<dyn Error>> {
    Ok(Binding::new(
        BindingId::new("web.account")?,
        Revision::new(revision)?,
        owners
            .iter()
            .map(|owner| OwnerRef::from_str(owner))
            .collect::<Result<Vec<_>, _>>()?,
        TargetId::new("web")?,
        UnitId::new("account.create")?,
        Artifacts::new(artifacts)?,
        exposures,
        evidence,
        extensions,
    )?)
}

fn base_binding() -> Result<Binding, Box<dyn Error>> {
    binding(
        1,
        &["owner://team/web"],
        vec![artifact(
            "signup",
            "src/signup.rs",
            Some(route_selector("/signup")?),
            Extensions::default(),
        )?],
        vec![exposure(Applicability::always(true))?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        Extensions::default(),
    )
}

fn display_extensions() -> Result<Extensions, Box<dyn Error>> {
    Ok(Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.display")?,
        ExtensionValue::Object(BTreeMap::from([(
            ExtensionKey::new("label")?,
            ExtensionValue::String("Signup".into()),
        )])),
    )]))?)
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
    let canonical = canonicalize_binding_v2(&base_binding()?)?;
    assert_eq!(std::str::from_utf8(canonical.bytes())?, BASE_BYTES);
    assert_eq!(canonical.digest().to_string(), BASE_DIGEST);
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
    let boundary = binding(
        MAX_SAFE_INTEGER_OVER,
        &["owner://team/web"],
        vec![artifact(
            "signup",
            "src/signup.rs",
            Some(route_selector("/signup")?),
            Extensions::default(),
        )?],
        vec![exposure(Applicability::always(true))?],
        vec![evidence(
            Some(MAX_SAFE_INTEGER_OVER),
            Some(MAX_SAFE_INTEGER_OVER),
            Extensions::default(),
        )?],
        extensions,
    )?;
    let canonical = canonicalize_binding_v2(&boundary)?;
    assert_eq!(std::str::from_utf8(canonical.bytes())?, BOUNDARY_BYTES);
    assert_eq!(canonical.digest().to_string(), BOUNDARY_DIGEST);
    Ok(())
}

#[test]
fn same_id_selector_and_path_changes_alter_identity() -> Result<(), Box<dyn Error>> {
    let base = canonicalize_binding_v2(&base_binding()?)?.digest();

    let selector_changed = binding(
        1,
        &["owner://team/web"],
        vec![artifact(
            "signup",
            "src/signup.rs",
            Some(route_selector("/register")?),
            Extensions::default(),
        )?],
        vec![exposure(Applicability::always(true))?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        Extensions::default(),
    )?;
    assert_ne!(
        base,
        canonicalize_binding_v2(&selector_changed)?.digest(),
        "a same-ID selector change must alter binding identity"
    );

    let path_changed = binding(
        1,
        &["owner://team/web"],
        vec![artifact(
            "signup",
            "src/other.rs",
            Some(route_selector("/signup")?),
            Extensions::default(),
        )?],
        vec![exposure(Applicability::always(true))?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        Extensions::default(),
    )?;
    assert_ne!(
        base,
        canonicalize_binding_v2(&path_changed)?.digest(),
        "a declared artifact path change must alter binding identity"
    );
    Ok(())
}

#[test]
fn evidence_constraint_changes_alter_identity() -> Result<(), Box<dyn Error>> {
    let base = canonicalize_binding_v2(&base_binding()?)?.digest();
    for changed in [
        evidence(Some(2), Some(60_000), Extensions::default())?,
        evidence(Some(1), Some(120_000), Extensions::default())?,
        evidence(Some(1), Some(60_000), extra_extensions("marker")?)?,
    ] {
        let candidate = binding(
            1,
            &["owner://team/web"],
            vec![artifact(
                "signup",
                "src/signup.rs",
                Some(route_selector("/signup")?),
                Extensions::default(),
            )?],
            vec![exposure(Applicability::always(true))?],
            vec![changed],
            Extensions::default(),
        )?;
        assert_ne!(
            base,
            canonicalize_binding_v2(&candidate)?.digest(),
            "a constraint or spec-extension change must alter identity"
        );
    }
    Ok(())
}

#[test]
fn semantic_extensions_alter_identity_and_display_extensions_stay_stable()
-> Result<(), Box<dyn Error>> {
    let base = canonicalize_binding_v2(&base_binding()?)?.digest();

    let semantic = binding(
        1,
        &["owner://team/web"],
        vec![artifact(
            "signup",
            "src/signup.rs",
            Some(route_selector("/signup")?),
            Extensions::default(),
        )?],
        vec![exposure(Applicability::always(true))?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        extra_extensions("marker")?,
    )?;
    assert_ne!(
        base,
        canonicalize_binding_v2(&semantic)?.digest(),
        "a non-display extension namespace must alter identity"
    );

    let display = binding(
        1,
        &["owner://team/web"],
        vec![artifact(
            "signup",
            "src/signup.rs",
            Some(route_selector("/signup")?),
            Extensions::default(),
        )?],
        vec![exposure(Applicability::always(true))?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        display_extensions()?,
    )?;
    assert_eq!(
        base,
        canonicalize_binding_v2(&display)?.digest(),
        "the display-only extension namespace must not alter identity"
    );
    Ok(())
}

#[test]
fn projection_excludes_parse_provenance_and_keeps_only_semantics() -> Result<(), Box<dyn Error>> {
    let projected = project_binding_v2(&base_binding()?)?;
    let object = projected
        .as_object()
        .ok_or("projected binding is not an object")?;
    let keys: Vec<&str> = object.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        vec![
            "artifacts",
            "evidence",
            "exposures",
            "extensions",
            "id",
            "owners",
            "revision",
            "target",
            "unit",
        ]
    );
    for provenance in [
        "comment",
        "description",
        "source",
        "location",
        "span",
        "path_span",
    ] {
        assert!(
            !object.contains_key(provenance),
            "projection must not invent a {provenance} field"
        );
    }

    // Two independently constructed but semantically identical bindings produce
    // identical bytes; formatting/comment text has no representation in Binding.
    let canonical = canonicalize_binding_v2(&base_binding()?)?;
    let rebuilt = canonicalize_binding_v2(&base_binding()?)?;
    assert_eq!(canonical, rebuilt);
    Ok(())
}

#[test]
fn set_members_are_ordered_by_complete_canonical_bytes() -> Result<(), Box<dyn Error>> {
    // Artifact "b" carries an extension that makes its canonical member bytes
    // sort before artifact "a", so member-byte order differs from ArtifactId
    // (BTreeMap/derived Ord) order.
    let artifacts = vec![
        artifact(
            "a",
            "src/a.rs",
            Some(route_selector("/a")?),
            Extensions::default(),
        )?,
        artifact(
            "b",
            "src/b.rs",
            Some(route_selector("/b")?),
            extra_extensions("zzz")?,
        )?,
    ];
    let in_id_order = binding(
        1,
        &["owner://team/web"],
        artifacts.clone(),
        vec![exposure(Applicability::always(true))?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        Extensions::default(),
    )?;
    let reversed = binding(
        1,
        &["owner://team/web"],
        artifacts.into_iter().rev().collect(),
        vec![exposure(Applicability::always(true))?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        Extensions::default(),
    )?;
    let first = project_binding_v2(&in_id_order)?;
    let second = project_binding_v2(&reversed)?;
    assert_eq!(first, second, "set permutations must agree");
    assert_eq!(
        first["artifacts"][0]["id"], "b",
        "artifacts must be ordered by complete member bytes, not ArtifactId"
    );

    // Membership values are ordered as a canonical set.
    let membership = Applicability::membership(
        DimensionId::new("region")?,
        MembershipOperator::In,
        vec![SymbolicValueId::new("us")?, SymbolicValueId::new("eu")?],
    )?;
    let with_members = binding(
        1,
        &["owner://team/web"],
        vec![artifact(
            "signup",
            "src/signup.rs",
            Some(route_selector("/signup")?),
            Extensions::default(),
        )?],
        vec![exposure(membership)?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        Extensions::default(),
    )?;
    let projected = project_binding_v2(&with_members)?;
    assert_eq!(
        projected["exposures"][0]["applicability"]["values"],
        serde_json::json!(["eu", "us"])
    );

    // Requirements and owners are canonical string sets.
    let comparison = Applicability::compare(
        DimensionId::new("region")?,
        ComparisonOperator::Equal,
        SymbolicValueId::new("eu")?,
    );
    let multi_owner = binding(
        1,
        &["owner://team/web", "owner://team/accounts"],
        vec![artifact(
            "signup",
            "src/signup.rs",
            Some(route_selector("/signup")?),
            Extensions::default(),
        )?],
        vec![exposure(comparison)?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        Extensions::default(),
    )?;
    let projected = project_binding_v2(&multi_owner)?;
    assert_eq!(
        projected["owners"],
        serde_json::json!(["owner://team/accounts", "owner://team/web"])
    );
    Ok(())
}

#[test]
fn ordered_extension_arrays_preserve_declared_order() -> Result<(), Box<dyn Error>> {
    let array_extension = |values: Vec<ExtensionValue>| -> Result<Extensions, Box<dyn Error>> {
        Ok(Extensions::new(BTreeMap::from([(
            ExtensionNamespace::new("dev.equivalencematrix.extra")?,
            ExtensionValue::Object(BTreeMap::from([(
                ExtensionKey::new("ordered")?,
                ExtensionValue::Array(values),
            )])),
        )]))?)
    };
    let build = |extensions: Extensions| -> Result<Binding, Box<dyn Error>> {
        binding(
            1,
            &["owner://team/web"],
            vec![artifact(
                "signup",
                "src/signup.rs",
                Some(route_selector("/signup")?),
                Extensions::default(),
            )?],
            vec![exposure(Applicability::always(true))?],
            vec![evidence(Some(1), Some(60_000), Extensions::default())?],
            extensions,
        )
    };
    let ascending = build(array_extension(vec![
        ExtensionValue::Integer(1),
        ExtensionValue::Integer(2),
    ])?)?;
    let descending = build(array_extension(vec![
        ExtensionValue::Integer(2),
        ExtensionValue::Integer(1),
    ])?)?;
    let ascending = project_binding_v2(&ascending)?;
    let descending = project_binding_v2(&descending)?;
    assert_eq!(
        ascending["extensions"]["dev.equivalencematrix.extra"]["ordered"],
        serde_json::json!([1, 2])
    );
    assert_eq!(
        descending["extensions"]["dev.equivalencematrix.extra"]["ordered"],
        serde_json::json!([2, 1])
    );
    assert_ne!(
        ascending, descending,
        "ordered extension arrays must not be sorted"
    );
    Ok(())
}
