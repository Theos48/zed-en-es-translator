use std::process::ExitCode;

use lsp_server::Connection;
use translator_core::{ProviderConfiguration, ProviderSelection};
use translator_lsp::{serve, state::ProviderDescriptor, ServerError};

fn main() -> ExitCode {
    if run().is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn run() -> Result<(), ServerError> {
    let configuration = ProviderConfiguration::from_env().map_err(|_| ServerError)?;
    let descriptor = ProviderDescriptor::from_configuration(&configuration);
    let provider = ProviderSelection::from_configuration(configuration).map_err(|_| ServerError)?;
    let workspace_root = std::env::current_dir().map_err(|_| ServerError)?;
    let (connection, io_threads) = Connection::stdio();

    serve(connection, workspace_root, provider, descriptor)?;
    io_threads.join().map_err(|_| ServerError)
}
