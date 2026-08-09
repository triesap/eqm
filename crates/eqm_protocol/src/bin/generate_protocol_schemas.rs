//! Deterministically generate protocol-owned JSON Schemas.

use eqm_domain::{SchemaKind, SchemaUri};
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
    fs::write(root.join("semantic-graph.schema.json"), bytes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = std::env::args().nth(1).ok_or("output directory required")?;
    let root = Path::new(&root);
    fs::create_dir_all(root)?;
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
