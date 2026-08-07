//! Digest-pinned adapter request, response, and inventory DTOs.

#![allow(missing_docs)]

use crate::{
    ADAPTER_REQUEST_SCHEMA, ADAPTER_RESPONSE_SCHEMA, DiagnosticDto, EvidenceSubjectDto,
    INVENTORY_SCHEMA,
};
use eqm_domain::{FactValue, Inventory, InventoryEntry};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterOperationDto {
    Discover,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterLimitsDto {
    pub timeout_ms: u64,
    pub max_input_bytes: u64,
    pub max_output_bytes: u64,
    pub max_entries: u64,
    pub max_depth: u64,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterRequestDto {
    pub schema: String,
    pub request_id: String,
    pub adapter: String,
    pub adapter_digest: String,
    pub operation: AdapterOperationDto,
    pub subject: EvidenceSubjectDto,
    pub target: String,
    pub target_root: String,
    pub limits: AdapterLimitsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(
    tag = "type",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum FactValueDto {
    Boolean(bool),
    Integer(i64),
    Symbol(String),
    Text(String),
}

impl From<&FactValue> for FactValueDto {
    fn from(value: &FactValue) -> Self {
        match value {
            FactValue::Boolean(value) => Self::Boolean(*value),
            FactValue::Integer(value) => Self::Integer(*value),
            FactValue::Symbol(value) => Self::Symbol(value.as_str().to_owned()),
            FactValue::Text(value) => Self::Text(value.as_str().to_owned()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryEntryDto {
    pub kind: String,
    pub key: String,
    pub attributes: BTreeMap<String, FactValueDto>,
    pub source: String,
}

impl From<&InventoryEntry> for InventoryEntryDto {
    fn from(value: &InventoryEntry) -> Self {
        Self {
            kind: value.kind().as_str().to_owned(),
            key: value.key().as_str().to_owned(),
            attributes: value
                .attributes()
                .iter()
                .map(|(key, value)| (key.as_str().to_owned(), value.into()))
                .collect(),
            source: value.source().as_str().to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryDto {
    pub schema: String,
    pub adapter: String,
    pub adapter_digest: String,
    pub subject: EvidenceSubjectDto,
    pub target: String,
    pub generated_at: String,
    pub completeness: String,
    pub entries: Vec<InventoryEntryDto>,
    pub diagnostics: Vec<DiagnosticDto>,
    pub inventory_digest: String,
}

impl From<&Inventory> for InventoryDto {
    fn from(value: &Inventory) -> Self {
        let mut diagnostics: Vec<_> = value
            .diagnostics()
            .iter()
            .map(DiagnosticDto::from_domain)
            .collect();
        diagnostics.sort_unstable();
        Self {
            schema: INVENTORY_SCHEMA.to_string(),
            adapter: value.adapter().as_str().to_owned(),
            adapter_digest: value.adapter_digest().to_string(),
            subject: value.subject().into(),
            target: value.target().as_str().to_owned(),
            generated_at: value.generated_at().to_string(),
            completeness: value.completeness().to_string(),
            entries: value
                .entries()
                .values()
                .map(InventoryEntryDto::from)
                .collect(),
            diagnostics,
            inventory_digest: value.inventory_digest().to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterStatusDto {
    Ok,
    Partial,
    Error,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterResponseDto {
    pub schema: String,
    pub request_id: String,
    pub adapter: String,
    pub adapter_digest: String,
    pub status: AdapterStatusDto,
    pub inventory: Option<InventoryDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterDtoError {
    Json,
    InvalidSchema,
    InvalidLimits,
    InvalidStatus,
    RequestMismatch,
}

impl AdapterRequestDto {
    pub fn from_json(bytes: &[u8]) -> Result<Self, AdapterDtoError> {
        if bytes.len() > 4 * 1024 * 1024 {
            return Err(AdapterDtoError::InvalidLimits);
        }
        let value: Self = serde_json::from_slice(bytes).map_err(|_| AdapterDtoError::Json)?;
        if value.schema != ADAPTER_REQUEST_SCHEMA.to_string() {
            return Err(AdapterDtoError::InvalidSchema);
        }
        if value.limits.timeout_ms < 1_000
            || value.limits.timeout_ms > 3_600_000
            || value.limits.max_input_bytes == 0
            || value.limits.max_input_bytes > 4 * 1024 * 1024
            || value.limits.max_output_bytes == 0
            || value.limits.max_output_bytes > 16 * 1024 * 1024
            || value.limits.max_entries == 0
            || value.limits.max_entries > 250_000
            || value.limits.max_depth == 0
            || value.limits.max_depth > 64
        {
            return Err(AdapterDtoError::InvalidLimits);
        }
        Ok(value)
    }
}

impl AdapterResponseDto {
    pub fn from_json(bytes: &[u8]) -> Result<Self, AdapterDtoError> {
        if bytes.len() > 16 * 1024 * 1024 {
            return Err(AdapterDtoError::InvalidLimits);
        }
        let mut value: Self = serde_json::from_slice(bytes).map_err(|_| AdapterDtoError::Json)?;
        if value.schema != ADAPTER_RESPONSE_SCHEMA.to_string() {
            return Err(AdapterDtoError::InvalidSchema);
        }
        if matches!(value.status, AdapterStatusDto::Error) == value.inventory.is_some() {
            return Err(AdapterDtoError::InvalidStatus);
        }
        if value
            .inventory
            .as_ref()
            .is_some_and(|inventory| inventory.entries.len() > 250_000)
        {
            return Err(AdapterDtoError::InvalidLimits);
        }
        value.diagnostics.sort_unstable();
        Ok(value)
    }

    pub fn matches_request(&self, request: &AdapterRequestDto) -> Result<(), AdapterDtoError> {
        if self.request_id != request.request_id
            || self.adapter != request.adapter
            || self.adapter_digest != request.adapter_digest
            || self.inventory.as_ref().is_some_and(|inventory| {
                inventory.subject != request.subject || inventory.target != request.target
            })
        {
            Err(AdapterDtoError::RequestMismatch)
        } else {
            Ok(())
        }
    }
}

impl std::fmt::Display for AdapterDtoError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl std::error::Error for AdapterDtoError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_status_controls_inventory_presence() {
        let value = br#"{"schema":"https://schemas.equivalencematrix.dev/v1/adapter-response","request_id":"run-1","adapter":"source.rust","adapter_digest":"sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","status":"error","inventory":{},"diagnostics":[]}"#;
        assert_eq!(
            AdapterResponseDto::from_json(value),
            Err(AdapterDtoError::Json)
        );
        let value = br#"{"schema":"https://schemas.equivalencematrix.dev/v1/adapter-response","request_id":"run-1","adapter":"source.rust","adapter_digest":"sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","status":"error","inventory":null,"diagnostics":[]}"#;
        assert!(AdapterResponseDto::from_json(value).is_ok());
    }
}
