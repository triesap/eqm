//! Public protocol data transfer objects for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod report;
mod results;
mod schema;

pub use report::{
    CommandIdentity, CommandResultDto, DiagnosticDto, EvaluationModeDto, InvocationContextDto,
    ProfileValueDto, ReportBuildError, ReportEnvelope, ResultSchema, SeverityDto,
    SourceLocationDto, SourcePositionDto, ToolVersionDto,
};
pub use results::*;
pub use schema::{
    ADAPTER_REQUEST_SCHEMA, ADAPTER_RESPONSE_SCHEMA, ATTESTATION_SCHEMA, DIAGNOSTIC_SCHEMA,
    EVIDENCE_RESULT_SCHEMA, INVENTORY_SCHEMA, PROTOCOL_SCHEMAS, RELEASE_RECORD_SCHEMA,
    RESULT_SCHEMA, RUNTIME_FACTS_SCHEMA, TEST_RESULT_SCHEMA,
};
