//! S039-R1: checked v2 comparison-context protocol ingress.
//!
//! The reader is bytes-first and bounded; it validates the exact schema marker,
//! typed SHA references and the selected-profile/dimension contract, then
//! binds an immutable checked carrier to canonical four-member semantic bytes
//! and a typed `ComparisonContextDigest`. Deserialization grants no trust.

use eqm_domain::{ComparisonContextDigest, Sha256Digest};
use eqm_protocol::schema_graph_v2::{
    CheckedComparisonContextV2, ComparisonContextError, ComparisonContextV2Dto,
};
use serde_json::{Value, json};
use std::error::Error;

fn digest(byte: char) -> String {
    format!("sha256:{}", byte.to_string().repeat(64))
}

fn profile(id: &str, revision: u64, values: Value) -> Value {
    json!({
        "definition": {
            "id": id,
            "revision": revision,
            "title": "Audience",
            "owners": ["owner://team/product"],
            "dimensions": [
                {"id": "locale", "values": ["en", "fr"]},
                {"id": "region", "values": ["eu", "us"]}
            ],
            "defaults": {},
            "extensions": {}
        },
        "values": values
    })
}

fn wire() -> Value {
    json!({
        "schema": eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string(),
        "contract_digest": digest('1'),
        "scenario_digest": digest('2'),
        "comparison_rule_digest": digest('3'),
        "profiles": [profile(
            "audience.default",
            1,
            json!([
                {"dimension": "locale", "state": "known", "value": "en"},
                {"dimension": "region", "state": "unknown"}
            ])
        )]
    })
}

fn parse(value: &Value) -> Result<CheckedComparisonContextV2, ComparisonContextError> {
    let bytes = serde_json::to_vec(value).map_err(|_| ComparisonContextError::Json)?;
    CheckedComparisonContextV2::from_bytes(&bytes)
}

#[test]
fn valid_comparison_wire_binds_canonical_semantic_identity() -> Result<(), Box<dyn Error>> {
    let bytes = serde_json::to_vec(&wire())?;
    let checked = CheckedComparisonContextV2::from_bytes(&bytes)?;
    assert_eq!(checked.raw_bytes(), bytes.as_slice());
    // The semantic root excludes only the schema marker.
    let semantic: Value = serde_json::from_slice(checked.semantic_bytes())?;
    let object = semantic
        .as_object()
        .ok_or("semantic root is not an object")?;
    let mut keys = object.keys().map(String::as_str).collect::<Vec<_>>();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "comparison_rule_digest",
            "contract_digest",
            "profiles",
            "scenario_digest"
        ]
    );
    assert_eq!(
        checked.digest(),
        ComparisonContextDigest::hash_content(checked.semantic_bytes())
    );
    // A permutation of a canonical set still yields the same identity.
    let mut reordered = wire();
    reordered["profiles"][0]["values"] = json!([
        {"dimension": "region", "state": "unknown"},
        {"dimension": "locale", "state": "known", "value": "en"}
    ]);
    assert_eq!(parse(&reordered)?.digest(), checked.digest());
    Ok(())
}

#[test]
fn invalid_schema_marker_and_digests_reject() -> Result<(), Box<dyn Error>> {
    let mut missing = wire();
    missing.as_object_mut().ok_or("wire")?.remove("schema");
    assert_eq!(parse(&missing), Err(ComparisonContextError::Json));

    let mut wrong = wire();
    wrong["schema"] = json!("https://example.com/other.schema.json");
    assert_eq!(parse(&wrong), Err(ComparisonContextError::InvalidSchema));

    for field in [
        "contract_digest",
        "scenario_digest",
        "comparison_rule_digest",
    ] {
        for bad in [
            json!("x"),
            json!(format!("sha256:{}", "A".repeat(64))),
            json!(5),
            json!(null),
        ] {
            let mut mutated = wire();
            mutated[field] = bad.clone();
            assert!(parse(&mutated).is_err(), "{field} accepted {bad}");
        }
    }
    Ok(())
}

#[test]
fn null_optionals_and_unknown_members_reject() -> Result<(), Box<dyn Error>> {
    let mut null_description = wire();
    null_description["profiles"][0]["definition"]["description"] = json!(null);
    assert!(
        parse(&null_description).is_err(),
        "explicit null must reject"
    );

    let mut null_value = wire();
    null_value["profiles"][0]["values"][0]["value"] = json!(null);
    assert!(
        parse(&null_value).is_err(),
        "explicit null value must reject"
    );

    let mut extra_root = wire();
    extra_root["unknown_member"] = json!(true);
    assert!(parse(&extra_root).is_err(), "root extra member must reject");

    let mut extra_definition = wire();
    extra_definition["profiles"][0]["definition"]["unknown_member"] = json!(true);
    assert!(
        parse(&extra_definition).is_err(),
        "definition extra member must reject"
    );
    Ok(())
}

#[test]
fn selected_profile_and_dimension_contract_rejects_misuse() -> Result<(), Box<dyn Error>> {
    // Unknown with a value.
    let mut unknown_value = wire();
    unknown_value["profiles"][0]["values"][1] = json!({
        "dimension": "region",
        "state": "unknown",
        "value": "eu"
    });
    assert!(
        parse(&unknown_value).is_err(),
        "unknown with value must reject"
    );

    // Known without a value.
    let mut known_missing = wire();
    known_missing["profiles"][0]["values"][0] = json!({
        "dimension": "locale",
        "state": "known"
    });
    assert!(
        parse(&known_missing).is_err(),
        "known without value must reject"
    );

    // Unknown state marker.
    let mut bad_state = wire();
    bad_state["profiles"][0]["values"][0] = json!({
        "dimension": "locale",
        "state": "bogus",
        "value": "en"
    });
    assert!(parse(&bad_state).is_err(), "bad state must reject");

    // Known value outside the finite declaration.
    let mut undeclared = wire();
    undeclared["profiles"][0]["values"][0] = json!({
        "dimension": "locale",
        "state": "known",
        "value": "de"
    });
    assert_eq!(
        parse(&undeclared),
        Err(ComparisonContextError::UndeclaredValue)
    );

    // Missing declared dimension.
    let mut missing = wire();
    missing["profiles"][0]["values"] = json!([
        {"dimension": "locale", "state": "known", "value": "en"}
    ]);
    assert_eq!(
        parse(&missing),
        Err(ComparisonContextError::MissingDimension)
    );

    // Undeclared dimension.
    let mut extra = wire();
    extra["profiles"][0]["values"] = json!([
        {"dimension": "locale", "state": "known", "value": "en"},
        {"dimension": "region", "state": "unknown"},
        {"dimension": "channel", "state": "unknown"}
    ]);
    assert_eq!(
        parse(&extra),
        Err(ComparisonContextError::UndeclaredDimension)
    );

    // Duplicate selected profile ID across revisions is a logical duplicate.
    let mut duplicate_profile = wire();
    duplicate_profile["profiles"] = json!([
        profile(
            "audience.default",
            1,
            json!([
                {"dimension": "locale", "state": "known", "value": "en"},
                {"dimension": "region", "state": "unknown"}
            ])
        ),
        profile(
            "audience.default",
            2,
            json!([
                {"dimension": "locale", "state": "known", "value": "fr"},
                {"dimension": "region", "state": "unknown"}
            ])
        )
    ]);
    assert_eq!(
        parse(&duplicate_profile).map(|_| ()),
        Err(ComparisonContextError::DuplicateProfile)
    );
    Ok(())
}

#[test]
fn checked_receiver_requires_equal_raw_bytes() -> Result<(), Box<dyn Error>> {
    let bytes = serde_json::to_vec(&wire())?;
    let record = ComparisonContextV2Dto::from_json(&bytes)?;
    let checked = record.checked(&bytes)?;
    assert_eq!(checked.record(), &record);
    let mut other = wire();
    other["contract_digest"] = json!(digest('9'));
    assert_eq!(
        record.checked(&serde_json::to_vec(&other)?).map(|_| ()),
        Err(ComparisonContextError::RecordMismatch)
    );
    // A checked carrier's digest is typed, never a raw digest cast.
    assert_eq!(
        checked.digest(),
        ComparisonContextDigest::hash_content(checked.semantic_bytes())
    );
    assert_ne!(
        checked.digest().to_string(),
        Sha256Digest::hash_content(b"unrelated").to_string()
    );
    Ok(())
}

#[test]
fn invalid_profile_definitions_reject_full_domain_invariants() -> Result<(), Box<dyn Error>> {
    // Empty owners, empty title and empty dimensions are domain construction
    // failures, not merely structural ones.
    for (field, value) in [
        ("owners", json!([])),
        ("title", json!("")),
        ("dimensions", json!([])),
    ] {
        let mut mutated = wire();
        mutated["profiles"][0]["definition"][field] = value;
        assert_eq!(
            parse(&mutated),
            Err(ComparisonContextError::InvalidDefinition),
            "{field} accepted"
        );
    }

    // Duplicate logical dimension key with distinct members is a domain
    // duplicate, not an exact-set duplicate.
    let mut duplicate_dimension = wire();
    duplicate_dimension["profiles"][0]["definition"]["dimensions"] = json!([
        {"id": "locale", "values": ["en", "fr"]},
        {"id": "locale", "values": ["de"]}
    ]);
    assert_eq!(
        parse(&duplicate_dimension),
        Err(ComparisonContextError::InvalidDefinition)
    );

    // A default outside its declared finite values is invalid.
    let mut bad_default = wire();
    bad_default["profiles"][0]["definition"]["defaults"] = json!({"locale": "de"});
    assert_eq!(
        parse(&bad_default),
        Err(ComparisonContextError::InvalidDefinition)
    );

    // An invalid recursive extension namespace is rejected by the owning domain
    // validator, not accepted as opaque JSON.
    let mut bad_extension = wire();
    bad_extension["profiles"][0]["definition"]["extensions"] =
        json!({"bad": {"type": "boolean", "value": true}});
    assert_eq!(
        parse(&bad_extension),
        Err(ComparisonContextError::InvalidExtension)
    );

    // Recursive extension depth beyond the domain bound is rejected.
    let mut deep = json!({"type": "boolean", "value": true});
    for _ in 0..20 {
        deep = json!({"type": "array", "value": [deep]});
    }
    let mut too_deep = wire();
    too_deep["profiles"][0]["definition"]["extensions"] = json!({"dev.example.ext": deep});
    assert_eq!(
        parse(&too_deep),
        Err(ComparisonContextError::InvalidExtension)
    );
    Ok(())
}
