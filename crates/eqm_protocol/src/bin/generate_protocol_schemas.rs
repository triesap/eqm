//! Deterministically generate protocol-owned JSON Schemas for one explicit
//! version.
//!
//! The generator is invoked once per coordinated schema version. Version 2
//! registers the five concrete protocol families in S015-R3; until then its
//! inventory is explicitly empty rather than a set of placeholder documents.

use eqm_domain::{SchemaKind, SchemaUri, SchemaVersion};
use eqm_protocol::*;
use schemars::JsonSchema;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::Path;

fn write<T: JsonSchema>(root: &Path, name: &str, id: &str) -> Result<(), Box<dyn Error>> {
    let mut value = serde_json::to_value(schemars::schema_for!(T))?;
    let object = value
        .as_object_mut()
        .ok_or("schema root is not an object")?;
    object.insert("$id".to_owned(), Value::String(id.to_owned()));
    let mut bytes = serde_json::to_vec_pretty(&value)?;
    bytes.push(b'\n');
    fs::create_dir_all(root)?;
    fs::write(root.join(format!("{name}.schema.json")), bytes)?;
    Ok(())
}

fn write_semantic_graph(root: &Path) -> Result<(), Box<dyn Error>> {
    let arrays = [
        "adapters",
        "bindings",
        "capabilities",
        "fragments",
        "imports",
        "journeys",
        "policies",
        "profiles",
        "runners",
        "surfaces",
        "targets",
        "waivers",
    ];
    let mut properties = serde_json::Map::new();
    for name in arrays {
        properties.insert(name.to_owned(), serde_json::json!({"type": "array"}));
    }
    properties.insert(
        "extensions".to_owned(),
        serde_json::json!({"type": "object"}),
    );
    properties.insert(
        "schema".to_owned(),
        serde_json::json!({"const": SchemaUri::new(SchemaKind::SemanticGraph).to_string()}),
    );
    let value = serde_json::json!({
        "$id": SchemaUri::new(SchemaKind::SemanticGraph).to_string(),
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "additionalProperties": false,
        "description": "Canonical EQM semantic graph projection.",
        "properties": properties,
        "required": [
            "adapters", "bindings", "capabilities", "extensions", "fragments", "imports",
            "journeys", "policies", "profiles", "runners", "schema", "surfaces", "targets",
            "waivers"
        ],
        "title": "SemanticGraph",
        "type": "object"
    });
    let mut bytes = serde_json::to_vec_pretty(&value)?;
    bytes.push(b'\n');
    fs::create_dir_all(root)?;
    fs::write(root.join("semantic-graph.schema.json"), bytes)?;
    Ok(())
}

fn write_semantic_graph_v2(root: &Path) -> Result<(), Box<dyn Error>> {
    let arrays = [
        "adapters",
        "bindings",
        "capabilities",
        "fragments",
        "imports",
        "journeys",
        "participation",
        "policies",
        "profiles",
        "runners",
        "scenarios",
        "source_components",
        "surfaces",
        "targets",
        "waivers",
    ];
    let mut properties = serde_json::Map::new();
    for name in arrays {
        properties.insert(name.to_owned(), serde_json::json!({"type": "array"}));
    }
    properties.insert(
        "extensions".to_owned(),
        serde_json::json!({"type": "object"}),
    );
    properties.insert(
        "schema".to_owned(),
        serde_json::json!({
            "const": SchemaUri::versioned(SchemaKind::SemanticGraph, SchemaVersion::V2)
                .to_string()
        }),
    );
    let required = [
        "adapters",
        "bindings",
        "capabilities",
        "extensions",
        "fragments",
        "imports",
        "journeys",
        "participation",
        "policies",
        "profiles",
        "runners",
        "scenarios",
        "schema",
        "source_components",
        "surfaces",
        "targets",
        "waivers",
    ];
    let value = serde_json::json!({
        "$id": SchemaUri::versioned(SchemaKind::SemanticGraph, SchemaVersion::V2).to_string(),
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "additionalProperties": false,
        "description": "Complete canonical EQM v2 semantic graph projection.",
        "properties": properties,
        "required": required,
        "title": "SemanticGraphV2",
        "type": "object"
    });
    let mut bytes = serde_json::to_vec_pretty(&value)?;
    bytes.push(b'\n');
    fs::create_dir_all(root)?;
    fs::write(root.join("semantic-graph.schema.json"), bytes)?;
    Ok(())
}

fn write_comparison_context_v2(root: &Path) -> Result<(), Box<dyn Error>> {
    let value = serde_json::json!({
        "$id": SchemaUri::versioned(SchemaKind::ComparisonContext, SchemaVersion::V2).to_string(),
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "additionalProperties": false,
        "description": "Shared EQM v2 comparison-context projection.",
        "properties": {
            "comparison_rule_digest": {"type": "string"},
            "contract_digest": {"type": "string"},
            "profiles": {"type": "array"},
            "scenario_digest": {"type": "string"},
            "schema": {
                "const": SchemaUri::versioned(SchemaKind::ComparisonContext, SchemaVersion::V2)
                    .to_string()
            }
        },
        "required": [
            "comparison_rule_digest",
            "contract_digest",
            "profiles",
            "scenario_digest",
            "schema"
        ],
        "title": "ComparisonContextV2",
        "type": "object"
    });
    let mut bytes = serde_json::to_vec_pretty(&value)?;
    bytes.push(b'\n');
    fs::create_dir_all(root)?;
    fs::write(root.join("comparison-context.schema.json"), bytes)?;
    Ok(())
}

fn write_v1(root: &Path) -> Result<(), Box<dyn Error>> {
    write::<ReportEnvelope<ValidateResultDto, EvidenceSubjectDto, EvidenceSubjectDto>>(
        root,
        "result",
        &RESULT_SCHEMA.to_string(),
    )?;
    write::<DiagnosticDto>(root, "diagnostic", &DIAGNOSTIC_SCHEMA.to_string())?;
    write::<TestResultDto>(root, "test-result", &TEST_RESULT_SCHEMA.to_string())?;
    write::<EvidenceResultDto>(root, "evidence-result", &EVIDENCE_RESULT_SCHEMA.to_string())?;
    write::<InventoryDto>(root, "inventory", &INVENTORY_SCHEMA.to_string())?;
    write::<RuntimeFactsDto>(root, "runtime-facts", &RUNTIME_FACTS_SCHEMA.to_string())?;
    write::<ReleaseRecordDto>(root, "release-record", &RELEASE_RECORD_SCHEMA.to_string())?;
    write_semantic_graph(root)?;
    write::<AttestationPredicateDto>(root, "attestation", &ATTESTATION_SCHEMA.to_string())?;
    write::<AdapterRequestDto>(root, "adapter-request", &ADAPTER_REQUEST_SCHEMA.to_string())?;
    write::<AdapterResponseDto>(
        root,
        "adapter-response",
        &ADAPTER_RESPONSE_SCHEMA.to_string(),
    )?;
    Ok(())
}

fn write_v2(root: &Path) -> Result<(), Box<dyn Error>> {
    write::<v2::EvidenceResultV2Dto>(
        root,
        "evidence-result",
        &EVIDENCE_RESULT_SCHEMA_V2.to_string(),
    )?;
    write::<v2::RuntimeFactsV2Dto>(root, "runtime-facts", &RUNTIME_FACTS_SCHEMA_V2.to_string())?;
    write::<v2::InventoryV2Dto>(root, "inventory", &INVENTORY_SCHEMA_V2.to_string())?;
    write::<v2::AdapterRequestV2Dto>(
        root,
        "adapter-request",
        &ADAPTER_REQUEST_SCHEMA_V2.to_string(),
    )?;
    write::<v2::AdapterResponseV2Dto>(
        root,
        "adapter-response",
        &ADAPTER_RESPONSE_SCHEMA_V2.to_string(),
    )?;
    write::<v2::SubjectManifestV2Dto>(
        root,
        "subject-manifest",
        &SUBJECT_MANIFEST_SCHEMA_V2.to_string(),
    )?;
    write_semantic_graph_v2(root)?;
    write_comparison_context_v2(root)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = std::env::args().skip(1);
    let root = arguments.next().ok_or("output directory required")?;
    let version = arguments.next().ok_or("schema version required")?;
    let version: SchemaVersion = version.parse().map_err(|_| "unsupported schema version")?;
    let root = Path::new(&root);
    match version {
        SchemaVersion::V1 => write_v1(root)?,
        SchemaVersion::V2 => write_v2(root)?,
    }
    Ok(())
}
