//! Conversion from authored DTOs into validated contract domain inputs.

use crate::dto::{
    ApplicabilityDto, ArtifactDto, ArtifactSelectorDto, EvidenceSelectorDto,
    EvidenceSpecificationDto, ExtensionsDto, RequirementDto,
};
use crate::{DocumentDto, ValidatedDocument};
use eqm_domain::{
    Applicability, ArgumentTemplate, Artifact, ArtifactSelector, Artifacts, Binding, Capability,
    ComparisonOperator, Description, DurationMillis, EnvironmentBinding, EnvironmentName,
    EnvironmentSource, EvidenceSelector, EvidenceSpecification, ExtensionKey, ExtensionNamespace,
    ExtensionValue, Extensions, Facet, Fragment, FragmentUse, HttpMethod, Journey,
    MembershipOperator, OwnerRef, Policy, PolicyRule, PolicySelector, PositiveCount, PositiveDays,
    Profile, ProfileDimension, RepoPath, Requirement, RequirementStatement, Revision,
    RouteSelector, RunnerDefinition, RunnerLimits, RunnerProgram, SecretBinding, SecretProviderRef,
    SelectorText, Surface, SymbolicValueId, TargetId, Title, Transition, TransitionTrigger,
    WaiverPolicy, WorkingDirectoryTemplate,
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

/// Converts one strict binding DTO and confines artifacts to its target root.
pub fn convert_binding(
    document: &ValidatedDocument,
    target_roots: &BTreeMap<TargetId, RepoPath>,
) -> Result<Binding, ConversionError> {
    let source = document.source().as_str();
    let DocumentDto::Binding(dto) = document.document() else {
        return Err(ConversionError::at(source, "schema"));
    };
    let target: TargetId = parse(&dto.target, "target", source)?;
    let target_root = target_roots
        .get(&target)
        .ok_or_else(|| ConversionError::at(source, "target"))?;
    Binding::new(
        parse(&dto.id, "id", source)?,
        build(Revision::new(dto.revision), "revision", source)?,
        owners(&dto.owners, source)?,
        target,
        parse(&dto.unit, "unit", source)?,
        build(
            Artifacts::new(
                dto.artifacts
                    .iter()
                    .map(|value| artifact(value, target_root, source))
                    .collect::<Result<_, _>>()?,
            ),
            "artifacts",
            source,
        )?,
        dto.exposures
            .iter()
            .map(|exposure| {
                Ok(eqm_domain::Exposure::new(
                    parse(&exposure.surface, "exposures.surface", source)?,
                    parse(&exposure.state, "exposures.state", source)?,
                    exposure
                        .applicability
                        .as_ref()
                        .map(|value| applicability(value, source))
                        .transpose()?
                        .unwrap_or_default(),
                    exposure
                        .route
                        .as_deref()
                        .map(|value| build(RouteSelector::new(value), "exposures.route", source))
                        .transpose()?,
                    extensions(&exposure.extensions, source)?,
                ))
            })
            .collect::<Result<_, ConversionError>>()?,
        dto.evidence
            .iter()
            .map(|value| evidence(value, source))
            .collect::<Result<_, _>>()?,
        extensions(&dto.extensions, source)?,
    )
    .map_err(|_| ConversionError::at(source, "binding"))
}

/// Converts one strict profile DTO into a finite symbolic profile family.
pub fn convert_profile(document: &ValidatedDocument) -> Result<Profile, ConversionError> {
    let source = document.source().as_str();
    let DocumentDto::Profile(dto) = document.document() else {
        return Err(ConversionError::at(source, "schema"));
    };
    let mut cohort_count = 1_usize;
    let dimensions = dto
        .dimensions
        .iter()
        .map(|dimension| {
            cohort_count = cohort_count
                .checked_mul(dimension.values.len())
                .ok_or_else(|| ConversionError::at(source, "dimensions.values"))?;
            if cohort_count > 100_000 {
                return Err(ConversionError::at(source, "dimensions.values"));
            }
            ProfileDimension::new(
                parse(&dimension.id, "dimensions.id", source)?,
                dimension
                    .values
                    .iter()
                    .map(|value| parse(value, "dimensions.values", source))
                    .collect::<Result<_, _>>()?,
                description(dimension.description.as_deref(), source)?,
            )
            .map_err(|_| ConversionError::at(source, "dimensions"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Profile::new(
        parse(&dto.id, "id", source)?,
        build(Revision::new(dto.revision), "revision", source)?,
        build(Title::new(dto.title.as_str()), "title", source)?,
        owners(&dto.owners, source)?,
        dimensions,
        dto.defaults
            .iter()
            .map(|(dimension, value)| {
                Ok((
                    parse(dimension, "defaults", source)?,
                    parse(value, "defaults", source)?,
                ))
            })
            .collect::<Result<_, ConversionError>>()?,
        description(dto.description.as_deref(), source)?,
        extensions(&dto.extensions, source)?,
    )
    .map_err(|_| ConversionError::at(source, "profile"))
}

/// Converts one strict policy DTO into its monotonic domain authority.
pub fn convert_policy(document: &ValidatedDocument) -> Result<Policy, ConversionError> {
    let source = document.source().as_str();
    let DocumentDto::Policy(dto) = document.document() else {
        return Err(ConversionError::at(source, "schema"));
    };
    let rules = dto
        .rules
        .iter()
        .map(|rule| {
            PolicyRule::new(
                PolicySelector::new(
                    optional_parse(&rule.selector.units, "rules.selector.units", source)?,
                    optional_parse(
                        &rule.selector.requirements,
                        "rules.selector.requirements",
                        source,
                    )?,
                    optional_parse(
                        &rule.selector.risk_classes,
                        "rules.selector.risk_classes",
                        source,
                    )?,
                    optional_parse(&rule.selector.facets, "rules.selector.facets", source)?,
                    optional_parse(&rule.selector.scopes, "rules.selector.scopes", source)?,
                )
                .map_err(|_| ConversionError::at(source, "rules.selector"))?,
                parse(&rule.minimum_level, "rules.minimum_level", source)?,
                rule.facets
                    .iter()
                    .map(|value| parse(value, "rules.facets", source))
                    .collect::<Result<_, _>>()?,
                parse(&rule.minimum_trust, "rules.minimum_trust", source)?,
                build(
                    DurationMillis::new(rule.maximum_age),
                    "rules.maximum_age",
                    source,
                )?,
                rule.minimum_count
                    .map(|value| build(PositiveCount::new(value), "rules.minimum_count", source))
                    .transpose()?,
            )
            .map_err(|_| ConversionError::at(source, "rules"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let waivers = dto
        .waivers
        .as_ref()
        .map(|waivers| {
            WaiverPolicy::new(
                waivers.allowed.unwrap_or(false),
                waivers
                    .maximum_days
                    .map(|value| build(PositiveDays::new(value), "waivers.maximum_days", source))
                    .transpose()?,
                waivers
                    .minimum_approvers
                    .map(|value| {
                        build(
                            PositiveCount::new(value),
                            "waivers.minimum_approvers",
                            source,
                        )
                    })
                    .transpose()?,
                waivers
                    .required_controls
                    .iter()
                    .map(|value| parse(value, "waivers.required_controls", source))
                    .collect::<Result<_, _>>()?,
            )
            .map_err(|_| ConversionError::at(source, "waivers"))
        })
        .transpose()?
        .unwrap_or_default();
    Policy::new(
        parse(&dto.id, "id", source)?,
        build(Revision::new(dto.revision), "revision", source)?,
        build(Title::new(dto.title.as_str()), "title", source)?,
        owners(&dto.owners, source)?,
        dto.profiles
            .iter()
            .map(|value| parse(value, "profiles", source))
            .collect::<Result<_, _>>()?,
        dto.required_targets
            .iter()
            .map(|value| parse(value, "required_targets", source))
            .collect::<Result<_, _>>()?,
        rules,
        waivers,
        description(dto.description.as_deref(), source)?,
        extensions(&dto.extensions, source)?,
    )
    .map_err(|_| ConversionError::at(source, "policy"))
}

/// Converts one strict runner DTO into a shell-free bounded runner definition.
pub fn convert_runner(document: &ValidatedDocument) -> Result<RunnerDefinition, ConversionError> {
    let source = document.source().as_str();
    let DocumentDto::Runner(dto) = document.document() else {
        return Err(ConversionError::at(source, "schema"));
    };
    if !(1_000..=3_600_000).contains(&dto.timeout_ms) {
        return Err(ConversionError::at(source, "timeout_ms"));
    }
    if dto.max_output_bytes == 0 || dto.max_output_bytes > 16 * 1024 * 1024 {
        return Err(ConversionError::at(source, "max_output_bytes"));
    }
    if dto
        .max_concurrency
        .is_some_and(|value| !(1..=64).contains(&value))
    {
        return Err(ConversionError::at(source, "max_concurrency"));
    }
    if dto.program.chars().any(char::is_whitespace)
        || dto.program.contains([';', '&', '|', '$', '`'])
    {
        return Err(ConversionError::at(source, "program"));
    }
    RunnerDefinition::new(
        parse(&dto.id, "id", source)?,
        build(Revision::new(dto.revision), "revision", source)?,
        owners(&dto.owners, source)?,
        parse(&dto.backend, "backend", source)?,
        RunnerProgram::Repository(parse(&dto.program, "program", source)?),
        dto.args
            .iter()
            .map(|value| argument(value, source))
            .collect::<Result<_, _>>()?,
        dto.cwd
            .as_deref()
            .map(|value| working_directory(value, source))
            .transpose()?,
        dto.environment
            .iter()
            .map(|binding| {
                let source_value = match binding.source.as_str() {
                    "literal" => EnvironmentSource::Literal(text(
                        binding
                            .value
                            .as_deref()
                            .ok_or_else(|| ConversionError::at(source, "environment.value"))?,
                        "environment.value",
                        source,
                    )?),
                    "trusted_path" if binding.value.is_none() => EnvironmentSource::TrustedPath,
                    "canonical_locale" if binding.value.is_none() => {
                        EnvironmentSource::CanonicalLocale
                    }
                    "utc_timezone" if binding.value.is_none() => EnvironmentSource::UtcTimezone,
                    _ => return Err(ConversionError::at(source, "environment.source")),
                };
                EnvironmentBinding::new(
                    build(
                        EnvironmentName::new(binding.name.as_str()),
                        "environment.name",
                        source,
                    )?,
                    source_value,
                )
                .map_err(|_| ConversionError::at(source, "environment"))
            })
            .collect::<Result<_, _>>()?,
        dto.secrets
            .iter()
            .map(|binding| {
                Ok(SecretBinding::new(
                    build(
                        EnvironmentName::new(binding.name.as_str()),
                        "secrets.name",
                        source,
                    )?,
                    build(
                        SecretProviderRef::new(binding.provider.as_str()),
                        "secrets.provider",
                        source,
                    )?,
                ))
            })
            .collect::<Result<_, ConversionError>>()?,
        build(
            RunnerLimits::new(
                build(DurationMillis::new(dto.timeout_ms), "timeout_ms", source)?,
                build(
                    PositiveCount::new(dto.max_output_bytes),
                    "max_output_bytes",
                    source,
                )?,
                dto.max_concurrency
                    .map(|value| build(PositiveCount::new(value), "max_concurrency", source))
                    .transpose()?,
            ),
            "limits",
            source,
        )?,
        dto.guarantees
            .iter()
            .map(|value| parse(value, "guarantees", source))
            .collect::<Result<_, _>>()?,
        extensions(&dto.extensions, source)?,
    )
    .map_err(|_| ConversionError::at(source, "runner"))
}

fn argument(value: &str, source: &str) -> Result<ArgumentTemplate, ConversionError> {
    match value {
        "{target_root}" => Ok(ArgumentTemplate::TargetRoot),
        "{selector_json}" => Ok(ArgumentTemplate::SelectorJson),
        "{result_path}" => Ok(ArgumentTemplate::ResultPath),
        _ if value.contains(['{', '}']) => Err(ConversionError::at(source, "args")),
        _ => Ok(ArgumentTemplate::Literal(text(value, "args", source)?)),
    }
}

fn working_directory(
    value: &str,
    source: &str,
) -> Result<WorkingDirectoryTemplate, ConversionError> {
    match value {
        "{target_root}" => Ok(WorkingDirectoryTemplate::TargetRoot),
        "{result_path}" => Ok(WorkingDirectoryTemplate::ResultPath),
        _ if value.contains(['{', '}']) => Err(ConversionError::at(source, "cwd")),
        _ => Ok(WorkingDirectoryTemplate::Repository(parse(
            value, "cwd", source,
        )?)),
    }
}

fn optional_parse<T>(
    values: &Option<Vec<String>>,
    field: &'static str,
    source: &str,
) -> Result<Option<Vec<T>>, ConversionError>
where
    T: FromStr,
{
    values
        .as_ref()
        .map(|values| {
            values
                .iter()
                .map(|value| parse(value, field, source))
                .collect()
        })
        .transpose()
}

fn artifact(
    dto: &ArtifactDto,
    target_root: &RepoPath,
    source: &str,
) -> Result<Artifact, ConversionError> {
    let path: RepoPath = parse(&dto.path, "artifacts.path", source)?;
    let within_target = path == *target_root
        || path
            .as_str()
            .strip_prefix(target_root.as_str())
            .is_some_and(|suffix| suffix.starts_with('/'));
    if !within_target {
        return Err(ConversionError::at(source, "artifacts.path"));
    }
    Artifact::new(
        parse(&dto.id, "artifacts.id", source)?,
        parse(&dto.role, "artifacts.role", source)?,
        path,
        dto.surface
            .as_deref()
            .map(|value| parse(value, "artifacts.surface", source))
            .transpose()?,
        dto.symbol
            .as_deref()
            .map(|value| build(SelectorText::new(value), "artifacts.symbol", source))
            .transpose()?,
        dto.selector
            .as_ref()
            .map(|value| artifact_selector(value, source))
            .transpose()?,
        extensions(&dto.extensions, source)?,
    )
    .map_err(|_| ConversionError::at(source, "artifacts"))
}

fn artifact_selector(
    dto: &ArtifactSelectorDto,
    source: &str,
) -> Result<ArtifactSelector, ConversionError> {
    match dto {
        ArtifactSelectorDto::Symbol { name, language } => Ok(ArtifactSelector::Symbol {
            name: text(name, "artifacts.selector.name", source)?,
            language: optional_text(language.as_deref(), "artifacts.selector.language", source)?,
        }),
        ArtifactSelectorDto::Route { path, method } => Ok(ArtifactSelector::Route {
            path: text(path, "artifacts.selector.path", source)?,
            method: method
                .as_deref()
                .map(|value| parse(value, "artifacts.selector.method", source))
                .transpose()?,
        }),
        ArtifactSelectorDto::Test {
            framework,
            test_id,
            suite,
        } => Ok(ArtifactSelector::Test {
            framework: text(framework, "artifacts.selector.framework", source)?,
            test_id: text(test_id, "artifacts.selector.test_id", source)?,
            suite: optional_text(suite.as_deref(), "artifacts.selector.suite", source)?,
        }),
        ArtifactSelectorDto::Inventory {
            record_type,
            key,
            value,
        } => Ok(ArtifactSelector::Inventory {
            record_type: text(record_type, "artifacts.selector.record_type", source)?,
            key: text(key, "artifacts.selector.key", source)?,
            value: optional_text(value.as_deref(), "artifacts.selector.value", source)?,
        }),
    }
}

fn evidence(
    dto: &EvidenceSpecificationDto,
    source: &str,
) -> Result<EvidenceSpecification, ConversionError> {
    EvidenceSpecification::new(
        parse(&dto.id, "evidence.id", source)?,
        parse(&dto.kind, "evidence.kind", source)?,
        dto.requirements
            .iter()
            .map(|value| parse(value, "evidence.requirements", source))
            .collect::<Result<_, _>>()?,
        dto.facets
            .iter()
            .map(|value| parse(value, "evidence.facets", source))
            .collect::<Result<_, _>>()?,
        dto.runner
            .as_deref()
            .map(|value| parse(value, "evidence.runner", source))
            .transpose()?,
        dto.selector
            .as_ref()
            .map(|value| evidence_selector(value, source))
            .transpose()?,
        dto.minimum_count
            .map(|value| build(PositiveCount::new(value), "evidence.minimum_count", source))
            .transpose()?,
        dto.freshness
            .map(|value| build(DurationMillis::new(value), "evidence.freshness", source))
            .transpose()?,
        extensions(&dto.extensions, source)?,
    )
    .map_err(|_| ConversionError::at(source, "evidence"))
}

fn evidence_selector(
    dto: &EvidenceSelectorDto,
    source: &str,
) -> Result<EvidenceSelector, ConversionError> {
    match dto {
        EvidenceSelectorDto::Symbol { name, language } => Ok(EvidenceSelector::Symbol {
            name: text(name, "evidence.selector.name", source)?,
            language: optional_text(language.as_deref(), "evidence.selector.language", source)?,
        }),
        EvidenceSelectorDto::Route { path, method } => Ok(EvidenceSelector::Route {
            path: text(path, "evidence.selector.path", source)?,
            method: method
                .as_deref()
                .map(|value| parse::<HttpMethod>(value, "evidence.selector.method", source))
                .transpose()?,
        }),
        EvidenceSelectorDto::Test {
            framework,
            test_id,
            suite,
        } => Ok(EvidenceSelector::Test {
            framework: text(framework, "evidence.selector.framework", source)?,
            test_id: text(test_id, "evidence.selector.test_id", source)?,
            suite: optional_text(suite.as_deref(), "evidence.selector.suite", source)?,
        }),
        EvidenceSelectorDto::Inventory {
            record_type,
            key,
            value,
        } => Ok(EvidenceSelector::Inventory {
            record_type: text(record_type, "evidence.selector.record_type", source)?,
            key: text(key, "evidence.selector.key", source)?,
            value: optional_text(value.as_deref(), "evidence.selector.value", source)?,
        }),
        EvidenceSelectorDto::Snapshot {
            snapshot_id,
            variant,
        } => Ok(EvidenceSelector::Snapshot {
            snapshot_id: text(snapshot_id, "evidence.selector.snapshot_id", source)?,
            variant: optional_text(variant.as_deref(), "evidence.selector.variant", source)?,
        }),
        EvidenceSelectorDto::Release { channel } => Ok(EvidenceSelector::Release {
            channel: parse(channel, "evidence.selector.channel", source)?,
        }),
    }
}

fn text(value: &str, field: &'static str, source: &str) -> Result<SelectorText, ConversionError> {
    build(SelectorText::new(value), field, source)
}

fn optional_text(
    value: Option<&str>,
    field: &'static str,
    source: &str,
) -> Result<Option<SelectorText>, ConversionError> {
    value.map(|value| text(value, field, source)).transpose()
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
    use crate::dto::{
        BindingDto, CapabilityDto, FragmentDto, JourneyDto, PolicyDto, ProfileDimensionDto,
        ProfileDto, RunnerDto, SurfaceDto,
    };
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

    fn binding() -> Result<BindingDto, toml::de::Error> {
        toml::from_str(
            r#"schema = "https://schemas.equivalencematrix.dev/v1/binding"
id = "binding.web"
revision = 1
owners = ["owner://team/web"]
target = "web"
unit = "account.create.primary.form"

[[artifacts]]
id = "form"
role = "view"
path = "apps/web/src/form.ts"
surface = "account.create.primary.form"

[[exposures]]
surface = "account.create.primary.form"
state = "required"
route = "/create"

[[evidence]]
id = "form_test"
kind = "test"
requirements = ["account.create.primary.form#submit"]
facets = ["behavior"]
runner = "runner.web"

[evidence.selector]
kind = "test"
framework = "vitest"
test_id = "form submits"
"#,
        )
    }

    fn target_roots() -> Result<BTreeMap<TargetId, RepoPath>, Box<dyn Error>> {
        Ok(BTreeMap::from([(
            TargetId::from_str("web")?,
            RepoPath::new("apps/web")?,
        )]))
    }

    #[test]
    fn binding_artifacts_exposures_and_evidence_convert() -> Result<(), Box<dyn Error>> {
        let binding = document(DocumentDto::Binding(binding()?))?;
        let converted = convert_binding(&binding, &target_roots()?)?;
        assert_eq!(converted.artifacts().values().len(), 1);
        assert_eq!(converted.exposures().len(), 1);
        assert_eq!(converted.evidence().len(), 1);
        Ok(())
    }

    #[test]
    fn binding_paths_and_coverage_fail_closed() -> Result<(), Box<dyn Error>> {
        let mut outside = binding()?;
        outside.artifacts[0].path = "apps/admin/form.ts".to_owned();
        let error = convert_binding(&document(DocumentDto::Binding(outside))?, &target_roots()?)
            .err()
            .ok_or("outside artifact accepted")?;
        assert_eq!(error.field(), "artifacts.path");

        let mut uncovered = binding()?;
        uncovered.artifacts[0].surface = None;
        let error = convert_binding(
            &document(DocumentDto::Binding(uncovered))?,
            &target_roots()?,
        )
        .err()
        .ok_or("uncovered view accepted")?;
        assert_eq!(error.field(), "artifacts");
        Ok(())
    }

    #[test]
    fn profile_and_policy_convert_with_finite_declared_dimensions() -> Result<(), Box<dyn Error>> {
        let profile: ProfileDto = toml::from_str(
            r#"schema = "https://schemas.equivalencematrix.dev/v1/profile"
id = "audience.default"
revision = 1
title = "Audience"
owners = ["owner://team/product"]

[[dimensions]]
id = "region"
values = ["eu", "us"]

[defaults]
region = "eu"
"#,
        )?;
        let converted = convert_profile(&document(DocumentDto::Profile(profile))?)?;
        assert_eq!(converted.dimensions().len(), 1);
        assert_eq!(converted.defaults().len(), 1);

        let policy: PolicyDto = toml::from_str(
            r#"schema = "https://schemas.equivalencematrix.dev/v1/policy"
id = "policy.default"
revision = 1
title = "Default policy"
owners = ["owner://team/security"]
profiles = ["audience.default"]
required_targets = ["web"]

[[rules]]
minimum_level = "required"
facets = ["behavior"]
minimum_trust = "trusted_ci"
maximum_age = 86400000

[rules.selector]
units = ["account.create.primary.form"]

[waivers]
allowed = true
maximum_days = 7
minimum_approvers = 2
required_controls = ["behavior"]
"#,
        )?;
        let converted = convert_policy(&document(DocumentDto::Policy(policy))?)?;
        assert_eq!(converted.rules().len(), 1);
        assert!(converted.waivers().allowed());
        Ok(())
    }

    #[test]
    fn invalid_defaults_and_unbounded_symbolic_cohorts_fail() -> Result<(), Box<dyn Error>> {
        let mut profile: ProfileDto = toml::from_str(
            r#"schema = "https://schemas.equivalencematrix.dev/v1/profile"
id = "audience.default"
revision = 1
title = "Audience"
owners = ["owner://team/product"]

[[dimensions]]
id = "region"
values = ["eu"]

[defaults]
region = "missing"
"#,
        )?;
        let error = convert_profile(&document(DocumentDto::Profile(profile.clone()))?)
            .err()
            .ok_or("invalid default accepted")?;
        assert_eq!(error.field(), "profile");

        let values: Vec<_> = (0..317).map(|index| format!("v{index}")).collect();
        profile.defaults.clear();
        profile.dimensions = vec![
            ProfileDimensionDto {
                id: "first".to_owned(),
                values: values.clone(),
                description: None,
            },
            ProfileDimensionDto {
                id: "second".to_owned(),
                values,
                description: None,
            },
        ];
        let error = convert_profile(&document(DocumentDto::Profile(profile))?)
            .err()
            .ok_or("unbounded cohort accepted")?;
        assert_eq!(error.field(), "dimensions.values");
        Ok(())
    }

    fn runner() -> Result<RunnerDto, toml::de::Error> {
        toml::from_str(
            r#"schema = "https://schemas.equivalencematrix.dev/v1/runner"
id = "runner.web"
revision = 1
owners = ["owner://team/tooling"]
backend = "local"
program = "bin/test-runner"
args = ["test", "{selector_json}", "{result_path}"]
cwd = "{target_root}"
timeout_ms = 60000
max_output_bytes = 1048576
max_concurrency = 2

[[environment]]
name = "PATH"
source = "trusted_path"

[[environment]]
name = "MODE"
source = "literal"
value = "test"

[[secrets]]
name = "TOKEN"
provider = "secret://vault/token"
"#,
        )
    }

    #[test]
    fn runner_conversion_is_shell_free_and_bounded() -> Result<(), Box<dyn Error>> {
        let converted = convert_runner(&document(DocumentDto::Runner(runner()?))?)?;
        assert_eq!(converted.args().len(), 3);
        assert_eq!(converted.environment().len(), 2);
        assert_eq!(converted.secrets().len(), 1);
        assert_eq!(converted.limits().max_concurrency().get(), 2);
        Ok(())
    }

    #[test]
    fn command_strings_placeholders_limits_and_guarantees_fail() -> Result<(), Box<dyn Error>> {
        for (mut runner, field) in [
            (
                {
                    let mut value = runner()?;
                    value.program = "cargo test".to_owned();
                    value
                },
                "program",
            ),
            (
                {
                    let mut value = runner()?;
                    value.args = vec!["--out={result_path}".to_owned()];
                    value
                },
                "args",
            ),
            (
                {
                    let mut value = runner()?;
                    value.timeout_ms = 0;
                    value
                },
                "timeout_ms",
            ),
        ] {
            let error = convert_runner(&document(DocumentDto::Runner(runner.clone()))?)
                .err()
                .ok_or("invalid runner accepted")?;
            assert_eq!(error.field(), field);
            runner.args.clear();
        }

        let mut unsupported = runner()?;
        unsupported.guarantees = vec!["network_denied".to_owned()];
        let error = convert_runner(&document(DocumentDto::Runner(unsupported))?)
            .err()
            .ok_or("unsupported local guarantee accepted")?;
        assert_eq!(error.field(), "runner");
        Ok(())
    }
}
