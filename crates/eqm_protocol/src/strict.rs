//! Bounded strict JSON ingestion for protocol records.
//!
//! Protocol records must never be normalized through an ambiguous
//! last-key-wins parse. This module parses JSON into a value while rejecting
//! duplicate object member names, rejects oversized input before parsing, and
//! leaves the original bytes untouched so callers can retain and verify them.

use serde::Deserialize;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Maximum accepted serialized protocol record size.
pub const MAX_JSON_BYTES: usize = 8 * 1024 * 1024;

/// Marker carried by the duplicate-member error so it can be classified after
/// Serde erases the concrete error type.
const DUPLICATE_MARKER: &str = "eqm.strict.duplicate-member";

/// Strict JSON ingestion failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrictJsonError {
    /// Input exceeded the accepted byte bound.
    TooLarge,
    /// Input was not valid JSON or exceeded the recursion bound.
    Syntax,
    /// An object declared the same member name more than once.
    DuplicateMember,
    /// The value did not match the requested protocol record.
    Conversion,
}

impl Display for StrictJsonError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::TooLarge => "protocol JSON exceeded the accepted size bound",
            Self::Syntax => "protocol JSON was malformed or too deeply nested",
            Self::DuplicateMember => "protocol JSON declared a duplicate object member",
            Self::Conversion => "protocol JSON did not match the requested record",
        };
        formatter.write_str(message)
    }
}

impl Error for StrictJsonError {}

/// Parses bounded JSON, rejecting duplicate object member names.
pub fn parse_strict(bytes: &[u8]) -> Result<Value, StrictJsonError> {
    if bytes.len() > MAX_JSON_BYTES {
        return Err(StrictJsonError::TooLarge);
    }
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = StrictValue::deserialize(&mut deserializer).map_err(|error| {
        if error.to_string().contains(DUPLICATE_MARKER) {
            StrictJsonError::DuplicateMember
        } else {
            StrictJsonError::Syntax
        }
    })?;
    deserializer.end().map_err(|_| StrictJsonError::Syntax)?;
    Ok(value.0)
}

/// Parses bounded strict JSON and converts it into a typed protocol record.
pub fn from_slice<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, StrictJsonError> {
    let value = parse_strict(bytes)?;
    from_value(value)
}

/// Converts an already strictly-parsed value into a typed protocol record.
pub fn from_value<T: for<'de> Deserialize<'de>>(value: Value) -> Result<T, StrictJsonError> {
    serde_json::from_value(value).map_err(|_| StrictJsonError::Conversion)
}

/// A JSON value parsed with duplicate-member rejection.
struct StrictValue(Value);

impl<'de> Deserialize<'de> for StrictValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictVisitor)
    }
}

struct StrictVisitor;

impl<'de> Visitor<'de> for StrictVisitor {
    type Value = StrictValue;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded strict JSON value")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::Bool(value)))
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::Number(Number::from(value))))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::Number(Number::from(value))))
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
        Number::from_f64(value)
            .map(|number| StrictValue(Value::Number(number)))
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::String(value.to_owned())))
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::String(value)))
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::Null))
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::Null))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        StrictValue::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element::<StrictValue>()? {
            values.push(value.0);
        }
        Ok(StrictValue(Value::Array(values)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut members = Map::new();
        let mut seen = BTreeSet::new();
        while let Some(name) = map.next_key::<String>()? {
            if !seen.insert(name.clone()) {
                return Err(de::Error::custom(DUPLICATE_MARKER));
            }
            let value = map.next_value::<StrictValue>()?;
            members.insert(name, value.0);
        }
        Ok(StrictValue(Value::Object(members)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_members_are_rejected_at_every_depth() {
        assert_eq!(
            parse_strict(br#"{"a":1,"a":2}"#),
            Err(StrictJsonError::DuplicateMember)
        );
        assert_eq!(
            parse_strict(br#"{"outer":{"a":1,"a":2}}"#),
            Err(StrictJsonError::DuplicateMember)
        );
        assert_eq!(
            parse_strict(br#"{"list":[{"a":1,"a":2}]}"#),
            Err(StrictJsonError::DuplicateMember)
        );
    }

    #[test]
    fn distinct_members_and_arrays_parse_unchanged() -> Result<(), Box<dyn std::error::Error>> {
        let value = parse_strict(br#"{"b":2,"a":[1,2],"n":null}"#)?;
        assert!(value.get("a").is_some());
        assert!(value.get("b").is_some());
        assert!(value.get("n").is_some_and(Value::is_null));
        Ok(())
    }

    #[test]
    fn malformed_and_oversized_input_fail_closed() {
        assert_eq!(parse_strict(b"{"), Err(StrictJsonError::Syntax));
        assert_eq!(parse_strict(b""), Err(StrictJsonError::Syntax));
        let oversized = vec![b' '; MAX_JSON_BYTES + 1];
        assert_eq!(parse_strict(&oversized), Err(StrictJsonError::TooLarge));
    }

    #[test]
    fn deeply_nested_input_is_bounded() {
        let mut text = String::new();
        for _ in 0..400 {
            text.push('[');
        }
        for _ in 0..400 {
            text.push(']');
        }
        assert_eq!(parse_strict(text.as_bytes()), Err(StrictJsonError::Syntax));
    }
}
