//! Stable diagnostic primitives and rendering.

use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroU32;
use std::str::FromStr;

/// A stable EQM diagnostic code.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticCode(u16);

impl DiagnosticCode {
    /// Creates a code when its number belongs to an allocated v1 range.
    #[must_use]
    pub const fn from_number(number: u16) -> Option<Self> {
        if Self::is_allocated(number) {
            Some(Self(number))
        } else {
            None
        }
    }

    /// Returns the four-digit numeric component.
    #[must_use]
    pub const fn number(self) -> u16 {
        self.0
    }

    const fn is_allocated(number: u16) -> bool {
        (number >= 1 && number <= 1_099) || (number >= 9_000 && number <= 9_099)
    }
}

impl Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "EQM-E{:04}", self.0)
    }
}

impl FromStr for DiagnosticCode {
    type Err = DiagnosticBuildError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Some(number) = value.strip_prefix("EQM-E") else {
            return Err(DiagnosticBuildError::InvalidCode);
        };
        if number.len() != 4 || !number.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(DiagnosticBuildError::InvalidCode);
        }
        let parsed = number
            .parse::<u16>()
            .map_err(|_| DiagnosticBuildError::InvalidCode)?;
        Self::from_number(parsed).ok_or(DiagnosticBuildError::InvalidCode)
    }
}

/// Diagnostic severity in stable display order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Severity {
    /// A blocking error.
    Error,
    /// A non-blocking warning.
    Warning,
    /// Additional information.
    Note,
}

impl Display for Severity {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
        })
    }
}

/// A validated, display-safe source identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceName(Box<str>);

impl SourceName {
    /// Validates a repository-relative source label or EQM resource URI.
    pub fn new(value: impl Into<Box<str>>) -> Result<Self, DiagnosticBuildError> {
        let value = value.into();
        validate_text(&value, 1_024).map_err(|_| DiagnosticBuildError::InvalidSource)?;
        if value.starts_with('/') || value.contains('\\') || value.chars().any(char::is_control) {
            return Err(DiagnosticBuildError::InvalidSource);
        }
        Ok(Self(value))
    }

    /// Returns the source label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SourceName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A one-based position in a source.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourcePosition {
    line: NonZeroU32,
    column: NonZeroU32,
}

impl SourcePosition {
    /// Creates a one-based position.
    pub fn new(line: u32, column: u32) -> Result<Self, DiagnosticBuildError> {
        let line = NonZeroU32::new(line).ok_or(DiagnosticBuildError::ZeroPosition)?;
        let column = NonZeroU32::new(column).ok_or(DiagnosticBuildError::ZeroPosition)?;
        Ok(Self { line, column })
    }

    /// Returns the one-based line.
    #[must_use]
    pub const fn line(self) -> u32 {
        self.line.get()
    }

    /// Returns the one-based column.
    #[must_use]
    pub const fn column(self) -> u32 {
        self.column.get()
    }
}

impl Display for SourcePosition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.line, self.column)
    }
}

/// A validated source span with inclusive start and exclusive end positions.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceLocation {
    source: SourceName,
    start: SourcePosition,
    end: SourcePosition,
}

impl SourceLocation {
    /// Creates a source span whose end is not before its start.
    pub fn new(
        source: SourceName,
        start: SourcePosition,
        end: SourcePosition,
    ) -> Result<Self, DiagnosticBuildError> {
        if end < start {
            return Err(DiagnosticBuildError::ReversedSpan);
        }
        Ok(Self { source, start, end })
    }

    /// Returns the source label.
    #[must_use]
    pub const fn source(&self) -> &SourceName {
        &self.source
    }

    /// Returns the inclusive start position.
    #[must_use]
    pub const fn start(&self) -> SourcePosition {
        self.start
    }

    /// Returns the exclusive end position.
    #[must_use]
    pub const fn end(&self) -> SourcePosition {
        self.end
    }
}

impl Display for SourceLocation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}-{}", self.source, self.start, self.end)
    }
}

/// Static registry metadata for one diagnostic code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticDescriptor {
    /// Stable code.
    pub code: DiagnosticCode,
    /// Default severity.
    pub severity: Severity,
    /// Short stable title.
    pub title: &'static str,
    /// Repository-relative specification reference.
    pub authority: &'static str,
    /// Explanation of the condition.
    pub explanation: &'static str,
    /// Default remediation guidance.
    pub remediation: &'static str,
}

/// One complete diagnostic instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    code: DiagnosticCode,
    severity: Severity,
    message: Box<str>,
    source: Option<SourceLocation>,
    related: Vec<SourceLocation>,
    remediation: Option<Box<str>>,
}

impl Diagnostic {
    /// Creates a diagnostic and normalizes related-location ordering.
    pub fn new(
        code: DiagnosticCode,
        severity: Severity,
        message: impl Into<Box<str>>,
        source: Option<SourceLocation>,
        mut related: Vec<SourceLocation>,
        remediation: Option<Box<str>>,
    ) -> Result<Self, DiagnosticBuildError> {
        let message = message.into();
        validate_text(&message, 4_096).map_err(|_| DiagnosticBuildError::InvalidMessage)?;
        if let Some(value) = remediation.as_deref() {
            validate_text(value, 4_096).map_err(|_| DiagnosticBuildError::InvalidRemediation)?;
        }
        related.sort_unstable();
        related.dedup();
        Ok(Self {
            code,
            severity,
            message,
            source,
            related,
            remediation,
        })
    }

    /// Creates a diagnostic from registered defaults.
    pub fn from_descriptor(
        descriptor: &DiagnosticDescriptor,
        message: impl Into<Box<str>>,
        source: Option<SourceLocation>,
        related: Vec<SourceLocation>,
    ) -> Result<Self, DiagnosticBuildError> {
        Self::new(
            descriptor.code,
            descriptor.severity,
            message,
            source,
            related,
            Some(descriptor.remediation.into()),
        )
    }

    /// Returns the stable code.
    #[must_use]
    pub const fn code(&self) -> DiagnosticCode {
        self.code
    }

    /// Returns the severity.
    #[must_use]
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    /// Returns the message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the primary location when available.
    #[must_use]
    pub const fn source(&self) -> Option<&SourceLocation> {
        self.source.as_ref()
    }

    /// Returns sorted, unique related locations.
    #[must_use]
    pub fn related(&self) -> &[SourceLocation] {
        &self.related
    }

    /// Returns remediation guidance when available.
    #[must_use]
    pub fn remediation(&self) -> Option<&str> {
        self.remediation.as_deref()
    }
}

impl Ord for Diagnostic {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.severity,
            self.code,
            &self.source,
            self.message.as_ref(),
        )
            .cmp(&(
                other.severity,
                other.code,
                &other.source,
                other.message.as_ref(),
            ))
    }
}

impl PartialOrd for Diagnostic {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Diagnostic {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}: ", self.code, self.severity)?;
        write_diagnostic_text(formatter, &self.message)?;
        if let Some(source) = &self.source {
            write!(formatter, "\n  --> {source}")?;
        }
        for related in &self.related {
            write!(formatter, "\n  related: {related}")?;
        }
        if let Some(remediation) = &self.remediation {
            formatter.write_str("\n  remediation: ")?;
            write_diagnostic_text(formatter, remediation)?;
        }
        Ok(())
    }
}

/// Validates that a diagnostic registry is strictly code-sorted and complete.
pub fn validate_diagnostic_registry(
    registry: &[DiagnosticDescriptor],
) -> Result<(), DiagnosticRegistryError> {
    for (index, descriptor) in registry.iter().enumerate() {
        validate_static_text(descriptor.title)
            .map_err(|_| DiagnosticRegistryError::EmptyField(descriptor.code))?;
        validate_static_text(descriptor.authority)
            .map_err(|_| DiagnosticRegistryError::EmptyField(descriptor.code))?;
        validate_static_text(descriptor.explanation)
            .map_err(|_| DiagnosticRegistryError::EmptyField(descriptor.code))?;
        validate_static_text(descriptor.remediation)
            .map_err(|_| DiagnosticRegistryError::EmptyField(descriptor.code))?;
        if let Some(previous) = index.checked_sub(1).and_then(|value| registry.get(value))
            && previous.code >= descriptor.code
        {
            return Err(DiagnosticRegistryError::NotStrictlySorted {
                previous: previous.code,
                current: descriptor.code,
            });
        }
    }
    Ok(())
}

fn validate_text(value: &str, maximum_bytes: usize) -> Result<(), ()> {
    if value.is_empty()
        || value.len() > maximum_bytes
        || value
            .chars()
            .any(|character| character.is_control() && character != '\n')
    {
        return Err(());
    }
    Ok(())
}

fn validate_static_text(value: &str) -> Result<(), ()> {
    validate_text(value, 4_096)
}

fn write_diagnostic_text(formatter: &mut Formatter<'_>, value: &str) -> fmt::Result {
    let mut segments = value.split('\n');
    if let Some(first) = segments.next() {
        formatter.write_str(first)?;
    }
    for segment in segments {
        formatter.write_str("\\n")?;
        formatter.write_str(segment)?;
    }
    Ok(())
}

/// Diagnostic construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticBuildError {
    /// The code has invalid syntax or is outside an allocated range.
    InvalidCode,
    /// The source label is invalid.
    InvalidSource,
    /// A line or column was zero.
    ZeroPosition,
    /// A source span ended before it began.
    ReversedSpan,
    /// The message was empty, oversized, or contained a forbidden control.
    InvalidMessage,
    /// Remediation was empty, oversized, or contained a forbidden control.
    InvalidRemediation,
}

impl Display for DiagnosticBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidCode => "invalid diagnostic code",
            Self::InvalidSource => "invalid diagnostic source",
            Self::ZeroPosition => "source positions are one-based",
            Self::ReversedSpan => "source span end precedes its start",
            Self::InvalidMessage => "invalid diagnostic message",
            Self::InvalidRemediation => "invalid diagnostic remediation",
        })
    }
}

impl Error for DiagnosticBuildError {}

/// Diagnostic registry validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticRegistryError {
    /// A required descriptor field was empty or invalid.
    EmptyField(DiagnosticCode),
    /// Entries were duplicated or out of order.
    NotStrictlySorted {
        /// The preceding code.
        previous: DiagnosticCode,
        /// The current code.
        current: DiagnosticCode,
    },
}

impl Display for DiagnosticRegistryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(code) => {
                write!(formatter, "registry descriptor for {code} is incomplete")
            }
            Self::NotStrictlySorted { previous, current } => {
                write!(
                    formatter,
                    "registry codes are not strictly sorted: {previous}, {current}"
                )
            }
        }
    }
}

impl Error for DiagnosticRegistryError {}

#[cfg(test)]
mod tests {
    use super::*;

    const PARSE_ERROR: DiagnosticDescriptor = DiagnosticDescriptor {
        code: DiagnosticCode(100),
        severity: Severity::Error,
        title: "Invalid manifest syntax",
        authority: "docs/specification/manifest-contracts.md",
        explanation: "The authored document is not valid TOML.",
        remediation: "Correct the reported syntax.",
    };

    fn position(line: u32, column: u32) -> Result<SourcePosition, DiagnosticBuildError> {
        SourcePosition::new(line, column)
    }

    fn location(
        source: &str,
        line: u32,
        column: u32,
    ) -> Result<SourceLocation, DiagnosticBuildError> {
        SourceLocation::new(
            SourceName::new(source)?,
            position(line, column)?,
            position(line, column + 1)?,
        )
    }

    #[test]
    fn code_parsing_is_exact_and_allocated() {
        assert_eq!("EQM-E0100".parse(), Ok(PARSE_ERROR.code));
        for invalid in [
            "EQM-E100",
            "eqm-E0100",
            "EQM-E1100",
            "EQM-E8999",
            "EQM-E9100",
        ] {
            assert_eq!(
                invalid.parse::<DiagnosticCode>(),
                Err(DiagnosticBuildError::InvalidCode)
            );
        }
    }

    #[test]
    fn locations_reject_zero_and_reversed_positions() -> Result<(), DiagnosticBuildError> {
        assert_eq!(
            SourcePosition::new(0, 1),
            Err(DiagnosticBuildError::ZeroPosition)
        );
        let source = SourceName::new("eqm/contracts/example.toml")?;
        assert_eq!(
            SourceLocation::new(source, position(2, 1)?, position(1, 1)?),
            Err(DiagnosticBuildError::ReversedSpan)
        );
        Ok(())
    }

    #[test]
    fn related_locations_are_sorted_and_deduplicated() -> Result<(), DiagnosticBuildError> {
        let later = location("eqm/contracts/z.toml", 2, 1)?;
        let earlier = location("eqm/contracts/a.toml", 1, 1)?;
        let diagnostic = Diagnostic::from_descriptor(
            &PARSE_ERROR,
            "invalid key",
            None,
            vec![later.clone(), earlier.clone(), later],
        )?;
        assert_eq!(
            diagnostic.related(),
            &[earlier, location("eqm/contracts/z.toml", 2, 1)?]
        );
        Ok(())
    }

    #[test]
    fn rendering_contains_every_diagnostic_component() -> Result<(), DiagnosticBuildError> {
        let diagnostic = Diagnostic::from_descriptor(
            &PARSE_ERROR,
            "invalid key",
            Some(location("eqm/contracts/a.toml", 2, 4)?),
            vec![location("eqm/contracts/b.toml", 8, 1)?],
        )?;
        assert_eq!(
            diagnostic.to_string(),
            "EQM-E0100 error: invalid key\n  --> eqm/contracts/a.toml:2:4-2:5\n  related: eqm/contracts/b.toml:8:1-8:2\n  remediation: Correct the reported syntax."
        );
        Ok(())
    }

    #[test]
    fn rendering_escapes_embedded_newlines() -> Result<(), DiagnosticBuildError> {
        let diagnostic = Diagnostic::new(
            PARSE_ERROR.code,
            Severity::Error,
            "invalid\nkey",
            None,
            Vec::new(),
            Some("use one\nkey".into()),
        )?;
        assert_eq!(
            diagnostic.to_string(),
            "EQM-E0100 error: invalid\\nkey\n  remediation: use one\\nkey"
        );
        Ok(())
    }

    #[test]
    fn ordering_follows_severity_code_source_and_message() -> Result<(), DiagnosticBuildError> {
        let code = PARSE_ERROR.code;
        let create = |severity, source: &str, message: &str| {
            Diagnostic::new(
                code,
                severity,
                message,
                Some(location(source, 1, 1)?),
                Vec::new(),
                None,
            )
        };
        let mut diagnostics = [
            create(Severity::Warning, "b.toml", "a")?,
            create(Severity::Error, "b.toml", "b")?,
            create(Severity::Error, "a.toml", "z")?,
        ];
        diagnostics.sort();
        assert_eq!(
            diagnostics[0]
                .source()
                .map(SourceLocation::source)
                .map(SourceName::as_str),
            Some("a.toml")
        );
        assert_eq!(diagnostics[1].severity(), Severity::Error);
        assert_eq!(diagnostics[2].severity(), Severity::Warning);
        Ok(())
    }

    #[test]
    fn registry_requires_complete_strictly_sorted_descriptors() {
        let second = DiagnosticDescriptor {
            code: DiagnosticCode(101),
            ..PARSE_ERROR
        };
        assert_eq!(validate_diagnostic_registry(&[PARSE_ERROR, second]), Ok(()));
        assert_eq!(
            validate_diagnostic_registry(&[second, PARSE_ERROR]),
            Err(DiagnosticRegistryError::NotStrictlySorted {
                previous: second.code,
                current: PARSE_ERROR.code,
            })
        );
    }
}
