use translator_core::ErrorCode;

const RESULT_SCHEMA: &str = include_str!(
    "../../../specs/009-zed-marketplace-install/contracts/translate-result.schema.json"
);

#[test]
fn exposes_complete_stable_error_code_list() {
    let codes: Vec<&'static str> = ErrorCode::ALL.iter().map(ErrorCode::as_str).collect();

    assert_eq!(
        codes,
        vec![
            "INVALID_INPUT",
            "UNSUPPORTED_LANGUAGE_PAIR",
            "UNSUPPORTED_FILE_TYPE",
            "FILE_TOO_LARGE",
            "FILE_NOT_FOUND",
            "PATH_NOT_ALLOWED",
            "NON_UTF8_INPUT",
            "NO_TRANSLATABLE_SEGMENTS",
            "PROVIDER_NOT_CONFIGURED",
            "PROVIDER_FAILED",
            "PROVIDER_TIMEOUT",
            "INTERNAL_ERROR",
        ]
    );
}

#[test]
fn result_schema_contains_every_error_code() {
    for code in ErrorCode::ALL {
        assert!(
            RESULT_SCHEMA.contains(code.as_str()),
            "schema is missing {}",
            code.as_str()
        );
    }
}
