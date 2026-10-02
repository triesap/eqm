//! Acyclic evaluation-record dependency identities (R20, AC020).
//!
//! The approved lifecycle is
//! `authority/candidate -> plan/campaign -> evidence/worker/runtime ->
//! frozen-input -> decision -> statement -> authorization -> deployment-receipt`.
//! References point to records that are already defined: a record may not
//! reference itself, a later lifecycle row, a missing record or form a cycle.
//! A checked [`ValidatedRecords`] is the only way to obtain a registry whose
//! directed references are realizable; it implies no storage or publication.

use crate::id::RecordId;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// One kind of record in the approved acyclic evaluation lifecycle.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RecordKind {
    /// Protected authority or approved baseline.
    Authority,
    /// Candidate subject manifest.
    Candidate,
    /// Approved acquisition plan.
    Plan,
    /// Authorized campaign.
    Campaign,
    /// Evidence assertion for a plan/campaign.
    Evidence,
    /// Worker receipt accompanying execution.
    Worker,
    /// Available runtime observation.
    Runtime,
    /// Frozen input manifest assembled for a decision.
    FrozenInput,
    /// Evaluation decision.
    Decision,
    /// Optional signed statement over a decision.
    Statement,
    /// Protected release authorization.
    Authorization,
    /// Deployment receipt or post-deployment observation.
    DeploymentReceipt,
}

impl RecordKind {
    /// Every lifecycle kind in dependency order.
    pub const ALL: [Self; 12] = [
        Self::Authority,
        Self::Candidate,
        Self::Plan,
        Self::Campaign,
        Self::Evidence,
        Self::Worker,
        Self::Runtime,
        Self::FrozenInput,
        Self::Decision,
        Self::Statement,
        Self::Authorization,
        Self::DeploymentReceipt,
    ];

    /// Returns this kind's lifecycle row. A reference may not point to a later
    /// row; evidence, worker and runtime observations share one row.
    #[must_use]
    pub const fn order(self) -> u8 {
        match self {
            Self::Authority => 0,
            Self::Candidate => 1,
            Self::Plan => 2,
            Self::Campaign => 3,
            Self::Evidence | Self::Worker | Self::Runtime => 4,
            Self::FrozenInput => 5,
            Self::Decision => 6,
            Self::Statement => 7,
            Self::Authorization => 8,
            Self::DeploymentReceipt => 9,
        }
    }

    /// Returns the exact canonical wire token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authority => "authority",
            Self::Candidate => "candidate",
            Self::Plan => "plan",
            Self::Campaign => "campaign",
            Self::Evidence => "evidence",
            Self::Worker => "worker",
            Self::Runtime => "runtime",
            Self::FrozenInput => "frozen-input",
            Self::Decision => "decision",
            Self::Statement => "statement",
            Self::Authorization => "authorization",
            Self::DeploymentReceipt => "deployment-receipt",
        }
    }
}

impl Display for RecordKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RecordKind {
    type Err = RecordError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|kind| kind.as_str() == value)
            .ok_or(RecordError::UnknownKind)
    }
}

/// One typed directed reference to an already-defined record.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RecordRef {
    kind: RecordKind,
    id: RecordId,
}

impl RecordRef {
    /// Creates a reference to a record of the given kind and identity.
    #[must_use]
    pub const fn new(kind: RecordKind, id: RecordId) -> Self {
        Self { kind, id }
    }

    /// Returns the referenced record kind.
    #[must_use]
    pub const fn kind(&self) -> RecordKind {
        self.kind
    }

    /// Returns the referenced record identity.
    #[must_use]
    pub const fn id(&self) -> &RecordId {
        &self.id
    }
}

impl Display for RecordRef {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.kind, self.id)
    }
}

/// A record with its declared, already-typed references.
///
/// Fields are private so an unchecked deserialization cannot promote raw data
/// into a validated record without going through [`RecordDescriptor::new`] and
/// [`validate_records`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordDescriptor {
    kind: RecordKind,
    id: RecordId,
    references: Vec<RecordRef>,
}

impl RecordDescriptor {
    /// Builds one record, rejecting a self-reference or a repeated reference.
    pub fn new(
        kind: RecordKind,
        id: RecordId,
        references: Vec<RecordRef>,
    ) -> Result<Self, RecordError> {
        let mut seen = BTreeSet::new();
        for reference in &references {
            if reference.kind == kind && reference.id == id {
                return Err(RecordError::SelfReference);
            }
            if !seen.insert(reference.clone()) {
                return Err(RecordError::DuplicateReference);
            }
        }
        Ok(Self {
            kind,
            id,
            references,
        })
    }

    /// Returns the record kind.
    #[must_use]
    pub const fn kind(&self) -> RecordKind {
        self.kind
    }

    /// Returns the record identity.
    #[must_use]
    pub const fn id(&self) -> &RecordId {
        &self.id
    }

    /// Returns the declared references in canonical order.
    #[must_use]
    pub fn references(&self) -> &[RecordRef] {
        &self.references
    }
}

/// A checked registry of records whose directed references are realizable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedRecords {
    records: Vec<RecordDescriptor>,
}

impl ValidatedRecords {
    /// Returns the validated records in supplied order.
    #[must_use]
    pub fn records(&self) -> &[RecordDescriptor] {
        &self.records
    }
}

type Node = (RecordKind, RecordId);

/// Validates one explicit registry against the acyclic lifecycle.
///
/// Rejects duplicate records, missing references, a later-row (future)
/// reference and any directed cycle. The checked result does not claim storage,
/// publication or trust.
pub fn validate_records(records: Vec<RecordDescriptor>) -> Result<ValidatedRecords, RecordError> {
    let mut known = BTreeSet::new();
    for record in &records {
        if !known.insert((record.kind, record.id.clone())) {
            return Err(RecordError::DuplicateRecord);
        }
    }

    let mut adjacency: BTreeMap<Node, Vec<Node>> = BTreeMap::new();
    for record in &records {
        for reference in &record.references {
            let target = (reference.kind, reference.id.clone());
            if !known.contains(&target) {
                return Err(RecordError::MissingReference);
            }
            if reference.kind.order() > record.kind.order() {
                return Err(RecordError::FutureReference);
            }
        }
        adjacency.insert(
            (record.kind, record.id.clone()),
            record
                .references
                .iter()
                .map(|reference| (reference.kind, reference.id.clone()))
                .collect(),
        );
    }

    // Iterative depth-first cycle detection with explicit colors.
    let mut state: BTreeMap<Node, u8> = BTreeMap::new();
    for root in adjacency.keys() {
        if state.get(root).copied().unwrap_or(0) != 0 {
            continue;
        }
        state.insert(root.clone(), 1);
        let mut stack: Vec<(Node, usize)> = vec![(root.clone(), 0)];
        while let Some((node, index)) = stack.last().cloned() {
            let neighbours = &adjacency[&node];
            if index < neighbours.len() {
                if let Some(frame) = stack.last_mut() {
                    frame.1 += 1;
                }
                let next = neighbours[index].clone();
                match state.get(&next).copied().unwrap_or(0) {
                    0 => {
                        state.insert(next.clone(), 1);
                        stack.push((next, 0));
                    }
                    1 => return Err(RecordError::CyclicReference),
                    _ => {}
                }
            } else {
                state.insert(node, 2);
                stack.pop();
            }
        }
    }

    Ok(ValidatedRecords { records })
}

/// Record dependency validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordError {
    /// The record kind token was not an approved lifecycle kind.
    UnknownKind,
    /// Two records shared one kind and identity.
    DuplicateRecord,
    /// One record declared the same reference twice.
    DuplicateReference,
    /// A record referenced itself.
    SelfReference,
    /// A reference named a record absent from the supplied registry.
    MissingReference,
    /// A reference pointed to a later lifecycle row.
    FutureReference,
    /// The directed reference graph contained a cycle.
    CyclicReference,
}

impl Display for RecordError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::UnknownKind => "unknown record kind",
            Self::DuplicateRecord => "duplicate record",
            Self::DuplicateReference => "duplicate record reference",
            Self::SelfReference => "record references itself",
            Self::MissingReference => "record reference is missing",
            Self::FutureReference => "record reference points to a later row",
            Self::CyclicReference => "record references form a cycle",
        })
    }
}

impl Error for RecordError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    fn id(value: &str) -> Result<RecordId, Box<dyn Error>> {
        Ok(RecordId::new(value)?)
    }

    fn reference(kind: RecordKind, name: &str) -> Result<RecordRef, Box<dyn Error>> {
        Ok(RecordRef::new(kind, id(name)?))
    }

    fn record(
        kind: RecordKind,
        name: &str,
        references: Vec<RecordRef>,
    ) -> Result<RecordDescriptor, Box<dyn Error>> {
        Ok(RecordDescriptor::new(kind, id(name)?, references)?)
    }

    #[test]
    fn kinds_round_trip_and_are_ordered() {
        for kind in RecordKind::ALL {
            assert_eq!(kind.as_str().parse::<RecordKind>(), Ok(kind));
        }
        assert!(RecordKind::Authority.order() < RecordKind::Candidate.order());
        assert!(RecordKind::Campaign.order() < RecordKind::Evidence.order());
        assert_eq!(
            RecordKind::Evidence.order(),
            RecordKind::Worker.order(),
            "evidence, worker and runtime inputs share one row"
        );
        assert_eq!(RecordKind::Runtime.order(), RecordKind::Worker.order());
        assert!(RecordKind::Authorization.order() < RecordKind::DeploymentReceipt.order());
        assert_eq!(
            "not-a-kind".parse::<RecordKind>(),
            Err(RecordError::UnknownKind)
        );
    }

    #[test]
    fn candidate_to_evidence_to_decision_chain_validates() -> Result<(), Box<dyn Error>> {
        let records = vec![
            record(RecordKind::Authority, "authority.baseline", vec![])?,
            record(
                RecordKind::Candidate,
                "candidate.one",
                vec![reference(RecordKind::Authority, "authority.baseline")?],
            )?,
            record(
                RecordKind::Plan,
                "plan.release",
                vec![
                    reference(RecordKind::Authority, "authority.baseline")?,
                    reference(RecordKind::Candidate, "candidate.one")?,
                ],
            )?,
            record(
                RecordKind::Campaign,
                "campaign.signup",
                vec![reference(RecordKind::Plan, "plan.release")?],
            )?,
            record(
                RecordKind::Evidence,
                "evidence.one",
                vec![
                    reference(RecordKind::Candidate, "candidate.one")?,
                    reference(RecordKind::Plan, "plan.release")?,
                    reference(RecordKind::Campaign, "campaign.signup")?,
                ],
            )?,
            record(
                RecordKind::Worker,
                "worker.one",
                vec![reference(RecordKind::Campaign, "campaign.signup")?],
            )?,
            record(
                RecordKind::Runtime,
                "runtime.one",
                vec![reference(RecordKind::Campaign, "campaign.signup")?],
            )?,
            record(
                RecordKind::FrozenInput,
                "frozen.one",
                vec![
                    reference(RecordKind::Evidence, "evidence.one")?,
                    reference(RecordKind::Worker, "worker.one")?,
                    reference(RecordKind::Runtime, "runtime.one")?,
                ],
            )?,
            record(
                RecordKind::Decision,
                "decision.one",
                vec![reference(RecordKind::FrozenInput, "frozen.one")?],
            )?,
            record(
                RecordKind::Statement,
                "statement.one",
                vec![reference(RecordKind::Decision, "decision.one")?],
            )?,
            record(
                RecordKind::Authorization,
                "authorization.one",
                vec![
                    reference(RecordKind::Decision, "decision.one")?,
                    reference(RecordKind::Statement, "statement.one")?,
                ],
            )?,
            record(
                RecordKind::DeploymentReceipt,
                "deployment.one",
                vec![reference(RecordKind::Authorization, "authorization.one")?],
            )?,
        ];
        let validated = validate_records(records)?;
        assert_eq!(validated.records().len(), 12);
        Ok(())
    }

    #[test]
    fn missing_reference_rejects() -> Result<(), Box<dyn Error>> {
        let records = vec![record(
            RecordKind::Evidence,
            "evidence.one",
            vec![reference(RecordKind::Plan, "plan.absent")?],
        )?];
        assert_eq!(
            validate_records(records),
            Err(RecordError::MissingReference)
        );
        Ok(())
    }

    #[test]
    fn self_reference_rejects() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            RecordDescriptor::new(
                RecordKind::Plan,
                id("plan.release")?,
                vec![reference(RecordKind::Plan, "plan.release")?],
            ),
            Err(RecordError::SelfReference)
        );
        Ok(())
    }

    #[test]
    fn future_reference_rejects() -> Result<(), Box<dyn Error>> {
        let records = vec![
            record(
                RecordKind::Evidence,
                "evidence.one",
                vec![reference(RecordKind::Decision, "decision.one")?],
            )?,
            record(
                RecordKind::FrozenInput,
                "frozen.one",
                vec![reference(RecordKind::Evidence, "evidence.one")?],
            )?,
            record(
                RecordKind::Decision,
                "decision.one",
                vec![reference(RecordKind::FrozenInput, "frozen.one")?],
            )?,
        ];
        assert_eq!(validate_records(records), Err(RecordError::FutureReference));
        Ok(())
    }

    #[test]
    fn duplicate_record_and_reference_reject() -> Result<(), Box<dyn Error>> {
        let records = vec![
            record(RecordKind::Plan, "plan.release", vec![])?,
            record(RecordKind::Plan, "plan.release", vec![])?,
        ];
        assert_eq!(validate_records(records), Err(RecordError::DuplicateRecord));
        assert_eq!(
            RecordDescriptor::new(
                RecordKind::Campaign,
                id("campaign.signup")?,
                vec![
                    reference(RecordKind::Plan, "plan.release")?,
                    reference(RecordKind::Plan, "plan.release")?,
                ],
            ),
            Err(RecordError::DuplicateReference)
        );
        Ok(())
    }

    #[test]
    fn cyclic_same_row_references_reject() -> Result<(), Box<dyn Error>> {
        let records = vec![
            record(
                RecordKind::Plan,
                "plan.a",
                vec![reference(RecordKind::Plan, "plan.b")?],
            )?,
            record(
                RecordKind::Plan,
                "plan.b",
                vec![reference(RecordKind::Plan, "plan.a")?],
            )?,
        ];
        assert_eq!(validate_records(records), Err(RecordError::CyclicReference));
        Ok(())
    }
}
