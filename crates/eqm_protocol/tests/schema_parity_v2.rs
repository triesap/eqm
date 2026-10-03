//! S043: v2 provider/cohort/graph schema parity and adversarial one-axis checks.
//!
//! The committed manifest and protocol schemas are validated with a real Draft
//! 2020-12 validator against representative authored documents and independent
//! one-axis negative mutations, so the public schema surface matches the model.

use jsonschema::{Draft, Validator};
use serde_json::{Value, json};
use std::error::Error;
use std::path::{Path, PathBuf};

fn validator(relative: &str) -> Result<Validator, Box<dyn Error>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative);
    let schema: Value = serde_json::from_slice(&std::fs::read(&path)?)?;
    Ok(jsonschema::options()
        .with_draft(Draft::Draft202012)
        .should_validate_formats(true)
        .build(&schema)?)
}

fn digest(byte: char) -> String {
    format!("sha256:{}", byte.to_string().repeat(64))
}

fn provider_member() -> Value {
    json!({
        "id": "provider.mailgun",
        "configuration_digest": digest('1'),
        "owners": ["owner://team/providers"],
        "extensions": {}
    })
}

fn provider_document() -> Value {
    let mut document = provider_member();
    document["schema"] = json!(
        "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/provider.schema.json"
    );
    document
}

fn cohort_member() -> Value {
    json!({
        "id": "cohort.n",
        "revision": 1,
        "scenario": "signup.flow",
        "assignments": [{
            "occurrence": "mailer",
            "repository": "https://example.com/team/project",
            "source_commit": "a".repeat(40),
            "source_content_digest": digest('2'),
            "provider_configuration_digest": digest('4'),
            "build_id": "build.n.mailer",
            "artifact_digest": digest('3')
        }],
        "extensions": {}
    })
}

fn cohort_document() -> Value {
    let mut document = cohort_member();
    document["schema"] = json!(
        "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/cohort.schema.json"
    );
    document
}

fn graph_document(provider: Value, cohort: Value) -> Value {
    json!({
        "schema": eqm_protocol::SEMANTIC_GRAPH_SCHEMA_V2.to_string(),
        "capabilities": [],
        "journeys": [],
        "surfaces": [],
        "fragments": [],
        "targets": [],
        "source_components": [],
        "providers": [provider],
        "cohorts": [cohort],
        "participation": [],
        "bindings": [],
        "scenarios": [],
        "policies": [],
        "profiles": [],
        "runners": [],
        "waivers": [],
        "imports": [],
        "adapters": [],
        "extensions": {}
    })
}

fn profile_definition() -> Value {
    json!({
        "id": "audience.default",
        "revision": 1,
        "title": "Audience",
        "owners": ["owner://team/product"],
        "dimensions": [{
            "id": "locale",
            "values": ["en", "fr"]
        }],
        "defaults": {"locale": "en"},
        "extensions": {}
    })
}

fn comparison_document() -> Value {
    json!({
        "schema": eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string(),
        "contract_digest": digest('1'),
        "scenario_digest": digest('2'),
        "profiles": [{
            "definition": profile_definition(),
            "values": [{"dimension": "locale", "state": "known", "value": "en"}]
        }],
        "comparison_rule_digest": digest('3')
    })
}

#[test]
fn provider_manifest_schema_accepts_and_rejects_one_axis() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/manifest/provider.schema.json")?;
    let document = provider_document();
    assert!(validator.is_valid(&document));

    let mut bad_digest = document.clone();
    bad_digest["configuration_digest"] = json!(5);
    assert!(!validator.is_valid(&bad_digest));

    let mut garbage_digest = document.clone();
    garbage_digest["configuration_digest"] = json!("not-a-digest");
    assert!(!validator.is_valid(&garbage_digest));

    let mut wrong_marker = document.clone();
    wrong_marker["schema"] = json!("https://example.com/not-provider");
    assert!(!validator.is_valid(&wrong_marker));

    let mut empty_owners = document.clone();
    empty_owners["owners"] = json!([]);
    assert!(!validator.is_valid(&empty_owners));

    let mut missing = document.clone();
    missing.as_object_mut().ok_or("object")?.remove("owners");
    assert!(!validator.is_valid(&missing));

    let mut unknown = document.clone();
    unknown["unknown_member"] = json!(true);
    assert!(!validator.is_valid(&unknown));

    let mut null_owner = document.clone();
    null_owner["owners"] = json!([null]);
    assert!(!validator.is_valid(&null_owner));
    Ok(())
}

#[test]
fn cohort_manifest_schema_accepts_and_rejects_one_axis() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/manifest/cohort.schema.json")?;
    assert!(validator.is_valid(&cohort_document()));

    let mut bad_revision = cohort_document();
    bad_revision["revision"] = json!("1");
    assert!(!validator.is_valid(&bad_revision));

    let mut missing_assignment_field = cohort_document();
    missing_assignment_field["assignments"][0]
        .as_object_mut()
        .ok_or("object")?
        .remove("repository");
    assert!(!validator.is_valid(&missing_assignment_field));

    let mut wrong_axis_type = cohort_document();
    wrong_axis_type["assignments"][0]["source_content_digest"] = json!(5);
    assert!(!validator.is_valid(&wrong_axis_type));

    let mut garbage_digest = cohort_document();
    garbage_digest["assignments"][0]["source_content_digest"] = json!("garbage");
    assert!(!validator.is_valid(&garbage_digest));

    let mut null_optional = cohort_document();
    null_optional["assignments"][0]["build_id"] = json!(null);
    assert!(
        !validator.is_valid(&null_optional),
        "authored optionals are absent-or-present, never null"
    );

    let mut wrong_marker = cohort_document();
    wrong_marker["schema"] = json!("https://example.com/not-cohort");
    assert!(!validator.is_valid(&wrong_marker));

    let mut empty_assignments = cohort_document();
    empty_assignments["assignments"] = json!([]);
    assert!(!validator.is_valid(&empty_assignments));

    let mut bad_configuration = cohort_document();
    bad_configuration["assignments"][0]["provider_configuration_digest"] = json!(5);
    assert!(!validator.is_valid(&bad_configuration));

    let mut unknown_assignment_member = cohort_document();
    unknown_assignment_member["assignments"][0]["unknown_member"] = json!(true);
    assert!(!validator.is_valid(&unknown_assignment_member));
    Ok(())
}

#[test]
fn graph_schema_validates_provider_and_cohort_members() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/protocol/semantic-graph.schema.json")?;
    let graph = graph_document(provider_member(), cohort_member());
    assert!(
        validator.is_valid(&graph),
        "graph must accept provider/cohort members"
    );

    let mut bad_provider = graph.clone();
    bad_provider["providers"][0]["configuration_digest"] = json!("x");
    assert!(!validator.is_valid(&bad_provider));

    let mut empty_provider_owners = graph.clone();
    empty_provider_owners["providers"][0]["owners"] = json!([]);
    assert!(!validator.is_valid(&empty_provider_owners));

    let mut null_cohort_optional = graph.clone();
    null_cohort_optional["cohorts"][0]["assignments"][0]["source_content_digest"] = json!(null);
    assert!(!validator.is_valid(&null_cohort_optional));

    let mut bad_cohort = graph.clone();
    bad_cohort["cohorts"][0]["assignments"][0]["build_id"] = json!(5);
    assert!(!validator.is_valid(&bad_cohort));

    let mut null_provider = graph.clone();
    null_provider["providers"] = json!([null]);
    assert!(!validator.is_valid(&null_provider));

    let mut missing_collection = graph.clone();
    missing_collection
        .as_object_mut()
        .ok_or("object")?
        .remove("cohorts");
    assert!(!validator.is_valid(&missing_collection));
    Ok(())
}

#[test]
fn schema_paths_exist_and_carry_exact_ids() -> Result<(), Box<dyn Error>> {
    for (relative, expected_suffix) in [
        (
            "schemas/v2/manifest/provider.schema.json",
            "/schemas/v2/manifest/provider.schema.json",
        ),
        (
            "schemas/v2/manifest/cohort.schema.json",
            "/schemas/v2/manifest/cohort.schema.json",
        ),
        (
            "schemas/v2/protocol/semantic-graph.schema.json",
            "/schemas/v2/protocol/semantic-graph.schema.json",
        ),
        (
            "schemas/v2/protocol/comparison-context.schema.json",
            "/schemas/v2/protocol/comparison-context.schema.json",
        ),
    ] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(relative);
        let document: Value = serde_json::from_slice(&std::fs::read(&path)?)?;
        assert_eq!(
            document["$id"].as_str().ok_or("$id")?,
            format!("https://raw.githubusercontent.com/triesap/eqm/master{expected_suffix}")
        );
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// RF61: generated manifests, graph and comparison schemas must reject the
// owning value defects that the checked readers already reject.
// ---------------------------------------------------------------------------

#[test]
fn cohort_schema_enforces_every_owning_value_contract() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/manifest/cohort.schema.json")?;
    assert!(
        validator.is_valid(&cohort_document()),
        "valid cohort must be accepted"
    );

    let mut revision_zero = cohort_document();
    revision_zero["revision"] = json!(0);
    assert!(!validator.is_valid(&revision_zero), "revision 0");

    let mut empty_id = cohort_document();
    empty_id["id"] = json!("");
    assert!(!validator.is_valid(&empty_id), "empty id");

    let mut invalid_scenario = cohort_document();
    invalid_scenario["scenario"] = json!("Not A Scenario");
    assert!(!validator.is_valid(&invalid_scenario), "invalid scenario");

    let mut malformed_commit = cohort_document();
    malformed_commit["assignments"][0]["source_commit"] = json!("xyz");
    assert!(!validator.is_valid(&malformed_commit), "malformed commit");

    let mut neither = cohort_document();
    neither["assignments"][0]
        .as_object_mut()
        .ok_or("object")?
        .remove("provider_configuration_digest");
    assert!(!validator.is_valid(&neither), "neither configuration kind");

    let mut both = cohort_document();
    both["assignments"][0]["target_configuration_digest"] = json!(digest('9'));
    assert!(!validator.is_valid(&both), "both configuration kinds");

    let mut duplicate = cohort_document();
    let first = duplicate["assignments"][0].clone();
    duplicate["assignments"]
        .as_array_mut()
        .ok_or("array")?
        .push(first);
    assert!(!validator.is_valid(&duplicate), "duplicate assignments");
    Ok(())
}

#[test]
fn provider_schema_rejects_duplicate_or_invalid_owners() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/manifest/provider.schema.json")?;
    assert!(validator.is_valid(&provider_document()));

    let mut duplicate_owners = provider_document();
    duplicate_owners["owners"] = json!(["owner://team/providers", "owner://team/providers"]);
    assert!(!validator.is_valid(&duplicate_owners), "duplicate owners");

    let mut invalid_owner = provider_document();
    invalid_owner["owners"] = json!(["https://team/providers"]);
    assert!(
        !validator.is_valid(&invalid_owner),
        "invalid owner vocabulary"
    );

    let mut invalid_id = provider_document();
    invalid_id["id"] = json!("provider");
    assert!(!validator.is_valid(&invalid_id), "provider id depth");
    Ok(())
}

#[test]
fn comparison_schema_enforces_owning_profile_contracts() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/protocol/comparison-context.schema.json")?;
    assert!(
        validator.is_valid(&comparison_document()),
        "valid comparison context must be accepted"
    );

    let mut empty_title = comparison_document();
    empty_title["profiles"][0]["definition"]["title"] = json!("");
    assert!(!validator.is_valid(&empty_title), "empty title");

    let mut revision_zero = comparison_document();
    revision_zero["profiles"][0]["definition"]["revision"] = json!(0);
    assert!(!validator.is_valid(&revision_zero), "revision 0");

    let mut empty_dimensions = comparison_document();
    empty_dimensions["profiles"][0]["definition"]["dimensions"] = json!([]);
    assert!(!validator.is_valid(&empty_dimensions), "empty dimensions");

    let mut invalid_owner = comparison_document();
    invalid_owner["profiles"][0]["definition"]["owners"] = json!(["https://team/product"]);
    assert!(!validator.is_valid(&invalid_owner), "invalid owner");

    let mut empty_profiles = comparison_document();
    empty_profiles["profiles"] = json!([]);
    assert!(!validator.is_valid(&empty_profiles), "empty profiles");
    Ok(())
}

#[test]
fn graph_schema_enforces_provider_and_cohort_value_contracts() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/protocol/semantic-graph.schema.json")?;
    assert!(validator.is_valid(&graph_document(provider_member(), cohort_member())));

    let mut duplicate_owners = graph_document(provider_member(), cohort_member());
    duplicate_owners["providers"][0]["owners"] = json!(["owner://team/a", "owner://team/a"]);
    assert!(!validator.is_valid(&duplicate_owners));

    let mut both_config = graph_document(provider_member(), cohort_member());
    both_config["cohorts"][0]["assignments"][0]["target_configuration_digest"] = json!(digest('9'));
    assert!(!validator.is_valid(&both_config));

    let mut neither_config = graph_document(provider_member(), cohort_member());
    neither_config["cohorts"][0]["assignments"][0]
        .as_object_mut()
        .ok_or("object")?
        .remove("provider_configuration_digest");
    assert!(!validator.is_valid(&neither_config));

    let mut revision_zero = graph_document(provider_member(), cohort_member());
    revision_zero["cohorts"][0]["revision"] = json!(0);
    assert!(!validator.is_valid(&revision_zero));
    Ok(())
}

#[test]
fn extension_union_rejects_unknown_members_in_every_branch() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/protocol/comparison-context.schema.json")?;

    let mut unknown_member = comparison_document();
    unknown_member["profiles"][0]["definition"]["extensions"] = json!({
        "dev.example.ext": {"type": "boolean", "value": true, "extra": 1}
    });
    assert!(!validator.is_valid(&unknown_member));

    let mut valid = comparison_document();
    valid["profiles"][0]["definition"]["extensions"] = json!({
        "dev.example.ext": {"type": "boolean", "value": true}
    });
    assert!(validator.is_valid(&valid));
    Ok(())
}
