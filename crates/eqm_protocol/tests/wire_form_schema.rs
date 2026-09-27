//! Confirms the generated `WireUint`/`WireInt` primitive schemas against a
//! conforming Draft 2020-12 validator.
//!
//! Two layers are kept distinct and never conflated:
//!
//! * the JSON-value schema layer decides values, so an integral number written
//!   as `1e3` or `1000.0` is an integer for JSON Schema; and
//! * the raw canonical-token layer, implemented by the runtime parser, rejects
//!   fractional/exponent spellings regardless of their numeric value.
//!
//! Each corpus token therefore carries an independent schema expectation and
//! parser expectation instead of assuming the two agree.

use eqm_protocol::{MAX_SAFE_INTEGER, WireInt, WireUint};
use jsonschema::{Draft, Validator};
use schemars::{JsonSchema, SchemaGenerator};
use serde_json::Value;
use std::error::Error;

fn validator<T: JsonSchema>() -> Result<Validator, Box<dyn Error>> {
    let schema = serde_json::to_value(T::json_schema(&mut SchemaGenerator::default()))?;
    Ok(jsonschema::options()
        .with_draft(Draft::Draft202012)
        .should_validate_formats(true)
        .build(&schema)?)
}

fn assert_layers(
    validator: &Validator,
    token: &str,
    schema_expected: bool,
    parser_expected: bool,
    parse: impl Fn(&str) -> bool,
) -> Result<(), Box<dyn Error>> {
    let value: Value = serde_json::from_str(token)?;
    assert_eq!(
        validator.is_valid(&value),
        schema_expected,
        "schema layer disagreed for {token}"
    );
    assert_eq!(
        parse(token),
        parser_expected,
        "raw-token layer disagreed for {token}"
    );
    Ok(())
}

#[test]
fn unsigned_primitives_separate_value_and_token_layers() -> Result<(), Box<dyn Error>> {
    let validator = validator::<WireUint>()?;
    let over_safe = (MAX_SAFE_INTEGER + 1).to_string();
    let corpus: Vec<(String, bool, bool)> = vec![
        ("0".to_owned(), true, true),
        (MAX_SAFE_INTEGER.to_string(), true, true),
        (over_safe.clone(), false, false),
        ("-1".to_owned(), false, false),
        ("1.5".to_owned(), false, false),
        // Integral JSON values spelled with an exponent or fraction: the schema
        // value layer accepts them, the raw token layer does not.
        ("1e3".to_owned(), true, false),
        ("1000.0".to_owned(), true, false),
        ("\"0\"".to_owned(), false, false),
        ("\"5\"".to_owned(), false, false),
        (format!("\"{MAX_SAFE_INTEGER}\""), false, false),
        (format!("\"{over_safe}\""), true, true),
        ("\"18446744073709551615\"".to_owned(), true, true),
        ("\"18446744073709551616\"".to_owned(), false, false),
        ("\"01\"".to_owned(), false, false),
        ("\"+1\"".to_owned(), false, false),
        ("\"9999999999999999\"".to_owned(), true, true),
        ("\"10000000000000000\"".to_owned(), true, true),
    ];
    for (token, schema_expected, parser_expected) in &corpus {
        assert_layers(
            &validator,
            token,
            *schema_expected,
            *parser_expected,
            |token| serde_json::from_str::<WireUint>(token).is_ok(),
        )?;
    }
    Ok(())
}

#[test]
fn signed_primitives_separate_value_and_token_layers() -> Result<(), Box<dyn Error>> {
    let validator = validator::<WireInt>()?;
    let positive_over_safe = (MAX_SAFE_INTEGER + 1).to_string();
    let negative_over_safe = format!("-{}", MAX_SAFE_INTEGER + 1);
    let corpus: Vec<(String, bool, bool)> = vec![
        ("0".to_owned(), true, true),
        (format!("-{MAX_SAFE_INTEGER}"), true, true),
        (MAX_SAFE_INTEGER.to_string(), true, true),
        (positive_over_safe.clone(), false, false),
        (negative_over_safe.clone(), false, false),
        ("1.5".to_owned(), false, false),
        ("1e3".to_owned(), true, false),
        ("\"0\"".to_owned(), false, false),
        ("\"5\"".to_owned(), false, false),
        ("\"-5\"".to_owned(), false, false),
        ("\"-0\"".to_owned(), false, false),
        (format!("\"{MAX_SAFE_INTEGER}\""), false, false),
        (format!("\"-{MAX_SAFE_INTEGER}\""), false, false),
        (format!("\"{positive_over_safe}\""), true, true),
        (format!("\"{negative_over_safe}\""), true, true),
        ("\"9223372036854775807\"".to_owned(), true, true),
        ("\"-9223372036854775808\"".to_owned(), true, true),
        ("\"9223372036854775808\"".to_owned(), false, false),
        ("\"-9223372036854775809\"".to_owned(), false, false),
        ("\"+9007199254740992\"".to_owned(), false, false),
    ];
    for (token, schema_expected, parser_expected) in &corpus {
        assert_layers(
            &validator,
            token,
            *schema_expected,
            *parser_expected,
            |token| serde_json::from_str::<WireInt>(token).is_ok(),
        )?;
    }
    Ok(())
}
