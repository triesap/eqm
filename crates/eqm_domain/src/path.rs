//! Lexically validated portable repository paths.

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use unicode_normalization::UnicodeNormalization;

const MAX_PATH_BYTES: usize = 1_024;
const MAX_SEGMENTS: usize = 128;

/// A normalized repository-relative path using `/` separators.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RepoPath(Box<str>);

impl RepoPath {
    /// Validates a path without touching the filesystem.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RepoPathError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(RepoPathError::Empty);
        }
        if value.len() > MAX_PATH_BYTES {
            return Err(RepoPathError::TooLong);
        }
        if value.starts_with('/') || value.starts_with("//") {
            return Err(RepoPathError::Absolute);
        }
        if value.contains('\\') {
            return Err(RepoPathError::InvalidSeparator);
        }
        if value.chars().any(char::is_control) {
            return Err(RepoPathError::ControlCharacter);
        }
        if value.nfc().ne(value.chars()) {
            return Err(RepoPathError::NotNormalized);
        }

        let mut count = 0_usize;
        for (index, segment) in value.split('/').enumerate() {
            count += 1;
            if count > MAX_SEGMENTS {
                return Err(RepoPathError::TooManySegments);
            }
            if segment.is_empty() {
                return Err(RepoPathError::EmptySegment);
            }
            if segment == "." || segment == ".." {
                return Err(RepoPathError::Traversal);
            }
            if index == 0 && is_drive_prefix(segment) {
                return Err(RepoPathError::DrivePrefix);
            }
            if segment.ends_with([' ', '.'])
                || segment.contains([':', '*', '?', '"', '<', '>', '|'])
                || is_reserved_device(segment)
            {
                return Err(RepoPathError::NonPortableSegment);
            }
        }
        Ok(Self(value.into()))
    }

    /// Returns the normalized wire path.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the deterministic key used to detect portable collisions.
    #[must_use]
    pub fn portable_collision_key(&self) -> Box<str> {
        self.0
            .chars()
            .map(|character| character.to_ascii_lowercase())
            .collect::<String>()
            .into_boxed_str()
    }
}

impl Display for RepoPath {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RepoPath {
    type Err = RepoPathError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

fn is_drive_prefix(segment: &str) -> bool {
    let bytes = segment.as_bytes();
    matches!(bytes, [letter, b':', ..] if letter.is_ascii_alphabetic())
}

fn is_reserved_device(segment: &str) -> bool {
    let stem = segment.split('.').next().unwrap_or_default();
    let upper = stem.to_ascii_uppercase();
    matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || upper
            .strip_prefix("COM")
            .or_else(|| upper.strip_prefix("LPT"))
            .is_some_and(|suffix| {
                matches!(suffix, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
            })
}

/// Repository path validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RepoPathError {
    /// The path was empty.
    Empty,
    /// The path exceeded 1,024 UTF-8 bytes.
    TooLong,
    /// The path was absolute or UNC-shaped.
    Absolute,
    /// A backslash separator was used.
    InvalidSeparator,
    /// The path contained a control character or NUL.
    ControlCharacter,
    /// The path was not Unicode NFC.
    NotNormalized,
    /// The path contained an empty segment.
    EmptySegment,
    /// The path contained `.` or `..` traversal.
    Traversal,
    /// The first segment was drive-relative or drive-absolute.
    DrivePrefix,
    /// The path exceeded 128 segments.
    TooManySegments,
    /// A segment is not portable across supported filesystems.
    NonPortableSegment,
}

impl Display for RepoPathError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Empty => "repository path is empty",
            Self::TooLong => "repository path exceeds 1,024 bytes",
            Self::Absolute => "repository path is absolute",
            Self::InvalidSeparator => "repository path uses a backslash separator",
            Self::ControlCharacter => "repository path contains a control character",
            Self::NotNormalized => "repository path is not Unicode NFC",
            Self::EmptySegment => "repository path contains an empty segment",
            Self::Traversal => "repository path contains traversal",
            Self::DrivePrefix => "repository path contains a drive prefix",
            Self::TooManySegments => "repository path exceeds 128 segments",
            Self::NonPortableSegment => "repository path contains a non-portable segment",
        })
    }
}

impl Error for RepoPathError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_normalized_repository_paths() -> Result<(), RepoPathError> {
        for value in [
            "eqm.toml",
            "eqm/contracts/signup.toml",
            "apps/café/view.swift",
        ] {
            assert_eq!(RepoPath::new(value)?.as_str(), value);
        }
        Ok(())
    }

    #[test]
    fn rejects_escape_separator_collision_and_normalization_hazards() {
        let cases = [
            ("", RepoPathError::Empty, "repository path is empty"),
            (
                "/etc/passwd",
                RepoPathError::Absolute,
                "repository path is absolute",
            ),
            (
                "C:relative",
                RepoPathError::DrivePrefix,
                "repository path contains a drive prefix",
            ),
            (
                "C:/absolute",
                RepoPathError::DrivePrefix,
                "repository path contains a drive prefix",
            ),
            (
                "eqm\\file",
                RepoPathError::InvalidSeparator,
                "repository path uses a backslash separator",
            ),
            (
                "eqm//file",
                RepoPathError::EmptySegment,
                "repository path contains an empty segment",
            ),
            (
                "eqm/./file",
                RepoPathError::Traversal,
                "repository path contains traversal",
            ),
            (
                "eqm/../file",
                RepoPathError::Traversal,
                "repository path contains traversal",
            ),
            (
                "eqm/file\0",
                RepoPathError::ControlCharacter,
                "repository path contains a control character",
            ),
            (
                "eqm/file.",
                RepoPathError::NonPortableSegment,
                "repository path contains a non-portable segment",
            ),
            (
                "eqm/NUL.txt",
                RepoPathError::NonPortableSegment,
                "repository path contains a non-portable segment",
            ),
            (
                "apps/cafe\u{301}/view",
                RepoPathError::NotNormalized,
                "repository path is not Unicode NFC",
            ),
        ];
        for (value, error, diagnostic) in cases {
            assert_eq!(RepoPath::new(value), Err(error), "accepted {value:?}");
            assert_eq!(error.to_string(), diagnostic);
        }
    }

    #[test]
    fn enforces_byte_and_segment_limits() {
        assert_eq!(
            RepoPath::new("a".repeat(1_025)),
            Err(RepoPathError::TooLong)
        );
        assert_eq!(
            RepoPath::new(vec!["a"; 129].join("/")),
            Err(RepoPathError::TooManySegments)
        );
        assert!(RepoPath::new(vec!["a"; 128].join("/")).is_ok());
    }

    #[test]
    fn portable_key_folds_ascii_case_without_changing_unicode() -> Result<(), RepoPathError> {
        let upper = RepoPath::new("Apps/Café/View.swift")?;
        let lower = RepoPath::new("apps/Café/view.swift")?;
        assert_eq!(
            upper.portable_collision_key(),
            lower.portable_collision_key()
        );
        assert_ne!(
            RepoPath::new("apps/CAFÉ/view.swift")?.portable_collision_key(),
            lower.portable_collision_key()
        );
        Ok(())
    }
}
