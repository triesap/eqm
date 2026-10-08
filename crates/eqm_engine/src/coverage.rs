//! Exact evidence coverage matching.

use eqm_domain::{
    EvidenceKind, EvidenceResult, EvidenceSpecId, EvidenceSubject, Facet, FullRequirementId,
    ProfileId, ProfileSelection, Sha256Digest, TargetId, UnitId,
};
use std::collections::{BTreeMap, BTreeSet};

/// Exact prepared coordinates required by one obligation/specification pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoverageExpectation {
    /// Exact local evidence specification ID.
    pub evidence_spec_id: EvidenceSpecId,
    /// Exact semantic specification digest.
    pub evidence_spec_digest: Sha256Digest,
    /// Exact obligation requirement.
    pub requirement: FullRequirementId,
    /// Exact obligation facet.
    pub facet: Facet,
    /// Exact repository/source/build/artifact/configuration and scope subject.
    pub subject: EvidenceSubject,
    /// Exact target carried by the result envelope.
    pub target: TargetId,
    /// Exact unit.
    pub unit: UnitId,
    /// Required evidence kind.
    pub kind: EvidenceKind,
    /// Exact semantic contract digest.
    pub contract_digest: Sha256Digest,
    /// Exact binding digest.
    pub binding_digest: Sha256Digest,
    /// Exact selected policy digest.
    pub policy_digest: Sha256Digest,
    /// Exact runner digest, including typed absence.
    pub runner_digest: Option<Sha256Digest>,
    /// Exact adapter digest, including typed absence.
    pub adapter_digest: Option<Sha256Digest>,
    /// Exact runtime-facts reference, including typed absence.
    pub runtime_facts_digest: Option<Sha256Digest>,
    /// Exact profile values.
    pub profiles: BTreeMap<ProfileId, ProfileSelection>,
    /// Exact release context, including typed absence.
    pub release_record_digest: Option<Sha256Digest>,
}

/// Evidence plus the specification ID supplied by its owning binding.
#[derive(Clone, Copy, Debug)]
pub struct EvidenceCandidate<'a> {
    /// Exact local evidence specification ID.
    pub evidence_spec_id: &'a EvidenceSpecId,
    /// Immutable result envelope.
    pub result: &'a EvidenceResult,
}

/// Exact mismatch axis; no fuzzy match is attempted.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CoverageMismatch {
    /// Specification ID differed.
    EvidenceSpecId,
    /// Specification digest differed.
    EvidenceSpecDigest,
    /// Requirement was not explicitly listed.
    Requirement,
    /// Facet was not explicitly listed.
    Facet,
    /// Scope subject differed.
    Subject,
    /// Repository location differed.
    Repository,
    /// Repository identity digest differed.
    RepositoryIdentity,
    /// Source commit differed.
    SourceCommit,
    /// Build identity or its typed absence differed.
    Build,
    /// Artifact identity or its typed absence differed.
    Artifact,
    /// Target configuration (including declared role) differed.
    TargetConfiguration,
    /// Target differed.
    Target,
    /// Unit differed.
    Unit,
    /// Evidence kind differed.
    Kind,
    /// Contract digest differed.
    ContractDigest,
    /// Binding digest differed.
    BindingDigest,
    /// Policy digest differed.
    PolicyDigest,
    /// Runner digest differed.
    RunnerDigest,
    /// Adapter digest differed.
    AdapterDigest,
    /// Runtime-facts reference or typed absence differed.
    RuntimeFacts,
    /// Profile selections differed.
    Profiles,
    /// Release context differed.
    ReleaseContext,
}

/// Overall exact coverage state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoverageStatus {
    /// At least one unique result matched every coordinate.
    Covered,
    /// No result matched every coordinate.
    Missing,
    /// Conflicting relevant immutable assertions make coverage unknowable.
    Unknown,
}

/// Complete deterministic coverage analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoverageReport {
    /// Overall status.
    pub status: CoverageStatus,
    /// Exact compatible result IDs.
    pub covered: BTreeSet<Sha256Digest>,
    /// Rejected result IDs and every mismatching coordinate.
    pub rejected: BTreeMap<Sha256Digest, BTreeSet<CoverageMismatch>>,
    /// Repeated immutable result IDs.
    pub duplicate_result_ids: BTreeSet<Sha256Digest>,
    /// Different compatible domain records claiming one immutable result ID.
    pub conflicting_result_ids: BTreeSet<Sha256Digest>,
}

/// Maps immutable evidence results to an obligation by exact coordinates only.
#[must_use]
pub fn evaluate_evidence_coverage(
    expected: &CoverageExpectation,
    candidates: &[EvidenceCandidate<'_>],
) -> CoverageReport {
    let mut seen = BTreeSet::new();
    let mut duplicates = BTreeSet::new();
    let mut covered = BTreeSet::new();
    let mut rejected = BTreeMap::new();
    let mut matching = BTreeMap::new();
    let mut conflicts = BTreeSet::new();
    for candidate in candidates {
        let result = candidate.result;
        if !seen.insert(result.id()) {
            duplicates.insert(result.id());
        }
        let mismatches = mismatches(expected, candidate);
        if mismatches.is_empty() {
            if let Some(previous) = matching.insert(result.id(), result)
                && previous != result
            {
                conflicts.insert(result.id());
            }
            covered.insert(result.id());
        } else {
            rejected
                .entry(result.id())
                .or_insert_with(BTreeSet::new)
                .extend(mismatches);
        }
    }
    for conflict in &conflicts {
        covered.remove(conflict);
    }
    let status = if conflicts.is_empty() {
        if covered.is_empty() {
            CoverageStatus::Missing
        } else {
            CoverageStatus::Covered
        }
    } else {
        CoverageStatus::Unknown
    };
    CoverageReport {
        status,
        covered,
        rejected,
        duplicate_result_ids: duplicates,
        conflicting_result_ids: conflicts,
    }
}

pub(super) fn mismatches(
    expected: &CoverageExpectation,
    candidate: &EvidenceCandidate<'_>,
) -> BTreeSet<CoverageMismatch> {
    let result = candidate.result;
    let checks = [
        (
            candidate.evidence_spec_id == &expected.evidence_spec_id,
            CoverageMismatch::EvidenceSpecId,
        ),
        (
            result.evidence_spec_digest() == expected.evidence_spec_digest,
            CoverageMismatch::EvidenceSpecDigest,
        ),
        (
            result.requirements().contains(&expected.requirement),
            CoverageMismatch::Requirement,
        ),
        (
            result.facets().contains(&expected.facet),
            CoverageMismatch::Facet,
        ),
        (
            result.subject().scope() == expected.subject.scope(),
            CoverageMismatch::Subject,
        ),
        (
            result.subject().repository() == expected.subject.repository(),
            CoverageMismatch::Repository,
        ),
        (
            result.subject().repository_id_digest() == expected.subject.repository_id_digest(),
            CoverageMismatch::RepositoryIdentity,
        ),
        (
            result.subject().source_commit() == expected.subject.source_commit(),
            CoverageMismatch::SourceCommit,
        ),
        (
            result.subject().build_id() == expected.subject.build_id(),
            CoverageMismatch::Build,
        ),
        (
            result.subject().artifact_digest() == expected.subject.artifact_digest(),
            CoverageMismatch::Artifact,
        ),
        (
            result.subject().target_configuration_digest()
                == expected.subject.target_configuration_digest(),
            CoverageMismatch::TargetConfiguration,
        ),
        (
            result.target() == &expected.target,
            CoverageMismatch::Target,
        ),
        (result.unit() == &expected.unit, CoverageMismatch::Unit),
        (result.kind() == expected.kind, CoverageMismatch::Kind),
        (
            result.contract_digest() == expected.contract_digest,
            CoverageMismatch::ContractDigest,
        ),
        (
            result.binding_digest() == expected.binding_digest,
            CoverageMismatch::BindingDigest,
        ),
        (
            result.policy_digest() == expected.policy_digest,
            CoverageMismatch::PolicyDigest,
        ),
        (
            result.runner_digest() == expected.runner_digest,
            CoverageMismatch::RunnerDigest,
        ),
        (
            result.adapter_digest() == expected.adapter_digest,
            CoverageMismatch::AdapterDigest,
        ),
        (
            result.runtime_facts_digest() == expected.runtime_facts_digest,
            CoverageMismatch::RuntimeFacts,
        ),
        (
            result.profile_values() == &expected.profiles,
            CoverageMismatch::Profiles,
        ),
        (
            result.release_record_digest() == expected.release_record_digest,
            CoverageMismatch::ReleaseContext,
        ),
    ];
    checks
        .into_iter()
        .filter_map(|(matches, mismatch)| (!matches).then_some(mismatch))
        .collect()
}
