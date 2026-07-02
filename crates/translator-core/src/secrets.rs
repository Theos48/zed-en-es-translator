pub(crate) fn contains_secret_pattern(input: &str) -> bool {
    let lower = input.to_ascii_lowercase();
    lower.contains("api_key")
        || lower.contains("authorization: bearer")
        || lower.contains("bearer ")
        || lower.contains(".env")
        || lower.contains("_token=")
        || lower.contains("service_token")
        || lower.contains("database_url=")
        || contains_private_key_header(&lower)
}

pub(crate) fn contains_sensitive_path(input: &str) -> bool {
    let lower = input.to_ascii_lowercase();
    lower.contains("/home/") || lower.contains("/users/") || lower.contains("\\users\\")
}

fn contains_private_key_header(lower: &str) -> bool {
    [
        "-----begin private key-----",
        "-----begin rsa private key-----",
        "-----begin ec private key-----",
        "-----begin openssh private key-----",
        "-----begin dsa private key-----",
    ]
    .iter()
    .any(|header| lower.contains(header))
}
