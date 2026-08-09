//! Exact-current schema and tool version identities.

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

const SCHEMA_BASE: &str = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1";

/// The public repository directory that owns a schema document.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SchemaGroup {
    /// Authored workspace and authority manifests.
    Manifest,
    /// Runtime, evidence, adapter, and report protocol documents.
    Protocol,
}

impl SchemaGroup {
    /// Returns the stable repository path component.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Manifest => "manifest",
            Self::Protocol => "protocol",
        }
    }
}

/// The coordinated EQM v1 schema version.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SchemaVersion {
    /// Version 1.
    V1,
}

impl SchemaVersion {
    /// Returns the only accepted version.
    pub const CURRENT: Self = Self::V1;

    /// Returns the URI path component.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        "v1"
    }
}

impl Display for SchemaVersion {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SchemaVersion {
    type Err = SchemaParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == Self::CURRENT.as_str() {
            Ok(Self::CURRENT)
        } else {
            Err(SchemaParseError::UnsupportedSchemaVersion)
        }
    }
}

/// A closed EQM v1 schema document kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SchemaKind {
    /// Workspace configuration.
    Workspace,
    /// Capability authority.
    Capability,
    /// Journey authority.
    Journey,
    /// Surface authority.
    Surface,
    /// Fragment authority.
    Fragment,
    /// Target binding authority.
    Binding,
    /// Policy authority.
    Policy,
    /// Profile authority.
    Profile,
    /// Runner authority.
    Runner,
    /// Waiver authority.
    Waiver,
    /// Import and adapter lock.
    Lock,
    /// Finalized semantic graph projection.
    SemanticGraph,
    /// Common result envelope.
    Result,
    /// Public diagnostic.
    Diagnostic,
    /// Normalized test result.
    TestResult,
    /// Immutable evidence result.
    EvidenceResult,
    /// Adapter inventory.
    Inventory,
    /// Runtime exposure facts.
    RuntimeFacts,
    /// Exact release record.
    ReleaseRecord,
    /// EQM attestation predicate.
    Attestation,
    /// Adapter request.
    AdapterRequest,
    /// Adapter response.
    AdapterResponse,
}

impl SchemaKind {
    /// Every accepted kind in stable lexical-identity order.
    pub const ALL: [Self; 22] = [
        Self::AdapterRequest,
        Self::AdapterResponse,
        Self::Attestation,
        Self::Binding,
        Self::Capability,
        Self::Diagnostic,
        Self::EvidenceResult,
        Self::Fragment,
        Self::Inventory,
        Self::Journey,
        Self::Lock,
        Self::Policy,
        Self::Profile,
        Self::ReleaseRecord,
        Self::Result,
        Self::Runner,
        Self::RuntimeFacts,
        Self::SemanticGraph,
        Self::Surface,
        Self::TestResult,
        Self::Waiver,
        Self::Workspace,
    ];

    /// Returns the exact final URI path component.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Capability => "capability",
            Self::Journey => "journey",
            Self::Surface => "surface",
            Self::Fragment => "fragment",
            Self::Binding => "binding",
            Self::Policy => "policy",
            Self::Profile => "profile",
            Self::Runner => "runner",
            Self::Waiver => "waiver",
            Self::Lock => "lock",
            Self::SemanticGraph => "semantic-graph",
            Self::Result => "result",
            Self::Diagnostic => "diagnostic",
            Self::TestResult => "test-result",
            Self::EvidenceResult => "evidence-result",
            Self::Inventory => "inventory",
            Self::RuntimeFacts => "runtime-facts",
            Self::ReleaseRecord => "release-record",
            Self::Attestation => "attestation",
            Self::AdapterRequest => "adapter-request",
            Self::AdapterResponse => "adapter-response",
        }
    }

    /// Returns the repository directory that owns this schema kind.
    #[must_use]
    pub const fn group(self) -> SchemaGroup {
        match self {
            Self::Workspace
            | Self::Capability
            | Self::Journey
            | Self::Surface
            | Self::Fragment
            | Self::Binding
            | Self::Policy
            | Self::Profile
            | Self::Runner
            | Self::Waiver
            | Self::Lock => SchemaGroup::Manifest,
            Self::SemanticGraph
            | Self::Result
            | Self::Diagnostic
            | Self::TestResult
            | Self::EvidenceResult
            | Self::Inventory
            | Self::RuntimeFacts
            | Self::ReleaseRecord
            | Self::Attestation
            | Self::AdapterRequest
            | Self::AdapterResponse => SchemaGroup::Protocol,
        }
    }
}

impl Display for SchemaKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SchemaKind {
    type Err = SchemaParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|kind| kind.as_str() == value)
            .ok_or(SchemaParseError::UnsupportedSchemaKind)
    }
}

/// An exact current EQM schema URI.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SchemaUri(SchemaKind);

impl SchemaUri {
    /// Creates the current URI for a closed schema kind.
    #[must_use]
    pub const fn new(kind: SchemaKind) -> Self {
        Self(kind)
    }

    /// Returns the document kind.
    #[must_use]
    pub const fn kind(self) -> SchemaKind {
        self.0
    }

    /// Returns the coordinated version.
    #[must_use]
    pub const fn version(self) -> SchemaVersion {
        SchemaVersion::CURRENT
    }
}

impl Display for SchemaUri {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{SCHEMA_BASE}/{group}/{kind}.schema.json",
            group = self.0.group().as_str(),
            kind = self.0.as_str(),
        )
    }
}

impl FromStr for SchemaUri {
    type Err = SchemaParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        SchemaKind::ALL
            .into_iter()
            .map(Self::new)
            .find(|uri| uri.to_string() == value)
            .ok_or(SchemaParseError::InvalidSchemaUri)
    }
}

/// The exact EQM tool version accepted by this build.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ToolVersion;

impl ToolVersion {
    /// The executing build's version.
    pub const CURRENT: Self = Self;

    /// Returns the exact package version compiled into this crate.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

impl Display for ToolVersion {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ToolVersion {
    type Err = SchemaParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == Self::CURRENT.as_str() {
            Ok(Self::CURRENT)
        } else {
            Err(SchemaParseError::UnsupportedToolVersion)
        }
    }
}

/// Failure to parse an exact-current schema or tool identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchemaParseError {
    /// The URI prefix, structure, or suffix is invalid.
    InvalidSchemaUri,
    /// The schema document kind is not in the v1 coordinated set.
    UnsupportedSchemaKind,
    /// The schema version is not exactly v1.
    UnsupportedSchemaVersion,
    /// The tool version does not equal the executing build.
    UnsupportedToolVersion,
}

impl Display for SchemaParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidSchemaUri => "invalid EQM schema URI",
            Self::UnsupportedSchemaKind => "unsupported EQM schema kind",
            Self::UnsupportedSchemaVersion => "unsupported EQM schema version",
            Self::UnsupportedToolVersion => "unsupported EQM tool version",
        })
    }
}

impl Error for SchemaParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_current_schema_round_trips() {
        for kind in SchemaKind::ALL {
            let uri = SchemaUri::new(kind);
            assert_eq!(uri.to_string().parse(), Ok(uri));
            assert_eq!(uri.kind(), kind);
            assert_eq!(uri.version(), SchemaVersion::V1);
        }
    }

    #[test]
    fn malformed_foreign_old_and_future_schemas_fail_closed() {
        for value in [
            "",
            "http://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/workspace.schema.json",
            "https://example.com/triesap/eqm/master/schemas/v1/manifest/workspace.schema.json",
            "https://raw.githubusercontent.com/triesap/eqm/main/schemas/v1/manifest/workspace.schema.json",
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v0/manifest/workspace.schema.json",
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/workspace.schema.json",
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/protocol/workspace.schema.json",
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/workspace.schema.json?x=1",
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/workspace.schema.json#x",
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/unknown.schema.json",
        ] {
            assert!(value.parse::<SchemaUri>().is_err(), "accepted {value}");
        }
    }

    #[test]
    fn schema_version_is_exact_current() {
        assert_eq!("v1".parse(), Ok(SchemaVersion::CURRENT));
        for value in ["", "1", "V1", "v0", "v2", "v1.0"] {
            assert_eq!(
                value.parse::<SchemaVersion>(),
                Err(SchemaParseError::UnsupportedSchemaVersion)
            );
        }
    }

    #[test]
    fn tool_version_is_exact_current() {
        let current = ToolVersion::CURRENT.as_str();
        assert_eq!(current.parse(), Ok(ToolVersion::CURRENT));
        for value in ["", "0.0.0", "999.0.0", "v0.1.0", "0.1"] {
            if value != current {
                assert_eq!(
                    value.parse::<ToolVersion>(),
                    Err(SchemaParseError::UnsupportedToolVersion)
                );
            }
        }
    }
}
