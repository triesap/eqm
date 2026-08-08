//! CLI-owned handler for thin MCP read-tool delegation.

use crate::cli::{ParseOutcome, ParsedCli, parse};
use crate::commands;
use eqm_mcp::{McpReadToolHandler, McpToolError, ReadTool};
use serde_json::Value;
use std::io::{self, BufReader};
use std::path::Path;

/// Reuses the exact CLI parse and command implementation paths for MCP reads.
pub struct CliReadToolHandler<'a> {
    start: &'a Path,
}

impl<'a> CliReadToolHandler<'a> {
    /// Creates a handler rooted at the current CLI workspace location.
    #[must_use]
    pub const fn new(start: &'a Path) -> Self {
        Self { start }
    }
}

impl McpReadToolHandler for CliReadToolHandler<'_> {
    fn invoke(&self, tool: ReadTool, input: &Value) -> Result<Value, McpToolError> {
        let arguments = arguments(tool, input)?;
        let ParseOutcome::Run(parsed) = parse(arguments).map_err(|_| McpToolError::InvalidInput)?
        else {
            return Err(McpToolError::InvalidInput);
        };
        execute(parsed, self.start).map_err(|_| McpToolError::Invocation)
    }
}

/// Runs the current MCP server directly over process stdio.
pub fn serve_stdio(parsed: ParsedCli, start: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let request = crate::session::SessionRequest::new(parsed.global, parsed.command.name);
    let session = crate::session::prepare(&request, start)?;
    let mcp = session.mcp_session()?;
    let handler = CliReadToolHandler::new(start);
    let stdin = io::stdin();
    let stdout = io::stdout();
    eqm_mcp::serve(&mcp, &handler, BufReader::new(stdin.lock()), stdout.lock())?;
    Ok(())
}

fn execute(parsed: ParsedCli, start: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let command = parsed.command.name;
    let execution = match command {
        crate::cli::CommandName::Context => commands::context::execute(parsed, start)?,
        crate::cli::CommandName::Matrix => commands::matrix::execute(parsed, start)?,
        crate::cli::CommandName::Affected => commands::affected::execute(parsed, start)?,
        crate::cli::CommandName::Check => commands::check::execute(parsed, start)?,
        crate::cli::CommandName::Explain => commands::explain::execute(parsed)?,
        _ => return Err("non-read command reached MCP handler".into()),
    };
    Ok(execution.payload.json)
}

fn arguments(tool: ReadTool, input: &Value) -> Result<Vec<String>, McpToolError> {
    let object = input.as_object().ok_or(McpToolError::InvalidInput)?;
    let string = |name| object.get(name).and_then(Value::as_str).map(str::to_owned);
    let mut arguments = match tool {
        ReadTool::Context => vec![
            "context".to_owned(),
            string("unit").ok_or(McpToolError::InvalidInput)?,
        ],
        ReadTool::Matrix => vec![
            "matrix".to_owned(),
            string("kind").ok_or(McpToolError::InvalidInput)?,
        ],
        ReadTool::Affected => vec![
            "--baseline".to_owned(),
            string("baseline").ok_or(McpToolError::InvalidInput)?,
            "affected".to_owned(),
        ],
        ReadTool::Check => vec!["check".to_owned()],
        ReadTool::Explain => vec![
            "explain".to_owned(),
            string("code").ok_or(McpToolError::InvalidInput)?,
        ],
    };
    for (field, option) in [
        ("target", "--target"),
        ("unit", "--unit"),
        ("max_bytes", "--max-bytes"),
        ("max_depth", "--max-depth"),
    ] {
        if (tool == ReadTool::Context && field == "unit")
            || (tool == ReadTool::Matrix && field == "kind")
        {
            continue;
        }
        if let Some(value) = object.get(field) {
            let value = value
                .as_str()
                .map(str::to_owned)
                .or_else(|| value.as_u64().map(|value| value.to_string()))
                .ok_or(McpToolError::InvalidInput)?;
            arguments.extend([option.to_owned(), value]);
        }
    }
    for (field, option) in [
        ("paths", "--path"),
        ("units", "--unit"),
        ("targets", "--target"),
    ] {
        if let Some(values) = object.get(field).and_then(Value::as_array) {
            for value in values {
                arguments.extend([
                    option.to_owned(),
                    value.as_str().ok_or(McpToolError::InvalidInput)?.to_owned(),
                ]);
            }
        }
    }
    Ok(arguments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_mcp::call_read_tool;
    use serde_json::json;
    use std::io::Cursor;
    use std::process::Command;

    #[test]
    fn every_read_tool_reuses_cli_envelopes_without_writes()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let before = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&root)
            .output()?
            .stdout;
        let baseline = root.to_string_lossy().into_owned();
        let handler = CliReadToolHandler::new(&root);
        for (name, input) in [
            (
                "eqm_context",
                json!({"unit":"account.create.signup.identifier"}),
            ),
            ("eqm_matrix", json!({"kind":"conformance"})),
            (
                "eqm_affected",
                json!({"baseline":baseline,"paths":["README.md"]}),
            ),
            ("eqm_check", json!({})),
            ("eqm_explain", json!({"code":"EQM-E0300"})),
        ] {
            let result = call_read_tool(&handler, name, &input)
                .map_err(|error| format!("{name}: {error}"))?;
            assert_eq!(
                result.structured_content["command"],
                name.trim_start_matches("eqm_")
            );
        }
        let after = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&root)
            .output()?
            .stdout;
        assert_eq!(before, after);
        Ok(())
    }

    #[test]
    fn stdio_handshake_lists_and_calls_are_json_only() -> Result<(), Box<dyn std::error::Error>> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let request = crate::session::SessionRequest::new(
            Default::default(),
            crate::cli::CommandName::McpServe,
        );
        let session = crate::session::prepare(&request, &root)?;
        let mcp = session.mcp_session()?;
        let handler = CliReadToolHandler::new(&root);
        let frames = [
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":eqm_mcp::MCP_PROTOCOL_VERSION}}),
            json!({"jsonrpc":"2.0","method":"notifications/initialized","params":{}}),
            json!({"jsonrpc":"2.0","id":2,"method":"resources/list","params":{}}),
            json!({"jsonrpc":"2.0","id":3,"method":"tools/list","params":{}}),
            json!({"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"eqm_explain","arguments":{"code":"EQM-E0300"}}}),
        ];
        let input = frames
            .iter()
            .map(serde_json::to_string)
            .collect::<Result<Vec<_>, _>>()?
            .join("\n")
            + "\n";
        let mut output = Vec::new();
        eqm_mcp::serve(&mcp, &handler, Cursor::new(input), &mut output)?;
        let text = String::from_utf8(output)?;
        let responses = text
            .lines()
            .map(serde_json::from_str::<Value>)
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(responses.len(), 4);
        assert_eq!(
            responses[0]["result"]["protocolVersion"],
            eqm_mcp::MCP_PROTOCOL_VERSION
        );
        assert!(
            responses[1]["result"]["resources"]
                .as_array()
                .is_some_and(|value| !value.is_empty())
        );
        assert_eq!(
            responses[2]["result"]["tools"].as_array().map(Vec::len),
            Some(5)
        );
        assert_eq!(
            responses[3]["result"]["structuredContent"]["command"],
            "explain"
        );
        Ok(())
    }
}
