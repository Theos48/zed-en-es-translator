use serde::{Deserialize, Serialize};

use crate::{
    ensure_provider_response_shape, ErrorCode, ProviderRequest, ProviderResponse, TranslateFailure,
};

const WIRE_VERSION: u32 = 1;

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct RunnerRequest<'a> {
    wire_version: u32,
    source_language: &'static str,
    target_language: &'static str,
    tone: &'static str,
    preserve: [&'static str; 3],
    segments: &'a [String],
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RunnerResponse {
    wire_version: u32,
    translations: Vec<String>,
}

pub(crate) fn encode_request(request: &ProviderRequest) -> Result<Vec<u8>, TranslateFailure> {
    let wire = RunnerRequest {
        wire_version: WIRE_VERSION,
        source_language: request.source_language.as_str(),
        target_language: request.target_language.as_str(),
        tone: request.tone.as_str(),
        preserve: ["markdown_structure", "code", "links"],
        segments: &request.segments,
    };
    serde_json::to_vec(&wire).map_err(|_| {
        TranslateFailure::new(
            ErrorCode::InternalError,
            "Embedded provider request encoding failed.",
        )
    })
}

pub(crate) fn decode_response(
    request: &ProviderRequest,
    output: &[u8],
) -> Result<ProviderResponse, TranslateFailure> {
    let wire: RunnerResponse = serde_json::from_slice(output).map_err(|_| invalid_response())?;
    if wire.wire_version != WIRE_VERSION {
        return Err(invalid_response());
    }
    let response = ProviderResponse {
        translated_segments: wire.translations,
    };
    ensure_provider_response_shape(request, &response)?;
    Ok(response)
}

fn invalid_response() -> TranslateFailure {
    TranslateFailure::new(
        ErrorCode::ProviderFailed,
        "Embedded provider returned an invalid response.",
    )
}
