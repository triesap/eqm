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
    let value: chrono::DateTime<chrono::Utc> = SystemTime::now().into();
    value
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
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
}
