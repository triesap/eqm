//! Shared entity values and the capability model.

use crate::{CapabilityId, JourneyId, LifecycleStatus, OwnerRef, RiskClass, SurfaceId};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroU64;
use unicode_normalization::UnicodeNormalization;

const MAX_EXTENSION_DEPTH: usize = 16;
const MAX_EXTENSION_NODES: usize = 1_024;
const MAX_EXTENSION_STRING_BYTES: usize = 16 * 1_024;
const MAX_EXTENSION_BYTES: usize = 256 * 1_024;

#[derive(Clone, Copy)]
struct ExtensionMeasure {
    nodes: usize,
    bytes: usize,
}

fn normalized_text(value: &str, maximum: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum
        && value.nfc().eq(value.chars())
        && !value
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\t'))
}

macro_rules! text_value {
    ($(#[$meta:meta])* $name:ident, $maximum:literal) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(Box<str>);

        impl $name {
            /// Validates normalized bounded text.
            pub fn new(value: impl Into<Box<str>>) -> Result<Self, EntityBuildError> {
                let value = value.into();
                if !normalized_text(&value, $maximum) {
                    return Err(EntityBuildError::InvalidText);
                }
                Ok(Self(value))
            }

            /// Returns the exact normalized text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

text_value!(
    /// A required entity title of at most 160 UTF-8 bytes.
    Title,
    160
);
text_value!(
    /// A normalized transition trigger of at most 256 UTF-8 bytes.
    TransitionTrigger,
    256
);

/// A positive authored authority revision.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Revision(NonZeroU64);

impl Revision {
    /// Creates a positive revision.
    pub fn new(value: u64) -> Result<Self, EntityBuildError> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or(EntityBuildError::RevisionRequired)
    }

    /// Returns the positive integer value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for Revision {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}
text_value!(
    /// Optional normative description text of at most 4,096 UTF-8 bytes.
    Description,
    4_096
);

/// A validated reverse-domain extension namespace.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExtensionNamespace(Box<str>);

impl ExtensionNamespace {
    /// Validates a namespace with at least three lowercase segments.
    pub fn new(value: impl Into<Box<str>>) -> Result<Self, EntityBuildError> {
        let value = value.into();
        let segments: Vec<_> = value.split('.').collect();
        if segments.len() < 3 || !segments.iter().all(|segment| extension_segment(segment)) {
            return Err(EntityBuildError::InvalidExtensionNamespace);
        }
        Ok(Self(value))
    }

    /// Returns the exact namespace.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether the namespace is the sole digest-excluded display namespace.
    #[must_use]
    pub fn is_display_only(&self) -> bool {
        self.as_str() == "dev.equivalencematrix.display"
    }
}

/// A validated extension-object key.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExtensionKey(Box<str>);

impl ExtensionKey {
    /// Validates one lowercase key segment.
    pub fn new(value: impl Into<Box<str>>) -> Result<Self, EntityBuildError> {
        let value = value.into();
        if !extension_segment(&value) {
            return Err(EntityBuildError::InvalidExtensionKey);
        }
        Ok(Self(value))
    }

    /// Returns the exact key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn extension_segment(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 63
        && matches!(value.as_bytes().first(), Some(first) if first.is_ascii_lowercase())
        && value
            .bytes()
            .skip(1)
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

/// A JSON-compatible, float-free extension value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExtensionValue {
    /// Boolean value.
    Boolean(bool),
    /// Signed integer value.
    Integer(i64),
    /// Normalized bounded string value.
    String(Box<str>),
    /// Ordered array value.
    Array(Vec<Self>),
    /// Deterministically keyed object value.
    Object(BTreeMap<ExtensionKey, Self>),
}

impl ExtensionValue {
    /// Creates a normalized extension string.
    pub fn string(value: impl Into<Box<str>>) -> Result<Self, EntityBuildError> {
        let value = value.into();
        if !normalized_text(&value, MAX_EXTENSION_STRING_BYTES) {
            return Err(EntityBuildError::InvalidExtensionValue);
        }
        Ok(Self::String(value))
    }

    fn measure(&self, depth: usize) -> Result<ExtensionMeasure, EntityBuildError> {
        if depth > MAX_EXTENSION_DEPTH {
            return Err(EntityBuildError::ExtensionDepthExceeded);
        }
        match self {
            Self::Boolean(_) => Ok(ExtensionMeasure { nodes: 1, bytes: 1 }),
            Self::Integer(_) => Ok(ExtensionMeasure { nodes: 1, bytes: 8 }),
            Self::String(value) => {
                if normalized_text(value, MAX_EXTENSION_STRING_BYTES) {
                    Ok(ExtensionMeasure {
                        nodes: 1,
                        bytes: value.len(),
                    })
                } else {
                    Err(EntityBuildError::InvalidExtensionValue)
                }
            }
            Self::Array(values) => values
                .iter()
                .try_fold(ExtensionMeasure { nodes: 1, bytes: 2 }, |measure, value| {
                    measure.add(value.measure(depth + 1)?)
                }),
            Self::Object(values) => values.iter().try_fold(
                ExtensionMeasure { nodes: 1, bytes: 2 },
                |measure, (key, value)| {
                    measure
                        .add(ExtensionMeasure {
                            nodes: 0,
                            bytes: key.as_str().len(),
                        })?
                        .add(value.measure(depth + 1)?)
                },
            ),
        }
    }
}

impl ExtensionMeasure {
    fn add(self, other: Self) -> Result<Self, EntityBuildError> {
        Ok(Self {
            nodes: self
                .nodes
                .checked_add(other.nodes)
                .ok_or(EntityBuildError::ExtensionNodesExceeded)?,
            bytes: self
                .bytes
                .checked_add(other.bytes)
                .ok_or(EntityBuildError::ExtensionBytesExceeded)?,
        })
    }
}

/// Validated namespaced extension data.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Extensions(BTreeMap<ExtensionNamespace, ExtensionValue>);

impl Extensions {
    /// Validates extension depth and total node count.
    pub fn new(
        values: BTreeMap<ExtensionNamespace, ExtensionValue>,
    ) -> Result<Self, EntityBuildError> {
        let measure = values.iter().try_fold(
            ExtensionMeasure { nodes: 0, bytes: 0 },
            |measure, (namespace, value)| {
                measure
                    .add(ExtensionMeasure {
                        nodes: 0,
                        bytes: namespace.as_str().len(),
                    })?
                    .add(value.measure(1)?)
            },
        )?;
        if measure.nodes > MAX_EXTENSION_NODES {
            return Err(EntityBuildError::ExtensionNodesExceeded);
        }
        if measure.bytes > MAX_EXTENSION_BYTES {
            return Err(EntityBuildError::ExtensionBytesExceeded);
        }
        Ok(Self(values))
    }

    /// Returns deterministic namespace/value pairs.
    #[must_use]
    pub const fn values(&self) -> &BTreeMap<ExtensionNamespace, ExtensionValue> {
        &self.0
    }

    /// Returns whether no extensions are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// A validated capability authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capability {
    id: CapabilityId,
    title: Title,
    status: LifecycleStatus,
    owners: BTreeSet<OwnerRef>,
    description: Option<Description>,
    extensions: Extensions,
}

impl Capability {
    /// Creates a capability with nonempty, duplicate-free owners.
    pub fn new(
        id: CapabilityId,
        title: Title,
        status: LifecycleStatus,
        owners: Vec<OwnerRef>,
        description: Option<Description>,
        extensions: Extensions,
    ) -> Result<Self, EntityBuildError> {
        let owners = owner_set(owners)?;
        Ok(Self {
            id,
            title,
            status,
            owners,
            description,
            extensions,
        })
    }

    /// Returns the capability ID.
    #[must_use]
    pub const fn id(&self) -> &CapabilityId {
        &self.id
    }
    /// Returns the title.
    #[must_use]
    pub const fn title(&self) -> &Title {
        &self.title
    }
    /// Returns lifecycle status.
    #[must_use]
    pub const fn status(&self) -> LifecycleStatus {
        self.status
    }
    /// Returns owners in deterministic order.
    #[must_use]
    pub const fn owners(&self) -> &BTreeSet<OwnerRef> {
        &self.owners
    }
    /// Returns the optional description.
    #[must_use]
    pub const fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

fn owner_set(owners: Vec<OwnerRef>) -> Result<BTreeSet<OwnerRef>, EntityBuildError> {
    if owners.is_empty() {
        return Err(EntityBuildError::OwnersRequired);
    }
    let owner_count = owners.len();
    let owners: BTreeSet<_> = owners.into_iter().collect();
    if owners.len() != owner_count {
        return Err(EntityBuildError::DuplicateOwner);
    }
    Ok(owners)
}

/// A directed journey transition between declared surfaces.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Transition {
    from: SurfaceId,
    to: SurfaceId,
    trigger: TransitionTrigger,
}

impl Transition {
    /// Creates a transition with validated endpoint and trigger types.
    #[must_use]
    pub const fn new(from: SurfaceId, to: SurfaceId, trigger: TransitionTrigger) -> Self {
        Self { from, to, trigger }
    }

    /// Returns the origin surface.
    #[must_use]
    pub const fn from(&self) -> &SurfaceId {
        &self.from
    }

    /// Returns the destination surface.
    #[must_use]
    pub const fn to(&self) -> &SurfaceId {
        &self.to
    }

    /// Returns the normalized transition trigger.
    #[must_use]
    pub const fn trigger(&self) -> &TransitionTrigger {
        &self.trigger
    }
}

/// A versioned journey authority with normative surface order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Journey {
    id: JourneyId,
    revision: Revision,
    title: Title,
    capability: CapabilityId,
    status: LifecycleStatus,
    risk_class: RiskClass,
    owners: BTreeSet<OwnerRef>,
    surfaces: Vec<SurfaceId>,
    transitions: BTreeSet<Transition>,
    description: Option<Description>,
    extensions: Extensions,
}

impl Journey {
    /// Creates a journey and validates local collection invariants.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: JourneyId,
        revision: Revision,
        title: Title,
        capability: CapabilityId,
        status: LifecycleStatus,
        risk_class: RiskClass,
        owners: Vec<OwnerRef>,
        surfaces: Vec<SurfaceId>,
        transitions: Vec<Transition>,
        description: Option<Description>,
        extensions: Extensions,
    ) -> Result<Self, EntityBuildError> {
        let owners = owner_set(owners)?;
        if surfaces.is_empty() {
            return Err(EntityBuildError::SurfacesRequired);
        }
        let unique_surfaces: BTreeSet<_> = surfaces.iter().collect();
        if unique_surfaces.len() != surfaces.len() {
            return Err(EntityBuildError::DuplicateSurface);
        }
        let transition_count = transitions.len();
        let transitions: BTreeSet<_> = transitions.into_iter().collect();
        if transitions.len() != transition_count {
            return Err(EntityBuildError::DuplicateTransition);
        }
        Ok(Self {
            id,
            revision,
            title,
            capability,
            status,
            risk_class,
            owners,
            surfaces,
            transitions,
            description,
            extensions,
        })
    }

    /// Returns the journey ID.
    #[must_use]
    pub const fn id(&self) -> &JourneyId {
        &self.id
    }
    /// Returns the authored revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns the title.
    #[must_use]
    pub const fn title(&self) -> &Title {
        &self.title
    }
    /// Returns the parent capability ID.
    #[must_use]
    pub const fn capability(&self) -> &CapabilityId {
        &self.capability
    }
    /// Returns lifecycle status.
    #[must_use]
    pub const fn status(&self) -> LifecycleStatus {
        self.status
    }
    /// Returns the required risk class.
    #[must_use]
    pub const fn risk_class(&self) -> RiskClass {
        self.risk_class
    }
    /// Returns owners in deterministic order.
    #[must_use]
    pub const fn owners(&self) -> &BTreeSet<OwnerRef> {
        &self.owners
    }
    /// Returns surfaces in normative authored order.
    #[must_use]
    pub fn surfaces(&self) -> &[SurfaceId] {
        &self.surfaces
    }
    /// Returns transitions in canonical tuple order.
    #[must_use]
    pub const fn transitions(&self) -> &BTreeSet<Transition> {
        &self.transitions
    }
    /// Returns the optional description.
    #[must_use]
    pub const fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

/// Entity construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntityBuildError {
    /// Required normalized text was invalid.
    InvalidText,
    /// At least one owner is required.
    OwnersRequired,
    /// An owner appeared more than once.
    DuplicateOwner,
    /// A versioned authority used revision zero.
    RevisionRequired,
    /// A journey did not declare any surfaces.
    SurfacesRequired,
    /// A journey declared a surface more than once.
    DuplicateSurface,
    /// A journey declared the same transition tuple more than once.
    DuplicateTransition,
    /// An extension namespace was invalid.
    InvalidExtensionNamespace,
    /// An extension object key was invalid.
    InvalidExtensionKey,
    /// An extension scalar value was invalid.
    InvalidExtensionValue,
    /// Extension nesting exceeded 16 levels.
    ExtensionDepthExceeded,
    /// Extension data exceeded 1,024 nodes.
    ExtensionNodesExceeded,
    /// Extension data exceeded 256 KiB.
    ExtensionBytesExceeded,
}

impl Display for EntityBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidText => "invalid normalized entity text",
            Self::OwnersRequired => "entity requires at least one owner",
            Self::DuplicateOwner => "entity owners contain a duplicate",
            Self::RevisionRequired => "entity revision must be positive",
            Self::SurfacesRequired => "journey requires at least one surface",
            Self::DuplicateSurface => "journey surfaces contain a duplicate",
            Self::DuplicateTransition => "journey transitions contain a duplicate tuple",
            Self::InvalidExtensionNamespace => "invalid extension namespace",
            Self::InvalidExtensionKey => "invalid extension key",
            Self::InvalidExtensionValue => "invalid extension value",
            Self::ExtensionDepthExceeded => "extension depth exceeds 16",
            Self::ExtensionNodesExceeded => "extension node count exceeds 1,024",
            Self::ExtensionBytesExceeded => "extension data exceeds 256 KiB",
        })
    }
}

impl Error for EntityBuildError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn owner(value: &str) -> Result<OwnerRef, crate::ExternalRefError> {
        value.parse()
    }

    #[test]
    fn capability_requires_complete_valid_authority() -> Result<(), Box<dyn Error>> {
        let capability = Capability::new(
            CapabilityId::new("account.create")?,
            Title::new("Account creation")?,
            LifecycleStatus::Active,
            vec![owner("owner://team/accounts")?],
            Some(Description::new("Create an account")?),
            Extensions::default(),
        )?;
        assert_eq!(capability.id().as_str(), "account.create");
        assert_eq!(capability.title().as_str(), "Account creation");
        assert_eq!(capability.status(), LifecycleStatus::Active);
        assert_eq!(capability.owners().len(), 1);
        assert!(capability.extensions().is_empty());
        Ok(())
    }

    #[test]
    fn empty_and_duplicate_owners_fail() -> Result<(), Box<dyn Error>> {
        let id = CapabilityId::new("account.create")?;
        let title = Title::new("Account creation")?;
        assert!(matches!(
            Capability::new(
                id.clone(),
                title.clone(),
                LifecycleStatus::Active,
                Vec::new(),
                None,
                Extensions::default(),
            ),
            Err(EntityBuildError::OwnersRequired)
        ));
        let account = owner("owner://team/accounts")?;
        assert!(matches!(
            Capability::new(
                id,
                title,
                LifecycleStatus::Active,
                vec![account.clone(), account],
                None,
                Extensions::default(),
            ),
            Err(EntityBuildError::DuplicateOwner)
        ));
        Ok(())
    }

    #[test]
    fn text_requires_nfc_and_bounds() {
        assert!(Title::new("Account creation").is_ok());
        assert_eq!(Title::new(""), Err(EntityBuildError::InvalidText));
        assert_eq!(
            Title::new("a".repeat(161)),
            Err(EntityBuildError::InvalidText)
        );
        assert_eq!(
            Title::new("cafe\u{301}"),
            Err(EntityBuildError::InvalidText)
        );
    }

    #[test]
    fn extensions_validate_namespace_depth_and_nodes() -> Result<(), EntityBuildError> {
        let namespace = ExtensionNamespace::new("dev.example.audit")?;
        let mut values = BTreeMap::new();
        values.insert(namespace, ExtensionValue::string("enabled")?);
        assert_eq!(Extensions::new(values)?.values().len(), 1);
        assert!(ExtensionNamespace::new("example.audit").is_err());

        let mut nested = ExtensionValue::Boolean(true);
        for _ in 0..16 {
            nested = ExtensionValue::Array(vec![nested]);
        }
        let mut too_deep = BTreeMap::new();
        too_deep.insert(ExtensionNamespace::new("dev.example.deep")?, nested);
        assert_eq!(
            Extensions::new(too_deep),
            Err(EntityBuildError::ExtensionDepthExceeded)
        );

        let large = ExtensionValue::string("a".repeat(MAX_EXTENSION_STRING_BYTES))?;
        let mut too_large = BTreeMap::new();
        too_large.insert(
            ExtensionNamespace::new("dev.example.large")?,
            ExtensionValue::Array(vec![large; 17]),
        );
        assert_eq!(
            Extensions::new(too_large),
            Err(EntityBuildError::ExtensionBytesExceeded)
        );
        Ok(())
    }

    #[test]
    fn journey_preserves_surface_order_and_sorts_transitions() -> Result<(), Box<dyn Error>> {
        let start = SurfaceId::new("account.create.signup.start")?;
        let done = SurfaceId::new("account.create.signup.done")?;
        let forward = Transition::new(
            start.clone(),
            done.clone(),
            TransitionTrigger::new("submit")?,
        );
        let journey = Journey::new(
            JourneyId::new("account.create.signup")?,
            Revision::new(1)?,
            Title::new("Sign up")?,
            CapabilityId::new("account.create")?,
            LifecycleStatus::Active,
            RiskClass::High,
            vec![owner("owner://team/accounts")?],
            vec![start.clone(), done.clone()],
            vec![forward.clone()],
            Some(Description::new("Create an account interactively")?),
            Extensions::default(),
        )?;
        assert_eq!(journey.revision().get(), 1);
        assert_eq!(journey.surfaces(), [start, done]);
        assert_eq!(journey.transitions().first(), Some(&forward));
        Ok(())
    }

    #[test]
    fn journey_rejects_invalid_local_collections() -> Result<(), Box<dyn Error>> {
        let start = SurfaceId::new("account.create.signup.start")?;
        let id = JourneyId::new("account.create.signup")?;
        let revision = Revision::new(1)?;
        let title = Title::new("Sign up")?;
        let capability = CapabilityId::new("account.create")?;
        let account_owner = owner("owner://team/accounts")?;
        let transition = Transition::new(
            start.clone(),
            start.clone(),
            TransitionTrigger::new("retry")?,
        );
        let build = |surfaces, transitions| {
            Journey::new(
                id.clone(),
                revision,
                title.clone(),
                capability.clone(),
                LifecycleStatus::Active,
                RiskClass::Medium,
                vec![account_owner.clone()],
                surfaces,
                transitions,
                None,
                Extensions::default(),
            )
        };
        assert_eq!(Revision::new(0), Err(EntityBuildError::RevisionRequired));
        assert!(matches!(
            build(Vec::new(), Vec::new()),
            Err(EntityBuildError::SurfacesRequired)
        ));
        assert!(matches!(
            build(vec![start.clone(), start.clone()], Vec::new()),
            Err(EntityBuildError::DuplicateSurface)
        ));
        assert!(matches!(
            build(vec![start], vec![transition.clone(), transition]),
            Err(EntityBuildError::DuplicateTransition)
        ));
        Ok(())
    }
}
