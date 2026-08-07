//! Exact current-v1 public protocol schema identities.

use eqm_domain::{SchemaKind, SchemaUri};

/// Common result-envelope schema.
pub const RESULT_SCHEMA: SchemaUri = SchemaUri::new(SchemaKind::Result);
/// Public diagnostic schema.
pub const DIAGNOSTIC_SCHEMA: SchemaUri = SchemaUri::new(SchemaKind::Diagnostic);
/// Normalized test-result schema.
pub const TEST_RESULT_SCHEMA: SchemaUri = SchemaUri::new(SchemaKind::TestResult);
/// Immutable evidence-result schema.
pub const EVIDENCE_RESULT_SCHEMA: SchemaUri = SchemaUri::new(SchemaKind::EvidenceResult);
/// Adapter inventory schema.
pub const INVENTORY_SCHEMA: SchemaUri = SchemaUri::new(SchemaKind::Inventory);
/// Runtime-facts schema.
pub const RUNTIME_FACTS_SCHEMA: SchemaUri = SchemaUri::new(SchemaKind::RuntimeFacts);
/// Exact release-record schema.
pub const RELEASE_RECORD_SCHEMA: SchemaUri = SchemaUri::new(SchemaKind::ReleaseRecord);
/// EQM attestation-predicate schema.
pub const ATTESTATION_SCHEMA: SchemaUri = SchemaUri::new(SchemaKind::Attestation);
/// Adapter-request schema.
pub const ADAPTER_REQUEST_SCHEMA: SchemaUri = SchemaUri::new(SchemaKind::AdapterRequest);
/// Adapter-response schema.
pub const ADAPTER_RESPONSE_SCHEMA: SchemaUri = SchemaUri::new(SchemaKind::AdapterResponse);

/// Every EQM-owned protocol schema in stable schema-kind order.
pub const PROTOCOL_SCHEMAS: [SchemaUri; 10] = [
    ADAPTER_REQUEST_SCHEMA,
    ADAPTER_RESPONSE_SCHEMA,
    ATTESTATION_SCHEMA,
    DIAGNOSTIC_SCHEMA,
    EVIDENCE_RESULT_SCHEMA,
    INVENTORY_SCHEMA,
    RELEASE_RECORD_SCHEMA,
    RESULT_SCHEMA,
    RUNTIME_FACTS_SCHEMA,
    TEST_RESULT_SCHEMA,
];

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::SchemaVersion;
    use std::collections::BTreeSet;

    #[test]
    fn protocol_schemas_are_unique_exact_and_current() {
        let values: Vec<_> = PROTOCOL_SCHEMAS.iter().map(ToString::to_string).collect();
        let unique: BTreeSet<_> = values.iter().collect();
        assert_eq!(unique.len(), PROTOCOL_SCHEMAS.len());
        assert!(
            values
                .iter()
                .all(|value| value.starts_with("https://schemas.equivalencematrix.dev/v1/"))
        );
        assert!(
            PROTOCOL_SCHEMAS
                .iter()
                .all(|schema| schema.version() == SchemaVersion::V1)
        );
        assert_eq!(RESULT_SCHEMA.kind(), SchemaKind::Result);
    }
}
