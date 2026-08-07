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
    CanonicalFragment, CanonicalGraph, CanonicalizationError, binding as project_binding,
    canonicalize_fragment, canonicalize_graph, capability as project_capability,
    fragment as project_fragment, journey as project_journey, policy as project_policy,
    profile as project_profile, runner as project_runner, surface as project_surface,
    target as project_target, waiver as project_waiver,
};
pub use config::{ConfigError, WorkspaceConfig, select_workspace_config};
pub use conversion::{
    ContractEntity, ConversionError, convert_binding, convert_contract, convert_policy,
    convert_profile, convert_runner, convert_waiver,
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
