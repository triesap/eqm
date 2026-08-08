//! Closed read-only tool schemas and delegation boundary.

use eqm_protocol::RESULT_SCHEMA;
use serde_json::{Map, Value, json};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Closed read-only MCP tool identity.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReadTool {
    /// Bounded unit context.
    Context,
    /// Complete matrix query.
    Matrix,
    /// Conservative affected analysis.
    Affected,
    /// Non-executing conformance check.
    Check,
    /// Diagnostic registry explanation.
    Explain,
}

impl ReadTool {
    /// Returns the exact public MCP tool name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Context => "eqm_context",
            Self::Matrix => "eqm_matrix",
            Self::Affected => "eqm_affected",
            Self::Check => "eqm_check",
            Self::Explain => "eqm_explain",
        }
    }
}

/// CLI-owned delegation that reuses existing command orchestration.
pub trait McpReadToolHandler {
    /// Invokes one already validated read-only tool.
    fn invoke(&self, tool: ReadTool, input: &Value) -> Result<Value, McpToolError>;
}

/// Structured authoritative tool result.
#[derive(Clone, Debug, PartialEq)]
pub struct McpToolResult {
    /// Common EQM result envelope.
    pub structured_content: Value,
}

/// Returns closed JSON Schemas for every read-only v1 tool.
#[must_use]
pub fn read_tool_schemas() -> BTreeMap<&'static str, Value> {
    [
        (
            ReadTool::Context,
            schema(&["unit"], &["unit", "target", "max_bytes", "max_depth"]),
        ),
        (
            ReadTool::Matrix,
            schema(&["kind"], &["kind", "unit", "target"]),
        ),
        (
            ReadTool::Affected,
            schema(&["baseline"], &["baseline", "paths"]),
        ),
        (ReadTool::Check, schema(&[], &["units", "targets"])),
        (ReadTool::Explain, schema(&["code"], &["code"])),
    ]
    .into_iter()
    .map(|(tool, schema)| (tool.name(), schema))
    .collect()
}

/// Validates a closed input, delegates to shared CLI orchestration, and validates its envelope.
pub fn call_read_tool(
    handler: &impl McpReadToolHandler,
    name: &str,
    input: &Value,
) -> Result<McpToolResult, McpToolError> {
    let tool = parse_tool(name)?;
    validate_input(tool, input)?;
    let structured_content = handler.invoke(tool, input)?;
    validate_envelope(tool, &structured_content)?;
    Ok(McpToolResult { structured_content })
}

fn parse_tool(name: &str) -> Result<ReadTool, McpToolError> {
    [
        ReadTool::Context,
        ReadTool::Matrix,
        ReadTool::Affected,
        ReadTool::Check,
        ReadTool::Explain,
    ]
    .into_iter()
    .find(|tool| tool.name() == name)
    .ok_or(McpToolError::UnknownTool)
}

fn validate_input(tool: ReadTool, input: &Value) -> Result<(), McpToolError> {
    let object = input.as_object().ok_or(McpToolError::InvalidInput)?;
    let (required, allowed): (&[&str], &[&str]) = match tool {
        ReadTool::Context => (&["unit"], &["unit", "target", "max_bytes", "max_depth"]),
        ReadTool::Matrix => (&["kind"], &["kind", "unit", "target"]),
        ReadTool::Affected => (&["baseline"], &["baseline", "paths"]),
        ReadTool::Check => (&[], &["units", "targets"]),
        ReadTool::Explain => (&["code"], &["code"]),
    };
    if object.keys().any(|key| !allowed.contains(&key.as_str()))
        || required.iter().any(|key| !object.contains_key(*key))
    {
        return Err(McpToolError::InvalidInput);
    }
    for (key, value) in object {
        let valid = match key.as_str() {
            "max_bytes" | "max_depth" => value.as_u64().is_some(),
            "paths" | "units" | "targets" => value
                .as_array()
                .is_some_and(|values| values.iter().all(Value::is_string)),
            _ => value.as_str().is_some_and(|value| !value.is_empty()),
        };
        if !valid {
            return Err(McpToolError::InvalidInput);
        }
    }
    Ok(())
}

fn validate_envelope(tool: ReadTool, value: &Value) -> Result<(), McpToolError> {
    let object = value.as_object().ok_or(McpToolError::InvalidResult)?;
    let command = match tool {
        ReadTool::Context => "context",
        ReadTool::Matrix => "matrix",
        ReadTool::Affected => "affected",
        ReadTool::Check => "check",
        ReadTool::Explain => "explain",
    };
    if object.get("schema").and_then(Value::as_str) != Some(&RESULT_SCHEMA.to_string())
        || object.get("command").and_then(Value::as_str) != Some(command)
        || object.get("result").is_none()
    {
        return Err(McpToolError::InvalidResult);
    }
    Ok(())
}

fn schema(required: &[&str], allowed: &[&str]) -> Value {
    let properties = allowed
        .iter()
        .map(|name| {
            let property = match *name {
                "max_bytes" | "max_depth" => json!({"type":"integer","minimum":1}),
                "paths" | "units" | "targets" => {
                    json!({"type":"array","items":{"type":"string"},"uniqueItems":true})
                }
                _ => json!({"type":"string","minLength":1}),
            };
            ((*name).to_owned(), property)
        })
        .collect::<Map<_, _>>();
    json!({"type":"object","properties":properties,"required":required,"additionalProperties":false})
}

/// Read-tool routing, input, delegation, or output failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpToolError {
    /// Tool name is outside the closed read-only set.
    UnknownTool,
    /// Tool input is not a closed valid projection.
    InvalidInput,
    /// CLI orchestration rejected the call.
    Invocation,
    /// Delegated output is not the corresponding common result envelope.
    InvalidResult,
}

impl Display for McpToolError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "MCP tool error: {self:?}")
    }
}
impl Error for McpToolError {}

#[cfg(test)]
mod tests {
    use super::*;

    struct Handler;
    impl McpReadToolHandler for Handler {
        fn invoke(&self, tool: ReadTool, _input: &Value) -> Result<Value, McpToolError> {
            Ok(
                json!({"schema":RESULT_SCHEMA.to_string(),"command":tool.name().trim_start_matches("eqm_"),"result":{}}),
            )
        }
    }

    #[test]
    fn schemas_are_closed_and_unknown_fields_tools_and_results_fail() {
        for schema in read_tool_schemas().values() {
            assert_eq!(schema["additionalProperties"], false);
        }
        assert!(call_read_tool(&Handler, "eqm_explain", &json!({"code":"EQM-E0300"})).is_ok());
        assert_eq!(
            call_read_tool(
                &Handler,
                "eqm_explain",
                &json!({"code":"EQM-E0300","extra":true})
            ),
            Err(McpToolError::InvalidInput)
        );
        assert_eq!(
            call_read_tool(&Handler, "eqm_unknown", &json!({})),
            Err(McpToolError::UnknownTool)
        );
    }
}
