//! Executes the frozen ADR-0013 numeric vectors against the wire types.
//!
//! Signed/unsigned kind is declared per vector id here so the frozen vector
//! text stays literal; the mapping is explicit and reviewable.

use eqm_protocol::{WireInt, WireUint};
use eqm_test_support::{REPRESENTATION_VECTORS, VectorOutcome};
use serde_json::Value;
use std::error::Error;

#[derive(Clone, Copy)]
enum WireKind {
    Unsigned,
    Signed,
}

fn wire_kind(id: &str) -> Option<WireKind> {
    match id {
        "RV-INTEGER-1" | "RV-INTEGER-2" | "RV-INTEGER-3" | "RV-INTEGER-4" | "RV-INTEGER-6"
        | "RV-DECIMAL-1" | "RV-DECIMAL-3" | "RV-DECIMAL-4" => Some(WireKind::Unsigned),
        "RV-INTEGER-5" | "RV-DECIMAL-2" => Some(WireKind::Signed),
        _ => None,
    }
}

#[test]
fn numeric_vectors_are_executable_controls() -> Result<(), Box<dyn Error>> {
    let mut checked = 0;
    for vector in REPRESENTATION_VECTORS {
        let Some(kind) = wire_kind(vector.id) else {
            continue;
        };
        checked += 1;
        let value: Value = serde_json::from_str(vector.input)?;
        let rendered = match kind {
            WireKind::Unsigned => serde_json::from_value::<WireUint>(value)
                .ok()
                .map(|parsed| serde_json::to_string(&parsed))
                .transpose()?,
            WireKind::Signed => serde_json::from_value::<WireInt>(value)
                .ok()
                .map(|parsed| serde_json::to_string(&parsed))
                .transpose()?,
        };
        match vector.outcome {
            VectorOutcome::Accept { output } => {
                assert_eq!(rendered.as_deref(), Some(output), "{}", vector.id);
            }
            VectorOutcome::Reject => {
                assert!(rendered.is_none(), "{} was accepted", vector.id);
            }
        }
    }
    assert!(checked >= 9, "expected the full numeric vector set");
    Ok(())
}
