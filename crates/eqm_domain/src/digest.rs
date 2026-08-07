//! Fixed SHA-256 content and semantic digests.

use sha2::{Digest as _, Sha256};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

const PREFIX: &str = "sha256:";
const HEX_LENGTH: usize = 64;

/// An approved domain-separation label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DigestDomain {
    /// Finalized EQM v1 semantic graph identity.
    SemanticGraph,
}

impl DigestDomain {
    /// Returns the exact bytes prepended before the zero delimiter.
    #[must_use]
    pub const fn as_bytes(self) -> &'static [u8] {
        match self {
            Self::SemanticGraph => b"eqm:v1:semantic-graph",
        }
    }
}

/// A fixed 32-byte SHA-256 digest.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sha256Digest([u8; 32]);

impl Sha256Digest {
    /// Creates a digest from exact raw bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Hashes raw content without a semantic domain label.
    #[must_use]
    pub fn hash_content(content: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content);
        Self(hasher.finalize().into())
    }

    /// Hashes the approved domain label, zero delimiter, and content bytes.
    #[must_use]
    pub fn hash_domain(domain: DigestDomain, content: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(domain.as_bytes());
        hasher.update([0]);
        hasher.update(content);
        Self(hasher.finalize().into())
    }

    /// Returns the exact 32 bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Consumes the value and returns its exact bytes.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; 32] {
        self.0
    }
}

impl Display for Sha256Digest {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(PREFIX)?;
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl FromStr for Sha256Digest {
    type Err = DigestParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Some(hex) = value.strip_prefix(PREFIX) else {
            return Err(DigestParseError::MissingPrefix);
        };
        if hex.len() != HEX_LENGTH {
            return Err(DigestParseError::WrongLength);
        }
        if hex.bytes().any(|byte| byte.is_ascii_uppercase()) {
            return Err(DigestParseError::UppercaseHex);
        }

        let mut bytes = [0_u8; 32];
        for (index, pair) in hex.as_bytes().chunks_exact(2).enumerate() {
            let high = decode_hex(pair[0]).ok_or(DigestParseError::InvalidHex)?;
            let low = decode_hex(pair[1]).ok_or(DigestParseError::InvalidHex)?;
            bytes[index] = (high << 4) | low;
        }
        Ok(Self(bytes))
    }
}

fn decode_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

/// SHA-256 wire parsing failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DigestParseError {
    /// The exact `sha256:` prefix was absent.
    MissingPrefix,
    /// The hexadecimal payload was not exactly 64 bytes.
    WrongLength,
    /// Uppercase hexadecimal is not canonical.
    UppercaseHex,
    /// The payload contained a non-hexadecimal byte.
    InvalidHex,
}

impl Display for DigestParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::MissingPrefix => "SHA-256 digest requires the sha256: prefix",
            Self::WrongLength => "SHA-256 digest requires 64 hexadecimal digits",
            Self::UppercaseHex => "SHA-256 digest requires lowercase hexadecimal",
            Self::InvalidHex => "SHA-256 digest contains invalid hexadecimal",
        })
    }
}

impl Error for DigestParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_GRAPH: &[u8] = br#"{"adapters":[],"bindings":[],"capabilities":[],"extensions":{},"fragments":[],"imports":[],"journeys":[],"policies":[],"profiles":[],"runners":[],"schema":"https://schemas.equivalencematrix.dev/v1/semantic-graph","surfaces":[],"targets":[],"waivers":[]}"#;

    #[test]
    fn content_hash_matches_standard_empty_vector() {
        assert_eq!(
            Sha256Digest::hash_content(b"").to_string(),
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn semantic_domain_matches_authority_vector() {
        assert_eq!(
            Sha256Digest::hash_domain(DigestDomain::SemanticGraph, EMPTY_GRAPH).to_string(),
            "sha256:2323afb42c366664f47a5f90c597c7968f651f74f875ed95aec4dcc02283994c"
        );
    }

    #[test]
    fn bytes_display_and_parse_round_trip() {
        let digest = Sha256Digest::from_bytes([0xab; 32]);
        let rendered = digest.to_string();
        assert_eq!(rendered.parse(), Ok(digest));
        assert_eq!(digest.as_bytes(), &[0xab; 32]);
        assert_eq!(digest.into_bytes(), [0xab; 32]);
    }

    #[test]
    fn malformed_digest_forms_fail_closed() {
        for (value, error) in [
            ("", DigestParseError::MissingPrefix),
            ("SHA256:00", DigestParseError::MissingPrefix),
            ("sha256:00", DigestParseError::WrongLength),
            (
                "sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                DigestParseError::UppercaseHex,
            ),
            (
                "sha256:gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg",
                DigestParseError::InvalidHex,
            ),
        ] {
            assert_eq!(value.parse::<Sha256Digest>(), Err(error));
        }
    }

    #[test]
    fn semantic_domain_is_not_raw_content_hashing() {
        assert_ne!(
            Sha256Digest::hash_domain(DigestDomain::SemanticGraph, EMPTY_GRAPH),
            Sha256Digest::hash_content(EMPTY_GRAPH)
        );
    }
}
