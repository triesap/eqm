//! Public protocol data transfer objects for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod adapter;
mod admission;
mod attestation;
mod canonical;
mod evidence;
mod records;
mod report;
mod results;
mod sarif;
mod schema;
mod strict;
pub mod v2;
mod wire;

pub use admission::{
    AdmittedEvidenceV2, EvidenceAdmissionContext, EvidenceAdmissionError, admit_evidence,
};
pub use canonical::{
    CanonicalCollectionError, CanonicalSet, canonical_member_bytes, is_authored_nfc,
};
pub use strict::{
    MAX_JSON_BYTES, StrictJsonError, from_slice as strict_from_slice,
    from_slice_limited as strict_from_slice_limited, parse_strict, parse_strict_limited,
};
pub use v2::*;
pub use wire::{MAX_SAFE_INTEGER, WireFormError, WireInt, WireUint, exact_signed, exact_unsigned};

pub use adapter::{
    AdapterDtoError, AdapterLimitsDto, AdapterOperationDto, AdapterRequestDto, AdapterResponseDto,
    AdapterStatusDto, FactValueDto, InventoryDto, InventoryEntryDto,
};
pub use attestation::{
    AttestationDtoError, AttestationPredicateDto, AttestationSubjectDto, DSSE_PAYLOAD_TYPE,
    DsseEnvelopeDto, DsseSignatureDto, IN_TOTO_STATEMENT_V1, InTotoStatementDto, SubjectDigestDto,
};
pub use evidence::{
    AttachmentDto, AttemptDto, CountsDto, EvidenceDtoError, EvidencePayloadDto, EvidenceResultDto,
    EvidenceSelectorDto, EvidenceSubjectDto, ExecutionPayloadDto, ScopeSubjectDto, TestResultDto,
};
pub use records::{ReleaseRecordDto, RuntimeFactDto, RuntimeFactsDto, RuntimeFactsDtoError};
pub use report::{
    CommandIdentity, CommandResultDto, DiagnosticDto, EvaluationModeDto, InvocationContextDto,
    ProfileValueDto, ReportBuildError, ReportEnvelope, ResultSchema, SeverityDto,
    SourceLocationDto, SourcePositionDto, ToolVersionDto,
};
pub use results::*;
pub use sarif::{
    SARIF_SCHEMA, SarifArtifactLocationDto, SarifDriverDto, SarifLocationDto, SarifLogDto,
    SarifMessageDto, SarifPhysicalLocationDto, SarifRegionDto, SarifResultDto, SarifRuleDto,
    SarifRunDto, SarifToolDto,
};
pub use schema::{
    ADAPTER_REQUEST_SCHEMA, ADAPTER_REQUEST_SCHEMA_V2, ADAPTER_RESPONSE_SCHEMA,
    ADAPTER_RESPONSE_SCHEMA_V2, ATTESTATION_SCHEMA, DIAGNOSTIC_SCHEMA, EVIDENCE_RESULT_SCHEMA,
    EVIDENCE_RESULT_SCHEMA_V2, INVENTORY_SCHEMA, INVENTORY_SCHEMA_V2, PROTOCOL_SCHEMAS,
    RELEASE_RECORD_SCHEMA, RESULT_SCHEMA, RUNTIME_FACTS_SCHEMA, RUNTIME_FACTS_SCHEMA_V2,
    TEST_RESULT_SCHEMA, V2_PROTOCOL_SCHEMAS, protocol_schemas,
};
