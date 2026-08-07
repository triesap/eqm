//! Bounded TOML 1.1 parsing with stable source spans.

use eqm_domain::{SourceLocation, SourceName, SourcePosition};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::ops::Range;

const MAX_DOCUMENT_BYTES: usize = 4 * 1024 * 1024;

/// One syntactically valid TOML source document.
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedToml {
    source: SourceName,
    text: Box<str>,
    root: toml::Table,
}

impl ParsedToml {
    /// Returns the source identity.
    #[must_use]
    pub const fn source(&self) -> &SourceName {
        &self.source
    }
    /// Returns exact decoded source text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
    /// Returns the parsed root table.
    #[must_use]
    pub const fn root(&self) -> &toml::Table {
        &self.root
    }
    /// Converts an in-bounds byte range to a one-based source span.
    pub fn location(&self, range: Range<usize>) -> Result<SourceLocation, ParseError> {
        source_location(&self.source, &self.text, range)
    }
}

/// TOML parse failure with an optional precise byte-derived location.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TomlSyntaxError {
    message: Box<str>,
    location: Option<SourceLocation>,
}

impl TomlSyntaxError {
    /// Returns the parser message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
    /// Returns the precise source span when the parser supplied one.
    #[must_use]
    pub const fn location(&self) -> Option<&SourceLocation> {
        self.location.as_ref()
    }
}

/// Parses one bounded UTF-8 TOML 1.1 document without filesystem access.
pub fn parse_toml(source: SourceName, bytes: &[u8]) -> Result<ParsedToml, ParseError> {
    if bytes.len() > MAX_DOCUMENT_BYTES {
        return Err(ParseError::DocumentTooLarge);
    }
    let text = std::str::from_utf8(bytes).map_err(|_| ParseError::InvalidUtf8)?;
    let root = toml::from_str::<toml::Table>(text).map_err(|error| {
        let location = error
            .span()
            .and_then(|range| source_location(&source, text, range).ok());
        ParseError::Syntax(TomlSyntaxError {
            message: error.message().into(),
            location,
        })
    })?;
    Ok(ParsedToml {
        source,
        text: text.into(),
        root,
    })
}

fn source_location(
    source: &SourceName,
    text: &str,
    range: Range<usize>,
) -> Result<SourceLocation, ParseError> {
    if range.start > range.end
        || range.end > text.len()
        || !text.is_char_boundary(range.start)
        || !text.is_char_boundary(range.end)
    {
        return Err(ParseError::InvalidSpan);
    }
    SourceLocation::new(
        source.clone(),
        position(text, range.start)?,
        position(text, range.end)?,
    )
    .map_err(|_| ParseError::InvalidSpan)
}

fn position(text: &str, offset: usize) -> Result<SourcePosition, ParseError> {
    let prefix = text.get(..offset).ok_or(ParseError::InvalidSpan)?;
    let line = u32::try_from(prefix.bytes().filter(|byte| *byte == b'\n').count() + 1)
        .map_err(|_| ParseError::InvalidSpan)?;
    let line_start = prefix.rfind('\n').map_or(0, |index| index + 1);
    let column = u32::try_from(prefix[line_start..].chars().count() + 1)
        .map_err(|_| ParseError::InvalidSpan)?;
    SourcePosition::new(line, column).map_err(|_| ParseError::InvalidSpan)
}

/// Bounded parser failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// Input exceeded the one-document byte limit.
    DocumentTooLarge,
    /// Input was not valid UTF-8.
    InvalidUtf8,
    /// TOML syntax or duplicate-key parsing failed.
    Syntax(TomlSyntaxError),
    /// A requested or parser-provided byte span was invalid.
    InvalidSpan,
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(error) => formatter.write_str(error.message()),
            other => write!(formatter, "{other:?}"),
        }
    }
}
impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn source() -> Result<SourceName, eqm_domain::DiagnosticBuildError> {
        SourceName::new("eqm/example.toml")
    }

    #[test]
    fn valid_unicode_and_spans_are_stable() -> Result<(), Box<dyn Error>> {
        let parsed = parse_toml(source()?, "title = \"Café\"\ncount = 1\n".as_bytes())?;
        assert_eq!(
            parsed.root().get("count").and_then(toml::Value::as_integer),
            Some(1)
        );
        let start = parsed.text().find("count").ok_or("count")?;
        let location = parsed.location(start..start + 5)?;
        assert_eq!(location.start().line(), 2);
        assert_eq!(location.start().column(), 1);
        assert_eq!(location.end().column(), 6);
        Ok(())
    }

    #[test]
    fn syntax_and_duplicate_keys_retain_locations() -> Result<(), Box<dyn Error>> {
        for invalid in ["key = [", "key = 1\nkey = 2\n"] {
            let error = match parse_toml(source()?, invalid.as_bytes()) {
                Err(ParseError::Syntax(error)) => error,
                _ => return Err("invalid TOML did not produce a syntax error".into()),
            };
            assert!(!error.message().is_empty());
            assert!(error.location().is_some());
        }
        Ok(())
    }

    #[test]
    fn utf8_size_and_invalid_ranges_fail_without_panics() -> Result<(), Box<dyn Error>> {
        assert_eq!(parse_toml(source()?, &[0xff]), Err(ParseError::InvalidUtf8));
        assert_eq!(
            parse_toml(source()?, &vec![b'a'; MAX_DOCUMENT_BYTES + 1]),
            Err(ParseError::DocumentTooLarge)
        );
        let parsed = parse_toml(source()?, b"key = 1")?;
        assert_eq!(parsed.location(10..11), Err(ParseError::InvalidSpan));
        Ok(())
    }
}
