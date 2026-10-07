//! Authored manifest loading and canonicalization for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod canonical;
mod config;
mod conversion;
mod discovery;
/// Strict source data-transfer objects for authored manifests.
pub mod dto;
mod formatter;
mod loader;
mod lockfile;
mod parse;
mod validation;

pub use canonical::{
    CanonicalBindingV2, CanonicalComparisonContextV2, CanonicalContractV2, CanonicalEvidenceSpecV2,
    CanonicalFragment, CanonicalGraph, CanonicalGraphV2, CanonicalParticipationV2,
    CanonicalPolicySelectionV2, CanonicalProviderConfigurationV2, CanonicalScenarioV2,
    CanonicalScopeBindingsV2, CanonicalTargetConfigurationV2, CanonicalizationError,
    binding as project_binding, binding_v2 as project_binding_v2, canonicalize_binding_v2,
    canonicalize_comparison_context_v2, canonicalize_contract_v2, canonicalize_evidence_spec_v2,
    canonicalize_fragment, canonicalize_graph, canonicalize_graph_v2,
    canonicalize_participation_v2, canonicalize_policy_selection_v2,
    canonicalize_provider_configuration_v2, canonicalize_scenario_v2,
    canonicalize_scope_bindings_v2, canonicalize_target_configuration_v2,
    capability as project_capability, comparison_context_v2 as project_comparison_context_v2,
    contract_v2 as project_contract_v2, evidence_spec_v2 as project_evidence_spec_v2,
    fragment as project_fragment, graph_v2 as project_graph_v2, journey as project_journey,
    participation_v2 as project_participation_v2, policy as project_policy,
    policy_selection_v2 as project_policy_selection_v2, profile as project_profile,
    runner as project_runner, scenario_v2 as project_scenario_v2,
    scope_bindings_v2 as project_scope_bindings_v2, semantic_graph_v2_schema_uri,
    surface as project_surface, target as project_target,
    target_configuration_v2 as project_target_configuration_v2, waiver as project_waiver,
};
pub use config::{ConfigError, WorkspaceConfig, select_workspace_config};
pub use conversion::{
    ContractEntity, ConversionError, convert_binding, convert_binding_with_roots, convert_cohort,
    convert_contract, convert_participation, convert_policy, convert_profile, convert_provider,
    convert_runner, convert_scenario, convert_waiver,
};
pub use conversion::{WaiverTemporalStatus, classify_waiver};
pub use discovery::{DiscoveredSource, DiscoveryError, SourceClass, discover_sources};
pub use formatter::{
    FormatError, FormatMode, FormatOutcome, format_manifest, format_manifest_file,
};
pub use loader::{LoadError, LoadedWorkspace, load_workspace};
pub use lockfile::{AdapterLock, ImportLock, LockError, WorkspaceLock, load_lockfile};
pub use parse::{ParseError, ParsedToml, TomlSyntaxError, parse_toml};
pub use validation::{
    DocumentDto, ValidatedDocument, ValidationError, ValidationErrorKind, decode_sources,
};
