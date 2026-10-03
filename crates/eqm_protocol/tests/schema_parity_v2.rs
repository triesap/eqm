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

#[test]
fn provider_manifest_schema_accepts_and_rejects_one_axis() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/manifest/provider.schema.json")?;
    let document = provider_document();
    assert!(validator.is_valid(&document));

    let mut bad_digest = document.clone();
    bad_digest["configuration_digest"] = json!(5);
    assert!(!validator.is_valid(&bad_digest));

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

    let mut unknown_assignment_member = cohort_document();
    unknown_assignment_member["assignments"][0]["unknown_member"] = json!(true);
    assert!(!validator.is_valid(&unknown_assignment_member));

    let mut unknown_axis = cohort_document();
    unknown_axis["assignments"][0]["target_configuration_digest"] = json!(digest('9'));
    assert!(
        !validator.is_valid(&unknown_axis),
        "authored cohort cannot declare graph-backed configuration"
    );
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
