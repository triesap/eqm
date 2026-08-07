//! Canonical UTC, calendar-date, and duration primitives.

use chrono::{DateTime, NaiveDate, SecondsFormat, Utc};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

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
