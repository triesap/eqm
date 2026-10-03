//! Canonical numeric wire forms frozen by ADR-0013.
//!
//! In the `eqm:v2` namespace exact integers are JSON integers within
//! `|n| <= 2^53-1` and canonical decimal strings outside that interval;
//! [`WireUint`] and [`WireInt`] implement that canonical pair over the full
//! `u64`/`i64` domains. Strings are used only outside the safe interval, so a
//! small-value string, negative zero, a leading zero, a plus sign, a fractional
//! or exponent token, and overflow are all rejected. The retained v1 contract
//! keeps its original full-`u64`/`i64` integer acceptance; v1 schemas and bytes
//! are unchanged and never acquire the v2 string form.

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

/// Returns an ECMA regex alternation matching exactly the canonical decimal
/// spellings of every integer in the inclusive range `low..=high`.
///
/// The range is split into digit-length buckets and each bucket is decomposed
/// into a flat alternation of literal prefixes, bounded character classes and
/// fixed-width digit runs, so the pattern encodes the numeric range exactly
/// instead of merely matching decimal syntax.
fn decimal_range_alternation(low: u128, high: u128) -> String {
    debug_assert!(low <= high);
    let low_text = low.to_string();
    let high_text = high.to_string();
    let mut parts = Vec::new();
    for length in low_text.len()..=high_text.len() {
        let lower = if length == low_text.len() {
            low
        } else {
            10u128.pow(length as u32 - 1)
        };
        let upper = if length == high_text.len() {
            high
        } else {
            10u128.pow(length as u32) - 1
        };
        parts.extend(range_fragments(&lower.to_string(), &upper.to_string()));
    }
    format!("(?:{})", parts.join("|"))
}

fn range_fragments(low: &str, high: &str) -> Vec<String> {
    let low_digits: Vec<u8> = low.bytes().map(|byte| byte - b'0').collect();
    let high_digits: Vec<u8> = high.bytes().map(|byte| byte - b'0').collect();
    range_fragments_digits(&low_digits, &high_digits)
}

fn range_fragments_digits(low: &[u8], high: &[u8]) -> Vec<String> {
    if low.is_empty() {
        return vec![String::new()];
    }
    if low.iter().all(|digit| *digit == 0) && high.iter().all(|digit| *digit == 9) {
        return vec![digit_run(low.len())];
    }
    let (Some((&low_head, low_tail)), Some((&high_head, high_tail))) =
        (low.split_first(), high.split_first())
    else {
        return vec![String::new()];
    };
    if low_head == high_head {
        return range_fragments_digits(low_tail, high_tail)
            .into_iter()
            .map(|suffix| format!("{low_head}{suffix}"))
            .collect();
    }
    let rest = low_tail.len();
    let mut fragments: Vec<String> = range_fragments_digits(low_tail, &vec![9u8; rest])
        .into_iter()
        .map(|suffix| format!("{low_head}{suffix}"))
        .collect();
    if high_head - low_head >= 2 {
        fragments.push(format!(
            "{}{}",
            digit_class(low_head + 1, high_head - 1),
            digit_run(rest)
        ));
    }
    fragments.extend(
        range_fragments_digits(&vec![0u8; rest], high_tail)
            .into_iter()
            .map(|suffix| format!("{high_head}{suffix}")),
    );
    fragments
}

fn digit_class(low: u8, high: u8) -> String {
    if low == high {
        low.to_string()
    } else {
        format!("[{low}-{high}]")
    }
}

fn digit_run(count: usize) -> String {
    if count == 0 {
        String::new()
    } else {
        format!("[0-9]{{{count}}}")
    }
}

fn canonical_unsigned_string_pattern() -> String {
    format!(
        "^{}$",
        decimal_range_alternation((MAX_SAFE_INTEGER as u128) + 1, u64::MAX as u128)
    )
}

fn canonical_signed_string_pattern() -> String {
    let positive = decimal_range_alternation((MAX_SAFE_INTEGER as u128) + 1, i64::MAX as u128);
    let negative =
        decimal_range_alternation((MAX_SAFE_INTEGER as u128) + 1, (i64::MAX as u128) + 1);
    format!("^(?:{positive}|-{negative})$")
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
            json!({
                "type": "string",
                "pattern": canonical_unsigned_string_pattern(),
            }),
        ]),
    );
    Schema::from(object)
}

/// Returns the schema for a canonical unsigned v2 wire integer of at least one.
///
/// This mirrors [`unsigned_schema`] but excludes zero, matching owning positive
/// authority revisions such as `Revision` (`NonZeroU64`).
pub fn positive_unsigned_schema(_generator: &mut SchemaGenerator) -> Schema {
    let mut object = Map::new();
    object.insert(
        "anyOf".to_owned(),
        Value::Array(vec![
            json!({
                "type": "integer",
                "format": "uint64",
                "minimum": 1,
                "maximum": MAX_SAFE_INTEGER,
            }),
            json!({
                "type": "string",
                "pattern": canonical_unsigned_string_pattern(),
            }),
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
            json!({
                "type": "string",
                "pattern": canonical_signed_string_pattern(),
            }),
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
        if parsed <= MAX_SAFE_INTEGER {
            return Err(E::custom(WireFormError::NonCanonical));
        }
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
        if parsed.unsigned_abs() <= MAX_SAFE_INTEGER {
            return Err(E::custom(WireFormError::NonCanonical));
        }
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
    if !is_canonical_digits(digits) || (negative && digits == "0") {
        return Err(WireFormError::NonCanonical);
    }
    // Parse the magnitude as an unsigned value so the exact `i64::MIN`
    // endpoint, whose magnitude is `i64::MAX + 1`, round-trips instead of
    // overflowing before negation.
    let magnitude: u128 = digits.parse().map_err(|_| WireFormError::Overflow)?;
    if negative {
        let limit = (i64::MAX as u128) + 1;
        if magnitude > limit {
            return Err(WireFormError::Overflow);
        }
        if magnitude == limit {
            Ok(i64::MIN)
        } else {
            Ok(-(magnitude as i64))
        }
    } else {
        i64::try_from(magnitude).map_err(|_| WireFormError::Overflow)
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
        assert!(from_json::<WireUint>("1e3").is_err());
        assert_eq!(
            from_json::<WireUint>("\"9007199254740992\"")
                .map(WireUint::get)
                .ok(),
            Some(9_007_199_254_740_992)
        );
        assert_eq!(
            from_json::<WireUint>("\"18446744073709551615\"")
                .map(WireUint::get)
                .ok(),
            Some(u64::MAX)
        );
        for rejected in [
            "\"0\"",
            "\"5\"",
            "\"9007199254740991\"",
            "\"01\"",
            "\"+1\"",
            "\"-5\"",
            "\"1.0\"",
            "\"1e3\"",
            "\"0007199254740992\"",
            "\"18446744073709551616\"",
        ] {
            assert!(
                from_json::<WireUint>(rejected).is_err(),
                "accepted {rejected}"
            );
        }
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
        assert!(from_json::<WireInt>("-1e3").is_err());
        assert_eq!(
            from_json::<WireInt>("\"-9007199254740992\"")
                .map(WireInt::get)
                .ok(),
            Some(-9_007_199_254_740_992)
        );
        assert_eq!(
            from_json::<WireInt>("\"-9223372036854775808\"")
                .map(WireInt::get)
                .ok(),
            Some(i64::MIN)
        );
        assert_eq!(
            from_json::<WireInt>("\"9223372036854775807\"")
                .map(WireInt::get)
                .ok(),
            Some(i64::MAX)
        );
        for rejected in [
            "\"-0\"",
            "\"-5\"",
            "\"5\"",
            "\"-9007199254740991\"",
            "\"9007199254740991\"",
            "\"+9007199254740992\"",
            "\"-9223372036854775809\"",
            "\"9223372036854775808\"",
        ] {
            assert!(
                from_json::<WireInt>(rejected).is_err(),
                "accepted {rejected}"
            );
        }
    }

    #[test]
    fn generated_range_patterns_match_small_ranges_exactly() {
        for (low, high) in [
            (0u128, 0u128),
            (1, 9),
            (9, 11),
            (97, 105),
            (90, 110),
            (995, 1005),
            (900, 1899),
        ] {
            let pattern = format!("^{}$", decimal_range_alternation(low, high));
            let regex = regex_lite::Regex::new(&pattern);
            assert!(
                regex.is_ok(),
                "range {low}..={high} pattern failed to compile"
            );
            let Ok(regex) = regex else {
                continue;
            };
            for value in 0..2000u128 {
                let text = value.to_string();
                let expected = value >= low && value <= high;
                assert_eq!(
                    regex.is_match(&text),
                    expected,
                    "range {low}..={high} disagreed on {text}"
                );
            }
            for probe in ["", "00", "01", "+1", "-1", "1.0"] {
                assert!(
                    !regex.is_match(probe),
                    "range {low}..={high} matched non-canonical {probe}"
                );
            }
        }
    }

    #[test]
    fn numeric_endpoints_round_trip_through_canonical_text() -> Result<(), Box<dyn Error>> {
        let unsigned = serde_json::to_string(&WireUint::new(u64::MAX))?;
        assert_eq!(unsigned, "\"18446744073709551615\"");
        assert_eq!(from_json::<WireUint>(&unsigned)?.get(), u64::MAX);
        for value in [i64::MIN, i64::MAX, -(MAX_SAFE_INTEGER as i64), 0] {
            let rendered = serde_json::to_string(&WireInt::new(value))?;
            assert_eq!(from_json::<WireInt>(&rendered)?.get(), value);
        }
        assert_eq!(
            serde_json::to_string(&WireInt::new(i64::MIN))?,
            "\"-9223372036854775808\""
        );
        assert_eq!(
            serde_json::to_string(&WireInt::new(i64::MAX))?,
            "\"9223372036854775807\""
        );
        assert_eq!(serde_json::to_string(&WireInt::new(0))?, "0");
        Ok(())
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
        assert_eq!(
            serde_json::to_string(&WireInt::new(-(MAX_SAFE_INTEGER as i64) - 1))?,
            "\"-9007199254740992\""
        );
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
