mod common;

use translator_core::ProviderConfiguration;
use translator_lsp::state::ProviderDescriptor;

use common::{code_action_params, range, TestClient};

#[test]
fn derives_safe_locality_labels_from_provider_configuration() {
    let offline = ProviderConfiguration::from_values(None, None, None, None).expect("mock");
    let local = ProviderConfiguration::from_values(
        Some("libretranslate"),
        Some("http://127.0.0.1:5000"),
        None,
        None,
    )
    .expect("local");
    let remote = ProviderConfiguration::from_values(
        Some("libretranslate"),
        Some("https://private-provider.example.invalid"),
        Some("PRIVATE_API_KEY_ENV"),
        Some("true"),
    )
    .expect("remote");

    assert_eq!(
        ProviderDescriptor::from_configuration(&offline).action_title(),
        "Translate English to Spanish [offline]"
    );
    assert_eq!(
        ProviderDescriptor::from_configuration(&local).action_title(),
        "Translate English to Spanish [local]"
    );
    let remote_descriptor = ProviderDescriptor::from_configuration(&remote);
    assert_eq!(
        remote_descriptor.action_title(),
        "Translate English to Spanish [remote - confirmation required]"
    );
    let debug = format!("{remote_descriptor:?}");
    assert!(!debug.contains("private-provider"));
    assert!(!debug.contains("PRIVATE_API_KEY_ENV"));
}

#[test]
fn code_action_exposes_only_the_safe_locality_label() {
    for (descriptor, expected) in [
        (ProviderDescriptor::local(), "[local]"),
        (
            ProviderDescriptor::remote(true),
            "[remote - confirmation required]",
        ),
    ] {
        let mut client = TestClient::new(descriptor);
        let uri = "file:///workspace/readme.md";
        client.open(uri, 1, "markdown", "Read the docs.");
        let response = client.request(
            "textDocument/codeAction",
            code_action_params(uri, range(0, 14)),
        );
        let wire = response.result.expect("actions").to_string();
        assert!(wire.contains(expected));
        assert!(!wire.contains("http"));
        client.shutdown();
    }
}
