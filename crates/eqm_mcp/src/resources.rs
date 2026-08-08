//! Closed current-version MCP resource URI and payload mapping.

use crate::PreparedMcpSession;
use eqm_domain::{SourceLocation, SourceName, SourcePosition, UnitId, UtcInstant};
use eqm_protocol::{
    CheckResultDto, CommandIdentity, ContextResultDto, EntityReferenceDto, EvaluationModeDto,
    FacetStatusDto, InvocationContextDto, ReportEnvelope, ResultStatusDto, ShowResultDto,
    SourceLocationDto, ValidateResultDto,
};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// One closed v1 EQM resource identity.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum McpResourceUri {
    /// Finalized workspace summary.
    Workspace,
    /// Exact semantic unit.
    Unit(UnitId),
    /// Bounded exact semantic unit context.
    Context(UnitId),
    /// Prepared findings view.
    Findings,
}

impl Display for McpResourceUri {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Workspace => formatter.write_str("eqm://v1/workspace"),
            Self::Unit(id) => write!(formatter, "eqm://v1/unit/{}", encode(id.as_str())),
            Self::Context(id) => write!(formatter, "eqm://v1/context/{}", encode(id.as_str())),
            Self::Findings => formatter.write_str("eqm://v1/findings"),
        }
    }
}

impl FromStr for McpResourceUri {
    type Err = McpResourceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let path = value
            .strip_prefix("eqm://v1/")
            .ok_or(McpResourceError::InvalidUri)?;
        match path {
            "workspace" => Ok(Self::Workspace),
            "findings" => Ok(Self::Findings),
            _ => {
                let (kind, encoded) = path.split_once('/').ok_or(McpResourceError::InvalidUri)?;
                if encoded.is_empty() || encoded.contains('/') {
                    return Err(McpResourceError::InvalidUri);
                }
                let id = UnitId::new(decode(encoded)?).map_err(|_| McpResourceError::InvalidUri)?;
                match kind {
                    "unit" => Ok(Self::Unit(id)),
                    "context" => Ok(Self::Context(id)),
                    _ => Err(McpResourceError::InvalidUri),
                }
            }
        }
    }
}

/// Explicit trust label for resource content.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceTrust {
    /// Finalized EQM authority only.
    TrustedAuthority,
    /// Authority plus product-derived records labeled inside the payload.
    Mixed,
}

/// One exact JSON resource content document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpResource {
    /// Exact current resource URI.
    pub uri: McpResourceUri,
    /// Resource trust classification.
    pub trust: ResourceTrust,
    /// Common protocol result-envelope JSON.
    pub text: String,
}

/// Lists the complete stable resource catalog for one finalized session.
#[must_use]
pub fn list_resources(session: &PreparedMcpSession<'_>) -> BTreeSet<McpResourceUri> {
    let graph = session.finalized().graph();
    let units = graph
        .capabilities()
        .keys()
        .map(ToString::to_string)
        .chain(graph.journeys().keys().map(ToString::to_string))
        .chain(graph.surfaces().keys().map(ToString::to_string))
        .chain(graph.fragments().keys().map(|(id, _)| id.to_string()))
        .filter_map(|id| UnitId::new(id).ok());
    BTreeSet::from([McpResourceUri::Workspace, McpResourceUri::Findings])
        .into_iter()
        .chain(units.clone().map(McpResourceUri::Unit))
        .chain(units.map(McpResourceUri::Context))
        .collect()
}

/// Reads one resource as a common EQM protocol result envelope.
pub fn read_resource(
    session: &PreparedMcpSession<'_>,
    uri: &McpResourceUri,
    evaluated_at: UtcInstant,
) -> Result<McpResource, McpResourceError> {
    let context = || {
        InvocationContextDto::<(), ()>::new(
            EvaluationModeDto::Development,
            Vec::new(),
            None,
            None,
            true,
            evaluated_at,
        )
        .map_err(|_| McpResourceError::Protocol)
    };
    let (text, trust) = match uri {
        McpResourceUri::Workspace => {
            let graph = session.finalized().graph();
            let counts = BTreeMap::from([
                ("bindings".to_owned(), count(graph.bindings().len())?),
                (
                    "capabilities".to_owned(),
                    count(graph.capabilities().len())?,
                ),
                ("fragments".to_owned(), count(graph.fragments().len())?),
                ("journeys".to_owned(), count(graph.journeys().len())?),
                ("policies".to_owned(), count(graph.policies().len())?),
                ("profiles".to_owned(), count(graph.profiles().len())?),
                ("surfaces".to_owned(), count(graph.surfaces().len())?),
                ("targets".to_owned(), count(graph.targets().len())?),
            ]);
            let result = ValidateResultDto {
                kind: CommandIdentity::Validate,
                valid: true,
                entity_counts: counts,
                graph_digest: session.workspace_digest().to_string(),
            };
            (
                ReportEnvelope::new(
                    CommandIdentity::Validate,
                    Some(session.workspace_digest()),
                    context()?,
                    Some(result),
                    Vec::new(),
                )
                .map_err(|_| McpResourceError::Protocol)?
                .to_json()
                .map_err(|_| McpResourceError::Protocol)?,
                ResourceTrust::TrustedAuthority,
            )
        }
        McpResourceUri::Unit(id) => {
            let entity = entity(session, id)?;
            let location = resource_location(uri)?;
            let result = ShowResultDto {
                kind: CommandIdentity::Show,
                entity_kind: entity.kind.clone(),
                entity_id: entity.id.clone(),
                source: location,
                entity,
            };
            (
                ReportEnvelope::new(
                    CommandIdentity::Show,
                    Some(session.workspace_digest()),
                    context()?,
                    Some(result),
                    Vec::new(),
                )
                .map_err(|_| McpResourceError::Protocol)?
                .to_json()
                .map_err(|_| McpResourceError::Protocol)?,
                ResourceTrust::TrustedAuthority,
            )
        }
        McpResourceUri::Context(id) => {
            let authority = entity(session, id)?;
            let result = ContextResultDto {
                kind: CommandIdentity::Context,
                unit: id.to_string(),
                target: None,
                authority,
                product_data: json!({"records":[],"trust":"untrusted_product_data"}),
                obligations: BTreeSet::new(),
                evidence: json!({"records":[],"trust":"untrusted_tool_output"}),
                findings: BTreeSet::new(),
                waivers: json!({"records":[],"trust":"protected_authority_required"}),
                truncated: false,
                omitted_bytes: 0,
            };
            (
                ReportEnvelope::new(
                    CommandIdentity::Context,
                    Some(session.workspace_digest()),
                    context()?,
                    Some(result),
                    Vec::new(),
                )
                .map_err(|_| McpResourceError::Protocol)?
                .to_json()
                .map_err(|_| McpResourceError::Protocol)?,
                ResourceTrust::Mixed,
            )
        }
        McpResourceUri::Findings => {
            let result = CheckResultDto {
                kind: CommandIdentity::Check,
                status: ResultStatusDto::Partial,
                obligation_counts: BTreeMap::from([(FacetStatusDto::Unknown, 0)]),
                findings: BTreeSet::new(),
            };
            (
                ReportEnvelope::new(
                    CommandIdentity::Check,
                    Some(session.workspace_digest()),
                    context()?,
                    Some(result),
                    Vec::new(),
                )
                .map_err(|_| McpResourceError::Protocol)?
                .to_json()
                .map_err(|_| McpResourceError::Protocol)?,
                ResourceTrust::TrustedAuthority,
            )
        }
    };
    Ok(McpResource {
        uri: uri.clone(),
        trust,
        text: String::from_utf8(text).map_err(|_| McpResourceError::Protocol)?,
    })
}

fn entity(
    session: &PreparedMcpSession<'_>,
    id: &UnitId,
) -> Result<EntityReferenceDto, McpResourceError> {
    let graph = session.finalized().graph();
    let value = id.as_str();
    if graph
        .capabilities()
        .keys()
        .any(|item| item.as_str() == value)
    {
        return Ok(EntityReferenceDto {
            kind: "capability".to_owned(),
            id: value.to_owned(),
            revision: None,
            digest: None,
        });
    }
    if let Some(item) = graph
        .journeys()
        .values()
        .find(|item| item.id().as_str() == value)
    {
        return Ok(EntityReferenceDto {
            kind: "journey".to_owned(),
            id: value.to_owned(),
            revision: Some(item.revision().get()),
            digest: None,
        });
    }
    if let Some(item) = graph
        .surfaces()
        .values()
        .find(|item| item.id().as_str() == value)
    {
        return Ok(EntityReferenceDto {
            kind: "surface".to_owned(),
            id: value.to_owned(),
            revision: Some(item.revision().get()),
            digest: None,
        });
    }
    if let Some(item) = graph
        .fragments()
        .values()
        .find(|item| item.id().as_str() == value)
    {
        return Ok(EntityReferenceDto {
            kind: "fragment".to_owned(),
            id: value.to_owned(),
            revision: Some(item.revision().get()),
            digest: None,
        });
    }
    Err(McpResourceError::NotFound)
}

fn resource_location(uri: &McpResourceUri) -> Result<SourceLocationDto, McpResourceError> {
    let position = SourcePosition::new(1, 1).map_err(|_| McpResourceError::Protocol)?;
    let source = SourceName::new(uri.to_string()).map_err(|_| McpResourceError::Protocol)?;
    let location =
        SourceLocation::new(source, position, position).map_err(|_| McpResourceError::Protocol)?;
    Ok(SourceLocationDto::from_domain(&location))
}

fn encode(value: &str) -> String {
    value
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-') {
                char::from(byte).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

fn decode(value: &str) -> Result<String, McpResourceError> {
    let bytes = value.as_bytes();
    let mut output = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let encoded = bytes
                .get(index + 1..index + 3)
                .ok_or(McpResourceError::InvalidUri)?;
            let text = std::str::from_utf8(encoded).map_err(|_| McpResourceError::InvalidUri)?;
            output.push(u8::from_str_radix(text, 16).map_err(|_| McpResourceError::InvalidUri)?);
            index += 3;
        } else {
            if !bytes[index].is_ascii_alphanumeric() && !matches!(bytes[index], b'.' | b'_' | b'-')
            {
                return Err(McpResourceError::InvalidUri);
            }
            output.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(output).map_err(|_| McpResourceError::InvalidUri)
}

fn count(value: usize) -> Result<u64, McpResourceError> {
    u64::try_from(value).map_err(|_| McpResourceError::Protocol)
}

/// Resource URI, lookup, or protocol construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpResourceError {
    /// URI scheme, version, path, or percent encoding is invalid.
    InvalidUri,
    /// Exact semantic unit is absent.
    NotFound,
    /// A common result envelope could not be constructed.
    Protocol,
}
impl Display for McpResourceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "MCP resource error: {self:?}")
    }
}
impl Error for McpResourceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uri_parser_is_current_closed_and_round_trips_encoded_units() -> Result<(), Box<dyn Error>> {
        for uri in [
            "eqm://v1/workspace",
            "eqm://v1/findings",
            "eqm://v1/unit/account.create.signup",
            "eqm://v1/context/account.create.signup",
        ] {
            let parsed: McpResourceUri = uri.parse()?;
            assert_eq!(parsed.to_string(), uri);
        }
        for invalid in [
            "http://v1/workspace",
            "eqm://v2/workspace",
            "eqm://v1/unit/",
            "eqm://v1/unit/a/b",
            "eqm://v1/unknown",
        ] {
            assert!(invalid.parse::<McpResourceUri>().is_err());
        }
        Ok(())
    }
}
