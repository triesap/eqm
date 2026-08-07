//! Stable exhaustive matrix construction for evaluation views.

use crate::ObligationKey;
use eqm_domain::{DiagnosticCode, Facet, TargetId, UnitId};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Closed matrix view family.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MatrixKind {
    /// Intended and observed exposure facts.
    Exposure,
    /// Obligation and target conformance.
    Conformance,
    /// Evidence coverage, outcome, freshness, and trust.
    Evidence,
    /// Exact release-gate state.
    Release,
    /// Required-target-set equivalence.
    Equivalence,
}

/// Typed stable matrix-axis coordinate.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MatrixAxisKey {
    /// Required implementation target.
    Target(TargetId),
    /// Contract or fragment-derived unit.
    Unit(UnitId),
    /// Independently evaluated facet.
    Facet(Facet),
}

/// One matrix row or column descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatrixAxis {
    /// Stable typed coordinate.
    pub key: MatrixAxisKey,
    /// Human-readable bounded label prepared by the caller.
    pub label: Box<str>,
}

/// Closed status vocabulary shared by all matrix views.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MatrixStatus {
    /// The coordinate does not participate.
    NotApplicable,
    /// Every applicable condition passed.
    Pass,
    /// Only visible waived conditions prevent an unconditional pass.
    Conditional,
    /// A terminal condition failed.
    Fail,
    /// Required evidence is stale.
    Stale,
    /// Required input is missing.
    Missing,
    /// Passing and failing immutable history coexist.
    Unstable,
    /// The result cannot be established from trusted complete input.
    Unknown,
}

/// Prepared value for one populated matrix coordinate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatrixValue {
    /// Coordinate status.
    pub status: MatrixStatus,
    /// Contributing obligations.
    pub obligations: BTreeSet<ObligationKey>,
    /// Contributing stable diagnostics.
    pub diagnostics: BTreeSet<DiagnosticCode>,
}

impl Default for MatrixValue {
    fn default() -> Self {
        Self {
            status: MatrixStatus::Unknown,
            obligations: BTreeSet::new(),
            diagnostics: BTreeSet::new(),
        }
    }
}

/// One complete matrix cell.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatrixCell {
    /// Row coordinate.
    pub row: MatrixAxisKey,
    /// Column coordinate.
    pub column: MatrixAxisKey,
    /// Complete value, explicitly unknown when no trusted value was prepared.
    pub value: MatrixValue,
}

/// Complete stable matrix data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix {
    /// View family.
    pub kind: MatrixKind,
    /// Rows in stable typed-key order.
    pub rows: Vec<MatrixAxis>,
    /// Columns in stable typed-key order.
    pub columns: Vec<MatrixAxis>,
    /// Complete row-major Cartesian cell set.
    pub cells: Vec<MatrixCell>,
}

/// Generates a complete stable matrix from explicit required axes and known values.
pub fn generate_matrix(
    kind: MatrixKind,
    rows: BTreeMap<MatrixAxisKey, Box<str>>,
    columns: BTreeMap<MatrixAxisKey, Box<str>>,
    values: BTreeMap<(MatrixAxisKey, MatrixAxisKey), MatrixValue>,
) -> Result<Matrix, MatrixError> {
    if rows.is_empty() || columns.is_empty() {
        return Err(MatrixError::EmptyAxis);
    }
    if rows.values().any(|label| label.is_empty()) || columns.values().any(|label| label.is_empty())
    {
        return Err(MatrixError::EmptyLabel);
    }
    if values
        .keys()
        .any(|(row, column)| !rows.contains_key(row) || !columns.contains_key(column))
    {
        return Err(MatrixError::ValueOutsideAxes);
    }

    let row_axes: Vec<_> = rows
        .iter()
        .map(|(key, label)| MatrixAxis {
            key: key.clone(),
            label: label.clone(),
        })
        .collect();
    let column_axes: Vec<_> = columns
        .iter()
        .map(|(key, label)| MatrixAxis {
            key: key.clone(),
            label: label.clone(),
        })
        .collect();
    let mut cells = Vec::with_capacity(row_axes.len().saturating_mul(column_axes.len()));
    for row in rows.keys() {
        for column in columns.keys() {
            cells.push(MatrixCell {
                row: row.clone(),
                column: column.clone(),
                value: values
                    .get(&(row.clone(), column.clone()))
                    .cloned()
                    .unwrap_or_default(),
            });
        }
    }
    Ok(Matrix {
        kind,
        rows: row_axes,
        columns: column_axes,
        cells,
    })
}

/// Matrix construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatrixError {
    /// A required axis was empty.
    EmptyAxis,
    /// An axis label was empty.
    EmptyLabel,
    /// A value named a coordinate outside the required axes.
    ValueOutsideAxes,
}

impl Display for MatrixError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for MatrixError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn every_view_is_complete_and_stably_row_major() -> Result<(), Box<dyn Error>> {
        let web = MatrixAxisKey::Target(TargetId::new("web")?);
        let ios = MatrixAxisKey::Target(TargetId::new("ios")?);
        let behavior = MatrixAxisKey::Facet(Facet::Behavior);
        let structure = MatrixAxisKey::Facet(Facet::Structure);
        let rows = BTreeMap::from([
            (web.clone(), Box::from("Web")),
            (ios.clone(), Box::from("iOS")),
        ]);
        let columns = BTreeMap::from([
            (structure.clone(), Box::from("Structure")),
            (behavior.clone(), Box::from("Behavior")),
        ]);
        let values = BTreeMap::from([(
            (web.clone(), behavior.clone()),
            MatrixValue {
                status: MatrixStatus::Pass,
                ..MatrixValue::default()
            },
        )]);

        for kind in [
            MatrixKind::Exposure,
            MatrixKind::Conformance,
            MatrixKind::Evidence,
            MatrixKind::Release,
            MatrixKind::Equivalence,
        ] {
            let matrix = generate_matrix(kind, rows.clone(), columns.clone(), values.clone())?;
            assert_eq!(
                matrix.rows.iter().map(|axis| &axis.key).collect::<Vec<_>>(),
                vec![&ios, &web]
            );
            assert_eq!(
                matrix
                    .columns
                    .iter()
                    .map(|axis| &axis.key)
                    .collect::<Vec<_>>(),
                vec![&structure, &behavior]
            );
            assert_eq!(matrix.cells.len(), 4);
            assert_eq!(matrix.cells[0].row, ios);
            assert_eq!(matrix.cells[0].column, structure);
            assert_eq!(matrix.cells[0].value.status, MatrixStatus::Unknown);
            assert_eq!(matrix.cells[2].row, web);
            assert_eq!(matrix.cells[3].row, web);
            assert_eq!(matrix.cells[3].column, behavior);
            assert_eq!(matrix.cells[3].value.status, MatrixStatus::Pass);
        }
        Ok(())
    }

    #[test]
    fn invalid_or_partial_preparation_fails_or_remains_explicit() -> Result<(), Box<dyn Error>> {
        let row = MatrixAxisKey::Unit(UnitId::new("account.create")?);
        let column = MatrixAxisKey::Facet(Facet::Behavior);
        assert_eq!(
            generate_matrix(
                MatrixKind::Conformance,
                BTreeMap::new(),
                BTreeMap::new(),
                BTreeMap::new()
            ),
            Err(MatrixError::EmptyAxis)
        );
        let outside = MatrixAxisKey::Target(TargetId::new("web")?);
        assert_eq!(
            generate_matrix(
                MatrixKind::Conformance,
                BTreeMap::from([(row.clone(), Box::from("Account"))]),
                BTreeMap::from([(column.clone(), Box::from("Behavior"))]),
                BTreeMap::from([((outside, column), MatrixValue::default())]),
            ),
            Err(MatrixError::ValueOutsideAxes)
        );
        Ok(())
    }
}
