//! CLI-owned handler for thin MCP read-tool delegation.

use crate::cli::{ParseOutcome, ParsedCli, parse};
use crate::commands;
use eqm_mcp::{McpReadToolHandler, McpToolError, ReadTool};
use serde_json::Value;
use std::fs::{self, OpenOptions};
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

    fn invoke_verify(&self, input: &Value) -> Result<Value, McpToolError> {
        let arguments = verify_arguments(input)?;
        let ParseOutcome::Run(parsed) = parse(arguments).map_err(|_| McpToolError::InvalidInput)?
        else {
            return Err(McpToolError::InvalidInput);
        };
        execute(parsed, self.start).map_err(|_| McpToolError::Invocation)
    }
}

/// Runs the current MCP server directly over process stdio.
pub fn serve_stdio(parsed: ParsedCli, start: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let allow_verify = parsed.command.options.contains_key("--allow-verify");
    let audit_path = parsed
        .command
        .options
        .get("--audit-output")
        .and_then(|values| values.first())
        .and_then(Option::as_deref)
        .map(str::to_owned);
    let request = crate::session::SessionRequest::new(parsed.global, parsed.command.name);
    let session = crate::session::prepare(&request, start)?;
    let mcp = session.mcp_session()?;
    let handler = CliReadToolHandler::new(start);
    let mut audit = audit_path
        .as_deref()
        .map(|path| open_audit(mcp.repository_root(), path))
        .transpose()?;
    let stdin = io::stdin();
    let stdout = io::stdout();
    eqm_mcp::serve(
        &mcp,
        &handler,
        BufReader::new(stdin.lock()),
        stdout.lock(),
        allow_verify,
        audit.as_mut().map(|file| file as &mut dyn io::Write),
    )?;
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
        crate::cli::CommandName::Verify => commands::verify::execute(parsed, start)?,
        _ => return Err("unsupported command reached MCP handler".into()),
    };
    Ok(execution.payload.json)
}

fn verify_arguments(input: &Value) -> Result<Vec<String>, McpToolError> {
    let object = input.as_object().ok_or(McpToolError::InvalidInput)?;
    let mut arguments = vec!["verify".to_owned()];
    for (field, option) in [
        ("unit", "--unit"),
        ("target", "--target"),
        ("baseline", "--baseline"),
    ] {
        if let Some(value) = object.get(field) {
            arguments.extend([
                option.to_owned(),
                value.as_str().ok_or(McpToolError::InvalidInput)?.to_owned(),
            ]);
        }
    }
    for (field, option) in [("affected", "--affected"), ("dry_run", "--dry-run")] {
        if object.get(field).and_then(Value::as_bool) == Some(true) {
            arguments.push(option.to_owned());
        }
    }
    Ok(arguments)
}

fn open_audit(root: &Path, relative: &str) -> Result<std::fs::File, Box<dyn std::error::Error>> {
    let relative = eqm_domain::RepoPath::new(relative)?;
    let root = root.canonicalize()?;
    let mut current = root.clone();
    let components = relative.as_str().split('/').collect::<Vec<_>>();
    for component in &components[..components.len().saturating_sub(1)] {
        current.push(component);
        let metadata = fs::symlink_metadata(&current)?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err("audit path parent must be a regular repository directory".into());
        }
    }
    let path = root.join(relative.as_str());
    if fs::symlink_metadata(&path).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return Err("audit path must not be a symlink".into());
    }
    Ok(OpenOptions::new().create(true).append(true).open(path)?)
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
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let before = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(root)
            .output()?
            .stdout;
        let baseline = root.to_string_lossy().into_owned();
        let handler = CliReadToolHandler::new(root);
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
            .current_dir(root)
            .output()?
            .stdout;
        assert_eq!(before, after);
        Ok(())
    }

    #[test]
    fn stdio_handshake_lists_and_calls_are_json_only() -> Result<(), Box<dyn std::error::Error>> {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let request = crate::session::SessionRequest::new(
            Default::default(),
            crate::cli::CommandName::McpServe,
        );
        let session = crate::session::prepare(&request, root)?;
        let mcp = session.mcp_session()?;
        let handler = CliReadToolHandler::new(root);
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
        eqm_mcp::serve(&mcp, &handler, Cursor::new(input), &mut output, false, None)?;
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

        let mut resource_families = responses[1]["result"]["resources"]
            .as_array()
            .ok_or("resources/list did not return an array")?
            .iter()
            .filter_map(|resource| resource["uri"].as_str())
            .filter_map(|uri| uri.strip_prefix("eqm://v1/").map(str::to_owned))
            .map(|path| path.split('/').next().unwrap_or_default().to_owned())
            .collect::<Vec<_>>();
        resource_families.sort();
        resource_families.dedup();
        let mut tool_names = responses[2]["result"]["tools"]
            .as_array()
            .ok_or("tools/list did not return an array")?
            .iter()
            .filter_map(|tool| tool["name"].as_str().map(str::to_owned))
            .collect::<Vec<_>>();
        tool_names.sort();
        let summary = json!({
            "called_command": responses[3]["result"]["structuredContent"]["command"],
            "protocol_version": responses[0]["result"]["protocolVersion"],
            "resource_families": resource_families,
            "tool_names": tool_names,
        });
        let mut actual = serde_json::to_vec(&summary)?;
        actual.push(b'\n');
        let golden = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/signup/goldens/mcp-workspace.json");
        assert_eq!(actual, fs::read(golden)?);
        Ok(())
    }

    #[test]
    fn verify_is_default_denied_and_explicit_authority_is_audited()
    -> Result<(), Box<dyn std::error::Error>> {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let request = crate::session::SessionRequest::new(
            Default::default(),
            crate::cli::CommandName::McpServe,
        );
        let session = crate::session::prepare(&request, root)?;
        let mcp = session.mcp_session()?;
        let handler = CliReadToolHandler::new(root);
        let input = |id| {
            [
                json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":eqm_mcp::MCP_PROTOCOL_VERSION}}),
                json!({"jsonrpc":"2.0","id":id,"method":"tools/list","params":{}}),
                json!({"jsonrpc":"2.0","id":id + 1,"method":"tools/call","params":{"name":"eqm_verify","arguments":{"dry_run":true}}}),
            ]
            .iter()
            .map(serde_json::to_string)
            .collect::<Result<Vec<_>, _>>()
            .map(|frames| frames.join("\n") + "\n")
        };

        let mut denied_output = Vec::new();
        eqm_mcp::serve(
            &mcp,
            &handler,
            Cursor::new(input(2)?),
            &mut denied_output,
            false,
            None,
        )?;
        let denied = String::from_utf8(denied_output)?
            .lines()
            .map(serde_json::from_str::<Value>)
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(
            denied[1]["result"]["tools"].as_array().map(Vec::len),
            Some(5)
        );
        assert_eq!(denied[2]["error"]["code"], -32601);

        let mut allowed_output = Vec::new();
        let mut audit = Vec::new();
        eqm_mcp::serve(
            &mcp,
            &handler,
            Cursor::new(input(5)?),
            &mut allowed_output,
            true,
            Some(&mut audit),
        )?;
        let allowed = String::from_utf8(allowed_output)?
            .lines()
            .map(serde_json::from_str::<Value>)
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(
            allowed[1]["result"]["tools"].as_array().map(Vec::len),
            Some(6)
        );
        assert_eq!(
            allowed[2]["result"]["structuredContent"]["command"],
            "verify"
        );
        let audit = String::from_utf8(audit)?
            .lines()
            .map(serde_json::from_str::<Value>)
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(audit.len(), 2);
        assert_eq!(audit[0]["decision"], "authorized");
        assert_eq!(audit[1]["decision"], "executed");
        assert!(audit.iter().all(|record| record["tool"] == "eqm_verify"));
        Ok(())
    }
}
