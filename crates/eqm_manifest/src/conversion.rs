//! Conversion from authored DTOs into validated contract domain inputs.

use crate::dto::{ApplicabilityDto, ExtensionsDto, RequirementDto};
use crate::{DocumentDto, ValidatedDocument};
use eqm_domain::{
    Applicability, Capability, ComparisonOperator, Description, ExtensionKey, ExtensionNamespace,
    ExtensionValue, Extensions, Facet, Fragment, FragmentUse, Journey, MembershipOperator,
    OwnerRef, Requirement, RequirementStatement, Revision, Surface, SymbolicValueId, Title,
    Transition, TransitionTrigger,
};
use serde_json::Value;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// One validated contract authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractEntity {
    /// Capability authority.
    Capability(Capability),
    /// Journey authority.
    Journey(Journey),
    /// Surface authority.
    Surface(Surface),
    /// Fragment authority.
    Fragment(Fragment),
}

/// Converts one strict contract DTO into its validated domain authority.
pub fn convert_contract(document: &ValidatedDocument) -> Result<ContractEntity, ConversionError> {
    let source = document.source().as_str();
    match document.document() {
        DocumentDto::Capability(dto) => Capability::new(
            parse(&dto.id, "id", source)?,
            build(Title::new(dto.title.as_str()), "title", source)?,
            parse(&dto.status, "status", source)?,
            owners(&dto.owners, source)?,
            description(dto.description.as_deref(), source)?,
            extensions(&dto.extensions, source)?,
        )
        .map(ContractEntity::Capability)
        .map_err(|_| ConversionError::at(source, "capability")),
        DocumentDto::Journey(dto) => Journey::new(
            parse(&dto.id, "id", source)?,
            build(Revision::new(dto.revision), "revision", source)?,
            build(Title::new(dto.title.as_str()), "title", source)?,
            parse(&dto.capability, "capability", source)?,
            parse(&dto.status, "status", source)?,
            parse(&dto.risk_class, "risk_class", source)?,
            owners(&dto.owners, source)?,
            dto.surfaces
                .iter()
                .map(|value| parse(value, "surfaces", source))
                .collect::<Result<_, _>>()?,
            dto.transitions
                .iter()
                .map(|transition| {
                    Ok(Transition::new(
                        parse(&transition.from, "transitions.from", source)?,
                        parse(&transition.to, "transitions.to", source)?,
                        build(
                            TransitionTrigger::new(transition.trigger.as_str()),
                            "transitions.trigger",
                            source,
                        )?,
                    ))
                })
                .collect::<Result<_, ConversionError>>()?,
            description(dto.description.as_deref(), source)?,
            extensions(&dto.extensions, source)?,
        )
        .map(ContractEntity::Journey)
        .map_err(|_| ConversionError::at(source, "journey")),
        DocumentDto::Surface(dto) => Surface::new(
            parse(&dto.id, "id", source)?,
            build(Revision::new(dto.revision), "revision", source)?,
            build(Title::new(dto.title.as_str()), "title", source)?,
            parse(&dto.journey, "journey", source)?,
            parse(&dto.status, "status", source)?,
            owners(&dto.owners, source)?,
            dto.requirements
                .iter()
                .map(|value| requirement(value, source))
                .collect::<Result<_, _>>()?,
            dto.fragments
                .iter()
                .map(|fragment| {
                    Ok(FragmentUse::new(
                        parse(&fragment.fragment, "fragments.fragment", source)?,
                        build(
                            Revision::new(fragment.revision),
                            "fragments.revision",
                            source,
                        )?,
                        parse(&fragment.digest, "fragments.digest", source)?,
                        fragment
                            .prefix
                            .as_deref()
                            .map(|value| parse(value, "fragments.prefix", source))
                            .transpose()?,
                    ))
                })
                .collect::<Result<_, ConversionError>>()?,
            description(dto.description.as_deref(), source)?,
            extensions(&dto.extensions, source)?,
        )
        .map(ContractEntity::Surface)
        .map_err(|_| ConversionError::at(source, "surface")),
        DocumentDto::Fragment(dto) => Fragment::new(
            parse(&dto.id, "id", source)?,
            build(Revision::new(dto.revision), "revision", source)?,
            build(Title::new(dto.title.as_str()), "title", source)?,
            parse(&dto.risk_class, "risk_class", source)?,
            owners(&dto.owners, source)?,
            dto.requirements
                .iter()
                .map(|value| requirement(value, source))
                .collect::<Result<_, _>>()?,
            description(dto.description.as_deref(), source)?,
            extensions(&dto.extensions, source)?,
        )
        .map(ContractEntity::Fragment)
        .map_err(|_| ConversionError::at(source, "fragment")),
        _ => Err(ConversionError::at(source, "schema")),
    }
}

fn requirement(dto: &RequirementDto, source: &str) -> Result<Requirement, ConversionError> {
    Requirement::new(
        parse(&dto.id, "requirements.id", source)?,
        parse(&dto.level, "requirements.level", source)?,
        parse(&dto.scope, "requirements.scope", source)?,
        build(
            RequirementStatement::new(dto.statement.as_str()),
            "requirements.statement",
            source,
        )?,
        dto.facets
            .iter()
            .map(|value| parse(value, "requirements.facets", source))
            .collect::<Result<Vec<Facet>, _>>()?,
        dto.applicability
            .as_ref()
            .map(|value| applicability(value, source))
            .transpose()?
            .unwrap_or_default(),
        dto.risk_class
            .as_deref()
            .map(|value| parse(value, "requirements.risk_class", source))
            .transpose()?,
        dto.provider
            .as_deref()
            .map(|value| parse(value, "requirements.provider", source))
            .transpose()?,
        extensions(&dto.extensions, source)?,
    )
    .map_err(|_| ConversionError::at(source, "requirements"))
}

fn applicability(dto: &ApplicabilityDto, source: &str) -> Result<Applicability, ConversionError> {
    match dto {
        ApplicabilityDto::Constant(value) => Ok(Applicability::always(value.always)),
        ApplicabilityDto::Comparison(value) => Ok(Applicability::compare(
            parse(&value.dimension, "applicability.dimension", source)?,
            match value.operator.as_str() {
                "eq" => ComparisonOperator::Equal,
                "ne" => ComparisonOperator::NotEqual,
                _ => return Err(ConversionError::at(source, "applicability.operator")),
            },
            parse(&value.value, "applicability.value", source)?,
        )),
        ApplicabilityDto::Membership(value) => Applicability::membership(
            parse(&value.dimension, "applicability.dimension", source)?,
            match value.operator.as_str() {
                "in" => MembershipOperator::In,
                "not_in" => MembershipOperator::NotIn,
                _ => return Err(ConversionError::at(source, "applicability.operator")),
            },
            value
                .values
                .iter()
                .map(|item| parse(item, "applicability.values", source))
                .collect::<Result<Vec<SymbolicValueId>, _>>()?,
        )
        .map_err(|_| ConversionError::at(source, "applicability.values")),
        ApplicabilityDto::All(value) => Applicability::all(
            value
                .all
                .iter()
                .map(|item| applicability(item, source))
                .collect::<Result<_, _>>()?,
        )
        .map_err(|_| ConversionError::at(source, "applicability.all")),
        ApplicabilityDto::Any(value) => Applicability::any(
            value
                .any
                .iter()
                .map(|item| applicability(item, source))
                .collect::<Result<_, _>>()?,
        )
        .map_err(|_| ConversionError::at(source, "applicability.any")),
        ApplicabilityDto::Not(value) => {
            Applicability::logical_not(applicability(&value.not, source)?)
                .map_err(|_| ConversionError::at(source, "applicability.not"))
        }
    }
}

fn owners(values: &[String], source: &str) -> Result<Vec<OwnerRef>, ConversionError> {
    values
        .iter()
        .map(|value| parse(value, "owners", source))
        .collect()
}

fn description(value: Option<&str>, source: &str) -> Result<Option<Description>, ConversionError> {
    value
        .map(|value| build(Description::new(value), "description", source))
        .transpose()
}

pub(crate) fn extensions(
    values: &ExtensionsDto,
    source: &str,
) -> Result<Extensions, ConversionError> {
    let values = values
        .iter()
        .map(|(namespace, value)| {
            Ok((
                build(
                    ExtensionNamespace::new(namespace.as_str()),
                    "extensions",
                    source,
                )?,
                extension_value(value, source)?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>, ConversionError>>()?;
    build(Extensions::new(values), "extensions", source)
}

fn extension_value(value: &Value, source: &str) -> Result<ExtensionValue, ConversionError> {
    match value {
        Value::Bool(value) => Ok(ExtensionValue::Boolean(*value)),
        Value::Number(value) => value
            .as_i64()
            .map(ExtensionValue::Integer)
            .ok_or_else(|| ConversionError::at(source, "extensions")),
        Value::String(value) => build(ExtensionValue::string(value.as_str()), "extensions", source),
        Value::Array(values) => values
            .iter()
            .map(|value| extension_value(value, source))
            .collect::<Result<_, _>>()
            .map(ExtensionValue::Array),
        Value::Object(values) => values
            .iter()
            .map(|(key, value)| {
                Ok((
                    build(ExtensionKey::new(key.as_str()), "extensions", source)?,
                    extension_value(value, source)?,
                ))
            })
            .collect::<Result<_, _>>()
            .map(ExtensionValue::Object),
        Value::Null => Err(ConversionError::at(source, "extensions")),
    }
}

fn parse<T>(value: &str, field: &'static str, source: &str) -> Result<T, ConversionError>
where
    T: FromStr,
{
    value
        .parse()
        .map_err(|_| ConversionError::at(source, field))
}

fn build<T, E>(
    result: Result<T, E>,
    field: &'static str,
    source: &str,
) -> Result<T, ConversionError> {
    result.map_err(|_| ConversionError::at(source, field))
}

/// Source- and field-associated domain conversion failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConversionError {
    source: Box<str>,
    field: &'static str,
}

impl ConversionError {
    fn at(source: &str, field: &'static str) -> Self {
        Self {
            source: source.into(),
            field,
        }
    }
    /// Returns the repository-relative source identity.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }
    /// Returns the nearest invalid field path.
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }
}

impl Display for ConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: invalid {}", self.source, self.field)
    }
}

impl Error for ConversionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::{CapabilityDto, FragmentDto, JourneyDto, SurfaceDto};
    use eqm_domain::RepoPath;

    fn document(document: DocumentDto) -> Result<ValidatedDocument, Box<dyn Error>> {
        Ok(ValidatedDocument::new(
            RepoPath::new("eqm/contracts/example.toml")?,
            document,
        ))
    }

    fn requirement_toml() -> &'static str {
        r#"[[requirements]]
id = "submit"
level = "required"
scope = "each_target"
statement = "The user can submit the form."
facets = ["behavior"]
"#
    }

    #[test]
    fn every_contract_family_converts_to_domain_authority() -> Result<(), Box<dyn Error>> {
        let capability: CapabilityDto = toml::from_str(
            r#"schema = "https://schemas.equivalencematrix.dev/v1/capability"
id = "account.create"
title = "Account"
status = "active"
owners = ["owner://team/accounts"]
"#,
        )?;
        assert!(matches!(
            convert_contract(&document(DocumentDto::Capability(capability))?)?,
            ContractEntity::Capability(_)
        ));

        let journey: JourneyDto = toml::from_str(
            r#"schema = "https://schemas.equivalencematrix.dev/v1/journey"
id = "account.create.primary"
revision = 1
title = "Create account"
capability = "account.create"
status = "active"
risk_class = "medium"
owners = ["owner://team/accounts"]
surfaces = ["account.create.primary.form"]
"#,
        )?;
        assert!(matches!(
            convert_contract(&document(DocumentDto::Journey(journey))?)?,
            ContractEntity::Journey(_)
        ));

        let surface: SurfaceDto = toml::from_str(&format!(
            r#"schema = "https://schemas.equivalencematrix.dev/v1/surface"
id = "account.create.primary.form"
revision = 1
title = "Account form"
journey = "account.create.primary"
status = "active"
owners = ["owner://team/accounts"]
{}"#,
            requirement_toml()
        ))?;
        assert!(matches!(
            convert_contract(&document(DocumentDto::Surface(surface))?)?,
            ContractEntity::Surface(_)
        ));

        let fragment: FragmentDto = toml::from_str(&format!(
            r#"schema = "https://schemas.equivalencematrix.dev/v1/fragment"
id = "common.form"
revision = 1
title = "Common form"
risk_class = "low"
owners = ["owner://team/design"]
{}"#,
            requirement_toml()
        ))?;
        assert!(matches!(
            convert_contract(&document(DocumentDto::Fragment(fragment))?)?,
            ContractEntity::Fragment(_)
        ));
        Ok(())
    }

    #[test]
    fn invalid_fields_report_the_nearest_field_and_source() -> Result<(), Box<dyn Error>> {
        let capability: CapabilityDto = toml::from_str(
            r#"schema = "https://schemas.equivalencematrix.dev/v1/capability"
id = "INVALID"
title = "Account"
status = "active"
owners = ["owner://team/accounts"]
"#,
        )?;
        let error = convert_contract(&document(DocumentDto::Capability(capability))?)
            .err()
            .ok_or("invalid ID accepted")?;
        assert_eq!(error.field(), "id");
        assert_eq!(error.source(), "eqm/contracts/example.toml");

        let surface: SurfaceDto = toml::from_str(&format!(
            r#"schema = "https://schemas.equivalencematrix.dev/v1/surface"
id = "account.create.primary.form"
revision = 1
title = "Account form"
journey = "account.create.primary"
status = "active"
owners = ["owner://team/accounts"]
{}"#,
            requirement_toml().replace("facets = [\"behavior\"]", "facets = [\"unknown\"]")
        ))?;
        let error = convert_contract(&document(DocumentDto::Surface(surface))?)
            .err()
            .ok_or("invalid facet accepted")?;
        assert_eq!(error.field(), "requirements.facets");
        Ok(())
    }
}
