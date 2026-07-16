const MAIN_SOURCE: &str = include_str!("../src/main.rs");

#[test]
fn server_constructs_the_adjacent_embedded_provider_directly() {
    assert!(
        MAIN_SOURCE.contains("EmbeddedProcessProvider::from_current_executable()"),
        "translator-lsp must construct its adjacent embedded provider directly"
    );
}

#[test]
fn server_has_no_runtime_provider_configuration_surface() {
    for retired_surface in [
        "ProviderConfiguration",
        "ProviderSelection",
        "TRANSLATOR_PROVIDER",
        "from_env()",
    ] {
        assert!(
            !MAIN_SOURCE.contains(retired_surface),
            "translator-lsp still references retired runtime configuration: {retired_surface}"
        );
    }
}
