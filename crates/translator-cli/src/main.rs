use std::io::Read;

use translator_core::{
    redact_failure, translate_file, translate_text, ErrorCode, MockProvider, TranslateFailure,
    TranslateRequest, TranslateResult, TranslateSuccess,
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

    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_err() {
        write_failure(TranslateFailure::new(
            ErrorCode::InvalidInput,
            "Failed to read request input.",
        ));
        return 1;
    }

    let request = match TranslateRequest::from_json(&input) {
        Ok(request) => request,
        Err(failure) => {
            write_failure(failure);
            return 1;
        }
    };

    let provider = MockProvider::new();
    let result: Result<TranslateSuccess, TranslateFailure> =
        if let Some(source_text) = request.source_text.as_deref() {
            translate_text(source_text, &provider)
        } else if let (Some(file_path), Some(workspace_root)) = (
            request.file_path.as_deref(),
            request.workspace_root.as_deref(),
        ) {
            translate_file(file_path, workspace_root, &provider)
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
    print!("{}", TranslateResult::Failure(failure.clone()).to_json());
    eprintln!("error_code={}", failure.code.as_str());
}
