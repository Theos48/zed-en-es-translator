use crate::acquisition::AcquisitionError;

/// Stable in-editor message for preparation failures.
pub fn acquisition_message(error: AcquisitionError) -> String {
    error.to_string()
}

/// Remove common content, credential, URL and absolute-path fragments before
/// returning any internal diagnostic through the Zed extension boundary.
pub fn redact_sensitive(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let lower = word.to_ascii_lowercase();
            if word.starts_with('/')
                || word.starts_with("http://")
                || word.starts_with("https://")
                || lower.contains("token=")
                || lower.contains("key=")
                || lower.contains("source_text=")
                || lower.contains("translated_text=")
            {
                "[redacted]"
            } else {
                word
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
