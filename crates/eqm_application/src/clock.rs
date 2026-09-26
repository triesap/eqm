//! The single host evaluation-time capture boundary.
//!
//! Semantic preparation and evaluation never read the wall clock themselves.
//! A transport captures one instant here and threads it through an explicit
//! request, so repeated evaluation with the same instant is deterministic and
//! process timeout/acquisition timers stay separate from evaluation time.

use eqm_domain::{TimeParseError, UtcInstant};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::time::SystemTime;

/// The host clock could not produce a canonical evaluation instant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClockError {
    /// The host instant was outside the canonical UTC representation.
    Unavailable,
}

impl Display for ClockError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("host evaluation time is unavailable")
    }
}

impl Error for ClockError {}

impl From<TimeParseError> for ClockError {
    fn from(_: TimeParseError) -> Self {
        Self::Unavailable
    }
}

/// Captures one explicit host evaluation instant for a request.
///
/// Callers must capture once per operation and pass the value explicitly;
/// evaluation must not call this again internally.
pub fn host_evaluation_time() -> Result<UtcInstant, ClockError> {
    canonical_instant(SystemTime::now())
}

/// Converts a system time into the canonical UTC instant without losing
/// subsecond precision. AutoSi preserves nanosecond precision (0/3/6/9
/// fractional digits), matching the canonical `UtcInstant` representation.
fn canonical_instant(value: SystemTime) -> Result<UtcInstant, ClockError> {
    let value: chrono::DateTime<chrono::Utc> = value.into();
    value
        .to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true)
        .parse()
        .map_err(ClockError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captured_host_time_is_canonical() -> Result<(), Box<dyn Error>> {
        let instant = host_evaluation_time()?;
        assert_eq!(instant.to_string().parse::<UtcInstant>()?, instant);
        Ok(())
    }

    #[test]
    fn subsecond_precision_is_preserved() -> Result<(), Box<dyn Error>> {
        use std::time::Duration;
        let millis = canonical_instant(SystemTime::UNIX_EPOCH + Duration::from_millis(1))?;
        assert_eq!(millis.to_string(), "1970-01-01T00:00:00.001Z");
        let nanos =
            canonical_instant(SystemTime::UNIX_EPOCH + Duration::from_nanos(1_500_000_123))?;
        assert_eq!(nanos.to_string(), "1970-01-01T00:00:01.500000123Z");
        assert_eq!(nanos.subsec_nanos(), 500_000_123);
        let micros = canonical_instant(SystemTime::UNIX_EPOCH + Duration::from_micros(1))?;
        assert_eq!(micros.to_string(), "1970-01-01T00:00:00.000001Z");
        Ok(())
    }
}
