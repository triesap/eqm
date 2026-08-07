//! Pure validated domain types for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod diagnostic;
mod digest;
mod entity;
mod evidence;
mod evidence_result;
mod id;
mod path;
mod policy;
mod reference;
mod schema;
mod time;
mod vocabulary;
mod waiver;

pub use diagnostic::{
    Diagnostic, DiagnosticBuildError, DiagnosticCode, DiagnosticDescriptor,
    DiagnosticRegistryError, Severity, SourceLocation, SourceName, SourcePosition,
    validate_diagnostic_registry,
};
pub use digest::{DigestDomain, DigestParseError, Sha256Digest};
pub use entity::{
    Applicability, ApplicabilityKind, Artifact, ArtifactSelector, Artifacts, Capability,
    ComparisonOperator, Description, EntityBuildError, Exposure, ExtensionKey, ExtensionNamespace,
    ExtensionValue, Extensions, Fragment, FragmentUse, Journey, MembershipOperator, Requirement,
    RequirementStatement, Revision, RouteSelector, SelectorText, Surface, Target, Title,
    Transition, TransitionTrigger,
};
pub use evidence::{
    EvidenceSelector, EvidenceSpecBuildError, EvidenceSpecification, PositiveCount,
};
pub use evidence_result::{
    AttemptAggregate, EvidenceAttachment, EvidenceAttempt, EvidenceCounts, EvidencePayload,
    EvidenceResult, EvidenceResultBuildError, EvidenceScopeSubject, EvidenceSubject,
    ExecutionPayload, ProducerRef, ProfileSelection, RepositoryIdentity, SourceCommit,
};
pub use id::{
    AdapterId, ArtifactId, BindingId, CapabilityId, DimensionId, EvidenceSpecId, FragmentId,
    FrameworkId, FullRequirementId, IdParseError, JourneyId, LocalRequirementId, PlatformId,
    PolicyId, ProfileId, ProviderId, RunnerId, SurfaceId, SymbolicValueId, TargetId, UnitId,
    WaiverId,
};
pub use path::{RepoPath, RepoPathError};
pub use policy::{
    Policy, PolicyBuildError, PolicyRule, PolicySelector, PositiveDays, Profile, ProfileDimension,
    WaiverPolicy,
};
pub use reference::{
    CatalogRef, CiRunRef, DesignRef, ExternalRefError, IssueRef, OwnerRef, ReleaseRef,
};
pub use schema::{SchemaKind, SchemaParseError, SchemaUri, SchemaVersion, ToolVersion};
pub use time::{CalendarDate, DurationMillis, TimeParseError, UtcInstant};
pub use vocabulary::{
    ArtifactRole, AttemptOutcome, EvidenceKind, Facet, HttpMethod, IntendedExposureState,
    LifecycleStatus, ReleaseChannel, RequirementLevel, RequirementScope, RiskClass, TrustLevel,
    VocabularyParseError,
};
pub use waiver::{
    Waiver, WaiverApplication, WaiverBuildError, WaiverProfileScope, WaiverReason, WaiverScope,
};
