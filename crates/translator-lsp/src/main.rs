use std::process::ExitCode;

use lsp_server::Connection;
use translator_core::EmbeddedProcessProvider;
use translator_lsp::{serve, ServerError};

fn main() -> ExitCode {
    if run().is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn run() -> Result<(), ServerError> {
    let provider = EmbeddedProcessProvider::from_current_executable().map_err(|_| ServerError)?;
    let workspace_root = std::env::current_dir().map_err(|_| ServerError)?;
    let (connection, io_threads) = Connection::stdio();

    serve(connection, workspace_root, provider)?;
    io_threads.join().map_err(|_| ServerError)
}
