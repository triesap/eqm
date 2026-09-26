//! Canonical numeric wire forms frozen by ADR-0013.
//!
//! Exact integers are JSON integers within `|n| <= 2^53-1`. The `eqm:v2`
//! namespace additionally carries larger exact quantities as unsigned
//! (`^(0|[1-9][0-9]*)$`) or signed (`^-?(0|[1-9][0-9]*)$`) decimal strings;
//! [`WireUint`] and [`WireInt`] implement that canonical pair. The current v1
//! contract remains integer-only and rejects values beyond the exact range
//! instead of silently accepting an ambiguous number. Fractional, overflowing
//! and non-canonical forms are rejected everywhere.

use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{Map, Value, json};
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// The largest integer that survives an IEEE-754 double exactly.
pub const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// Canonical numeric wire-form failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WireFormError {
    /// The value was fractional or a non-numeric JSON type.
    NotAnInteger,
    /// The value exceeded the representable exact range.
    Overflow,
    /// A decimal string was not in canonical form.
    NonCanonical,
}

impl Display for WireFormError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::NotAnInteger => "value is not an exact integer",
            Self::Overflow => "value exceeds the exact representable range",
            Self::NonCanonical => "decimal string is not in canonical form",
        };
        formatter.write_str(message)
    }
}

impl Error for WireFormError {}

/// Rejects an unsigned exact quantity outside the exact integer range.
pub fn exact_unsigned(value: u64) -> Result<u64, WireFormError> {
    if value <= MAX_SAFE_INTEGER {
        Ok(value)
    } else {
        Err(WireFormError::Overflow)
    }
}

/// Rejects a signed exact quantity outside the exact integer range.
pub fn exact_signed(value: i64) -> Result<i64, WireFormError> {
    if value.unsigned_abs() <= MAX_SAFE_INTEGER {
        Ok(value)
    } else {
        Err(WireFormError::Overflow)
    }
}

fn decimal_schema(pattern: &str) -> Value {
    json!({"type": "string", "pattern": pattern})
}

fn unsigned_schema() -> Schema {
    let mut object = Map::new();
    object.insert(
        "anyOf".to_owned(),
        Value::Array(vec![
            json!({
                "type": "integer",
                "format": "uint64",
                "minimum": 0,
                "maximum": MAX_SAFE_INTEGER,
            }),
            decimal_schema("^(0|[1-9][0-9]*)$"),
        ]),
    );
    Schema::from(object)
}

fn signed_schema() -> Schema {
    let mut object = Map::new();
    object.insert(
        "anyOf".to_owned(),
        Value::Array(vec![
            json!({
                "type": "integer",
                "format": "int64",
                "minimum": -(MAX_SAFE_INTEGER as i64),
                "maximum": MAX_SAFE_INTEGER,
            }),
            decimal_schema("^-?(0|[1-9][0-9]*)$"),
        ]),
    );
    Schema::from(object)
}

/// A canonical unsigned v2 wire integer.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WireUint(u64);

impl WireUint {
    /// Creates a canonical unsigned v2 wire integer.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the exact value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl Serialize for WireUint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0 <= MAX_SAFE_INTEGER {
            serializer.serialize_u64(self.0)
        } else {
            serializer.serialize_str(&self.0.to_string())
        }
    }
}

struct WireUintVisitor;

impl Visitor<'_> for WireUintVisitor {
    type Value = WireUint;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("an unsigned exact integer or decimal string")
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        exact_unsigned(value).map(WireUint).map_err(E::custom)
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        u64::try_from(value)
            .ok()
            .and_then(|value| exact_unsigned(value).ok())
            .map(WireUint)
            .ok_or_else(|| E::custom(WireFormError::Overflow))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        let parsed = parse_unsigned_decimal(value).map_err(E::custom)?;
        Ok(WireUint(parsed))
    }
}

impl<'de> Deserialize<'de> for WireUint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(WireUintVisitor)
    }
}

impl JsonSchema for WireUint {
    fn schema_name() -> Cow<'static, str> {
        "WireUint".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        unsigned_schema()
    }
}

/// A canonical signed v2 wire integer.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WireInt(i64);

impl WireInt {
    /// Creates a canonical signed v2 wire integer.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the exact value.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl Serialize for WireInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0.unsigned_abs() <= MAX_SAFE_INTEGER {
            serializer.serialize_i64(self.0)
        } else {
            serializer.serialize_str(&self.0.to_string())
        }
    }
}

struct WireIntVisitor;

impl Visitor<'_> for WireIntVisitor {
    type Value = WireInt;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("a signed exact integer or decimal string")
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        i64::try_from(value)
            .ok()
            .and_then(|value| exact_signed(value).ok())
            .map(WireInt)
            .ok_or_else(|| E::custom(WireFormError::Overflow))
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        exact_signed(value).map(WireInt).map_err(E::custom)
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        let parsed = parse_signed_decimal(value).map_err(E::custom)?;
        Ok(WireInt(parsed))
    }
}

impl<'de> Deserialize<'de> for WireInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(WireIntVisitor)
    }
}

impl JsonSchema for WireInt {
    fn schema_name() -> Cow<'static, str> {
        "WireInt".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        signed_schema()
    }
}

fn is_canonical_digits(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && (value == "0" || !value.starts_with('0'))
}

fn parse_unsigned_decimal(value: &str) -> Result<u64, WireFormError> {
    if !is_canonical_digits(value) {
        return Err(WireFormError::NonCanonical);
    }
    value.parse().map_err(|_| WireFormError::Overflow)
}

fn parse_signed_decimal(value: &str) -> Result<i64, WireFormError> {
    let (negative, digits) = match value.strip_prefix('-') {
        Some(digits) => (true, digits),
        None => (false, value),
    };
    if !is_canonical_digits(digits) {
        return Err(WireFormError::NonCanonical);
    }
    let magnitude: i64 = digits.parse().map_err(|_| WireFormError::Overflow)?;
    if negative {
        if magnitude == 0 {
            return Err(WireFormError::NonCanonical);
        }
        magnitude.checked_neg().ok_or(WireFormError::Overflow)
    } else {
        Ok(magnitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_json<T: for<'de> Deserialize<'de>>(text: &str) -> Result<T, serde_json::Error> {
        serde_json::from_str(text)
    }

    #[test]
    fn unsigned_integers_accept_exact_range_and_reject_beyond() {
        assert_eq!(from_json::<WireUint>("0").map(WireUint::get).ok(), Some(0));
        assert_eq!(
            from_json::<WireUint>("9007199254740991")
                .map(WireUint::get)
                .ok(),
            Some(MAX_SAFE_INTEGER)
        );
        assert!(from_json::<WireUint>("9007199254740992").is_err());
        assert!(from_json::<WireUint>("-1").is_err());
        assert!(from_json::<WireUint>("1.5").is_err());
        assert_eq!(
            from_json::<WireUint>("\"9007199254740992\"")
                .map(WireUint::get)
                .ok(),
            Some(9_007_199_254_740_992)
        );
        assert!(from_json::<WireUint>("\"01\"").is_err());
        assert!(from_json::<WireUint>("\"+1\"").is_err());
    }

    #[test]
    fn signed_integers_accept_exact_range_and_reject_overflow() {
        assert_eq!(
            from_json::<WireInt>("-9007199254740991")
                .map(WireInt::get)
                .ok(),
            Some(-(MAX_SAFE_INTEGER as i64))
        );
        assert!(from_json::<WireInt>("-9007199254740992").is_err());
        assert_eq!(
            from_json::<WireInt>("\"-9007199254740992\"")
                .map(WireInt::get)
                .ok(),
            Some(-9_007_199_254_740_992)
        );
        assert!(from_json::<WireInt>("\"-0\"").is_err());
    }

    #[test]
    fn canonical_serialization_uses_integers_in_range_and_strings_beyond()
    -> Result<(), Box<dyn Error>> {
        assert_eq!(serde_json::to_string(&WireUint::new(5))?, "5");
        assert_eq!(
            serde_json::to_string(&WireUint::new(MAX_SAFE_INTEGER + 1))?,
            "\"9007199254740992\""
        );
        assert_eq!(serde_json::to_string(&WireInt::new(-5))?, "-5");
        Ok(())
    }

    #[test]
    fn exact_range_helpers_reject_out_of_range_quantities() {
        assert_eq!(exact_unsigned(MAX_SAFE_INTEGER), Ok(MAX_SAFE_INTEGER));
        assert_eq!(
            exact_unsigned(MAX_SAFE_INTEGER + 1),
            Err(WireFormError::Overflow)
        );
        assert_eq!(
            exact_signed(-(MAX_SAFE_INTEGER as i64)),
            Ok(-(MAX_SAFE_INTEGER as i64))
        );
        assert_eq!(
            exact_signed(-(MAX_SAFE_INTEGER as i64) - 1),
            Err(WireFormError::Overflow)
        );
    }
}
