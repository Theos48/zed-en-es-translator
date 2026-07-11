use std::io::Read;

use translator_core::{
    redact_failure, translate_file_with_confirmation, translate_text_with_confirmation, ErrorCode,
    ProviderSelection, TranslateFailure, TranslateRequest, TranslateResult, TranslateSuccess,
    MAX_INPUT_BYTES,
};

fn main() {
    let code = run();
    std::process::exit(code);
}

fn run() -> i32 {
    if std::env::args_os().len() > 1 {
        write_failure(TranslateFailure::new(
            ErrorCode::InvalidInput,
            "Command-line arguments are not accepted.",
        ));
        return 1;
    }

    let mut input_bytes = Vec::new();
    let read_result = std::io::stdin()
        .take((MAX_INPUT_BYTES + 1) as u64)
        .read_to_end(&mut input_bytes);
    if read_result.is_err() {
        write_failure(TranslateFailure::new(
            ErrorCode::InvalidInput,
            "Failed to read request input.",
        ));
        return 1;
    }
    if input_bytes.len() > MAX_INPUT_BYTES {
        write_failure(TranslateFailure::new(
            ErrorCode::InvalidInput,
            "Request input exceeds the configured size limit.",
        ));
        return 1;
    }
    let input = match String::from_utf8(input_bytes) {
        Ok(input) => input,
        Err(_) => {
            write_failure(TranslateFailure::new(
                ErrorCode::NonUtf8Input,
                "The input must be UTF-8 text.",
            ));
            return 1;
        }
    };

    let request = match TranslateRequest::from_json(&input) {
        Ok(request) => request,
        Err(failure) => {
            write_failure(failure);
            return 1;
        }
    };

    let provider = match ProviderSelection::from_env() {
        Ok(provider) => provider,
        Err(failure) => {
            write_failure(failure);
            return 1;
        }
    };
    let result: Result<TranslateSuccess, TranslateFailure> =
        if let Some(source_text) = request.source_text.as_deref() {
            translate_text_with_confirmation(source_text, &provider, request.remote_confirmed)
        } else if let (Some(file_path), Some(workspace_root)) = (
            request.file_path.as_deref(),
            request.workspace_root.as_deref(),
        ) {
            translate_file_with_confirmation(
                file_path,
                workspace_root,
                &provider,
                request.remote_confirmed,
            )
        } else {
            Err(TranslateFailure::new(
                ErrorCode::InvalidInput,
                "Request shape is invalid.",
            ))
        };

    match result {
        Ok(success) => {
            print!("{}", TranslateResult::Success(success).to_json());
            0
        }
        Err(failure) => {
            write_failure(failure);
            1
        }
    }
}

fn write_failure(failure: TranslateFailure) {
    let failure = redact_failure(failure);
    let error_code = failure.code.as_str();
    print!("{}", TranslateResult::Failure(failure).to_json());
    eprintln!("error_code={error_code}");
}
