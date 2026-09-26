//! Canonical UTC, calendar-date, and duration primitives.

use chrono::{DateTime, NaiveDate, SecondsFormat, Utc};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// The exact ECMA regex for a canonical v2 UTC instant spelling.
///
/// Years 0001-9999, literal `Z`, and the minimal AutoSi fractional group of
/// 0/3/6/9 digits. It excludes leap-second notation (`:60`) and redundant
/// fractions; calendar validity is a separate cross-field check.
pub const V2_UTC_PATTERN: &str = r"^(000[1-9]|00[1-9][0-9]|0[1-9][0-9]{2}|[1-9][0-9]{3})-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])T([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9](\.[0-9]{3}|\.[0-9]{6}|\.[0-9]{9})?Z$";

/// A canonical v2 UTC instant with exact nanoseconds.
///
/// This is the explicit v2 boundary: it rejects leap-second notation, years
/// outside 0001-9999, offsets, a missing `Z` and redundant fractional forms.
/// It never changes the retained v1 [`UtcInstant`] parser.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UtcInstantV2(DateTime<Utc>);

impl UtcInstantV2 {
    /// Returns whole Unix seconds when representable.
    #[must_use]
    pub const fn unix_seconds(self) -> i64 {
        self.0.timestamp()
    }

    /// Returns the nanosecond component.
    #[must_use]
    pub const fn subsec_nanos(self) -> u32 {
        self.0.timestamp_subsec_nanos()
    }
}

impl Display for UtcInstantV2 {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0.to_rfc3339_opts(SecondsFormat::AutoSi, true))
    }
}

impl FromStr for UtcInstantV2 {
    type Err = TimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() < 20 || value.len() > 30 || !value.is_ascii() || !value.ends_with('Z') {
            return Err(TimeParseError::InvalidInstant);
        }
        if value.as_bytes().get(10) != Some(&b'T') {
            return Err(TimeParseError::NonCanonicalInstant);
        }
        let date = &value[0..10];
        let time = &value[11..value.len() - 1];
        if date.starts_with("0000") {
            return Err(TimeParseError::InvalidInstant);
        }
        let (base, fraction) = match time.split_once('.') {
            Some((base, fraction)) => (base, Some(fraction)),
            None => (time, None),
        };
        if base.len() != 8 {
            return Err(TimeParseError::NonCanonicalInstant);
        }
        if base.get(6..8) == Some("60") {
            return Err(TimeParseError::LeapSecond);
        }
        if let Some(fraction) = fraction
            && (!matches!(fraction.len(), 3 | 6 | 9)
                || !fraction.bytes().all(|byte| byte.is_ascii_digit()))
        {
            return Err(TimeParseError::NonCanonicalInstant);
        }
        let parsed = DateTime::parse_from_rfc3339(value)
            .map_err(|_| TimeParseError::InvalidInstant)?
            .with_timezone(&Utc);
        let instant = Self(parsed);
        if instant.to_string() != value {
            return Err(TimeParseError::NonCanonicalInstant);
        }
        Ok(instant)
    }
}

/// A canonical UTC instant with nanosecond precision.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UtcInstant(DateTime<Utc>);

impl UtcInstant {
    /// Returns whole Unix seconds when representable.
    #[must_use]
    pub const fn unix_seconds(self) -> i64 {
        self.0.timestamp()
    }

    /// Returns the nanosecond component.
    #[must_use]
    pub const fn subsec_nanos(self) -> u32 {
        self.0.timestamp_subsec_nanos()
    }
}

impl Display for UtcInstant {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0.to_rfc3339_opts(SecondsFormat::AutoSi, true))
    }
}

impl FromStr for UtcInstant {
    type Err = TimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if !value.ends_with('Z') {
            return Err(TimeParseError::NonUtcInstant);
        }
        let parsed = DateTime::parse_from_rfc3339(value)
            .map_err(|_| TimeParseError::InvalidInstant)?
            .with_timezone(&Utc);
        let instant = Self(parsed);
        if instant.to_string() != value {
            return Err(TimeParseError::NonCanonicalInstant);
        }
        Ok(instant)
    }
}

/// A Gregorian calendar date in exact `YYYY-MM-DD` form.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CalendarDate(NaiveDate);

impl Display for CalendarDate {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl CalendarDate {
    /// Returns the nonnegative whole-day distance to a later or equal date.
    #[must_use]
    pub fn days_until(self, later: Self) -> Option<u64> {
        u64::try_from((later.0 - self.0).num_days()).ok()
    }
}

impl FromStr for CalendarDate {
    type Err = TimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 10 || !value.is_ascii() {
            return Err(TimeParseError::InvalidDate);
        }
        let parsed = NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .map_err(|_| TimeParseError::InvalidDate)?;
        let date = Self(parsed);
        if date.to_string() != value || value.starts_with("0000") {
            return Err(TimeParseError::InvalidDate);
        }
        Ok(date)
    }
}

/// A nonnegative duration represented by bounded integer milliseconds.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DurationMillis(u64);

impl DurationMillis {
    /// Largest value that converts losslessly to the signed runtime duration APIs.
    pub const MAX: u64 = i64::MAX as u64;

    /// Creates a bounded millisecond duration.
    pub const fn new(milliseconds: u64) -> Result<Self, TimeParseError> {
        if milliseconds > Self::MAX {
            Err(TimeParseError::DurationOverflow)
        } else {
            Ok(Self(milliseconds))
        }
    }

    /// Returns the exact millisecond count.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl Display for DurationMillis {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for DurationMillis {
    type Err = TimeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty()
            || (value.len() > 1 && value.starts_with('0'))
            || !value.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(TimeParseError::InvalidDuration);
        }
        let value = value
            .parse::<u64>()
            .map_err(|_| TimeParseError::DurationOverflow)?;
        Self::new(value)
    }
}

/// Date, instant, or duration validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimeParseError {
    /// The instant was malformed or outside the supported range.
    InvalidInstant,
    /// The instant used a non-`Z` offset.
    NonUtcInstant,
    /// The instant was valid but not in canonical wire form.
    NonCanonicalInstant,
    /// The instant used leap-second notation, which v2 rejects.
    LeapSecond,
    /// The calendar date was invalid or noncanonical.
    InvalidDate,
    /// The duration was malformed or noncanonical.
    InvalidDuration,
    /// The duration exceeded the signed runtime boundary.
    DurationOverflow,
}

impl Display for TimeParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidInstant => "invalid UTC instant",
            Self::NonUtcInstant => "UTC instant requires the Z suffix",
            Self::NonCanonicalInstant => "UTC instant is not in canonical form",
            Self::LeapSecond => "leap seconds are not a supported UTC instant",
            Self::InvalidDate => "invalid calendar date",
            Self::InvalidDuration => "invalid millisecond duration",
            Self::DurationOverflow => "millisecond duration exceeds its bound",
        })
    }
}

impl Error for TimeParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_instants_round_trip_and_order() -> Result<(), TimeParseError> {
        let first: UtcInstant = "2026-08-07T12:00:00Z".parse()?;
        let fractional: UtcInstant = "2026-08-07T12:00:00.123456789Z".parse()?;
        let later: UtcInstant = "2026-08-07T12:00:01Z".parse()?;
        assert_eq!(first.to_string(), "2026-08-07T12:00:00Z");
        assert_eq!(fractional.subsec_nanos(), 123_456_789);
        assert!(first < fractional && fractional < later);
        Ok(())
    }

    #[test]
    fn non_utc_and_noncanonical_instants_fail_closed() {
        for (value, error) in [
            ("2026-08-07T12:00:00+00:00", TimeParseError::NonUtcInstant),
            ("2026-08-07T12:00:00-04:00", TimeParseError::NonUtcInstant),
            ("2026-08-07 12:00:00Z", TimeParseError::NonCanonicalInstant),
            (
                "2026-08-07T12:00:00.000Z",
                TimeParseError::NonCanonicalInstant,
            ),
            ("not-time", TimeParseError::NonUtcInstant),
        ] {
            assert_eq!(value.parse::<UtcInstant>(), Err(error));
        }
    }

    #[test]
    fn v2_instants_accept_only_canonical_forms() -> Result<(), TimeParseError> {
        for value in [
            "0001-01-01T00:00:00Z",
            "2026-01-02T03:04:05Z",
            "2026-01-02T03:04:05.100Z",
            "2026-01-02T03:04:05.000001Z",
            "2026-01-02T03:04:05.000000001Z",
            "9999-12-31T23:59:59Z",
        ] {
            let instant: UtcInstantV2 = value.parse()?;
            assert_eq!(instant.to_string(), value);
        }
        assert_eq!(
            "2026-01-02T03:04:05.123456789Z"
                .parse::<UtcInstantV2>()?
                .subsec_nanos(),
            123_456_789
        );
        for value in [
            "2016-12-31T23:59:60Z",
            "2016-12-31T23:59:60.000Z",
            "0000-01-01T00:00:00Z",
            "2026-01-02T03:04:05.1Z",
            "2026-01-02T03:04:05.000Z",
            "2026-01-02T03:04:05.000000Z",
            "2026-01-02T03:04:05+00:00",
            "2026-01-02T03:04:05",
            "2026-02-30T00:00:00Z",
            "2026-01-02 03:04:05Z",
        ] {
            assert!(value.parse::<UtcInstantV2>().is_err(), "accepted {value}");
        }
        assert_eq!(
            "2016-12-31T23:59:60Z".parse::<UtcInstantV2>().err(),
            Some(TimeParseError::LeapSecond)
        );
        Ok(())
    }

    #[test]
    fn calendar_dates_validate_leap_and_format_boundaries() -> Result<(), TimeParseError> {
        let leap: CalendarDate = "2024-02-29".parse()?;
        assert_eq!(leap.to_string(), "2024-02-29");
        for invalid in [
            "2023-02-29",
            "2024-2-29",
            "0000-01-01",
            "2024-13-01",
            "+2024-01-01",
        ] {
            assert_eq!(
                invalid.parse::<CalendarDate>(),
                Err(TimeParseError::InvalidDate)
            );
        }
        Ok(())
    }

    #[test]
    fn durations_reject_noncanonical_and_overflow_values() -> Result<(), TimeParseError> {
        let zero: DurationMillis = "0".parse()?;
        let maximum = DurationMillis::new(DurationMillis::MAX)?;
        assert_eq!(zero.get(), 0);
        assert_eq!(maximum.to_string(), DurationMillis::MAX.to_string());
        assert_eq!(
            DurationMillis::new(DurationMillis::MAX + 1),
            Err(TimeParseError::DurationOverflow)
        );
        for invalid in ["", "00", "+1", "-1", "1.0", "18446744073709551616"] {
            assert!(
                invalid.parse::<DurationMillis>().is_err(),
                "accepted {invalid}"
            );
        }
        Ok(())
    }
}
