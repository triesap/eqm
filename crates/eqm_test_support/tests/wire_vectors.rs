//! Executes every frozen wire-discipline representation vector against the
//! exact control its declared discipline names.
//!
//! Dispatch is by `VectorDiscipline`, never by vector ID or a minimum count, so
//! a newly added applicable vector cannot be silently skipped.

use eqm_domain::{Sha256Digest, UtcInstantV2};
use eqm_protocol::{WireInt, WireUint, v2};
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
