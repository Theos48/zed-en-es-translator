//! Shared integration test helpers for the MCP server crate.

// Each integration test compiles this module separately and uses a different helper subset.
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use rmcp::model::CallToolResult;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use translator_core::Provider;
use translator_mcp::protocol::{TranslateFileParams, TranslateTextParams};
use translator_mcp::tools::TranslatorMcpServer;

const READ_TIMEOUT: Duration = Duration::from_secs(5);

pub struct McpServerProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl McpServerProcess {
    pub async fn initialize(&mut self) -> Result<Value, Box<dyn std::error::Error>> {
        self.send_json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-11-25",
                "capabilities": {},
                "clientInfo": {
                    "name": "translator-mcp-test",
                    "version": "0.0.0"
                }
            }
        }))
        .await?;
        let response = self.read_response_for_id(1).await?;
        self.send_json(&json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }))
        .await?;
        Ok(response)
    }

    pub async fn send_json(&mut self, message: &Value) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string(message)?;
        self.stdin.write_all(serialized.as_bytes()).await?;
        self.stdin.write_all(b"\n").await?;
        self.stdin.flush().await?;
        Ok(())
    }

    pub async fn read_response_for_id(
        &mut self,
        expected_id: u64,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let response = tokio::time::timeout(READ_TIMEOUT, async {
            loop {
                let mut line = String::new();
                let bytes = self.stdout.read_line(&mut line).await?;
                if bytes == 0 {
                    return Err::<Value, Box<dyn std::error::Error>>(
                        "server stdout closed before response".into(),
                    );
                }
                let value: Value = serde_json::from_str(&line)?;
                if value.get("id").and_then(Value::as_u64) == Some(expected_id) {
                    return Ok::<Value, Box<dyn std::error::Error>>(value);
                }
            }
        })
        .await??;

        Ok(response)
    }
}

impl Drop for McpServerProcess {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}

pub fn spawn_server() -> McpServerProcess {
    let mut child = Command::new(env!("CARGO_BIN_EXE_translator-mcp"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("spawn translator-mcp");

    let stdin = child.stdin.take().expect("server stdin");
    let stdout = child.stdout.take().expect("server stdout");

    McpServerProcess {
        child,
        stdin,
        stdout: BufReader::new(stdout),
    }
}

pub fn temp_case(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_mcp_{name}_{}_{}",
        std::process::id(),
        unique_suffix()
    ));
    fs::create_dir_all(&root).expect("temp root");
    root
}

pub fn write_file(path: &Path, content: impl AsRef<[u8]>) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent dir");
    }
    fs::write(path, content).expect("write file");
}

pub fn translate_text_params(source_text: &str) -> TranslateTextParams {
    TranslateTextParams {
        source_text: source_text.to_string(),
        source_language: Some("en".to_string()),
        target_language: Some("es".to_string()),
        tone: Some("technical_neutral".to_string()),
        preserve_formatting: Some(true),
    }
}

pub fn translate_file_params(workspace: &Path, file_path: &str) -> TranslateFileParams {
    TranslateFileParams {
        workspace_root: workspace.to_string_lossy().into_owned(),
        file_path: file_path.to_string(),
        source_language: Some("en".to_string()),
        target_language: Some("es".to_string()),
        tone: Some("technical_neutral".to_string()),
        preserve_formatting: Some(true),
    }
}

pub fn translate_text_error_value(params: TranslateTextParams) -> Value {
    tool_result_value(TranslatorMcpServer::new().translate_text(params))
}

pub fn translate_text_error_value_with_provider(provider: impl Provider) -> Value {
    tool_result_value(
        TranslatorMcpServer::with_provider(provider)
            .translate_text(translate_text_params("Read the docs.")),
    )
}

pub fn translate_file_error_value(params: TranslateFileParams) -> Value {
    tool_result_value(TranslatorMcpServer::new().translate_file(params))
}

pub fn assert_tool_error_code(value: &Value, code: &str) {
    assert_eq!(value["isError"], true);
    assert_eq!(value["structuredContent"]["code"], code);
}

pub fn assert_tool_error_code_redacts(value: &Value, code: &str, forbidden: &str) {
    assert_tool_error_code(value, code);
    assert!(!value.to_string().contains(forbidden));
}

fn tool_result_value(result: CallToolResult) -> Value {
    serde_json::to_value(result).expect("serialize tool result")
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos()
}
