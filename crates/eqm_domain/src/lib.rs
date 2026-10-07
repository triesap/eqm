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

mod cohort;
mod diagnostic;
mod digest;
mod entity;
mod evidence;
mod evidence_result;
mod expected_selection;
mod graph;
mod id;
mod inventory;
mod path;
mod policy;
mod provider;
mod record;
mod reference;
mod runner;
mod scenario;
mod schema;
mod scope_bindings;
mod selection;
mod subject_manifest;
mod time;
mod vocabulary;
mod waiver;

pub use cohort::{Cohort, CohortAssignment, CohortError};
pub use diagnostic::{
    Diagnostic, DiagnosticBuildError, DiagnosticCode, DiagnosticDescriptor,
    DiagnosticRegistryError, Severity, SourceLocation, SourceName, SourcePosition,
    validate_diagnostic_registry,
};
pub use digest::{
    AdapterDigest, AttemptDigest, AuthorityDigest, BindingDigest, CampaignDigest,
    ComparisonContextDigest, ContractDigest, DigestDomain, DigestParseError,
    EvaluationContextDigest, EvidenceSpecificationDigest, ExpectedSelectionManifestDigest,
    ParticipationDigest, PlanDigest, PolicySelectionDigest, ProviderConfigurationDigest,
    RawObjectDigest, RequirementScenarioBindingsDigest, RunnerDigest, ScenarioDigest,
    SemanticEvidenceDigest, SemanticGraphV2Digest, Sha256Digest, SubjectManifestDigest,
    TargetConfigurationDigest, TrustConfigurationDigest,
};
pub use entity::{
    Applicability, ApplicabilityKind, ApplicabilityView, Artifact, ArtifactSelector, Artifacts,
    Binding, Capability, ComparisonOperator, Description, EntityBuildError, Exposure, ExtensionKey,
    ExtensionNamespace, ExtensionValue, Extensions, Fragment, FragmentUse, Journey,
    MembershipOperator, Requirement, RequirementStatement, Revision, RouteSelector, SelectorText,
    SourceComponent, Surface, Target, Title, Transition, TransitionTrigger, is_nfc_text,
};
pub use evidence::{
    EvidenceSelector, EvidenceSpecBuildError, EvidenceSpecification, PositiveCount,
};
pub use evidence_result::{
    AttemptAggregate, EvidenceAttachment, EvidenceAttempt, EvidenceCounts, EvidenceExecution,
    EvidenceIdentityRelation, EvidenceLineage, EvidenceLineageParts, EvidencePayload,
    EvidenceResult, EvidenceResultBuildError, EvidenceScopeSubject, EvidenceSubject,
    ExecutionPayload, ExpectedLineageAuthority, MAX_RAW_EVIDENCE_BYTES, ProducerRef,
    ProfileSelection, RawEvidenceObject, RepositoryIdentity, SourceCommit,
};
pub use expected_selection::{
    ExpectedSelectionError, ExpectedSelectionManifest, ExpectedSelectionSource,
    FrozenSelectionAlternative, SelectionCountUnit,
};
pub use graph::{
    AdapterLockIdentity, ImportLockIdentity, SourceComponentError, WorkspaceGraph,
    WorkspaceGraphBuildError, WorkspaceGraphInput, validate_source_components,
};
pub use id::{
    AdapterId, ArtifactId, AttemptId, BindingId, CampaignId, CapabilityId, DimensionId,
    EvidenceSpecId, FragmentId, FrameworkId, FullRequirementId, IdParseError, JourneyId,
    LocalRequirementId, LogicalCaseId, OccurrenceId, ParticipationRuleId, PlanId, PlatformId,
    PolicyId, ProfileId, ProviderId, RecordId, RoleId, RunId, RunnerId, ScenarioId,
    SourceComponentId, SurfaceId, SymbolicValueId, TargetId, UnitId, WaiverId,
};
pub use inventory::{
    AppVersion, BuildNumber, FactValue, Inventory, InventoryBuildError, InventoryEntry,
    ReconcileStatus, ReleaseRecord, RuntimeFact, RuntimeFactsSnapshot,
};
pub use path::{RepoPath, RepoPathError};
pub use policy::{
    ParticipationError, ParticipationRule, Policy, PolicyBuildError, PolicyRule, PolicySelector,
    PositiveDays, Profile, ProfileDimension, WaiverPolicy, validate_participation_rules,
};
pub use provider::{Provider, ProviderBuildError};
pub use record::{
    RecordDescriptor, RecordError, RecordKind, RecordRef, ValidatedRecords, validate_records,
};
pub use reference::{
    CatalogRef, CiRunRef, DesignRef, ExternalRefError, IssueRef, OwnerRef, ReleaseRef,
};
pub use runner::{
    AdapterDefinition, AdapterLimits, ArgumentTemplate, DiscoveryMode, EnvironmentBinding,
    EnvironmentName, EnvironmentSource, RunnerBuildError, RunnerDefinition, RunnerLimits,
    RunnerProgram, SecretBinding, SecretProviderRef, WorkingDirectoryTemplate,
};
pub use scenario::{
    Scenario, ScenarioDependency, ScenarioError, ScenarioParticipant, ScenarioSubject,
};
pub use schema::{
    SchemaGroup, SchemaKind, SchemaParseError, SchemaUri, SchemaVersion, ToolVersion,
};
pub use scope_bindings::{RequirementScenarioBindings, ScenarioRef, ScopeBindingsError};
pub use selection::{
    ComparisonContextInputV2, PolicySelectionInputError, PolicySelectionInputV2,
    SelectedDimensionStateV2, SelectedProfileInputV2,
};
pub use subject_manifest::{SubjectAssignment, SubjectManifest, SubjectManifestError};
pub use time::{
    CalendarDate, DurationMillis, TimeParseError, UtcInstant, UtcInstantV2, V2_UTC_PATTERN,
};
pub use vocabulary::{
    ArtifactRole, AttemptOutcome, EvidenceKind, Facet, HttpMethod, IntendedExposureState,
    InventoryCompleteness, LifecycleStatus, ReleaseChannel, RequirementLevel, RequirementScope,
    RiskClass, RunnerBackend, RunnerGuarantee, TrustLevel, VocabularyParseError,
};
pub use waiver::{
    Waiver, WaiverApplication, WaiverBuildError, WaiverProfileScope, WaiverReason, WaiverScope,
};
