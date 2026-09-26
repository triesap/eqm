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
use serde_json::Value;
use std::error::Error;
use std::str::FromStr;

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

/// Executes the absence scenarios against the real v2 nested subject DTO.
fn run_absence(vector: &RepresentationVector) -> Result<(), Box<dyn Error>> {
    let mut value = serde_json::json!({
        "repository": "https://example.com/team/project",
        "repository_id_digest": Sha256Digest::hash_content(b"repo").to_string(),
        "scope": {"kind": "target", "target": "web"},
        "source_commit": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "target_configuration_digest": Sha256Digest::hash_content(b"config").to_string(),
    });
    match vector.outcome {
        VectorOutcome::Accept { .. } => {
            let subject: v2::EvidenceSubjectV2Dto = serde_json::from_value(value)?;
            let text = serde_json::to_string(&subject)?;
            assert!(
                !text.contains("build_id"),
                "{}: absent optional emitted",
                vector.id
            );
            assert!(
                !text.contains("artifact_digest"),
                "{}: absent optional emitted",
                vector.id
            );
            Ok(())
        }
        VectorOutcome::Reject => {
            value["build_id"] = Value::Null;
            assert!(
                serde_json::from_value::<v2::EvidenceSubjectV2Dto>(value).is_err(),
                "{}: explicit null accepted",
                vector.id
            );
            Ok(())
        }
    }
}
