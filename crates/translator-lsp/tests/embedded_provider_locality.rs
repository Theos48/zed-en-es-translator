use translator_core::{ErrorCode, ProviderConfiguration};
use translator_lsp::state::{ProviderDescriptor, ProviderRuntime};

#[test]
fn embedded_configuration_is_offline_and_never_prompts_for_remote_confirmation() {
    let configuration =
        ProviderConfiguration::from_values(Some("embedded_local"), None, None, None)
            .expect("embedded configuration");
    let descriptor = ProviderDescriptor::from_configuration(&configuration);

    assert_eq!(
        descriptor.action_title(),
        "Translate English to Spanish [offline]"
    );
    assert!(!descriptor.allow_remote());

    let failure = ProviderRuntime::from_configuration(configuration)
        .expect_err("unready installation must fail closed");
    assert_eq!(failure.code, ErrorCode::ProviderNotConfigured);
    assert!(!format!("{failure:?}").contains("/home/"));
}
