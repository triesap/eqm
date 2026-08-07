//! Pure validated domain types for EquivalenceMatrix.
//!
//! This crate owns semantic values and local construction invariants. It does
//! not parse manifests, resolve references, access the filesystem, execute
//! runners, evaluate policy, or serialize the public protocol. Those layers
//! must cross this validation boundary explicitly.
//!
//! [`WorkspaceGraphInput`] collects already validated authority and
//! [`WorkspaceGraph`] freezes it into deterministic indexes. Graph construction
//! rejects duplicate semantic keys but deliberately leaves dangling-reference,
//! lifecycle, fragment-expansion, and policy checks to the engine.
//!
//! # I/O-free construction
//!
//! ```
//! use eqm_domain::{
//!     Capability, CapabilityId, Extensions, LifecycleStatus, OwnerRef, Title,
//!     WorkspaceGraph, WorkspaceGraphInput,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let capability = Capability::new(
//!     CapabilityId::new("account.create")?,
//!     Title::new("Account creation")?,
//!     LifecycleStatus::Active,
//!     vec!["owner://team/accounts".parse::<OwnerRef>()?],
//!     None,
//!     Extensions::default(),
//! )?;
//! let graph = WorkspaceGraph::new(WorkspaceGraphInput {
//!     capabilities: vec![capability],
//!     ..WorkspaceGraphInput::default()
//! })?;
//!
//! assert!(graph.capabilities().contains_key(&CapabilityId::new("account.create")?));
//! assert!(graph.targets().is_empty());
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]

mod diagnostic;
mod digest;
mod entity;
mod evidence;
mod evidence_result;
mod graph;
mod id;
mod inventory;
mod path;
mod policy;
mod reference;
mod runner;
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
    Applicability, ApplicabilityKind, ApplicabilityView, Artifact, ArtifactSelector, Artifacts,
    Binding, Capability, ComparisonOperator, Description, EntityBuildError, Exposure, ExtensionKey,
    ExtensionNamespace, ExtensionValue, Extensions, Fragment, FragmentUse, Journey,
    MembershipOperator, Requirement, RequirementStatement, Revision, RouteSelector, SelectorText,
    Surface, Target, Title, Transition, TransitionTrigger,
};
pub use evidence::{
    EvidenceSelector, EvidenceSpecBuildError, EvidenceSpecification, PositiveCount,
};
pub use evidence_result::{
    AttemptAggregate, EvidenceAttachment, EvidenceAttempt, EvidenceCounts, EvidencePayload,
    EvidenceResult, EvidenceResultBuildError, EvidenceScopeSubject, EvidenceSubject,
    ExecutionPayload, ProducerRef, ProfileSelection, RepositoryIdentity, SourceCommit,
};
pub use graph::{
    AdapterLockIdentity, ImportLockIdentity, WorkspaceGraph, WorkspaceGraphBuildError,
    WorkspaceGraphInput,
};
pub use id::{
    AdapterId, ArtifactId, BindingId, CapabilityId, DimensionId, EvidenceSpecId, FragmentId,
    FrameworkId, FullRequirementId, IdParseError, JourneyId, LocalRequirementId, PlatformId,
    PolicyId, ProfileId, ProviderId, RunnerId, SurfaceId, SymbolicValueId, TargetId, UnitId,
    WaiverId,
};
pub use inventory::{
    AppVersion, BuildNumber, FactValue, Inventory, InventoryBuildError, InventoryEntry,
    ReconcileStatus, ReleaseRecord, RuntimeFact, RuntimeFactsSnapshot,
};
pub use path::{RepoPath, RepoPathError};
pub use policy::{
    Policy, PolicyBuildError, PolicyRule, PolicySelector, PositiveDays, Profile, ProfileDimension,
    WaiverPolicy,
};
pub use reference::{
    CatalogRef, CiRunRef, DesignRef, ExternalRefError, IssueRef, OwnerRef, ReleaseRef,
};
pub use runner::{
    AdapterDefinition, AdapterLimits, ArgumentTemplate, DiscoveryMode, EnvironmentBinding,
    EnvironmentName, EnvironmentSource, RunnerBuildError, RunnerDefinition, RunnerLimits,
    RunnerProgram, SecretBinding, SecretProviderRef, WorkingDirectoryTemplate,
};
pub use schema::{SchemaKind, SchemaParseError, SchemaUri, SchemaVersion, ToolVersion};
pub use time::{CalendarDate, DurationMillis, TimeParseError, UtcInstant};
pub use vocabulary::{
    ArtifactRole, AttemptOutcome, EvidenceKind, Facet, HttpMethod, IntendedExposureState,
    InventoryCompleteness, LifecycleStatus, ReleaseChannel, RequirementLevel, RequirementScope,
    RiskClass, RunnerBackend, RunnerGuarantee, TrustLevel, VocabularyParseError,
};
pub use waiver::{
    Waiver, WaiverApplication, WaiverBuildError, WaiverProfileScope, WaiverReason, WaiverScope,
};
