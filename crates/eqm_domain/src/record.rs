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
use std::collections::BTreeMap;
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
    /// row, except for the approved historical deployment inputs; evidence,
    /// worker and runtime observations share one row.
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
    ///
    /// The declared references are stored sorted by their complete typed
    /// `(kind, id)` identity, so permuting the supplied member order is
    /// equivalent and [`RecordDescriptor::references`] is canonical.
    pub fn new(
        kind: RecordKind,
        id: RecordId,
        mut references: Vec<RecordRef>,
    ) -> Result<Self, RecordError> {
        for reference in &references {
            if reference.kind == kind && reference.id == id {
                return Err(RecordError::SelfReference);
            }
        }
        references.sort();
        if references.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(RecordError::DuplicateReference);
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

    /// Returns the declared references in canonical complete-typed order.
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

/// Returns whether a reference from `source` to `target` respects the approved
/// lifecycle direction. Runtime and FrozenInput inputs may consume an
/// already-defined DeploymentReceipt; every other reference may only point to
/// its own or an earlier lifecycle row. Evidence, worker and runtime share a row.
const fn direction_allowed(source: RecordKind, target: RecordKind) -> bool {
    target.order() <= source.order()
        || matches!(
            (source, target),
            (RecordKind::Runtime, RecordKind::DeploymentReceipt)
                | (RecordKind::FrozenInput, RecordKind::DeploymentReceipt)
        )
}

/// Validates one explicit registry against the acyclic lifecycle.
///
/// The supplied `Vec` is the caller-declared **definition history**: its order
/// is creation order, not a timestamp/path/kind sort, and it is preserved in
/// the checked result. A checked record may reference only a definition that
/// appears earlier in that history, so no reference points at a future
/// definition.
///
/// Rejects duplicate records, missing or wrong-kind references, later-row
/// (future) references, forbidden same-kind forward definitions and any
/// directed cycle. Runtime and FrozenInput inputs may consume an
/// already-defined DeploymentReceipt; initial readiness may not wait on its own
/// future deployment. The checked result does not claim storage, publication or
/// trust.
pub fn validate_records(records: Vec<RecordDescriptor>) -> Result<ValidatedRecords, RecordError> {
    let mut known: BTreeMap<Node, usize> = BTreeMap::new();
    for (index, record) in records.iter().enumerate() {
        if known
            .insert((record.kind, record.id.clone()), index)
            .is_some()
        {
            return Err(RecordError::DuplicateRecord);
        }
    }

    // Every reference must name a supplied definition before the graph walk.
    for record in &records {
        for reference in &record.references {
            if !known.contains_key(&(reference.kind, reference.id.clone())) {
                return Err(RecordError::MissingReference);
            }
        }
    }

    let mut adjacency: BTreeMap<Node, Vec<Node>> = BTreeMap::new();
    for record in &records {
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

    // Each reference must resolve to an earlier definition and respect the
    // lifecycle direction (with the two approved historical deployment inputs).
    for (index, record) in records.iter().enumerate() {
        for reference in &record.references {
            let target = (reference.kind, reference.id.clone());
            if known.get(&target).is_some_and(|at| *at >= index) {
                return Err(RecordError::FutureReference);
            }
            if !direction_allowed(record.kind, reference.kind) {
                return Err(RecordError::FutureReference);
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
    /// A reference pointed to a later definition or a later lifecycle row.
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

    fn initial_lifecycle() -> Result<Vec<RecordDescriptor>, Box<dyn Error>> {
        Ok(vec![
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
        ])
    }

    #[test]
    fn candidate_to_evidence_to_decision_chain_validates() -> Result<(), Box<dyn Error>> {
        let validated = validate_records(initial_lifecycle()?)?;
        assert_eq!(validated.records().len(), 12);
        Ok(())
    }

    #[test]
    fn historical_deployment_inputs_validate_after_an_initial_lifecycle()
    -> Result<(), Box<dyn Error>> {
        // A full initial lifecycle, then a post-deployment observation consumed
        // by a later runtime input, a later frozen input and a later decision.
        let mut records = initial_lifecycle()?;
        records.push(record(
            RecordKind::Runtime,
            "runtime.post_deploy",
            vec![reference(RecordKind::DeploymentReceipt, "deployment.one")?],
        )?);
        records.push(record(
            RecordKind::FrozenInput,
            "frozen.post_deploy",
            vec![
                reference(RecordKind::Runtime, "runtime.post_deploy")?,
                reference(RecordKind::DeploymentReceipt, "deployment.one")?,
            ],
        )?);
        records.push(record(
            RecordKind::Decision,
            "decision.post_deploy",
            vec![reference(RecordKind::FrozenInput, "frozen.post_deploy")?],
        )?);
        let validated = validate_records(records)?;
        assert_eq!(validated.records().len(), 15);
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
    fn wrong_kind_reference_rejects() -> Result<(), Box<dyn Error>> {
        // A candidate exists, but the reference names it as a plan.
        let records = vec![
            record(RecordKind::Candidate, "candidate.one", vec![])?,
            record(
                RecordKind::Plan,
                "plan.release",
                vec![reference(RecordKind::Plan, "candidate.one")?],
            )?,
        ];
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
        // Evidence may not reference its future decision, even when the
        // decision is present and the graph is acyclic.
        let records = vec![
            record(
                RecordKind::Evidence,
                "evidence.one",
                vec![reference(RecordKind::Decision, "decision.one")?],
            )?,
            record(RecordKind::Decision, "decision.one", vec![])?,
        ];
        assert_eq!(validate_records(records), Err(RecordError::FutureReference));
        Ok(())
    }

    #[test]
    fn same_kind_forward_definition_rejects() -> Result<(), Box<dyn Error>> {
        let records = vec![
            record(
                RecordKind::Plan,
                "plan.a",
                vec![reference(RecordKind::Plan, "plan.b")?],
            )?,
            record(RecordKind::Plan, "plan.b", vec![])?,
        ];
        assert_eq!(validate_records(records), Err(RecordError::FutureReference));
        Ok(())
    }

    #[test]
    fn future_receipt_rejects() -> Result<(), Box<dyn Error>> {
        // A later runtime may consume an earlier receipt, but not one that is
        // defined after it.
        let records = vec![
            record(
                RecordKind::Runtime,
                "runtime.one",
                vec![reference(
                    RecordKind::DeploymentReceipt,
                    "deployment.later",
                )?],
            )?,
            record(RecordKind::DeploymentReceipt, "deployment.later", vec![])?,
        ];
        assert_eq!(validate_records(records), Err(RecordError::FutureReference));
        Ok(())
    }

    #[test]
    fn initial_readiness_on_a_future_deployment_rejects() -> Result<(), Box<dyn Error>> {
        // Decision may never reference a deployment receipt, even one defined
        // earlier: initial readiness cannot wait on its own deployment.
        let records = vec![
            record(RecordKind::DeploymentReceipt, "deployment.one", vec![])?,
            record(
                RecordKind::Decision,
                "decision.one",
                vec![reference(RecordKind::DeploymentReceipt, "deployment.one")?],
            )?,
        ];
        assert_eq!(validate_records(records), Err(RecordError::FutureReference));
        Ok(())
    }

    #[test]
    fn evidence_to_decision_remains_forbidden() -> Result<(), Box<dyn Error>> {
        let records = vec![
            record(RecordKind::Decision, "decision.one", vec![])?,
            record(
                RecordKind::Evidence,
                "evidence.one",
                vec![reference(RecordKind::Decision, "decision.one")?],
            )?,
        ];
        assert_eq!(validate_records(records), Err(RecordError::FutureReference));
        Ok(())
    }

    #[test]
    fn cyclic_history_rejects() -> Result<(), Box<dyn Error>> {
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

    #[test]
    fn initial_readiness_deployment_cycle_rejects() -> Result<(), Box<dyn Error>> {
        // A frozen input waiting on a receipt that waits back on the frozen
        // input is a directed cycle, not a realizable readiness history.
        let records = vec![
            record(
                RecordKind::FrozenInput,
                "frozen.one",
                vec![reference(RecordKind::DeploymentReceipt, "deployment.one")?],
            )?,
            record(
                RecordKind::DeploymentReceipt,
                "deployment.one",
                vec![reference(RecordKind::FrozenInput, "frozen.one")?],
            )?,
        ];
        assert_eq!(validate_records(records), Err(RecordError::CyclicReference));
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
    fn reordered_definitions_into_forward_references_reject() -> Result<(), Box<dyn Error>> {
        // The same records validate in definition order...
        let ordered = vec![
            record(RecordKind::Plan, "plan.a", vec![])?,
            record(
                RecordKind::Plan,
                "plan.b",
                vec![reference(RecordKind::Plan, "plan.a")?],
            )?,
        ];
        assert!(validate_records(ordered).is_ok());
        // ... but moving the referenced definition later makes it a forward
        // reference and must reject. History is never sorted to make it pass.
        let reordered = vec![
            record(
                RecordKind::Plan,
                "plan.b",
                vec![reference(RecordKind::Plan, "plan.a")?],
            )?,
            record(RecordKind::Plan, "plan.a", vec![])?,
        ];
        assert_eq!(
            validate_records(reordered),
            Err(RecordError::FutureReference)
        );
        Ok(())
    }

    #[test]
    fn permuted_reference_member_supply_is_equivalent() -> Result<(), Box<dyn Error>> {
        let first = RecordDescriptor::new(
            RecordKind::Campaign,
            id("campaign.signup")?,
            vec![
                reference(RecordKind::Plan, "plan.release")?,
                reference(RecordKind::Authority, "authority.baseline")?,
            ],
        )?;
        let second = RecordDescriptor::new(
            RecordKind::Campaign,
            id("campaign.signup")?,
            vec![
                reference(RecordKind::Authority, "authority.baseline")?,
                reference(RecordKind::Plan, "plan.release")?,
            ],
        )?;
        assert_eq!(first, second);
        assert_eq!(first.references(), second.references());
        Ok(())
    }

    #[test]
    fn references_are_stored_in_canonical_complete_typed_order() -> Result<(), Box<dyn Error>> {
        let descriptor = RecordDescriptor::new(
            RecordKind::Authorization,
            id("authorization.one")?,
            vec![
                reference(RecordKind::Statement, "statement.one")?,
                reference(RecordKind::Decision, "decision.one")?,
                reference(RecordKind::Evidence, "evidence.one")?,
            ],
        )?;
        let kinds: Vec<RecordKind> = descriptor
            .references()
            .iter()
            .map(RecordRef::kind)
            .collect();
        // Complete typed order: kind first, then identity.
        assert_eq!(
            kinds,
            vec![
                RecordKind::Evidence,
                RecordKind::Decision,
                RecordKind::Statement
            ]
        );
        assert!(
            descriptor
                .references()
                .windows(2)
                .all(|pair| pair[0] < pair[1])
        );
        Ok(())
    }
}
