//! Strict protocol ingestion regressions: duplicate members never win.

use eqm_protocol::{EvidenceDtoError, EvidenceResultDto, StrictJsonError, parse_strict};

#[test]
fn duplicate_top_level_member_is_rejected_before_conversion() {
    let json = br#"{"schema":1,"schema":2}"#;
    assert_eq!(
        EvidenceResultDto::from_json(json),
        Err(EvidenceDtoError::Json)
    );
}

#[test]
fn duplicate_nested_member_is_rejected_before_conversion() {
    let json = br#"{"subject":{"target":"a","target":"b"}}"#;
    assert_eq!(
        EvidenceResultDto::from_json(json),
        Err(EvidenceDtoError::Json)
    );
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
