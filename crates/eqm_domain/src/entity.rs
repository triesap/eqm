//! Shared entity values and the capability model.

use crate::{
    ArtifactId, ArtifactRole, BindingId, CapabilityId, DimensionId, EvidenceSpecId,
    EvidenceSpecification, Facet, FragmentId, FrameworkId, HttpMethod, IntendedExposureState,
    JourneyId, LifecycleStatus, LocalRequirementId, OwnerRef, PlatformId, ProviderId, RepoPath,
    RequirementLevel, RequirementScope, RiskClass, Sha256Digest, SurfaceId, SymbolicValueId,
    TargetId, UnitId,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroU64;
use unicode_normalization::UnicodeNormalization;

const MAX_EXTENSION_DEPTH: usize = 16;
const MAX_EXTENSION_NODES: usize = 1_024;
const MAX_EXTENSION_STRING_BYTES: usize = 16 * 1_024;
const MAX_EXTENSION_BYTES: usize = 256 * 1_024;
const MAX_APPLICABILITY_DEPTH: usize = 16;
const MAX_APPLICABILITY_NODES: usize = 256;

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
    /// Normalized artifact selector text of at most 512 UTF-8 bytes.
    SelectorText,
    512
);
text_value!(
    /// A normalized provider-neutral route selector of at most 512 UTF-8 bytes.
    RouteSelector,
    512
);
text_value!(
    /// A normalized transition trigger of at most 256 UTF-8 bytes.
    TransitionTrigger,
    256
);
text_value!(
    /// One normalized user-observable assertion of at most 4,096 UTF-8 bytes.
    RequirementStatement,
    4_096
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

/// Single-value applicability comparison.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComparisonOperator {
    /// Dimension equals the value.
    Equal,
    /// Dimension differs from the value.
    NotEqual,
}

impl ComparisonOperator {
    /// Returns the exact wire value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Equal => "eq",
            Self::NotEqual => "ne",
        }
    }
}

/// Set-membership applicability comparison.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MembershipOperator {
    /// Dimension is in the value set.
    In,
    /// Dimension is outside the value set.
    NotIn,
}

impl MembershipOperator {
    /// Returns the exact wire value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::In => "in",
            Self::NotIn => "not_in",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ApplicabilityExpression {
    Constant(bool),
    Comparison(DimensionId, ComparisonOperator, SymbolicValueId),
    Membership(DimensionId, MembershipOperator, BTreeSet<SymbolicValueId>),
    All(BTreeSet<Applicability>),
    Any(BTreeSet<Applicability>),
    Not(Box<Applicability>),
}

/// The discriminant of a validated applicability expression.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ApplicabilityKind {
    /// Boolean constant.
    Constant,
    /// Single-value comparison.
    Comparison,
    /// Set-membership comparison.
    Membership,
    /// Logical conjunction.
    All,
    /// Logical disjunction.
    Any,
    /// Logical negation.
    Not,
}

/// Read-only view of a validated applicability expression.
#[derive(Clone, Copy, Debug)]
pub enum ApplicabilityView<'a> {
    /// Boolean constant.
    Constant(bool),
    /// Single-value comparison.
    Comparison(&'a DimensionId, ComparisonOperator, &'a SymbolicValueId),
    /// Set membership comparison.
    Membership(
        &'a DimensionId,
        MembershipOperator,
        &'a BTreeSet<SymbolicValueId>,
    ),
    /// Canonically ordered conjunction.
    All(&'a BTreeSet<Applicability>),
    /// Canonically ordered disjunction.
    Any(&'a BTreeSet<Applicability>),
    /// Logical negation.
    Not(&'a Applicability),
}

/// A finite symbolic applicability expression.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Applicability {
    expression: ApplicabilityExpression,
    depth: u8,
    nodes: u16,
}

impl Applicability {
    /// Returns a read-only structured view for evaluation and projection.
    #[must_use]
    pub const fn view(&self) -> ApplicabilityView<'_> {
        match &self.expression {
            ApplicabilityExpression::Constant(value) => ApplicabilityView::Constant(*value),
            ApplicabilityExpression::Comparison(dimension, operator, value) => {
                ApplicabilityView::Comparison(dimension, *operator, value)
            }
            ApplicabilityExpression::Membership(dimension, operator, values) => {
                ApplicabilityView::Membership(dimension, *operator, values)
            }
            ApplicabilityExpression::All(values) => ApplicabilityView::All(values),
            ApplicabilityExpression::Any(values) => ApplicabilityView::Any(values),
            ApplicabilityExpression::Not(value) => ApplicabilityView::Not(value),
        }
    }

    /// Creates a boolean constant.
    #[must_use]
    pub const fn always(value: bool) -> Self {
        Self {
            expression: ApplicabilityExpression::Constant(value),
            depth: 1,
            nodes: 1,
        }
    }

    /// Creates a single-value symbolic comparison.
    #[must_use]
    pub const fn compare(
        dimension: DimensionId,
        operator: ComparisonOperator,
        value: SymbolicValueId,
    ) -> Self {
        Self {
            expression: ApplicabilityExpression::Comparison(dimension, operator, value),
            depth: 1,
            nodes: 1,
        }
    }

    /// Creates a nonempty, duplicate-free membership comparison.
    pub fn membership(
        dimension: DimensionId,
        operator: MembershipOperator,
        values: Vec<SymbolicValueId>,
    ) -> Result<Self, EntityBuildError> {
        if values.is_empty() {
            return Err(EntityBuildError::ApplicabilityOperandsRequired);
        }
        let count = values.len();
        let values: BTreeSet<_> = values.into_iter().collect();
        if values.len() != count {
            return Err(EntityBuildError::DuplicateApplicabilityOperand);
        }
        Ok(Self {
            expression: ApplicabilityExpression::Membership(dimension, operator, values),
            depth: 1,
            nodes: 1,
        })
    }

    /// Creates a nonempty conjunction and canonicalizes child order.
    pub fn all(values: Vec<Self>) -> Result<Self, EntityBuildError> {
        Self::logical(values, true)
    }

    /// Creates a nonempty disjunction and canonicalizes child order.
    pub fn any(values: Vec<Self>) -> Result<Self, EntityBuildError> {
        Self::logical(values, false)
    }

    fn logical(values: Vec<Self>, conjunction: bool) -> Result<Self, EntityBuildError> {
        if values.is_empty() {
            return Err(EntityBuildError::ApplicabilityOperandsRequired);
        }
        let count = values.len();
        let values: BTreeSet<_> = values.into_iter().collect();
        if values.len() != count {
            return Err(EntityBuildError::DuplicateApplicabilityOperand);
        }
        let depth = values.iter().map(|value| value.depth).max().unwrap_or(0) + 1;
        let nodes = values.iter().try_fold(1_u16, |total, value| {
            total
                .checked_add(value.nodes)
                .ok_or(EntityBuildError::ApplicabilityNodesExceeded)
        })?;
        let expression = if conjunction {
            ApplicabilityExpression::All(values)
        } else {
            ApplicabilityExpression::Any(values)
        };
        Self::with_bounds(expression, depth, nodes)
    }

    /// Creates a logical negation.
    pub fn logical_not(value: Self) -> Result<Self, EntityBuildError> {
        let depth = value.depth + 1;
        let nodes = value
            .nodes
            .checked_add(1)
            .ok_or(EntityBuildError::ApplicabilityNodesExceeded)?;
        Self::with_bounds(ApplicabilityExpression::Not(Box::new(value)), depth, nodes)
    }

    fn with_bounds(
        expression: ApplicabilityExpression,
        depth: u8,
        nodes: u16,
    ) -> Result<Self, EntityBuildError> {
        if usize::from(depth) > MAX_APPLICABILITY_DEPTH {
            return Err(EntityBuildError::ApplicabilityDepthExceeded);
        }
        if usize::from(nodes) > MAX_APPLICABILITY_NODES {
            return Err(EntityBuildError::ApplicabilityNodesExceeded);
        }
        Ok(Self {
            expression,
            depth,
            nodes,
        })
    }

    /// Returns the expression form without exposing invalid construction.
    #[must_use]
    pub const fn kind(&self) -> ApplicabilityKind {
        match &self.expression {
            ApplicabilityExpression::Constant(_) => ApplicabilityKind::Constant,
            ApplicabilityExpression::Comparison(..) => ApplicabilityKind::Comparison,
            ApplicabilityExpression::Membership(..) => ApplicabilityKind::Membership,
            ApplicabilityExpression::All(_) => ApplicabilityKind::All,
            ApplicabilityExpression::Any(_) => ApplicabilityKind::Any,
            ApplicabilityExpression::Not(_) => ApplicabilityKind::Not,
        }
    }

    /// Returns the validated tree depth.
    #[must_use]
    pub const fn depth(&self) -> u8 {
        self.depth
    }

    /// Returns the validated total node count.
    #[must_use]
    pub const fn nodes(&self) -> u16 {
        self.nodes
    }
}

impl Default for Applicability {
    fn default() -> Self {
        Self::always(true)
    }
}

/// One validated local requirement record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Requirement {
    id: LocalRequirementId,
    level: RequirementLevel,
    scope: RequirementScope,
    statement: RequirementStatement,
    facets: BTreeSet<Facet>,
    applicability: Applicability,
    risk_class: Option<RiskClass>,
    provider: Option<ProviderId>,
    extensions: Extensions,
}

impl Requirement {
    /// Creates a requirement and enforces facet and provider rules.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: LocalRequirementId,
        level: RequirementLevel,
        scope: RequirementScope,
        statement: RequirementStatement,
        facets: Vec<Facet>,
        applicability: Applicability,
        risk_class: Option<RiskClass>,
        provider: Option<ProviderId>,
        extensions: Extensions,
    ) -> Result<Self, EntityBuildError> {
        if facets.is_empty() {
            return Err(EntityBuildError::FacetsRequired);
        }
        let count = facets.len();
        let facets: BTreeSet<_> = facets.into_iter().collect();
        if facets.len() != count {
            return Err(EntityBuildError::DuplicateFacet);
        }
        match (scope, provider.is_some()) {
            (RequirementScope::SharedProvider, false) => {
                return Err(EntityBuildError::ProviderRequired);
            }
            (RequirementScope::SharedProvider, true) | (_, false) => {}
            (_, true) => return Err(EntityBuildError::ProviderForbidden),
        }
        Ok(Self {
            id,
            level,
            scope,
            statement,
            facets,
            applicability,
            risk_class,
            provider,
            extensions,
        })
    }

    /// Returns the local requirement ID.
    #[must_use]
    pub const fn id(&self) -> &LocalRequirementId {
        &self.id
    }
    /// Returns the requirement strength.
    #[must_use]
    pub const fn level(&self) -> RequirementLevel {
        self.level
    }
    /// Returns the obligation scope.
    #[must_use]
    pub const fn scope(&self) -> RequirementScope {
        self.scope
    }
    /// Returns the atomic normative statement.
    #[must_use]
    pub const fn statement(&self) -> &RequirementStatement {
        &self.statement
    }
    /// Returns facets in canonical order.
    #[must_use]
    pub const fn facets(&self) -> &BTreeSet<Facet> {
        &self.facets
    }
    /// Returns the finite symbolic applicability expression.
    #[must_use]
    pub const fn applicability(&self) -> &Applicability {
        &self.applicability
    }
    /// Returns an optional local risk elevation.
    #[must_use]
    pub const fn risk_class(&self) -> Option<RiskClass> {
        self.risk_class
    }
    /// Returns the shared provider when required by scope.
    #[must_use]
    pub const fn provider(&self) -> Option<&ProviderId> {
        self.provider.as_ref()
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

/// A versioned immutable fragment of reusable requirements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fragment {
    id: FragmentId,
    revision: Revision,
    title: Title,
    risk_class: RiskClass,
    owners: BTreeSet<OwnerRef>,
    requirements: BTreeMap<LocalRequirementId, Requirement>,
    description: Option<Description>,
    extensions: Extensions,
}

impl Fragment {
    /// Creates a fragment with nonempty uniquely identified requirements.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: FragmentId,
        revision: Revision,
        title: Title,
        risk_class: RiskClass,
        owners: Vec<OwnerRef>,
        requirements: Vec<Requirement>,
        description: Option<Description>,
        extensions: Extensions,
    ) -> Result<Self, EntityBuildError> {
        let owners = owner_set(owners)?;
        if requirements.is_empty() {
            return Err(EntityBuildError::RequirementsRequired);
        }
        let count = requirements.len();
        let requirements: BTreeMap<_, _> = requirements
            .into_iter()
            .map(|requirement| (requirement.id().clone(), requirement))
            .collect();
        if requirements.len() != count {
            return Err(EntityBuildError::DuplicateRequirement);
        }
        Ok(Self {
            id,
            revision,
            title,
            risk_class,
            owners,
            requirements,
            description,
            extensions,
        })
    }

    /// Returns the fragment ID.
    #[must_use]
    pub const fn id(&self) -> &FragmentId {
        &self.id
    }
    /// Returns the positive authored revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns the title.
    #[must_use]
    pub const fn title(&self) -> &Title {
        &self.title
    }
    /// Returns the inherited fragment risk.
    #[must_use]
    pub const fn risk_class(&self) -> RiskClass {
        self.risk_class
    }
    /// Returns owners in deterministic order.
    #[must_use]
    pub const fn owners(&self) -> &BTreeSet<OwnerRef> {
        &self.owners
    }
    /// Returns requirements keyed by immutable local identity.
    #[must_use]
    pub const fn requirements(&self) -> &BTreeMap<LocalRequirementId, Requirement> {
        &self.requirements
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

/// An exact immutable use of one fragment authority.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FragmentUse {
    fragment: FragmentId,
    revision: Revision,
    digest: Sha256Digest,
    prefix: Option<LocalRequirementId>,
}

impl FragmentUse {
    /// Creates an exact fragment pin; no override fields exist in this type.
    #[must_use]
    pub const fn new(
        fragment: FragmentId,
        revision: Revision,
        digest: Sha256Digest,
        prefix: Option<LocalRequirementId>,
    ) -> Self {
        Self {
            fragment,
            revision,
            digest,
            prefix,
        }
    }
    /// Returns the pinned fragment ID.
    #[must_use]
    pub const fn fragment(&self) -> &FragmentId {
        &self.fragment
    }
    /// Returns the exact pinned revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns the exact pinned semantic digest.
    #[must_use]
    pub const fn digest(&self) -> &Sha256Digest {
        &self.digest
    }
    /// Returns the optional local requirement prefix.
    #[must_use]
    pub const fn prefix(&self) -> Option<&LocalRequirementId> {
        self.prefix.as_ref()
    }
}

/// A versioned surface authority containing direct and pinned requirements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Surface {
    id: SurfaceId,
    revision: Revision,
    title: Title,
    journey: JourneyId,
    status: LifecycleStatus,
    owners: BTreeSet<OwnerRef>,
    requirements: BTreeMap<LocalRequirementId, Requirement>,
    fragments: BTreeSet<FragmentUse>,
    description: Option<Description>,
    extensions: Extensions,
}

impl Surface {
    /// Creates a nonempty surface composition with unique local identities and pins.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: SurfaceId,
        revision: Revision,
        title: Title,
        journey: JourneyId,
        status: LifecycleStatus,
        owners: Vec<OwnerRef>,
        requirements: Vec<Requirement>,
        fragments: Vec<FragmentUse>,
        description: Option<Description>,
        extensions: Extensions,
    ) -> Result<Self, EntityBuildError> {
        let owners = owner_set(owners)?;
        if requirements.is_empty() && fragments.is_empty() {
            return Err(EntityBuildError::SurfaceCompositionRequired);
        }
        let requirement_count = requirements.len();
        let requirements: BTreeMap<_, _> = requirements
            .into_iter()
            .map(|requirement| (requirement.id().clone(), requirement))
            .collect();
        if requirements.len() != requirement_count {
            return Err(EntityBuildError::DuplicateRequirement);
        }
        let fragment_count = fragments.len();
        let fragments: BTreeSet<_> = fragments.into_iter().collect();
        if fragments.len() != fragment_count {
            return Err(EntityBuildError::DuplicateFragmentUse);
        }
        Ok(Self {
            id,
            revision,
            title,
            journey,
            status,
            owners,
            requirements,
            fragments,
            description,
            extensions,
        })
    }

    /// Returns the surface ID.
    #[must_use]
    pub const fn id(&self) -> &SurfaceId {
        &self.id
    }
    /// Returns the positive authored revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns the title.
    #[must_use]
    pub const fn title(&self) -> &Title {
        &self.title
    }
    /// Returns the parent journey ID.
    #[must_use]
    pub const fn journey(&self) -> &JourneyId {
        &self.journey
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
    /// Returns direct requirements keyed by local identity.
    #[must_use]
    pub const fn requirements(&self) -> &BTreeMap<LocalRequirementId, Requirement> {
        &self.requirements
    }
    /// Returns exact fragment uses in deterministic pin order.
    #[must_use]
    pub const fn fragments(&self) -> &BTreeSet<FragmentUse> {
        &self.fragments
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

/// A repository implementation target authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Target {
    id: TargetId,
    root: RepoPath,
    platform: PlatformId,
    framework: FrameworkId,
    owners: BTreeSet<OwnerRef>,
    extensions: Extensions,
}

impl Target {
    /// Creates a target with a validated repository root and nonempty owners.
    pub fn new(
        id: TargetId,
        root: RepoPath,
        platform: PlatformId,
        framework: FrameworkId,
        owners: Vec<OwnerRef>,
        extensions: Extensions,
    ) -> Result<Self, EntityBuildError> {
        Ok(Self {
            id,
            root,
            platform,
            framework,
            owners: owner_set(owners)?,
            extensions,
        })
    }
    /// Returns the target ID.
    #[must_use]
    pub const fn id(&self) -> &TargetId {
        &self.id
    }
    /// Returns the repository-relative source root.
    #[must_use]
    pub const fn root(&self) -> &RepoPath {
        &self.root
    }
    /// Returns the extensible platform ID.
    #[must_use]
    pub const fn platform(&self) -> &PlatformId {
        &self.platform
    }
    /// Returns the extensible framework ID.
    #[must_use]
    pub const fn framework(&self) -> &FrameworkId {
        &self.framework
    }
    /// Returns owners in deterministic order.
    #[must_use]
    pub const fn owners(&self) -> &BTreeSet<OwnerRef> {
        &self.owners
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

/// A typed provider-neutral artifact selector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArtifactSelector {
    /// A source symbol with optional language.
    Symbol {
        /// Exact symbol name.
        name: SelectorText,
        /// Optional source language.
        language: Option<SelectorText>,
    },
    /// A route path with optional closed HTTP method.
    Route {
        /// Provider-neutral route path.
        path: SelectorText,
        /// Optional closed HTTP method.
        method: Option<HttpMethod>,
    },
    /// A test identity with optional suite.
    Test {
        /// Test framework identity.
        framework: SelectorText,
        /// Exact framework test identity.
        test_id: SelectorText,
        /// Optional test suite identity.
        suite: Option<SelectorText>,
    },
    /// A provider-neutral inventory coordinate.
    Inventory {
        /// Provider-neutral record type.
        record_type: SelectorText,
        /// Exact record key.
        key: SelectorText,
        /// Optional expected record value.
        value: Option<SelectorText>,
    },
}

/// One artifact declared by a target binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Artifact {
    id: ArtifactId,
    role: ArtifactRole,
    path: RepoPath,
    surface: Option<SurfaceId>,
    symbol: Option<SelectorText>,
    selector: Option<ArtifactSelector>,
    extensions: Extensions,
}

impl Artifact {
    /// Creates an artifact and enforces user-visible coverage locators.
    pub fn new(
        id: ArtifactId,
        role: ArtifactRole,
        path: RepoPath,
        surface: Option<SurfaceId>,
        symbol: Option<SelectorText>,
        selector: Option<ArtifactSelector>,
        extensions: Extensions,
    ) -> Result<Self, EntityBuildError> {
        let user_visible = matches!(
            role,
            ArtifactRole::Entrypoint
                | ArtifactRole::View
                | ArtifactRole::Route
                | ArtifactRole::Component
        );
        if user_visible && surface.is_none() && symbol.is_none() && selector.is_none() {
            return Err(EntityBuildError::ArtifactLocatorRequired);
        }
        Ok(Self {
            id,
            role,
            path,
            surface,
            symbol,
            selector,
            extensions,
        })
    }
    /// Returns the local artifact ID.
    #[must_use]
    pub const fn id(&self) -> &ArtifactId {
        &self.id
    }
    /// Returns the semantic artifact role.
    #[must_use]
    pub const fn role(&self) -> ArtifactRole {
        self.role
    }
    /// Returns the repository path.
    #[must_use]
    pub const fn path(&self) -> &RepoPath {
        &self.path
    }
    /// Returns an optional covered surface.
    #[must_use]
    pub const fn surface(&self) -> Option<&SurfaceId> {
        self.surface.as_ref()
    }
    /// Returns optional normalized symbol metadata.
    #[must_use]
    pub const fn symbol(&self) -> Option<&SelectorText> {
        self.symbol.as_ref()
    }
    /// Returns an optional typed selector.
    #[must_use]
    pub const fn selector(&self) -> Option<&ArtifactSelector> {
        self.selector.as_ref()
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

/// A nonempty set of artifacts uniquely keyed within one binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Artifacts(BTreeMap<ArtifactId, Artifact>);

impl Artifacts {
    /// Validates nonempty unique artifact IDs.
    pub fn new(values: Vec<Artifact>) -> Result<Self, EntityBuildError> {
        if values.is_empty() {
            return Err(EntityBuildError::ArtifactsRequired);
        }
        let count = values.len();
        let values: BTreeMap<_, _> = values
            .into_iter()
            .map(|value| (value.id().clone(), value))
            .collect();
        if values.len() != count {
            return Err(EntityBuildError::DuplicateArtifact);
        }
        Ok(Self(values))
    }
    /// Returns artifacts in local ID order.
    #[must_use]
    pub const fn values(&self) -> &BTreeMap<ArtifactId, Artifact> {
        &self.0
    }
}

/// A profile-relative intended exposure declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Exposure {
    surface: SurfaceId,
    state: IntendedExposureState,
    applicability: Applicability,
    route: Option<RouteSelector>,
    extensions: Extensions,
}

impl Exposure {
    /// Creates an intended exposure without conflating runtime observation.
    #[must_use]
    pub const fn new(
        surface: SurfaceId,
        state: IntendedExposureState,
        applicability: Applicability,
        route: Option<RouteSelector>,
        extensions: Extensions,
    ) -> Self {
        Self {
            surface,
            state,
            applicability,
            route,
            extensions,
        }
    }
    /// Returns the intended surface.
    #[must_use]
    pub const fn surface(&self) -> &SurfaceId {
        &self.surface
    }
    /// Returns required or prohibited intent.
    #[must_use]
    pub const fn state(&self) -> IntendedExposureState {
        self.state
    }
    /// Returns the finite symbolic applicability selector.
    #[must_use]
    pub const fn applicability(&self) -> &Applicability {
        &self.applicability
    }
    /// Returns an optional route selector.
    #[must_use]
    pub const fn route(&self) -> Option<&RouteSelector> {
        self.route.as_ref()
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

/// A versioned declaration that maps one product unit onto one implementation target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Binding {
    id: BindingId,
    revision: Revision,
    owners: BTreeSet<OwnerRef>,
    target: TargetId,
    unit: UnitId,
    artifacts: Artifacts,
    exposures: Vec<Exposure>,
    evidence: BTreeMap<EvidenceSpecId, EvidenceSpecification>,
    extensions: Extensions,
}

impl Binding {
    /// Creates a binding with unique owners, exposures, and evidence IDs.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: BindingId,
        revision: Revision,
        owners: Vec<OwnerRef>,
        target: TargetId,
        unit: UnitId,
        artifacts: Artifacts,
        exposures: Vec<Exposure>,
        evidence: Vec<EvidenceSpecification>,
        extensions: Extensions,
    ) -> Result<Self, EntityBuildError> {
        let owners = owner_set(owners)?;
        for (index, exposure) in exposures.iter().enumerate() {
            if exposures[..index].contains(exposure) {
                return Err(EntityBuildError::DuplicateExposure);
            }
        }
        let evidence_count = evidence.len();
        let evidence: BTreeMap<_, _> = evidence
            .into_iter()
            .map(|specification| (specification.id().clone(), specification))
            .collect();
        if evidence.len() != evidence_count {
            return Err(EntityBuildError::DuplicateEvidenceSpecification);
        }
        Ok(Self {
            id,
            revision,
            owners,
            target,
            unit,
            artifacts,
            exposures,
            evidence,
            extensions,
        })
    }

    /// Returns the binding authority ID.
    #[must_use]
    pub const fn id(&self) -> &BindingId {
        &self.id
    }
    /// Returns the authored revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns owners in deterministic order.
    #[must_use]
    pub const fn owners(&self) -> &BTreeSet<OwnerRef> {
        &self.owners
    }
    /// Returns the implementation target.
    #[must_use]
    pub const fn target(&self) -> &TargetId {
        &self.target
    }
    /// Returns the fully qualified product unit.
    #[must_use]
    pub const fn unit(&self) -> &UnitId {
        &self.unit
    }
    /// Returns the nonempty artifacts by local ID.
    #[must_use]
    pub const fn artifacts(&self) -> &Artifacts {
        &self.artifacts
    }
    /// Returns intended exposures in authored order until graph finalization.
    #[must_use]
    pub fn exposures(&self) -> &[Exposure] {
        &self.exposures
    }
    /// Returns evidence specifications by local ID.
    #[must_use]
    pub const fn evidence(&self) -> &BTreeMap<EvidenceSpecId, EvidenceSpecification> {
        &self.evidence
    }
    /// Returns normative extensions.
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
    /// A requirement did not declare any facets.
    FacetsRequired,
    /// A requirement declared a facet more than once.
    DuplicateFacet,
    /// Shared-provider scope omitted its provider.
    ProviderRequired,
    /// A non-provider scope declared a provider.
    ProviderForbidden,
    /// A logical or membership expression omitted its operands.
    ApplicabilityOperandsRequired,
    /// An applicability operand appeared more than once.
    DuplicateApplicabilityOperand,
    /// Applicability nesting exceeded 16 levels.
    ApplicabilityDepthExceeded,
    /// Applicability data exceeded 256 nodes.
    ApplicabilityNodesExceeded,
    /// A fragment did not declare any requirements.
    RequirementsRequired,
    /// A local requirement ID appeared more than once in one authority.
    DuplicateRequirement,
    /// A surface declared neither a direct requirement nor a fragment use.
    SurfaceCompositionRequired,
    /// The same exact fragment pin appeared more than once.
    DuplicateFragmentUse,
    /// A user-visible artifact omitted every coverage locator.
    ArtifactLocatorRequired,
    /// A binding did not declare any artifacts.
    ArtifactsRequired,
    /// A local artifact ID appeared more than once.
    DuplicateArtifact,
    /// A binding repeated the same exposure declaration.
    DuplicateExposure,
    /// A binding repeated a local evidence-specification ID.
    DuplicateEvidenceSpecification,
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
            Self::FacetsRequired => "requirement requires at least one facet",
            Self::DuplicateFacet => "requirement facets contain a duplicate",
            Self::ProviderRequired => "shared-provider requirement requires a provider",
            Self::ProviderForbidden => "requirement provider is forbidden for this scope",
            Self::ApplicabilityOperandsRequired => "applicability requires at least one operand",
            Self::DuplicateApplicabilityOperand => "applicability contains a duplicate operand",
            Self::ApplicabilityDepthExceeded => "applicability depth exceeds 16",
            Self::ApplicabilityNodesExceeded => "applicability node count exceeds 256",
            Self::RequirementsRequired => "fragment requires at least one requirement",
            Self::DuplicateRequirement => "authority requirements contain a duplicate local ID",
            Self::SurfaceCompositionRequired => {
                "surface requires at least one direct requirement or fragment use"
            }
            Self::DuplicateFragmentUse => "surface fragments contain a duplicate exact pin",
            Self::ArtifactLocatorRequired => "user-visible artifact requires a coverage locator",
            Self::ArtifactsRequired => "binding requires at least one artifact",
            Self::DuplicateArtifact => "binding artifacts contain a duplicate local ID",
            Self::DuplicateExposure => "binding exposures contain a duplicate declaration",
            Self::DuplicateEvidenceSpecification => {
                "binding evidence contains a duplicate local ID"
            }
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

    fn requirement(value: &str) -> Result<Requirement, Box<dyn Error>> {
        Ok(Requirement::new(
            LocalRequirementId::new(value)?,
            RequirementLevel::Required,
            RequirementScope::EachTarget,
            RequirementStatement::new("The behavior is available")?,
            vec![Facet::Behavior],
            Applicability::default(),
            None,
            None,
            Extensions::default(),
        )?)
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

    #[test]
    fn requirement_enforces_facets_and_provider_scope() -> Result<(), Box<dyn Error>> {
        let id = LocalRequirementId::new("reachable")?;
        let statement = RequirementStatement::new("The form is reachable")?;
        let base = |scope, facets, provider| {
            Requirement::new(
                id.clone(),
                RequirementLevel::Required,
                scope,
                statement.clone(),
                facets,
                Applicability::default(),
                Some(RiskClass::High),
                provider,
                Extensions::default(),
            )
        };
        let provider = ProviderId::new("identity.primary")?;
        let requirement = base(
            RequirementScope::SharedProvider,
            vec![Facet::Reachability],
            Some(provider.clone()),
        )?;
        assert_eq!(requirement.provider(), Some(&provider));
        assert_eq!(
            requirement.applicability().kind(),
            ApplicabilityKind::Constant
        );
        assert!(matches!(
            base(RequirementScope::EachTarget, Vec::new(), None),
            Err(EntityBuildError::FacetsRequired)
        ));
        assert!(matches!(
            base(
                RequirementScope::EachTarget,
                vec![Facet::Behavior, Facet::Behavior],
                None,
            ),
            Err(EntityBuildError::DuplicateFacet)
        ));
        assert!(matches!(
            base(
                RequirementScope::SharedProvider,
                vec![Facet::Behavior],
                None,
            ),
            Err(EntityBuildError::ProviderRequired)
        ));
        assert!(matches!(
            base(
                RequirementScope::EndToEnd,
                vec![Facet::Behavior],
                Some(provider),
            ),
            Err(EntityBuildError::ProviderForbidden)
        ));
        Ok(())
    }

    #[test]
    fn applicability_is_bounded_and_duplicate_free() -> Result<(), Box<dyn Error>> {
        let dimension = DimensionId::new("region")?;
        let eu = SymbolicValueId::new("eu")?;
        assert!(matches!(
            Applicability::membership(dimension.clone(), MembershipOperator::In, Vec::new()),
            Err(EntityBuildError::ApplicabilityOperandsRequired)
        ));
        assert!(matches!(
            Applicability::membership(
                dimension.clone(),
                MembershipOperator::In,
                vec![eu.clone(), eu.clone()],
            ),
            Err(EntityBuildError::DuplicateApplicabilityOperand)
        ));
        let comparison = Applicability::compare(dimension, ComparisonOperator::Equal, eu);
        assert!(matches!(
            Applicability::all(vec![comparison.clone(), comparison]),
            Err(EntityBuildError::DuplicateApplicabilityOperand)
        ));
        let mut nested = Applicability::always(true);
        for _ in 1..MAX_APPLICABILITY_DEPTH {
            nested = Applicability::logical_not(nested)?;
        }
        assert_eq!(nested.depth(), MAX_APPLICABILITY_DEPTH as u8);
        assert_eq!(
            Applicability::logical_not(nested),
            Err(EntityBuildError::ApplicabilityDepthExceeded)
        );
        let many: Vec<_> = (0..MAX_APPLICABILITY_NODES)
            .map(|index| {
                Ok(Applicability::compare(
                    DimensionId::new(format!("d{index}"))?,
                    ComparisonOperator::Equal,
                    SymbolicValueId::new("on")?,
                ))
            })
            .collect::<Result<_, crate::IdParseError>>()?;
        assert_eq!(
            Applicability::all(many),
            Err(EntityBuildError::ApplicabilityNodesExceeded)
        );
        Ok(())
    }

    #[test]
    fn fragment_requires_unique_nonempty_composition() -> Result<(), Box<dyn Error>> {
        let build = |requirements| {
            Fragment::new(
                FragmentId::new("shared.account").map_err(|_| EntityBuildError::InvalidText)?,
                Revision::new(2)?,
                Title::new("Shared account behavior")?,
                RiskClass::Medium,
                vec![owner("owner://team/accounts").map_err(|_| EntityBuildError::InvalidText)?],
                requirements,
                None,
                Extensions::default(),
            )
        };
        assert!(matches!(
            build(Vec::new()),
            Err(EntityBuildError::RequirementsRequired)
        ));
        let item = requirement("reachable")?;
        assert!(matches!(
            build(vec![item.clone(), item]),
            Err(EntityBuildError::DuplicateRequirement)
        ));
        let fragment = build(vec![requirement("reachable")?, requirement("submittable")?])?;
        assert_eq!(fragment.revision().get(), 2);
        assert_eq!(
            fragment
                .requirements()
                .keys()
                .map(LocalRequirementId::as_str)
                .collect::<Vec<_>>(),
            ["reachable", "submittable"]
        );
        Ok(())
    }

    #[test]
    fn surface_requires_unique_direct_or_exact_pinned_composition() -> Result<(), Box<dyn Error>> {
        let id = SurfaceId::new("account.create.signup.start")?;
        let revision = Revision::new(1)?;
        let title = Title::new("Start signup")?;
        let journey = JourneyId::new("account.create.signup")?;
        let account_owner = owner("owner://team/accounts")?;
        let digest: Sha256Digest =
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".parse()?;
        let fragment_use = FragmentUse::new(
            FragmentId::new("shared.account")?,
            revision,
            digest,
            Some(LocalRequirementId::new("shared")?),
        );
        let build = |requirements, fragments| {
            Surface::new(
                id.clone(),
                revision,
                title.clone(),
                journey.clone(),
                LifecycleStatus::Active,
                vec![account_owner.clone()],
                requirements,
                fragments,
                None,
                Extensions::default(),
            )
        };
        assert!(matches!(
            build(Vec::new(), Vec::new()),
            Err(EntityBuildError::SurfaceCompositionRequired)
        ));
        let direct = requirement("reachable")?;
        assert!(matches!(
            build(vec![direct.clone(), direct], Vec::new()),
            Err(EntityBuildError::DuplicateRequirement)
        ));
        assert!(matches!(
            build(Vec::new(), vec![fragment_use.clone(), fragment_use.clone()]),
            Err(EntityBuildError::DuplicateFragmentUse)
        ));
        let surface = build(vec![requirement("reachable")?], vec![fragment_use.clone()])?;
        assert_eq!(surface.fragments().first(), Some(&fragment_use));
        assert_eq!(surface.requirements().len(), 1);
        assert_eq!(fragment_use.revision().get(), 1);
        assert_eq!(
            fragment_use.prefix().map(LocalRequirementId::as_str),
            Some("shared")
        );
        Ok(())
    }

    #[test]
    fn target_keeps_implementation_identity_dimensions_separate() -> Result<(), Box<dyn Error>> {
        let target = Target::new(
            TargetId::new("web")?,
            RepoPath::new("apps/web")?,
            PlatformId::new("web")?,
            FrameworkId::new("sveltekit")?,
            vec![owner("owner://team/web")?],
            Extensions::default(),
        )?;
        assert_eq!(target.id().as_str(), "web");
        assert_eq!(target.root().as_str(), "apps/web");
        assert_eq!(target.platform().as_str(), "web");
        assert_eq!(target.framework().as_str(), "sveltekit");
        assert!(matches!(
            Target::new(
                TargetId::new("ios")?,
                RepoPath::new("apps/ios")?,
                PlatformId::new("ios")?,
                FrameworkId::new("swiftui")?,
                Vec::new(),
                Extensions::default(),
            ),
            Err(EntityBuildError::OwnersRequired)
        ));
        Ok(())
    }

    #[test]
    fn artifacts_enforce_role_locators_and_unique_ids() -> Result<(), Box<dyn Error>> {
        let missing = Artifact::new(
            ArtifactId::new("signup")?,
            ArtifactRole::View,
            RepoPath::new("src/signup.rs")?,
            None,
            None,
            None,
            Extensions::default(),
        );
        assert_eq!(missing, Err(EntityBuildError::ArtifactLocatorRequired));
        let artifact = Artifact::new(
            ArtifactId::new("signup")?,
            ArtifactRole::Route,
            RepoPath::new("src/signup.rs")?,
            Some(SurfaceId::new("account.create.signup.start")?),
            Some(SelectorText::new("signup_handler")?),
            Some(ArtifactSelector::Route {
                path: SelectorText::new("/signup")?,
                method: Some(HttpMethod::Get),
            }),
            Extensions::default(),
        )?;
        assert_eq!(artifact.role(), ArtifactRole::Route);
        assert!(matches!(
            Artifacts::new(Vec::new()),
            Err(EntityBuildError::ArtifactsRequired)
        ));
        assert!(matches!(
            Artifacts::new(vec![artifact.clone(), artifact]),
            Err(EntityBuildError::DuplicateArtifact)
        ));
        Ok(())
    }

    #[test]
    fn exposure_is_symbolic_intent_not_observed_state() -> Result<(), Box<dyn Error>> {
        let applicability = Applicability::compare(
            DimensionId::new("region")?,
            ComparisonOperator::Equal,
            SymbolicValueId::new("eu")?,
        );
        let exposure = Exposure::new(
            SurfaceId::new("account.create.signup.start")?,
            IntendedExposureState::Required,
            applicability,
            Some(RouteSelector::new("/signup")?),
            Extensions::default(),
        );
        assert_eq!(exposure.state(), IntendedExposureState::Required);
        assert_eq!(
            exposure.applicability().kind(),
            ApplicabilityKind::Comparison
        );
        assert_eq!(exposure.route().map(RouteSelector::as_str), Some("/signup"));
        Ok(())
    }

    #[test]
    fn binding_enforces_local_uniqueness_and_preserves_coordinates() -> Result<(), Box<dyn Error>> {
        let artifact = Artifact::new(
            ArtifactId::new("config")?,
            ArtifactRole::Configuration,
            RepoPath::new("config/eqm.toml")?,
            None,
            None,
            None,
            Extensions::default(),
        )?;
        let exposure = Exposure::new(
            SurfaceId::new("account.create.signup.start")?,
            IntendedExposureState::Required,
            Applicability::default(),
            None,
            Extensions::default(),
        );
        let build = |exposures| {
            Binding::new(
                BindingId::new("web.account").map_err(|_| EntityBuildError::InvalidText)?,
                Revision::new(1)?,
                vec![owner("owner://team/web").map_err(|_| EntityBuildError::InvalidText)?],
                TargetId::new("web").map_err(|_| EntityBuildError::InvalidText)?,
                UnitId::new("account.create").map_err(|_| EntityBuildError::InvalidText)?,
                Artifacts::new(vec![artifact.clone()])?,
                exposures,
                Vec::new(),
                Extensions::default(),
            )
        };
        assert_eq!(
            build(vec![exposure.clone(), exposure.clone()]),
            Err(EntityBuildError::DuplicateExposure)
        );
        let binding = build(vec![exposure])?;
        assert_eq!(binding.target().as_str(), "web");
        assert_eq!(binding.unit().as_str(), "account.create");
        assert_eq!(binding.artifacts().values().len(), 1);
        assert!(binding.evidence().is_empty());
        Ok(())
    }
}
