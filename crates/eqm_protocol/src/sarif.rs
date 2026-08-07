//! Deterministic diagnostic projection into SARIF 2.1.0.

#![allow(missing_docs)]

use eqm_domain::{Diagnostic, DiagnosticDescriptor, Severity, SourceLocation, ToolVersion};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SARIF_SCHEMA: &str = "https://json.schemastore.org/sarif-2.1.0.json";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SarifLogDto {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub runs: Vec<SarifRunDto>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SarifRunDto {
    pub tool: SarifToolDto,
    pub results: Vec<SarifResultDto>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SarifToolDto {
    pub driver: SarifDriverDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SarifDriverDto {
    pub name: String,
    pub version: String,
    pub rules: Vec<SarifRuleDto>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SarifRuleDto {
    pub id: String,
    pub name: String,
    #[serde(rename = "shortDescription")]
    pub short_description: SarifMessageDto,
    #[serde(rename = "fullDescription")]
    pub full_description: SarifMessageDto,
    pub help: SarifMessageDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SarifMessageDto {
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SarifResultDto {
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    pub level: String,
    pub message: SarifMessageDto,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub locations: Vec<SarifLocationDto>,
    #[serde(
        rename = "relatedLocations",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub related_locations: Vec<SarifLocationDto>,
    pub properties: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SarifLocationDto {
    #[serde(rename = "physicalLocation")]
    pub physical_location: SarifPhysicalLocationDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SarifPhysicalLocationDto {
    #[serde(rename = "artifactLocation")]
    pub artifact_location: SarifArtifactLocationDto,
    pub region: SarifRegionDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SarifArtifactLocationDto {
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SarifRegionDto {
    #[serde(rename = "startLine")]
    pub start_line: u32,
    #[serde(rename = "startColumn")]
    pub start_column: u32,
    #[serde(rename = "endLine")]
    pub end_line: u32,
    #[serde(rename = "endColumn")]
    pub end_column: u32,
}

impl SarifLogDto {
    pub fn from_diagnostics(
        diagnostics: &[Diagnostic],
        descriptors: &[DiagnosticDescriptor],
    ) -> Self {
        let descriptor_by_code: BTreeMap<_, _> =
            descriptors.iter().map(|item| (item.code, item)).collect();
        let mut rule_codes: Vec<_> = diagnostics.iter().map(Diagnostic::code).collect();
        rule_codes.sort_unstable();
        rule_codes.dedup();
        let rules = rule_codes
            .into_iter()
            .map(|code| {
                let descriptor = descriptor_by_code.get(&code).copied();
                SarifRuleDto {
                    id: code.to_string(),
                    name: descriptor.map_or_else(|| code.to_string(), |item| item.title.to_owned()),
                    short_description: SarifMessageDto {
                        text: descriptor
                            .map_or_else(|| code.to_string(), |item| item.title.to_owned()),
                    },
                    full_description: SarifMessageDto {
                        text: descriptor.map_or_else(
                            || "EQM diagnostic".to_owned(),
                            |item| item.explanation.to_owned(),
                        ),
                    },
                    help: SarifMessageDto {
                        text: descriptor.map_or_else(
                            || "Review the EQM diagnostic.".to_owned(),
                            |item| item.remediation.to_owned(),
                        ),
                    },
                }
            })
            .collect();
        let mut sorted: Vec<_> = diagnostics.iter().collect();
        sorted.sort_unstable();
        let results = sorted
            .into_iter()
            .map(|item| {
                let mut properties = BTreeMap::new();
                if let Some(remediation) = item.remediation() {
                    properties.insert("eqm.remediation".to_owned(), remediation.to_owned());
                }
                SarifResultDto {
                    rule_id: item.code().to_string(),
                    level: severity(item.severity()).to_owned(),
                    message: SarifMessageDto {
                        text: item.message().to_owned(),
                    },
                    locations: item.source().map(location).into_iter().collect(),
                    related_locations: item.related().iter().map(location).collect(),
                    properties,
                }
            })
            .collect();
        Self {
            schema: SARIF_SCHEMA.to_owned(),
            version: "2.1.0".to_owned(),
            runs: vec![SarifRunDto {
                tool: SarifToolDto {
                    driver: SarifDriverDto {
                        name: "eqm".to_owned(),
                        version: ToolVersion::CURRENT.to_string(),
                        rules,
                    },
                },
                results,
            }],
        }
    }
}

fn severity(value: Severity) -> &'static str {
    match value {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
    }
}

fn location(value: &SourceLocation) -> SarifLocationDto {
    SarifLocationDto {
        physical_location: SarifPhysicalLocationDto {
            artifact_location: SarifArtifactLocationDto {
                uri: value.source().as_str().to_owned(),
            },
            region: SarifRegionDto {
                start_line: value.start().line(),
                start_column: value.start().column(),
                end_line: value.end().line(),
                end_column: value.end().column(),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{DiagnosticCode, SourceName, SourcePosition};

    #[test]
    fn sarif_has_one_run_and_plain_deterministic_results() -> Result<(), Box<dyn std::error::Error>>
    {
        let diagnostic = Diagnostic::new(
            DiagnosticCode::from_number(100).ok_or("code")?,
            Severity::Error,
            "invalid manifest",
            Some(SourceLocation::new(
                SourceName::new("eqm/a.toml")?,
                SourcePosition::new(1, 2)?,
                SourcePosition::new(1, 3)?,
            )?),
            Vec::new(),
            Some("correct syntax".into()),
        )?;
        let log = SarifLogDto::from_diagnostics(&[diagnostic], &[]);
        assert_eq!(log.runs.len(), 1);
        let json = serde_json::to_string(&log)?;
        assert!(json.contains("\"version\":\"2.1.0\""));
        assert!(json.contains("\"ruleId\":\"EQM-E0100\""));
        assert!(!json.contains("markdown"));
        Ok(())
    }
}
