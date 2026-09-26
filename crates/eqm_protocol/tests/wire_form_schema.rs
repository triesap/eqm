//! Confirms the generated v2 primitive schemas encode the same numeric forms
//! the runtime `WireUint`/`WireInt` parsers accept.
//!
//! Each primitive schema is the frozen `anyOf` of an in-range JSON integer
//! branch and a canonical out-of-range decimal-string branch. This test decides
//! both branches directly (numeric comparison for integers, the generated range
//! pattern for strings) over the same boundary corpus that drives the parser,
//! so a schema/parser disagreement fails rather than passing on prose.

use eqm_protocol::{MAX_SAFE_INTEGER, WireInt, WireUint};
use regex_lite::Regex;
use schemars::{JsonSchema, SchemaGenerator};
use serde_json::Value;
use std::error::Error;

/// Returns the JSON integer branch and string branch of a primitive schema.
fn branches(schema: &Value) -> Option<(&Value, &Value)> {
    let branches = schema.get("anyOf")?.as_array()?;
    Some((branches.first()?, branches.get(1)?))
}

/// Decides the two-branch primitive schema for one JSON token.
fn schema_accepts(schema: &Value, token: &str) -> Option<bool> {
    let (integer, string) = branches(schema)?;
    match serde_json::from_str::<Value>(token).ok()? {
        Value::Number(number) => {
            let Some(value) = number.as_i64() else {
                // Fractional or exponent tokens never match the integer branch.
                return Some(false);
            };
            let minimum = integer.get("minimum")?.as_i64()?;
            let maximum = integer.get("maximum")?.as_i64()?;
            Some(value >= minimum && value <= maximum)
        }
        Value::String(text) => {
            let pattern = string.get("pattern")?.as_str()?;
            Some(Regex::new(pattern).ok()?.is_match(&text))
        }
        _ => Some(false),
    }
}

fn generated<T: JsonSchema>(generator: &mut SchemaGenerator) -> Result<Value, Box<dyn Error>> {
    Ok(serde_json::to_value(T::json_schema(generator))?)
}

fn assert_corpus_agrees(
    schema: &Value,
    corpus: &[(&str, bool)],
    parse: impl Fn(&str) -> bool,
) -> Result<(), Box<dyn Error>> {
    for (token, expected) in corpus {
        assert_eq!(parse(token), *expected, "parser disagreed for {token}");
        let accepted = schema_accepts(schema, token)
            .ok_or_else(|| format!("primitive schema could not decide the token {token}"))?;
        assert_eq!(accepted, *expected, "schema disagreed for {token}");
    }
    Ok(())
}

#[test]
fn unsigned_primitive_schema_agrees_with_the_wire_parser() -> Result<(), Box<dyn Error>> {
    let schema = generated::<WireUint>(&mut SchemaGenerator::default())?;
    let over_safe = (MAX_SAFE_INTEGER + 1).to_string();
    let corpus: Vec<(String, bool)> = vec![
        ("0".to_owned(), true),
        (MAX_SAFE_INTEGER.to_string(), true),
        (over_safe.clone(), false),
        ("-1".to_owned(), false),
        ("1.5".to_owned(), false),
        ("1e3".to_owned(), false),
        ("\"0\"".to_owned(), false),
        ("\"5\"".to_owned(), false),
        (format!("\"{MAX_SAFE_INTEGER}\""), false),
        (format!("\"{over_safe}\""), true),
        ("\"18446744073709551615\"".to_owned(), true),
        ("\"18446744073709551616\"".to_owned(), false),
        ("\"01\"".to_owned(), false),
        ("\"+1\"".to_owned(), false),
        (format!("\"000{over_safe}\""), false),
        ("\"9999999999999999\"".to_owned(), true),
        ("\"10000000000000000\"".to_owned(), true),
    ];
    let corpus: Vec<(&str, bool)> = corpus
        .iter()
        .map(|(token, expected)| (token.as_str(), *expected))
        .collect();
    assert_corpus_agrees(&schema, &corpus, |token| {
        serde_json::from_str::<WireUint>(token).is_ok()
    })
}

#[test]
fn signed_primitive_schema_agrees_with_the_wire_parser() -> Result<(), Box<dyn Error>> {
    let schema = generated::<WireInt>(&mut SchemaGenerator::default())?;
    let positive_over_safe = (MAX_SAFE_INTEGER + 1).to_string();
    let negative_over_safe = format!("-{}", MAX_SAFE_INTEGER + 1);
    let corpus: Vec<(String, bool)> = vec![
        ("0".to_owned(), true),
        (format!("-{MAX_SAFE_INTEGER}"), true),
        (MAX_SAFE_INTEGER.to_string(), true),
        (positive_over_safe.clone(), false),
        (negative_over_safe.clone(), false),
        ("1.5".to_owned(), false),
        ("1e3".to_owned(), false),
        ("\"0\"".to_owned(), false),
        ("\"5\"".to_owned(), false),
        ("\"-5\"".to_owned(), false),
        ("\"-0\"".to_owned(), false),
        (format!("\"{MAX_SAFE_INTEGER}\""), false),
        (format!("\"-{MAX_SAFE_INTEGER}\""), false),
        (format!("\"{positive_over_safe}\""), true),
        (format!("\"{negative_over_safe}\""), true),
        ("\"9223372036854775807\"".to_owned(), true),
        ("\"-9223372036854775808\"".to_owned(), true),
        ("\"9223372036854775808\"".to_owned(), false),
        ("\"-9223372036854775809\"".to_owned(), false),
        ("\"+9007199254740992\"".to_owned(), false),
    ];
    let corpus: Vec<(&str, bool)> = corpus
        .iter()
        .map(|(token, expected)| (token.as_str(), *expected))
        .collect();
    assert_corpus_agrees(&schema, &corpus, |token| {
        serde_json::from_str::<WireInt>(token).is_ok()
    })
}
