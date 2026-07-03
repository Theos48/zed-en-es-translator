#[tokio::main]
async fn main() {
    if let Err(error) = translator_mcp::run_stdio_server().await {
        eprintln!("error_code=INTERNAL_ERROR");
        eprintln!("diagnostic={}", error.stderr_diagnostic());
        std::process::exit(1);
    }
}
