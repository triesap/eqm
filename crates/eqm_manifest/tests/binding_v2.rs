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
    BindingDigest, BindingId, ComparisonOperator, DimensionId, DurationMillis, EntityBuildError,
    EvidenceKind, EvidenceSelector, EvidenceSpecId, EvidenceSpecification, Exposure, ExtensionKey,
    ExtensionNamespace, ExtensionValue, Extensions, Facet, FullRequirementId, HttpMethod,
    IntendedExposureState, MembershipOperator, OwnerRef, PositiveCount, RepoPath, Revision,
    RunnerId, SelectorText, SurfaceId, SymbolicValueId, TargetId, UnitId,
};
use eqm_manifest::{
    CanonicalizationError, canonicalize_binding_v2, load_workspace, project_binding_v2,
};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
const MAX_SAFE_INTEGER_OVER: u64 = MAX_SAFE_INTEGER + 1;

const BASE_BYTES: &str = r#"{"artifacts":[{"extensions":{},"id":"signup","path":"src/signup.rs","role":"route","selector":{"kind":"route","method":"get","path":"/signup"},"surface":"account.create.signup.start","symbol":"signup_handler"}],"evidence":[{"extensions":{},"facets":["behavior"],"freshness":60000,"id":"signup_test","kind":"test","minimum_count":1,"requirements":["account.create.signup.start#visible"],"runner":"runner.cargo","selector":{"framework":"cargo","kind":"test","test_id":"signup"}}],"exposures":[{"applicability":{"always":true},"extensions":{},"state":"required","surface":"account.create.signup.start"}],"extensions":{},"id":"web.account","owners":["owner://team/web"],"revision":1,"target":"web","unit":"account.create"}"#;

const BASE_DIGEST: &str = "sha256:b8154689fb2253814a3184f5e19436c6aaddfc4ecae0618a5c5a44b5a74f8c87";

const BOUNDARY_BYTES: &str = r#"{"artifacts":[{"extensions":{},"id":"signup","path":"src/signup.rs","role":"route","selector":{"kind":"route","method":"get","path":"/signup"},"surface":"account.create.signup.start","symbol":"signup_handler"}],"evidence":[{"extensions":{},"facets":["behavior"],"freshness":"9007199254740992","id":"signup_test","kind":"test","minimum_count":"9007199254740992","requirements":["account.create.signup.start#visible"],"runner":"runner.cargo","selector":{"framework":"cargo","kind":"test","test_id":"signup"}}],"exposures":[{"applicability":{"always":true},"extensions":{},"state":"required","surface":"account.create.signup.start"}],"extensions":{"dev.equivalencematrix.extra":{"type":"object","value":{"boundary":{"type":"integer","value":"-9223372036854775808"}}}},"id":"web.account","owners":["owner://team/web"],"revision":"9007199254740992","target":"web","unit":"account.create"}"#;

const BOUNDARY_DIGEST: &str =
    "sha256:d12c3f60e676d774449cdd600209af986e574fc920b88afffe4d44bdf201d1f6";

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
        ascending["extensions"]["dev.equivalencematrix.extra"]["value"]["ordered"],
        serde_json::json!({
            "type": "array",
            "value": [
                {"type": "integer", "value": 1},
                {"type": "integer", "value": 2}
            ]
        })
    );
    assert_eq!(
        descending["extensions"]["dev.equivalencematrix.extra"]["value"]["ordered"],
        serde_json::json!({
            "type": "array",
            "value": [
                {"type": "integer", "value": 2},
                {"type": "integer", "value": 1}
            ]
        })
    );
    assert_ne!(
        ascending, descending,
        "ordered extension arrays must not be sorted"
    );
    Ok(())
}

#[test]
fn binding_identity_is_the_typed_domain_wrapper() -> Result<(), Box<dyn Error>> {
    let canonical = canonicalize_binding_v2(&base_binding()?)?;
    // The API returns the concrete `BindingDigest`, not a generic SHA-256 value.
    let typed: BindingDigest = canonical.digest();
    assert_eq!(typed, BindingDigest::hash_content(canonical.bytes()));
    // Raw extraction is explicit and only at the wire/legacy boundary.
    assert_eq!(typed.as_digest().to_string(), BASE_DIGEST);
    Ok(())
}

fn extension_named(value: ExtensionValue) -> Result<Extensions, Box<dyn Error>> {
    Ok(Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.extra")?,
        ExtensionValue::Object(BTreeMap::from([(ExtensionKey::new("same_id")?, value)])),
    )]))?)
}

fn binding_with_extension(value: ExtensionValue) -> Result<Binding, Box<dyn Error>> {
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
        extension_named(value)?,
    )
}

#[test]
fn extension_integer_and_text_with_same_digits_never_collide() -> Result<(), Box<dyn Error>> {
    // Same binding ID and every other field fixed; only the extension value type
    // differs. The recursive {type,value} encoding must keep them distinct.
    for (integer, text) in [
        (ExtensionValue::Integer(i64::MIN), "-9223372036854775808"),
        (
            ExtensionValue::Integer(MAX_SAFE_INTEGER_OVER as i64),
            "9007199254740992",
        ),
    ] {
        let integer = canonicalize_binding_v2(&binding_with_extension(integer)?)?;
        let string = canonicalize_binding_v2(&binding_with_extension(ExtensionValue::String(
            text.into(),
        ))?)?;
        assert_ne!(
            integer.bytes(),
            string.bytes(),
            "integer {text} and text must not share canonical bytes"
        );
        assert_ne!(
            integer.digest(),
            string.digest(),
            "integer {text} and text must not share identity"
        );
    }

    // Nested array/object payloads retain the distinction at depth.
    let nested_integer = ExtensionValue::Object(BTreeMap::from([(
        ExtensionKey::new("nested")?,
        ExtensionValue::Array(vec![
            ExtensionValue::Integer(i64::MIN),
            ExtensionValue::Integer(MAX_SAFE_INTEGER_OVER as i64),
        ]),
    )]));
    let nested_text = ExtensionValue::Object(BTreeMap::from([(
        ExtensionKey::new("nested")?,
        ExtensionValue::Array(vec![
            ExtensionValue::String("-9223372036854775808".into()),
            ExtensionValue::String("9007199254740992".into()),
        ]),
    )]));
    assert_ne!(
        canonicalize_binding_v2(&binding_with_extension(nested_integer)?)?.bytes(),
        canonicalize_binding_v2(&binding_with_extension(nested_text)?)?.bytes()
    );

    // An authored object that merely resembles a tag is not the typed integer it
    // resembles: the object is tagged as `object`, not flattened into `integer`.
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
        canonicalize_binding_v2(&binding_with_extension(tag_like)?)?.bytes(),
        canonicalize_binding_v2(&binding_with_extension(ExtensionValue::Integer(i64::MIN))?)?
            .bytes()
    );
    Ok(())
}

#[test]
fn extension_encoding_is_recursive_for_every_variant() -> Result<(), Box<dyn Error>> {
    let projected = project_binding_v2(&binding_with_extension(ExtensionValue::Object(
        BTreeMap::from([
            (ExtensionKey::new("bool")?, ExtensionValue::Boolean(true)),
            (ExtensionKey::new("int")?, ExtensionValue::Integer(7)),
            (
                ExtensionKey::new("str")?,
                ExtensionValue::String("x".into()),
            ),
            (
                ExtensionKey::new("arr")?,
                ExtensionValue::Array(vec![ExtensionValue::Integer(1)]),
            ),
        ]),
    ))?)?;
    let root = &projected["extensions"]["dev.equivalencematrix.extra"]["value"]["same_id"];
    assert_eq!(
        root,
        &serde_json::json!({
            "type": "object",
            "value": {
                "arr": {"type": "array", "value": [{"type": "integer", "value": 1}]},
                "bool": {"type": "boolean", "value": true},
                "int": {"type": "integer", "value": 7},
                "str": {"type": "string", "value": "x"}
            }
        })
    );
    Ok(())
}

fn example_workspace() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("workspace root unavailable")?
        .join("examples/android-ios");
    let directory = tempfile::tempdir()?;
    copy_tree(&source, directory.path())?;
    fs::create_dir_all(directory.path().join(".git"))?;
    Ok(directory)
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn loaded_binding(root: &Path) -> Result<Binding, Box<dyn Error>> {
    let loaded = load_workspace(root, None)?;
    loaded
        .graph_input()
        .bindings
        .iter()
        .find(|binding| binding.id().as_str() == "binding.ios.auth_signup")
        .cloned()
        .ok_or_else(|| "binding.ios.auth_signup unavailable".into())
}

#[test]
fn loaded_manifest_comment_and_formatting_do_not_change_projection() -> Result<(), Box<dyn Error>> {
    let repository = example_workspace()?;
    let path = repository.path().join("eqm/bindings/ios.auth_signup.toml");
    let original = fs::read_to_string(&path)?;

    let baseline_binding = loaded_binding(repository.path())?;
    let id = baseline_binding.id().clone();
    let revision = baseline_binding.revision();
    let baseline = canonicalize_binding_v2(&baseline_binding)?;

    // Comments and formatting have no representation in the typed Binding: the
    // loaded graph must project to identical bytes with the same ID/revision.
    let reformatted = format!(
        "# authored comment retained in the manifest, not in the projection\n\n{}\n   \n",
        original.replace("revision = 1", "revision = 1  # inline comment")
    );
    assert_ne!(reformatted, original);
    fs::write(&path, reformatted)?;
    let reloaded = loaded_binding(repository.path())?;
    assert_eq!(reloaded.id(), &id, "ID must stay fixed");
    assert_eq!(reloaded.revision(), revision, "revision must stay fixed");
    let after = canonicalize_binding_v2(&reloaded)?;
    assert_eq!(after.bytes(), baseline.bytes());
    assert_eq!(after.digest(), baseline.digest());

    // A real semantic selector mutation keeps the IDs fixed but changes bytes.
    let semantic = original.replace(
        "test_id = \"testSignupIdentifierDefaultsToEmail\"",
        "test_id = \"testSignupIdentifierChanged\"",
    );
    assert_ne!(semantic, original);
    fs::write(&path, semantic)?;
    let mutated_binding = loaded_binding(repository.path())?;
    assert_eq!(mutated_binding.id(), &id, "ID must stay fixed");
    assert_eq!(mutated_binding.revision(), revision);
    let mutated = canonicalize_binding_v2(&mutated_binding)?;
    assert_ne!(mutated.bytes(), after.bytes());
    assert_ne!(mutated.digest(), after.digest());
    Ok(())
}

#[test]
fn projected_duplicate_after_display_exclusion_is_reachable() -> Result<(), Box<dyn Error>> {
    let exposure_with_display = |label: &str| -> Result<Exposure, Box<dyn Error>> {
        let extensions = Extensions::new(BTreeMap::from([(
            ExtensionNamespace::new("dev.equivalencematrix.display")?,
            ExtensionValue::String(label.into()),
        )]))?;
        Ok(Exposure::new(
            SurfaceId::new("account.create.signup.start")?,
            IntendedExposureState::Required,
            Applicability::always(true),
            None,
            extensions,
        ))
    };
    let plain = || -> Result<Exposure, Box<dyn Error>> {
        Ok(Exposure::new(
            SurfaceId::new("account.create.signup.start")?,
            IntendedExposureState::Required,
            Applicability::always(true),
            None,
            Extensions::default(),
        ))
    };

    // Two exposures differ only in their display-only extension. `Binding::new`
    // accepts both because the full values differ...
    let duplicate = binding(
        1,
        &["owner://team/web"],
        vec![artifact(
            "signup",
            "src/signup.rs",
            Some(route_selector("/signup")?),
            Extensions::default(),
        )?],
        vec![exposure_with_display("A")?, exposure_with_display("B")?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        Extensions::default(),
    )?;
    // ...but excluding the display namespace makes them identical projected
    // members, which the canonical set rejects rather than silently collapsing.
    assert_eq!(
        canonicalize_binding_v2(&duplicate),
        Err(CanonicalizationError::DuplicateMember)
    );

    // A truly identical exposure pair is rejected earlier by `Binding::new`.
    let identical = binding(
        1,
        &["owner://team/web"],
        vec![artifact(
            "signup",
            "src/signup.rs",
            Some(route_selector("/signup")?),
            Extensions::default(),
        )?],
        vec![plain()?, plain()?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        Extensions::default(),
    );
    let Err(error) = identical else {
        return Err("identical exposures unexpectedly accepted".into());
    };
    assert!(matches!(
        error.downcast_ref::<eqm_domain::EntityBuildError>(),
        Some(eqm_domain::EntityBuildError::DuplicateExposure)
    ));

    // Distinct exposures (different non-display semantics) remain accepted and
    // project to distinct members.
    let distinct = binding(
        1,
        &["owner://team/web"],
        vec![artifact(
            "signup",
            "src/signup.rs",
            Some(route_selector("/signup")?),
            Extensions::default(),
        )?],
        vec![
            Exposure::new(
                SurfaceId::new("account.create.signup.start")?,
                IntendedExposureState::Required,
                Applicability::always(true),
                None,
                Extensions::default(),
            ),
            Exposure::new(
                SurfaceId::new("account.create.signup.start")?,
                IntendedExposureState::Prohibited,
                Applicability::always(true),
                None,
                Extensions::default(),
            ),
        ],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        Extensions::default(),
    )?;
    let projected = project_binding_v2(&distinct)?;
    assert_eq!(
        projected["exposures"].as_array().map(Vec::len),
        Some(2),
        "distinct exposures must remain two canonical members"
    );
    Ok(())
}

#[test]
fn nested_display_extensions_are_excluded_at_embedded_levels() -> Result<(), Box<dyn Error>> {
    let build = |artifact_extensions: Extensions,
                 evidence_extensions: Extensions|
     -> Result<Binding, Box<dyn Error>> {
        binding(
            1,
            &["owner://team/web"],
            vec![artifact(
                "signup",
                "src/signup.rs",
                Some(route_selector("/signup")?),
                artifact_extensions,
            )?],
            vec![exposure(Applicability::always(true))?],
            vec![evidence(Some(1), Some(60_000), evidence_extensions)?],
            Extensions::default(),
        )
    };
    let plain = build(Extensions::default(), Extensions::default())?;
    let decorated = build(display_extensions()?, display_extensions()?)?;
    assert_eq!(
        canonicalize_binding_v2(&plain)?,
        canonicalize_binding_v2(&decorated)?,
        "display extensions at embedded entity levels must not alter identity"
    );
    Ok(())
}

fn binding_with_applicability(applicability: Applicability) -> Result<Binding, Box<dyn Error>> {
    binding(
        1,
        &["owner://team/web"],
        vec![artifact(
            "signup",
            "src/signup.rs",
            Some(route_selector("/signup")?),
            Extensions::default(),
        )?],
        vec![exposure(applicability)?],
        vec![evidence(Some(1), Some(60_000), Extensions::default())?],
        Extensions::default(),
    )
}

fn comparison(dimension: &str, value: &str) -> Result<Applicability, Box<dyn Error>> {
    Ok(Applicability::compare(
        DimensionId::new(dimension)?,
        ComparisonOperator::Equal,
        SymbolicValueId::new(value)?,
    ))
}

#[test]
fn applicability_all_and_any_children_are_semantic_sets() -> Result<(), Box<dyn Error>> {
    // Identical child objects supplied in opposite input order are the same
    // semantic set and must produce identical bytes and digests.
    let forward = Applicability::all(vec![
        comparison("region", "us")?,
        comparison("tier", "gold")?,
    ])?;
    let permuted = Applicability::all(vec![
        comparison("tier", "gold")?,
        comparison("region", "us")?,
    ])?;
    let forward = canonicalize_binding_v2(&binding_with_applicability(forward)?)?;
    let permuted = canonicalize_binding_v2(&binding_with_applicability(permuted)?)?;
    assert_eq!(forward.bytes(), permuted.bytes());
    assert_eq!(forward.digest(), permuted.digest());

    let any_forward = Applicability::any(vec![
        comparison("region", "us")?,
        comparison("tier", "gold")?,
    ])?;
    let any_permuted = Applicability::any(vec![
        comparison("tier", "gold")?,
        comparison("region", "us")?,
    ])?;
    assert_eq!(
        canonicalize_binding_v2(&binding_with_applicability(any_forward)?)?,
        canonicalize_binding_v2(&binding_with_applicability(any_permuted)?)?
    );

    // A real child-value change is a different semantic set with a new identity.
    let changed = Applicability::all(vec![
        comparison("region", "eu")?,
        comparison("tier", "gold")?,
    ])?;
    assert_ne!(
        forward.digest(),
        canonicalize_binding_v2(&binding_with_applicability(changed)?)?.digest(),
        "a real child-value change must alter binding identity"
    );

    // Exact duplicate children reject at the existing domain boundary.
    assert!(matches!(
        Applicability::all(vec![
            comparison("region", "us")?,
            comparison("region", "us")?
        ]),
        Err(EntityBuildError::DuplicateApplicabilityOperand)
    ));
    assert!(matches!(
        Applicability::any(vec![
            comparison("region", "us")?,
            comparison("region", "us")?
        ]),
        Err(EntityBuildError::DuplicateApplicabilityOperand)
    ));
    Ok(())
}

#[test]
fn applicability_child_order_follows_jcs_member_bytes_not_enum_order() -> Result<(), Box<dyn Error>>
{
    // Derived Rust enum order puts `Comparison` before `All`, but a nested `all`
    // child serializes as `{"all":...}` which sorts before `{"dimension":...}`.
    // The reviewed literal output order proves the projection sorts by member
    // bytes rather than deriving the wire order from the enum.
    let region_child = comparison("region", "us")?;
    let nested_all = Applicability::all(vec![comparison("tier", "gold")?])?;
    let projected = project_binding_v2(&binding_with_applicability(Applicability::all(vec![
        region_child,
        nested_all,
    ])?)?)?;
    assert_eq!(
        projected["exposures"][0]["applicability"],
        serde_json::json!({
            "all": [
                {"all": [{"dimension": "tier", "operator": "eq", "value": "gold"}]},
                {"dimension": "region", "operator": "eq", "value": "us"}
            ]
        })
    );

    // The `any` form has the same `{"any":` < `{"dimension":` trap.
    let region_child = comparison("region", "us")?;
    let nested_any = Applicability::any(vec![comparison("tier", "gold")?])?;
    let projected = project_binding_v2(&binding_with_applicability(Applicability::any(vec![
        region_child,
        nested_any,
    ])?)?)?;
    assert_eq!(
        projected["exposures"][0]["applicability"],
        serde_json::json!({
            "any": [
                {"any": [{"dimension": "tier", "operator": "eq", "value": "gold"}]},
                {"dimension": "region", "operator": "eq", "value": "us"}
            ]
        })
    );
    Ok(())
}

#[test]
fn nested_logical_and_membership_children_project_recursively() -> Result<(), Box<dyn Error>> {
    let membership = Applicability::membership(
        DimensionId::new("region")?,
        MembershipOperator::In,
        vec![SymbolicValueId::new("us")?, SymbolicValueId::new("eu")?],
    )?;
    let negated = Applicability::logical_not(Applicability::any(vec![membership])?)?;
    let outer = Applicability::all(vec![negated, comparison("tier", "gold")?])?;
    let projected = project_binding_v2(&binding_with_applicability(outer)?)?;
    assert_eq!(
        projected["exposures"][0]["applicability"],
        serde_json::json!({
            "all": [
                {"dimension": "tier", "operator": "eq", "value": "gold"},
                {"not": {"any": [
                    {"dimension": "region", "operator": "in", "values": ["eu", "us"]}
                ]}}
            ]
        })
    );
    Ok(())
}

#[test]
fn swapping_values_across_fixed_dimensions_changes_identity() -> Result<(), Box<dyn Error>> {
    // Retained from S018-R2 as a *semantic mutation*, not an order proof: the
    // region and tier dimensions stay fixed while their values are exchanged, so
    // the resulting sets genuinely differ and the identity must change.
    let forward = Applicability::all(vec![
        comparison("region", "us")?,
        comparison("tier", "gold")?,
    ])?;
    let swapped = Applicability::all(vec![
        comparison("region", "gold")?,
        comparison("tier", "us")?,
    ])?;
    let forward = canonicalize_binding_v2(&binding_with_applicability(forward)?)?;
    let swapped = canonicalize_binding_v2(&binding_with_applicability(swapped)?)?;
    assert_ne!(
        forward.bytes(),
        swapped.bytes(),
        "exchanging values across fixed dimensions is a semantic mutation"
    );
    assert_ne!(forward.digest(), swapped.digest());

    let membership_forward = Applicability::any(vec![
        Applicability::membership(
            DimensionId::new("region")?,
            MembershipOperator::In,
            vec![SymbolicValueId::new("us")?],
        )?,
        Applicability::membership(
            DimensionId::new("tier")?,
            MembershipOperator::In,
            vec![SymbolicValueId::new("gold")?],
        )?,
    ])?;
    let membership_swapped = Applicability::any(vec![
        Applicability::membership(
            DimensionId::new("region")?,
            MembershipOperator::In,
            vec![SymbolicValueId::new("gold")?],
        )?,
        Applicability::membership(
            DimensionId::new("tier")?,
            MembershipOperator::In,
            vec![SymbolicValueId::new("us")?],
        )?,
    ])?;
    assert_ne!(
        canonicalize_binding_v2(&binding_with_applicability(membership_forward)?)?.bytes(),
        canonicalize_binding_v2(&binding_with_applicability(membership_swapped)?)?.bytes()
    );
    Ok(())
}
