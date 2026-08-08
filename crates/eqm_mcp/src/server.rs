//! Current-version line-framed JSON-RPC MCP stdio server.

use crate::{
    McpReadToolHandler, McpResourceUri, PreparedMcpSession, call_read_tool, call_verify_tool,
    list_resources, read_resource, read_tool_schemas, verify_tool_schema,
};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::Sha256Digest;
use eqm_domain::UtcInstant;
use serde_json::{Map, Value, json};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::{BufRead, Write};
use std::time::SystemTime;

/// The sole supported v1 MCP protocol revision.
pub const MCP_PROTOCOL_VERSION: &str = "2025-06-18";
const MAX_FRAME_BYTES: usize = 1024 * 1024;

/// Serves newline-delimited JSON-RPC until EOF without writing logs to protocol output.
pub fn serve(
    session: &PreparedMcpSession<'_>,
    handler: &impl McpReadToolHandler,
    mut input: impl BufRead,
    mut output: impl Write,
    allow_verify: bool,
    mut audit: Option<&mut dyn Write>,
) -> Result<(), McpServerError> {
    let mut initialized = false;
    let mut frame = Vec::new();
    loop {
        frame.clear();
        let count = input
            .read_until(b'\n', &mut frame)
            .map_err(|_| McpServerError::Io)?;
        if count == 0 {
            break;
        }
        if frame.len() > MAX_FRAME_BYTES {
            write_response(
                &mut output,
                error(Value::Null, -32600, "frame exceeds v1 limit"),
            )?;
            continue;
        }
        while matches!(frame.last(), Some(b'\n' | b'\r')) {
            frame.pop();
        }
        if frame.is_empty() {
            write_response(
                &mut output,
                error(Value::Null, -32700, "empty JSON-RPC frame"),
            )?;
            continue;
        }
        let request: Value = match serde_json::from_slice(&frame) {
            Ok(value) => value,
            Err(_) => {
                write_response(&mut output, error(Value::Null, -32700, "malformed JSON"))?;
                continue;
            }
        };
        let Some(object) = request.as_object() else {
            write_response(
                &mut output,
                error(Value::Null, -32600, "request must be an object"),
            )?;
            continue;
        };
        let id = object.get("id").cloned();
        let notification = id.is_none();
        let response = dispatch(
            session,
            handler,
            object,
            &mut initialized,
            allow_verify,
            &mut audit,
        );
        if !notification {
            write_response(&mut output, response.unwrap_or_else(|response| response))?;
        }
    }
    Ok(())
}

fn dispatch(
    session: &PreparedMcpSession<'_>,
    handler: &impl McpReadToolHandler,
    object: &Map<String, Value>,
    initialized: &mut bool,
    allow_verify: bool,
    audit: &mut Option<&mut dyn Write>,
) -> Result<Value, Value> {
    let id = object.get("id").cloned().unwrap_or(Value::Null);
    if object
        .keys()
        .any(|key| !matches!(key.as_str(), "jsonrpc" | "id" | "method" | "params"))
        || object.get("jsonrpc").and_then(Value::as_str) != Some("2.0")
        || object.get("method").and_then(Value::as_str).is_none()
        || object
            .get("id")
            .is_some_and(|value| !(value.is_string() || value.is_number()))
    {
        return Err(error(id, -32600, "invalid JSON-RPC request"));
    }
    let method = object["method"]
        .as_str()
        .ok_or_else(|| error(id.clone(), -32600, "invalid method"))?;
    let params = object.get("params").cloned().unwrap_or_else(|| json!({}));
    if method == "initialize" {
        let version = params
            .as_object()
            .and_then(|value| value.get("protocolVersion"))
            .and_then(Value::as_str);
        if version != Some(MCP_PROTOCOL_VERSION) {
            return Err(error(id, -32602, "unsupported MCP protocol version"));
        }
        *initialized = true;
        return Ok(success(
            id,
            json!({
                "protocolVersion":MCP_PROTOCOL_VERSION,
                "capabilities":{"resources":{"listChanged":false,"subscribe":false},"tools":{"listChanged":false}},
                "serverInfo":{"name":"eqm","version":env!("CARGO_PKG_VERSION")}
            }),
        ));
    }
    if !*initialized {
        return Err(error(id, -32002, "server is not initialized"));
    }
    match method {
        "notifications/initialized" => Ok(success(id, json!({}))),
        "ping" => Ok(success(id, json!({}))),
        "resources/list" => Ok(success(
            id,
            json!({"resources":list_resources(session).into_iter().map(|uri| json!({"uri":uri.to_string(),"name":uri.to_string(),"mimeType":"application/json"})).collect::<Vec<_>>() }),
        )),
        "resources/read" => {
            let uri = params
                .as_object()
                .and_then(|value| value.get("uri"))
                .and_then(Value::as_str)
                .ok_or_else(|| error(id.clone(), -32602, "resource URI required"))?;
            let uri: McpResourceUri = uri
                .parse()
                .map_err(|_| error(id.clone(), -32602, "invalid resource URI"))?;
            let resource = read_resource(
                session,
                &uri,
                evaluated_at().map_err(|_| error(id.clone(), -32603, "clock unavailable"))?,
            )
            .map_err(|_| error(id.clone(), -32602, "resource unavailable"))?;
            Ok(success(
                id,
                json!({"contents":[{"uri":resource.uri.to_string(),"mimeType":"application/json","text":resource.text}]}),
            ))
        }
        "tools/list" => {
            let mut tools = read_tool_schemas()
                .into_iter()
                .map(|(name, input_schema)| json!({"name":name,"inputSchema":input_schema}))
                .collect::<Vec<_>>();
            if allow_verify {
                tools.push(json!({"name":"eqm_verify","inputSchema":verify_tool_schema()}));
            }
            Ok(success(id, json!({"tools":tools})))
        }
        "tools/call" => {
            let params = params
                .as_object()
                .ok_or_else(|| error(id.clone(), -32602, "tool params required"))?;
            if params
                .keys()
                .any(|key| !matches!(key.as_str(), "name" | "arguments"))
            {
                return Err(error(id, -32602, "unknown tool call field"));
            }
            let name = params
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| error(id.clone(), -32602, "tool name required"))?;
            let arguments = params
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));
            let result = if name == "eqm_verify" {
                if !allow_verify || audit.is_none() {
                    return Err(error(id, -32601, "tool not available"));
                }
                write_audit(session, audit, &arguments, "authorized")
                    .map_err(|_| error(id.clone(), -32603, "audit unavailable"))?;
                let result = call_verify_tool(handler, &arguments);
                write_audit(
                    session,
                    audit,
                    &arguments,
                    if result.is_ok() {
                        "executed"
                    } else {
                        "rejected"
                    },
                )
                .map_err(|_| error(id.clone(), -32603, "audit unavailable"))?;
                result.map_err(|_| error(id.clone(), -32602, "tool call rejected"))?
            } else {
                call_read_tool(handler, name, &arguments)
                    .map_err(|_| error(id.clone(), -32602, "tool call rejected"))?
            };
            Ok(success(
                id,
                json!({"content":[],"structuredContent":result.structured_content,"isError":false}),
            ))
        }
        _ => Err(error(id, -32601, "method not found")),
    }
}

fn write_audit(
    session: &PreparedMcpSession<'_>,
    audit: &mut Option<&mut dyn Write>,
    arguments: &Value,
    decision: &str,
) -> Result<(), McpServerError> {
    let request = serde_json::to_vec(arguments).map_err(|_| McpServerError::Protocol)?;
    let record = json!({
        "tool":"eqm_verify",
        "decision":decision,
        "request_digest":Sha256Digest::hash_content(&request).to_string(),
        "workspace_digest":session.workspace_digest().to_string()
    });
    let mut bytes = serde_json::to_vec(&record).map_err(|_| McpServerError::Protocol)?;
    bytes.push(b'\n');
    let sink = audit.as_deref_mut().ok_or(McpServerError::Io)?;
    sink.write_all(&bytes).map_err(|_| McpServerError::Io)?;
    sink.flush().map_err(|_| McpServerError::Io)
}

fn success(id: Value, result: Value) -> Value {
    json!({"jsonrpc":"2.0","id":id,"result":result})
}
fn error(id: Value, code: i64, message: &str) -> Value {
    json!({"jsonrpc":"2.0","id":id,"error":{"code":code,"message":message}})
}

fn write_response(output: &mut impl Write, response: Value) -> Result<(), McpServerError> {
    serde_json::to_writer(&mut *output, &response).map_err(|_| McpServerError::Protocol)?;
    output.write_all(b"\n").map_err(|_| McpServerError::Io)?;
    output.flush().map_err(|_| McpServerError::Io)
}

fn evaluated_at() -> Result<UtcInstant, McpServerError> {
    let value: DateTime<Utc> = SystemTime::now().into();
    value
        .to_rfc3339_opts(SecondsFormat::Secs, true)
        .parse()
        .map_err(|_| McpServerError::Protocol)
}

/// Stdio transport or frame-construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpServerError {
    /// Input or output failed.
    Io,
    /// A response could not be encoded.
    Protocol,
}
impl Display for McpServerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "MCP server error: {self:?}")
    }
}
impl Error for McpServerError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_version_and_malformed_frames_fail_as_pure_json() {
        let responses = [
            error(json!(1), -32602, "unsupported MCP protocol version"),
            error(Value::Null, -32700, "malformed JSON"),
        ];
        for response in responses {
            assert!(serde_json::to_vec(&response).is_ok());
        }
    }
}
