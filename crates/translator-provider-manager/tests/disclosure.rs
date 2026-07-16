mod common;

use translator_provider_manager::disclosure::Disclosure;
use translator_provider_manager::manifest::ProviderManifest;

#[test]
fn disclosure_is_bounded_content_free_and_bound_to_exact_consent() {
    let runner = common::RUNNER;
    let artifacts = common::fixture_artifacts();
    let manifest = ProviderManifest::from_json(&common::approved_manifest(runner, &artifacts))
        .expect("approved manifest");

    let disclosure = Disclosure::from_manifest(&manifest).expect("safe disclosure");
    let output = disclosure.render();

    assert!(output.contains(common::MANIFEST_DIGEST));
    assert!(output.contains("purpose=offline_english_to_spanish_translation"));
    assert!(output.contains("runtime_source=mozilla_translations_pinned"));
    assert!(output.contains("translator-embedded-runtime@f31423c7"));
    assert!(output.contains("model=fixture-0@1;license=MPL-2.0"));
    assert!(output.contains("vocabulary=fixture-1@1;license=MPL-2.0"));
    assert!(output.contains("lexical_shortlist=fixture-2@1;license=MPL-2.0"));
    assert!(output.contains("normal_translation_network=none"));
    assert!(output.contains("scope=user_xdg_data"));
    assert!(output.contains("publication=blocked"));
    assert!(output.len() <= 2_048);
    assert!(!output.contains("/home/"));
    assert!(!output.contains("example.invalid"));
}
