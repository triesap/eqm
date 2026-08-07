//! Closed outer DTOs for the complete v1 command-result surface.

#![allow(missing_docs)]

use crate::{CommandIdentity, CommandResultDto, ProfileValueDto, SourceLocationDto};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(
    Clone, Copy, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ResultStatusDto {
    Ok,
    Partial,
    Error,
}

#[derive(
    Clone, Copy, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum FacetStatusDto {
    Satisfied,
    Failed,
    Missing,
    Stale,
    Unknown,
    Unstable,
    Waived,
    NotApplicable,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EntityReferenceDto {
    pub kind: String,
    pub id: String,
    pub revision: Option<u64>,
    pub digest: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocationDto {
    pub role: String,
    pub path: String,
    pub symbol: Option<String>,
    pub source: Option<SourceLocationDto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObligationDto {
    pub id: String,
    pub policy: String,
    pub profile_values: Vec<ProfileValueDto>,
    pub unit: String,
    pub requirement: String,
    pub scope: String,
    pub scope_subject: String,
    pub facet: String,
    pub minimum_trust: String,
    pub maximum_age_ms: u64,
    pub minimum_count: u64,
    pub status: FacetStatusDto,
    pub evidence: BTreeSet<String>,
    pub waiver: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FindingDto {
    pub diagnostic_code: String,
    pub obligation: Option<String>,
    pub status: FacetStatusDto,
    pub evidence: Option<String>,
    pub waiver: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixAxisDto {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixCellDto {
    pub row: String,
    pub column: String,
    pub status: FacetStatusDto,
    pub obligations: BTreeSet<String>,
    pub diagnostic_codes: BTreeSet<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExposureFactDto {
    pub name: String,
    pub value: String,
    pub source: String,
    pub freshness: String,
    pub effective_trust: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExposureComparisonDto {
    pub fact: String,
    pub expected: String,
    pub observed: String,
    pub result: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticChangeDto {
    pub unit: Option<String>,
    pub requirement: Option<String>,
    pub target: Option<String>,
    pub facet: Option<String>,
    pub kind: String,
    pub field: String,
    pub before: Option<Value>,
    pub after: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DoctorCheckDto {
    pub id: String,
    pub status: ResultStatusDto,
    pub message: String,
    pub remediation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileChangeDto {
    pub path: String,
    pub action: String,
    pub before_digest: Option<String>,
    pub after_digest: Option<String>,
}

macro_rules! mutation_result {
    ($name:ident, $command:ident) => {
        #[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
        #[serde(deny_unknown_fields)]
        pub struct $name {
            pub kind: CommandIdentity,
            pub dry_run: bool,
            pub changes: BTreeSet<FileChangeDto>,
            pub written: BTreeSet<String>,
        }
        impl CommandResultDto for $name {
            fn command(&self) -> CommandIdentity {
                CommandIdentity::$command
            }
            fn declared_command(&self) -> CommandIdentity {
                self.kind
            }
        }
    };
}

mutation_result!(InitResultDto, Init);
mutation_result!(NewResultDto, New);
mutation_result!(FmtResultDto, Fmt);
mutation_result!(LockUpdateResultDto, LockUpdate);

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidateResultDto {
    pub kind: CommandIdentity,
    pub valid: bool,
    pub entity_counts: BTreeMap<String, u64>,
    pub graph_digest: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckResultDto {
    pub kind: CommandIdentity,
    pub status: ResultStatusDto,
    pub obligation_counts: BTreeMap<FacetStatusDto, u64>,
    pub findings: BTreeSet<FindingDto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShowResultDto<E> {
    pub kind: CommandIdentity,
    pub entity_kind: String,
    pub entity_id: String,
    pub source: SourceLocationDto,
    pub entity: E,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocateResultDto {
    pub kind: CommandIdentity,
    pub unit: String,
    pub target: Option<String>,
    pub locations: BTreeSet<LocationDto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextResultDto<A, P, E, W> {
    pub kind: CommandIdentity,
    pub unit: String,
    pub target: Option<String>,
    pub authority: A,
    pub product_data: P,
    pub obligations: BTreeSet<ObligationDto>,
    pub evidence: E,
    pub findings: BTreeSet<FindingDto>,
    pub waivers: W,
    pub truncated: bool,
    pub omitted_bytes: u64,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixResultDto {
    pub kind: CommandIdentity,
    pub matrix_kind: String,
    pub rows: BTreeSet<MatrixAxisDto>,
    pub columns: BTreeSet<MatrixAxisDto>,
    pub cells: BTreeSet<MatrixCellDto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObligationsResultDto {
    pub kind: CommandIdentity,
    pub filters: BTreeMap<String, String>,
    pub obligations: BTreeSet<ObligationDto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiffResultDto {
    pub kind: CommandIdentity,
    pub baseline_digest: String,
    pub candidate_digest: String,
    pub changes: Vec<SemanticChangeDto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AffectedResultDto {
    pub kind: CommandIdentity,
    pub baseline_digest: String,
    pub changed_paths: BTreeSet<String>,
    pub units: BTreeSet<String>,
    pub obligations: BTreeSet<String>,
    pub conservative: bool,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiscoverResultDto<I> {
    pub kind: CommandIdentity,
    pub adapter: String,
    pub target: String,
    pub inventory: I,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReconcileResultDto {
    pub kind: CommandIdentity,
    pub target: String,
    pub unit: Option<String>,
    pub facts: BTreeSet<ExposureFactDto>,
    pub comparisons: BTreeSet<ExposureComparisonDto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerifyResultDto<S, E> {
    pub kind: CommandIdentity,
    pub selection: S,
    pub evidence_results: E,
    pub summary: BTreeMap<FacetStatusDto, u64>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttestResultDto<S> {
    pub kind: CommandIdentity,
    pub statement: S,
    pub signed: bool,
    pub signer: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseCheckResultDto<S, E> {
    pub kind: CommandIdentity,
    pub subject: S,
    pub status: String,
    pub conformance: String,
    pub equivalence: String,
    pub exposure: E,
    pub waivers: BTreeSet<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExplainResultDto {
    pub kind: CommandIdentity,
    pub code: String,
    pub title: String,
    pub authority: String,
    pub explanation: String,
    pub remediation: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DoctorResultDto {
    pub kind: CommandIdentity,
    pub checks: BTreeSet<DoctorCheckDto>,
    pub status: ResultStatusDto,
}

macro_rules! fixed_result {
    ($type:ty, $command:ident) => {
        impl CommandResultDto for $type {
            fn command(&self) -> CommandIdentity {
                CommandIdentity::$command
            }
            fn declared_command(&self) -> CommandIdentity {
                self.kind
            }
        }
    };
}

fixed_result!(ValidateResultDto, Validate);
fixed_result!(CheckResultDto, Check);
fixed_result!(LocateResultDto, Locate);
fixed_result!(MatrixResultDto, Matrix);
fixed_result!(ObligationsResultDto, Obligations);
fixed_result!(DiffResultDto, Diff);
fixed_result!(AffectedResultDto, Affected);
fixed_result!(ReconcileResultDto, Reconcile);
fixed_result!(ExplainResultDto, Explain);
fixed_result!(DoctorResultDto, Doctor);

impl<E> CommandResultDto for ShowResultDto<E> {
    fn command(&self) -> CommandIdentity {
        CommandIdentity::Show
    }
    fn declared_command(&self) -> CommandIdentity {
        self.kind
    }
}
impl<A, P, E, W> CommandResultDto for ContextResultDto<A, P, E, W> {
    fn command(&self) -> CommandIdentity {
        CommandIdentity::Context
    }
    fn declared_command(&self) -> CommandIdentity {
        self.kind
    }
}
impl<I> CommandResultDto for DiscoverResultDto<I> {
    fn command(&self) -> CommandIdentity {
        CommandIdentity::Discover
    }
    fn declared_command(&self) -> CommandIdentity {
        self.kind
    }
}
impl<S, E> CommandResultDto for VerifyResultDto<S, E> {
    fn command(&self) -> CommandIdentity {
        CommandIdentity::Verify
    }
    fn declared_command(&self) -> CommandIdentity {
        self.kind
    }
}
impl<S> CommandResultDto for AttestResultDto<S> {
    fn command(&self) -> CommandIdentity {
        CommandIdentity::Attest
    }
    fn declared_command(&self) -> CommandIdentity {
        self.kind
    }
}
impl<S, E> CommandResultDto for ReleaseCheckResultDto<S, E> {
    fn command(&self) -> CommandIdentity {
        CommandIdentity::ReleaseCheck
    }
    fn declared_command(&self) -> CommandIdentity {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_records_emit_independent_of_insertion_order() -> Result<(), serde_json::Error> {
        let make = |items: [&str; 2]| DoctorResultDto {
            kind: CommandIdentity::Doctor,
            checks: items
                .into_iter()
                .map(|id| DoctorCheckDto {
                    id: id.to_owned(),
                    status: ResultStatusDto::Ok,
                    message: "healthy".to_owned(),
                    remediation: None,
                })
                .collect(),
            status: ResultStatusDto::Ok,
        };
        assert_eq!(
            serde_json::to_vec(&make(["zeta", "alpha"]))?,
            serde_json::to_vec(&make(["alpha", "zeta"]))?
        );
        Ok(())
    }

    #[test]
    fn result_shapes_reject_unknown_fields() {
        assert!(
            serde_json::from_str::<DoctorResultDto>(
                r#"{"kind":"doctor","checks":[],"status":"ok","extra":true}"#
            )
            .is_err()
        );
        assert!(
            serde_json::from_str::<ValidateResultDto>(
                r#"{"kind":"validate","valid":true,"entity_counts":{},"graph_digest":"sha256:bad"}"#
            )
            .is_ok()
        );
    }
}
