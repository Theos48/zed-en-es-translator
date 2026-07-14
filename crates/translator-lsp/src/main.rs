use std::process::ExitCode;

use lsp_server::Connection;
use translator_core::ProviderConfiguration;
use translator_lsp::{serve, state::ProviderRuntime, ServerError};

fn main() -> ExitCode {
    if run().is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn run() -> Result<(), ServerError> {
    let configuration = ProviderConfiguration::from_env().map_err(|_| ServerError)?;
    let (provider, descriptor) = ProviderRuntime::from_configuration(configuration)
        .map_err(|_| ServerError)?
        .into_parts();
    let workspace_root = std::env::current_dir().map_err(|_| ServerError)?;
    let (connection, io_threads) = Connection::stdio();

    serve(connection, workspace_root, provider, descriptor)?;
    io_threads.join().map_err(|_| ServerError)
}
