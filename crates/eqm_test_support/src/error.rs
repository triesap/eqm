//! Sanitized fixture-construction failures.

use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// A fixture helper could not construct a requested value.
///
/// Fixture helpers deliberately retain only the underlying reason message.
/// They never carry authority, credentials, or a synthetic trust decision into
/// a caller that could mistake test scaffolding for production capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureError(String);

impl FixtureError {
    /// Wraps an underlying construction failure as a sanitized message.
    pub fn new(message: impl Display) -> Self {
        Self(message.to_string())
    }

    /// Returns the sanitized reason message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.0
    }
}

impl Display for FixtureError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "fixture construction failed: {}", self.0)
    }
}

impl Error for FixtureError {}
