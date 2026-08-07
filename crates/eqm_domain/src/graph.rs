//! Deterministic containers for validated graph authority.

use crate::{
    AdapterDefinition, AdapterId, Binding, BindingId, Capability, CapabilityId, Extensions,
    Fragment, FragmentId, Journey, JourneyId, Policy, PolicyId, Profile, ProfileId,
    RepositoryIdentity, Revision, RunnerDefinition, RunnerId, SelectorText, Sha256Digest,
    SourceCommit, Surface, SurfaceId, Target, TargetId, TrustLevel, Waiver, WaiverId,
};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Exact resolved import identity retained in the semantic graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportLockIdentity {
    /// Imported fragment ID.
    pub id: FragmentId,
    /// Exact imported revision.
    pub revision: Revision,
    /// Canonical source repository.
    pub source: RepositoryIdentity,
    /// Immutable source commit.
    pub resolved: SourceCommit,
    /// Exact semantic digest.
    pub digest: Sha256Digest,
    /// Declared trust, defaulting to untrusted local.
    pub trust: TrustLevel,
    /// Optional detached signature metadata.
    pub signature: Option<SelectorText>,
}

/// Exact resolved adapter lock identity retained in the semantic graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterLockIdentity {
    /// Adapter ID.
    pub id: AdapterId,
    /// Exact immutable version.
    pub version: SelectorText,
    /// Canonical source repository.
    pub source: RepositoryIdentity,
    /// Immutable source commit.
    pub resolved: SourceCommit,
    /// Exact executable digest.
    pub digest: Sha256Digest,
    /// Exact protocol revision.
    pub protocol: Revision,
    /// Declared trust, defaulting to untrusted local.
    pub trust: TrustLevel,
    /// Optional detached signature metadata.
    pub signature: Option<SelectorText>,
}

/// Validated authority awaiting engine-level cross-reference resolution.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorkspaceGraphInput {
    /// Capability authorities.
    pub capabilities: Vec<Capability>,
    /// Journey authorities.
    pub journeys: Vec<Journey>,
    /// Surface authorities.
    pub surfaces: Vec<Surface>,
    /// Exact fragment revisions.
    pub fragments: Vec<Fragment>,
    /// Workspace target authorities.
    pub targets: Vec<Target>,
    /// Target-to-unit bindings.
    pub bindings: Vec<Binding>,
    /// Exact policy revisions.
    pub policies: Vec<Policy>,
    /// Exact profile revisions.
    pub profiles: Vec<Profile>,
    /// Exact runner revisions.
    pub runners: Vec<RunnerDefinition>,
    /// Exact waiver revisions.
    pub waivers: Vec<Waiver>,
    /// Digest-pinned adapter definitions.
    pub adapters: Vec<AdapterDefinition>,
    /// Resolved imports retained in semantic identity.
    pub imports: Vec<ImportLockIdentity>,
    /// Resolved adapter locks retained in semantic identity.
    pub adapter_locks: Vec<AdapterLockIdentity>,
    /// Normative workspace extensions.
    pub extensions: Extensions,
}

/// Immutable, deterministically indexed workspace authority.
///
/// Construction validates only duplicate keys. Referential integrity and graph
/// invariants deliberately remain engine responsibilities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkspaceGraph {
    capabilities: BTreeMap<CapabilityId, Capability>,
    journeys: BTreeMap<JourneyId, Journey>,
    surfaces: BTreeMap<SurfaceId, Surface>,
    fragments: BTreeMap<(FragmentId, Revision), Fragment>,
    targets: BTreeMap<TargetId, Target>,
    bindings: BTreeMap<BindingId, Binding>,
    binding_coordinates: BTreeMap<(TargetId, crate::UnitId), BindingId>,
    policies: BTreeMap<(PolicyId, Revision), Policy>,
    profiles: BTreeMap<(ProfileId, Revision), Profile>,
    runners: BTreeMap<(RunnerId, Revision), RunnerDefinition>,
    waivers: BTreeMap<(WaiverId, Revision), Waiver>,
    adapters: BTreeMap<(AdapterId, SelectorText, Sha256Digest), AdapterDefinition>,
    imports: BTreeMap<(FragmentId, Revision, Sha256Digest), ImportLockIdentity>,
    adapter_locks: BTreeMap<(AdapterId, SelectorText, Sha256Digest), AdapterLockIdentity>,
    extensions: Extensions,
}

impl WorkspaceGraph {
    /// Builds sorted indexes and rejects every duplicate semantic authority.
    pub fn new(input: WorkspaceGraphInput) -> Result<Self, WorkspaceGraphBuildError> {
        let capabilities = unique_index(input.capabilities, |value| value.id().clone())
            .map_err(|()| WorkspaceGraphBuildError::DuplicateCapability)?;
        let journeys = unique_index(input.journeys, |value| value.id().clone())
            .map_err(|()| WorkspaceGraphBuildError::DuplicateJourney)?;
        let surfaces = unique_index(input.surfaces, |value| value.id().clone())
            .map_err(|()| WorkspaceGraphBuildError::DuplicateSurface)?;
        let fragments = unique_index(input.fragments, |value| {
            (value.id().clone(), value.revision())
        })
        .map_err(|()| WorkspaceGraphBuildError::DuplicateFragment)?;
        let targets = unique_index(input.targets, |value| value.id().clone())
            .map_err(|()| WorkspaceGraphBuildError::DuplicateTarget)?;
        let bindings = unique_index(input.bindings, |value| value.id().clone())
            .map_err(|()| WorkspaceGraphBuildError::DuplicateBinding)?;
        let mut binding_coordinates = BTreeMap::new();
        for binding in bindings.values() {
            if binding_coordinates
                .insert(
                    (binding.target().clone(), binding.unit().clone()),
                    binding.id().clone(),
                )
                .is_some()
            {
                return Err(WorkspaceGraphBuildError::DuplicateBindingCoordinate);
            }
        }
        let policies = unique_index(input.policies, |value| {
            (value.id().clone(), value.revision())
        })
        .map_err(|()| WorkspaceGraphBuildError::DuplicatePolicy)?;
        let profiles = unique_index(input.profiles, |value| {
            (value.id().clone(), value.revision())
        })
        .map_err(|()| WorkspaceGraphBuildError::DuplicateProfile)?;
        let runners = unique_index(input.runners, |value| {
            (value.id().clone(), value.revision())
        })
        .map_err(|()| WorkspaceGraphBuildError::DuplicateRunner)?;
        let waivers = unique_index(input.waivers, |value| {
            (value.id().clone(), value.revision())
        })
        .map_err(|()| WorkspaceGraphBuildError::DuplicateWaiver)?;
        let adapters = unique_index(input.adapters, |value| {
            (value.id().clone(), value.version().clone(), value.digest())
        })
        .map_err(|()| WorkspaceGraphBuildError::DuplicateAdapter)?;
        let imports = unique_index(input.imports, |value| {
            (value.id.clone(), value.revision, value.digest)
        })
        .map_err(|()| WorkspaceGraphBuildError::DuplicateImportLock)?;
        let adapter_locks = unique_index(input.adapter_locks, |value| {
            (value.id.clone(), value.version.clone(), value.digest)
        })
        .map_err(|()| WorkspaceGraphBuildError::DuplicateAdapterLock)?;
        Ok(Self {
            capabilities,
            journeys,
            surfaces,
            fragments,
            targets,
            bindings,
            binding_coordinates,
            policies,
            profiles,
            runners,
            waivers,
            adapters,
            imports,
            adapter_locks,
            extensions: input.extensions,
        })
    }

    /// Returns capabilities in ID order.
    #[must_use]
    pub const fn capabilities(&self) -> &BTreeMap<CapabilityId, Capability> {
        &self.capabilities
    }
    /// Returns journeys in ID order.
    #[must_use]
    pub const fn journeys(&self) -> &BTreeMap<JourneyId, Journey> {
        &self.journeys
    }
    /// Returns surfaces in ID order.
    #[must_use]
    pub const fn surfaces(&self) -> &BTreeMap<SurfaceId, Surface> {
        &self.surfaces
    }
    /// Returns fragments in `(id, revision)` order.
    #[must_use]
    pub const fn fragments(&self) -> &BTreeMap<(FragmentId, Revision), Fragment> {
        &self.fragments
    }
    /// Returns targets in ID order.
    #[must_use]
    pub const fn targets(&self) -> &BTreeMap<TargetId, Target> {
        &self.targets
    }
    /// Returns bindings in authority-ID order.
    #[must_use]
    pub const fn bindings(&self) -> &BTreeMap<BindingId, Binding> {
        &self.bindings
    }
    /// Finds the unique binding for an exact `(target, unit)` coordinate.
    #[must_use]
    pub fn binding_for(&self, target: &TargetId, unit: &crate::UnitId) -> Option<&Binding> {
        self.binding_coordinates
            .get(&(target.clone(), unit.clone()))
            .and_then(|id| self.bindings.get(id))
    }
    /// Returns policies in `(id, revision)` order.
    #[must_use]
    pub const fn policies(&self) -> &BTreeMap<(PolicyId, Revision), Policy> {
        &self.policies
    }
    /// Returns profiles in `(id, revision)` order.
    #[must_use]
    pub const fn profiles(&self) -> &BTreeMap<(ProfileId, Revision), Profile> {
        &self.profiles
    }
    /// Returns runners in `(id, revision)` order.
    #[must_use]
    pub const fn runners(&self) -> &BTreeMap<(RunnerId, Revision), RunnerDefinition> {
        &self.runners
    }
    /// Returns waivers in `(id, revision)` order.
    #[must_use]
    pub const fn waivers(&self) -> &BTreeMap<(WaiverId, Revision), Waiver> {
        &self.waivers
    }
    /// Returns adapters in `(id, version, digest)` order.
    #[must_use]
    pub const fn adapters(
        &self,
    ) -> &BTreeMap<(AdapterId, SelectorText, Sha256Digest), AdapterDefinition> {
        &self.adapters
    }
    /// Returns resolved imports in `(id, revision, digest)` order.
    #[must_use]
    pub const fn imports(
        &self,
    ) -> &BTreeMap<(FragmentId, Revision, Sha256Digest), ImportLockIdentity> {
        &self.imports
    }
    /// Returns resolved adapter locks in `(id, version, digest)` order.
    #[must_use]
    pub const fn adapter_locks(
        &self,
    ) -> &BTreeMap<(AdapterId, SelectorText, Sha256Digest), AdapterLockIdentity> {
        &self.adapter_locks
    }
    /// Returns normative workspace extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

fn unique_index<T, K: Ord>(
    values: Vec<T>,
    mut key: impl FnMut(&T) -> K,
) -> Result<BTreeMap<K, T>, ()> {
    let count = values.len();
    let values: BTreeMap<_, _> = values
        .into_iter()
        .map(|value| (key(&value), value))
        .collect();
    (values.len() == count).then_some(values).ok_or(())
}

/// Duplicate authority detected while constructing graph indexes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkspaceGraphBuildError {
    /// Capability ID repeated.
    DuplicateCapability,
    /// Journey ID repeated.
    DuplicateJourney,
    /// Surface ID repeated.
    DuplicateSurface,
    /// Exact fragment revision repeated.
    DuplicateFragment,
    /// Target ID repeated.
    DuplicateTarget,
    /// Binding authority ID repeated.
    DuplicateBinding,
    /// More than one binding claimed a target/unit pair.
    DuplicateBindingCoordinate,
    /// Exact policy revision repeated.
    DuplicatePolicy,
    /// Exact profile revision repeated.
    DuplicateProfile,
    /// Exact runner revision repeated.
    DuplicateRunner,
    /// Exact waiver revision repeated.
    DuplicateWaiver,
    /// Exact adapter identity repeated.
    DuplicateAdapter,
    /// Exact import lock identity repeated.
    DuplicateImportLock,
    /// Exact adapter lock identity repeated.
    DuplicateAdapterLock,
}

impl Display for WorkspaceGraphBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for WorkspaceGraphBuildError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LifecycleStatus, OwnerRef, Title};
    use std::str::FromStr;

    fn capability(id: &str) -> Result<Capability, Box<dyn Error>> {
        Ok(Capability::new(
            CapabilityId::new(id)?,
            Title::new(id)?,
            LifecycleStatus::Active,
            vec![OwnerRef::from_str("owner://team/platform")?],
            None,
            Extensions::default(),
        )?)
    }

    #[test]
    fn insertion_order_does_not_change_indexes() -> Result<(), Box<dyn Error>> {
        let first = WorkspaceGraph::new(WorkspaceGraphInput {
            capabilities: vec![capability("zeta.unit")?, capability("alpha.unit")?],
            ..WorkspaceGraphInput::default()
        })?;
        let second = WorkspaceGraph::new(WorkspaceGraphInput {
            capabilities: vec![capability("alpha.unit")?, capability("zeta.unit")?],
            ..WorkspaceGraphInput::default()
        })?;
        assert_eq!(first, second);
        assert_eq!(
            first
                .capabilities()
                .keys()
                .map(CapabilityId::as_str)
                .collect::<Vec<_>>(),
            ["alpha.unit", "zeta.unit"]
        );
        Ok(())
    }

    #[test]
    fn duplicate_authority_is_rejected_before_resolution() -> Result<(), Box<dyn Error>> {
        let duplicate = capability("account.create")?;
        assert_eq!(
            WorkspaceGraph::new(WorkspaceGraphInput {
                capabilities: vec![duplicate.clone(), duplicate],
                ..WorkspaceGraphInput::default()
            }),
            Err(WorkspaceGraphBuildError::DuplicateCapability)
        );
        Ok(())
    }

    #[test]
    fn empty_graph_has_complete_empty_indexes() -> Result<(), WorkspaceGraphBuildError> {
        let graph = WorkspaceGraph::new(WorkspaceGraphInput::default())?;
        assert!(graph.capabilities().is_empty());
        assert!(graph.bindings().is_empty());
        assert!(graph.adapters().is_empty());
        assert!(graph.extensions().is_empty());
        Ok(())
    }
}
