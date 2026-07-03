//! MCP tool registration and execution helpers.

use std::sync::Arc;

use rmcp::{
    model::{
        object, CallToolRequestParams, CallToolResult, JsonObject, ListToolsResult,
        PaginatedRequestParams, ServerCapabilities, ServerInfo, Tool, ToolAnnotations,
    },
    service::RequestContext,
    ErrorData as McpError, RoleServer, ServerHandler,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use translator_core::{
    translate_file, translate_text, ErrorCode, MockProvider, Provider, TranslateFailure,
};

use crate::protocol::{
    error_result, success_result, translate_file_input_schema, translate_text_input_schema,
    TranslateFileParams, TranslateTextParams, TRANSLATE_FILE_TOOL_NAME, TRANSLATE_TEXT_TOOL_NAME,
};

/// MCP server that exposes the offline translator core.
#[derive(Debug, Clone)]
pub struct TranslatorMcpServer<P = MockProvider> {
    provider: P,
}

impl TranslatorMcpServer<MockProvider> {
    /// Create a server using the deterministic offline provider.
    pub fn new() -> Self {
        Self {
            provider: MockProvider::new(),
        }
    }
}

impl<P> TranslatorMcpServer<P> {
    /// Create a server with an explicit provider.
    pub fn with_provider(provider: P) -> Self {
        Self { provider }
    }
}

impl<P> TranslatorMcpServer<P>
where
    P: Provider,
{
    /// Execute `translate_text` and map expected failures to MCP tool errors.
    pub fn translate_text(&self, params: TranslateTextParams) -> CallToolResult {
        match params
            .validate()
            .and_then(|()| translate_text(&params.source_text, &self.provider))
        {
            Ok(success) => success_result(success),
            Err(failure) => error_result(failure),
        }
    }

    /// Execute `translate_file` and map expected failures to MCP tool errors.
    pub fn translate_file(&self, params: TranslateFileParams) -> CallToolResult {
        match translate_file_result(params, &self.provider) {
            Ok(success) => success_result(success),
            Err(failure) => error_result(failure),
        }
    }

    async fn translate_file_blocking(&self, params: TranslateFileParams) -> CallToolResult
    where
        P: Clone + Send + 'static,
    {
        let provider = self.provider.clone();
        match tokio::task::spawn_blocking(move || translate_file_result(params, &provider)).await {
            Ok(Ok(success)) => success_result(success),
            Ok(Err(failure)) => error_result(failure),
            Err(_) => error_result(TranslateFailure::new(
                ErrorCode::InternalError,
                "Internal server task failed.",
            )),
        }
    }
}

impl Default for TranslatorMcpServer<MockProvider> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P> ServerHandler for TranslatorMcpServer<P>
where
    P: Provider + Clone + Send + Sync + 'static,
{
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult::with_all_items(tool_definitions()))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let arguments = request.arguments.unwrap_or_default();
        match request.name.as_ref() {
            TRANSLATE_TEXT_TOOL_NAME => Ok(match decode_params::<TranslateTextParams>(arguments) {
                Ok(params) => self.translate_text(params),
                Err(failure) => error_result(failure),
            }),
            TRANSLATE_FILE_TOOL_NAME => Ok(match decode_params::<TranslateFileParams>(arguments) {
                Ok(params) => self.translate_file_blocking(params).await,
                Err(failure) => error_result(failure),
            }),
            _ => Err(McpError::invalid_params("Unknown tool.", None)),
        }
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        tool_definitions()
            .into_iter()
            .find(|tool| tool.name.as_ref() == name)
    }
}

/// Return the static MCP tool definitions for this feature.
pub fn tool_definitions() -> Vec<Tool> {
    vec![
        Tool::new(
            TRANSLATE_TEXT_TOOL_NAME,
            "Translate direct English text to Spanish using the offline provider.",
            Arc::new(object(translate_text_input_schema())),
        )
        .with_annotations(read_only_annotations("Translate text")),
        Tool::new(
            TRANSLATE_FILE_TOOL_NAME,
            "Translate an allowed Markdown or text file inside the authorized workspace.",
            Arc::new(object(translate_file_input_schema())),
        )
        .with_annotations(read_only_annotations("Translate file")),
    ]
}

fn read_only_annotations(title: &str) -> ToolAnnotations {
    ToolAnnotations::from_raw(
        Some(title.to_string()),
        Some(true),
        Some(false),
        Some(true),
        Some(false),
    )
}

fn decode_params<T>(arguments: JsonObject) -> Result<T, TranslateFailure>
where
    T: DeserializeOwned,
{
    serde_json::from_value(Value::Object(arguments))
        .map_err(|_| TranslateFailure::invalid_input("Invalid tool arguments."))
}

fn translate_file_result(
    params: TranslateFileParams,
    provider: &impl Provider,
) -> Result<translator_core::TranslateSuccess, TranslateFailure> {
    params
        .validate()
        .and_then(|()| translate_file(&params.file_path, &params.workspace_root, provider))
}
