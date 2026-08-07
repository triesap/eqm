//! Pure validated domain types for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod diagnostic;
mod digest;
mod id;
mod path;
mod schema;

pub use diagnostic::{
    Diagnostic, DiagnosticBuildError, DiagnosticCode, DiagnosticDescriptor,
    DiagnosticRegistryError, Severity, SourceLocation, SourceName, SourcePosition,
    validate_diagnostic_registry,
};
pub use digest::{DigestDomain, DigestParseError, Sha256Digest};
pub use id::{
    AdapterId, ArtifactId, BindingId, CapabilityId, EvidenceSpecId, FragmentId, FullRequirementId,
    IdParseError, JourneyId, LocalRequirementId, PolicyId, ProfileId, ProviderId, RunnerId,
    SurfaceId, TargetId, UnitId, WaiverId,
};
pub use path::{RepoPath, RepoPathError};
pub use schema::{SchemaKind, SchemaParseError, SchemaUri, SchemaVersion, ToolVersion};
