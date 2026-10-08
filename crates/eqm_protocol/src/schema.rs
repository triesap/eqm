//! Exact version-qualified public protocol schema identities.

use eqm_domain::{SchemaKind, SchemaUri, SchemaVersion};

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

/// Explicit v2 complete obligation-query result envelope.
pub const RESULT_SCHEMA_V2: SchemaUri = SchemaUri::versioned(SchemaKind::Result, SchemaVersion::V2);

/// Explicit v2 evidence-result schema.
pub const EVIDENCE_RESULT_SCHEMA_V2: SchemaUri =
    SchemaUri::versioned(SchemaKind::EvidenceResult, SchemaVersion::V2);
/// Explicit v2 runtime-facts schema.
pub const RUNTIME_FACTS_SCHEMA_V2: SchemaUri =
    SchemaUri::versioned(SchemaKind::RuntimeFacts, SchemaVersion::V2);
/// Explicit v2 inventory schema.
pub const INVENTORY_SCHEMA_V2: SchemaUri =
    SchemaUri::versioned(SchemaKind::Inventory, SchemaVersion::V2);
/// Explicit v2 adapter-request schema.
pub const ADAPTER_REQUEST_SCHEMA_V2: SchemaUri =
    SchemaUri::versioned(SchemaKind::AdapterRequest, SchemaVersion::V2);
/// Explicit v2 adapter-response schema.
pub const ADAPTER_RESPONSE_SCHEMA_V2: SchemaUri =
    SchemaUri::versioned(SchemaKind::AdapterResponse, SchemaVersion::V2);
/// Explicit v2 subject-manifest schema.
pub const SUBJECT_MANIFEST_SCHEMA_V2: SchemaUri =
    SchemaUri::versioned(SchemaKind::SubjectManifest, SchemaVersion::V2);
/// Explicit v2 semantic-graph schema.
pub const SEMANTIC_GRAPH_SCHEMA_V2: SchemaUri =
    SchemaUri::versioned(SchemaKind::SemanticGraph, SchemaVersion::V2);
/// Explicit v2 comparison-context schema.
pub const COMPARISON_CONTEXT_SCHEMA_V2: SchemaUri =
    SchemaUri::versioned(SchemaKind::ComparisonContext, SchemaVersion::V2);

/// Every v1 EQM-owned protocol schema in stable schema-kind order.
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

/// Every v2 EQM-owned protocol schema in stable schema-kind order.
///
/// S015-R3 registers the five concrete v2 protocol families: evidence-result,
/// runtime-facts, inventory, adapter-request and adapter-response. DEV-009 S038
/// adds the explicit subject-manifest family; DEV-014 P03-R3 registers the
/// typed semantic-graph and comparison-context families once their real member
/// schemas replace the placeholders.
pub const V2_PROTOCOL_SCHEMAS: [SchemaUri; 9] = [
    ADAPTER_REQUEST_SCHEMA_V2,
    ADAPTER_RESPONSE_SCHEMA_V2,
    COMPARISON_CONTEXT_SCHEMA_V2,
    EVIDENCE_RESULT_SCHEMA_V2,
    INVENTORY_SCHEMA_V2,
    RESULT_SCHEMA_V2,
    RUNTIME_FACTS_SCHEMA_V2,
    SEMANTIC_GRAPH_SCHEMA_V2,
    SUBJECT_MANIFEST_SCHEMA_V2,
];

/// Returns the protocol schema inventory for an explicit version.
#[must_use]
pub const fn protocol_schemas(version: SchemaVersion) -> &'static [SchemaUri] {
    match version {
        SchemaVersion::V1 => &PROTOCOL_SCHEMAS,
        SchemaVersion::V2 => &V2_PROTOCOL_SCHEMAS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{SchemaGroup, SchemaVersion};
    use std::collections::BTreeSet;

    #[test]
    fn protocol_schemas_are_unique_exact_and_current() {
        let values: Vec<_> = PROTOCOL_SCHEMAS.iter().map(ToString::to_string).collect();
        let unique: BTreeSet<_> = values.iter().collect();
        assert_eq!(unique.len(), PROTOCOL_SCHEMAS.len());
        assert!(
            PROTOCOL_SCHEMAS
                .iter()
                .all(|schema| schema.kind().group() == SchemaGroup::Protocol)
        );
        assert!(
            PROTOCOL_SCHEMAS
                .iter()
                .all(|schema| schema.version() == SchemaVersion::V1)
        );
        assert_eq!(RESULT_SCHEMA.kind(), SchemaKind::Result);
    }

    #[test]
    fn protocol_inventories_are_version_qualified_and_unique() {
        assert_eq!(protocol_schemas(SchemaVersion::V1), &PROTOCOL_SCHEMAS);
        for version in SchemaVersion::ALL {
            let inventory = protocol_schemas(version);
            let unique: BTreeSet<_> = inventory.iter().map(ToString::to_string).collect();
            assert_eq!(unique.len(), inventory.len());
            assert!(
                inventory.iter().all(|schema| schema.version() == version),
                "inventory for {version} mixes versions"
            );
            assert!(
                inventory
                    .iter()
                    .all(|schema| schema.kind().group() == SchemaGroup::Protocol)
            );
        }
        assert_eq!(
            protocol_schemas(SchemaVersion::V2).len(),
            V2_PROTOCOL_SCHEMAS.len()
        );
        assert!(
            V2_PROTOCOL_SCHEMAS
                .iter()
                .all(|schema| schema.version() == SchemaVersion::V2)
        );
    }
}
