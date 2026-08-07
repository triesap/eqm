//! Validated identifiers for EQM authority and graph entities.

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

const MAX_SEGMENT_BYTES: usize = 63;
const MAX_ID_BYTES: usize = 255;
const MAX_REQUIREMENT_ID_BYTES: usize = 320;

fn validate_id(
    value: &str,
    minimum_segments: usize,
    maximum_segments: Option<usize>,
) -> Result<(), IdParseError> {
    if value.is_empty() {
        return Err(IdParseError::Empty);
    }
    if value.len() > MAX_ID_BYTES {
        return Err(IdParseError::TooLong);
    }
    let mut count = 0_usize;
    for segment in value.split('.') {
        count += 1;
        if segment.is_empty() {
            return Err(IdParseError::EmptySegment);
        }
        if segment.len() > MAX_SEGMENT_BYTES {
            return Err(IdParseError::SegmentTooLong);
        }
        let mut bytes = segment.bytes();
        if !matches!(bytes.next(), Some(first) if first.is_ascii_lowercase()) {
            return Err(IdParseError::InvalidCharacter);
        }
        if !bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_') {
            return Err(IdParseError::InvalidCharacter);
        }
    }
    if count < minimum_segments {
        return Err(IdParseError::TooFewSegments);
    }
    if maximum_segments.is_some_and(|maximum| count > maximum) {
        return Err(IdParseError::TooManySegments);
    }
    Ok(())
}

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident, $minimum:literal, $maximum:expr) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(Box<str>);

        impl $name {
            /// Parses and validates this identifier type.
            pub fn new(value: impl Into<Box<str>>) -> Result<Self, IdParseError> {
                let value = value.into();
                validate_id(&value, $minimum, $maximum)?;
                Ok(Self(value))
            }

            /// Returns the exact wire value.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = IdParseError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
    };
}

define_id!(
    /// A capability authority ID.
    CapabilityId,
    2,
    None
);
define_id!(
    /// A journey authority ID.
    JourneyId,
    3,
    None
);
define_id!(
    /// A surface authority ID.
    SurfaceId,
    4,
    None
);
define_id!(
    /// A reusable fragment authority ID.
    FragmentId,
    2,
    None
);
define_id!(
    /// A target ID declared by the workspace.
    TargetId,
    1,
    None
);
define_id!(
    /// A target binding authority ID.
    BindingId,
    2,
    None
);
define_id!(
    /// A policy authority ID.
    PolicyId,
    2,
    None
);
define_id!(
    /// A profile authority ID.
    ProfileId,
    2,
    None
);
define_id!(
    /// A runner authority ID.
    RunnerId,
    2,
    None
);
define_id!(
    /// A waiver authority ID.
    WaiverId,
    2,
    None
);
define_id!(
    /// A shared-provider authority ID.
    ProviderId,
    2,
    None
);
define_id!(
    /// A locked adapter authority ID.
    AdapterId,
    2,
    None
);
define_id!(
    /// A capability, journey, surface, or fragment reference ID.
    UnitId,
    2,
    None
);
define_id!(
    /// A local artifact ID within one binding.
    ArtifactId,
    1,
    Some(1)
);
define_id!(
    /// A local evidence-specification ID within one binding.
    EvidenceSpecId,
    1,
    Some(1)
);
define_id!(
    /// A local requirement ID within one surface or fragment.
    LocalRequirementId,
    1,
    Some(1)
);

/// A fully qualified requirement identity: `<unit>#<local>`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FullRequirementId {
    value: Box<str>,
    separator: usize,
}

impl FullRequirementId {
    /// Parses a fully qualified requirement identity.
    pub fn new(value: impl Into<Box<str>>) -> Result<Self, IdParseError> {
        let value = value.into();
        if value.len() > MAX_REQUIREMENT_ID_BYTES {
            return Err(IdParseError::TooLong);
        }
        let mut separators = value.match_indices('#');
        let Some((separator, _)) = separators.next() else {
            return Err(IdParseError::MissingQualification);
        };
        if separators.next().is_some() {
            return Err(IdParseError::MultipleQualifications);
        }
        validate_id(&value[..separator], 2, None)?;
        validate_id(&value[separator + 1..], 1, Some(1))?;
        Ok(Self { value, separator })
    }

    /// Returns the exact wire value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the fully qualified owning unit or fragment ID.
    #[must_use]
    pub fn owner(&self) -> &str {
        &self.value[..self.separator]
    }

    /// Returns the local requirement segment.
    #[must_use]
    pub fn local(&self) -> &str {
        &self.value[self.separator + 1..]
    }
}

impl Display for FullRequirementId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FullRequirementId {
    type Err = IdParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Identifier validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdParseError {
    /// The identifier was empty.
    Empty,
    /// The complete identifier exceeded its byte limit.
    TooLong,
    /// A segment was empty.
    EmptySegment,
    /// A segment exceeded 63 ASCII bytes.
    SegmentTooLong,
    /// A character or initial character violated the ASCII grammar.
    InvalidCharacter,
    /// The identifier had fewer segments than its type requires.
    TooFewSegments,
    /// The identifier had more segments than its type permits.
    TooManySegments,
    /// A full requirement lacked `#` qualification.
    MissingQualification,
    /// A full requirement contained more than one `#`.
    MultipleQualifications,
}

impl Display for IdParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Empty => "identifier is empty",
            Self::TooLong => "identifier exceeds its byte limit",
            Self::EmptySegment => "identifier contains an empty segment",
            Self::SegmentTooLong => "identifier segment exceeds 63 bytes",
            Self::InvalidCharacter => "identifier violates the lowercase ASCII grammar",
            Self::TooFewSegments => "identifier has too few segments for its type",
            Self::TooManySegments => "identifier has too many segments for its type",
            Self::MissingQualification => "requirement identifier is not fully qualified",
            Self::MultipleQualifications => "requirement identifier has multiple qualifiers",
        })
    }
}

impl Error for IdParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_types_enforce_segment_depth() {
        assert!(CapabilityId::new("account.create").is_ok());
        assert_eq!(
            CapabilityId::new("account"),
            Err(IdParseError::TooFewSegments)
        );
        assert!(JourneyId::new("account.create.signup").is_ok());
        assert_eq!(
            JourneyId::new("account.create"),
            Err(IdParseError::TooFewSegments)
        );
        assert!(SurfaceId::new("account.create.signup.identifier").is_ok());
        assert_eq!(
            SurfaceId::new("account.create.signup"),
            Err(IdParseError::TooFewSegments)
        );
        assert!(TargetId::new("web").is_ok());
        assert!(LocalRequirementId::new("visible").is_ok());
        assert_eq!(
            LocalRequirementId::new("form.visible"),
            Err(IdParseError::TooManySegments)
        );
    }

    #[test]
    fn grammar_and_boundaries_fail_closed() {
        let long_segment = "a".repeat(64);
        let long_id = format!("a.{}", "b".repeat(254));
        for invalid in [
            "", "A.b", "1a.b", "a-b.c", "a..b", ".a.b", "a.b.", "a/b.c", "a#b.c",
        ] {
            assert!(CapabilityId::new(invalid).is_err(), "accepted {invalid}");
        }
        assert_eq!(
            CapabilityId::new(format!("a.{long_segment}")),
            Err(IdParseError::SegmentTooLong)
        );
        assert_eq!(CapabilityId::new(long_id), Err(IdParseError::TooLong));
        assert!(CapabilityId::new(format!("a.{}", "b".repeat(63))).is_ok());
    }

    #[test]
    fn generated_ascii_segments_match_the_reference_grammar() {
        for first in 0_u8..=127 {
            for second in [None, Some(b'a'), Some(b'0'), Some(b'_'), Some(b'-')] {
                let mut bytes = vec![first];
                if let Some(value) = second {
                    bytes.push(value);
                }
                let Ok(segment) = String::from_utf8(bytes) else {
                    continue;
                };
                let expected = segment
                    .as_bytes()
                    .first()
                    .is_some_and(u8::is_ascii_lowercase)
                    && segment.as_bytes().iter().skip(1).all(|byte| {
                        byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_'
                    });
                assert_eq!(LocalRequirementId::new(segment).is_ok(), expected);
            }
        }
    }

    #[test]
    fn full_requirement_is_exact_and_splittable() -> Result<(), IdParseError> {
        let full = FullRequirementId::new("account.create.signup.form#phone_visible")?;
        assert_eq!(full.owner(), "account.create.signup.form");
        assert_eq!(full.local(), "phone_visible");
        assert_eq!(full.to_string(), "account.create.signup.form#phone_visible");
        for invalid in [
            "phone_visible",
            "account.create#",
            "#phone_visible",
            "account.create#a#b",
            "account.create#a.b",
        ] {
            assert!(
                FullRequirementId::new(invalid).is_err(),
                "accepted {invalid}"
            );
        }
        Ok(())
    }

    #[test]
    fn ordering_and_hash_identity_use_exact_wire_values() -> Result<(), IdParseError> {
        let mut values = [
            TargetId::new("web_b")?,
            TargetId::new("web")?,
            TargetId::new("web_a")?,
        ];
        values.sort();
        let rendered: Vec<&str> = values.iter().map(TargetId::as_str).collect();
        assert_eq!(rendered, ["web", "web_a", "web_b"]);
        Ok(())
    }
}
