//! Executes every frozen wire-discipline representation vector against the
//! exact control its declared discipline names.
//!
//! Dispatch is by `VectorDiscipline`, never by vector ID or a minimum count, so
//! a newly added applicable vector cannot be silently skipped.

use eqm_domain::{RawObjectDigest, Sha256Digest, UtcInstantV2};
use eqm_protocol::{CanonicalSet, WireInt, WireUint, is_authored_nfc, v2};
use eqm_test_support::{
    REPRESENTATION_VECTORS, RepresentationVector, VectorDiscipline, VectorOutcome,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::str::FromStr;

/// Minimal v2 absence probe: one optional member that is omitted when absent
/// and rejected when explicitly `null`.
#[derive(Debug, Deserialize, Serialize)]
struct AbsenceProbe {
    #[serde(
        default,
        deserialize_with = "v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    optional: Option<String>,
}

#[test]
fn every_applicable_representation_vector_executes() -> Result<(), Box<dyn Error>> {
    let applicable = REPRESENTATION_VECTORS
        .iter()
        .filter(|vector| vector.discipline != VectorDiscipline::SemanticRule)
        .count();
    let mut executed = 0;
    for vector in REPRESENTATION_VECTORS {
        match vector.discipline {
            // Semantic rules are executed by their later owning steps; the
            // catalog test guarantees they are never wire categories.
            VectorDiscipline::SemanticRule => {}
            VectorDiscipline::UnsignedInteger => {
                executed += 1;
                run_unsigned(vector)?;
            }
            VectorDiscipline::SignedInteger => {
                executed += 1;
                run_signed(vector)?;
            }
            VectorDiscipline::Timestamp => {
                executed += 1;
                run_timestamp(vector)?;
            }
            VectorDiscipline::Absence => {
                executed += 1;
                run_absence(vector)?;
            }
            VectorDiscipline::Digest => {
                executed += 1;
                run_digest(vector)?;
            }
            VectorDiscipline::Set => {
                executed += 1;
                run_set(vector)?;
            }
            VectorDiscipline::Sequence => {
                executed += 1;
                run_sequence(vector)?;
            }
            VectorDiscipline::AuthoredText => {
                executed += 1;
                run_authored_text(vector)?;
            }
            VectorDiscipline::OpaqueBytes => {
                executed += 1;
                run_opaque_bytes(vector)?;
            }
        }
    }
    assert_eq!(
        executed, applicable,
        "an applicable vector was not executed"
    );
    Ok(())
}

fn check(vector: &RepresentationVector, rendered: Option<String>) -> Result<(), Box<dyn Error>> {
    match vector.outcome {
        VectorOutcome::Accept { output } => {
            assert_eq!(rendered.as_deref(), Some(output), "{}", vector.id);
        }
        VectorOutcome::Reject => {
            assert!(rendered.is_none(), "{} was accepted", vector.id);
        }
    }
    Ok(())
}

fn run_unsigned(vector: &RepresentationVector) -> Result<(), Box<dyn Error>> {
    let Ok(value) = serde_json::from_str::<Value>(vector.input) else {
        return check(vector, None);
    };
    let rendered = match serde_json::from_value::<WireUint>(value) {
        Ok(parsed) => Some(serde_json::to_string(&parsed)?),
        Err(_) => None,
    };
    check(vector, rendered)
}

fn run_signed(vector: &RepresentationVector) -> Result<(), Box<dyn Error>> {
    let Ok(value) = serde_json::from_str::<Value>(vector.input) else {
        return check(vector, None);
    };
    let rendered = match serde_json::from_value::<WireInt>(value) {
        Ok(parsed) => Some(serde_json::to_string(&parsed)?),
        Err(_) => None,
    };
    check(vector, rendered)
}

fn run_timestamp(vector: &RepresentationVector) -> Result<(), Box<dyn Error>> {
    let rendered = UtcInstantV2::from_str(vector.input)
        .ok()
        .map(|value| value.to_string());
    check(vector, rendered)
}

fn run_digest(vector: &RepresentationVector) -> Result<(), Box<dyn Error>> {
    let rendered = Sha256Digest::from_str(vector.input)
        .ok()
        .map(|value| value.to_string());
    check(vector, rendered)
}

/// Executes an absence vector by feeding its literal input into the v2 absence
/// probe and comparing the exact serialized output.
fn run_absence(vector: &RepresentationVector) -> Result<(), Box<dyn Error>> {
    let rendered = match serde_json::from_str::<AbsenceProbe>(vector.input) {
        Ok(probe) => Some(serde_json::to_string(&probe)?),
        Err(_) => None,
    };
    check(vector, rendered)
}

/// Executes a semantic-set vector through the real `CanonicalSet` boundary:
/// permutations are accepted and emitted in canonical member-byte order, and
/// duplicates reject before collection.
fn run_set(vector: &RepresentationVector) -> Result<(), Box<dyn Error>> {
    let rendered = match serde_json::from_str::<Vec<String>>(vector.input) {
        Ok(members) => match CanonicalSet::from_members(members) {
            Ok(set) => Some(serde_json::to_string(&set)?),
            Err(_) => None,
        },
        Err(_) => None,
    };
    check(vector, rendered)
}

/// Executes an ordered-sequence vector through a plain `Vec`: order and
/// multiplicity are preserved.
fn run_sequence(vector: &RepresentationVector) -> Result<(), Box<dyn Error>> {
    let rendered = match serde_json::from_str::<Vec<String>>(vector.input) {
        Ok(members) => Some(serde_json::to_string(&members)?),
        Err(_) => None,
    };
    check(vector, rendered)
}

/// Executes an authored-text vector: JSON string input, exact NFC acceptance
/// and canonical JSON string output. No normalization is applied.
fn run_authored_text(vector: &RepresentationVector) -> Result<(), Box<dyn Error>> {
    let rendered = match serde_json::from_str::<String>(vector.input) {
        Ok(text) if is_authored_nfc(&text) => Some(serde_json::to_string(&text)?),
        _ => None,
    };
    check(vector, rendered)
}

/// Executes an opaque-byte vector against literal bytes and their exact
/// raw-object domain digest. The input is a byte-escape literal where `\xNN`
/// denotes one raw byte; it is never routed through the authored JSON-string
/// validator.
fn run_opaque_bytes(vector: &RepresentationVector) -> Result<(), Box<dyn Error>> {
    let VectorOutcome::Accept { output } = vector.outcome else {
        return Err(format!("{}: opaque-byte vector must accept", vector.id).into());
    };
    let bytes = decode_byte_literal(vector.input)?;
    let expected = decode_byte_literal(output)?;
    assert_eq!(bytes, expected, "{}: opaque bytes changed", vector.id);
    let digest = RawObjectDigest::hash_content(&bytes).to_string();
    if let Some(expected_digest) = vector.expected_digest {
        assert_eq!(
            digest, expected_digest,
            "{}: opaque raw-object digest",
            vector.id
        );
    }
    Ok(())
}

/// A frozen-vector byte-literal decoding failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ByteLiteralError {
    /// A `\x` escape was truncated.
    TruncatedEscape,
    /// A backslash introduced an unsupported escape.
    InvalidEscape,
}

impl std::fmt::Display for ByteLiteralError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::TruncatedEscape => "byte literal has a truncated \\xNN escape",
            Self::InvalidEscape => "byte literal has an unsupported escape",
        })
    }
}

impl std::error::Error for ByteLiteralError {}

/// Decodes a byte-escape literal: `\xNN` yields one raw byte and every other
/// character contributes its exact UTF-8 bytes. Malformed or truncated escapes
/// are rejected rather than silently dropping characters.
fn decode_byte_literal(literal: &str) -> Result<Vec<u8>, ByteLiteralError> {
    let mut bytes = Vec::with_capacity(literal.len());
    let mut characters = literal.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            let mut buffer = [0_u8; 4];
            bytes.extend_from_slice(character.encode_utf8(&mut buffer).as_bytes());
            continue;
        }
        match characters.next() {
            Some('x') => {
                let high = characters
                    .next()
                    .and_then(|digit| digit.to_digit(16))
                    .ok_or(ByteLiteralError::TruncatedEscape)?;
                let low = characters
                    .next()
                    .and_then(|digit| digit.to_digit(16))
                    .ok_or(ByteLiteralError::TruncatedEscape)?;
                bytes.push((high * 16 + low) as u8);
            }
            _ => return Err(ByteLiteralError::InvalidEscape),
        }
    }
    Ok(bytes)
}

#[test]
fn byte_literal_decoder_rejects_malformed_and_truncated_escapes() {
    assert_eq!(
        decode_byte_literal("cafe\\xcc\\x81").ok(),
        Some(b"cafe\xcc\x81".to_vec())
    );
    for malformed in ["trailing\\", "\\x", "\\x1", "\\xG1", "\\q"] {
        assert!(
            decode_byte_literal(malformed).is_err(),
            "byte-literal decoder accepted {malformed:?}"
        );
    }
}
