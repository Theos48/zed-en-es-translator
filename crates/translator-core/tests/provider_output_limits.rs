use translator_core::{
    translate_text, ErrorCode, Provider, ProviderRequest, ProviderResponse, TranslateFailure,
    MAX_OUTPUT_BYTES,
};

struct OversizedProvider;

impl Provider for OversizedProvider {
    fn translate(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        Ok(ProviderResponse {
            translated_segments: request
                .segments
                .iter()
                .map(|_| "a".repeat(MAX_OUTPUT_BYTES + 1))
                .collect(),
        })
    }
}

#[test]
fn rejects_provider_output_above_limit() {
    let err = translate_text("Read the docs.", &OversizedProvider)
        .expect_err("oversized provider output should fail");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
}
