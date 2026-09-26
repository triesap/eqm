//! Strict protocol ingestion regressions: bounded limits and duplicate members.

use eqm_protocol::{
    ADAPTER_RESPONSE_SCHEMA, AdapterDtoError, AdapterResponseDto, AdapterStatusDto,
    EvidenceSubjectDto, FactValueDto, INVENTORY_SCHEMA, InventoryDto, InventoryEntryDto,
    StrictJsonError, parse_strict, parse_strict_limited,
};
use std::collections::BTreeMap;

const DIGEST: &str = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

fn subject() -> EvidenceSubjectDto {
    EvidenceSubjectDto {
        repository: "https://example.com/product".to_owned(),
        repository_id_digest: DIGEST.to_owned(),
        scope: eqm_protocol::ScopeSubjectDto::Target {
            target: "web".to_owned(),
        },
        source_commit: "a".repeat(40),
        build_id: None,
        artifact_digest: None,
        target_configuration_digest: DIGEST.to_owned(),
    }
}

fn inventory_with_padding(padding: usize) -> InventoryDto {
    InventoryDto {
        schema: INVENTORY_SCHEMA.to_string(),
        adapter: "adapter.compose_export".to_owned(),
        adapter_digest: DIGEST.to_owned(),
        subject: subject(),
        target: "web".to_owned(),
        generated_at: "2026-01-02T03:04:05Z".to_owned(),
        completeness: "complete".to_owned(),
        entries: vec![InventoryEntryDto {
            kind: "navigation".to_owned(),
            key: "home".to_owned(),
            attributes: BTreeMap::from([(
                "framework".to_owned(),
                FactValueDto::Text("x".repeat(padding)),
            )]),
            source: "build/eqm/navigation.json".to_owned(),
        }],
        diagnostics: Vec::new(),
        inventory_digest: DIGEST.to_owned(),
    }
}

fn response_with_padding(padding: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = AdapterResponseDto {
        schema: ADAPTER_RESPONSE_SCHEMA.to_string(),
        request_id: "run-1".to_owned(),
        adapter: "adapter.compose_export".to_owned(),
        adapter_digest: DIGEST.to_owned(),
        status: AdapterStatusDto::Ok,
        inventory: Some(inventory_with_padding(padding)),
        diagnostics: Vec::new(),
    };
    Ok(serde_json::to_vec(&response)?)
}

#[test]
fn valid_adapter_response_between_eight_and_sixteen_mib_is_accepted()
-> Result<(), Box<dyn std::error::Error>> {
    let bytes = response_with_padding(9 * 1024 * 1024)?;
    assert!(bytes.len() > 8 * 1024 * 1024);
    assert!(bytes.len() <= 16 * 1024 * 1024);
    let dto = AdapterResponseDto::from_json(&bytes)?;
    assert_eq!(dto.status, AdapterStatusDto::Ok);
    Ok(())
}

#[test]
fn adapter_response_beyond_its_declared_bound_is_rejected() -> Result<(), Box<dyn std::error::Error>>
{
    let bytes = response_with_padding(17 * 1024 * 1024)?;
    assert!(bytes.len() > 16 * 1024 * 1024);
    assert_eq!(
        AdapterResponseDto::from_json(&bytes),
        Err(AdapterDtoError::InvalidLimits)
    );
    Ok(())
}

#[test]
fn explicit_limits_accept_at_the_boundary_and_reject_beyond_it()
-> Result<(), Box<dyn std::error::Error>> {
    let exact = br#"{"a":1}"#;
    assert!(parse_strict_limited(exact, exact.len()).is_ok());
    assert_eq!(
        parse_strict_limited(exact, exact.len() - 1),
        Err(StrictJsonError::TooLarge)
    );
    Ok(())
}

#[test]
fn valid_inventory_control_parses_and_duplicate_nested_attribute_is_rejected()
-> Result<(), Box<dyn std::error::Error>> {
    let valid = serde_json::to_string(&inventory_with_padding(0))?;
    assert!(InventoryDto::from_json(valid.as_bytes()).is_ok());

    let needle = "\"framework\":{\"type\":\"text\",\"value\":\"\"}";
    assert!(
        valid.contains(needle),
        "unexpected compact encoding: {valid}"
    );
    let duplicated = valid.replace(
        needle,
        "\"framework\":{\"type\":\"text\",\"value\":\"\"},\"framework\":{\"type\":\"text\",\"value\":\"\"}",
    );
    assert_ne!(duplicated, valid);
    assert_eq!(
        InventoryDto::from_json(duplicated.as_bytes()),
        Err(AdapterDtoError::Json)
    );
    Ok(())
}

#[test]
fn duplicate_top_level_member_is_rejected_before_conversion() {
    let json = br#"{"schema":1,"schema":2}"#;
    assert_eq!(
        eqm_protocol::EvidenceResultDto::from_json(json),
        Err(eqm_protocol::EvidenceDtoError::Json)
    );
}

#[test]
fn duplicate_nested_member_is_rejected_before_conversion() {
    let json = br#"{"subject":{"target":"a","target":"b"}}"#;
    assert_eq!(
        eqm_protocol::EvidenceResultDto::from_json(json),
        Err(eqm_protocol::EvidenceDtoError::Json)
    );
}

#[test]
fn escaped_spelling_of_a_member_name_is_still_a_duplicate() {
    let json = br#"{"a":1,"\u0061":2}"#;
    assert_eq!(parse_strict(json), Err(StrictJsonError::DuplicateMember));
}

#[test]
fn duplicate_members_inside_arrays_are_rejected() {
    let json = br#"{"profiles":[{"id":"a","id":"b"}]}"#;
    assert_eq!(parse_strict(json), Err(StrictJsonError::DuplicateMember));
}

#[test]
fn distinct_members_parse_and_original_bytes_are_not_modified()
-> Result<(), Box<dyn std::error::Error>> {
    let json = br#"{"b":2,"a":1}"#.to_vec();
    let original = json.clone();
    let value = parse_strict(&json)?;
    assert_eq!(value.get("a").and_then(serde_json::Value::as_u64), Some(1));
    assert_eq!(value.get("b").and_then(serde_json::Value::as_u64), Some(2));
    assert_eq!(json, original);
    Ok(())
}
