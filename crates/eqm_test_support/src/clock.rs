//! Deterministic injected evaluation time for fixtures.
//!
//! Fixtures must never read the wall clock. These helpers return canonical
//! fixed instants so repeated preparation is byte-identical, and parsing is
//! delegated to the domain's canonical UTC parser so malformed inputs fail
//! exactly as production would.

use eqm_domain::{TimeParseError, UtcInstant};
use std::str::FromStr;

/// The canonical fixed evaluation instant used by deterministic fixtures.
pub const FIXED_EVALUATION_TIME: &str = "2026-01-02T03:04:05Z";

/// A canonical instant strictly before [`FIXED_EVALUATION_TIME`].
pub const FIXED_EARLIER_TIME: &str = "2026-01-01T00:00:00Z";

/// A canonical instant strictly after [`FIXED_EVALUATION_TIME`].
pub const FIXED_LATER_TIME: &str = "2026-01-03T06:07:08Z";

/// Parses an exact canonical UTC instant for fixture construction.
pub fn instant(value: &str) -> Result<UtcInstant, TimeParseError> {
    UtcInstant::from_str(value)
}

/// Returns the fixed evaluation instant.
pub fn evaluation_time() -> Result<UtcInstant, TimeParseError> {
    instant(FIXED_EVALUATION_TIME)
}

/// Returns the fixed earlier instant.
pub fn earlier_time() -> Result<UtcInstant, TimeParseError> {
    instant(FIXED_EARLIER_TIME)
}

/// Returns the fixed later instant.
pub fn later_time() -> Result<UtcInstant, TimeParseError> {
    instant(FIXED_LATER_TIME)
}
