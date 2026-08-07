//! Pure validated domain types for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod diagnostic;
mod digest;
mod entity;
mod id;
mod path;
mod reference;
mod schema;
mod time;
mod vocabulary;

pub use diagnostic::{
    Diagnostic, DiagnosticBuildError, DiagnosticCode, DiagnosticDescriptor,
    DiagnosticRegistryError, Severity, SourceLocation, SourceName, SourcePosition,
    validate_diagnostic_registry,
};
pub use digest::{DigestDomain, DigestParseError, Sha256Digest};
pub use entity::{
    Applicability, ApplicabilityKind, Capability, ComparisonOperator, Description,
    EntityBuildError, ExtensionKey, ExtensionNamespace, ExtensionValue, Extensions, Fragment,
    FragmentUse, Journey, MembershipOperator, Requirement, RequirementStatement, Revision, Surface,
    Target, Title, Transition, TransitionTrigger,
};
pub use id::{
    AdapterId, ArtifactId, BindingId, CapabilityId, DimensionId, EvidenceSpecId, FragmentId,
    FrameworkId, FullRequirementId, IdParseError, JourneyId, LocalRequirementId, PlatformId,
    PolicyId, ProfileId, ProviderId, RunnerId, SurfaceId, SymbolicValueId, TargetId, UnitId,
    WaiverId,
};
pub use path::{RepoPath, RepoPathError};
pub use reference::{
    CatalogRef, CiRunRef, DesignRef, ExternalRefError, IssueRef, OwnerRef, ReleaseRef,
};
pub use schema::{SchemaKind, SchemaParseError, SchemaUri, SchemaVersion, ToolVersion};
pub use time::{CalendarDate, DurationMillis, TimeParseError, UtcInstant};
pub use vocabulary::{
    Facet, LifecycleStatus, RequirementLevel, RequirementScope, RiskClass, VocabularyParseError,
};
