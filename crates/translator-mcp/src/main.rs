#[tokio::main]
async fn main() {
    if let Err(error) = translator_mcp::run_stdio_server().await {
        eprintln!("error_code=INTERNAL_ERROR");
        eprintln!(
            "diagnostic={}",
            translator_core::redact_text(&error.to_string())
        );
        std::process::exit(1);
    }
}
