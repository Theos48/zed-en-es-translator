//! MCP server boundary for the translator core.

pub mod protocol;
pub mod tools;

use std::fmt;

use rmcp::ServiceExt;

/// Error returned while running the stdio MCP server.
#[derive(Debug)]
pub enum StdioServerError {
    /// The MCP server failed during initialization.
    Initialize(Box<rmcp::service::ServerInitializeError>),
    /// The background MCP service task failed.
    Join(tokio::task::JoinError),
}

impl fmt::Display for StdioServerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StdioServerError::Initialize(error) => {
                write!(formatter, "MCP initialization failed: {error}")
            }
            StdioServerError::Join(_) => {
                write!(formatter, "MCP service task failed.")
            }
        }
    }
}

impl StdioServerError {
    /// Return a redacted diagnostic string suitable for stderr.
    pub fn stderr_diagnostic(&self) -> String {
        match self {
            StdioServerError::Initialize(_) => translator_core::redact_text(&self.to_string()),
            StdioServerError::Join(_) => "MCP service task failed.".to_string(),
        }
    }
}

impl std::error::Error for StdioServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StdioServerError::Initialize(error) => Some(error.as_ref()),
            StdioServerError::Join(error) => Some(error),
        }
    }
}

/// Run the translator MCP server over stdio until the client closes the session.
pub async fn run_stdio_server() -> Result<(), StdioServerError> {
    let service = tools::TranslatorMcpServer::new()
        .serve(rmcp::transport::stdio())
        .await
        .map_err(|error| StdioServerError::Initialize(Box::new(error)))?;

    service.waiting().await.map_err(StdioServerError::Join)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::StdioServerError;

    #[tokio::test]
    async fn stderr_diagnostic_hides_join_panic_payload() {
        let join_error = tokio::spawn(async {
            panic!("plain panic payload should stay private");
        })
        .await
        .expect_err("task should panic");
        let error = StdioServerError::Join(join_error);

        assert_eq!(error.stderr_diagnostic(), "MCP service task failed.");
        assert!(!error.to_string().contains("plain panic payload"));
        assert!(!error.stderr_diagnostic().contains("plain panic payload"));
    }
}
