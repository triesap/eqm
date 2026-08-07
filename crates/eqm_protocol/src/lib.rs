//! Public protocol data transfer objects for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod report;

pub use report::{
    CommandIdentity, CommandResultDto, DiagnosticDto, EvaluationModeDto, InvocationContextDto,
    ProfileValueDto, ReportBuildError, ReportEnvelope, ResultSchema, SeverityDto,
    SourceLocationDto, SourcePositionDto, ToolVersionDto,
};
