//! Shared helpers for the separate v2 component projections.
//!
//! These encode the DEV-005 representation rules that the binding and
//! evidence-specification projections share: approved v2 integer wire forms,
//! canonical member-byte set ordering with duplicate rejection, and extension
//! projection that excludes only `dev.equivalencematrix.display` while
//! preserving ordered extension arrays.

use super::CanonicalizationError;
use eqm_domain::ExtensionValue;
use serde_json::{Map, Value, json};

/// The largest integer that survives an IEEE-754 double exactly.
pub(super) const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// Returns the approved v2 canonical unsigned integer form.
pub(super) fn wire_uint(value: u64) -> Value {
    if value <= MAX_SAFE_INTEGER {
        json!(value)
    } else {
        json!(value.to_string())
    }
}

/// Returns the approved v2 canonical signed integer form.
pub(super) fn wire_int(value: i64) -> Value {
    if value.unsigned_abs() <= MAX_SAFE_INTEGER {
        json!(value)
    } else {
        json!(value.to_string())
    }
}

/// Canonically orders set members by complete JCS member bytes and rejects
/// exact duplicates instead of silently erasing them.
pub(super) fn canonical_set(
    members: impl IntoIterator<Item = Value>,
) -> Result<Value, CanonicalizationError> {
    let mut encoded: Vec<(Vec<u8>, Value)> = Vec::new();
    for member in members {
        let bytes = serde_json_canonicalizer::to_vec(&member)
            .map_err(|_| CanonicalizationError::Serialization)?;
        encoded.push((bytes, member));
    }
    encoded.sort_by(|left, right| left.0.cmp(&right.0));
    for window in encoded.windows(2) {
        if window[0].0 == window[1].0 {
            return Err(CanonicalizationError::DuplicateMember);
        }
    }
    Ok(Value::Array(
        encoded.into_iter().map(|(_, member)| member).collect(),
    ))
}

/// Canonically orders a set of authored string members.
pub(super) fn canonical_strings(
    values: impl IntoIterator<Item = String>,
) -> Result<Value, CanonicalizationError> {
    canonical_set(values.into_iter().map(Value::String))
}

/// Projects extensions with v2 integer forms, excluding only the display
/// namespace. Every extension value is recursively encoded as an object with
/// exactly `type` and `value`, so an integer and a digit-containing string can
/// never collapse to the same canonical bytes. Nested extension arrays keep
/// their declared order; object members are JCS-keyed maps.
pub(super) fn extensions_v2(
    values: &eqm_domain::Extensions,
) -> Result<Value, CanonicalizationError> {
    let mut projected = Map::new();
    for (namespace, value) in values.values() {
        if namespace.is_display_only() {
            continue;
        }
        projected.insert(namespace.as_str().to_owned(), extension_value_v2(value)?);
    }
    Ok(Value::Object(projected))
}

fn tagged(kind: &str, payload: Value) -> Value {
    let mut encoded = Map::new();
    encoded.insert("type".to_owned(), Value::String(kind.to_owned()));
    encoded.insert("value".to_owned(), payload);
    Value::Object(encoded)
}

fn extension_value_v2(value: &ExtensionValue) -> Result<Value, CanonicalizationError> {
    Ok(match value {
        ExtensionValue::Boolean(value) => tagged("boolean", json!(value)),
        ExtensionValue::Integer(value) => tagged("integer", wire_int(*value)),
        ExtensionValue::String(value) => tagged("string", json!(value)),
        ExtensionValue::Array(values) => tagged(
            "array",
            Value::Array(
                values
                    .iter()
                    .map(extension_value_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        ),
        ExtensionValue::Object(values) => {
            let mut projected = Map::new();
            for (key, value) in values {
                projected.insert(key.as_str().to_owned(), extension_value_v2(value)?);
            }
            tagged("object", Value::Object(projected))
        }
    })
}
