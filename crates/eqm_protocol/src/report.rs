//! Closed common report envelope and diagnostic transfer objects.

use eqm_domain::{
    Diagnostic, DiagnosticCode, DimensionId, Facet, FullRequirementId, ProfileId, ProfileSelection,
    Revision, Severity, Sha256Digest, SourceName, SymbolicValueId, TargetId, ToolVersion, UnitId,
    UtcInstant,
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Exact v1 result schema marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResultSchema;

impl Serialize for ResultSchema {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&crate::RESULT_SCHEMA.to_string())
    }
}

impl<'de> Deserialize<'de> for ResultSchema {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        if value == crate::RESULT_SCHEMA.to_string() {
            Ok(Self)
        } else {
            Err(serde::de::Error::custom("unsupported result schema"))
        }
    }
}

/// Exact executing tool-version marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolVersionDto;

impl Serialize for ToolVersionDto {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(ToolVersion::CURRENT.as_str())
    }
}

impl<'de> Deserialize<'de> for ToolVersionDto {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        if value == ToolVersion::CURRENT.as_str() {
            Ok(Self)
        } else {
            Err(serde::de::Error::custom("unsupported tool version"))
        }
    }
}

/// Closed command identity used by result envelopes and discriminated results.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandIdentity {
    /// Initialize a workspace.
    Init,
    /// Create an authority document.
    New,
    /// Format authored manifests.
    Fmt,
    /// Validate and finalize the graph.
    Validate,
    /// Evaluate policy without execution.
    Check,
    /// Show an entity.
    Show,
    /// Locate implementation declarations.
    Locate,
    /// Produce bounded unit context.
    Context,
    /// Produce a matrix view.
    Matrix,
    /// List obligations.
    Obligations,
    /// Compare semantic graphs.
    Diff,
    /// Compute affected authority.
    Affected,
    /// Invoke an inventory adapter.
    Discover,
    /// Reconcile intended and observed facts.
    Reconcile,
    /// Execute approved evidence runners.
    Verify,
    /// Produce an attestation.
    Attest,
    /// Evaluate an exact release.
    ReleaseCheck,
    /// Explain a diagnostic.
    Explain,
    /// Run non-executing health checks.
    Doctor,
    /// Update immutable lock entries.
    LockUpdate,
    /// Serve the MCP stdio adapter.
    McpServe,
}

/// Closed evaluation mode.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationModeDto {
    /// Local development evaluation.
    Development,
    /// Pull-request evaluation.
    PullRequest,
    /// Release evaluation.
    Release,
}

/// One selected profile dimension, sortable by profile then dimension.
#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileValueDto {
    profile: String,
    revision: u64,
    dimension: String,
    value: String,
}

impl ProfileValueDto {
    /// Expands a domain profile selection into sorted protocol records.
    #[must_use]
    pub fn from_selection(selection: &ProfileSelection) -> Vec<Self> {
        selection
            .values()
            .iter()
            .map(|(dimension, value)| Self {
                profile: selection.profile().as_str().to_owned(),
                revision: selection.revision().get(),
                dimension: dimension.as_str().to_owned(),
                value: value.as_str().to_owned(),
            })
            .collect()
    }
}

/// Explicit invocation context. Subject and baseline shapes are supplied by
/// their owning command DTOs and cannot be untyped maps through this API.
#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InvocationContextDto<S, B> {
    mode: EvaluationModeDto,
    profiles: Vec<ProfileValueDto>,
    subject: Option<S>,
    baseline: Option<B>,
    offline: bool,
    evaluated_at: String,
}

impl<S, B> InvocationContextDto<S, B> {
    /// Creates a context with deterministic, duplicate-free profile values.
    pub fn new(
        mode: EvaluationModeDto,
        mut profiles: Vec<ProfileValueDto>,
        subject: Option<S>,
        baseline: Option<B>,
        offline: bool,
        evaluated_at: UtcInstant,
    ) -> Result<Self, ReportBuildError> {
        profiles.sort_unstable();
        let original = profiles.len();
        profiles.dedup();
        if profiles.len() != original {
            return Err(ReportBuildError::DuplicateProfileValue);
        }
        Ok(Self {
            mode,
            profiles,
            subject,
            baseline,
            offline,
            evaluated_at: evaluated_at.to_string(),
        })
    }

    fn normalize(mut self) -> Result<Self, ReportBuildError> {
        let _: UtcInstant = self
            .evaluated_at
            .parse()
            .map_err(|_| ReportBuildError::InvalidContext)?;
        for profile in &self.profiles {
            let _: ProfileId = profile
                .profile
                .parse()
                .map_err(|_| ReportBuildError::InvalidContext)?;
            Revision::new(profile.revision).map_err(|_| ReportBuildError::InvalidContext)?;
            let _: DimensionId = profile
                .dimension
                .parse()
                .map_err(|_| ReportBuildError::InvalidContext)?;
            let _: SymbolicValueId = profile
                .value
                .parse()
                .map_err(|_| ReportBuildError::InvalidContext)?;
        }
        self.profiles.sort_unstable();
        let original = self.profiles.len();
        self.profiles.dedup();
        if self.profiles.len() != original {
            return Err(ReportBuildError::DuplicateProfileValue);
        }
        Ok(self)
    }
}

/// One one-based source position.
#[derive(
    Clone, Copy, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(deny_unknown_fields)]
pub struct SourcePositionDto {
    line: u32,
    column: u32,
}

/// One source span in CLI protocol output.
#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceLocationDto {
    uri: String,
    start: SourcePositionDto,
    end: SourcePositionDto,
}

impl SourceLocationDto {
    /// Converts one validated domain source span to its public wire shape.
    #[must_use]
    pub fn from_domain(value: &eqm_domain::SourceLocation) -> Self {
        let source = value.source().as_str();
        Self {
            uri: if source.starts_with("eqm://") {
                source.to_owned()
            } else {
                format!("file:{source}")
            },
            start: SourcePositionDto {
                line: value.start().line(),
                column: value.start().column(),
            },
            end: SourcePositionDto {
                line: value.end().line(),
                column: value.end().column(),
            },
        }
    }
}

/// Closed diagnostic severity on the wire.
#[derive(
    Clone, Copy, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SeverityDto {
    /// Blocking error.
    Error,
    /// Non-blocking warning.
    Warning,
    /// Informational note.
    Note,
}

impl From<Severity> for SeverityDto {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Error => Self::Error,
            Severity::Warning => Self::Warning,
            Severity::Note => Self::Note,
        }
    }
}

/// Public diagnostic record with every optional semantic coordinate explicit.
#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticDto {
    code: String,
    severity: SeverityDto,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<SourceLocationDto>,
    related: Vec<SourceLocationDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requirement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    obligation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remediation: Option<String>,
}

impl DiagnosticDto {
    /// Converts a domain diagnostic without inventing absent semantic coordinates.
    #[must_use]
    pub fn from_domain(value: &Diagnostic) -> Self {
        Self {
            code: value.code().to_string(),
            severity: value.severity().into(),
            message: value.message().to_owned(),
            source: value.source().map(SourceLocationDto::from_domain),
            related: value
                .related()
                .iter()
                .map(SourceLocationDto::from_domain)
                .collect(),
            unit: None,
            target: None,
            requirement: None,
            obligation: None,
            facet: None,
            status: None,
            remediation: value.remediation().map(str::to_owned),
        }
    }

    fn normalize(mut self) -> Result<Self, ReportBuildError> {
        let _: DiagnosticCode = self
            .code
            .parse()
            .map_err(|_| ReportBuildError::InvalidDiagnostic)?;
        if !bounded_text(&self.message, 4_096)
            || self
                .remediation
                .as_deref()
                .is_some_and(|value| !bounded_text(value, 4_096))
            || self
                .unit
                .as_deref()
                .is_some_and(|value| value.parse::<UnitId>().is_err())
            || self
                .target
                .as_deref()
                .is_some_and(|value| value.parse::<TargetId>().is_err())
            || self
                .requirement
                .as_deref()
                .is_some_and(|value| value.parse::<FullRequirementId>().is_err())
            || self
                .facet
                .as_deref()
                .is_some_and(|value| value.parse::<Facet>().is_err())
            || self
                .obligation
                .as_deref()
                .is_some_and(|value| !bounded_text(value, 320))
            || self
                .status
                .as_deref()
                .is_some_and(|value| !bounded_token(value))
            || self
                .source
                .as_ref()
                .is_some_and(|value| !valid_location(value))
            || self.related.iter().any(|value| !valid_location(value))
        {
            return Err(ReportBuildError::InvalidDiagnostic);
        }
        self.related.sort_unstable();
        self.related.dedup();
        Ok(self)
    }
}

fn bounded_text(value: &str, maximum: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum
        && !value
            .chars()
            .any(|character| character.is_control() && character != '\n')
}

fn bounded_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn valid_location(value: &SourceLocationDto) -> bool {
    let source = value
        .uri
        .strip_prefix("file:")
        .or_else(|| value.uri.strip_prefix("eqm://").map(|_| value.uri.as_str()));
    source.is_some_and(|source| SourceName::new(source).is_ok())
        && value.start.line > 0
        && value.start.column > 0
        && value.end.line > 0
        && value.end.column > 0
        && value.start <= value.end
}

impl Ord for DiagnosticDto {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (
            self.severity,
            &self.code,
            &self.source,
            &self.unit,
            &self.target,
            &self.requirement,
            &self.facet,
            &self.message,
        )
            .cmp(&(
                other.severity,
                &other.code,
                &other.source,
                &other.unit,
                &other.target,
                &other.requirement,
                &other.facet,
                &other.message,
            ))
    }
}

impl PartialOrd for DiagnosticDto {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Implemented by every closed command-result DTO.
pub trait CommandResultDto {
    /// Returns the command shape owned by this result type.
    fn command(&self) -> CommandIdentity;

    /// Returns the serialized discriminant, which normally equals [`Self::command`].
    fn declared_command(&self) -> CommandIdentity {
        self.command()
    }
}

/// Common JSON response envelope.
#[derive(Clone, Debug, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReportEnvelope<R, S, B> {
    #[schemars(with = "String")]
    schema: ResultSchema,
    #[schemars(with = "String")]
    tool_version: ToolVersionDto,
    command: CommandIdentity,
    workspace_digest: Option<String>,
    context: InvocationContextDto<S, B>,
    result: Option<R>,
    diagnostics: Vec<DiagnosticDto>,
}

impl<R: CommandResultDto, S, B> ReportEnvelope<R, S, B> {
    /// Creates a deterministic envelope and checks its result discriminant.
    pub fn new(
        command: CommandIdentity,
        workspace_digest: Option<Sha256Digest>,
        context: InvocationContextDto<S, B>,
        result: Option<R>,
        mut diagnostics: Vec<DiagnosticDto>,
    ) -> Result<Self, ReportBuildError> {
        if result.as_ref().is_some_and(|value| {
            value.command() != command || value.declared_command() != value.command()
        }) {
            return Err(ReportBuildError::CommandMismatch);
        }
        diagnostics.sort_unstable();
        Ok(Self {
            schema: ResultSchema,
            tool_version: ToolVersionDto,
            command,
            workspace_digest: workspace_digest.map(|value| value.to_string()),
            context,
            result,
            diagnostics,
        })
    }

    /// Serializes one compact UTF-8 JSON document.
    pub fn to_json(&self) -> Result<Vec<u8>, ReportBuildError>
    where
        R: Serialize,
        S: Serialize,
        B: Serialize,
    {
        serde_json::to_vec(self).map_err(|_| ReportBuildError::Json)
    }

    /// Parses an exact envelope, rejecting unknown fields and mismatched commands.
    pub fn from_json(bytes: &[u8]) -> Result<Self, ReportBuildError>
    where
        R: DeserializeOwned,
        S: DeserializeOwned,
        B: DeserializeOwned,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw<R, S, B> {
            schema: ResultSchema,
            tool_version: ToolVersionDto,
            command: CommandIdentity,
            workspace_digest: Option<String>,
            context: InvocationContextDto<S, B>,
            result: Option<R>,
            diagnostics: Vec<DiagnosticDto>,
        }
        let raw: Raw<R, S, B> =
            serde_json::from_slice(bytes).map_err(|_| ReportBuildError::Json)?;
        let digest = raw
            .workspace_digest
            .as_deref()
            .map(str::parse)
            .transpose()
            .map_err(|_| ReportBuildError::InvalidWorkspaceDigest)?;
        let context = raw.context.normalize()?;
        let diagnostics = raw
            .diagnostics
            .into_iter()
            .map(DiagnosticDto::normalize)
            .collect::<Result<Vec<_>, _>>()?;
        let _ = (raw.schema, raw.tool_version);
        Self::new(raw.command, digest, context, raw.result, diagnostics)
    }
}

/// Report construction or exact JSON failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReportBuildError {
    /// A profile/dimension coordinate repeated.
    DuplicateProfileValue,
    /// Result and envelope command discriminants differed.
    CommandMismatch,
    /// Workspace digest was malformed.
    InvalidWorkspaceDigest,
    /// Invocation context contained an invalid or noncanonical value.
    InvalidContext,
    /// Diagnostic content violated its closed field constraints.
    InvalidDiagnostic,
    /// JSON was malformed, non-current, or contained unknown fields.
    Json,
}

impl Display for ReportBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for ReportBuildError {}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{DiagnosticCode, SourceLocation, SourceName, SourcePosition};

    #[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
    #[serde(deny_unknown_fields)]
    struct ValidateResult {
        kind: CommandIdentity,
        valid: bool,
    }

    impl CommandResultDto for ValidateResult {
        fn command(&self) -> CommandIdentity {
            self.kind
        }
    }

    fn context() -> Result<InvocationContextDto<(), ()>, Box<dyn Error>> {
        Ok(InvocationContextDto::new(
            EvaluationModeDto::Development,
            Vec::new(),
            None,
            None,
            true,
            "2026-08-07T12:00:00Z".parse()?,
        )?)
    }

    #[test]
    fn golden_envelope_round_trips_with_exact_fields() -> Result<(), Box<dyn Error>> {
        let report = ReportEnvelope::new(
            CommandIdentity::Validate,
            None,
            context()?,
            Some(ValidateResult {
                kind: CommandIdentity::Validate,
                valid: true,
            }),
            Vec::new(),
        )?;
        let bytes = report.to_json()?;
        assert_eq!(
            String::from_utf8(bytes.clone())?,
            concat!(
                "{\"schema\":\"https://schemas.equivalencematrix.dev/v1/result\",",
                "\"tool_version\":\"0.1.0\",\"command\":\"validate\",",
                "\"workspace_digest\":null,\"context\":{\"mode\":\"development\",",
                "\"profiles\":[],\"subject\":null,\"baseline\":null,\"offline\":true,",
                "\"evaluated_at\":\"2026-08-07T12:00:00Z\"},",
                "\"result\":{\"kind\":\"validate\",\"valid\":true},\"diagnostics\":[]}"
            )
        );
        assert_eq!(
            ReportEnvelope::<ValidateResult, (), ()>::from_json(&bytes)?,
            report
        );
        Ok(())
    }

    #[test]
    fn diagnostics_convert_and_sort_deterministically() -> Result<(), Box<dyn Error>> {
        let location = SourceLocation::new(
            SourceName::new("eqm/account.toml")?,
            SourcePosition::new(2, 3)?,
            SourcePosition::new(2, 8)?,
        )?;
        let warning = Diagnostic::new(
            DiagnosticCode::from_number(101).ok_or("code")?,
            Severity::Warning,
            "warning",
            Some(location),
            Vec::new(),
            None,
        )?;
        let error = Diagnostic::new(
            DiagnosticCode::from_number(100).ok_or("code")?,
            Severity::Error,
            "error",
            None,
            Vec::new(),
            None,
        )?;
        let report = ReportEnvelope::<ValidateResult, (), ()>::new(
            CommandIdentity::Validate,
            None,
            context()?,
            None,
            vec![
                DiagnosticDto::from_domain(&warning),
                DiagnosticDto::from_domain(&error),
            ],
        )?;
        let json = String::from_utf8(report.to_json()?)?;
        assert!(json.find("EQM-E0100").ok_or("first")? < json.find("EQM-E0101").ok_or("next")?);
        assert!(json.contains("file:eqm/account.toml"));
        Ok(())
    }

    #[test]
    fn unknown_fields_versions_and_command_mismatch_fail() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            ReportEnvelope::new(
                CommandIdentity::Check,
                None,
                context()?,
                Some(ValidateResult {
                    kind: CommandIdentity::Validate,
                    valid: true,
                }),
                Vec::new(),
            ),
            Err(ReportBuildError::CommandMismatch)
        );
        let unknown = br#"{"schema":"https://schemas.equivalencematrix.dev/v1/result","tool_version":"0.1.0","command":"validate","workspace_digest":null,"context":{"mode":"development","profiles":[],"subject":null,"baseline":null,"offline":true,"evaluated_at":"2026-08-07T12:00:00Z"},"result":null,"diagnostics":[],"extra":true}"#;
        assert_eq!(
            ReportEnvelope::<ValidateResult, (), ()>::from_json(unknown),
            Err(ReportBuildError::Json)
        );
        Ok(())
    }
}
