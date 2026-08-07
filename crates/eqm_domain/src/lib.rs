//! Pure validated domain types for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod diagnostic;
mod id;
mod schema;

pub use diagnostic::{
    Diagnostic, DiagnosticBuildError, DiagnosticCode, DiagnosticDescriptor,
    DiagnosticRegistryError, Severity, SourceLocation, SourceName, SourcePosition,
    validate_diagnostic_registry,
};
pub use id::{
    AdapterId, ArtifactId, BindingId, CapabilityId, EvidenceSpecId, FragmentId, FullRequirementId,
    IdParseError, JourneyId, LocalRequirementId, PolicyId, ProfileId, ProviderId, RunnerId,
    SurfaceId, TargetId, UnitId, WaiverId,
};
pub use schema::{SchemaKind, SchemaParseError, SchemaUri, SchemaVersion, ToolVersion};
