mod common;

use serde_json::Value;
use translator_core::{ProviderConfiguration, ProviderSelection};
use translator_mcp::protocol::{translate_text_input_schema, TranslateTextParams};
use translator_mcp::tools::TranslatorMcpServer;

#[test]
fn schema_exposes_optional_remote_confirmation() {
    let schema = translate_text_input_schema();

    assert_eq!(
        schema["properties"]["remote_confirmed"]["type"],
        Value::String("boolean".to_string())
    );
}

#[test]
fn mcp_denies_unconfirmed_non_local_provider() {
    let value =
        translate_text_with_remote_provider(common::translate_text_params("Read the docs."));

    common::assert_tool_error_code(&value, "REMOTE_CONFIRMATION_REQUIRED");
}

#[test]
fn mcp_blocks_confirmed_non_local_secret_before_contact() {
    let mut params = common::translate_text_params("API_KEY=fake_test_key_123456");
    params.remote_confirmed = Some(true);
    let value = translate_text_with_remote_provider(params);

    common::assert_tool_error_code_redacts(&value, "SECRET_DETECTED", "fake_test_key");
}

fn translate_text_with_remote_provider(params: TranslateTextParams) -> Value {
    let config = ProviderConfiguration::from_values(
        Some("libretranslate"),
        Some("https://translations.example.invalid"),
        None,
        Some("true"),
    )
    .expect("provider config");
    let provider = ProviderSelection::from_configuration(config).expect("provider selection");
    serde_json::to_value(TranslatorMcpServer::with_provider(provider).translate_text(params))
        .expect("tool result json")
}
