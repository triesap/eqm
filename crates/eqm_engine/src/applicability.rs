//! Total three-valued evaluation over declared finite profile dimensions.

use eqm_domain::{
    Applicability, ApplicabilityView, ComparisonOperator, DimensionId, MembershipOperator, Profile,
    SymbolicValueId,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Total applicability truth value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TruthValue {
    /// The expression is satisfied.
    True,
    /// The expression is not satisfied.
    False,
    /// A required declared dimension has no known selected value.
    Unknown,
}

impl TruthValue {
    const fn logical_not(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Unknown => Self::Unknown,
        }
    }

    const fn logical_all(self, other: Self) -> Self {
        match (self, other) {
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::True, Self::True) => Self::True,
            _ => Self::Unknown,
        }
    }

    const fn logical_any(self, other: Self) -> Self {
        match (self, other) {
            (Self::True, _) | (_, Self::True) => Self::True,
            (Self::False, Self::False) => Self::False,
            _ => Self::Unknown,
        }
    }
}

/// Validated finite declarations and explicit known/unknown selected values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicabilityContext {
    declarations: BTreeMap<DimensionId, BTreeSet<SymbolicValueId>>,
    values: BTreeMap<DimensionId, Option<SymbolicValueId>>,
}

impl ApplicabilityContext {
    /// Creates a context from one exact profile and explicit selected values.
    ///
    /// Omitted dimensions and entries with `None` both evaluate as unknown.
    pub fn new(
        profile: &Profile,
        values: BTreeMap<DimensionId, Option<SymbolicValueId>>,
    ) -> Result<Self, ApplicabilityError> {
        let declarations: BTreeMap<_, _> = profile
            .dimensions()
            .iter()
            .map(|(id, dimension)| (id.clone(), dimension.values().clone()))
            .collect();
        for (dimension, value) in &values {
            let allowed = declarations
                .get(dimension)
                .ok_or(ApplicabilityError::UndeclaredDimension)?;
            if value.as_ref().is_some_and(|value| !allowed.contains(value)) {
                return Err(ApplicabilityError::UndeclaredValue);
            }
        }
        Ok(Self {
            declarations,
            values,
        })
    }

    fn declaration(
        &self,
        dimension: &DimensionId,
    ) -> Result<&BTreeSet<SymbolicValueId>, ApplicabilityError> {
        self.declarations
            .get(dimension)
            .ok_or(ApplicabilityError::UndeclaredDimension)
    }

    fn value(&self, dimension: &DimensionId) -> Option<&SymbolicValueId> {
        self.values.get(dimension).and_then(Option::as_ref)
    }
}

/// Validates and evaluates one applicability expression without short-circuiting validation.
pub fn evaluate_applicability(
    expression: &Applicability,
    context: &ApplicabilityContext,
) -> Result<TruthValue, ApplicabilityError> {
    validate_expression(expression, context)?;
    Ok(evaluate_validated(expression, context))
}

fn validate_expression(
    expression: &Applicability,
    context: &ApplicabilityContext,
) -> Result<(), ApplicabilityError> {
    match expression.view() {
        ApplicabilityView::Constant(_) => Ok(()),
        ApplicabilityView::Comparison(dimension, _, value) => {
            if context.declaration(dimension)?.contains(value) {
                Ok(())
            } else {
                Err(ApplicabilityError::UndeclaredValue)
            }
        }
        ApplicabilityView::Membership(dimension, _, values) => {
            let declared = context.declaration(dimension)?;
            if values.iter().all(|value| declared.contains(value)) {
                Ok(())
            } else {
                Err(ApplicabilityError::UndeclaredValue)
            }
        }
        ApplicabilityView::All(values) | ApplicabilityView::Any(values) => {
            for value in values {
                validate_expression(value, context)?;
            }
            Ok(())
        }
        ApplicabilityView::Not(value) => validate_expression(value, context),
    }
}

fn evaluate_validated(expression: &Applicability, context: &ApplicabilityContext) -> TruthValue {
    match expression.view() {
        ApplicabilityView::Constant(value) => {
            if value {
                TruthValue::True
            } else {
                TruthValue::False
            }
        }
        ApplicabilityView::Comparison(dimension, operator, expected) => {
            let Some(actual) = context.value(dimension) else {
                return TruthValue::Unknown;
            };
            let equal = actual == expected;
            let satisfied = match operator {
                ComparisonOperator::Equal => equal,
                ComparisonOperator::NotEqual => !equal,
            };
            if satisfied {
                TruthValue::True
            } else {
                TruthValue::False
            }
        }
        ApplicabilityView::Membership(dimension, operator, expected) => {
            let Some(actual) = context.value(dimension) else {
                return TruthValue::Unknown;
            };
            let included = expected.contains(actual);
            let satisfied = match operator {
                MembershipOperator::In => included,
                MembershipOperator::NotIn => !included,
            };
            if satisfied {
                TruthValue::True
            } else {
                TruthValue::False
            }
        }
        ApplicabilityView::All(values) => values.iter().fold(TruthValue::True, |result, value| {
            result.logical_all(evaluate_validated(value, context))
        }),
        ApplicabilityView::Any(values) => values.iter().fold(TruthValue::False, |result, value| {
            result.logical_any(evaluate_validated(value, context))
        }),
        ApplicabilityView::Not(value) => evaluate_validated(value, context).logical_not(),
    }
}

/// Applicability declaration or selection failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicabilityError {
    /// An expression or selected value named a dimension outside the profile.
    UndeclaredDimension,
    /// An expression or selected value named a value outside its finite declaration.
    UndeclaredValue,
}

impl Display for ApplicabilityError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::UndeclaredDimension => "applicability dimension is not declared",
            Self::UndeclaredValue => "applicability value is not declared",
        })
    }
}

impl Error for ApplicabilityError {}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{Extensions, OwnerRef, ProfileDimension, ProfileId, Revision, Title};
    use std::str::FromStr;

    fn id<T: FromStr>(value: &str) -> Result<T, T::Err> {
        value.parse()
    }

    fn profile() -> Result<Profile, Box<dyn Error>> {
        Ok(Profile::new(
            ProfileId::new("audience.default")?,
            Revision::new(1)?,
            Title::new("Audience")?,
            vec![OwnerRef::from_str("owner://team/product")?],
            vec![ProfileDimension::new(
                id("region")?,
                vec![id("eu")?, id("us")?],
                None,
            )?],
            Vec::new(),
            None,
            Extensions::default(),
        )?)
    }

    fn context(value: Option<&str>) -> Result<ApplicabilityContext, Box<dyn Error>> {
        ApplicabilityContext::new(
            &profile()?,
            BTreeMap::from([(id("region")?, value.map(id).transpose()?)]),
        )
        .map_err(Into::into)
    }

    #[test]
    fn comparison_membership_and_unknown_match_the_leaf_table() -> Result<(), Box<dyn Error>> {
        let eu = Applicability::compare(id("region")?, ComparisonOperator::Equal, id("eu")?);
        let ne = Applicability::compare(id("region")?, ComparisonOperator::NotEqual, id("eu")?);
        let inside =
            Applicability::membership(id("region")?, MembershipOperator::In, vec![id("eu")?])?;
        let outside =
            Applicability::membership(id("region")?, MembershipOperator::NotIn, vec![id("eu")?])?;
        for (selected, expected) in [
            (
                Some("eu"),
                [
                    TruthValue::True,
                    TruthValue::False,
                    TruthValue::True,
                    TruthValue::False,
                ],
            ),
            (
                Some("us"),
                [
                    TruthValue::False,
                    TruthValue::True,
                    TruthValue::False,
                    TruthValue::True,
                ],
            ),
            (None, [TruthValue::Unknown; 4]),
        ] {
            let context = context(selected)?;
            assert_eq!(
                [
                    evaluate_applicability(&eu, &context)?,
                    evaluate_applicability(&ne, &context)?,
                    evaluate_applicability(&inside, &context)?,
                    evaluate_applicability(&outside, &context)?,
                ],
                expected
            );
        }
        Ok(())
    }

    #[test]
    fn boolean_operators_cover_every_three_valued_pair() -> Result<(), Box<dyn Error>> {
        let left_expressions = [
            (TruthValue::True, Applicability::always(true)),
            (TruthValue::False, Applicability::always(false)),
            (
                TruthValue::Unknown,
                Applicability::compare(id("region")?, ComparisonOperator::Equal, id("eu")?),
            ),
        ];
        let right_expressions =
            [
                (
                    TruthValue::True,
                    Applicability::logical_not(Applicability::always(false))?,
                ),
                (
                    TruthValue::False,
                    Applicability::logical_not(Applicability::always(true))?,
                ),
                (
                    TruthValue::Unknown,
                    Applicability::logical_not(Applicability::logical_not(
                        Applicability::compare(id("region")?, ComparisonOperator::Equal, id("eu")?),
                    )?)?,
                ),
            ];
        let context = context(None)?;
        for (left_value, left) in &left_expressions {
            assert_eq!(
                evaluate_applicability(&Applicability::logical_not(left.clone())?, &context)?,
                left_value.logical_not()
            );
            for (right_value, right) in &right_expressions {
                assert_eq!(
                    evaluate_applicability(
                        &Applicability::all(vec![left.clone(), right.clone()])?,
                        &context
                    )?,
                    left_value.logical_all(*right_value)
                );
                assert_eq!(
                    evaluate_applicability(
                        &Applicability::any(vec![left.clone(), right.clone()])?,
                        &context
                    )?,
                    left_value.logical_any(*right_value)
                );
            }
        }
        Ok(())
    }

    #[test]
    fn undeclared_dimensions_and_values_fail_even_in_short_circuited_branches()
    -> Result<(), Box<dyn Error>> {
        let context = context(Some("eu"))?;
        let dimension =
            Applicability::compare(id("missing")?, ComparisonOperator::Equal, id("eu")?);
        let value = Applicability::compare(id("region")?, ComparisonOperator::Equal, id("apac")?);
        assert_eq!(
            evaluate_applicability(&dimension, &context),
            Err(ApplicabilityError::UndeclaredDimension)
        );
        assert_eq!(
            evaluate_applicability(
                &Applicability::all(vec![Applicability::always(false), value])?,
                &context
            ),
            Err(ApplicabilityError::UndeclaredValue)
        );
        assert_eq!(
            ApplicabilityContext::new(
                &profile()?,
                BTreeMap::from([(id("region")?, Some(id("apac")?))]),
            ),
            Err(ApplicabilityError::UndeclaredValue)
        );
        Ok(())
    }
}
