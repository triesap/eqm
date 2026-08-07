//! Public protocol data transfer objects for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod adapter;
mod attestation;
mod evidence;
mod records;
mod report;
mod results;
mod sarif;
mod schema;

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
pub use records::{ReleaseRecordDto, RuntimeFactDto, RuntimeFactsDto};
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
    ADAPTER_REQUEST_SCHEMA, ADAPTER_RESPONSE_SCHEMA, ATTESTATION_SCHEMA, DIAGNOSTIC_SCHEMA,
    EVIDENCE_RESULT_SCHEMA, INVENTORY_SCHEMA, PROTOCOL_SCHEMAS, RELEASE_RECORD_SCHEMA,
    RESULT_SCHEMA, RUNTIME_FACTS_SCHEMA, TEST_RESULT_SCHEMA,
};
